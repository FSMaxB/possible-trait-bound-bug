use reqwest::header::HeaderName;

pub fn broken<KEY>()
where
    HeaderName: From<KEY>,
{
    header(reqwest::header::CONTENT_LENGTH);
}

fn works() {
    header(reqwest::header::CONTENT_LENGTH);
}

pub fn header<K>(key: K)
where
    HeaderName: From<K>,
{
    drop(key);
}
