use super::SystemHttpClient;
use crate::Error;
use std::{
	cell::UnsafeCell,
	sync::atomic::{self, AtomicU8},
	time::Duration,
};

const PENDING: u8 = 0;
const BUSY: u8 = 1;
const READY: u8 = 2;

struct GlobalResolvedSystemHttpClient {
	state: AtomicU8,
	value: UnsafeCell<Option<SystemHttpClient>>,
}
impl GlobalResolvedSystemHttpClient {
	fn get() -> Option<SystemHttpClient> {
		static HTTP_CLIENT: GlobalResolvedSystemHttpClient = GlobalResolvedSystemHttpClient {
			state: AtomicU8::new(PENDING),
			value: UnsafeCell::new(None),
		};

		let mut spin_wait = Duration::from_millis(50);
		loop {
			match HTTP_CLIENT.state.fetch_max(BUSY, atomic::Ordering::SeqCst) {
				BUSY => (),
				PENDING => break,
				READY => return unsafe { *HTTP_CLIENT.value.get() },
				_ => unreachable!(),
			}
			std::thread::sleep(spin_wait);
			spin_wait = Duration::max(spin_wait * 2, Duration::from_secs(2));
		}

		let resolved = SystemHttpClient::resolve();

		unsafe { *HTTP_CLIENT.value.get() = resolved };

		HTTP_CLIENT.state.store(READY, atomic::Ordering::Release);

		resolved
	}
}
unsafe impl Sync for GlobalResolvedSystemHttpClient {}

pub(crate) fn resolve() -> Result<SystemHttpClient, Error> {
	match GlobalResolvedSystemHttpClient::get() {
		Some(client) => Ok(client),
		None => Err(Error::SystemHTTPClientNotFound),
	}
}

#[must_use]
/// Returns whether the system has a compatible HTTP client installed
pub fn installed() -> bool {
	GlobalResolvedSystemHttpClient::get().is_some()
}

#[must_use]
/// Returns the system's compatible HTTP client used for requests, if one is installed.
pub fn http_client() -> Option<SystemHttpClient> {
	GlobalResolvedSystemHttpClient::get()
}
