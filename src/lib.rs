pub fn broken<T>()
where
    (): From<T>,
{
    call(());
}

fn works() {
    call(());
}

pub fn call<T>(_: T)
where
    (): From<T>,
{
}
