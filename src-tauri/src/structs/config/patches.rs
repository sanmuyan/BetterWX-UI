use crate::structs::config::patterns::Patterns;
use crate::structs::config::variables::Variables;
use crate::structs::config::features::Features;
use crate::structs::config::{
    get_item_by_code, ismain, replace_ellipsis, replace_wildcards, substitute_variables, GetCode,
};
use crate::win::is_file_exists;

use anyhow::Result;
use serde::{Deserialize, Serialize};

/**
 * @description: 对vec Patch 的包装，用于添加方法
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patches(pub Vec<Patch>);

impl Patches {
    /**
     * @description: 调用所有patch的process方法
     * @param {*} variables 提供的可用的变量集合
     * @return {*}
     */
    pub fn process(&mut self, variables: &Variables) -> Result<()> {
        self.0
            .iter_mut()
            .try_for_each(|patch| patch.process(variables))?;
        //过滤掉不可用的patch，只保留可用的patch
        //不能过滤否则会导致 build_file_info 时，无法获取到usedfiles
        //以后再看看怎么修改
        //self.0.retain(|patch| patch.supported && !patch.disabled);
        Ok(())
    }
    /**
     * @description: 在搜索基址后，对 patches 状态进行更新
     */
    pub fn replace_patches_by_base_patches(&mut self, base_patches: &Patches) -> Result<()> {
        self.0
            .iter_mut()
            .try_for_each(|patch| patch.replace_patch_by_base_patches(base_patches))?;
        //过滤掉不可用的patch，只保留可用的patch
        //不能过滤否则会导致 build_file_info 时，无法获取到usedfiles
        //以后再看看怎么修改
        //self.0.retain(|patch| patch.supported && !patch.disabled);
        Ok(())
    }

   
    /**
     * @description: 在搜索基址后，对 featrues 状态进行更新
     */
    pub fn filter_patches_by_featrues(&mut self, features: &Features) -> Result<()> {
        let all_dependencies = features.extract_vec_string_dependencies();
        self.0.retain(|patch| all_dependencies.contains(&patch.code));
        Ok(())
    }

    /**
     * @description: //所有补丁需要使用的文件集合
     */
    pub fn get_used_files(&self) -> Vec<String> {
        let mut files: Vec<String> = self
            .0
            .iter()
            .map(|patch| patch.saveas.to_string())
            .collect();
        files.sort_unstable();
        files.dedup();
        files
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Patch {
    pub code: String,   //唯一标识
    pub target: String, //目标文件
    pub saveas: String, //保存为文件
    pub name: String,   //名称
    #[serde(default)]
    pub description: String, //描述
    #[serde(default)]
    pub disabled: bool, //是否禁用
    #[serde(default)]
    pub supported: bool, //是否支持
    pub patterns: Patterns, //匹配特征码集合
    #[serde(default)]
    pub pattern: String, //当前版本使用的特征码
    #[serde(default)]
    pub replace: String, //替换特征码
    #[serde(default)]
    pub multiple: bool, //是否多个地址
    #[serde(default)]
    pub addresses: Vec<Address>, // 一组基址信息
    //是否已经patched
    #[serde(default)]
    pub patched: bool, //是否已经patched
    //原始数据
    #[serde(default)]
    pub origina: String, //原始数据
    //是否已经patched
    #[serde(default)]
    pub searched: bool, //是否已经搜索过
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    //基址
    #[serde(default)]
    pub start: usize, //基址起始位置
    //长度
    #[serde(default)]
    pub len: usize, //基址长度
    //长度
    #[serde(default)]
    pub end: usize, //基址结束位置
}

impl Address {
    pub fn new(start: usize, end: usize, len: usize) -> Self {
        Self { start, end, len }
    }
}

impl Patch {
    /**
     * @description: 调用所有patch的process方法
     */
    pub fn process(&mut self, variables: &Variables) -> Result<()> {
        //如果禁用了，则直接返回
        if self.disabled {
            return Ok(());
        }
        //处理Patterns
        if let Ok(partten) = self.patterns.process(variables) {
            self.disabled = partten.disabled;
            self.supported = true;
            self.pattern = partten.pattern.to_string();
            self.replace = partten.replace.to_string();
        }
        self.patterns.0.clear();
        
        //处理 变量
        self.saveas = substitute_variables(&self.saveas, variables);
        self.target = substitute_variables(&self.target, variables);
        self.replace = substitute_variables(&self.replace, variables).to_lowercase();
        self.pattern = substitute_variables(&self.pattern, variables).to_lowercase();
        //build file_info 时使用
        if let Some(num) = variables.get_value("num") {
            if ismain(num) {
                //是主程序 替换 saveas 为 target
                self.saveas = self.target.to_string();
            }
            //修复省略号
            self.replace = replace_ellipsis(&self.replace, &self.pattern)?;
            //替换通配符?为...
            self.replace = self.replace.replace("?", ".");
            self.pattern = self.pattern.replace("?", ".");
            //修复通配符
            if !self.origina.is_empty() {
                self.pattern = replace_wildcards(&self.pattern, &self.origina)?;
                self.replace = replace_wildcards(&self.replace, &self.origina)?;
            }
        }
        Ok(())
    }

    /**
     * @description: 在搜索基址后，对 patch 信息进行更新，以便在后续构建文件信息时使用
     */
    pub fn replace_patch_by_base_patches(&mut self, base_patches: &Patches) -> Result<()> {
        let base_patch = get_item_by_code(&base_patches.0, &self.code)?.clone();
        self.supported = base_patch.supported;
        self.addresses = base_patch.addresses;
        self.origina = base_patch.origina.to_string();
        self.searched = base_patch.searched;
        Ok(())
    }

    /**
     * @description: 获取存在的路径，不存在则使用主程序路径
     */
    pub fn get_exists_path(&self) -> &str {
        if is_file_exists(&self.saveas) {
            &self.saveas
        } else {
            &self.target
        }
    }
}

impl GetCode for Patch {
    fn get_code(&self) -> &str {
        &self.code
    }
}
