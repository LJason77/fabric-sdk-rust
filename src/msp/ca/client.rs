use std::fs::read_to_string;

use serde::{Deserialize, Serialize};

/// CA 的客户端
#[derive(Debug, Deserialize, Serialize)]
pub struct Names {
    #[serde(rename = "C")]
    pub c: String,
    #[serde(rename = "ST")]
    pub st: String,
    #[serde(rename = "L")]
    pub l: String,
    #[serde(rename = "O")]
    pub o: String,
    #[serde(rename = "OU")]
    pub ou: String,
}

/// CA 的客户端
#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    pub cn: String,
    pub names: Names,
    pub hosts: String,
}

impl Default for Client {
    fn default() -> Self {
        let hosts = read_to_string("/etc/hostname")
            .expect("读取失败：")
            .replace("\n", "");
        Client {
            cn: "admin".to_string(),
            names: Names {
                c: "US".to_string(),
                st: "North Carolina".to_string(),
                l: "".to_string(),
                o: "Hyperledger".to_string(),
                ou: "Fabric".to_string(),
            },
            hosts,
        }
    }
}

impl Client {
    pub fn from_file(path: &str) -> Client {
        match read_to_string(path) {
            Ok(yaml) => match serde_yaml::from_str::<Self>(&yaml) {
                Ok(client) => client,
                Err(err) => {
                    println!("配置文件解析失败：{:?}\n使用默认配置！", err);
                    Self::default()
                }
            },
            Err(err) => {
                println!("读取 {} 文件失败：{:?}\n使用默认配置！", path, err);
                Self::default()
            }
        }
    }
}
