use crate::structs::config::patches::Patches;
use crate::structs::config::variables::Variables;
use crate::structs::config::GetVersion;
use crate::structs::config::{
    get_item_by_variables_install_version, get_num_and_ismain, substitute_variables,
};

use anyhow::Result;
use serde::{Deserialize, Serialize};

/**
 * 对vec Feature 的包装，用于添加方法
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Features(pub Vec<Feature>);

impl Features {
    /**
     * @description: 调用所有feature的process方法
     * @param {*} variables 提供的可用的变量集合
     * @param {*} patches 提供的可用的patches集合，用于校验 features 的依赖关系，中是否存在 对应的 patch, 并将其 supported 字段设置为 true
     * @param {*} features 提供的默认的features集合，用于替换依赖关系中 string 类型的feature
     * @return {*}
     */
    pub fn process(
        &mut self,
        variables: &Variables,
        patches: &mut Patches,
        features: Option<&Features>,
    ) -> Result<()> {
        // 处理所有feature
        self.0
            .iter_mut()
            .try_for_each(|feature| feature.process(variables, patches, features))?;

        // 获取版本信息
        let (_, has_num, is_main) = get_num_and_ismain(variables);
        // 过滤feature
        self.0.retain(|feature| {
            let Feature::FeatureDetail(fd) = feature else {
                return false;
            };
            if has_num {
                if is_main {
                    fd.inmain
                } else {
                    fd.incoexist
                }
            } else {
                true
            }
        });
        Ok(())
    }

    // /**
    //  * @description: 对所有 dependencies 依赖的patches code 扁平化处理
    //  * @param {*} self
    //  * @return {*}
    //  */
    // pub fn extract_vec_string_dependencies(&self) -> Vec<String> {
    //     let mut unique_deps = HashSet::new();
    //     self.0
    //         .iter()
    //         .filter_map(|feature| match feature {
    //             Feature::FeatureDetail(fd) => {
    //                 if !fd.disabled && fd.supported {
    //                     match &fd.dependencies {
    //                         Dependencies::VecString(vec_str) => Some(vec_str.clone()),
    //                         _ => None,
    //                     }
    //                 } else {
    //                     None
    //                 }
    //             }
    //             _ => None,
    //         })
    //         .flatten()
    //         .for_each(|dep| {
    //             if !dep.is_empty() {
    //                 unique_deps.insert(dep);
    //             }
    //         });

    //     unique_deps.into_iter().collect()
    // }

    /**
     * @description: 通过 code 取出对应的 feature
     */
    pub fn get_feature_detail_by_code(&self, code: &str) -> Option<FeatureDetail> {
        let feature = self.0.iter().find(|feature| match feature {
            Feature::FeatureDetail(feature) => {
                feature.code == *code && feature.supported && !feature.disabled
            }
            Feature::String(s) => s == code,
        });
        match feature {
            Some(Feature::FeatureDetail(fd)) => Some(fd.clone()),
            _ => None,
        }
    }

    /**
     * @description: 通过基址信息，修复 patches 和 features 状态
     */
    pub fn fix_features_by_base_patches(
        &mut self,
        variables: &Variables,
        patches: &mut Patches,
    ) -> Result<()> {
        //处理功能
        self.process(variables, patches, None)?;
        //处理依赖
        //patches.filter_patches_by_featrues(&self)?;
        Ok(())
    }
}

