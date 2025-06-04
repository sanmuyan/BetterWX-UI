<template>
    <div v-if="showLoading" class="full-screen flex justify-center items-center not-select">
        <div class="flex col justify-center items-center" style="height: 200px; width: 300px;">
            <ProgressSpinner style="width: 50px; height: 50px" strokeWidth="8" fill="transparent" />
            <TransitionGroup name="fade" tag="div" class="text-center w-full overflow-hidden m-y" style="height: 60px;">
                <div v-for="(message, index) in mesages" :key="index" :style="{ opacity: 1 - index * 0.15 }"
                    class="w-full text-center">
                    {{ message }}
                </div>
            </TransitionGroup>
        </div>
    </div>
    <!-- 检测到软件更新 -->
    <Dialog class="not-select" v-model:visible="showDialog" modal header="提示" style="width: 20rem;" @hide="closeDialog"
        :closable="updateInfo.notforce == true">
        <div class="flex col">
            <label class="m-b">{{ `发现新版本: ${updateInfo.version}` }}</label>
            <label class="m-b text-prewarp">{{ updateInfo.description }}</label>
            <div class="flex items-center justify-center gap m-y">
                <template v-for="(button, index) in updateInfo.buttons" :key="index">
                    <Button :label="button.name" @click="openUrl(button.url)" size="small"
                        :severity="button.severity ? button.severity : ''" />
                </template>
            </div>
        </div>
    </Dialog>
</template>

<script setup>
import { onMounted, ref } from 'vue'
import { getVersion, getName } from '@tauri-apps/api/app'
import { UPDATE_URL } from "@/config/app_config.js"
import { compareVersion, sleep } from "@/utils/utils.js"
import { http } from "@/utils/http.js"
import { save, read } from "@/utils/store.js"
import { openUrl } from "@/utils/bridge.js"
import { USE_SAVE_CONFIG, SPLASH_DELAY, SPLASH_SUCCESS_DELAY, TEST_MODE } from '@/config/app_config.js';
import { Window } from '@tauri-apps/api/window'
import * as bridge from "@/utils/bridge.js"
import { decryptText } from "@/utils/crypto.js"

const updateInfo = ref({})
const showLoading = ref(true)
const showDialog = ref(false)
const mesages = ref([])

onMounted(() => {
    checkUpdate()
})

/**
 * @description: 检查更新
 */
async function checkUpdate() {
    try {
        // 检查软件更新
        await addMsg("正在检查软件更新")
        const appVersion = await getVersion()
        const appName = await getName()
        // 设置title
        let showVersion = TEST_MODE ? `${appVersion} 测试版` : appVersion
        setTitle(appName, showVersion)
        let info = await http(UPDATE_URL + "?r=" + Math.random())
        updateInfo.value = await dealOtherVersion(info)
        console.log("updateInfo.valueupdateInfo.value", updateInfo.value);
        //从更新文件加载app名称
        if (!TEST_MODE && updateInfo.value.name) {
            setTitle(updateInfo.value.name || appName, appVersion)
        }
        const hasUpdate = compareVersion(appVersion, updateInfo.value.version) < 0
        console.log(`本地软件版本号:${appVersion}, 软件版本号:${updateInfo.value.version}`)
        if (hasUpdate) {
            await addMsg("发现新版本: " + updateInfo.value.version, 1000)
            showDialog.value = true
            updateInfo.value.hasUpdate = hasUpdate
            return
        }
        await closeDialog()
    } catch (error) {
        console.error(error)
        await addMsg(`软件初始化失败:${error}`)
    }
}

/**
 * @description: 取消更新
 */
async function closeDialog() {
    if (updateInfo.value.hasUpdate) {
        await addMsg("取消更新")
    }
    showDialog.value = false
    try {
        // 检查配置文件更新
        await addMsg("正在检查配置文件更新")
        let config = await loadConfig()
        // 处理配置文件

        let parseConfig = await bridge.parseConfig(config)
        parseConfig.rules.sort((a, b) => a.index - b.index)
        await addMsg(`配置文件 ${config.version} 加载完成`)
        await addMsg("软件初始化成功", SPLASH_SUCCESS_DELAY)
        showLoading.value = false
        loaded({ parseConfig, updateInfo: updateInfo.value })
    } catch (error) {
        console.error(error)
        await addMsg(`软件初始化失败:${error}`)
    }
}

/**
 * @description: setTitle
 */
function setTitle(name, version) {
    const currentWindow = Window.getCurrent()
    currentWindow.setTitle(`${name} v${version}`)
}

/**
 * @description: 加载配置文件
 * @return {*}
 */
async function loadConfig() {
    let config = (await import('@/config/config.json')).default
    if (USE_SAVE_CONFIG) {
        const storeConfig = await read(config)
        if (storeConfig) {
            console.log("加载缓存config")
            config = storeConfig
        }
    }
    // 检查配置文件是否需要更新
    if (compareVersion(config.version, updateInfo.value.config.version) < 0) {

        await addMsg("发现配置文件: " + updateInfo.value.config.version)
        const appName = await getName()
        config = await http(updateInfo.value.config.url + "?r=" + Math.random(), { text: true })
        let key = `${appName} ${updateInfo.value.config.version}`
        console.log("解密key", key);
        config = JSON.parse(decryptText(config, key))
        console.log("解析配置文件", config)
        if (updateInfo.value.config.version != config.version) {
            throw new Error("update和config配置文件版本号不一致")
        }
        if (USE_SAVE_CONFIG) {
            console.log("缓存config")
            await save(config)
        }
    }
    return config
}

async function dealOtherVersion(info) {
    const appVersion = await getVersion()
    info?.others?.find((item) => {
        if (compareVersion(appVersion, item.version) >= 0) {
            info = { ...info, ...item }
            return info
        }
    })
    return info
}

/**
 * @description: 模拟下缓慢加载的效果吧
 * @param msg 
 */
async function addMsg(msg, delay = SPLASH_DELAY) {
    mesages.value.unshift(msg)
    if (mesages.value.length > 5) {
        mesages.value.pop()
    }
    await sleep(delay)
}

/**
 * @description: 加载完成，通知上层
 */
const emit = defineEmits(["loaded"])
function loaded(config) {
    emit('loaded', { data: config })
}


</script>