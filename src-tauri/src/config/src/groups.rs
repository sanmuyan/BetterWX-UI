use crate::addresses::Addresses;
use crate::errors::ConfigError;
use crate::errors::Result;
use crate::serders::default::default_1;
use crate::serders::skippers::skip_if_1;
use crate::serders::skippers::skip_if_empty;
use crate::variables::Variables;
use log::debug;
use log::error;
use log::info;
use macros::ImpConfigVecIsEmptyTrait;
use macros::SortedDeserializeByVersionDesc;
use serde::Deserialize;
use serde::Serialize;
use utils::patch::patch::UPatch;
use utils::tools::replace_ellipsis;
use utils::version::Version;

#[derive(
    Debug, Clone, Serialize, Default, ImpConfigVecIsEmptyTrait, SortedDeserializeByVersionDesc,
)]
pub struct Groups(pub Vec<Group>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub version: Version,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub pattern: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub replace: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    // replace2 用于搜索特征码时临时使用
    pub replace2: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub description: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "skip_if_empty")]
    pub disabled: bool,
    #[serde(default = "default_1")]
    #[serde(skip_serializing_if = "skip_if_1")]
    pub count: usize,
}

impl Group {
    /// 构造 replace2 数据，用于 搜索特征码
    pub fn init(&mut self, variables: &Variables) -> Result<()> {
        // 核对 num 和 num_hex 存在
        let _ = variables.get_num()?;
        let _ = variables.get_num_hex()?;
        if self.replace.is_empty() || self.replace.as_str() == "..." {
            self.replace2 = "".to_string();
            return Ok(());
        }
        // 判断是否是包含 地址计算
        let mut replace = self.replace.clone();
        replace = variables.substitute(replace);
        replace = variables.substitute_add(replace, true, "")?;
        replace = replace_ellipsis(&replace, &self.pattern)?;
        self.replace2 = replace;
        Ok(())
    }

    pub fn search(&self, upatch: &UPatch, usereplace: bool, name: &str) -> Result<Addresses> {
        let pattern = &self.pattern;
        let text = if usereplace {
            "补丁码".to_string()
        } else {
            "特征码".to_string()
        };
        let p = if usereplace {
            let replace2 = self.replace2.as_str();
            if replace2 == "" {
                return Ok(Addresses::default());
            }
            replace2
        } else {
            pattern.as_str()
        };

        debug!("使用 {} 搜索 {} 地址, 特征码:{}", text, name, p);
        match upatch.search_all(p) {
            Ok(poses) => {
                let len = poses.len();

                // 基址数量限制
                if len > 5 || (self.count > 0 && self.count != len) {
                    return Err(ConfigError::AddressesTooMuchError(
                        name.to_owned(),
                        len,
                        self.count,
                    )
                    .into());
                }

                info!(
                    "搜索 {} {}。成功！地址:{:?} ,len {} ,count {}",
                    name, text, poses, len, self.count
                );

                let addresses = Addresses::create(
                    &upatch,
                    poses,
                    self.replace.as_str(),
                    pattern.len() / 2,
                    usereplace,
                )?;
                Ok(addresses)
            }
            Err(e) => {
                error!("搜索 {} {}。失败！{}为:{}", name, text, text, p);
                return Err(e.into());
            }
        }
    }
}
