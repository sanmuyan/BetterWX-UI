use crate::errors::Result;
use config::views::address_view::AddressView;
use config::views::features_view::FeaturesView;
use config::views::files_view::FileView;
use config::views::files_view::FilesView;
use config::views::orignal_view::OrignalViews;
use config::views::path_view::PathView;
use services::rule;

#[tauri::command(async)]
pub async fn rule_get_path(code: &str) -> Result<PathView> {
    Ok(rule::rule_get_path(code).await?)
}

#[tauri::command(async)]
pub async fn rule_search_address(code: &str) -> Result<AddressView> {
    Ok(rule::rule_search_address(code).await?)
}

#[tauri::command(async)]
pub async fn rule_walk_files(code: &str) -> Result<FilesView> {
    let files = rule::rule_walk_files(code).await?;
    Ok(files)
}

#[tauri::command(async)]
pub async fn rule_patch(code: &str, num: usize, fcode: &str, status: bool) -> Result<FeaturesView> {
    Ok(rule::rule_patch(code, num, fcode, status).await?)
}

#[tauri::command(async)]
pub async fn rule_make_coexist(code: &str, num: usize) -> Result<FileView> {
    Ok(rule::rule_make_coexist(code, num).await?)
}

#[tauri::command(async)]
pub async fn rule_del_coexist(code: &str, num: usize) -> Result<()> {
    Ok(rule::rule_del_coexist(code, num).await?)
}

#[tauri::command(async)]
pub async fn rule_read_orignal(code: &str, num: usize, fcode: &str) -> Result<OrignalViews> {
    Ok(rule::rule_read_orignal(code, num, fcode).await?)
}

#[tauri::command(async)]
pub async fn rule_patch_by_replace(
    code: &str,
    num: usize,
    fcode: &str,
    ovs: OrignalViews,
) -> Result<()> {
    Ok(rule::rule_patch_by_replace(code, num, fcode, &ovs).await?)
}
