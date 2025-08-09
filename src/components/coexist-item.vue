<template>
    <div class="h-16 relative">
        <div class="h-16 flex flex-row items-center justify-between">
            <div class="flex flex-row items-center">
                <b class="mr-4">{{ title }}</b>
                <template v-for="(feature, index) in features" :key="index">
                    <div v-if="isShow(feature, 'checkbox')" class="mr-2 flex flex-col items-center justify-center ">
                        <Checkbox v-model="feature.selected" binary v-tooltip.top="toolTips(feature)"
                            @change="check_change(feature)" />
                    </div>
                    <div v-if="isShow(feature, 'switch')"
                        class="mr-2 flex flex-col items-center justify-center no-focus"
                        v-tooltip.top="toolTips(feature)" @click="switch_change(feature)">
                        <Button :label="feature.name" :variant="!feature.status ? 'outlined' : ''" size="small"
                            :severity="feature.status ? 'success' : feature.severity"
                            :disabled="is_disabled(feature)" />
                    </div>
                    <div v-if="isShow(feature, 'button')" class="mr-2 flex flex-col items-center justify-center "
                        v-tooltip.top="toolTips(feature)" @click="handleEvent(true, feature)">
                        <Button v-if="feature.icon" :icon="`${feature.icon}`" size="small" :severity="feature.severity"
                            :disabled="is_disabled(feature)" />
                        <Button v-else :label="feature.name" size="small" :severity="feature.severity"
                            :disabled="is_disabled(feature)" />
                    </div>
                </template>
            </div>
            <div class="flex flex-row items-center mr-1">
                <template v-for="(feature, index) in features" :key="index">
                    <div v-if="isShow(feature, 'button', true)" class="mr-2 flex flex-col items-center justify-center "
                        v-tooltip.top="toolTips(feature)" @click="handleEvent(true, feature)">
                        <Button v-if="feature.icon" :icon="`${feature.icon}`" size="small" :severity="feature.severity"
                            :disabled="is_disabled(feature)" />
                        <Button v-else :label="feature.name" size="small" :severity="feature.severity"
                            :disabled="is_disabled(feature)" />
                    </div>
                </template>
            </div>
        </div>
        <div v-if="note" class="absolute -bottom-0 left-0 right-0 text-xs truncate text-gray-500">
            {{ note }}
        </div>
    </div>
</template>
<script setup>
import { computed } from "vue";

const props = defineProps({
    title: { type: String, default: "" },
    num: { type: Number, default: 0 },
    features: { type: Array, default: [] },
    note: { type: String, default: "" }
})

const emit = defineEmits(["event"])

function handleEvent($event, feature) {
    console.log($event);
    console.log(feature);
    emit("event", {
        feature: feature,
        status: $event,
        num: props.num
    })
}

function check_change(feature) {
    handleEvent(feature.selected, feature)
}


function switch_change(feature) {
    if (is_disabled.value(feature)) {
        return
    }
    handleEvent(!feature.status, feature)
}

const is_disabled = computed(() => (feature) => {
    return feature.disabled || !feature.supported
})

const isShow = computed(() => (feature, bntype, inright) => {
    if (feature.disabled) {
        return false
    }
    if ((inright && feature.index < 100) || (!inright && feature.index >= 100)) {
        return false
    }
    if (feature.bntype !== bntype) {
        return false
    }
    if (feature.dependfeatures?.length) {
        let show = feature.dependfeatures.every(fcode => props.features.find(f => f.code === fcode)?.status)
        if (!show) {
            return false
        }
    }
    return true
})

const toolTips = computed(() => (feature) => {
    let tip = is_disabled.value(feature) ? "已失效：\n" + feature.description : feature.description;
    return {
        value: tip,
        showDelay: feature.tdelay || 500
    }
})
</script>