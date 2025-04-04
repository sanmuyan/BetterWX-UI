<template>
    <ScrollPanel :style="style">
        <div class="m-x not-select">
            <Tag :value="installName" :severity="tagSeverity1" class="m-r"></Tag>
            <Tag :value="`特征码版本: ${props.parseConfigRule.version}`" :severity="tagSeverity2"></Tag>
            <div v-if="isValid">
                <template v-for="(fileInfo) in filesInfo" :key="fileInfo.num">
                    <CoexistList :fileInfo="fileInfo" :rule="baseRule" @event="handleEvent" class="border-b p-y">
                    </CoexistList>
                </template>
            </div>
            <Loading :show="showLoading" />
        </div>
    </ScrollPanel>
</template>

<script setup>
import { ref, watch, inject, computed, onMounted, nextTick } from "vue"
import CoexistList from "@/components/coexist-list.vue"
import Loading from "@/components/loading.vue"
import * as bridge from "@/utils/bridge.js"
import { read, save, clearAll } from "@/utils/store.js"
import { USE_SAVE_BASE_RULE } from '@/config/app_config.js'
import { getValueByCode } from "@/utils/utils.js"

const props = defineProps({
    parseConfigRule: { type: Object, default: {}, required: true },
    style: { type: Object, required: true },
    init: { type: Boolean, default: false },
})

const showToast = inject('showToast')
const inited = ref(false)
const baseRule = ref({})
const filesInfo = ref([])
const showLoading = ref(false)
const version = ref("")
const initError = ref("")
onMounted(async () => {
})

watch(() => props.init, async (newValue) => {
    if (newValue) {
        if (initError.value) {
            showToast(initError.value)
            return
        }
        if (!isValid.value) {
            showToast(installName.value)
        } else if (!inited.value) {
            console.log(props.parseConfigRule)
            init()
        }
        inited.value = true
    }

})


/**
 * @description: 初始化
 */
async function init() {
    try {
        showLoading.value = true
        version.value = getValueByCode(props.parseConfigRule.variables, "install_version")
        let base = false
        if (USE_SAVE_BASE_RULE) {
            //读取基址缓存
            base = await read(props.parseConfigRule)
            console.log("加载缓存基址")
        }
        if (!base?.version) {
            base = await bridge.searchBaseAddress(props.parseConfigRule)
        }
        let all_supported = base.version
        console.log("基址配置", base)
        base.patches.forEach(item => {
            if (!item.supported) {
                all_supported = false
                showToast(`搜索补丁基址失败: ${item.name}`)
            }
        })
        baseRule.value = base
        if (all_supported ) {
            //写入基址缓存
            if(USE_SAVE_BASE_RULE){
                await save(baseRule.value)
            }
        }else{
            //清除缓存
            clearAll()
        }
        filesInfo.value = await bridge.refreshFilesInfo(baseRule.value)
        console.log(filesInfo.value);
    } catch (error) {
        console.log(error)
        initError.value = `出错了:${error}`
        props.parseConfigRule.supported = false
        clearAll()
        showToast(initError.value)
    } finally {
        showLoading.value = false
    }
}

/**
 * @description: event 消息分派
 * @param {*} payload
 * @return {*}
 */
async function handleEvent(payload) {
    let { code, num, status } = payload
    let fileInfo = filesInfo.value.find(item => item.num == num)
    let feature = fileInfo?.features.find(item => item.code == code)
    if (showLoading.value) {
        feature.status = status
        await nextTick()
        feature.status = !status
        showToast("请等待上一个操作完成")
        return
    }
    try {
        showLoading.value = true
        if (!baseRule.value?.version) {
            await init()
        }
        let method = feature.method || feature.code
        console.log(feature, num, method)
        switch (method) {
            case "refresh":
                filesInfo.value = await bridge.refreshFilesInfo(baseRule.value)
                console.log(filesInfo.value);
                break
            case "del":
                await del(fileInfo)
                break
            case "open":
                await bridge.runApp(feature.target)
                break
            case "floder":
                await bridge.openFolder(feature.target)
                break
            case "patch":
                await applyPatch(feature, fileInfo, status)
                break
            case "coexist":
                await makeCoexist(feature)
                break
            case "clear":
                await clearAll()
                baseRule.value = {}
                break
            default:
                throw new Error(`未实现方法:${method}`)
        }
    } catch (error) {
        console.log(error)
        showToast(error)
    } finally {
        showLoading.value = false
    }
}

