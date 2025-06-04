use crate::structs::config::patches::Patches;
use crate::structs::config::rules::Rule;
use crate::structs::config::Config;
use crate::structs::files_info::FilesInfo;

use crate::patch;
use crate::win;
use anyhow::Ok;
use anyhow::Result;

/**
 * @description: 预处理解析 config
 * @return {*}
 */
pub fn process_config(config: &mut Config) -> Result<()> {
    config.process()
}

/**
 * @description: 搜索基址
 * @return {*} 返回修补基址后的rule
 */
pub fn search_base_address(rule: &mut Rule) -> Result<()> {
    //构建文件信息
    let mut file_info = rule.build_file_info_by_num(-1)?;
    println!("----------------------------------------file_info: {:?}", file_info.usedfiles);
    //备份文件
    win::backup_files(file_info.usedfiles)?;
    //读取文件补丁信息
    patch::read_patches(&mut file_info.patches)?;
    //替换 config.rule 的 patches 为 base_patches，用于后续 build_file_info_by_num 时，使用 base_patches 来构建文件信息
    rule.patches
        .replace_patches_by_base_patches(&file_info.patches)?;
    //根据 patches 修复 feature 功能状态
    rule.features
        .fix_features_by_base_patches(&rule.variables, &mut rule.patches)?;
    Ok(())
}

/**
 * @description: 刷新文件信息
 * @return {*} 返回s
 */
pub fn refresh_files_info(rule: &Rule) -> Result<FilesInfo> {
    let mut files_info = rule.build_files_info()?;
    for file_info in files_info.0.iter_mut() {
        //读取文件补丁信息
        patch::read_patches(&mut file_info.patches)?;
        //修复feature状态 by patches ,
        file_info
            .features
            .fix_features_by_base_patches(&rule.variables, &mut file_info.patches)?;
    }
    Ok(files_info)
}

/*
* @description: 应用补丁
* @return {*} 返回修补基址后的rule
*/
pub fn apply_patch(patches: &mut Patches) -> Result<()> {
    Ok(patch::apply_patch(patches)?)
}
