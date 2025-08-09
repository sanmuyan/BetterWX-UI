<template>
    <div v-if="!update.version">
        <splash @loaded="loaded" />
    </div>
    <div v-else>
        <Tabs v-model:value="tabIndex" class="h-screen flex flex-col">
            <TabList class="!text-center">
                <Tab value="readme" class="!pt-0 !pb-0">说明</Tab>
                <Tab v-for="(rule, index) in config" :key="index" :value="rule.code" class="!pt-0 !pb-0 h-14">{{
                    rule.name }}</Tab>
            </TabList>
            <TabPanels class="!pt-0 !pb-0 flex-1 min-h-0">
                <TabPanel value="readme" class="flex flex-col h-full">
                    <ReadmePanel :init="tabIndex == 'readme'" :data="update.readme"></ReadmePanel>
                </TabPanel>
                <TabPanel v-for="(rule, index) in config" :key="index" :value="rule.code" class="flex flex-col h-full">
                    <PatchPanel :init="tabIndex == rule.code" :data="rule"></PatchPanel>
                </TabPanel>
            </TabPanels>
        </Tabs>
    </div>
    <Toast position="bottom-left" />
</template>

<script setup>
import { ref, watch, provide, onMounted } from "vue"
import Splash from "@/components/splash.vue"
import ReadmePanel from "@/components/readme-panel.vue"
import PatchPanel from "@/components/patch-panel.vue"
import { useToast } from "primevue/usetoast"
import * as storeApis from "@/apis/store.js"

provide("showToast", showToast)
provide("showToastInfo", showToastInfo)

const toast = useToast()
const update = ref({})
const config = ref([])
const tabIndex = ref()

watch(tabIndex, (newValue, oldValue) => {
    if (oldValue == newValue) return
    storeApis.store_save("tabIndex", newValue)
})

onMounted(async () => {
    disableRefresh()
    tabIndex.value = await getIndex()
})

function loaded(data) {
    console.log(data);
    update.value = data.update
    config.value = data.config
    console.log(update.value.readme);
}

async function getIndex() {
    try {
        let index = await storeApis.store_read("tabIndex")
        if (index) {
            return index
        }else{
            return "readme"
        }
    } catch (error) {
        return "readme"
    }
}

function showToast(payload) {
    payload = payload.message ? payload.message : payload
    let defaultPayload = {
        summary: "消息",
        detail: "",
        severity: "error",
        life: 2500
    }
    if (typeof payload === "string") {
        defaultPayload.detail = payload
    } else {
        defaultPayload = { ...defaultPayload, ...payload }
    }
    defaultPayload.severity = defaultPayload.error ? "error" : "contrast"
    toast.add(defaultPayload)
}

function showToastInfo(payload) {
    payload = payload.message ? payload.message : payload
    let defaultPayload = {
        summary: "消息",
        detail: "",
        severity: "success",
        life: 2000
    }
    if (typeof payload === "string") {
        defaultPayload.detail = payload
    } else {
        defaultPayload = { ...defaultPayload, ...payload }
    }
    defaultPayload.severity = defaultPayload.severity ? defaultPayload.severity : "contrast"
    toast.add(defaultPayload)
}

const disableRefresh = () => {
    if (import.meta.env.DEV) return
    document.addEventListener("keydown", function (event) {
        if (
            event.key === "F5" ||
            event.key === "F12" ||
            (event.ctrlKey && event.key === "r") ||
            (event.metaKey && event.key === "r")
        ) {
            event.preventDefault()
        }
    })
    document.addEventListener("contextmenu", function (event) {
        event.preventDefault()
    })
}
</script>