import { fetch } from "@tauri-apps/plugin-http"
import { UPDATE_BASE_URL} from "@/config/app_config.js"

/**
 * @description: http请求封装
 * @param {string} url
 * @param {object} options
 * @return {*}
 */
async function http(url, options = {}) {
  const { timeout = 5000, headers = {}, text = false } = options
  try {
    const controller = new AbortController()
    const timeoutId = setTimeout(() => controller.abort(), timeout)
    url = url.startsWith("http")? url : UPDATE_BASE_URL + (url.startsWith("/") ? url.slice(1) :url)
    const response = await fetch(url, {
      ...options,
      signal: controller.signal,
      headers: {
        ...headers
      }
    })
    clearTimeout(timeoutId)
    if (!response.ok) {
      console.log("网路请求错误:",url)
      throw new Error(`网路请求错误: ${response.status}`)
    }
    return text ? await response.text() : await response.json()
  } catch (error) {
    if (error.name === "AbortError") {
      throw new Error("网络请求超时")
    }
    throw error
  }
}

export { http }