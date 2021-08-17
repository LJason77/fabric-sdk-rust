use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct PersonLogin {
    pub account: String,
    pub password: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    /// 编号（工号、学号）
    pub id: String,
    /// 登录账号
    pub account: String,
    /// 登录密码
    pub password: String,
    /// 联系电话
    pub phone: String,
    /// 电子邮箱
    pub email: String,
    /// 姓名
    pub name: String,
    /// 地址
    pub address: String,
    /// 学院
    pub college: String,
    /// 系部
    pub department: String,
    /// 班级
    pub class: String,
    /// 人员类型，学生，老师，学校
    #[serde(rename = "type")]
    pub type_: String,
    /// 人员状态
    pub status: String,
}
