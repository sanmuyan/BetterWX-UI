use crate::errors::ConfigError;
use crate::errors::Result;
use crate::patches::Patches;
use macros::ImpConfigVecIsEmptyTrait;
use macros::ImpConfigVecWrapperTrait;
use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;
use serde::Serializer;
use serde::ser::SerializeMap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fmt::Display;
use std::result::Result as RResult;
use utils::patch::jump_offset::calculate_jump_offset_bytes;

pub const LOCATION_CODE: &str = "install_location";
pub const VERSIUON_CODE: &str = "install_version";
pub const NUM_CODE: &str = "num";
pub const NUM_HEX_CODE: &str = "num_hex";
pub const ISMAIN_CODE: &str = "ismain";

//const BACK_SUFFIX: &str = "_back}";
const SAVE_SUFFIX: &str = "_save}";
const BASE_SUFFIX: &str = "_base}";

#[derive(Clone, Default, ImpConfigVecIsEmptyTrait, ImpConfigVecWrapperTrait)]
pub struct Variables(pub Vec<Variable>);

impl Variables {
    pub fn init(&mut self) -> Result<()> {
        let mut values = Vec::new();
        for variable in self.0.iter() {
            let v = if let VariableValue::String(s) = variable.get_value() {
                VariableValue::String(self.substitute(s.to_string()))
            } else {
                variable.get_value().clone()
            };
            values.push(v);
        }
        for (variable, new_value) in self.0.iter_mut().zip(values) {
            variable.set_value(new_value);
        }
        Ok(())
    }

    fn create_varibales_by_regex<S: AsRef<str>>(text: S, regex: &str, replace: &str) -> Self {
        let re = regex::Regex::new(regex).unwrap();
        let js_variables = re
            .captures_iter(text.as_ref())
            .filter_map(|cap| cap.get(1))
            .map(|m| {
                let code = m.as_str();
                let value = replace.replace("#", code);
                Variable::new(code, value)
            })
            .collect::<Vec<Variable>>();
        Self(js_variables)
    }

    pub fn create_js_varibales<S: AsRef<str>>(text: S) -> Self {
        Self::create_varibales_by_regex(text, r"\$\{([^}]+?)\}", r"${#}")
    }

    pub fn create_add_varibales<S: AsRef<str>>(text: S) -> Self {
        Self::create_varibales_by_regex(text, r"\[([^}]+?)\]", r"$[#]")
    }

    pub fn get_install_loction(&self) -> Result<&str> {
        if let Some(v) = self.find_variable(LOCATION_CODE)
            && let Some(s) = v.as_str()
        {
            return Ok(s);
        }
        Err(ConfigError::GetVariabledValueError(LOCATION_CODE.to_string()).into())
    }

    pub fn get_install_version(&self) -> Result<&str> {
        if let Some(v) = self.find_variable(VERSIUON_CODE)
            && let Some(s) = v.as_str()
        {
            return Ok(s);
        }
        Err(ConfigError::GetVariabledValueError(VERSIUON_CODE.to_string()).into())
    }

    pub fn get_ismain(&self) -> Result<bool> {
        if let Some(v) = self.find_variable(ISMAIN_CODE)
            && let Some(b) = v.as_bool()
        {
            return Ok(b);
        }
        Err(ConfigError::GetVariabledValueError(ISMAIN_CODE.to_string()).into())
    }

    pub fn get_num(&self) -> Result<usize> {
        if let Some(v) = self.find_variable(NUM_CODE)
            && let Some(n) = v.as_usize()
        {
            return Ok(n);
        }
        Err(ConfigError::GetVariabledValueError(NUM_CODE.to_string()).into())
    }

