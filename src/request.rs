use crate::{
	clients::{resolve::resolve, SystemHttpClientInterface},
	url::ValidUrl,
	Error,
};
use std::time::Duration;

/// A response from a sent request
#[derive(Debug)]
pub struct Response {
	/// The body of the response
	pub body: Vec<u8>,
}

/// A builder for a request
pub struct RequestBuilder<U: ValidUrl> {
	url: U,
	timeout: Option<Duration>,
}
impl<U: ValidUrl> RequestBuilder<U> {
	/// Creates a new request builder with the given URL
	#[must_use]
	pub fn new(url: U) -> Self {
		Self { url, timeout: None }
	}

	/// Sets the timeout for the request
	///
	/// # Panics
	///
	/// Panics if the timeout is zero.
	#[must_use]
	pub fn timeout(mut self, timeout: Option<Duration>) -> Self {
		self.timeout = match timeout {
			Some(timeout) if !timeout.is_zero() => Some(timeout),
			None => None,
			_ => panic!("Timeout must be non-zero"),
		};
		self
	}

	/// Sends the request
	pub fn send(self) -> Result<Response, Error> {
		resolve()?.get(self.url.validate()?, self.timeout)
	}
}
