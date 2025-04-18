/*
 * @Author: afaa1991
 * @Date: 2025-03-31 10:39:52
 * @LastEditors: afaa1991
 * @LastEditTime: 2025-03-31 19:00:11
 * @Description:
 */

use crate::structs::config::features::Features;
use crate::structs::config::patches::Patches;

use serde::{Deserialize, Serialize};

/**
 * 用于传递到前端的文件信息集合
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct FilesInfo(pub Vec<FileInfo>);

impl FilesInfo {
    pub fn new(files_info: Vec<FileInfo>) -> Self {
        Self(files_info)
    }
}

/**
 * 用于传递到前端的文件信息
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub index: i32,             // 用于前端排序
    pub num: String,            // 用于共存 num
    pub ismain: bool,           //是否是主程序
    pub name: String,           //用于显示的文件名
    pub patches: Patches,       //补丁信息集合
    pub features: Features,     //功能集合
    pub usedfiles: Vec<String>, //所有补丁需要使用的文件集合
}

impl FileInfo {
    pub fn new(
        index: i32,
        num: String,
        ismain: bool,
        name: String,
        patches: Patches,
        features: Features,
        usedfiles: Vec<String>,
    ) -> Self {
        Self {
            index,
            num,
            ismain,
            name,
            patches,
            features,
            usedfiles,
        }
    }
}
