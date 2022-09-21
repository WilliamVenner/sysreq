use super::*;

#[allow(non_camel_case_types)]
pub struct cURL;
impl SystemHttpClient for cURL {
	const COMMAND: &'static str = "curl";

	fn get(&self, uri: &str) -> Result<Vec<u8>, Error> {
		let output = spawn(Self::COMMAND).arg("-g").arg(&uri).output()?;
		if output.status.success() {
			Ok(output.stdout)
		} else {
			Err(Error::CommandFailed {
				status: output.status,
				stdout: CommandFailedOutput(output.stdout),
				stderr: CommandFailedOutput(output.stderr)
			})
		}
	}
}