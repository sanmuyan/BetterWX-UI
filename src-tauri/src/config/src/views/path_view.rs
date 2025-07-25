use crate::features::Features;
use crate::rules::Rule;
use crate::serders::skippers::skip_if_empty;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PathView {
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub hfeatures: Features,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub installed: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub install_location: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub install_version: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub rtype: usize,
}

impl From<&Rule> for PathView {
    fn from(rule: &Rule) -> Self {
        let install_location = rule
            .variables
            .get_install_loction()
            .unwrap_or("")
            .to_string();
        let install_version = rule
            .variables
            .get_install_version()
            .unwrap_or("")
            .to_string();
        Self {
            rtype: rule.rtype.clone() as usize,
            hfeatures: rule.hfeatures.clone(),
            installed: rule.installed,
            install_location,
            install_version,
        }
    }
}
