import { invoke } from "@tauri-apps/api/core"

/**
 * 解析配置文件
 * @param {*} path 
 */
export async function checkConfig(config) {
    return await invoke('check_config', { config })
}

/**
 * 解析rule
 * @param {*} path 
 */
export async function parseRule(rule) {
    return await invoke('parse_rule', { rule })
}

/**
 * 判断文件是否存在
 * @param {*} path 
 */
export async function isFileExists(path) {
    return await invoke('is_file_exists', { path })
}

/**
 * 判断一组文件是否存在
 * @param {*} path 
 */
export async function isFilesExists(files) {
    return await invoke('is_files_exists', { files })
}

/**
 * @description: 删除一组文件
 */
export async function delFiles(files) {
    await invoke('del_files', { files })
}

/**
 * @description: 运行应用
 */
export async function runApp(file) {
    await invoke('run_app', { file })
}

/**
 * @description: 打开网页
 */
export async function openUrl(url) {
    await invoke('open_url', { url })
}

/**
 * @description: 打开文件夹
 */
export async function openFolder(folder) {
    await invoke('open_folder', { folder })
}


/**
 * @description: 搜索基址
 * @param {*} rule 
 * @returns 
 */
export async function searchBaseAddress(rule) {
    return await invoke('search_base_address', { rule })
}

/**
 * @description: 刷新文件信息
 * @param {*} rule 
 * @returns 
 */
export async function refreshFilesInfo(rule) {
    let filesInfo = await invoke('refresh_files_info', { rule })
    filesInfo.sort((a, b) => a.index - b.index)
    filesInfo.forEach(fileInfo => {
        fileInfo.features.sort((a, b) => a.index - b.index)
    })
    return filesInfo
}

/**
 * @description: 打补丁
 * @param {*} patches 
 * @return {*} patches
 */
export async function applyPatch(patches) {
    return await invoke('apply_patch', { patches })
}


export async function removePatchesBackupFiles(patches) {
    return await invoke('remove_patches_backup_files', { patches })
}

/**
 * @description: 根据规则和序号构建文件信息
 * @param {*} rule 
 * @param {*} num 
 * @returns 
 */
export async function buildFileInfoByNum(rule, num) {
    let fileInfo = await invoke('build_file_info_by_num', { rule, num })
    fileInfo.features = fileInfo.features.sort((a, b) => a.index - b.index)
    return fileInfo
}

/**
 * @description: 根据规则和序号构建功能信息
 * @param {*} rule 
 * @param {*} num 
 * @returns 
 */
export async function buildFeatureFileInfo(rule) {
    let fileInfo =  await invoke('build_feature_file_info', {rule})
    fileInfo.features = fileInfo.features.sort((a, b) => a.index - b.index)
    return fileInfo
}

/**
 * @description: 运行所有选中程序
 * @returns 
 */
export async function runApps(files, login, close) {
    return await invoke('run_apps', { files, login, close })
}

/**
 * @description: 关闭所有选中程序
 * @returns 
 */
export async function closeApps(files) {
    return await invoke('close_apps', { files })
}

/**
 * @description: 创建快捷方式
 * @returns 
 */
export async function createShortcutToDesktop(exe,name,icon,args) {
    return await invoke('create_shortcut_to_desktop', { exe,name,icon,args })
}

/**
 * @description: 创建快捷方式
 * @returns 
 */
export async function getRuntimeFile() {
    return await invoke('get_runtime_file', {})
}




