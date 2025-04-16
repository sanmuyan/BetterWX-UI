pub mod features;
pub mod patches;
pub mod patterns;
pub mod regedit;
pub mod rules;
pub mod variables;

use crate::structs::config::features::Features;
use crate::structs::config::rules::Rules;
use crate::structs::config::variables::Variables;

use anyhow::{anyhow, Result};
use faster_hex::hex_decode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub version: String, //版本号
    #[serde(default)]
    pub description: String, //描述
    #[serde(default)]
    pub disabled: bool, //是否禁用
    #[serde(default)]
    pub supported: bool, //是否支持，好像没有什么用
    pub rules: Rules,    //规则配置集合
}

impl Config {
    /**
     * @description: config处理的入口，对所有rule的process方法
     */
    pub fn process(&mut self) -> Result<()> {
        //加载default features
        let features: Features = serde_json::from_str(DEFAULT_FEATURE)
            .map_err(|e| anyhow!("解析默认 features 失败:{}", e))?;
        // 调用所有rule的process方法
        self.rules.process(&features)
    }
}

/**
 * @description: 将十六进制字符串解码为字节向量
 * @param hex_str 要解码的十六进制字符串
 * @return 成功返回解码后的字节向量，失败返回错误信息
 */
pub fn str_to_hex(s: &str) -> String {
    s.bytes().map(|b| format!("{:02x}", b)).collect()
}

/**
 * @description: hex_decode_to_vec 函数
 * @param hex_str 要解码的十六进制字符串
 * @return 成功返回解码后的字节向量，失败返回错误信息
 */
pub fn hex_decode_to_vec(hex_str: &str) -> Result<Vec<u8>> {
    let mut vec = vec![0; hex_str.len() / 2];
    hex_decode(hex_str.as_bytes(), &mut vec).map_err(|_| anyhow!("字符串转为u8数组失败"))?;
    Ok(vec)
}

/**
 * @description: 替换字符串中的通配符
 * @param str1 要替换的字符串
 * @param str2 包含通配符的字符串
 * @return 替换后的字符串
 */
pub fn replace_wildcards(wildcard_str: &str, origina: &str) -> Result<String> {
    if wildcard_str.is_empty() {
        return Ok(wildcard_str.to_string());
    }
    // 确保两个字符串长度相同
    if origina.is_empty() {
        return Err(anyhow!("替换通配符,原始字符为空"));
    }
    // 确保两个字符串长度相同
    if wildcard_str.len() != origina.len() {
        return Err(anyhow!("替换通配符,字符长度不一致"));
    }
    // 将str2转换为字符向量以便修改
    let mut wildcard_chars: Vec<char> = wildcard_str.chars().collect();
    // 遍历str2，替换通配符
    for (i, c) in wildcard_str.chars().enumerate() {
        if c == '.' {
            // 使用str1对应位置的字符替换
            wildcard_chars[i] = origina.chars().nth(i).unwrap();
        }
    }
    // 将字符向量转换回字符串
    Ok(wildcard_chars
        .into_iter()
        .collect::<String>())
}

/**
 * @description: 替换字符串中的省略号
 * @param str1 要替换的字符串
 * @param str2 包含省略号的字符串
 * @return 替换后的字符串
 */
fn replace_ellipsis(ellipsis_str: &str, origina: &str) -> Result<String> {
    // 确保两个字符串长度相同
    if !origina.is_empty() {
        if let Some(index) = ellipsis_str.find("...") {
            let l_text = &ellipsis_str[..index];
            let r_text = &ellipsis_str[index + 3..];
            let m_text = &origina[l_text.len()..&origina.len() - &r_text.len()];
            return Ok(format!("{}{}{}", l_text, m_text, r_text));
        }
    }
    Ok(ellipsis_str.to_string())
}

/**
 * @description: 判断是否时主程序
 */
fn ismain(num: &str) -> bool {
    num == "Z" || num == "z"
}

/**
 * @description: 从变量数组中获取 num 和 ismain
 */
fn get_num_and_ismain(variables: &Variables) -> (String, bool,bool) {
     // 检查是否包含 ${num} 变量
   let num = if let Some(num) = variables.get_value("num") {
      num
  } else {
      ""
  };
  let is_main = ismain(num);
  return (num.to_string(),num != "", is_main);
}

/**
 * @description: 替换字符串中的变量
 * @param str1 要替换的字符串 ${变量} 形式 ，和 js 保持一致
 * @param variables 包含变量的变量集合
 * @return 替换后的字符串
 */
fn substitute_variables(value: &str, variables: &Variables) -> String {
    let (_,_, is_main) = get_num_and_ismain(variables);
    let mut result = value.to_string();
    while let Some(start) = result.find("${") {
        if let Some(end) = result[start..].find('}') {
            let full_range = start..start + end + 1;
            let  var_name = &result[start + 2..start + end];
            if let Some(var) = variables.get_value(var_name) {
                //如果是主程序，且替换字符包含 num || num_u8
                if is_main && (var_name == "num" || var_name == "num_u8") {
                    result = "...".to_string();
                } else {
                    result.replace_range(full_range, &var);
                }
            } else {
                break;
            }
        } else {
            break;
        }
    }
    result
}

/**
 * @description: 比较两个版本号的大小
 * @param v1 第一个版本号
 * @param v2 第二个版本号
 * @return 如果 v1 小于 v2，返回 -1；如果 v1 大于 v2，返回 1；如果相等，返回 0。
 */
