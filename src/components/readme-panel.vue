<template>
    <ScrollPanel :style="style">
        <div class="m-x " style="line-height:3rem;" v-html="markdown.render(readmeContent)" @click="handleLinkClick">
        </div>
    </ScrollPanel>
    <Loading :show="showLoading" />
</template>

<script setup>
import { ref, onMounted } from "vue"
import { http } from "@/utils/http.js"
import Loading from "@/components/loading.vue"
import { USE_SAVE_README } from "@/config/app_config.js"
import { read, save } from "@/utils/store.js"
import { openUrl } from "@/utils/bridge.js"

import MarkdownIt from "markdown-it"
const markdown = new MarkdownIt({
    html: true
})

const showLoading = ref(false)
const readmeContent = ref("")

const props = defineProps({
    style: { type: Object, required: true },
    updateInfoReadme: { type: Object, default: {} }
})

onMounted(async () => {
    getReadme()
})

/**
 * @description: 获取 readme 内容
 * @return {*}
 */
async function getReadme() {
    let content = "# 暂无内容"
    let loadContent = false
    showLoading.value = true
    let saveData = {
        version: props.updateInfoReadme.version,
        content,
    }
    if (USE_SAVE_README) {
        //读取缓存readme
        loadContent = (await read(saveData))?.content
        console.log("加载缓存readme")
    }
    if (!loadContent) {
        try {
            loadContent = await http(props.updateInfoReadme.url + "?r=" + Math.random(), { text: true })
            saveData.content = loadContent
            if (USE_SAVE_README) {
                //保存readme
                console.log("保存readme")
                await save(saveData)
            }
        } catch (error) {
            content = `# 加载失败\n${error}`
            console.log(error)
        }
    }
    showLoading.value = false
    readmeContent.value = loadContent || content
}

// 添加链接点击处理函数
function handleLinkClick(event) {
    if (event.target.tagName === 'A') {
        event.preventDefault()
        console.log(event.target.href);
        openUrl(event.target.href)
    }
}
</script>