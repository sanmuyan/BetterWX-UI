/**
 * 比较两个版本号
 * @param {string} version1 2.0.0
 * @param {string} version2 3.0.0
 * @returns {number} 1: version1 > version2, -1: version1 < version2, 0: 相等
 */
function compareVersion(version1, version2) {
    if (!version1 || !version2) throw new Error("版本号不能为空")
    const v1 = version1.split(".").map(Number)
    const v2 = version2.split(".").map(Number)
    const maxLength = Math.max(v1.length, v2.length)
    for (let i = 0; i < maxLength; i++) {
        const num1 = v1[i] || 0
        const num2 = v2[i] || 0
        if (num1 > num2) return 1
        if (num1 < num2) return -1
    }
    return 0
}

/**
 * number转为u8
 * @param {number} num 
 * @returns u8
 */
function num2u8(num) {
    // 将数字转换为字符串
    const str = num.toString()
    // 使用TextEncoder将字符串编码为UTF-8的Uint8Array
    const encoder = new TextEncoder()
    const u8Array = encoder.encode(str)
    // 将Uint8Array转换为十六进制字符串
    return Array.from(u8Array, byte => byte.toString(16).padStart(2, "0")).join("")
}

/**
 * 是否是主程序
 * @param {number} num 
 * @returns u8
 */
function ismain(num) {
    return num.toString().toLowerCase() === "z"
}

/**
 * 根据str1字符串 修复str2字符串中的通配符
 * @param {string} str1 - 要替换的字符串
 * @param {string} str2 - 包含通配符的字符串
 * @return {string} - 替换后的字符串
 */
function fixWildcards(str1, str2) {
    // 将两个字符串转换为数组以便逐个字符处理
    const arr1 = str1.split('')
    const arr2 = str2.split('')

    // 遍历两个数组，将str2中的通配符替换为str1的对应字符
    for (let i = 0; i < arr2.length; i++) {
        if (arr2[i] === '.') {
            arr2[i] = arr1[i]
        }
    }
    // 将处理后的数组重新组合成字符串并返回
    return arr2.join('')
}

/**
 * @description: 变量替换
 * @param {*} json 原始json
 * @param {*} variables 提供替换变量的值
 * @return {*}
 */
function replaceVariables(json, variables, preprocessor) {
    // 递归遍历对象
    function traverse(obj) {
        for (let key in obj) {
            if (typeof obj[key] === "object" && obj[key] !== null) {
                traverse(obj[key]) // 递归处理嵌套对象
            } else if (typeof obj[key] === "string") {

                if (typeof preprocessor === "function") {
                    obj[key] = preprocessor(obj, key, obj[key])
                }
                // 替换${变量名}格式的字符串
                obj[key] = obj[key].replaceAll(/\$\{([^}]+)\}/g, (match, p1) => {
                    return variables[p1] || match // 如果上下文中有对应的变量值则替换，否则保留原样
                })

            }
        }
    }
    traverse(json)
    return json
}

/**
 * 修复code前缀
 * @param {*} text 
 * @returns 
 */
function fixCodePrefix(code) {
    return code.replace(/^[!+-]+/, '');
}

/**
 * 判断code前缀类型
 * @param {*} str 
 * @returns 
 */
function codePrefixType(code) {
    if (!code) return 0;
    const firstChar = code.charAt(0);
    if (firstChar === '!') return 1;
    if (firstChar === '+') return 2;
    if (firstChar === '-') return 3;
    return 0;
}

/**
 * 根据code前缀类型获取status
 * @param {*} code
 * @param {*} status
 */
function getStatusBycCdePrefix(code, status) {
    switch (codePrefixType(code)) {
        case 0: return status;
        case 1: return !status;
        case 2: return true;
        case 3: return false;
    }
}

/**
 * 将文本转为大端序的两位十六进制字符串
 * @param {*} text 
 * @returns 
 */
function textToBigHex(text, pushEnd) {
    // 创建TextEncoder将字符串转为Uint8Array
    const encoder = new TextEncoder();
    const bytes = encoder.encode(text);

    // 将每个字节转为大端序的两位十六进制字符串
    let hexString = '';
    for (let i = 0; i < bytes.length; i++) {
        // 使用padStart确保始终是两位十六进制
        hexString += bytes[i].toString(16).padStart(2, '0');
    }
    if (pushEnd) {
        hexString += '00'
    }
    return hexString.toUpperCase(); // 转为大写字母形式
}

/**
 * 将大端序十六进制字符串转换为文本（支持中文且不区分大小写）
 * @param {string} hexStr 十六进制字符串
 * @returns {string} 解码后的文本
 */
function bigHexToText(hexStr) {
    // 统一转为大写处理（不区分大小写）
    hexStr = hexStr.toUpperCase();
    // 将十六进制字符串转为字节数组
    const bytes = [];
    for (let i = 0; i < hexStr.length; i += 2) {
        const byteStr = hexStr.substring(i, i + 2);
        // 遇到00立即终止
        if (byteStr === '00') break;

        bytes.push(parseInt(byteStr, 16));
    }

    // 使用TextDecoder解码（自动处理中文）
    const decoder = new TextDecoder('utf-8');
    return decoder.decode(new Uint8Array(bytes));
}

/**
 * 延迟执行
 * @param {number} ms - 延迟时间，单位毫秒
 * @returns {Promise} - 在指定时间后resolve的Promise
 */
function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms))
}

/**
 * 判断值是否为空
 * @param {*} value - 要判断的值
 * @returns {boolean} - 如果值为空返回true，否则返回false
 */
function isEmpty(value) {
    if (value === null || value === undefined) {
        return true
    }
    if (Array.isArray(value) && value.length === 0) {
        return true
    }
    if (typeof value === 'object' && Object.keys(value).length === 0) {
        return true
    }
    return false
}

/**
 * @description: 根据 code 从 variables 获取 value
 * @param {*} variables 
 * @param {*} code 
 * @returns 
 */
function getValueByCode(variables, code) {
    return variables?.find(variable => variable.code == code)?.value
}


export { compareVersion, num2u8, ismain, fixWildcards, replaceVariables, sleep, isEmpty, getValueByCode, codePrefixType,fixCodePrefix, getStatusBycCdePrefix, bigHexToText, textToBigHex }