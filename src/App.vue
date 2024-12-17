<template>

  <main class="container">
    <Fieldset legend="路径设置">
      <div class="flex m-b">
        <FloatLabel variant="on" class="flex-1 m-r">
          <InputText id="on_label" fluid v-model="wx_path" size="small" @input="wx_path_input" />
          <label for="on_label">请选择Weixin.dll文件</label>
        </FloatLabel>
        <Button label="选择" class="m-r" size="small" @click="select_file" />
        <Button label="52pojie" size="small" @click="open_52" />
      </div>
    </Fieldset>

    <Fieldset>
      <template #legend>
        <div class="flex row">
          <b class="m-r">功能区</b>
          <Tag v-if="wx_ver" :value="wx_ver" severity="success"></Tag>
        </div>
      </template>
      <template v-for="(item, index) in list" :key="index">
        <coexistList :isMain="item.id == -1" :data="item" :index="index" :num="item.id" :unlock="item.unlock"
          :revoke="item.revoke" @switch_change="switch_change" @refresh="refresh" @loc="open_folder" @add="add"
          @del="del" @open_app="open_app" class="border-b p-y">
        </coexistList>
      </template>
    </Fieldset>

  </main>
  <Toast />
  <div v-if="loading" class="float" style="height: 6px;">
    <ProgressBar mode="indeterminate" style="height: 6px;"></ProgressBar>
  </div>
</template>
<script setup>
import coexistList from "./components/coexist-list.vue";
import { onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useToast } from 'primevue/usetoast';
import { message } from '@tauri-apps/plugin-dialog';
import { exit } from '@tauri-apps/plugin-process';

const toast = useToast();
const wx_loc = ref("")
const wx_ver = ref("")
const wx_path = ref("")
const loading = ref(false)
const list = ref([]);
const name = ref("")

onMounted(async () => {
  //获取 lock.ini 文件夹路径
  let r = await check_admin()
  if (!r) {
    await message('请以管理员模式运行', { title: '消息', kind: 'error' });
    await exit(1);
    return
  }
  loading.value = true
  show_contrast("正在初始化,请稍等", 1000)
  let temp = await invoke("wx_install_loc");
  if (!temp[0] || !temp[1]) {
    show_contrast("获取安装目录失败，请手动选择Weixin.dll位置")
    loading.value = false
    return
  }
  wx_loc.value = temp[0]
  wx_ver.value = temp[1]
  wx_path.value = temp.join("\\") + "\\Wexin.dll"
  await wx_init()
  loading.value = false
})


/**
 * 列出所有共存文件
 */
async function wx_list_all(show_loading = false, show_msg = true) {
  await do_command("wx_list_all", {}, (r) => {
    list.value = r
  }, show_loading, show_msg, "获取共存文件列表ok")
}

/**
 * 初始化
 */
async function wx_init() {
  list.value = []
  let r = await do_command("wx_init", { exeLoc: wx_loc.value, version: wx_ver.value }, async () => {
    await wx_list_all()
  }, false, true, "初始化完成")
}

/**
 * 手动选择文件
 */
async function select_file() {
  if (loading.value == true) {
    show_contrast("正在初始化,请稍等")
    return
  }

  const file = await open({
    multiple: false,
    directory: false,
  });
  deal_wx_path(file, true)
}


/**
 * 手动输入文件位置
 * @param e 
 */
function wx_path_input(e) {
  const v = e.currentTarget.value.trim()
  if (v == "") return
  deal_wx_path(v, fasle)
}

/**
 * 处理微信路径
 * @param file 
 * @param show_toast 
 */
async function deal_wx_path(file, show_toast) {
  if (!file) return
  if (!file.endsWith("Weixin.dll")) {
    if (show_toast) show_contrast("错误的路径，请选择Weixin.dll文件")
    return
  }
  let paths = file.split("\\")
  let loc = paths.slice(0, -2).join("\\")
  if (loc == wx_loc.value) return
  loading.value = true
  show_contrast("正在初始化")
  wx_loc.value = loc
  wx_ver.value = paths.slice(-2, -1)[0]
  wx_path.value = file
  await wx_init()
  loading.value = false
}

/**
 * 补丁状态切换
 * @param arg 
 */
