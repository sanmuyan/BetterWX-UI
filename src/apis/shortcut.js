import { invoke } from "@tauri-apps/api/core"

export async function shortcut_to_desktop(file, name, icon, args) {
    return await invoke("shortcut_to_desktop", { file, name, icon, args })
}

export async function shortcut_to_startup(file, name, icon, args) {
    return await invoke("shortcut_to_startup", { file, name, icon, args })
}