pub fn compare_versions(v1: &str, v2: &str) -> i32 {
    let v1_parts: Vec<&str> = v1.split('.').collect();
    let v2_parts: Vec<&str> = v2.split('.').collect();
    let max_len = std::cmp::max(v1_parts.len(), v2_parts.len());
    for i in 0..max_len {
        let num1 = v1_parts
            .get(i)
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        let num2 = v2_parts
            .get(i)
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        match num1.cmp(&num2) {
            std::cmp::Ordering::Less => return -1,
            std::cmp::Ordering::Greater => return 1,
            std::cmp::Ordering::Equal => continue,
        }
    }
    0
}

// /**
//  * @description: 通过 code 获取 item 的可变引用
//  */
// fn get_mut_item_by_code<'a, T>(vec_data: &'a mut Vec<T>, code: &str) -> Result<&'a mut T>
// where
//     T: GetCode,
// {
//     if let Some(rule) = vec_data.iter_mut().find(|t| t.get_code() == code) {
//         return Ok(rule);
//     }
//     Err(anyhow!("未找到匹配的项:{}", code))
// }


/**
 * @description: 通过 code 获取 item 的不可变引用
 */
fn get_item_by_code<'a, T>(vec_data: &'a Vec<T>, code: &str) -> Result<&'a T>
where
    T: GetCode,
{
    if let Some(rule) = vec_data.iter().find(|t| t.get_code() == code) {
        return Ok(rule);
    }
    Err(anyhow!("未找到匹配的项:{}", code))
}

/**
 * @description: 通过 install_version 获取当前安装版本支持的项
 */
fn get_item_by_variables_install_version<'a, T>(
    vec_data: &'a Vec<T>,
    variables: &Variables,
) -> Result<&'a T>
where
    T: GetVersion,
{
    // 需要 Clone 因为返回的是克隆值
    let install_version = variables
        .get_value("install_version")
        .ok_or_else(|| anyhow!("缺少必要变量: install_version"))?;
    if let Some(item) = vec_data
        .into_iter()
        .find(|item| compare_versions(install_version, item.get_version()) >= 0)
    {
        return Ok(item);
    } else {
        return Err(anyhow!("未找到匹配的版本的Item:{}", install_version));
    }
}

/**
 * @description: get_version，用于 get_item_by_version 使用
 */
trait GetVersion {
    fn get_version(&self) -> &str;
}

/**
 * @description: get_code，用于 get_item_by_code 使用
 */
trait GetCode {
    fn get_code(&self) -> &str;
}

/**
 * @description: 定义默认的 features 配置
 */
const DEFAULT_FEATURE: &str = r#"[
    {
      "code": "revoke",
      "name": "撤回",
      "method": "patch",
      "description": "调整防撤回状态",
      "inmain": true,
      "incoexist": true,
      "index": 6,
      "style": "switch",
      "disabled": false,
      "supported": true,
      "target": "",
      "dependencies": [
        "revoke"
      ]
    },
    {
      "code": "mutex",
      "name": "多开",
      "method": "patch",
      "description": "调整多开状态",
      "inmain": true,
      "incoexist": false,
      "index": 7,
      "style": "switch",
      "disabled": false,
      "supported": true,
      "target": "",
      "dependencies": [
        "mutex"
      ]
    },
    {
      "code": "coexist",
      "name": "共存",
      "method": "",
      "description": "制作共存文件",
      "inmain": true,
      "incoexist": false,
      "index": 5,
      "style": "button",
      "disabled": false,
      "supported": true,
      "target": "",
      "dependencies": [
        "mutex",
        "config",
        "host",
        "dllname"
      ]
    },
    {
      "code": "open",
      "name": "运行",
      "method": "",
      "description": "运行当前程序",
      "inmain": true,
      "incoexist": true,
      "index": 3,
      "style": "button",
      "disabled": false,
      "supported": true,
      "target": "${path_exe}",
      "saveas": "${new_path_exe}",
      "dependencies": [
        ""
      ]
    },
    {
      "code": "del",
      "name": "删除",
      "method": "",
      "description": "删除共存文件",
      "inmain": false,
      "incoexist": true,
      "index": 4,
      "style": "button",
      "disabled": false,
      "supported": true,
      "severity":"danger",
      "target": "",
      "dependencies": [
        ""
      ]
    },
    {
      "code": "floder",
      "name": "目录",
      "method": "",
      "description": "打开文件所在目录",
      "inmain": true,
      "incoexist": false,
      "index": 2,
      "style": "button",
      "disabled": false,
      "supported": true,
      "target": "${install_location}",
      "dependencies": [
        ""
      ]
    },
    {
      "code": "refresh",
      "name": "刷新",
      "method": "",
      "description": "重新读取所有文件状态",
      "inmain": true,
      "incoexist": false,
      "index": 1,
      "style": "button",
      "disabled": false,
      "supported": true,
      "target": "",
      "dependencies": [
        ""
      ]
    },
    {
      "code": "clear",
      "name": "清缓",
      "method": "",
      "description": "清除软件缓存",
      "inmain": true,
      "incoexist": false,
      "index": 0,
      "style": "button",
      "disabled": false,
      "supported": true,
      "target": "",
      "dependencies": [
        ""
      ]
    },
    {
      "code": "note",
      "name": "备注",
      "method": "",
      "description": "",
      "inmain": true,
      "incoexist": true,
      "index": 99,
      "style": "button",
      "disabled": false,
      "supported": true,
      "target": "",
      "dependencies": [
        ""
      ]
    }
  ]"#;
