#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
	/// This system does not have an HTTP client installed
	#[error("This system does not have an HTTP client installed")]
	SystemHTTPClientNotFound,

	/// An I/O error occurred
	#[error("I/O error: {0}")]
	IoError(#[from] std::io::Error),

	/// The provided URL is invalid
	#[error("invalid URL: {0}")]
	InvalidUrl(#[from] url::ParseError),

	/// The URL must have a http or https scheme for security reasons
	#[error("URL must have http or https scheme")]
	InvalidUrlScheme,

	/// Generic failure with HTTP client
	#[error("Process exited with code {status:?}")]
	CommandFailed {
		/// The returned exit status from the HTTP client process
		status: std::process::ExitStatus,

		/// The standard output stream it returned
		stdout: CommandFailedOutput,

		/// The standard error stream it returned
		stderr: CommandFailedOutput,
	},
}