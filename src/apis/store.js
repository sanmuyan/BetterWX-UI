import { invoke } from "@tauri-apps/api/core"

export  async function store_read(name){
   return await invoke("store_read",{name})
}

export  async function  store_save(name,data){
     return await invoke("store_save",{name,data})
}
