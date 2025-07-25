use serde::Serialize;
use utils::empty::Empty;

pub fn skip_if_tdelay(value: &usize) -> bool {
    *value == 150 || *value == 0
}

pub fn skip_if_empty<T: Serialize + Empty>(value: &T) -> bool {
    value.is_empty()
}

