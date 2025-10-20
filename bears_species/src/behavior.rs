pub trait Describe {
    fn description(&self) -> &'static str;
}

pub trait Code<T> {
    type Decoded;

    fn code(&self) -> T;
    fn from_code(code: T) -> Option<Self::Decoded>;
}
