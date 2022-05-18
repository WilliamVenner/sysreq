pub trait ValidUrl: AsRef<str> {
	fn validate(&self) -> Result<&str, super::Error> {
		let url = url::Url::parse(self.as_ref())?;
		if matches!(url.scheme(), "http" | "https") {
			Ok(self.as_ref())
		} else {
			Err(super::Error::InvalidUrlScheme)
		}
	}
}
impl<S: AsRef<str>> ValidUrl for S {}