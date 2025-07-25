use crate::errors::ConfigError;
use crate::errors::Result;
use crate::features::Features;
use macros::ImpConfigVecIsEmptyTrait;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, Default, ImpConfigVecIsEmptyTrait)]
pub struct DFeatures(pub Vec<String>);

impl DFeatures {
    pub fn init(&self) -> Result<Features> {
        let mut features: Features = serde_json::from_str(DEFAULT_FEATURE_STR)
            .map_err(|_| ConfigError::DefaultFeaturesDeserializeError)?;
        features.0.retain(|f| !self.0.contains(&f.code));
        Ok(features)
    }
}

const DEFAULT_FEATURE_STR: &str = r#"[
    {
        "code": "select",
        "description": "选中",
        "inmain": true,
        "incoexist": true,
        "index": 30,
        "bntype": "checkbox",
        "selected": true,
        "target":"${exe_name_save}"
    },
    {
        "code": "select_all",
        "description": "全选",
        "inhead": true,
        "index": 31,
        "bntype": "checkbox"
    },
    {
        "code": "close_all",
        "name": "一键关闭",
        "description": "退出所有选中的软件",
        "severity": "danger",
        "inhead": true,
        "index": 33
    },{
        "code": "lnk_all",
        "name": "一键快捷",
        "description": "创建一键启动快捷方式到桌面",
        "target": "${exe_base}",
        "inhead": true,
        "index": 100
    },
    {
        "code": "folder",
        "name": "打开目录",
        "description": "打开文件所在目录",
        "inhead": true,
        "index": 120,
        "target": "${exe_path}"
    },
    {
        "code": "lnk",
        "name": "快捷",
        "description": "添加快捷方式到桌面",
        "inmain": true,
        "incoexist": true,
        "index": 131,
        "target": "${exe_save}"
    },
    {
        "code": "open",
        "name": "运行",
        "description": "运行当前程序",
        "inmain": true,
        "incoexist": true,
        "index": 140,
        "target": "${exe_save}"
    },
    {
        "code": "close",
        "name": "关闭",
        "description": "关闭当前程序",
        "inmain": true,
        "incoexist": true,
        "index": 141,
        "severity": "danger",
        "target":"${exe_name_save}"
    },
    {
        "code": "del",
        "name": "删除",
        "description": "删除共存文件",
        "incoexist": true,
        "index": 151,
        "disabled": false,
        "severity": "danger"
    }
]"#;
