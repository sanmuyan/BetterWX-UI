<template>
  <!-- 加载中 -->
  <splash @loaded="loaded" />
  <!-- 加载完成 -->
  <div v-if="config.version" class="container">
    <Tabs v-model:value="tabIndex">
      <TabList>
        <Tab value="readme">说明</Tab>
        <template v-for="(configRule, index) in config.rules" :key="index">
          <Tab :value="configRule.code">{{ configRule.name }}</Tab>
        </template>
      </TabList>
      <TabPanels class="p-t-0">
        <TabPanel value="readme">
          <ReadmePanel :style="scrollTotalStyle" :updateInfoReadme="updateInfo.readme" />
        </TabPanel>
        <template v-for="(configRule, index) in config.rules" :key="index">
          <TabPanel :value="configRule.code">
            <PatchPanel :style="scrollStyle" :init="tabIndex == configRule.code"
              :configRule="configRule" />
          </TabPanel>
        </template>
      </TabPanels>
    </Tabs>
  </div>
  <Toast position="bottom-left" />
</template>

<script setup>
import { ref, watch, provide, computed, onMounted,nextTick } from "vue"
import Splash from "@/components/splash.vue"
import PatchPanel from "@/components/patch-panel.vue"
import ReadmePanel from "@/components/readme-panel.vue"
import { useToast } from "primevue/usetoast"
import { MESSAGE_LIFE, TEST_MODE } from "@/config/app_config.js"
import { read, save } from "@/utils/store.js"

provide('showToast', showToast)

const toast = useToast()
const config = ref({})
const updateInfo = ref({})
const tabIndex = ref()

onMounted(async () => {
  disableRefresh()
  let storeData = {
    tabIndex: ""
  }
  let index = await read(storeData)
  tabIndex.value = index.tabIndex ? index.tabIndex : "readme"
  await nextTick()
})

watch(tabIndex, (newValue) => {
  let storeData = {
    tabIndex: newValue
  }
  save(storeData)
})

/**
 * @description: 禁用刷新，禁止使用 F5 刷新
 */
const disableRefresh = () => {
  if (TEST_MODE) return
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
  config.value = payload.data.config
  console.log("解析config", config.value);
  updateInfo.value = payload.data.updateInfo
  console.log("updateInfo", updateInfo.value)
}

/**
 * @description: 全部滚动区域高度
 * @param {*} computed
 * @return {*}
 */
const scrollTotalStyle = computed(() => {
  return {
    height: `calc(100vh - 44px)`,
    width: '100%'
  }
})

/**
 * @description: 滚动区域高度
 * @param {*} computed
 * @return {*}
 */
const scrollStyle = computed(() => {
  return {
    height: `calc(100vh - 142px)`,
    width: '100%'
  }
})

/**
 * @description: toast
 * @param {*} payload
 * @return {*}
 */
function showToast(payload) {
  payload = payload.message ? payload.message : payload
  let defaultPayload = {
    summary: "消息",
    detail: "",
    severity: "error",
    life: MESSAGE_LIFE,
    error: true
  }
  if (typeof payload === "string") {
    defaultPayload.detail = payload
  } else {
    defaultPayload = { ...defaultPayload, ...payload }
  }
  defaultPayload.severity = defaultPayload.error ? "error" : "contrast"
  toast.add(defaultPayload)
}
</script>
