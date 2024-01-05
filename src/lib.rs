use reqwest::header::{HeaderName, HeaderValue};
use std::error::Error;

pub fn broken<KEY>()
where
    HeaderName: TryFrom<KEY>,
    <HeaderName as TryFrom<KEY>>::Error: Into<Box<dyn Error>>,
{
    header(reqwest::header::CONTENT_LENGTH, 1234);
}

fn works() {
    header(reqwest::header::CONTENT_LENGTH, 1234);
}

pub fn header<K, V>(key: K, value: V)
where
    HeaderName: TryFrom<K>,
    <HeaderName as TryFrom<K>>::Error: Into<Box<dyn Error>>,
    HeaderValue: TryFrom<V>,
    <HeaderValue as TryFrom<V>>::Error: Into<Box<dyn Error>>,
{
    drop(key);
    drop(value);
}
