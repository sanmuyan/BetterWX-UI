use crate::errors::Result;

#[tauri::command(async)]
pub fn store_read(name:&str)->Result<String>{
    Ok(services::store::store_read(name)?)
}

#[tauri::command(async)]
pub fn store_save(name:&str,data:&str)->Result<()>{
    Ok(services::store::store_save(name,data)?)
}
