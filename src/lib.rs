//! Simple, virtually-zero-dependencies HTTP client wrapping a system client.
//!
//! For when you want to make dead simple HTTP requests without breaking the bank.
//!
//! ## Supported Backends
//!
//! * wget
//! * cURL
//! * PowerShell (`Invoke-WebRequest`)
//! # Usage
//!
//! In your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! sysreq = "0"
//! ```
//!
//! In your code:
//!
//! ```rust
//! let html = sysreq::get("https://www.rust-lang.org/").unwrap();
//! println!("{}", String::from_utf8_lossy(&html));
//! ```

use url::Url;
use std::process::{Command, ExitStatus, Stdio};

mod tests;

#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Wrapper around Stdio output that prints as a string for debugging purposes.
pub struct CommandFailedOutput(pub Vec<u8>);
impl std::fmt::Display for CommandFailedOutput {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from_utf8_lossy(&self.0))
    }
}
impl std::fmt::Debug for CommandFailedOutput {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}
impl From<CommandFailedOutput> for Vec<u8> {
    #[inline(always)]
    fn from(out: CommandFailedOutput) -> Self {
        out.0
    }
}

#[derive(Debug, thiserror::Error)]
/// Errors that sysreq can return
pub enum Error {
    #[error("This system does not have an HTTP client installed")]
    SystemHTTPClientNotFound,

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Process exited with code {status:?}")]
    CommandFailed {
        status: ExitStatus,
        stdout: CommandFailedOutput,
        stderr: CommandFailedOutput,
    },
}

pub(crate) trait SystemHTTPClient: Sized + Send + Sync {
    const COMMAND: &'static str;

    fn installed_spawn() -> Command {
        Command::new(Self::COMMAND)
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

		#[allow(non_camel_case_types)]
		#[derive(Clone, Copy, Debug)]
		enum ResolvedSystemHTTPClient {
			$($(#[$cfg])? $client,)*
		}
		impl SystemHTTPClient for ResolvedSystemHTTPClient {
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

		lazy_static::lazy_static! {
			static ref HTTP_CLIENT: Option<ResolvedSystemHTTPClient> = {
				#[allow(clippy::never_loop)]
				loop {
					$($(#[$cfg])? {
						if <$mod::$client as SystemHTTPClient>::installed() {
							break Some(ResolvedSystemHTTPClient::$client);
						}
					})*
					break None;
				}
			};
		}

		/// Returns a list of supported system HTTP clients
		///
		/// This should be used to inform users of their choices if an HTTP client wasn't found on their system.
		pub fn supported_http_clients() -> &'static [&'static str] {
			&[
				$($(#[$cfg])? { stringify!($client) }),*
			]
		}

		#[cfg(test)]
		fn all_http_clients() -> impl Iterator<Item = ResolvedSystemHTTPClient> {
			[
				$($(#[$cfg])? {
					if <$mod::$client>::installed() {
						Some(ResolvedSystemHTTPClient::$client)
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
    mod powershell::PowerShell;
    #[cfg(not(windows))] mod curl::cURL;
}

fn http_client() -> Result<ResolvedSystemHTTPClient, Error> {
    match *HTTP_CLIENT {
        Some(client) => Ok(client),
        None => Err(Error::SystemHTTPClientNotFound),
    }
}

#[inline]
/// Returns whether the system has a compatible HTTP client installed
pub fn installed() -> bool {
    HTTP_CLIENT.is_some()
}

/// Perform a GET request to the given URL
pub fn get(uri: impl AsRef<str>) -> Result<Vec<u8>, Error> {
    let uri = uri.as_ref();
    let _: Url = uri.try_into()?;
    http_client()?.get(uri)
}
