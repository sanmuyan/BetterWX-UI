<template>
    <ScrollPanel class="flex-1 min-h-0">
        <div class="mx-4 " style="line-height:3rem;" v-html="markdown.render(readmeContent)" @click="handleLinkClick">
        </div>
    </ScrollPanel>
    <Loading :show="showLoading" />
</template>

<script setup>
import { ref, watch, nextTick } from "vue"
import { update_readme_check } from "@/apis/update.js"
import { cmd_open_url } from "@/apis/cmd.js"
import MarkdownIt from "markdown-it"
import markdownItClass from "markdown-it-class"

const props = defineProps({
    data: { type: Object, default: {} },
    init: { type: Boolean, default: false }
})

const markdown = new MarkdownIt({
    html: true
}).use(markdownItClass, {
    h1: "text-4xl font-bold my-4",
    h2: "text-3xl font-bold my-3",
    h3: "text-2xl font-bold my-2",
    h4: "text-xl font-bold my-2",
    h5: "text-lg font-bold my-1",
    h6: "text-base font-bold my-1",
    p: "my-2",
    a: "text-blue-500 hover:text-blue-700 underline",
    ul: "list-disc pl-5 my-2",
    ol: "list-decimal pl-5 my-2",
    blockquote: "border-l-4 border-gray-300 pl-4 italic my-2",
    code: "bg-gray-100 rounded px-1 py-0.5 text-sm font-mono",
    pre: "bg-gray-100 rounded p-2 my-2 overflow-x-auto"
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
    if (event.target.tagName === "A") {
        event.preventDefault()
        console.log(event.target.href);
        cmd_open_url(event.target.href)
    }
}
</script>