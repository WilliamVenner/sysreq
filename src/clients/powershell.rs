use std::io::BufRead;

use super::*;

trait ContainsSlice {
	fn contains_slice(&self, subslice: &[u8]) -> bool;
}
impl ContainsSlice for [u8] {
	fn contains_slice(&self, subslice: &[u8]) -> bool {
		if self.len() < subslice.len() {
			return false;
		}
		(0..=self.len() - subslice.len())
			.any(|start| &self[start..start + subslice.len()] == subslice)
	}
}

pub struct PowerShell;
impl SystemHttpClientInterface for PowerShell {
	#[cfg(windows)]
	const COMMAND: &'static str = "powershell";

	#[cfg(not(windows))]
	const COMMAND: &'static str = "pwsh";

	fn installed_spawn() -> Command {
		let mut cmd = spawn(Self::COMMAND);
		cmd.arg("-help");
		cmd
	}

	fn get(&self, url: &str, timeout: Option<Duration>) -> Result<Response, Error> {
		fn format_script(url: &str, timeout: Option<Duration>) -> String {
			let uri_escaped;
			let url = if url.find('\'').is_some() {
				uri_escaped = url.replace('\'', "''");
				&uri_escaped
			} else {
				url
			};
			format!(
				r#"
				$ErrorActionPreference = "Stop"
				[System.Net.ServicePointManager]::MaxServicePointIdleTime = {timeout_ms};
				$data = (Invoke-WebRequest -TimeoutSec {timeout_sec} '{url}').Content;
				$Writer = New-Object System.IO.BinaryWriter([console]::OpenStandardOutput());
				$Writer.Write($data, 0, $data.length)
				$Writer.Flush()
				$Writer.Close()
			"#,
				url = url,
				timeout_ms = timeout.map(|timeout| (timeout.as_secs_f64().max(0.001) * 1000.0).round() as u64).unwrap_or(0),
				timeout_sec = timeout.map(|timeout| timeout.as_secs_f64().max(1.0).round() as u64).unwrap_or(0)
			)
		}

		let mut output = spawn(Self::COMMAND).arg("-command").arg(format_script(url, timeout)).output()?;

		// Remove the trailing CRLF PowerShell adds...
		if output.stdout.len() >= 2 && &output.stdout[output.stdout.len() - 2..] == b"\r\n" {
			output.stdout.pop();
			output.stdout.pop();
		}

		if output.status.success() {
			Ok(Response { body: output.stdout })
		} else {
			if cfg!(windows) {
				if (&output.stderr)
					.lines()
					.next()
					.and_then(|line| Some(line.ok()?.contains("The operation has timed out")))
					.unwrap_or(false)
				{
					return Err(Error::IoError(std::io::Error::from(std::io::ErrorKind::TimedOut)));
				}
			} else if cfg!(unix) {
				// ??? wtf microsoft
				if output.stderr.contains_slice(b"HttpClient.Timeout") {
					return Err(Error::IoError(std::io::Error::from(std::io::ErrorKind::TimedOut)));
				}
			}

			Err(Error::CommandFailed {
				status: output.status,
				stdout: CommandFailedOutput(output.stdout),
				stderr: CommandFailedOutput(output.stderr),
			})
		}
	}
}
