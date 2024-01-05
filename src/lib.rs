use reqwest::header::HeaderName;
use std::error::Error;

pub fn header<KEY>(reqwest_builder: reqwest::RequestBuilder)
where
    HeaderName: TryFrom<KEY>,
    <HeaderName as TryFrom<KEY>>::Error: Into<Box<dyn Error>>,
{
    let _ = reqwest_builder.header(reqwest::header::CONTENT_LENGTH, 1234);
}

fn test(reqwest_builder: reqwest::RequestBuilder) {
    reqwest_builder.header(reqwest::header::CONTENT_LENGTH, 1234);
}