    pub fn get_num_hex(&self) -> Result<&str> {
        if let Some(v) = self.find_variable(NUM_HEX_CODE)
            && let Some(s) = v.as_str()
        {
            return Ok(s);
        }
        Err(ConfigError::GetVariabledValueError(NUM_HEX_CODE.to_string()).into())
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn extend(&mut self, variables: Variables) {
        self.0.extend(variables.0);
    }

    pub fn push(&mut self, variable: Variable) {
        self.0.push(variable);
    }

    pub fn find_variable(&self, code: &str) -> Option<&Variable> {
        self.0.iter().find(|v| v.code == code)
    }

    pub fn set_value<S: Into<String>, V: Into<VariableValue>>(&mut self, code: S, value: V) {
        let code = code.into();
        let value = value.into();
        if let Some(v) = self
            .0
            .iter_mut()
            .find(|v| v.code.eq_ignore_ascii_case(&code))
        {
            v.value = value;
            return;
        }
        let v = Variable::new(code, value);
        self.0.push(v);
    }

    pub fn fix_main_target(&self, save_file: &str) -> String {
        if let Ok(ismain) = self.get_ismain() {
            if ismain {
                return save_file.replace(SAVE_SUFFIX, BASE_SUFFIX);
            }
        }
        save_file.to_string()
    }

    pub fn substitute<S: Into<String>>(&self, text: S) -> String {
        let mut result = text.into();
        let js_variables = Variables::create_js_varibales(&result);
        for js_variable in js_variables.0 {
            if let Some(v) = self.find_variable(&js_variable.code) {
                let value = v.value.to_string();
                result = result.replace(js_variable.value.to_string().as_str(), value.as_str());
            }
        }
        result
    }

    pub fn substitute_add<S: Into<String>>(
        &self,
        text: S,
        use_wildcards: bool,
        pattern_code: &str,
    ) -> Result<String> {
        let mut result = text.into();
        let js_variables = Variables::create_add_varibales(&result);
        for js_variable in js_variables.0 {
            let v_code = js_variable.code.to_string();
            let value = js_variable.value.to_string();
            let code_split = v_code.as_str().split('|').collect::<Vec<&str>>();

            if code_split.len() != 3 {
                return Err(ConfigError::InvalidVariable(v_code.to_string()));
            }

            let code = code_split[0];

            let len = code_split[2]
                    .parse::<usize>()
                    .map_err(|_| ConfigError::InvalidVariable(code.to_string()))?;

            if use_wildcards {
                result = result.replace(value.as_str(), "??".repeat(len).as_str());
                continue;
            }

            if let Some(v1) = self.find_variable(code)
                && let Some(v2) = self.find_variable(pattern_code)
            {
                let target_addr = v1
                    .as_usize()
                    .ok_or(ConfigError::GetVariabledValueError(code.to_string()))?
                    as u64;
                let current_addr = v2
                    .as_usize()
                    .ok_or(ConfigError::GetVariabledValueError(pattern_code.to_string()))?
                    as u64;
                let offset_adjust = code_split[1]
                    .parse::<i64>()
                    .map_err(|_| ConfigError::InvalidVariable(code.to_string()))?;
                let mut patch_code =
                    calculate_jump_offset_bytes(current_addr, target_addr, offset_adjust)?.to_hex();
                let parch_code_len = patch_code.len() / 2;
                if parch_code_len > len {
                    return Err(ConfigError::CalcAddressError(pattern_code.to_string()));
                }
                if parch_code_len < len {
                    let diff_len = len - parch_code_len;
                    patch_code = format!("{}{}", patch_code, "90".repeat(diff_len));
                }
                result = result.replace(value.as_str(), patch_code.as_str());
            } else {
                return Err(ConfigError::GetVariabledValueError(code.to_string()).into());
            }
        }
        Ok(result)
    }
}

impl TryFrom<&Patches> for Variables {
    type Error = ConfigError;
    fn try_from(patches: &Patches) -> RResult<Self, Self::Error> {
        let mut variables = Variables::default();
        for patch in &patches.0 {
            for pattern in &patch.patterns.0 {
                for address in &pattern.addresses.0 {
                    let add = address.start_rva;
                    if add <= 0 {
                        return Err(ConfigError::InvalidAddress);
                    }
                    let v = VariableValue::Usize(add);
                    let variable = Variable::new(pattern.code.clone(), v);
                    variables.push(variable)
                }
            }
        }
        if variables.is_empty() {
            return Err(ConfigError::AddressesEmptyError);
        }
        Ok(variables)
    }
}

impl Debug for Variables {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(for v in &self.0 {
            write!(f, "{} = {}\n", v.code, v.value)?;
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VariableValue {
    String(String),
    Number(i64),
    Usize(usize),
    Float(f64),
    Boolean(bool),
}

impl Display for VariableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VariableValue::String(s) => write!(f, "{}", s),
            VariableValue::Number(n) => write!(f, "{}", n),
            VariableValue::Float(fl) => write!(f, "{}", fl),
            VariableValue::Boolean(b) => write!(f, "{}", b),
            VariableValue::Usize(u) => write!(f, "{}", u),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    code: String,
    value: VariableValue,
}

impl Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.code, self.value)
    }
}

