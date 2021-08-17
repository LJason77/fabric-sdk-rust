#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

pub mod core;

/// 根据提供的选项集初始化SDK。
/// ConfigOptions 提供应用程序的配置。
pub fn new(config: core::Config) {
    println!("{:#?}\n", config);
}
