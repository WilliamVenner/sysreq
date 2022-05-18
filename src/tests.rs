#[test]
fn test_get() {
    use crate::SystemHTTPClient;

    for client in super::all_http_clients() {
        let example = reqwest::blocking::Client::new()
            .get("https://www.google.com/favicon.ico")
            .send()
            .unwrap()
            .bytes()
            .unwrap()
            .to_vec();

        let result = client.get("https://www.google.com/favicon.ico").unwrap();

        assert_eq!(
            String::from_utf8_lossy(&example),
            String::from_utf8_lossy(&result),
            "{client:?} failed"
        );
    }
}
