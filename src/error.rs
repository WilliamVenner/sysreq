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

#[derive(Debug)]
/// Errors that sysreq can return
pub enum Error {
	/// This system does not have an HTTP client installed
	SystemHTTPClientNotFound,

	/// An I/O error occurred
	IoError(std::io::Error),

	/// The provided URL is invalid
	InvalidUrl(url::ParseError),

	/// The URL must have a http or https scheme for security reasons
	InvalidUrlScheme,

	/// Generic failure with HTTP client
	CommandFailed {
		/// The returned exit status from the HTTP client process
		status: std::process::ExitStatus,

		/// The standard output stream it returned
		stdout: CommandFailedOutput,

		/// The standard error stream it returned
		stderr: CommandFailedOutput,
	},
}
impl std::fmt::Display for Error {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Error::SystemHTTPClientNotFound => {
				write!(f, "This system does not have an HTTP client installed")
			}
			Error::IoError(e) => write!(f, "I/O error: {}", e),
			Error::InvalidUrl(e) => write!(f, "Invalid URL: {}", e),
			Error::InvalidUrlScheme => write!(f, "URL must have http or https scheme"),
			Error::CommandFailed { status, .. } => write!(f, "Process exited with code {status:?}"),
		}
	}
}
impl std::error::Error for Error {}
impl From<url::ParseError> for Error {
	fn from(err: url::ParseError) -> Self {
		Self::InvalidUrl(err)
	}
}
impl From<std::io::Error> for Error {
	fn from(err: std::io::Error) -> Self {
		Self::IoError(err)
	}
}
