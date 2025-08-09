use crate::addresses::Addresses;
use crate::errors::ConfigError;
use crate::errors::Result;
use crate::groups::Group;
use crate::groups::Groups;
use crate::serders::skippers::skip_if_empty;
use crate::variables::Variables;
use crate::views::orignal_view::OrignalView;
use log::debug;
use macros::FieldDescGetters;
use macros::FieldNameGetters;
use macros::ImpConfigVecIsEmptyTrait;
use macros::ImpConfigVecWrapperTrait;
use serde::Deserialize;
use serde::Serialize;
use utils::empty::Empty;
use utils::patch::patch::UPatch;

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Default,
    ImpConfigVecIsEmptyTrait,
    ImpConfigVecWrapperTrait,
)]
pub struct Patterns(pub Vec<Pattern>);

impl Patterns {
    pub fn init(&mut self, variables: &Variables) -> Result<()> {
        for pattern in &mut self.0 {
            pattern.init(variables)?;
        }
        Ok(())
    }

    pub fn set_patched(&mut self, upatch: &UPatch) -> Result<()> {
        for pattern in &mut self.0 {
            pattern.set_patched(upatch)?;
        }
        Ok(())
    }

    pub fn search(&mut self, upatch: &UPatch) -> Result<()> {
        for pattern in &mut self.0 {
            pattern.search(upatch)?;
        }
        Ok(())
    }
}
impl Patterns {
    pub fn is_supported(&self) -> bool {
        self.0.iter().all(|pattern| pattern.disabled || pattern.supported)
    }

    pub fn is_searched(&self) -> bool {
        self.0
            .iter()
            .any(|pattern| pattern.searched)
    }

    pub fn is_patched(&self) -> bool {
        self.0.iter().any(|pattern| pattern.addresses.is_patched())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldDescGetters, FieldNameGetters)]

pub struct Pattern {
    pub code: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub groups: Groups,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub group: Option<Group>,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub description: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub disabled: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub supported: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub addresses: Addresses,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub patched: bool,
        #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub searched: bool,
}

impl Pattern {
    pub fn init(&mut self, variables: &Variables) -> Result<()> {
        // 禁用不处理
        if self.disabled {
            return Ok(());
        }

        // 校验字段
        if let Ok(num) = variables.get_num()
            && num == 10
            && self.groups.is_empty()
            && self.group.is_none()
        {
            return Err(
                ConfigError::ConfigFieldMissing(format!("{}的groups", self.get_name())).into(),
            );
        }

        // 从 groups 中获取对应 version 的特征码数据
        if !self.groups.is_empty() {
            let version = variables.get_install_version()?.to_string();
            let group = self
                .groups
                .take_first_less_by_version(version.as_str())
                .map_err(|_| ConfigError::PatternNotSupported(version))?;
            self.disabled = group.disabled;

            // 未禁用，添加到 group 用于后续搜索
            if !self.disabled {
                self.group = Some(group);
            }
            // 清除 groups
            self.groups = Groups::default();
            return Ok(());
        }

        // gropu 存在数据 处理需要搜索的数据，然后搜索
        if let Some(group) = &mut self.group {
            group.init(variables)?;
        }

        // addresses 构建补丁数据
        if !self.addresses.is_empty() {
            //初始化 patched
            self.patched = false;
            self.addresses.init(variables)?;
        }

        Ok(())
    }

    pub fn search(&mut self, upatch: &UPatch) -> Result<()> {
        // 禁用不处理
        if self.disabled {
            self.supported = true;
            self.searched = true;
            return Ok(());
        }

        if !self.supported
            && let Some(group) = &self.group
        {
            debug!("-------------------------------------");
            let name = self.get_name();
            if let Ok(addresses) = group.search(upatch, false, name) {
                self.addresses = addresses;
                self.supported = true;
                self.group = None;
                return Ok(());
            }
            match group.search(upatch, true, name) {
                Ok(addresses) => {
                    self.addresses = addresses;
                    self.supported = true;
                    self.group = None;
                    return Err(ConfigError::BackFileIsPatched(self.get_name().to_owned()).into());
                }
                Err(_) => {
                    self.supported = false;
                    self.group = None;
                }
            }
        }
        return Ok(());
    }

    pub fn set_patched(&mut self, upatch: &UPatch) -> Result<bool> {
        self.patched = self.addresses.get_patched(upatch)?;
        Ok(self.patched)
    }

    pub fn patch(&mut self, upatch: &mut UPatch, status: bool) -> Result<()> {
        debug!("开始执行 {} 补丁", self.get_name());
        // 禁用不处理
        if self.disabled {
            debug!("开始执行 {} 补丁disabled  pass", self.get_name());
            return Ok(());
        }
        if self.addresses.is_empty() {
            return Err(ConfigError::DependPatchNotFoundError(self.get_name().to_string()).into());
        }
        self.addresses.patch(upatch, status)?;
        Ok(())
    }

     pub fn patch_by_replace(&mut self, upatch: &mut UPatch, replace: &str) -> Result<()> {
        // 禁用不处理
        if self.disabled {
            return Ok(());
        }

        if self.addresses.is_empty() {
            return Err(ConfigError::DependPatchNotFoundError(self.get_name().to_string()).into());
        }
        debug!("开始执行 {} 补丁", self.get_name());
        self.addresses.patch_by_replace(upatch, replace)?;
        Ok(())
    } 

    pub fn read_orignal(&self, upatch: &mut UPatch) ->Result<OrignalView> {
        // 禁用不处理
        if self.disabled {
            return Ok(OrignalView::default());
        }

        if self.addresses.is_empty() {
            return Err(ConfigError::DependPatchNotFoundError(self.get_name().to_string()).into());
        }
        self.addresses.read_orignal(upatch)
    }
    
}
