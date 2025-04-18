<template>
  <!-- 加载中 -->
  <splash @loaded="loaded" />
  <!-- 加载完成 -->
  <div v-if="parseConfig.version" class="container">

    <Tabs v-model:value="tabIndex">
      <TabList>
        <Tab value="readme">说明</Tab>
        <template v-for="(parseConfigRule, index) in parseConfig.rules" :key="index">
          <Tab :value="parseConfigRule.code">{{ parseConfigRule.name }}</Tab>
        </template>
      </TabList>
      <TabPanels>
        <TabPanel value="readme">
          <ReadmePanel :style="scrollStyle" :updateInfoReadme="updateInfo.readme"/>
        </TabPanel>
        <template v-for="(parseConfigRule, index) in parseConfig.rules" :key="index">
          <TabPanel :value="parseConfigRule.code">
            <PatchPanel :style="scrollStyle" :init="tabIndex == parseConfigRule.code" :parseConfigRule="parseConfigRule" />
          </TabPanel>
        </template>
      </TabPanels>
    </Tabs>
  </div>
  <Toast position="bottom-left" />
</template>

<script setup>
import { ref,provide, computed, onMounted } from "vue"
import Splash from "@/components/splash.vue"
import PatchPanel from "@/components/patch-panel.vue"
import ReadmePanel from "@/components/readme-panel.vue"
import { useToast } from "primevue/usetoast"
import { MESSAGE_LIFE } from "@/config/app_config.js"

provide('showToast', showToast)

const toast = useToast()
const parseConfig = ref({})
const updateInfo = ref({})
const tabIndex = ref("readme")

onMounted(async () => {
  disableRefresh()
})

/**
 * @description: 禁用刷新，禁止使用 F5 刷新
 */
const disableRefresh = () => {
  if(import.meta.env.DEV) return
  document.addEventListener('keydown', function (event) {
    if (
      event.key === 'F5' || 
      event.key === 'F12' ||
      (event.ctrlKey && event.key === 'r') ||
      (event.metaKey && event.key === 'r')
    ) {
      event.preventDefault()
    }
  })

  document.addEventListener('contextmenu', function (event) {
    event.preventDefault()
  })
}

/**
 * @description: 接收从 splash 传递过来的消息
 * @return {*}
 */
function loaded(payload) {
  parseConfig.value = payload.data.parseConfig
  console.log("解析config",parseConfig.value);
  updateInfo.value = payload.data.updateInfo
  console.log("updateInfo",updateInfo.value )
}

/**
 * @description: 滚动区域高度
 * @param {*} computed
 * @return {*}
 */
const scrollStyle = computed(() => {
  return {
    height: `calc(100vh - 62px)`,
    width: '100%'
  }
})

/**
 * @description: toast
 * @param {*} payload
 * @return {*}
 */
function showToast(payload) {
  payload = payload.message? payload.message : payload
  let defaultPayload = {
    summary: "消息",
    detail: "", 
    severity: "error",
    life : MESSAGE_LIFE,
    error: true
  }
  if (typeof payload === "string") {
    defaultPayload.detail = payload
  }else{
    defaultPayload = {...defaultPayload, ...payload}
  }
  defaultPayload.severity = defaultPayload.error ? "error" : "contrast"
  toast.add(defaultPayload)
}
</script>
