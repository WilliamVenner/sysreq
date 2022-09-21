use std::process::Stdio;
use crate::error::{Error, CommandFailedOutput};

pub(crate) mod resolve;

/// **Use `spawn` to create a new `std::process::Command`, not `Command::new`!**
type Command = std::process::Command;

#[inline(always)]
fn spawn<S: AsRef<std::ffi::OsStr>>(program: S) -> Command {
	#[allow(unused_mut)]
	let mut command = Command::new(program);

	#[cfg(windows)] {
		use std::os::windows::process::CommandExt;

		// Don't create a new window!
		const CREATE_NO_WINDOW: u32 = 0x08000000;
		command.creation_flags(CREATE_NO_WINDOW);
	}

	command
}

pub(crate) trait SystemHttpClient: Sized + Send + Sync {
	const COMMAND: &'static str;

	fn installed_spawn() -> Command {
		spawn(Self::COMMAND)
	}

	fn installed() -> bool {
		!matches!(
			Self::installed_spawn()
				.stdin(Stdio::null())
				.stdout(Stdio::null())
				.stderr(Stdio::null())
				.status(),

			Err(err) if err.kind() == std::io::ErrorKind::NotFound
		)
	}

	fn get(&self, uri: &str) -> Result<Vec<u8>, Error>;
}

// Defines available system HTTP clients in `mod`s in order of preference
macro_rules! system_http_clients {
	{$($(#[$cfg:meta])? mod $mod:ident::$client:ident;)*} => {
		$($(#[$cfg])? mod $mod;)*

		#[derive(Clone, Copy, Debug)]
		pub(super) enum ResolvedSystemHttpClient {
			$($(#[$cfg])? #[allow(non_camel_case_types)] $client,)*
		}
		impl SystemHttpClient for ResolvedSystemHttpClient {
			const COMMAND: &'static str = "";

			fn installed() -> bool {
				unimplemented!()
			}

			fn get(&self, uri: &str) -> Result<Vec<u8>, Error> {
				match self {
					$($(#[$cfg])? Self::$client => $mod::$client.get(uri)),*
				}
			}
		}
		impl ResolvedSystemHttpClient {
			fn resolve() -> Option<Self> {
				#[allow(clippy::never_loop)]
				loop {
					$($(#[$cfg])? {
						if <$mod::$client as SystemHttpClient>::installed() {
							break Some(ResolvedSystemHttpClient::$client);
						}
					})*
					break None;
				}
			}
		}

		/// Returns a list of supported system HTTP clients
		///
		/// This should be used to inform users of their choices if an HTTP client wasn't found on their system.
		pub const fn supported_http_clients() -> &'static [&'static str] {
			&[
				$($(#[$cfg])? { stringify!($client) }),*
			]
		}

		#[cfg(test)]
		pub(crate) fn all_http_clients() -> impl Iterator<Item = ResolvedSystemHttpClient> {
			[
				$($(#[$cfg])? {
					if <$mod::$client>::installed() {
						Some(ResolvedSystemHttpClient::$client)
					} else {
						None
					}
				}),*
			].into_iter().flatten()
		}
	};
}
system_http_clients! {
	mod wget::wget;

	// Prefer PowerShell over cURL on Windows systems
	#[cfg(windows)]
	mod powershell::PowerShell;
	#[cfg(windows)]
	mod curl::cURL;

	// Prefer cURL over PowerShell on non-Windows systems
	#[cfg(not(windows))]
	mod curl::cURL;
	#[cfg(not(windows))]
	mod powershell::PowerShell;
}