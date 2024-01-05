use http::{HeaderName, HeaderValue};
use std::error::Error;

pub struct RequestBuilder {
    reqwest_builder: reqwest::RequestBuilder,
}

impl RequestBuilder {
    pub fn header<KEY>(mut self) -> Self
    where
        HeaderName: TryFrom<KEY>,
        <HeaderName as TryFrom<KEY>>::Error: Into<Box<dyn Error>>,
    {
        self.reqwest_builder = self
            .reqwest_builder
            .header(http::header::CONTENT_LENGTH, 1234);
        self
    }

    fn test(mut self) -> Self {
        self.reqwest_builder = self
            .reqwest_builder
            .header(http::header::CONTENT_LENGTH, 1234);
        self
    }
}
