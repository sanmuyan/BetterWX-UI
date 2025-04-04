import { Store } from "@tauri-apps/plugin-store"
import { isEmpty,getValueByCode } from "@/utils/utils.js"

let store = {}

/**
 * 清楚缓存
 */
async function clearAll()  {
    Object.keys(store).forEach(key => {
        store[key].clear()
    });
   // await Store.clear()
}
 
/**
 * 初始化
 */
async function init(key) {
    if (!store[key]) {
        store[key] = await Store.load(`${key}.json`, { autoSave: false })
    }
}

/**
 * 读取缓存
 * @param {string} key 缓存的key
 * @returns 
 */
async function read(baseData) {
    let { key, version } = getSaveKey(baseData)
    console.log("缓存读取",key, version)
    await init(key)
    let storeData = await store[key].get(key)
    return storeData?.data && storeData?.version == version ? storeData.data:false
}

/**
 * 写入缓存
 * @param {string} key 缓存的key
 * @param {object} value 缓存的内容
 */
async function save(data,clean=false) {
    let { key, version } = getSaveKey(data)
    console.log("缓存写入",key, version)
    await init(key)
    await store[key].set(key, {
        version,
        data:clean?false:data
    })
    await store[key].save()
}

/**
 * @description: 获取缓存的key和version
 * @param {*} data 
 * @returns 
 */
function getSaveKey(data){
    if(data.hasOwnProperty("rules")){
        //config
        return getConfigSaveKey(data)
    }else if(data.hasOwnProperty("code")){
        // baseRUle
        return getBaseRuleSaveKey(data)
    }else if(data.hasOwnProperty("content")){
        // baseRUle
        return getReadmeSaveKey(data)
    }
    console.log(data)
    throw new Error("缓存失败，未知的缓存类型")
}

/**
 * @description: 获取 baserule 缓存的key和version
 * @param {*} data
 * @return {*}
 */
function getBaseRuleSaveKey(data) {
    let key = `${data.code}_base_rule`
    console.log(data.variables);
    let version = `${getValueByCode(data.variables,"install_version") || "0.0.1" }_${data.version || "0.0.1"}`
    return { key, version }
}

/**
 * @description: 获取 readme 缓存的key和version
 * @param {*} data
 * @return {*}
 */
function getReadmeSaveKey(data) {
    let key = `README`
    // saveVersion:4.0.3.11_0.0.1
    let version = `${data.version}`
    return { key, version }
}

/**
 * @description: 获取 config 缓存的key和version
 * @param {*} data
 * @return {*}
 */
function getConfigSaveKey(data) {
    let key = `config`
    // saveVersion:4.0.3.11_0.0.1
    let version = `${data.version}`
    return { key, version }
}

export { read, save,clearAll}