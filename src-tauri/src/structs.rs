use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug)]
pub struct Patch {
    pub name: String,
    pub loc: Vec<(usize, usize)>,
    pub original: Vec<u8>,
    pub patch: Vec<u8>,
}

impl Clone for Patch {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            loc: self.loc.clone(),
            original: self.original.clone(),
            patch: self.patch.clone(),
        }
    }
}

#[derive(Debug)]
pub struct Patchs {
    pub unlock: Patch,
    pub revoke: Patch,
    pub coexist: Patch,
    pub autologin: Patch,
    pub exe: Patch,
}

#[derive(Debug)]
pub struct WxPath {
    pub exe_loc: PathBuf,
    pub dll_loc: PathBuf,
    pub exe_file: PathBuf,
    pub dll_file: PathBuf,
}

#[derive(Debug)]
pub struct WxData {
    pub exe_data: Vec<u8>,
    pub dll_data: Vec<u8>,
}

#[derive(Debug)]
pub struct WxInfo {
    pub wx_path: WxPath,
    pub wx_data: WxData,
    pub patchs: Patchs,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoexistFileInfo {
    pub id: i32,
    pub dll_name: String,
    pub dll_file: PathBuf,
    pub exe_name: String,
    pub exe_file: PathBuf,
    pub unlock: bool,
    pub revoke: bool,
}