/**
 * @description: 定义feature的结构，支持两种类型的feature，一种是string类型，一种是FeatureDetail类型，
 * 同时支持嵌套的Dependencies类型，其中Dependencies类型支持两种类型，一种是VecString类型，一种是VecDependency类型，
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Feature {
    String(String),
    FeatureDetail(FeatureDetail),
}

impl Feature {
    /**
     * @description: 调用所有feature的process方法
     * @param {*} variables 提供的可用的变量集合
     * @param {*} patches 提供的可用的patches集合，用于校验 features 的依赖关系，中是否存在 对应的 patch, 并将其 supported 字段设置为 true
     * @param {*} features 提供的默认的features集合，用于替换依赖关系中 string 类型的feature
     * @return {*}
     */
    pub fn process(
        &mut self,
        variables: &Variables,
        patches: &mut Patches,
        features: Option<&Features>,
    ) -> Result<()> {
        match self {
            // 替换依赖关系中 string 类型的feature
            Feature::String(code) => {
                //是否提供了默认的features集合，用于替换依赖关系中 string 类型的feature
                if let Some(features) = features {
                    // 从默认的features集合中取出对应的feature
                    if let Some(fd) = features.get_feature_detail_by_code(code) {
                        // 替换依赖关系中 string 类型的feature
                        *self = Feature::FeatureDetail(fd);
                    }
                }
            }
            //
            Feature::FeatureDetail(feature) => {
                // 如果禁用 则直接返回
                if feature.disabled {
                    return Ok(());
                }
                // 如果是VecString类型，则校验其依赖关系，中是否存在 对应的 patch, 并将其 supported 字段设置为 true
                if let Ok(dependency) = feature.dependencies.process(variables) {
                    feature.disabled = dependency.disabled;
                    feature.supported = true;
                    // 回写到 feature.dependencies 中
                    feature.dependencies = Dependencies::VecString(dependency.dependencies);
                }
            }
        }
        match self {
            Feature::FeatureDetail(feature) => {
                // 替换feature target 中的变量
                //如果不是主程序，需要替换 target 字段为 saveas 字段
                let (_, han_num, is_main) = get_num_and_ismain(variables);
                if han_num && !is_main && !feature.saveas.is_empty() {
                    feature.target = feature.saveas.clone();
                }
                feature.saveas = substitute_variables(&feature.saveas, variables);
                feature.target = substitute_variables(&feature.target, variables);
            }
            Feature::String(_) => {}
        }
        // 校验 feature 的依赖关系，中是否存在 对应的 patch, 并将其 supported 字段设置为 true
        self.check_feature_by_patches(patches)
    }

    /**
     * @description: 校验 feature 的依赖关系，中是否存在 对应的 patch, 并将其 supported 字段设置为 true
     * @param {*} patches 提供的可用的patches集合，用于校验 features 的依赖关系，中是否存在 对应的 patch, 并将其 supported 字段设置为 true
     * @return {*}
     */
    pub fn check_feature_by_patches(&mut self, patches: &mut Patches) -> Result<()> {
        //根据 dependencies VecString 提供的code  在 patches 查找是否可用存在状态，修改 supported 状态
        match self {
            //如果是FeatureDetail类型，则校验其依赖关系，中是否存在 对应的 patch, 并将其 supported 字段设置为 true
            Feature::FeatureDetail(feature) => {
                // 如果禁用 则直接返回
                if feature.disabled {
                    return Ok(());
                }
                // 如果是VecString类型，则校验其依赖关系，中是否存在 对应的 patch, 并将其 supported 字段设置为 true
                if let Dependencies::VecString(deps) = &feature.dependencies {
                    // 校验其依赖关系，中是否存在 对应的 patch, 并将其 supported 字段设置为 true
                    feature.supported = deps.iter().all(|code| {
                        //不需要依赖补丁，则返回真
                        if code.is_empty() {
                            return true;
                        }
                        // 在 patches 查找是否可用存在状态，修改 supported 状态
                        patches
                            .0
                            .iter()
                            .any(|patch| patch.code == *code && patch.supported && !patch.disabled)
                    });
                    // 校验其依赖关系，中是否存在 对应的 patch, 并将其 status 字段设置为 true
                    feature.status = true;
                    feature.status = deps.iter().all(|code| {
                        //不需要依赖补丁，则返回真
                        if code.is_empty() {
                            return true;
                        }
                        // 在 patches 查找是否可用存在状态，如果存在一个  patched 为 false 的，则返回 false
                        !patches
                            .0
                            .iter()
                            .any(|patch| patch.code == *code && !patch.patched)
                    });
                }
            }
            Feature::String(_) => {}
        }
        Ok(())
    }
}

