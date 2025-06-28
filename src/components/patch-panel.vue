<template>
    <div class="m-x not-select">
        <CoexistList :fileInfo="FeatureFileInfo" :rule="baseRule" :mainFileInfo="filesInfo?.[0]"
            :note="getNote(FeatureFileInfo.num)" @event="handleEvent" class="border-b">
        </CoexistList>
    </div>
    <ScrollPanel :style="style">
        <div class="m-x not-select">
            <div v-if="parseRule.installed">
                <template v-for="(fileInfo) in filesInfo" :key="fileInfo.num">
                    <CoexistList :fileInfo="fileInfo" :mainFileInfo="filesInfo?.[0]" :rule="baseRule"
                        :note="getNote(fileInfo.num)" @event="handleEvent" class="border-b">
                    </CoexistList>
                </template>
            </div>
            <Loading :show="showLoading" />
        </div>
    </ScrollPanel>
    <div class="float-bottom border-t">
        <div class="flex row items-center justify-between"
            style="height: 48px;padding: var(--p-tabs-tabpanel-padding);">
            <div>
                <Tag :value="installName" :severity="tagSeverity1" class="m-r"></Tag>
                <Tag :value="`特征码版本: ${parseRule.version}`" :severity="tagSeverity2" class="m-r"></Tag>
            </div>
            <div>
                <div v-if="parseRule.news">
                    <span>{{parseRule.news}}!</span>
                </div>
            </div>
        </div>
    </div>
    <!-- input -->
    <Dialog class="not-select" v-model:visible="inputDialog.show" modal :header="inputDialog.title"
        style="width: 30rem;" :closable="false">
        <div class="flex col">
            <template v-for="(item, index) in inputDialog.texts" :key="index">
                <div class="flex col justify-center gap m-b">
                    <b class="text-ellipsis">{{ item.label }}</b>
                    <InputText class="w-100" type="text" v-model="item.text"></InputText>
                </div>
            </template>
            <div class="flex justify-end gap  m-y">
                <Button type="button" label="取消" severity="secondary" @click="inputCancle" size="small"></Button>
                <Button type="button" label="确认" @click="inputConfirm" size="small"></Button>
            </div>
        </div>
    </Dialog>
</template>

<script setup>
import { ref, watch, inject, computed, onMounted, nextTick } from "vue"
import CoexistList from "@/components/coexist-list.vue"
import Loading from "@/components/loading.vue"
import * as bridge from "@/utils/bridge.js"
import { read, save, clearAll } from "@/utils/store.js"
import { USE_SAVE_BASE_RULE } from '@/config/app_config.js'
import { getValueByCode, fixCodePrefix, getStatusBycCdePrefix, codePrefixType, textToBigHex, bigHexToText } from "@/utils/utils.js"

const props = defineProps({
    configRule: { type: Object, default: {}, required: true },
    style: { type: Object, required: true },
    init: { type: Boolean, default: false }
})

const showToast = inject('showToast')
const inited = ref(false)
const parseRule = ref({})
const baseRule = ref({})
const filesInfo = ref([])
const FeatureFileInfo = ref({})
const showLoading = ref(false)
const version = ref(false)
const initError = ref("")
const notes = ref([])
const inputDialog = ref({})

onMounted(async () => {

})

watch(() => props.init, async (newValue) => {
    if (newValue) {
        await nextTick()
        if (initError.value) {
            showToast(initError.value)
            return
        }
        if (!inited.value) {
            init()
            inited.value = true
        }
    }
}, { immediate: true })



/**
 * @description: 初始化
 */
