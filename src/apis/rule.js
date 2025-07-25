import { invoke } from "@tauri-apps/api/core"

export async function rule_get_path(code) {
    return await invoke("rule_get_path",{code})
}

export async function rule_search_address(code) {
    return await invoke("rule_search_address",{code})
}

export async function rule_walk_files(code) {
    return await invoke("rule_walk_files",{code})
}

export async function rule_patch(code,num,fcode,status) {
    return await invoke("rule_patch",{code,num,fcode,status})
}

export async function rule_patch_by_replace(code,num,fcode,ovs) {
    console.log(code,num,fcode,ovs);
    return await invoke("rule_patch_by_replace",{code,num,fcode,ovs})
}


export async function rule_make_coexist(code, num) {
    return await invoke("rule_make_coexist",{code,num})
}

export async function rule_del_coexist(code, num) {
    return await invoke("rule_del_coexist",{code,num})
}

export async function rule_read_orignal(code,num,fcode) {
    return await invoke("rule_read_orignal",{code,num,fcode})
}