/**
 * @description: 定义StyleType的结构，支持两种类型的StyleType，一种是switch，一种是button，
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StyleType {
    Switch,
    Button,
}

/**
 * @description: 定义Dependencies的结构，支持两种类型的Dependencies，一种是VecString类型，一种是VecDependency类型，
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Dependencies {
    VecString(Vec<String>), // 依赖关系，为 patch 的 code 集合
    // 依赖关系，通过版本号取出对应的依赖关系，
    //如果没有对应的版本号依赖，则将 supported 字段设置为 false, 并将 disabled 字段设置为 true
    //如果有则 替换 dependencies 字段为对应的依赖关系，
    VecDependency(Vec<Dependency>),
}

impl Dependencies {
    /**
     * @description: 调用所有dependencies的process方法
     * @param {*} variables 提供的可用的变量集合
     * @return {*}
     */
    pub fn process(&mut self, variables: &Variables) -> Result<Dependency> {
        match self {
            // 如果是VecString类型，则直接返回
            Dependencies::VecString(dependencies) => Ok(Dependency::new(
                "".to_string(),
                "".to_string(),
                false,
                dependencies.to_vec(),
            )),
            // 如果是VecDependency类型，则调用process方法
            Dependencies::VecDependency(dependencies) => {
                // 用取出支持当前的版本号的依赖关系，
                Ok(get_item_by_variables_install_version(dependencies, variables)?.clone())
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureDetail {
    pub code: String, // 唯一标识
    pub name: String, // 名称
    #[serde(default)]
    pub method: String, // 方法，在前端调用的具体方法，为空则默认为 code
    #[serde(default)]
    pub description: String, // 描述
    #[serde(default)]
    pub inmain: bool, // 是否在程序上显示
    #[serde(default)]
    pub incoexist: bool, // 是否在共存程序上显示
    pub index: i32,   // 排序
    #[serde(rename = "style")]
    pub style: StyleType, // 样式，支持两种类型，一种是switch，一种是button
    #[serde(default)]
    pub severity: String, // 按钮显示样式
    #[serde(default)]
    pub disabled: bool, // 是否禁用
    #[serde(default)]
    pub supported: bool, // 是否支持
    #[serde(default)]
    pub target: String, // 目标，用于指定目标程序/目录
    #[serde(default)]
    pub saveas: String, // 目标，用于指定目标程序/目录
    // 依赖关系，通过版本号取出对应的依赖关系，
    //如果没有对应的版本号依赖，则将 supported 字段设置为 false, 并将 disabled 字段设置为 true
    //如果有则 替换 dependencies 字段为对应的依赖关系，
    #[serde(default = "default_vec_string")]
    pub dependencies: Dependencies,
    #[serde(default)]
    pub status: bool, // 当前功能状态
}
/**
 * @description: Dependencies 序列化默认值
 * @return {*} Dependencies::VecString
 */
fn default_vec_string() -> Dependencies {
    Dependencies::VecString(vec!["".to_string()])
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub version: String, // 版本号
    #[serde(default)]
    pub description: String, // 描述
    #[serde(default)]
    pub disabled: bool, // 是否禁用
    pub dependencies: Vec<String>, // 依赖关系，为 patch 的 code 集合
}

impl Dependency {
    pub fn new(
        version: String,
        description: String,
        disabled: bool,
        dependencies: Vec<String>,
    ) -> Self {
        Self {
            version,
            description,
            disabled,
            dependencies,
        }
    }
}

impl GetVersion for Dependency {
    fn get_version(&self) -> &str {
        &self.version
    }
}
