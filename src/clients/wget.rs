use super::*;

#[allow(non_camel_case_types)]
pub struct wget;
impl SystemHttpClientInterface for wget {
	const COMMAND: &'static str = "wget";

	fn get(&self, url: &str, timeout: Option<Duration>) -> Result<Response, Error> {
		let output = spawn(Self::COMMAND).arg(format!("--timeout={}", timeout.unwrap_or(Duration::ZERO).as_secs_f64())).arg("--tries=1").args(&["-qO", "-"]).arg(&url).output()?;
		if output.status.success() {
			Ok(Response {
				body: output.stdout
			})
		} else if output.status.code() == Some(4) {
			Err(Error::IoError(std::io::Error::from(std::io::ErrorKind::TimedOut)))
		} else {
			Err(Error::CommandFailed {
				status: output.status,
				stdout: CommandFailedOutput(output.stdout),
				stderr: CommandFailedOutput(output.stderr)
			})
		}
	}
}