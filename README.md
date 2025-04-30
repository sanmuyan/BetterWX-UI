<h3 align="center"><img src="https://raw.githubusercontent.com/afaa1991/BetterWx-UI/refs/heads/2.0.0/src-tauri/icons/128x128@2x.png" width="250px"></h3>

<p align="center">
  <img src="https://img.shields.io/badge/Platform-Windows-green">
  <img src="https://img.shields.io/github/stars/afaa1991/BetterWx-UI">
  <img src="https://img.shields.io/badge/WeChat-3.9~4.0-blue">
</p>

# ✨ BetterWx-UI ✨

多开、防撤回提示、多账号免扫码登录的终极解决方案

## 🔥 主要功能

- ✅ 多开支持
- ✅ 消息防撤回
- ✅ 多账号免扫码登录
- ✳️ **4.0 支持消息撤回编辑**
- ✳️ **4.0 支持消息撤回提示**
- ✳️ **4.0 支持自定义撤回提示消息（在共存上使用）**

## ✒️ 更新说明

  - 🔆 修复自动更新获取注册表错误的问题。
  - 🔆 支持绿色版，放在绿色版目录内运行。

## 📌 支持版本

- Windows版 3.9.12.51（64位） 和 4.0.3.11+ 正式版

## ⚙️ 功能说明

- 3.9版本：支持撤回提示
- 4.0版本：支持撤回编辑，支持撤回提示，支持自定义撤回提示消息。

## 📜 项目来源

基于大佬 Zetaloop 的开源项目制作的UI工具

- 原项目地址: [https://github.com/zetaloop/BetterWX](https://github.com/zetaloop/BetterWX)
- UI工具地址: [https://github.com/afaa1991/BetterWx-UI](https://github.com/afaa1991/BetterWx-UI)

## ❓ 常见问题解决方案

- 使用官方客户端覆盖安装

- 清除软件缓存，重启软件

- 程序闪退，请安装webview2运行环境

- [webview2下载地址](https://developer.microsoft.com/zh-cn/microsoft-edge/webview2/?form=MA13LH#download)

## 🖥️ 系统兼容性

|    系统版本    |    支持情况    |        备注       |
|---------------|---------------|-------------------|
| Windows 10/11 |    ✅ 支持    | 需要webview2运行环境   |
| Windows 7/8   |    ✅ 支持  | 需要webview2运行环境 + VxKex插件 |

## 🔄 批量启动多账号

- 方法1.[BetterWx-Starter](https://github.com/afaa1991/BetterWx-Starter)

- 基于 Rust 的实现的批量启动小玩具 支持自动排序、自动登录

- 方法2.  以下内容保存到 `.bat` 文件，运行。

- 修改“`D:\AppData\Tencent\Weixin`” 为你的微信安装目录。

- `Weixin1.exe` 共存的exe文件名。需要启动哪个，添加哪个。

```bash
start "" "D:\AppData\Tencent\Weixin\Weixin.exe"
start "" "D:\AppData\Tencent\Weixin\Weixin1.exe"
timeout /t 1 /nobreak >nul
```

## 💾 下载地址

 - [https://wwtt.lanzn.com/b0pmh8e1i?请输入密码：52pj](https://wwtt.lanzn.com/b0pmh8e1i?请输入密码：52pj)

## 📺 截图

<h3 align="center"><img src="https://raw.githubusercontent.com/afaa1991/BetterWx-UI/refs/heads/2.0.0/screenshot.png" width="640px"></h3>
