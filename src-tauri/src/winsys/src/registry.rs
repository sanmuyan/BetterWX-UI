use crate::errors::Result;
use crate::types::wstr::WSTR;
use thiserror::Error;
use windows::Win32::System::Registry::HKEY;
use windows::Win32::System::Registry::HKEY_CLASSES_ROOT;
use windows::Win32::System::Registry::HKEY_CURRENT_CONFIG;
use windows::Win32::System::Registry::HKEY_CURRENT_USER;
use windows::Win32::System::Registry::HKEY_LOCAL_MACHINE;
use windows::Win32::System::Registry::HKEY_USERS;
use windows::Win32::System::Registry::REG_VALUE_TYPE;
use windows::Win32::System::Registry::RRF_RT_REG_SZ;
use windows::Win32::System::Registry::RegGetValueW;

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("不支持的注册表根路径")]
    UnsupportHkeyRoot,

    #[error("请先设置需要读取的字段名")]
    FieldEmptyError,

    #[error("读取注册表失败")]
    RegGetValueWError,

    #[error("注册表读取结果转换失败")]
    ResultConvertError,
}

pub struct Registry {
    hkey: HKEY,
    sub_key: String,
    field: Option<String>,
}

impl Registry {
    pub fn new(path: &str) -> Result<Self> {
        let normalized_path = path.replace("/", "\\");
        let paths: Vec<&str> = normalized_path.split("\\").collect();
        let root: &str = &paths[0].to_ascii_uppercase();
        let sub_key = paths[1..].join("\\");
        let hkey = Self::convert_root(root)?;
        Ok(Self {
            hkey,
            sub_key,
            field: None,
        })
    }

    fn convert_root(root: &str) -> Result<HKEY> {
        let hkey = match root {
            "HKEY_CLASSES_ROOT" => HKEY_CLASSES_ROOT,
            "HKEY_CURRENT_USER" => HKEY_CURRENT_USER,
            "HKEY_LOCAL_MACHINE" => HKEY_LOCAL_MACHINE,
            "HKEY_USERS" => HKEY_USERS,
            "HKEY_CURRENT_CONFIG" => HKEY_CURRENT_CONFIG,
            _ => Err(RegistryError::UnsupportHkeyRoot)?,
        };
        Ok(hkey)
    }

    pub fn set_hkey(mut self, root: &str) -> Result<Self> {
        self.hkey = Self::convert_root(root)?;
        Ok(self)
    }

    pub fn set_path(mut self, path: &str) -> Result<Self> {
        let normalized_path = path.replace("/", "\\");
        let paths: Vec<&str> = normalized_path.split("\\").collect();
        let root: &str = &paths[0].to_ascii_uppercase();
        let sub_key = paths[1..].join("\\");
        self.hkey = Self::convert_root(root)?;
        self.sub_key = sub_key;
        Ok(self)
    }

    pub fn set_field(mut self, field: &str) -> Self {
        self.field = Some(field.to_string());
        self
    }

    pub fn read(&self) -> Result<String> {
        if let Some(field) = &self.field {
            return self.read_value(field);
        }
        Err(RegistryError::FieldEmptyError.into())
    }

    pub fn read_value(&self, field: &str) -> Result<String> {
        let mut field_type = REG_VALUE_TYPE(0);
        let mut buffer_size = 0u32;
        let mut field_wstr = WSTR::new(Some(field));
        let field_pwstr = field_wstr.to_pwstr();
        let mut sub_key = WSTR::new(Some(&self.sub_key));
        let sub_key_pwstr = sub_key.to_pwstr();
        let read_size = unsafe {
            RegGetValueW(
                self.hkey,
                sub_key_pwstr,
                field_pwstr,
                RRF_RT_REG_SZ,
                Some(&mut field_type),
                None,
                Some(&mut buffer_size),
            )
        };
        if read_size.is_err() {
            return Err(RegistryError::RegGetValueWError.into());
        }

        let mut buffer = vec![0u16; (buffer_size / 2) as usize];

        let read_field = unsafe {
            RegGetValueW(
                self.hkey,
                sub_key_pwstr,
                field_pwstr,
                RRF_RT_REG_SZ,
                None,
                Some(buffer.as_mut_ptr() as *mut _),
                Some(&mut buffer_size),
            )
        };
        if read_field.is_err() {
            return Err(RegistryError::RegGetValueWError.into());
        }
        // 转换为字符串并去除末尾的null字符
        let result = String::from_utf16(&buffer)
            .map_err(|_| RegistryError::ResultConvertError)?
            .trim_end_matches('\0')
            .to_string();

        Ok(result)
    }
}
