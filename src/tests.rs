#[test]
fn test_get() {
    use crate::SystemHTTPClient;

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
