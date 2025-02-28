
<h3 align="center"><img src="https://raw.githubusercontent.com/afaa1991/BetterWx-UI/refs/heads/1.1.2/src-tauri/icons/128x128@2x.png" width="250px"></h3>

<p align="center">
  <img src="https://img.shields.io/badge/Platform-Windows-green">
  <img src="https://img.shields.io/github/stars/afaa1991/BetterWx-UI">
</p>


# BetterWx-UI

微信Windows版 支持4.0.2 双开&防撤回&多账号共存 UI工具
支持平台：Windows x64

版本支持：4.0+

根据大佬 Zetaloop 开源 制作的 ui 工具

大佬开源地址 [https://github.com/zetaloop/BetterWX](https://github.com/zetaloop/BetterWX)

UI工具开源地址 [https://github.com/afaa1991/BetterWx-UI](https://github.com/afaa1991/BetterWx-UI)


## 说明

1.  不支持3.9的微信。

2.  以管理员模式启动，在修改之前，原文件会备份到 `Weixin.dll.bak` 和 `Weixin.exe.bak`。

3.  防撤回无提示，保留了PC本机可以撤回编辑的功能。【所有 `revokemsg` 消息变为未知消息，不响应撤回操作】

4.  多开。【移除 `lock.ini` 锁文件检测】

5. 共存版制作器。【生成一个 `Weixinζ.exe` 和 `Weixin.dlζ`，其设置数据保存在 `global_confζg`、自动登录端口数据保存在 `host-redirect.xmζ`】

6. 不是共存启动器，不要当启动器用，当然你要这么用也没问题。 更好的方式是分别为目录下的 `Weixinζ.exe` 创建快捷方式使用。

## 常见问题

1. 找不到 `Weixin.dll`。

   - 请确认是`4.0+`的版本，不支持`3.9`。

   - `WeiXin.dll` 在`4.0`的安装目录对应的版号文件夹里面。

   - 通过官方安装的微信一般能自动获取路径。

2. 微信 `3.9.x` 能用吗？
   - 猜你想找 [huiyadanli/RevokeMsgPatcher](https://github.com/huiyadanli/RevokeMsgPatcher/)。

3. 共存、多开、防撤回这几个特性我可以挑选几个 or 全都要吗？
   - 除了多开是作为共存的前置条件，别的可以自己选。

4. 企业微信能用吗？
   - 不支持。

5. Win7，Win8 能用吗?

   - 不能。    

   - 基于tauti制作，只支持Win10和Win11带 `webview2` 的系统。

   - Win7,Win8不支持，解决 `webview2` 运行时也能用。

5. 防撤回有没有提示。
   - 没有，我也不会，等大佬们解决。

6. 怎么批量启动多个账号。

    ```bash
    start "" "D:\AppData\Tencent\Weixin\Weixin.exe"
    start "" "D:\AppData\Tencent\Weixin\Weixin1.exe"
    timeout /t 1 /nobreak >nul
    ```
    - 修改“`D:\AppData\Tencent\Weixin`” 为你的微信安装目录。

    - `Weixin1.exe` 共存的exe文件名。需要启动哪个，添加哪个。

    -  以上内容保存到 `.bat` 文件，运行。


## 更新说明

 - ### 1.1.2

   - 修复就收不到语音消息的问题
   - 注意：请使用1.1.1版本关闭所有功能后或使用官方安装包覆盖安装后再操作。

 - ### 1.1.1

   - 调整了部分页面UI


 - ### 1.1.0

   - 支持4.0.2


- ### 1.0.2


    - 修改共存锁定文件为 `lock.inζ`,


    - 共存独立锁定,不再需要双开了。 


- ### 1.0.1


    - 默认 管理员模式启动。 


## 下载地址
 - [https://wwtt.lanzn.com/b0pmh8e1i 密码:52pj](https://wwtt.lanzn.com/b0pmh8e1i)

