use http::{HeaderName, HeaderValue};

pub struct RequestBuilder {
    reqwest_builder: reqwest::RequestBuilder,
    error: Option<anyhow::Error>,
}

impl RequestBuilder {
    pub fn header<KEY, VALUE>(mut self, key: KEY, value: VALUE) -> Self
    where
        HeaderName: TryFrom<KEY>,
        <HeaderName as TryFrom<KEY>>::Error: Into<anyhow::Error>,
        HeaderValue: TryFrom<VALUE>,
        <HeaderValue as TryFrom<VALUE>>::Error: Into<anyhow::Error>,
    {
        let key = match HeaderName::try_from(key) {
            Ok(key) => key,
            Err(error) => {
                self.error = Some(error.into());
                return self;
            }
        };
        let value = match HeaderValue::try_from(value) {
            Ok(value) => value,
            Err(error) => {
                self.error = Some(error.into());
                return self;
            }
        };

        self.reqwest_builder = self.reqwest_builder.header(key, value);
        self
    }

    fn test(mut self) -> Self {
        let key = match HeaderName::try_from(http::header::CONTENT_LENGTH) {
            Ok(key) => key,
            Err(error) => {
                self.error = Some(error.into());
                return self;
            }
        };
        let value = match HeaderValue::try_from(1234) {
            Ok(value) => value,
            Err(error) => {
                self.error = Some(error.into());
                return self;
            }
        };

        self.reqwest_builder = self.reqwest_builder.header(key, value);
        self
    }
}
