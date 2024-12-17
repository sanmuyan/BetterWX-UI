pub const WX_DLL_NAME: &str = "Weixin.dll";
pub const WX_EXE_NAME: &str = "Weixin.exe";
pub const NEW_WX_EXE_NAME: &str = "Weixin#.exe";
pub const NEW_WX_DLL_NAME: &str = "Weixin.dl#";
pub const WX_DLL_BAK_NAME: &str = "Weixin.dll.bak";
pub const WX_EXE_BAK_NAME: &str = "Weixin.exe.bak";

pub const UNLOCK_PATTERN: &str = "
        C7 44 24 ?? FF FF FF FF
        31 F6
        45 31 C0
        41 B9 FF FF FF FF
        FF 15 ?? ?? ?? ??
        85 C0
        75 0F";
pub const UNLOCK_REPLACE: &str = "...EB 0F";

pub const REVOKE_PATTERN: &str = "
75 21 
48 B8 72 65 76 6F 6B 65 6D 73
48 89 05 ?? ?? ?? ??
66 C7 05 ?? ?? ?? ?? 67 00
C6 05 ?? ?? ?? ?? 01
48 8D 3D
";

pub const REVOKE_REPLACE: &str = "
EB 21
...
";

pub const COEXIST_CONFIG_PATTERN: &str = "
48 B8 67 6C 6F 62 61 6C 5F 63
48 89 05 ?? ?? ?? ??
C7 05 ?? ?? ?? ?? 6F 6E 66 69
66 C7 05 ?? ?? ?? ?? 67 00
";

//打补丁时候 需要修改FF 为 num_u8
pub const COEXIST_CONFIG_REPLACE: &str = "
...
C7 05 ?? ?? ?? ?? 6F 6E 66 FF
66 C7 05 ?? ?? ?? ?? 67 00
";

// Redirect host-redirect.xml -> host-redirect.xm1
pub const AUTOLOGIN_PATTERN: &str = "686F73742D72656469726563742E786D6C";

//Redirect Weixin.dll -> Weixin.dl1
pub const EXE_PATTERN: &str = "570065006900780069006E002E0064006C006C";
