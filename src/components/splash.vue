<template>
    <div v-if="showLoading" class="h-screen w-screen flex justify-center items-center select-none">
        <div class="flex flex-col w-full h-40">
            <ProgressSpinner style="width: 50px; height: 50px" strokeWidth="8" fill="transparent" />
            <TransitionGroup name="fade" tag="div" class="text-center w-full h-60 overflow-hidden">
                <div v-for="(message, index) in mesages" :key="index" :style="{ opacity: 1 - index * 0.15 }"
                    class="w-full text-center">
                    {{ message }}
                </div>
            </TransitionGroup>
        </div>
    </div>
    <!-- 检测到软件更新 -->
    <Dialog class="not-select" v-model:visible="showUpdateDialog" modal header="提示" style="width: 20rem;">
        <div class="flex flex-col">
            <label class="mb-1">{{ `发现新版本: ${update.nversion}` }}</label>
            <label class="mb-2 text-prewarp">{{ update.description }}</label>
            <div class="flex items-center justify-center space-x-4 my-4">
                <template v-for="(button, index) in update.buttons" :key="index">
                    <Button :label="button.name" @click="openUrl(button.url)" size="small"
                        :severity="button.severity ? button.severity : ''" />
                </template>
            </div>
        </div>
    </Dialog>
</template>

<script setup>
import { onMounted, ref,computed } from "vue"
import { sleep } from "@/utils/tools.js"
import * as updateApiss from "@/apis/update.js"
import { Window } from "@tauri-apps/api/window"
import { getVersion, getName } from "@tauri-apps/api/app"

const update = ref({})
const showLoading = ref(true)
const mesages = ref([])

onMounted(() => {
    check_update()
})

async function check_update() {
    try {
        await addMsg("检查软件更新...")
        update.value = await updateApiss.update_check()
        const appVersion = await getVersion()
        const appName = await getName()
        let info = update.value
        setTitle(info.name || appName, appVersion)
        if (info.nversion) {
            await addMsg(`发现新版本 v${info.nversion}，请更新`)
            if (info.force) {
                return
            }else{
                await sleep(2000)
            }
        }
        console.log(info);
        await addMsg("获取配置文件...")
        let config = await updateApiss.update_config_check(info.config)
        console.log(config);
        await addMsg(`软件初始化完成！`)
        loaded({
            config,
            update: info
        })

    } catch (error) {
        await addMsg(`软件初始化失败！`)
        await addMsg(`${error}`)
        console.error(error);
    }
}

const emit = defineEmits(["loaded"])
function loaded(data) {
    emit("loaded", data)
}

function setTitle(name, version) {
    const currentWindow = Window.getCurrent()
    currentWindow.setTitle(`${name} v${version}`)
}

async function addMsg(msg, delay = 0) {
    mesages.value.unshift(msg)
    if (mesages.value.length > 5) {
        mesages.value.pop()
    }
    await sleep(delay)
}

const showUpdateDialog = computed(() => {
    return update.value.nversion && update.value.force
})
</script>