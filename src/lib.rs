#![forbid(unsafe_code)]
#![deny(missing_docs)]

//! Simple, virtually-zero-dependencies HTTP client wrapping a system client.
//!
//! "Virtually-zero" means no unnecessary runtime dependencies. The only runtime dependency, other than `std`, is URL validation, which is required for security reasons.
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

mod tests;

mod error;
pub use error::Error;

mod clients;
use clients::SystemHTTPClient;
pub use clients::{installed, supported_http_clients};

mod url;
use crate::url::ValidUrl;

/// Perform a GET request to the given URL
pub fn get(uri: impl ValidUrl) -> Result<Vec<u8>, Error> {
	let uri = uri.validate()?;
	clients::resolve()?.get(uri)
}
