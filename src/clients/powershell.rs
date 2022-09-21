use super::*;

pub struct PowerShell;
impl SystemHTTPClient for PowerShell {
	#[cfg(windows)]
	const COMMAND: &'static str = "powershell";

	#[cfg(not(windows))]
	const COMMAND: &'static str = "pwsh";

	fn installed_spawn() -> Command {
		let mut cmd = spawn(Self::COMMAND);
		cmd.arg("-help");
		cmd
	}

	fn get(&self, uri: &str) -> Result<Vec<u8>, Error> {
		fn format_script(uri: &str) -> String {
			let uri_escaped;
			let uri = if uri.find('\'').is_some() {
				uri_escaped = uri.replace('\'', "''");
				&uri_escaped
			} else {
				uri
			};
			format!(r#"
				$data = (Invoke-WebRequest '{uri}').Content;
				$Writer = New-Object System.IO.BinaryWriter([console]::OpenStandardOutput());
				$Writer.Write($data, 0, $data.length)
				$Writer.Flush()
				$Writer.Close()
			"#)
		}

		let mut output = spawn(Self::COMMAND).arg("-command").arg(format_script(uri)).output()?;

		// Remove the trailing CRLF PowerShell adds...
		if output.stdout.len() >= 2 && &output.stdout[output.stdout.len() - 2..] == b"\r\n" {
			output.stdout.pop();
			output.stdout.pop();
		}

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