impl Variable {
    pub fn new<S: Into<String>, V: Into<VariableValue>>(code: S, value: V) -> Self {
        let code = code.into();
        let value = value.into();
        Self { code, value }
    }

    pub fn get_code(&self) -> &str {
        &self.code
    }

    pub fn get_value(&self) -> &VariableValue {
        &self.value
    }

    pub fn set_value(&mut self, value: impl Into<VariableValue>) {
        self.value = value.into();
    }

    // 添加获取特定类型值的方法
    pub fn as_str(&self) -> Option<&str> {
        if let VariableValue::String(s) = &self.value {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_i64(&self) -> Option<i64> {
        if let VariableValue::Number(n) = &self.value {
            Some(*n)
        } else {
            None
        }
    }

    pub fn as_usize(&self) -> Option<usize> {
        if let VariableValue::Usize(u) = &self.value {
            Some(*u)
        } else {
            None
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        if let VariableValue::Float(f) = &self.value {
            Some(*f)
        } else if let VariableValue::Number(n) = &self.value {
            Some(*n as f64)
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if let VariableValue::Boolean(b) = &self.value {
            Some(*b)
        } else {
            None
        }
    }
}

// 为各种类型实现Into<VariableValue>
impl From<String> for VariableValue {
    fn from(s: String) -> Self {
        VariableValue::String(s)
    }
}

impl From<&String> for VariableValue {
    fn from(s: &String) -> Self {
        VariableValue::String(s.to_string())
    }
}

impl From<&str> for VariableValue {
    fn from(s: &str) -> Self {
        VariableValue::String(s.to_string())
    }
}

impl From<i64> for VariableValue {
    fn from(n: i64) -> Self {
        VariableValue::Number(n)
    }
}

impl From<usize> for VariableValue {
    fn from(n: usize) -> Self {
        VariableValue::Usize(n)
    }
}

impl From<f64> for VariableValue {
    fn from(f: f64) -> Self {
        VariableValue::Float(f)
    }
}

impl From<bool> for VariableValue {
    fn from(b: bool) -> Self {
        VariableValue::Boolean(b)
    }
}

/// 序列化和反序列化
impl Serialize for Variable {
    fn serialize<S>(&self, serializer: S) -> RResult<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.code, &self.value)?;
        map.end()
    }
}

impl<'de> Deserialize<'de> for Variable {
    fn deserialize<D>(deserializer: D) -> RResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // 使用临时结构体来反序列化
        #[derive(Deserialize)]
        struct TempVariable {
            #[serde(rename = "$key")]
            code: String,
            #[serde(flatten)]
            value: VariableValue,
        }

        let temp = TempVariable::deserialize(deserializer)?;
        Ok(Variable {
            code: temp.code,
            value: temp.value,
        })
    }
}

impl Serialize for Variables {
    fn serialize<S>(&self, serializer: S) -> RResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(self.0.len()))?;
        for variable in &self.0 {
            map.serialize_entry(&variable.code, &variable.value)?;
        }
        map.end()
    }
}

impl<'de> Deserialize<'de> for Variables {
    fn deserialize<D>(deserializer: D) -> RResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let map: HashMap<String, VariableValue> = Deserialize::deserialize(deserializer)?;
        let variables = map
            .into_iter()
            .map(|(code, value)| Variable { code, value })
            .collect();
        Ok(Variables(variables))
    }
}
