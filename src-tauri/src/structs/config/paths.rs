use std::path::Path;

use crate::{structs::config::variables::Variables, utils::regedit::read_regedit};
use crate::utils::tools::extract_variables;
use crate::utils::file::get_file_version;
use crate::utils::process::get_runtime_path;

use anyhow::{anyhow, Result};
use log::debug;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathsResult(pub Vec<PathResultItem>);

impl PathsResult {
    pub fn to_values(&self) -> Vec<Value> {
        self.0
            .iter()
            .map(|item| serde_json::to_value(item).unwrap_or_default())
            .collect()
    }

    pub fn get_value_by_code(&self, code: &str) -> Result<String> {
        self.0
            .iter()
            .find(|item| item.code == code)
            .map(|item| item.value.clone())
            .ok_or(anyhow!("无法获取安装目录，请尝试放置对应程序目录内"))
    }

    fn create_item_by_path_item(
        &self,
        value: &str,
        path_item: &PathItem,
    ) -> Result<PathResultItem> {
        let mut item = PathResultItem::default();
        item.value = value.to_string();
        item.path = if path_item.path.is_empty() {
            value.to_string()
        } else {
            calc_string(&path_item.path, &item.to_value(), &self.to_values())
        };
        item.file = calc_string(&path_item.file, &item.to_value(), &self.to_values());
        Ok(item)
    }

