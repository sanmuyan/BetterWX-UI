use crate::structs::config::variables::Variables;
use crate::structs::config::{get_item_by_variables_install_version, GetVersion};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/**
 * @description: 对vec Pattern 的包装，用于添加方法
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patterns(pub Vec<Pattern>);

impl Patterns {
    /**
     * @description: 调用所有patch的process方法，通过版本号取出对应的特征码
     */
    pub fn process(&self, variables: &Variables) -> Result<&Pattern> {
        get_item_by_variables_install_version(&self.0, variables)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pattern {
    pub version: String, //版本号
    pub pattern: String, //匹配的特征码
    pub replace: String, //替换的字符
    #[serde(default)]
    pub description: String, //描述
    #[serde(default)]
    pub disabled: bool, //是否禁用
}

impl GetVersion for Pattern {
    fn get_version(&self) -> &str {
        &self.version
    }
}
