use crate::structs::config::{paths::{PathResultItem, PathsResult}, substitute_variables};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/**
 * @description: 对vec Variable 的包装，用于添加方法
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variables(pub Vec<Variable>);
impl Variables {
    /**
     * @description: 调用所有 variable 的 process 方法
     */
    pub fn process(&mut self, variables: Variables) -> Result<()> {
        let _ = self.0
            .iter_mut()
            .try_for_each(|variable| variable.process(&variables));
        self.0.extend(variables.0);
        Ok(())
    }

    /**
     * @description: 通过 code 获取 variable 的值
     */
    pub fn get_value(&self, code: &str) -> Option<&String> {
        self.0.iter().find(|x| x.code == code).map(|x| &x.value)
    }
}

impl From <PathsResult> for Variables {
    fn from(paths_result: PathsResult) -> Variables {
        let mut result: Vec<Variable> = Vec::new();
        paths_result.0.iter().for_each(|path_result_item|{
            result.push(path_result_item.into());
        });
        Variables(result)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Variable {
    pub code: String,  //变量代码
    pub value: String, //变量值
    #[serde(default)]
    pub name: String, //变量名称
    #[serde(default)]
    pub description: String, //变量描述
}

impl Variable {
    pub fn new(code: &str, value: &str) -> Self {
        Self {
            code: code.to_string(),
            value: value.to_string(),
            name: "".to_string(),
            description: "".to_string(),
        }
    }

    /**
     * @description: 通过传入的变量，替换变量的值
     */
    pub fn process(&mut self, variables: &Variables) -> Result<()> {
        self.value = substitute_variables(&self.value, variables);
        Ok(())
    } 
}

impl From<&PathResultItem> for Variable {
    fn from(item: &PathResultItem) -> Self {
        Self::new(&item.code, &item.value)
    }
}
