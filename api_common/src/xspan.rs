#[derive(Clone)]
pub struct XSpanIdString(pub String);

pub trait Has<T> {
    fn get(&self) -> &T;
}
