<template>
    <div class="flex row justify-between items-center">
        <div class="flex row justify-center">
            <b class="m-r">{{ fileInfo.name }}</b>
            <template v-for="(feature, index) in fileInfo.features" :key="index">
                <div v-if="isShow(feature, 'switch')" class="flex items-center gap  m-x">
                    <label :for="feature.code">{{ feature.name }}</label>
                    <ToggleSwitch v-tooltip.top="toolTips(feature)" :inputId="feature.code"
                        v-model="feature.status" :disabled="feature.disabled || !feature.supported"
                        :invalid="!feature.supported"  @value-change="handleEvent($event,feature)" />
                </div>
            </template>
        </div>
        <div class="flex items-center gap">
            <template v-for="(feature, index) in fileInfo.features" :key="index">
                <Button v-if="isShow(feature, 'button')" v-tooltip.top="toolTips(feature)" :label="feature.name"
                    @click="handleEvent($event,feature)" size="small" :severity="feature.severity ? feature.severity : ''"
                    :disabled="feature.disabled || !feature.supported" />
            </template>
        </div>
    </div>
</template>

<script setup>
import { computed } from "vue"

const props = defineProps({
    fileInfo: { type: Object, default: {} },
})

const emit = defineEmits(['event'])
function handleEvent($event,feature) {
    emit('event', {
        code:feature.code,
        num:props.fileInfo.num,
        status:$event
    }) 
}

/**
 * @description: 是否显示
 * @param {*} computed
 * @param {*} style
 * @return {*}
 */
const isShow = computed(() => (feature, style) => {
    //是否禁用
    if (feature.style !== style) {
        return false
    }
    return props.fileInfo.ismain ? feature.inmain : feature.incoexist
})

/**
 * @description: 提示
 * @param {*} computed
 * @return {*}
 */ 
const toolTips = computed(() => (feature) => {
    return {
        value: feature.description,
        showDelay: 1000
    }
})

</script>