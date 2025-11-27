// 定义相关的工具函数

use std::fs;
use std::path::PathBuf;

use crate::tools_types::{SelfDefineMonitorConfig};



pub fn get_file_path(file_name: &str) -> PathBuf {
    let current_dir = std::env::current_dir().unwrap();
    current_dir.join(file_name)
}


// 读取JSON文件
pub fn read_json_file(file_path: &str) -> Result<Vec<SelfDefineMonitorConfig>, Box<dyn std::error::Error>> {
    let file_path = get_file_path(file_path);
    let content = fs::read_to_string(file_path)?;  // 读取为字符串
    let json_data: Vec<SelfDefineMonitorConfig> = serde_json::from_str(&content)?; // 反序列化为结构体
    Ok(json_data)
}