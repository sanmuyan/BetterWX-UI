<template>
    <div v-if="showLoading"  class="float-bottom" style="height: 6px;">
        <ProgressBar mode="indeterminate" style="height: 6px;"></ProgressBar>
    </div>
</template>

<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
    show: {
        type: Boolean,
        default: false
    }
})

const showLoading = ref(false)

/**
 * @description: 监听 show 变化，延迟取消显示
 * @return {*}
 */
watch(() => props.show, (newValue) => {
    newValue ? showLoading.value = true : setTimeout(() => {
        showLoading.value = false
    }, 150)
})

</script>