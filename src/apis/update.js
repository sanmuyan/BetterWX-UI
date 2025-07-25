import { invoke } from "@tauri-apps/api/core"


export async function update_check() {
    return await invoke("update_check")
}

export async function update_config_check(uconfig) {
    return await invoke("update_config_check", { uconfig })
}

export async function update_readme_check(ureadme) {
    return await invoke("update_readme_check", { ureadme })
}