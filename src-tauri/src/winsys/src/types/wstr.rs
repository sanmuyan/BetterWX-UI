use std::iter::once;
use windows::core::PCWSTR;
use windows::core::PWSTR;
pub struct WSTR {
    data: Option<Vec<u16>>,
}

impl WSTR {
    pub fn new(s: Option<&str>) -> Self {
        Self {
            data: s.map(|s| s.encode_utf16().chain(once(0)).collect()),
        }
    }

    pub fn to_pcwstr(&self) -> PCWSTR {
        self.data
            .as_ref()
            .map(|s| PCWSTR::from_raw(s.as_ptr()))
            .unwrap_or_else(|| PCWSTR::null())
    }

    pub fn to_pwstr(&mut self) -> PWSTR {
        self.data
            .as_mut()
            .map(|s| PWSTR::from_raw(s.as_mut_ptr()))
            .unwrap_or_else(|| PWSTR::null())
    }
}
