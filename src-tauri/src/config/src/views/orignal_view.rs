use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrignalViews(pub Vec<OrignalView>);

#[derive(Debug,Default, Clone, Serialize, Deserialize)]
pub struct OrignalView {
    pub pcode: String,
    pub pname: String,
    pub orignal: String,
    pub start: usize,
    pub len: usize,
}

impl OrignalView {
    pub fn new<S: Into<String>>(pcode: S, pname: S, orignal: S, start: usize, len: usize) -> Self {
        Self {
            pcode: pcode.into(),
            pname: pname.into(),
            orignal: orignal.into(),
            start,
            len,
        }
    }
}
