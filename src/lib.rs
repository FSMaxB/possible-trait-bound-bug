use http::HeaderName;
use std::error::Error;

pub struct RequestBuilder {
    reqwest_builder: reqwest::RequestBuilder,
}

impl RequestBuilder {
    pub fn header<KEY>(mut self)
    where
        HeaderName: TryFrom<KEY>,
        <HeaderName as TryFrom<KEY>>::Error: Into<Box<dyn Error>>,
    {
        let _ = self
            .reqwest_builder
            .header(http::header::CONTENT_LENGTH, 1234);
    }

    fn test(mut self) {
        let _ = self
            .reqwest_builder
            .header(http::header::CONTENT_LENGTH, 1234);
    }
}
