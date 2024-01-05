use reqwest::header::HeaderName;

pub fn broken<T>()
where
    HeaderName: From<T>,
{
    call(reqwest::header::CONTENT_LENGTH);
}

fn works() {
    call(reqwest::header::CONTENT_LENGTH);
}

pub fn call<T>(_: T)
where
    HeaderName: From<T>,
{
}