async function init() {
    try {
        showLoading.value = true
        console.log("原始Rule ", props.configRule)
        parseRule.value = await bridge.parseRule(props.configRule)
        console.log("解析后的Rule", parseRule.value)
        if (!parseRule.value.installed || !parseRule.value.supported) {
            showToast(installName.value)
            return
        }
        version.value = version.value || getValueByCode(parseRule.value.variables, "install_version")
        let base = false
        if (USE_SAVE_BASE_RULE) {
            //读取基址缓存
            base = await read(parseRule.value)
            console.log("取基址缓存", base)
        }
        if (!base?.version) {
            base = await bridge.searchBaseAddress(parseRule.value)
        }
        console.log("基址配置", base)
        let all_notpatched = base.patches.every(item => !item.patched)
        if (!all_notpatched) {
            await bridge.removePatchesBackupFiles(parseRule.value.patches)
            throw new Error(`备份文件无效，请尝试重装WX`)
        }
        let all_supported = base.patches.every(item => item.supported)
        baseRule.value = base
        if (all_supported) {
            //写入基址缓存
            if (USE_SAVE_BASE_RULE) {
                await save(baseRule.value)
            }
        } else {
            showToast(`部分或全部功能不支持，已禁用`)
            //清除缓存
            clearAll()
        }
        FeatureFileInfo.value = await bridge.buildFeatureFileInfo(baseRule.value)
        console.log("功能区", FeatureFileInfo.value);
        filesInfo.value = await bridge.refreshFilesInfo(baseRule.value)
        console.log("构建的filesInfo.value", filesInfo.value);
        //加载选中状态
        await getSelectAll()
        //加载备注
        await getNotes()
    } catch (error) {
        console.log(error)
        initError.value = `${error}`
        parseRule.value.supported = false
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
    let fileInfo = num == "Y" ? FeatureFileInfo.value : filesInfo.value.find(item => item.num == num)
    console.log(payload, fileInfo);
    let feature = fileInfo?.features.find(item => item.code == code)
    if (showLoading.value) {
        showToast("请等待上一个操作完成")
        return
    }
    await nextTick()
    try {
        showLoading.value = true
        if (!baseRule.value?.version) {
            await init()
        }
        let method = feature.method || feature.code
        console.log(feature, num, method)
        switch (method) {
            case "refresh":
                await init()
                break
            case "del":
                await del(fileInfo)
                break
            case "open":
                await bridge.runApps([feature.target], false, false)
                break
            case "floder":
                await bridge.openFolder(feature.target)
                break
            case "patch":
                await doPatch(fileInfo, feature, status)
                break
            case "patch_input":
                await doPatchInput(fileInfo, feature)
                break
            case "coexist":
                await makeCoexist(feature)
                break
            case "clear":
                await clearAll()
                baseRule.value = {}
                break
            case "note":
                await setNote(fileInfo)
                break
            case "select":
                await setSelectAll(status, num)
                break
            case "select_all":
                await setSelectAll(status)
                break
            case "open_all":
                await openAll(true)
                break
            case "close":
                await bridge.closeApps([feature.target])
                break
            case "close_all":
                await openAll(false)
                break
            default:
                let name = feature.name || method
                throw new Error(`未实现方法:${name}`)
        }
    } catch (error) {
        //还原功能状态
        feature.status = !status
        await nextTick()
        console.log(error)
        showToast(error)
    } finally {
        showLoading.value = false
    }
}

/**
 * @description: 运行全部
 * @return {*}
 */
async function openAll(status) {
    let filesInfoFilter = filesInfo.value.filter(item => item.features.find(feature => feature.code == "select" && feature.selected))
    let files = []
    for (let fileInfo of filesInfoFilter) {
        let feature = fileInfo.features.find(item => item.code == "open")
        files.push(feature.target)
    }
    if (files.length == 0) {
        showToast("请选择需要启动/关闭的程序")
        return
    }
    if (status) {
        await bridge.runApps(files, true, true)
    } else {
        await bridge.closeApps(files)
    }
}

/**
 * @description: 设置全选反选
 * @return {*}
 */
async function setSelectAll(status, num) {
    let selectedItems = []
    filesInfo.value.forEach(item => {
        item.features.forEach(feature => {
            if (feature.code == "select") {
                if (num) {
                    feature.selected = num == "Y" || item.num == num ? status : feature.selected
                } else {
                    feature.selected = status
                }
                if (feature.selected) {
                    selectedItems.push(item.num)
                }
            }
        })
    })
    let storeData = {
        code: baseRule.value.code,
        selected: selectedItems
    }
    await save(storeData)
}

/**
 * @description: 设置全选反选
 * @return {*}
 */
async function getSelectAll() {
    let storeSelecteds = await read({
        code: baseRule.value.code,
        selected: []
    })
    let selected = storeSelecteds?.selected || []
    filesInfo.value.forEach(item => {
        item.features.forEach(feature => {
            if (feature.code == "select") {
                feature.selected = selected.includes(item.num)
            }
        })
    })
    FeatureFileInfo.value.features.forEach(feature => {
        if (feature.code == "select_all") {
            feature.selected = selected.length == filesInfo.value.length
        }
    })
}

/**
 * @description: 从存储加载所有备注
 * @return {*}
 */
async function getNotes() {
    let storeNotes = await read({
        code: baseRule.value.code,
        notes: []
    })
    notes.value = Array.isArray(storeNotes?.notes) && storeNotes?.notes ?storeNotes?.notes : []
    console.log("加载备注", notes.value);
}

/**
 * @description: 设置备注
 * @return {*}
 */
async function setNote(fileInfo) {
    let index = notes.value?.findIndex(item => item.num == fileInfo.num)
    let text = notes.value?.[index]?.note || ""
    // 打开对话框并等待用户输入
    console.log();
    let texts = [{
        text: text,
        maxLen: 32,
    }]
    let noteText = await getInputValue(texts, "请输入备注")
    if (!inputDialog.value.confirm) return;  // 用户取消输入
    console.log(noteText[0]);
    noteText = noteText?.[0].result.substring(0, 32)
    if (index == -1) {
        notes.value.push({
            num: fileInfo.num,
            note: noteText
        })
    } else {
        notes.value[index].note = noteText.trim()
    }
    let storeData = {
        code: baseRule.value.code,
        notes: notes.value
    }
    await save(storeData)
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
 * 输入类型的补丁
 * @param fileInfo 
 * @param feature 
 */
async function doPatchInput(fileInfo, feature) {
    //检查前置功能
    let dependfeature = feature.dependfeature || []
    if (dependfeature.length > 0) {
        dependfeature.forEach(code => {
            let find = fileInfo.features.find(item => item.code == code)
            if (!find || !find.status) {
                throw new Error(`请先开启 ${find.name || find.code} 功能`)
            }
            return true
        })
    }
    let dependencies = feature.dependencies
    let patchesFilter = filterPatchesByDependencies(fileInfo, dependencies)
    if (patchesFilter.length == 0) {
        throw new Error(`找不到 ${feature.name || feature.code} 对应的补丁数据`)
    }
    let texts = []
    patchesFilter.forEach(patch => {
        let text = patch.replace || patch.origina || patch.pattern || ""
        let hasZero = text.endsWith("00")
        if (text.length == 0) {
            throw new Error(`补丁特征码错误`)
        }
        texts.push({
            code: patch.code,
            text: bigHexToText(text),
            encode: true,
            hasZero,
            maxLen: text.length / 2,
            label: patch.description || patch.name || patch.code,
        })
    })
    let title = feature.description || feature.name || feature.code
    texts = await getInputValue(texts, title)
    if (!inputDialog.value.confirm || !texts) return;
    let patches = []
    patchesFilter.forEach(patch => {
        let find = texts.find(item => item.code == patch.code)
        let desc = patch.description || patch.name || patch.code
        if (!find) {
            throw new Error(`未找到 ${desc} 的输入`)
        }
        let notInputLen = find.hasZero ? 2 : 0
        if (find.result.length <= notInputLen) {
            throw new Error(`输入的 ${desc} 的不能为空`)
        }
        if (find.result.length < patch.origina.length) {
            find.result += patch.origina.substring(find.result.length)
        }
        patch.replace = find.result
        patch.status = true
        patches.push(patch)
    })
    console.log(patches);
    if (patches.length == 0) {
        throw new Error(`应用 ${title} 的补丁数据为空`)
    }
    return applyPatches(fileInfo, patches)
}

/**
 * @description: 执行补丁操作
 * @param code 
 * @param status 
 */
async function doPatch(fileInfo, feature, status, extFeatures = []) {
    console.log("applyPatch", feature);
    status = feature.style == "switch" ? status : true
    //同步状态
    feature.status = status
    //await nextTick()
    await nextTick()
    let features = [feature, ...extFeatures]
    let dependencies = features.map(item => item.dependencies).flat()
    let patchesFilter = filterPatchesByDependencies(fileInfo, dependencies, status)
    return applyPatches(fileInfo, patchesFilter)
}

/**
 * 通过依赖过滤出需要打补丁的文件
 * @param fileInfo 
 * @param dependencies 
 */
function filterPatchesByDependencies(fileInfo, dependencies, status) {
    let patchesFilter = []
    dependencies.forEach(code => {
        let fixedCode = fixCodePrefix(code)
        let find = fileInfo.patches.find(item => item.code == fixedCode && item.supported)
        if (!find) {
            //如果是 - 开头，可以忽略
            if (codePrefixType(code) !== 3) {
                throw new Error(`未找到 ${code} 的依赖`)
            }
        } else {
            let newStatus = getStatusBycCdePrefix(code, status)
            let newPatch = JSON.parse(JSON.stringify(find))
            newPatch.status = newStatus
            patchesFilter.push(newPatch)
        }
    })
    return patchesFilter
}

/**
 * @description: 应用补丁
 * @param fileInfo 
 * @param patches 
 */
async function applyPatches(fileInfo, patches) {
    //取出依赖的 patch
    if (patches.length == 0) {
        throw new Error(`应用补丁数据无效`)
    }
    console.log("调用后台执行补丁操作", patches);
    // 调用后台执行补丁操作
    patches = await bridge.applyPatch(patches)
    //重置所有featrues status
    fileInfo.features.forEach(feature => {
        feature.status = feature.dependencies.every(code => {
            if (!code) {
                return feature.status;
            }
            const fixedCode = fixCodePrefix(code);
            const patch = patches.find(p => p.code === fixedCode);
            const needStatus = getStatusBycCdePrefix(code, true);
            if (!patch) {
                return feature.status;
            }
            return !needStatus || (patch.patched === needStatus);
        });
    })
    console.log("修补后的fileinfo", fileInfo);
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
    console.log("fileInfo", fileInfo);
    //从主程序 feature 切换到当前共存文件 feature
    let mainFileInfo = filesInfo.value.find(fileInfo => fileInfo.ismain)
    let nowFeature = mainFileInfo.features.find(item => item.code == feature.code)
    console.log("nowFeature", nowFeature);
    if (!nowFeature) {
        throw new Error(`找不到 ${feature.name || feature.code} 对应的功能数据`)
    }
    // //同步主程序状态
    // //过滤出主程序激活i的 feature
    // const mainActivedFeatures = mainFileInfo.features.filter(feature => feature.status)
    // //设置当前程序功能状态同步主程序
    // const extFeatures = fileInfo.features.filter(feature => mainActivedFeatures.some(item => item.code === feature.code)).forEach(feature => {
    //     feature.status = true
    // })
    //打补丁，保存文件
    await doPatch(fileInfo, nowFeature, true, [])
    //添加的 fileInfo 到 filesInfo 中，展示到页面上
    //是否存在
    fileInfo.features.sort((a, b) => a.index - b.index)
    //设置默认select为true
    fileInfo.features.map(item => {
        if (item.code == "select") {
            item.selected = true
        }
    })
    if (!filesInfo.value.find(item => item.num == num)) {
        filesInfo.value.push(fileInfo)
        filesInfo.value.sort((a, b) => a.index - b.index)
    }
    setSelectAll(true, num)
}

/**
 * @description: 获取输入值
 * @return {Promise<Array<String>>} 用户输入的值
 */
function getInputValue(texts, title) {
    title = title || "请输入"
    texts = texts || [{}]
    return new Promise((resolve) => {
        let lastShowLoading = showLoading.value
        showLoading.value = false
        inputDialog.value = {
            show: true,
            texts: texts,
            title,
            confirm: false
        }
        // 监听对话框关闭
        const unwatch = watch(inputDialog, (newVal) => {
            if (!newVal.show) {
                unwatch()  // 停止监听
                showLoading.value = lastShowLoading
                resolve(inputDialog.value.texts)
            }
        }, { deep: true })
    })
}

/**
 * @description: 取消输入
 * @return {*}
 */
function inputCancle() {
    inputDialog.value.show = false
}

/**
 * @description: 确认输入
 * @return {*}  
 */
function inputConfirm() {
    for (let index = 0; index < inputDialog.value.texts.length; index++) {
        const item = inputDialog.value.texts[index];
        item.text = item.text.trim()
        let newText = item.text
        let textLength = newText.length
        if (item.encode) {
            newText = textToBigHex(newText, item.hasZero, item.maxLen)
            textLength = newText.length / 2
        }
        //长度
        if (item.maxLen && textLength > item.maxLen) {
            let prefix = item.label ? `输入 ${item.label} 太长了，` : ""
            let suffix = item.encode ? `，[1个中文长度为3]` : ""
            showToast(`${prefix}输入长度为：${textLength}，最大长度为：${item.maxLen}。${suffix}`)
            return
        }
        item.result = newText
    }
    inputDialog.value.confirm = true
    inputDialog.value.show = false
}

/**
 * @description: 获取备注
 * @param {*} computed
 * @return {*}
 */
const getNote = computed(() => (num) => {
    let note = notes.value?.find(item => item.num == num)?.note || ""
    return note
})

const installName = computed(() => {
    if (!parseRule.value.installed) {
        return `未检测到： ${parseRule.value.name || parseRule.value.code}`
    } else if (!parseRule.value.supported) {
        return `不支持此版本： ${version.value}`
    } else {
        return `安装的版本： ${version.value}`
    }
})

/**
 * @description: 是否安装
 * @param {*} computed
 * @return {*}
 */
const isValid = computed(() => {
    return parseRule.value.installed && parseRule.value.supported
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
    return !parseRule.value.supported ? "danger" : "success"
})


</script>