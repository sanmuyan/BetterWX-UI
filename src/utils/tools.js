export function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms))
}

export function text2hex(text) {
    const encoder = new TextEncoder();
    const bytes = encoder.encode(text);
    let hexString = "";
    for (let i = 0; i < bytes.length; i++) {
        hexString += bytes[i].toString(16).padStart(2, "0");
    }
    if (!hexString.endsWith("00")) {
        hexString += "00"
    }
    return hexString.toUpperCase();
}

export function hex2text(hexStr) {
    hexStr = hexStr.toUpperCase();
    const bytes = [];
    for (let i = 0; i < hexStr.length; i += 2) {
        const byteStr = hexStr.substring(i, i + 2);
        if (byteStr === "00") break;

        bytes.push(parseInt(byteStr, 16));
    }
    const decoder = new TextDecoder("utf-8");
    return decoder.decode(new Uint8Array(bytes));
}