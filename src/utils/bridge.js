import { invoke } from "@tauri-apps/api/core"

/**
 * 解析配置文件
 * @param {*} path 
 */
async function parseConfig(config) {
    return await invoke('parse_config',{config})
}

/**
 * 判断文件是否存在
 * @param {*} path 
 */
async function isFileExists(path) {
    return await invoke('is_file_exists',{path})
}

/**
 * 判断一组文件是否存在
 * @param {*} path 
 */
async function isFilesExists(files) {
    return await invoke('is_files_exists',{files})
}

/**
 * @description: 删除一组文件
 */
async function delFiles(files) {
    await invoke('del_files',{files})
}

/**
 * @description: 运行应用
 */
async function runApp(file) {
    await invoke('run_app',{file})
}

/**
 * @description: 打开网页
 */
async function openUrl(url) {
    await invoke('open_url',{url})
}

/**
 * @description: 打开文件夹
 */
async function openFolder(folder) {
    await invoke('open_folder',{folder})
}


/**
 * @description: 搜索基址
 * @param {*} rule 
 * @returns 
 */
async function  searchBaseAddress(rule) {
    return await invoke('search_base_address',{rule}) 
}

/**
 * @description: 刷新文件信息
 * @param {*} rule 
 * @returns 
 */
async function  refreshFilesInfo(rule) {
    let filesInfo = await invoke('refresh_files_fnfo',{rule}) 
    filesInfo.sort((a,b)=>a.index-b.index)
    filesInfo.forEach(fileInfo=>{
        fileInfo.features.sort((a,b)=>a.index-b.index)
    })
    return filesInfo
}



/**
 * @description: 打补丁
 * @param {*} patches 
 * @return {*} patches
 */
async function applyPatch(patches,status) {
    return await invoke('apply_patch',{patches,status})
}

/**
 * @description: 根据规则和序号构建文件信息
 * @param {*} rule 
 * @param {*} num 
 * @returns 
 */
async function buildFileInfoByNum(rule,num) {
    return await invoke('build_file_info_by_num',{rule,num})
}

export {
    isFileExists,
    isFilesExists,
    parseConfig,
    searchBaseAddress,
    refreshFilesInfo,
    buildFileInfoByNum,
    applyPatch,
    delFiles,
    runApp,
    openUrl,
    openFolder
}