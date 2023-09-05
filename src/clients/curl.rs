use super::*;

#[allow(non_camel_case_types)]
pub struct cURL;
impl SystemHttpClientInterface for cURL {
	const COMMAND: &'static str = "curl";

	fn get(&self, url: &str, timeout: Option<Duration>) -> Result<Response, Error> {
		let output = spawn(Self::COMMAND)
			.arg("-m")
			.arg(timeout.unwrap_or(Duration::ZERO).as_secs_f64().to_string())
			.arg("-L")
			.arg(&url)
			.output()?;

		if output.status.success() {
			Ok(Response { body: output.stdout })
		} else if output.status.code() == Some(28) {
			Err(Error::IoError(std::io::Error::from(std::io::ErrorKind::TimedOut)))
		} else {
			Err(Error::CommandFailed {
				status: output.status,
				stdout: CommandFailedOutput(output.stdout),
				stderr: CommandFailedOutput(output.stderr),
			})
		}
	}
}
