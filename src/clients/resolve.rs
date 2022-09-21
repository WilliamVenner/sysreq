use std::{sync::atomic::{AtomicU8, self}, cell::UnsafeCell, time::Duration};
use super::ResolvedSystemHttpClient;
use crate::Error;

const PENDING: u8 = 0;
const BUSY: u8 = 1;
const READY: u8 = 2;

struct GlobalResolvedSystemHttpClient {
	state: AtomicU8,
	value: UnsafeCell<Option<ResolvedSystemHttpClient>>
}
impl GlobalResolvedSystemHttpClient {
	fn get() -> Option<ResolvedSystemHttpClient> {
		static HTTP_CLIENT: GlobalResolvedSystemHttpClient = GlobalResolvedSystemHttpClient {
			state: AtomicU8::new(PENDING),
			value: UnsafeCell::new(None)
		};

		let mut spin_wait = Duration::from_millis(50);
		loop {
			match HTTP_CLIENT.state.fetch_max(BUSY, atomic::Ordering::SeqCst) {
				BUSY => (),
				PENDING => break,
				READY => return unsafe { *HTTP_CLIENT.value.get() },
				_ => unreachable!()
			}
			std::thread::sleep(spin_wait);
			spin_wait = Duration::max(spin_wait * 2, Duration::from_secs(2));
		}

		let resolved = ResolvedSystemHttpClient::resolve();

		unsafe { *HTTP_CLIENT.value.get() = resolved };

		HTTP_CLIENT.state.store(READY, atomic::Ordering::Release);

		resolved
	}
}
unsafe impl Sync for GlobalResolvedSystemHttpClient {}

pub(crate) fn resolve() -> Result<ResolvedSystemHttpClient, Error> {
	match GlobalResolvedSystemHttpClient::get() {
		Some(client) => Ok(client),
		None => Err(Error::SystemHTTPClientNotFound),
	}
}

#[inline]
/// Returns whether the system has a compatible HTTP client installed
pub fn installed() -> bool {
	GlobalResolvedSystemHttpClient::get().is_some()
}