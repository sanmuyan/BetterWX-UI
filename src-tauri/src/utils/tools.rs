use std::collections::HashSet;
use regex::Regex;


/**
 * @description: 比较两个版本号的大小
 * @param v1 第一个版本号
 * @param v2 第二个版本号
 * @return 如果 v1 小于 v2，返回 -1；如果 v1 大于 v2，返回 1；如果相等，返回 0。
 */
pub fn compare_versions(v1: &str, v2: &str) -> i32 {
    let v1_parts: Vec<&str> = v1.split('.').collect();
    let v2_parts: Vec<&str> = v2.split('.').collect();
    let max_len = std::cmp::max(v1_parts.len(), v2_parts.len());
    for i in 0..max_len {
        let num1 = v1_parts
            .get(i)
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        let num2 = v2_parts
            .get(i)
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);
        match num1.cmp(&num2) {
            std::cmp::Ordering::Less => return -1,
            std::cmp::Ordering::Greater => return 1,
            std::cmp::Ordering::Equal => continue,
        }
    }
    0
}

pub fn extract_variables(template: &str) -> HashSet<String> {
    let re = Regex::new(r"\{([^}]+)\}").unwrap();
    let mut variables = HashSet::new();
    for cap in re.captures_iter(template) {
        if let Some(var) = cap.get(1) {
            variables.insert(var.as_str().to_string());
        }
    }
    variables
}