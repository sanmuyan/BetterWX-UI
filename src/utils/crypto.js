import CryptoJS from 'crypto-js'

// 加密函数
function encryptText(text, password) {
    try {
        return CryptoJS.AES.encrypt(text, password).toString()
    } catch (error) {
        throw new Error('加密失败')
    }
}

// 解密函数
function decryptText(encryptedText, password) {
    try {
        const bytes = CryptoJS.AES.decrypt(encryptedText, password)
        return bytes.toString(CryptoJS.enc.Utf8)
    } catch (error) {
        throw new Error('解密失败')
    }
}

export { encryptText, decryptText } 