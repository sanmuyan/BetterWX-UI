pub trait Empty {
    fn is_empty(&self) -> bool;
}

impl<T: Empty> Empty for Option<T> {
    fn is_empty(&self) -> bool {
        self.as_ref().map(|s| s.is_empty()).unwrap_or(true)
    }
}

impl Empty for String {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl Empty for Option<&usize> {
    fn is_empty(&self) -> bool {
        self.map(|s| *s == 0).unwrap_or(true)
    }
}

impl Empty for usize {
    fn is_empty(&self) -> bool {
        *self == 0
    }
}

impl<T: Default> Empty for Vec<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl Empty for bool {
    fn is_empty(&self) -> bool {
        *self == false
    }
}
