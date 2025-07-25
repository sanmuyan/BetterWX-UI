import { invoke } from "@tauri-apps/api/core"

export async function cmd_close_app(name) {
    return await invoke('cmd_close_app', { name })
}


export async function cmd_run_app(file) {
    return await invoke('cmd_run_app', { file })
}


export async function cmd_open_url(url) {
    return await invoke('cmd_open_url', { url })
}


export async function cmd_open_folder(path) {
    return await invoke('cmd_open_folder', { path })
}