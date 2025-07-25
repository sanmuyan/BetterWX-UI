use super::errors::Result;
use log::debug;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ToolsError {
    #[error("替换通配符出错，原始字符为空")]
    ReplaceWildcardsorignalEmptyError,

    #[error("替换省略符出错，原始字符为空")]
    ReplaceEllipsisorignalEmptyError,

    #[error("替换省略符出错，字符长度不一致，请检查配置文件")]
    ReplaceEllipsisDifferentLengthError,

    #[error("替换通配符出错，字符长度不一致，请检查配置文件")]
    ReplaceWildcardsDifferentLengthError,
}

pub fn replace_ellipsis<S: AsRef<str>>(ellipsis_str: S, orignal: S) -> Result<String> {
    let ellipsis_str = ellipsis_str.as_ref();
    let orignal = orignal.as_ref();
    // 确保两个字符串长度相同
    if orignal.is_empty() {
        return Err(ToolsError::ReplaceEllipsisorignalEmptyError.into());
    }
    if let Some(index) = ellipsis_str.find("...") {
        let l_text = &ellipsis_str[..index];
        let r_text = &ellipsis_str[index + 3..];
        let m_text = &orignal[l_text.len()..&orignal.len() - &r_text.len()];
        return Ok(format!("{}{}{}", l_text, m_text, r_text));
    }
    if ellipsis_str.len() != orignal.len() {
        debug!("省略符：{}，原始字符串：{}", ellipsis_str, orignal);
        return Err(ToolsError::ReplaceEllipsisDifferentLengthError.into());
    }
    Ok(ellipsis_str.to_string())
}

pub fn replace_wildcards<S: AsRef<str>>(wildcard_str: S, orignal: S) -> Result<String> {
    let wildcard_str = wildcard_str.as_ref();
    let orignal = orignal.as_ref();
    // 确保两个字符串长度相同

    if wildcard_str.is_empty() {
        return Ok(wildcard_str.to_string());
    }
    // 确保两个字符串长度相同
    if orignal.is_empty() {
        return Err(ToolsError::ReplaceWildcardsorignalEmptyError.into());
    }
    // 确保两个字符串长度相同
    if wildcard_str.len() != orignal.len() {
        debug!("通配符：{}，原始字符串：{}", wildcard_str, orignal);
        return Err(ToolsError::ReplaceWildcardsDifferentLengthError.into());
    }
    // 将str2转换为字符向量以便修改
    let mut wildcard_chars: Vec<char> = wildcard_str.chars().collect();
    // 遍历str2，替换通配符
    for (i, c) in wildcard_str.chars().enumerate() {
        if c == '?' {
            // 使用str1对应位置的字符替换
            wildcard_chars[i] = orignal.chars().nth(i).unwrap();
        }
    }
    // 将字符向量转换回字符串
    Ok(wildcard_chars.into_iter().collect::<String>())
}

#[macro_export]
macro_rules! destructure_assign {
    ($self:ident, $other:ident, $($field:ident),*) => {
        $(
            if $self.$field.is_empty() && !$other.$field.is_empty() {
                $self.$field = $other.$field.clone();
            }
        )*
    };
}
