#[cfg(test)]
use crate::SystemHttpClient;

#[test]
fn test_get() {
	let reqwest = reqwest::blocking::Client::new();

	for test_url in ["https://www.google.com/favicon.ico", "http://www.example.org"] {
		for client in crate::clients::all_http_clients() {
			let result = client.get(test_url).unwrap();

			let example = reqwest
				.get(test_url)
				.send()
				.unwrap()
				.bytes()
				.unwrap()
				.to_vec();

			if example != result {
				let example = String::from_utf8_lossy(&example);
				let result = String::from_utf8_lossy(&result);
				panic!("Client: {client:?}\nURL: {test_url}\n\nDiff:\n{}", difference::Changeset::new(example.as_ref(), result.as_ref(), ""));
			}
		}
	}
}

#[test]
fn test_naughty_url() {
	match super::get("file:///etc/passwd") {
		Ok(_) => panic!("pwned"),
		Err(super::Error::InvalidUrlScheme) => {},
		Err(err) => panic!("{err}")
	}
}

#[test]
fn test_str_interp_url() {
	std::env::set_var("SYSREQ_PWNED", "http://example.org");

	for client in crate::clients::all_http_clients() {
		for interp in ["$SYSREQ_PWNED", "`SYSREQ_PWNED`", "${SYSREQ_PWNED}", "[[SYSREQ_PWNED]]"].into_iter() {
			if let Ok(result) = client.get(interp) {
				if !result.is_empty() {
					panic!("This should have failed: {}", String::from_utf8_lossy(&result));
				}
			}
		}

		if let Ok(result) = client.get("#//\"\"\"\"\"'''''[[]]`````${hello}$hello###") {
			if !result.is_empty() {
				panic!("This should have failed: {}", String::from_utf8_lossy(&result));
			}
		}

		let example = client.get("http://example.org").unwrap();
		let result = client.get("http://example.org/#//\"\"\"\"\"'''''[[]]`````${hello}$hello###").unwrap();
		if example != result {
			let example = String::from_utf8_lossy(&example);
			let result = String::from_utf8_lossy(&result);
			panic!("Diff:\n{}", difference::Changeset::new(example.as_ref(), result.as_ref(), ""));
		}
	}
}
