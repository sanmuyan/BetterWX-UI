use crate::rules::Rule;
use crate::serders::skippers::skip_if_empty;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AddressView {
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub supported: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub patched: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub rtype: usize,
}

impl From<&Rule> for AddressView {
    fn from(rule: &Rule) -> Self {
        Self {
            rtype: rule.rtype.clone() as usize,
            supported: rule.supported,
            patched: rule.patched,
        }
    }
}
