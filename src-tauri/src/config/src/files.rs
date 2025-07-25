use crate::rules::Rules;
use macros::ImpConfigVecIsEmptyTrait;
use macros::ImpConfigVecWrapperTrait;
use serde::Deserialize;
use serde::Serialize;

#[derive(
    Debug, Default, Serialize, Deserialize, ImpConfigVecIsEmptyTrait, ImpConfigVecWrapperTrait,
)]
pub struct FilesRules(pub Vec<FileRules>);

#[derive(Debug, Serialize, Deserialize)]
pub struct FileRules {
    pub code: String,
    pub rules: Rules,
}

impl FileRules {
    pub fn new(code: &str, rules: Rules) -> Self {
        Self {
            code: code.to_string(),
            rules,
        }
    }
}
