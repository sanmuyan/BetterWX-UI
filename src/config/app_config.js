// TEST_MODE
const TEST_MODE = import.meta.env.DEV
// 网络请求的地址
const BASE_URL = "https://gitee.com/afaa1991/BetterWX-UI/raw/master"
// 网络请求的地址 PRO
const UPDATE_BASE_URL = BASE_URL + (TEST_MODE?"/.gitignore/":"/.vscode/")
// 更新文件的地址
const UPDATE_URL = "update.json"
// 是否保存规则
const USE_SAVE_BASE_RULE = true
// 是否保存配置
const USE_SAVE_CONFIG = true
// 是否保存readme
const USE_SAVE_README = true
// 消息显示的时间
const MESSAGE_LIFE = 3000
// SPLASH消息显示延迟
const SPLASH_DELAY = 0
// SPLASH加载完成后延迟
const SPLASH_SUCCESS_DELAY = 300

export { UPDATE_BASE_URL, UPDATE_URL, USE_SAVE_BASE_RULE, USE_SAVE_CONFIG, MESSAGE_LIFE, USE_SAVE_README, SPLASH_DELAY, SPLASH_SUCCESS_DELAY, TEST_MODE }
