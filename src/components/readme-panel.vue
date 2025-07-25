<template>
    <ScrollPanel class="flex-1 min-h-0">
        <div class="mx-4 " style="line-height:3rem;" v-html="markdown.render(readmeContent)" @click="handleLinkClick">
        </div>
    </ScrollPanel>
    <Loading :show="showLoading" />
</template>

<script setup>
import { ref, watch, nextTick } from 'vue'
import { update_readme_check } from '@/apis/update.js'
import { cmd_open_url } from '@/apis/cmd.js'

const props = defineProps({
    data: { type: Object, default: {} },
    init: { type: Boolean, default: false }
})

import MarkdownIt from "markdown-it"
const markdown = new MarkdownIt({
    html: true
})

const inited = ref(false)
const initError = ref("")
const showLoading = ref(false)
const readmeContent = ref("")

watch(() => props.init, async (newValue) => {
    if (newValue) {
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

async function init() {
    await nextTick()
    showLoading.value = true
    try {
        readmeContent.value = await update_readme_check(props.data)
    } catch (error) {
        console.log(error);
        readmeContent.value = "# 出错了\n" + error
    } finally {
        showLoading.value = false
    }
}

function handleLinkClick(event) {
    if (event.target.tagName === 'A') {
        event.preventDefault()
        console.log(event.target.href);
        cmd_open_url(event.target.href)
    }
}
</script>
 