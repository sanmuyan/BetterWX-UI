import { invoke } from "@tauri-apps/api/core"

export async function process_run_apps(paths, login) {
     return await invoke("process_run_apps",{paths,login})
}
export async function process_run_app(file) {
    return await invoke("process_run_app",{file})
}

export async function process_close_apps(files) {
    return await invoke("process_close_apps",{files})
}

export async function process_close_app(fileName,delay) {
    return await invoke("process_close_app",{fileName,delay})
}