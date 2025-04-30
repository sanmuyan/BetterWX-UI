<template>
    <div class="flex row justify-between items-center" style="height: 48px;">
        <div class="flex col justify-between">
            <div class="flex row items-center gap">
                <b class="m-r">{{ fileInfo.name }}</b>
                <template v-for="(feature, index) in fileInfo.features" :key="index">
                    <div v-if="isShow(feature, 'checkbox')" class="flex items-center gap m-r">
                        <Checkbox v-model="feature.status" @change="checkBoxChange(feature)" binary />
                    </div>
                    <div v-if="isShow(feature, 'switch')" class="flex items-center gap m-r">
                        <label :for="feature.code">{{ feature.name }}</label>
                        <ToggleSwitch v-tooltip.top="toolTips(feature)" :inputId="feature.code" v-model="feature.status"
                            :disabled="feature.disabled || !feature.supported" :invalid="!feature.supported"
                            @value-change="handleEvent($event, feature)" />
                    </div>
                    <Button v-if="isShow(feature, 'button')" v-tooltip.top="toolTips(feature)" :label="feature.name"
                        @click="handleEvent($event, feature)" size="small"
                        :severity="feature.severity ? feature.severity : ''"
                        :disabled="feature.disabled || !feature.supported" />
                </template>
            </div>
            <b v-if="note && hasFeature('note')" class="m-r text-ellipsis"
                style="font-size: 0.8rem;width: 40vw;height: 1rem;">{{note }}</b>
        </div>
        <div class="flex gap">
            <template v-for="(feature, index) in fileInfo.features" :key="index">
                <Button v-if="isShow(feature, 'button', true)" v-tooltip.top="toolTips(feature)" :label="feature.name"
                    @click="handleEvent($event, feature)" size="small"
                    :severity="feature.severity ? feature.severity : ''"
                    :disabled="feature.disabled || !feature.supported" />
            </template>
        </div>
    </div>

</template>

<script setup>
import { computed } from "vue"

const props = defineProps({
    fileInfo: { type: Object, default: {} },
    mainFileInfo: { type: Object, default: {} },
    note: { type: String, default: '' }
})

const emit = defineEmits(['event'])  

function checkBoxChange( feature) {
    handleEvent(feature.status, feature)
}

function handleEvent($event, feature) {
    emit('event', {
        code: feature.code,
        num: props.fileInfo.num,
        status: $event
    })
}

/**
 * @description: 是否显示
 * @param {*} computed
 * @param {*} style
 * @return {*}
 */
const hasFeature = computed(() => (code) => {
    return props.fileInfo.features.find((feature) => feature.code === code && (props.fileInfo.ismain ? feature.inmain : feature.incoexist))
})

/**
 * @description: 是否显示
 * @param {*} computed
 * @param {*} style
 * @return {*}
 */
const isShow = computed(() => (feature, style, inRight) => {
    // 检查位置条件 (左侧index<100, 右侧index>=100)
    if ((inRight && feature.index < 100) || (!inRight && feature.index >= 100)) {
        return false
    }
    // 检查样式是否匹配
    if (feature.style !== style) {
        return false
    }
    // 基础显示条件
    const show = props.fileInfo.ishead
        ? feature.inhead
        : (props.fileInfo.ismain ? feature.inmain : feature.incoexist)

    if (!show) return false;
    // 特殊功能依赖检查
    if (feature.dependfeature?.length > 0) {
        return feature.dependfeature.every(df =>
            props.fileInfo.features.some(f => f.code === df && f.status)
        )
    }
    let filterList = []
    // 特殊功能依赖检查
    switch (feature.code) {
        case "select_all":
            filterList = ["select"]
            break
        case "open_all":
        case "close_all":
            filterList = ["select", "open"]
            break
    }
    if (!filterList.length) {
        return true
    }
    return props.mainFileInfo?.features
        ?.filter(f => filterList.includes(f.code) && f.status)
        .every(f => f.status)
})

/**
 * @description: 提示
 * @param {*} computed
 * @return {*}
 */
const toolTips = computed(() => (feature) => {
    return {
        value: feature.description,
        showDelay: feature.tdelay || 1000
    }
})

</script>