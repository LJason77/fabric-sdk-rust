use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ConfigBackend {
    /// SDK 解析的版本号
    pub version: String,
}

impl ConfigBackend {
    /// 从配置文件中读取
    pub fn from_file(path: &str) {
        match read_to_string(path) {
            Ok(yaml) => {
                let s = serde_yaml::from_str::<Self>(&yaml);
                println!("{:?}\n", s.unwrap());
            }
            Err(err) => {
                println!("读取 {} 文件失败：{:?}\n", path, err);
            }
        }
    }
}
