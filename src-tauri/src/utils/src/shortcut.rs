use crate::errors::Result;
use serde::Deserialize;
use serde::Serialize;
use winsys::shortcut::Shortcut;

pub fn to_desktop(
    file: Option<&str>,
    name: Option<&str>,
    icon: Option<&str>,
    args: Option<ShortCutArgs>,
) -> Result<()> {
    let mut r = Shortcut::default().set_desktop_as_target()?;
    if let Some(file) = file {
        r = r.set_file(file);
    }
    if let Some(name) = name {
        r = r.set_name(name);
    }
    if let Some(icon) = icon {
        r = r.set_icon(icon);
    }
    if let Some(args) = args {
        r = r.set_args(args.to_string());
    }
    Ok(r.run()?)
}

pub fn to_startup(
    file: Option<&str>,
    name: Option<&str>,
    icon: Option<&str>,
    args: Option<ShortCutArgs>,
) -> Result<()> {
    let mut r = Shortcut::default().set_startup_as_target()?;
    if let Some(file) = file {
        r = r.set_file(file);
    }
    if let Some(name) = name {
        r = r.set_name(name);
    }
    if let Some(icon) = icon {
        r = r.set_icon(icon);
    }
    if let Some(args) = args {
        r = r.set_args(args.to_string());
    }

    Ok(r.run()?)
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ShortCutArgs {
    pub code: String,
    pub name: String,
    pub path: String,
    pub list: String,
    #[serde(default)]
    #[serde(skip_serializing)]
    pub level: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing)]
    pub login: Option<String>,
}

impl ShortCutArgs {
    pub fn new<S: Into<String>>(
        code: S,
        name: S,
        path: S,
        list: S,
        level: Option<S>,
        login: Option<String>,
    ) -> Self {
        Self {
            code: code.into(),
            name: name.into(),
            path: path.into(),
            list: list.into(),
            level: level.map(|s| s.into()),
            login: login.map(|s| s.into()),
        }
    }

    pub fn check(&self) -> bool {
        self.code.len() > 0 && self.name.len() > 0 && self.path.len() > 0 && self.list.len() > 0
    }
}

impl std::fmt::Display for ShortCutArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "code=\"{}\" name=\"{}\" path=\"{}\" list=\"{}\"",
            self.code, self.name, self.path, self.list
        )?;
        if let Some(level) = &self.level {
            write!(f, " level=\"{}\"", level)?;
        }
        if let Some(login) = &self.login {
            write!(f, " login=\"{}\"", login)?;
        }
        Ok(())
    }
}

impl From<Vec<String>> for ShortCutArgs {
    fn from(args: Vec<String>) -> Self {
        let mut result = Self::default();
        for arg in args {
            if let Some((key, value)) = arg.split_once('=') {
                match key {
                    "code" => result.code = value.to_string(),
                    "name" => result.name = value.to_string(),
                    "path" => result.path = value.to_string(),
                    "list" => result.list = value.to_string(),
                    "level" => result.level = Some(value.to_string()),
                    "login" => result.login = Some(value.to_string()),
                    _ => continue,
                }
            }
        }

        result
    }
}
