<h3 align="center"><img src="https://raw.githubusercontent.com/afaa1991/BetterWx-UI/refs/heads/2.0.0/src-tauri/icons/128x128@2x.png" width="250px"></h3>

<p align="center">
  <img src="https://img.shields.io/badge/Platform-Windows-green">
  <img src="https://img.shields.io/github/stars/afaa1991/BetterWx-UI">
</p>


# BetterWx-UI

微信多账号免扫码登陆的终极解决方案  

多开&防撤回&多账号共存 UI工具  

微信Windows版 支持3.9-4.0正式版  

问题反馈交流裙：512064696  

根据大佬 Zetaloop 开源 制作的 ui 工具

大佬开源地址 [https://github.com/zetaloop/BetterWX](https://github.com/zetaloop/BetterWX)

UI工具开源地址 [https://github.com/afaa1991/BetterWx-UI](https://github.com/afaa1991/BetterWx-UI)


## 说明

1.  添加了对3.9的支持，带撤回提示。

2.  4.0防撤回无提示


## 常见问题

1. Win7，Win8 能用吗?

   - 不能。    

   - 基于tauri制作，只支持Win10和Win11带 `webview2` 的系统。

   - Win7,Win8不支持，解决 `webview2` 运行时也能用。

2. 怎么批量启动多个账号。

    ```bash
    start "" "D:\AppData\Tencent\Weixin\Weixin.exe"
    start "" "D:\AppData\Tencent\Weixin\Weixin1.exe"
    timeout /t 1 /nobreak >nul
    ```
    - 修改“`D:\AppData\Tencent\Weixin`” 为你的微信安装目录。

    - `Weixin1.exe` 共存的exe文件名。需要启动哪个，添加哪个。

    -  以上内容保存到 `.bat` 文件，运行。


## 更新说明

 - ### 2.0.2

   - 修复windows 27813 启动白屏问题。

   - 添加管理员启动

 - ### 2.0.1

   - 支持 3.9.12.51 版本，支持共存(可能还需要一些测试)

 - ### 2.0.0

   - 支持4.0正式版


## 下载地址
 - [https://wwtt.lanzn.com/b0pmh8e1i?请输入密码：52pj](https://wwtt.lanzn.com/b0pmh8e1i?请输入密码：52pj)  


## 截图
 <h3 align="center"><img src="https://raw.githubusercontent.com/afaa1991/BetterWx-UI/refs/heads/2.0.0/screenshot.png" width="640px"></h3>

