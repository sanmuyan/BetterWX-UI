use crate::structs::config::patches::{Address, Patches};
use crate::structs::config::{hex_decode_to_vec, replace_wildcards};

use anyhow::{anyhow, Result};
use faster_hex::hex_string;
use regex::Regex;
use std::collections::HashMap;

/**
 * 应用补丁数据
 * @param patches 包含所有补丁信息的可变引用
 * @return 成功返回Ok(()), 失败返回错误信息
 */
pub fn apply_patch(patches: &mut Patches) -> Result<()> {
    let mut file_cache: HashMap<String, Vec<u8>> = HashMap::new();
    // 先验证所有补丁数据
    for patch in patches.0.iter() {
        if patch.addresses.is_empty() {
            return Err(anyhow!("修补文件缺少基址信息:{}", &patch.name));
        }
    }
    // 应用补丁
    for patch in patches.0.iter_mut() {
        if !patch.supported || patch.disabled {
            continue;
        }
        let path = patch.get_exists_path();
        println!("应用补丁操作的文件:{}", &path);
        //缓存文件数据
        let file_data = file_cache
            .entry(patch.saveas.to_string())
            .or_insert_with(|| std::fs::read(&path).unwrap_or_else(|_| Vec::new()));
        //读取文件失败
        if file_data.is_empty() {
            return Err(anyhow!("读取文件失败:{}", &path));
        }
        // 设置 应用补丁的数据，是还原还是打补丁
        let patch_str = if patch.status {
            &patch.replace
        } else {
            &patch.pattern
        };
        let patch_bytes = hex_decode_to_vec(patch_str)?;
        println!("应用补丁:{} status:{} patch_str:{}", patch.code,patch.status,patch_str);
        for address in patch.addresses.iter() {
            // 检查是否超出文件长度
            if address.end > file_data.len() || patch_bytes.len() != address.len {
                return Err(anyhow!(
                    "修补文件基址信息和补丁文件长度不一致: {}",
                    &patch.name
                ));
            }
            //修补指定区域的数据
            file_data[address.start..address.end].copy_from_slice(&patch_bytes);
            //patch.patched = status;
        }
        patch.patched = patch.status;
    }
    // 保存修改后的文件
    for (path, data) in file_cache {
        println!("应用补丁保存的文件:{}", &path);
        std::fs::write(path, data).map_err(|_| anyhow!("保存文件失败，文件被占用，或者以管理员模式启动"))?;
    }
    Ok(())
}

/**
 * 搜索补丁数据
 * @param patches 包含所有补丁信息的可变引用
 * @return 成功返回Ok(()), 失败返回错误信息
 */
pub fn read_patches(patches: &mut Patches) -> Result<()> {
    let mut file_cache: HashMap<String, Vec<u8>> = HashMap::new();
    let mut file_cache_str: HashMap<String, String> = HashMap::new();

    for patch in patches.0.iter_mut() {
        if !patch.supported || patch.disabled {
            continue;
        }
        let path = patch.get_exists_path();

        let file_data = file_cache
            .entry(patch.saveas.to_string())
            .or_insert_with(|| std::fs::read(&path).unwrap_or_else(|_| Vec::new()));
        //读取文件失败
        if file_data.is_empty() {
            return Err(anyhow!("读取文件失败:{}", &path));
        }

        // 搜索模式
        if patch.addresses.is_empty() {
            if patch.searched {
                return Ok(());
            }
            patch.searched = true;
            let file_data_str = file_cache_str
                .entry(patch.saveas.to_string())
                .or_insert_with(|| hex_string(file_data));
            //首次搜索原始特征码
            if patch.pattern.is_empty() {
                return Err(anyhow!("待匹配特征码无效:{} ", &patch.code));
            }
            let mut search_result = hex_search(file_data_str, &patch.pattern, patch.multiple)?;
            //如果没有找到，再搜索替换特征码
            if !search_result.0 && !patch.replace.is_empty() {
                search_result = hex_search(file_data_str, &patch.replace, patch.multiple)?
            }

            let (found, origina, addresses) = search_result;
            patch.supported = found;
            // 根据搜索结果修改补丁信息
            if found {
                patch.addresses = addresses;
                patch.origina = origina;
                //修复通配符 . 返回的前台
                patch.replace = replace_wildcards(&patch.replace, &patch.origina)?;
                patch.pattern = replace_wildcards(&patch.pattern, &patch.origina)?;
                patch.patched = &patch.pattern != &patch.pattern;
                patch.status = patch.patched;
            }
            println!(
                "搜索结果: {} - code:{}  - patched:{} - supported:{}, addesses: {:?}",
                found, &patch.code, patch.patched, patch.supported, patch.addresses
            );
           
        } else {
            // 读取模式
            let mut patched = true;
            for address in patch.addresses.iter_mut() {
                let slice = &file_data[address.start..address.end];
                let hex_str = hex_string(slice);
                patched = patched
                    && match hex_str {
                        _ if hex_str != patch.pattern => true,
                        _ => false,
                    };
            }
            patch.patched = patched;
            patch.status = patched;
            println!("读取模式: {} - code:{}", patch.patched, patch.code)
        }
    }
    Ok(())
}

/**
 * 在十六进制数据中搜索指定特征码
 * @param data 要搜索的十六进制字符串
 * @param reg_text 要匹配的特征码
 *
 * @return 返回一个元组
 */
fn hex_search(data: &str, reg_text: &str, multiple: bool) -> Result<(bool, String, Vec<Address>)> {
    let reg =
        Regex::new(&reg_text.to_ascii_lowercase()).map_err(|e| anyhow!("特征码错误 {}", e))?;
    let mut result = Vec::new();
    let mut origina = String::new();
    let captures: Vec<_> = reg.captures_iter(data).collect();
    //添加对多个地址的支持
    // 如果不允许多个地址，且找到多个 提前返回
    if captures.len() ==1 || (multiple && captures.len() > 1) {
        for capture in captures {
            if let Some(matched) = capture.get(capture.len()-1) {
                let start = matched.start() / 2;
                let end = matched.end() / 2;
                let len = end - start;
                origina = matched.as_str().to_string();
                result.push(Address::new(start, end, len));
            }
        }
    }
    Ok((!result.is_empty(),origina,result))
}
