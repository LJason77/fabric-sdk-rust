use rocket::{
    http::Status,
    post,
    response::status::Custom,
    serde::json::{json, Json, Value},
};

use crate::models::person::PersonLogin;

/// 登录
#[post("/login", data = "<login>", format = "application/json")]
pub async fn login(login: Json<PersonLogin>) -> Custom<Value> {
    println!("{:?}\n", login);
    Custom(Status::Ok, json!({ "token": "aaa" }))
}

// TODO 注册
