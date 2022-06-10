use super::*;

#[allow(non_camel_case_types)]
pub struct wget;
impl SystemHTTPClient for wget {
	const COMMAND: &'static str = "wget";

	fn get(&self, uri: &str) -> Result<Vec<u8>, Error> {
		let output = spawn(Self::COMMAND).args(&["-qO", "-"]).arg(&uri).output()?;
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