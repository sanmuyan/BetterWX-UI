<h3 align="center"><img src="https://raw.githubusercontent.com/afaa1991/BetterWX-UI/refs/heads/2.0.0/src-tauri/icons/128x128@2x.png" width="250px"></h3>

<p align="center">
  <img src="https://img.shields.io/badge/Platform-Windows-green">
  <img src="https://img.shields.io/github/stars/afaa1991/BetterWX-UI">
  <img src="https://img.shields.io/badge/WeChat-3.9~4.0-blue">
</p>

# ✨ BetterWX-UI ✨

**使用软件可能导致功能限制、封号等行为。请自行评估风险，不承担任何责任。**

多开、防撤回提示、多账号免扫码登录的终极解决方案

## 🔥 主要功能

- ✅ 多开支持，建议使用共存方案
- ✅ 消息防撤回
- ✅ 多账号免扫码登录
- ✳️ 撤回2：支持消息撤回编辑
- ✳️ 撤回2：支持消息撤回提示
- ✳️ 撤回2：支持自定义撤回提示消息（在共存上使用）
- ✳️ **撤回3：感谢EEEEhex大佬的撤回方法，感谢zetaloop大佬的补丁版方案**

## 📞 反馈交流

- 问题反馈交流裙：512064696  QQ频道号：pd27172200

## 📌 支持版本

- Windows版 3.9.12.51（64位） 和 4.0.3.11+ 正式版

## ⚙️ 功能说明

- 3.9版本：支持撤回提示
- 4.0版本：支持撤回编辑，支持撤回提示，支持自定义撤回提示消息。

## 📜 鸣谢

- 项目开源地址
    - BetterWX-UI [https://github.com/afaa1991/BetterWX-UI](https://github.com/afaa1991/BetterWX-UI)

- **zetaloop：原始项目**
    - BetterWX [https://github.com/zetaloop/BetterWX](https://github.com/zetaloop/BetterWX)

- EEEEhex
    - RevokeHook [https://github.com/EEEEhex/RevokeHook](https://github.com/EEEEhex/RevokeHook)

- huiyadanli
    - RevokeMsgPatcher [https://github.com/huiyadanli/RevokeMsgPatcher](https://github.com/huiyadanli/RevokeMsgPatcher)

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

- 方法1. [BetterWX-Starter](https://github.com/afaa1991/BetterWX-Starter)

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

<h3 align="center"><img src="https://raw.githubusercontent.com/afaa1991/BetterWX-UI/refs/heads/2.0.0/screenshot.png" width="640px"></h3>
