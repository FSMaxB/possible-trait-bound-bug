use reqwest::header::HeaderName;
use std::error::Error;

pub fn broken<KEY>()
where
    HeaderName: TryFrom<KEY>,
    <HeaderName as TryFrom<KEY>>::Error: Into<Box<dyn Error>>,
{
    header(reqwest::header::CONTENT_LENGTH);
}

fn works() {
    header(reqwest::header::CONTENT_LENGTH);
}

pub fn header<K>(key: K)
where
    HeaderName: TryFrom<K>,
    <HeaderName as TryFrom<K>>::Error: Into<Box<dyn Error>>,
{
    drop(key);
}
