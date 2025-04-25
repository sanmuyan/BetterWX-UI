use crate::structs::config::variables::Variables;
use crate::win::get_install_variables;

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/**
 * @description: 注册表配置
 * @param {*} path 注册表路径
 * @param {*} fields 注册表字段
 * @return {*}
 */
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Regedit {
    pub path: String,
    pub fields: Variables,
}

impl Regedit {
    /**
     * @description: 对所有字段进行处理，获取必要的字段
     */
    pub fn process(&mut self) -> Result<()> {
        if self.path.is_empty() {
            return Err(anyhow!("缺少必要注册表字段: path"));
        }
        println!("处理注册表: {:?}", self);
        // 去注册表获取字段内容
        get_install_variables(self)?;
        //检查是否缺少必要字段
        self.check_fleids("install_location")
    }

    /**
     * @description: 检查是否缺少必要字段
     */
    fn check_fleids(&self, code: &str) -> Result<()> {
        self.fields
            .get_value(code)
            .ok_or_else(|| anyhow!("缺少必要注册表字段: {}", { code }))?;
        Ok(())
    }
}
