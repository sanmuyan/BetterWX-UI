use crate::errors::Result;
use utils::cmd::Cmd;

pub fn cmd_close_app(name: &str) -> Result<()> {
    Ok(Cmd::new(name).close_app()?)
}

pub fn cmd_run_app(file: &str) -> Result<()> {
    Ok(Cmd::new(file).run_app()?)
}

pub fn cmd_open_url(url: &str) -> Result<()> {
    Ok(Cmd::new(url).open_url()?)
}

pub fn cmd_open_folder(path: &str) -> Result<()> {
    Ok(Cmd::new(path).open_folder()?)
}

