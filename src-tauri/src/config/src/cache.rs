use crate::errors::ConfigError;
use crate::errors::Result;
use std::collections::HashMap;
use std::fmt::Debug;
use utils::empty::Empty;
use utils::patch::patch::UPatch;

#[derive(Default)]
pub struct Cache(HashMap<String, UPatch>);

impl Cache {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn get(&self, key: &str) -> Option<&UPatch> {
        self.0.get(key)
    }

    pub fn insert(&mut self, key: &str, patch: UPatch) {
        self.0.insert(key.to_string(), patch);
    }

    pub fn get_or_insert(
        &mut self,
        key: &str,
        input: &str,
        save: &str,
        with_write: bool,
    ) -> crate::errors::Result<&mut UPatch> {
        if !self.0.contains_key(key) {
            let patch: UPatch = UPatch::create(input, save, with_write)?;
            self.0.insert(key.to_string(), patch);
        }
        self.0
            .get_mut(key)
            .ok_or(ConfigError::CacheNotFindError.into())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn save(&self) -> Result<()> {
        for (_, patch) in &self.0 {
            patch.save()?;
        }
        Ok(())
    }
}

impl Debug for Cache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, patch) in &self.0 {
            writeln!(f, "key:{},len: {}", key, patch.len())?;
        }
        Ok(())
    }
}

impl Clone for Cache {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl Drop for Cache {
    fn drop(&mut self) {
        self.clear();
    }
}

impl Empty for Cache {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}