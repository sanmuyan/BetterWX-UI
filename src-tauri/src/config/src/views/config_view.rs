use crate::Config;
use crate::errors::ConfigError;
use crate::errors::Result;
use crate::rules::Rule;
use crate::rules::Rules;
use crate::serders::skippers::skip_if_empty;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ConfigViews(pub Vec<InitView>);

impl TryFrom<&Rules> for ConfigViews {
    type Error = ConfigError;
    fn try_from(rules: &Rules) -> Result<Self> {
        let mut views = Vec::new();
        for rule in &rules.0 {
            let view = InitView::try_from(rule)?;
            views.push(view);
        }
        Ok(Self(views))
    }
}

impl TryFrom<&Config> for ConfigViews {
    type Error = ConfigError;
    fn try_from(confg: &Config) -> Result<Self> {
        let views = ConfigViews::try_from(&confg.rules)?;
        Ok(views)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct InitView {
    pub code: String,
    pub index: usize,
    pub version: String,
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub news: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub description: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub disabled: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub supported: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub rtype: usize,
}

impl TryFrom<&Rule> for InitView {
    type Error = ConfigError;
    fn try_from(rule: &Rule) -> Result<Self> {
        rule.check_is_config_type()?;
        let view = Self {
            code: rule.code.clone(),
            index: rule.index,
            rtype: rule.rtype.clone() as usize,
            version: rule.version.clone(),
            name: rule.get_name().to_string(),
            news: rule.news.clone(),
            description: rule.description.clone(),
            disabled: rule.disabled,
            supported: rule.supported,
        };
        Ok(view)
    }
}
