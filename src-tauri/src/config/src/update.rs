use crate::errors::Result;
use crate::serders::skippers::skip_if_empty;
use macros::ImpConfigVecIsEmptyTrait;
use macros::SortedDeserializeByVersionDesc;
use serde::Deserialize;
use serde::Serialize;
use setting::MAIN_PKG_VERSION;
use thiserror::Error;
use utils::destructure_assign;
use utils::empty::Empty;
use utils::version::Version;

#[derive(Debug, Error)]
pub enum UpdatesError {
    #[error("检测更新失败：无法获取更新数据")]
    UpdatesIsEmpty,

    #[error("发现新版本更新：v{0}，请更新软件")]
    ForceUpdate(String),
}

#[derive(
    Debug, Clone, Serialize, Default, ImpConfigVecIsEmptyTrait, SortedDeserializeByVersionDesc,
)]

pub struct Updates(pub Vec<Update>);

impl Updates {
    pub fn get_update(&mut self) -> Result<Update> {
        if self.is_empty() {
            return Err(UpdatesError::UpdatesIsEmpty.into());
        }
        let main_ver = Version::new(MAIN_PKG_VERSION);
        let self0_ver = Version::new(self.0[0].version.version.as_str());
        if &main_ver >= &self0_ver {
            return Ok(self.0.remove(0));
        }
        if self.0.len() == 1 {
            let mut update = self.0.remove(0);
            if &main_ver < &self0_ver && update.nversion == Version::default() {
                update.nversion = update.version.clone();
            }
            return Ok(update);
        }
        if let Ok(mut update) = self.take_first_less_by_version(MAIN_PKG_VERSION) {
            if self.is_empty() {
                return Ok(update);
            }
            let update0 = &self.0[0];
            destructure_assign!(
                update,
                update0,
                name,
                force,
                description,
                news,
                buttons,
                config,
                readme
            );
            if update.nversion == Version::default() {
                update.nversion = update0.version.clone();
            }
            return Ok(update);
        }
        Err(UpdatesError::ForceUpdate(self.0[0].version.to_string()).into())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]

pub struct Update {
    pub version: Version,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub nversion: Version,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub force: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub description: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub news: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub buttons: Vec<VerData>,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub config: VerData,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub readme: VerData,
}

impl Update {
    pub fn check_force_update(&self) -> Result<()> {
        if self.force {
            return Err(UpdatesError::ForceUpdate(self.nversion.to_string()).into());
        }
        Ok(())
    }
}
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VerData {
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub version: Version,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub name: String,
    pub data: String,
}

impl Empty for VerData {
    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_rule_services_parse() {}
}