/**
 * @description: 删除共存
 * @return {*}
 */
async function del(fileInfo) {
    await bridge.delFiles(fileInfo.usedfiles)
    filesInfo.value = filesInfo.value.filter(item => item.num !== fileInfo.num)
}

/**
 * @description: 执行补丁操作
 * @param code 
 * @param status 
 */
async function applyPatch(feature, fileInfo, status, extFeatures = []) {
    console.log("applyPatch", feature);
    status = feature.style == "switch" ? status : true
    //同步状态
    feature.status = status
    await nextTick()
    let features = [feature, ...extFeatures]
    let dependencies = features.map(item => item.dependencies).flat()
    //await nextTick()
    let patchesFilter = fileInfo.patches.filter(patch =>
        dependencies.some(dep => dep === patch.code)
    )
    //取出依赖的 patch
    if (patchesFilter.length == 0) {
        throw new Error(`未找到依赖补丁:${feature.name}`)
    }
    try {
        // 调用后台执行补丁操作
        console.log(patchesFilter);
        patchesFilter = await bridge.applyPatch(patchesFilter, status)
    } catch (error) {
        //失败，还原weitch状态
        feature.status = !status
        console.log("调用后台执行补丁操作失败")
        throw error
    }
    return fileInfo
}

/**
 * @description: 制作共存
 * @param code 
 * @param status 
 */
async function makeCoexist(feature) {
    //遍历 filesInfo，找到共存文件缺口
    let num = -1
    for (let index = 0; index < 10; index++) {
        if (!filesInfo.value.find(item => item.num == index)) {
            //找到第一个不存在的文件
            num = index
            break
        }
    }
    if (num == -1) {
        throw new Error("共存文件已满")
    }
    // 构建共存文件信息
    let fileInfo = await bridge.buildFileInfoByNum(baseRule.value, num)
    //从主程序 feature 切换到当前共存文件 feature
    let mainFileInfo = filesInfo.value.find(fileInfo => fileInfo.ismain)
    let nowFeature = mainFileInfo.features.find(item => item.code == feature.code)
    //同步主程序状态
    //过滤出主程序激活i的 feature
    const mainActivedFeatures = mainFileInfo.features.filter(feature => feature.status)
    //设置当前程序功能状态同步主程序
    const extFeatures = fileInfo.features.filter(feature => mainActivedFeatures.some(item => item.code === feature.code)).forEach(feature => {
        feature.status = true
    })
    //打补丁，保存文件
    await applyPatch(nowFeature, fileInfo, true, extFeatures)
    //添加的 fileInfo 到 filesInfo 中，展示到页面上
    //是否存在
    if (!filesInfo.value.find(item => item.num == num)) {
        filesInfo.value.push(fileInfo)
        filesInfo.value.sort((a, b) => a.index - b.index)
    }
}

const installName = computed(() => {
    if (!props.parseConfigRule.installed) {
        return `未安装: ${props.parseConfigRule.name}`
    } else if (!props.parseConfigRule.supported) {
        return `不支持此版本: ${version.value}`
    } else {
        return `安装的版本: ${version.value}`
    }
})

/**
 * @description: 是否安装且支持
 * @param {*} computed
 * @return {*}
 */
const isValid = computed(() => {
    return props.parseConfigRule.installed && props.parseConfigRule.supported
})

/**
 * @description: tag1标签样式
 * @param {*} computed
 * @return {*}
 */
const tagSeverity1 = computed(() => {
    return !isValid.value ? "danger" : "success"
})

/**
 * @description: tag2标签样式
 * @param {*} computed
 * @return {*}
 */
const tagSeverity2 = computed(() => {
    return !props.parseConfigRule.supported ? "danger" : "success"
})


</script>