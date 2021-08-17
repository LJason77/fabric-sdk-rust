use std::fs::read_to_string;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// 日志等级
#[derive(Debug)]
pub enum LoggingLevel {
    INFO,
}

impl Serialize for LoggingLevel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            LoggingLevel::INFO => "info",
        })
    }
}

impl<'de> Deserialize<'de> for LoggingLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "info" | _ => LoggingLevel::INFO,
        })
    }
}

/// 日志
#[derive(Debug, Deserialize, Serialize)]
pub struct Logging {
    pub level: LoggingLevel,
}

/// 路径配置
#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Deserialize, Serialize)]
pub struct PathConfig {
    pub path: String,
}

/// 凭证存储
#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialStore {
    /// 可选。由用户存储使用。如果所有凭证都嵌入配置中，并且在其他地方进行注册，则不需要
    pub path: Option<String>,
    /// 可选。基于软件的实现需要一个密钥存储。基于 PKCS#11 的实现则不需要。
    #[serde(rename = "cryptoStore")]
    pub crypto_store: Option<PathConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Security {
    pub enabled: bool,
    #[serde(rename = "hashAlgorithm")]
    pub hash_algorithm: String,
    #[serde(rename = "softVerify")]
    pub soft_verify: bool,
    pub level: i16,
}

/// 客户端的 BCCSP 配置
#[derive(Debug, Deserialize, Serialize)]
pub struct BCCSP {
    pub security: Security,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Client {
    /// 这个应用实例属于哪个组织
    pub organization: String,
    /// 日志
    pub logging: Logging,
    /// 编码配置
    pub cryptoconfig: PathConfig,
    /// 凭证存储
    #[serde(rename = "credentialStore")]
    pub credential_store: CredentialStore,
    /// 客户端的 BCCSP 配置
    #[serde(rename = "BCCSP")]
    pub bccsp: BCCSP,
}

/// 如果缺少通道配置或已定义的通道配置缺少信息，则使用默认通道。
/// 如果通道没有定义对等体，那么将使用默认通道的对等体。
/// 如果通道没有定义订单，那么将使用默认通道的订单。
/// 如果通道没有定义策略，那么将使用默认通道的策略。
/// 另外，如果通道定义了策略，而某些策略信息缺失，那么缺失的信息将由默认通道来填补。
#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelName {
    pub orderers: Vec<String>,
}

/// 可选。
/// 大多数应用程序都会有这个部分，这样就可以根据下面的内容来构建通道对象。
/// 如果一个应用程序正在创建通道，那么它很可能不需要这一部分。
#[derive(Debug, Deserialize, Serialize)]
pub struct Channels {
    pub channelname: ChannelName,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// SDK 解析的版本号
    pub version: String,
    /// 客户端
    pub client: Client,
    /// 通道
    pub channels: Option<Channels>,
}

impl Config {
    /// 从配置文件中读取
    pub fn from_file(path: &str) -> Config {
        match read_to_string(path) {
            Ok(yaml) => match serde_yaml::from_str::<Self>(&yaml) {
                Ok(config) => config,
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

impl Default for Config {
    fn default() -> Self {
        Config {
            version: "1.0.0".to_string(),
            client: Client {
                organization: "Org1".to_string(),
                logging: Logging {
                    level: LoggingLevel::INFO,
                },
                cryptoconfig: PathConfig {
                    path: "./crypto-config".to_string(),
                },
                credential_store: CredentialStore {
                    path: None,
                    crypto_store: None,
                },
                bccsp: BCCSP {
                    security: Security {
                        enabled: false,
                        hash_algorithm: "SHA2".to_string(),
                        soft_verify: false,
                        level: 0,
                    },
                },
            },
            channels: None,
        }
    }
}
