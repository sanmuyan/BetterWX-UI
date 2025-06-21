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
        for address in patch.addresses.iter() {
            // 设置 应用补丁的数据，是还原还是打补丁
            let patch_str = if patch.status {
                &patch.replace
            } else {
                &address.origina
            };
            println!(
                "应用补丁:{} status:{} patch_str {}",
                patch.code, patch.status, patch_str
            );
            let patch_bytes = hex_decode_to_vec(patch_str)?;
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
        std::fs::write(path, data).map_err(|_| anyhow!("保存文件失败，请先关闭所有WX程序"))?;
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
        // 搜索模式
        if patch.addresses.is_empty() {
            let path: &str = &patch.target;
            let mut patched = false;
            let file_data = file_cache
                .entry(patch.target.to_string())
                .or_insert_with(|| std::fs::read(&path).unwrap_or_else(|_| Vec::new()));
            //读取文件失败
            if file_data.is_empty() {
                return Err(anyhow!("读取文件失败:{}", &path));
            }
            if patch.searched {
                return Ok(());
            }
            patch.searched = true;
            let file_data_str = file_cache_str
                .entry(patch.target.to_string())
                .or_insert_with(|| hex_string(file_data));
            //首次搜索原始特征码
            if patch.pattern.is_empty() {
                return Err(anyhow!("待匹配特征码无效:{} ", &patch.code));
            }
            let mut search_result =
                hex_search(file_data, file_data_str, &patch.pattern, patch.multiple)?;
            //如果没有找到，再搜索替换特征码
            if !search_result.0 && !patch.replace.is_empty() {
                patched = true;
                search_result =
                    hex_search(file_data, file_data_str, &patch.replace, patch.multiple)?
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
                patch.patched = patched;
                patch.status = patched;
            }
            println!(
                "搜索结果: {} - code:{}  - patched:{} - supported:{}, addesses: {:?}",
                found, &patch.code, patch.patched, patch.supported, patch.addresses
            );
        } else {
            // 读取模式
            let path: &str = patch.get_exists_path();
            println!("读取模式:{}", &path);
            let file_data = file_cache
                .entry(path.to_string())
                .or_insert_with(|| std::fs::read(&path).unwrap_or_else(|_| Vec::new()));
            //读取文件失败
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

fn hex_search(
    data: &[u8],
    data_text: &str,
    reg_text: &str,
    multiple: bool,
) -> Result<(bool, String, Vec<Address>)> {
    if is_reg_pattern(reg_text) {
        return text_search(data_text, reg_text, multiple);
    } else {
        return sunday_search(data, reg_text, multiple);
    }
}

/**
 * 在十六进制数据中搜索指定特征码
 * @param data 要搜索的十六进制字符串
 * @param reg_text 要匹配的特征码
 *
 * @return 返回一个元组
 */
fn text_search(data: &str, reg_text: &str, multiple: bool) -> Result<(bool, String, Vec<Address>)> {
    let reg =
        Regex::new(&reg_text.to_ascii_lowercase()).map_err(|e| anyhow!("特征码错误 {}", e))?;
    let mut result = Vec::new();
    let mut origina = String::new();
    let captures: Vec<_> = reg.captures_iter(data).collect();
    //添加对多个地址的支持
    // 如果不允许多个地址，且找到多个 提前返回
    if captures.len() == 1 || (multiple && captures.len() > 1) {
        for capture in captures {
            if let Some(matched) = capture.get(capture.len() - 1) {
                let start = matched.start() / 2;
                let end = matched.end() / 2;
                let len = end - start;
                origina = matched.as_str().to_string();
                result.push(Address::new(start, end, len, origina.clone()));
            }
        }
    }
    Ok((!result.is_empty(), origina, result))
}

/**
 * 使用Sunday算法在u8数组中搜索模式
 * @param data 要搜索的u8数组
 * @param pattern 要匹配的模式字符串，支持.通配符
 * @param multiple 是否允许多个匹配结果
 * @return 返回一个元组(是否找到匹配, 匹配的原始数据, 匹配的地址列表)
 */
pub fn sunday_search(
    data: &[u8],
    pattern: &str,
    multiple: bool,
) -> Result<(bool, String, Vec<Address>)> {
    let mut result = Vec::new();
    let mut origina = String::new();
    // 将hex字符串转换为字节序列
    let hex_bytes = pattern.replace("..", "00").replace("??", "00");
    let pattern_bytes = hex_decode_to_vec(&hex_bytes)
        .map_err(|e| anyhow!("无效的hex模式字符串: {}", e))?;
    
    let pattern_len = pattern_bytes.len();
    let data_len = data.len();

    if pattern_len == 0 || data_len < pattern_len {
        return Ok((false, origina, result));
    }

    // 预处理坏字符跳转表
    let mut shift = [pattern_len + 1; 256];
    for (i, &b) in pattern_bytes.iter().enumerate() {
        shift[b as usize] = pattern_len - i;
    }

    let mut i = 0;
    while i <= data_len - pattern_len {
        // 尝试匹配模式
        let mut matched = true;
        for (j, &p) in pattern_bytes.iter().enumerate() {
            // 处理通配符
            let pattern_char = pattern.chars().nth(j * 2).unwrap_or('\0');
            if pattern_char != '.' && pattern_char != '?' && data[i + j] != p {
                matched = false;
                break;
            }
        }

        if matched {
            // 记录匹配结果
            let start = i;
            let end = i + pattern_len;
            let matched_bytes = &data[start..end];
            origina = hex_string(matched_bytes);
            result.push(Address::new(start, end, pattern_len, origina.clone()));

            if !multiple {
                break;
            }
            i += pattern_len;
        } else {
            // Sunday算法跳转
            if i + pattern_len < data_len {
                let next_char = data[i + pattern_len];
                i += shift[next_char as usize];
            } else {
                break;
            }
        }
    }

    Ok((!result.is_empty(), origina, result))
}

fn is_reg_pattern(reg_text: &str) -> bool {
    !reg_text
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '?')
}
