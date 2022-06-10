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
		#[inline]
		fn format_script(uri: &str) -> String {
			format!(r#"
				$data = (Invoke-WebRequest '{uri}').Content;
				$Writer = New-Object System.IO.BinaryWriter([console]::OpenStandardOutput());
				$Writer.Write($data, 0, $data.length)
				$Writer.Flush()
				$Writer.Close()
			"#)
		}

		let uri = if uri.find('\'').is_some() {
			format_script(&uri.replace('\'', "''"))
		} else {
			format_script(uri)
		};

		let mut output = spawn(Self::COMMAND).arg("-command").arg(&uri).output()?;

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