async function switch_change(arg) {

  let item = list.value[arg.index]
  if (arg.revoke == item.revoke && arg.unlock == item.unlock) return
  let back = JSON.parse(JSON.stringify(item))
  item.unlock = arg.unlock
  item.revoke = arg.revoke
  let r = await do_command("wx_do_patch", { isUnlock: arg.unlock, isRevoke: arg.revoke, coexistNumber: item.id }, (r) => {
    if (!r.length) {
      show_contrast("好像失败了")
      return
    }
    list.value[arg.index] = r[0]
  }, true, true)
  if (!r) {
    list.value[arg.index] = back
  }
}

/**
 * 制作一个共存
 * @param arg 
 */
async function add(arg) {
  //计算需要添加的num
  let num = 10
  let ids = []
  for (let i = 0; i < list.value.length; i++) {
    ids.push(list.value[i].id)
  }
  for (let i = 0; i < 10; i++) {
    if (ids.includes(i)) { continue } else { num = i; break }
  }
  if (num == 10) {
    show_contrast("添加失败,已经有太多了")
    return
  }
  await do_command("wx_do_patch", { isUnlock: true, isRevoke: list.value[0].revoke, coexistNumber: num }, (r) => {
    if (!r.length) {
      show_contrast("好像失败了")
      return
    }
    list.value.push(r[0])
    list.value.sort((a, b) => a.id - b.id);
  }, true, true)
}

/**
 * 删除一个共存
 * @param arg 
 */
async function del(arg) {
  await do_command("wx_del_corexist", { list: JSON.stringify(list.value[arg.index]) }, () => {
    list.value.splice(arg.index, 1);
  }, true, true)
}

async function open_folder() {
  await do_command("wx_open_folder", { file: wx_loc.value }, true, true)
}


async function open_52() {
  await do_command("wx_open_url", { url: "https://www.52pojie.cn/thread-1991091-1-1.html" }, true, true)
}
async function refresh() {
  await wx_list_all(true, true)
}


/**
 * 
 * @param file 运行
 */
async function check_admin() {
  return await do_command("win_is_admin", {}, () => {
  }, false, false)
}

/**
 * 
 * @param file 运行
 */
async function open_app(file) {
  await do_command("wx_open_app", { file: file }, true, true)
}

async function do_command(command, arg, cb, show_loading, show_msg = false, msg = "ok") {
  if (show_loading) loading.value = true
  cb = typeof cb === "function" ? cb : () => { }
  try {
    let r = await invoke(command, arg)
    await cb(r)
    if (show_msg) {
      msg = msg ? msg : "ok"
      show_contrast(msg)
    }
    return r
  } catch (error) {
    show_contrast(error)
    return false
  } finally {
    if (show_loading) loading.value = false
  }
}

/**
 * toast
 */
function show_contrast(detail, life = 1000) {
  toast.add({ severity: 'contrast', summary: '消息', detail: detail, life: detail, life });
};
</script>

<style>
.flex {
  display: flex;
}

.flex-1 {
  flex: 1;
}

.w-100 {
  width: 100%;
}

.row {
  flex-direction: row;
}

.col {
  flex-direction: column;
}

.justify-center {
  justify-content: center;
}

.justify-start {
  justify-content: start;
}

.justify-between {
  justify-content: space-between;
}

.items-center {
  align-items: center;
}

.gap {
  gap: 4px;
}

.m-x {
  margin-left: 8px;
  margin-right: 8px;
}

.m-y {
  margin-top: 8px;
  margin-bottom: 8px;
}

.m-b {
  margin-bottom: 8px;
}

.m-r {
  margin-right: 8px;
}

.p-t {
  padding-top: 8px;
}

.p-b {
  padding-bottom: 8px;
}

.p-y {
  padding-top: 8px;
  padding-bottom: 8px;
}

.border-b {
  border-bottom: solid 1px;
  border-color: color-mix(in srgb, var(--p-surface-200) calc(100%* var(--tw-border-opacity, 1)), transparent);
}

.float {

  position: fixed;
  width: 100%;
  bottom: 0px;
  margin: 0 !important;
  padding: 0 !important;
  border: 0 !important;
}

.container {
  padding: 8px;
}

* {
  margin: 0;
  padding: 0;
}
</style>