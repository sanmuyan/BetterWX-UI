use crate::structs::config::features::Features;
use crate::structs::config::patches::Patches;
use crate::structs::config::regedit::Regedit;
use crate::structs::config::variables::{Variable, Variables};
use crate::structs::config::GetCode;
use crate::structs::config::{ismain, str_to_hex};
use crate::structs::files_info::{FileInfo, FilesInfo};
//use crate::structs::config::{get_item_by_code, get_mut_item_by_code, ismain, str_to_hex};
use crate::win::{del_files, filter_files_is_exists};

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};

/**
 * @description: 对vec Rule 的包装，用于添加方法
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Rules(pub Vec<Rule>);

impl Rules {
    /**
     * @description: 校验 rule 的 code 唯一性
     */
    pub fn check_code(&self) -> Result<()> {
        let mut codes = std::collections::HashSet::new();
        for rule in &self.0 {
            if !codes.insert(&rule.code) {
                return Err(anyhow!("发现重复的规则 code: {}", rule.code));
            }
        }
        Ok(())
    }
    /**
     * @description: 调用所有rule的process方法
     */
    pub fn process(&mut self, features: &Features) -> Result<()> {
        self.check_code()?;
        self.0
            .iter_mut()
            .try_for_each(|rule| rule.process(features))
    }

    // /**
    //  * @description: 通过 code 获取 rule 的可变引用
    //  */
    // pub fn get_mut_rule_by_code(&mut self, code: &str) -> Result<&mut Rule> {
    //     get_mut_item_by_code(&mut self.0, code)
    // }

    // /**
    //  * @description: 通过 code 获取 rule 的不可变引用
    //  */
    // pub fn get_rule_by_code(&mut self, code: &str) -> Result<&Rule> {
    //     get_item_by_code(&self.0, code)
    // }
}

/**
 * @description: 规则配置
 */
#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub code: String,    //规则代码
    pub version: String, //版本号
    pub name: String,    //规则名称 ，用于显示在前端 tab 上
    #[serde(default)]
    pub index: i32, //规则索引，用于排序
    #[serde(default)]
    pub description: String, //规则描述
    #[serde(default)]
    pub disabled: bool, //是否禁用
    #[serde(default)]
    pub supported: bool, //是否支持
    #[serde(default)]
    pub installed: bool, //是否安装
    pub regedit: Regedit, //注册表配置
    pub variables: Variables, //变量配置
    pub patches: Patches, //补丁配置
    pub features: Features, //功能配置
}

impl Rule {
    /**
     * @description: 调用所有rule的process方法
     */
    pub fn process(&mut self, features: &Features) -> Result<()> {
        //处理注册表字段
        if let Err(err) = self.regedit.process() {
            //todo 取出错误文本
            println!("未安装: {:?}", err);
            return Ok(());
        }
        println!("rule.regedit 处理完成: {:?}", self.regedit.fields);
        //读取注册表字段成功，表示已安装
        self.installed = true;
        //将当前注册表字段作为变量，用于后续处理
        let variables = &self.regedit.fields;
        println!("已经安装: {:?}", self.code);
        //处理rule中的variables,使用注册表字段对其替换
        self.variables.process(variables)?;
        //注册表字段作为变量混入到rule.variables
        self.variables.0.extend(variables.0.clone());
        //处理补丁
        self.patches.process(&self.variables)?;
        //处理功能
        self.features
            .process(&self.variables, &mut self.patches, Some(features))?;
        //过滤下当前版本使用的补丁
        self.patches.retain_patches_by_featrues(&self.features)?;
        //判断所有self.patches是否全部为 suppoted
        let all_supported = self.patches.0.iter().all(|patch| patch.supported);
        //初始化完成，设置支持状态
        self.supported = all_supported;
        Ok(())
    }

    /**
     * @description: 检查是否可用
     */
    fn check_status_available(&self) -> Result<()> {
        let name = if self.name.is_empty() {
            &self.name
        } else {
            &self.code
        };
        if self.disabled {
            return Err(anyhow!("已禁用: {:?}", name));
        }
        if !self.installed {
            return Err(anyhow!("未安装: {:?}", name));
        }
        Ok(())
    }

    /**
     * @description: 构建所有可能文件信息列表
     * @return {*} 返回存在的文件信息列表
     */
    pub fn build_files_info(&self) -> Result<FilesInfo> {
        self.check_status_available()?;
        let mut files_info = Vec::new();
        //判断是否存在共存功能
        let max = if let Some(_) = self.features.get_feature_detail_by_code("coexist") {
            10
        } else {
            0
        };
        //遍历 -1 到 10，构建文件信息, -1 为主程序
        for index in -1..max {
            let file_info = self.build_file_info_by_num(index)?;
            //如果文件不存在，则跳过
            let (all_exists, filter_files) = filter_files_is_exists(&file_info.usedfiles);
            //避免旧版本共存文件对版本更新后影响
            //如果没有全部存在，并且不是主程序，则删除文件
            if !all_exists && index != -1 {
                let _ = del_files(filter_files);
            } else {
                //否则添加到files_info中
                files_info.push(file_info);
            }
        }
        Ok(FilesInfo::new(files_info))
    }

    /**
     * @description: 构建指定文件信息
     */
    pub fn build_file_info_by_num(&self, index: i32) -> Result<FileInfo> {
        self.check_status_available()?;
        let rule = self;
        //克隆一份rule，用于后续处理
        let variables = &mut rule.variables.clone();
        //如果index < 0, 则表示主程序，否则表示共存
        let num = if index < 0 {
            "z".to_string()
        } else {
            index.to_string()
        };
        //将num 转为 u8 数组
        let num_u8 = str_to_hex(&num);
        //将num 和 num_u8 作为变量，用于后续处理
        let variable_num = Variable::new("num", &num);
        let variable_num_u8 = Variable::new("num_u8", &num_u8);
        variables.0.push(variable_num);
        variables.0.push(variable_num_u8);
        let ismain = ismain(&num);
        //设置名称
        let name = if ismain {
            "主程序".to_string()
        } else {
            format!("共存-{num}")
        };
        //处理变量中的 num 和 num_u8
        let mut patches = rule.patches.clone();
        patches.process(&variables)?;
        let mut features = rule.features.clone();
        features.process(&variables, &mut patches, None)?;
        //获取所有使用的文件
        let usedfiles = patches.get_used_files();
        Ok(FileInfo::new(
            index, num, ismain, name, patches, features, usedfiles,
        ))
    }
}

impl GetCode for Rule {
    fn get_code(&self) -> &str {
        &self.code
    }
}