    fn update(&mut self, path_result_item: PathResultItem) {
        let index = self
            .0
            .iter()
            .position(|item| item.code == path_result_item.code);
        if let Some(index) = index {
            self.0[index] = path_result_item;
        } else {
            self.0.push(path_result_item);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathResultItem {
    pub code: String,
    pub name: String,
    pub description: String,
    pub value: String,
    pub path: String,
    pub file: String,
    pub status: bool,
}

impl PathResultItem {
    pub fn default() -> Self {
        Self {
            value: String::new(),
            path: String::new(),
            file: String::new(),
            code: String::new(),
            name: String::new(),
            description: String::new(),
            status: false,
        }
    }
    pub fn to_value(&self) -> Value {
        serde_json::to_value(self).unwrap_or_default()
    }
    pub fn check_file(&self) -> bool {
        return Path::new(&self.file).exists();
    }

    pub fn fill_by_path_item(&mut self, path_item: &PathItem) {
        self.description = path_item.description.to_string();
        self.name = path_item.name.to_string();
        self.code = path_item.code.to_string();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Paths(pub Vec<PathItem>);

impl Paths {
    pub fn get_all_path(
        &self
    ) -> Result<Variables> {
        let mut paths_result = PathsResult(Vec::new());
        for path_item in &self.0 {
            if !path_item.do_methods(&mut paths_result)? {
                break;
            }
        }
        debug!("所有获取路径结果: {:?}", paths_result);
        Ok(paths_result.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathItem {
    pub code: String,
    pub index: usize,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    pub methods: PathMethods,
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub file: String,
}

impl PathItem {
    pub fn do_methods(&self, paths_result: &mut PathsResult) -> Result<bool> {
        let ret_value = paths_result.get_value_by_code(&self.code);
        if let Ok(ret_value) = ret_value {
            return Ok(self.check_value(&ret_value, paths_result)?);
        }
        for path_method in &self.methods.0 {
            let value = match paths_result.get_value_by_code(&self.code) {
                Ok(result_value) if result_value.is_empty() => {
                    path_method.do_method(paths_result)
                }
                Ok(result_value) => result_value.to_string(),
                Err(_) => path_method.do_method(paths_result),
            };
            if self.check_value(&value, paths_result)? {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn check_value(
        &self,
        value: &str,
        paths_result: &mut PathsResult
    ) -> Result<bool> {
        if !value.is_empty() {
            let mut path_result_item = paths_result.create_item_by_path_item(&value, self)?;
            if path_result_item.check_file() {
                path_result_item.fill_by_path_item(&self);
                path_result_item.status = true;
                paths_result.update(path_result_item);
                return Ok(true);
            }
        }
        Ok(false)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathMethods(pub Vec<PathMethod>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathMethod {
    pub method: PathMethodType,
    pub index: usize,
    #[serde(default)]
    pub args: Option<PathMethodArgs>,
    #[serde(default)]
    pub retry: usize,
    #[serde(default)]
    pub unprefix: String,
    #[serde(default)]
    pub unsuffix: String,
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub suffix: String,
}

impl PathMethod {
    pub fn do_method(
        &self,
        paths_result: &mut PathsResult
    ) -> String {
        let retry = if self.retry < 1 { 1 } else { self.retry };
        for _ in 0..retry {
            let r = match self.method {
                PathMethodType::RunTime => self.get_path_by_runtime(),
                PathMethodType::Regedit => self.get_path_by_regedit(paths_result),
                PathMethodType::FileInfo => self.get_path_by_fileinfo(paths_result),
                PathMethodType::Calculate => self.get_path_by_calculate(paths_result)
            };
            if let Ok(r) = r {
                return self.fix_result(r);
            }
        }
        String::new()
    }

    fn chech_args(&self, fields: &[&str]) -> Result<()> {
        let args = self
            .args
            .as_ref()
            .ok_or_else(|| anyhow!("获取路径方法缺失必要的参数 args"))?;
        args.check_field(fields)
    }

    fn fix_result(&self, result: String) -> String {
        let mut result = result;
        if !self.unprefix.is_empty() {
            let index = result.find(&self.unprefix).unwrap_or(0);
            if index > 0 {
                result = result[index + self.unprefix.len()..].to_string();
            }
        }
        if !self.unsuffix.is_empty() {
            let index = result.find(&self.unsuffix).unwrap_or(0);
            if index > 0 {
                result = result[..index].to_string();
            }
        }
        if !self.prefix.is_empty() {
            result = format!("{}{}", self.prefix, result);
        }
        if !self.suffix.is_empty() {
            result = format!("{}{}", result, self.suffix);
        }
        debug!("修正后的路径 fix_result: {}", result);
        result
    }

    fn get_path_by_calculate(&self, paths_result: &mut PathsResult) -> Result<String> {
        self.chech_args(&["value"])?;
        let args = self.args.as_ref().unwrap().calc_string(paths_result);
        debug!("获取到计算路径 get_path_by_calculate: {}", args.value);
        Ok(args.value)
    }

    fn get_path_by_fileinfo(&self, paths_result: &mut PathsResult) -> Result<String> {
        self.chech_args(&["path"])?;
        let args = self.args.as_ref().unwrap().calc_string(paths_result);
        debug!("获取到文件路径 get_path_by_fileinfo: {}", &args.path);
        get_file_version(&args.path)
    }

    fn get_path_by_runtime(&self) -> Result<String> {
        let path = get_runtime_path()?;
        debug!("获取到运行所在路径 get_path_by_runtime: {}", path);
        Ok(path)
    }

    fn get_path_by_regedit(&self, paths_result: &mut PathsResult) -> Result<String> {
        self.chech_args(&["path", "field"])?;
        let args = self.args.as_ref().unwrap().calc_string(paths_result);
        if let Ok(value) = read_regedit(&args.path, &args.field) {
            debug!("获取到注册表路径 get_path_by_regedit: {}", value);
            return Ok(value);
        }
        Ok(String::new())
    }

}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PathMethodType {
    RunTime,
    Calculate,
    FileInfo,
    Regedit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathMethodArgs {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub value: String,
}

impl PathMethodArgs {
    pub fn calc_string(&self, paths_result: &PathsResult) -> Self {
        let paths_result = paths_result.to_values();
        let self_item = json!({});
        let path = calc_string(&self.path, &self_item, &paths_result);
        let field = calc_string(&self.field, &self_item, &paths_result);
        let value = calc_string(&self.value, &self_item, &paths_result);
        Self { path, field, value }
    }

    pub fn check_field(&self, fields: &[&str]) -> Result<()> {
        if fields.is_empty() {
            return Ok(());
        }
        let args_value = serde_json::to_value(self)?;
        if let Some(obj) = args_value.as_object() {
            for field in fields {
                match obj.get(*field) {
                    Some(Value::String(s)) if s.is_empty() => {
                        return Err(anyhow!("获取路径该方法缺失必要的参数: args.{}", field));
                    }
                    None => {
                        return Err(anyhow!("获取路径该方法参数不存在: args.{}", field));
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

fn calc_string(temp_string: &str, self_item: &Value, self_list: &Vec<Value>) -> String {
    let mut temp_string = temp_string.to_string();
    let variables = extract_variables(&temp_string);
    for var in variables {
        if let Some(ret_val) = if var.contains('.') {
            get_value_from_list(&var, self_list)
        } else {
            get_value_from_self(&var, self_item)
        } {
            let from = &format!("${{{}}}", var);
            temp_string = temp_string.replace(from, &ret_val);
        };
    }
    temp_string
}

fn get_value_from_list(var: &str, self_list: &Vec<Value>) -> Option<String> {
    let var_parts: Vec<&str> = var.split('.').collect();
    let code = var_parts[0];
    let field = var_parts[1];
    for item in self_list {
        //遍历比较 code 值
        if let Some(item_code) = item.get("code") {
            if code == item_code.as_str().unwrap_or("") {
                //如果 code 值相等，获取 field 值
                if let Some(v) = item.get(field) {
                    return Some(v.as_str().unwrap_or("").to_string());
                }
                return Some(String::new());
            }
        }
    }
    None
}

fn get_value_from_self(field: &str, self_item: &Value) -> Option<String> {
    if let Some(v) = self_item.get(field) {
        return Some(v.as_str().unwrap_or("").to_string());
    }
    None
}
