use http::{HeaderName, HeaderValue};
use std::error::Error;

pub struct RequestBuilder {
    reqwest_builder: reqwest::RequestBuilder,
}

impl RequestBuilder {
    pub fn header<KEY>(mut self, key: KEY) -> Self
    where
        HeaderName: TryFrom<KEY>,
        <HeaderName as TryFrom<KEY>>::Error: Into<Box<dyn Error>>,
    {
        let key = match HeaderName::try_from(key) {
            Ok(key) => key,
            Err(_) => {
                return self;
            }
        };

        self.reqwest_builder = self.reqwest_builder.header(key, 1234);
        self
    }

    fn test(mut self) -> Self {
        let key = match HeaderName::try_from(http::header::CONTENT_LENGTH) {
            Ok(key) => key,
            Err(_) => {
                return self;
            }
        };

        self.reqwest_builder = self.reqwest_builder.header(key, 1234);
        self
    }
}
