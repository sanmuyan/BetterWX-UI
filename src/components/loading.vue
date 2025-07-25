<template>
    <div class="absolute bottom-0 left-0 right-0" style="height: 6px;">
        <ProgressBar v-if="showLoading"  mode="indeterminate" style="height: 6px;"></ProgressBar>
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

watch(() => props.show, (newValue) => {
    newValue ? showLoading.value = true : setTimeout(() => {
        showLoading.value = false
    }, 100)
})

</script>