pub trait ValidUrl: AsRef<str> {
	#[cfg(feature = "validate")]
	fn validate(&self) -> Result<&str, super::Error> {
		let url = url::Url::parse(self.as_ref())?;
		if matches!(url.scheme(), "http" | "https") {
			Ok(self.as_ref())
		} else {
			Err(super::Error::InvalidUrlScheme)
		}
	}
	#[cfg(not(feature = "validate"))]
	fn validate(&self) -> Result<&str, super::Error> {
		if self.as_ref().starts_with("http://") || self.as_ref().starts_with("https://") {
			Ok(self.as_ref())
		} else {
			Err(super::Error::InvalidUrlScheme)
		}
	}
}
impl<S: AsRef<str>> ValidUrl for S {}
