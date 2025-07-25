pub mod addresses;
pub mod cache;
pub mod dfetures;
pub mod errors;
pub mod features;
pub mod files;
pub mod groups;
pub mod patches;
pub mod paths;
pub mod patterns;
pub mod rules;
pub mod serders;
pub mod update;
pub mod variables;
pub mod views;

use crate::files::FilesRules;
use crate::rules::RuleType;
use crate::serders::skippers::skip_if_empty;
use errors::Result;
use rules::Rules;
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;
use utils::version::Version;
// use std::sync::Mutex;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

#[derive(Debug)]
pub struct ConfigArc(pub Arc<Mutex<Config>>);

impl ConfigArc {
    pub async fn get(&self) -> MutexGuard<'_, Config> {
        self.0.lock().await
    }

    pub async fn with_config_mut<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Config) -> Result<T>,
    {
        let mut guard = self.get().await;
        f(&mut *guard)
    }

    pub async fn with_rules_mut<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Rules) -> Result<T>,
    {
        let mut guard = self.get().await;
        let rules = &mut guard.rules;
        f(rules)
    }

    pub async fn with_files_mut<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&mut Rules) -> Result<T>,
    {
        let mut guard = self.get().await;
        let rules = &mut guard.rules;
        f(rules)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub version: Version,
    #[serde(default)]
    ruletype: RuleType,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub name: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub disabled: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub supported: bool,
    pub rules: Rules,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub files: FilesRules,
}

pub trait ConfigVecWrapperTrait {
    type Item;
    fn get(&self, code: &str) -> Result<&Self::Item>;
    fn get_mut(&mut self, code: &str) -> Result<&mut Self::Item>;
    fn find(&self, code: &str) -> Option<&Self::Item>;
    fn find_mut(&mut self, code: &str) -> Option<&mut Self::Item>;
    fn push(&mut self, item: Self::Item);
    fn take(&mut self, code: &str) -> Result<Self::Item>;
    fn len(&self) -> usize;
    fn clear(&mut self);
}

fn convert_num(num: usize) -> (bool, String) {
    match num {
        0 => (true, "主程序".to_string()),
        _ => (false, format!("共存-{}", num)),
    }
}
