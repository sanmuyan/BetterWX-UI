[
    {
        "code": "select",
        "method": "select",
        "description": "选中",
        "inmain": true,
        "incoexist": true,
        "index": 30,
        "tdelay": 100,
        "style": "checkbox",
        "supported": true
    },
    {
        "code": "select_all",
        "method": "select_all",
        "description": "全选",
        "inhead": true,
        "index": 31,
        "tdelay": 100,
        "style": "checkbox",
        "supported": true
    },
    {
        "code": "open_all",
        "name": "一键启动",
        "method": "",
        "description": "运行所有选中的软件",
        "inhead": true,
        "index": 32,
        "tdelay": 100,
        "style": "button",
        "supported": true
    },
    {
        "code": "close_all",
        "name": "一键关闭",
        "method": "",
        "description": "退出所有选中的软件",
        "severity": "danger",
        "inhead": true,
        "index": 33,
        "tdelay": 100,
        "style": "button",
        "supported": true
    },
    {
        "code": "revoke",
        "name": "防撤",
        "method": "patch",
        "description": "调整防撤回状态",
        "inmain": true,
        "incoexist": true,
        "index": 40,
        "tdelay": 100,
        "style": "switch",
        "supported": true,
        "dependencies": [
            "revoke"
        ]
    },
    {
        "code": "mutex",
        "name": "多开",
        "method": "patch",
        "description": "调整多开状态",
        "inmain": true,
        "index": 50,
        "tdelay": 100,
        "style": "switch",
        "supported": true,
        "dependencies": [
            "mutex"
        ]
    },
    {
        "code": "clear",
        "name": "清理缓存",
        "method": "",
        "description": "清除软件缓存",
        "inhead": true,
        "index": 100,
        "tdelay": 100,
        "style": "button",
        "supported": true
    },
    {
        "code": "refresh",
        "name": "刷新状态",
        "description": "重新读取所有文件状态",
        "inhead": true,
        "index": 110,
        "tdelay": 100,
        "style": "button",
        "supported": true
    },
    {
        "code": "floder",
        "name": "打开目录",
        "description": "打开文件所在目录",
        "inhead": true,
        "index": 120,
        "tdelay": 100,
        "style": "button",
        "supported": true,
        "target": "${install_location}"
    },
    {
        "code": "note",
        "name": "备注",
        "description": "添加备注",
        "inmain": true,
        "incoexist": true,
        "index": 130,
        "tdelay": 100,
        "style": "button",
        "supported": true
    },
    {
        "code": "lnk",
        "name": "快捷",
        "description": "添加快捷方式到桌面",
        "inmain": true,
        "incoexist": true,
        "index": 131,
        "tdelay": 100,
        "style": "button",
        "supported": true,
        "target": "${path_exe}",
        "saveas": "${new_path_exe}"
    },
    {
        "code": "open",
        "name": "运行",
        "method": "",
        "description": "运行当前程序",
        "inmain": true,
        "incoexist": true,
        "index": 140,
        "tdelay": 100,
        "style": "button",
        "supported": true,
        "target": "${path_exe}",
        "saveas": "${new_path_exe}"
    },
    {
        "code": "close",
        "name": "关闭",
        "method": "",
        "description": "关闭当前程序",
        "inmain": true,
        "incoexist": true,
        "index": 141,
        "tdelay": 100,
        "style": "button",
        "supported": true,
        "severity": "danger",
        "target": "${path_exe}",
        "saveas": "${new_path_exe}"
    },
    {
        "code": "coexist",
        "name": "共存",
        "description": "制作共存文件",
        "inmain": true,
        "index": 150,
        "tdelay": 100,
        "style": "button",
        "supported": true,
        "dependencies": [
            "mutex_name",
            "config",
            "host",
            "dllname",
            "window_name",
            "-mutex"
        ]
    },
    {
        "code": "del",
        "name": "删除",
        "description": "删除共存文件",
        "incoexist": true,
        "index": 151,
        "tdelay": 100,
        "style": "button",
        "disabled": false,
        "supported": true,
        "severity": "danger"
    }
]