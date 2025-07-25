use crate::errors::ConfigError;
use crate::errors::Result;
use crate::features::Features;
use crate::rules::Rule;
use crate::rules::Rules;
use crate::serders::skippers::skip_if_empty;
use serde::Serialize;

#[derive(Debug, Default, Clone, Serialize)]

pub struct FilesView(pub Vec<FileView>);

impl TryFrom<&Rules> for FilesView {
    type Error = ConfigError;
    fn try_from(files: &Rules) -> Result<Self> {
        let views = files
            .0
            .iter()
            .map(|file| FileView::try_from(file))
            .collect::<Result<Vec<_>>>()?;
        Ok(Self(views))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FileView {
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    ismain: bool,
    index: usize,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    features: Features,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    rtype: usize,
}

impl TryFrom<&Rule> for FileView {
    type Error = ConfigError;
    fn try_from(rule: &Rule) -> Result<Self> {
        rule.check_is_fileed_type()?;
        let view = Self {
            rtype: rule.rtype.clone() as usize,
            features: rule.features.clone(),
            name: rule.name.clone(),
            ismain: rule.ismain,
            index: rule.index,
        };
        Ok(view)
    }
}
