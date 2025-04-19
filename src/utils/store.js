import { Store } from "@tauri-apps/plugin-store"
import { isEmpty, getValueByCode } from "@/utils/utils.js"

//可清除
let store = {}
//不可清除
let store2 = {}
/**
 * 清楚缓存
 */
async function clearAll() {
    Object.keys(store).forEach(key => {
        store[key].clear()
    });
    // await Store.clear()
}

/**
 * 初始化
 */
/**
 * 初始化存储实例
 * @param {string} key 存储键名
 * @param {boolean} cleanable 是否可清除的存储
 */
async function init(key, cleanable = true) {
    try {
        const targetStore = cleanable ? store : store2;
        if (!targetStore[key]) {
            targetStore[key] = await Store.load(`${key}.json`, { autoSave: false });
        }
        return targetStore[key]
    } catch (error) {
        console.error(`初始化存储失败 (key: ${key})`, error);
        throw error;
    }
}

/**
 * 读取缓存
 * @param {string} key 缓存的key
 * @returns 
 */
async function read(baseData) {
    let { key, version, cleanable } = getSaveKey(baseData)
    console.log("缓存读取","key:", key,"version:", version)
    let store = await init(key, cleanable)
    let storeData = await store.get(key)
    return storeData?.data && (storeData?.version == version || !version )? storeData.data : false
}

/**
 * 保存缓存
 * @param {*} data 缓存的数据
 * @param {*} clean 是否清除缓存
 */
async function save(data, clean = false) {
    let { key, version, cleanable } = getSaveKey(data)
    console.log("缓存写入", key, version,data)
    let store = await init(key, cleanable)
    await store.set(key, {
        version,
        data: clean ? false : data
    })
    await store.save()
}

/**
 * @description: 获取缓存的key和version
 * @param {*} data 
 * @returns 
 */
function getSaveKey(data) {
    if (data.hasOwnProperty("content")) {
        // baseRUle
        return getReadmeSaveKey(data)
    } else if (data.hasOwnProperty("notes")) {
        // baseRUle
        return getNotesSaveKey(data)
    }else  if (data.hasOwnProperty("rules")) {
        //config
        return getConfigSaveKey(data)
    } else if (data.hasOwnProperty("code")) {
        // baseRUle
        return getBaseRuleSaveKey(data)
    }
    throw new Error("缓存失败，未知的缓存类型")
}

/**
 * @description: 获取 baserule 缓存的key和version
 * @param {*} data
 * @return {*}
 */
function getBaseRuleSaveKey(data) {
    let key = `${data.code}_base_rule`
    console.log("store getBaseRuleSaveKey",data.variables);
    let version = `${getValueByCode(data.variables, "install_version") || "0.0.1"}_${data.version || "0.0.1"}`
    return { key, version, cleanable: true }
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
    return { key, version, cleanable: true }
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
    return { key, version, cleanable: true }
}

/**
 * @description: 获取 Note 缓存的key和version
 * @param {*} data
 * @return {*}
 */
function getNotesSaveKey(data) {
    let key = `${data.code}_notes`
    return { key, version: null, cleanable: false }
}
export { read, save, clearAll }