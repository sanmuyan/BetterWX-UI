use crate::errors::Result;
use config::update::Update;
use config::update::VerData;
use config::views::config_view::ConfigViews;
use services::update;

#[tauri::command(async)]
pub async fn update_check() -> Result<Update> {
    Ok(update::update_check().await?)
}

#[tauri::command(async)]
pub async fn update_config_check(uconfig: VerData) -> Result<ConfigViews> {
     Ok(update::config_check(&uconfig).await?)
}

#[tauri::command(async)]
pub async fn update_readme_check(ureadme: VerData) -> Result<String> {
     Ok(update::readme_check(&ureadme).await?)
}
