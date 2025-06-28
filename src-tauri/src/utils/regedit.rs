use anyhow::{anyhow, Result};
use windows_registry::{LOCAL_MACHINE,CLASSES_ROOT,CURRENT_USER,USERS,CURRENT_CONFIG};


/**
 * @description: 读取注册表
 * @param path 注册表路径
 * @param field 注册表字段
 * @return 注册表值
 */
pub fn read_regedit(path: &str, field: &str) -> Result<String> {
    if path.is_empty() {
        return Err(anyhow!("读取注册表路径不能为空"));
    }
    if field.is_empty() {
        return Err(anyhow!("读取注册表字段不能为空"));
    }
    let paths: Vec<&str> = path.split("\\").collect();
    let p:&str = &paths[0].to_ascii_uppercase();
    let p2 = paths[1..].join("\\");
    let regedit = match p {
        "HKEY_CURRENT_USER" => CURRENT_USER.open(p2)?,
        "HKEY_CLASSES_ROOT" => CLASSES_ROOT.open(p2)?,
        "HKEY_USERS"=> USERS.open(p2)?,
        "HKEY_CURRENT_CONFIG"=> CURRENT_CONFIG.open(p2)?,
        "HKEY_LOCAL_MACHINE" => LOCAL_MACHINE.open(p2)?,
        _ => return Err(anyhow!("注册表路径无效"))
    };
    let value = regedit
        .get_string(field)
        .map_err(|_| anyhow!("读取注册表字段失败"))?;
    Ok(value)
}