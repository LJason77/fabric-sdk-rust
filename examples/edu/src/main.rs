#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

use api::user;
use fabric::core::Config;
use rocket::routes;

mod api;
mod models;

#[rocket::main]
async fn main() -> Result<(), String> {
    let config = Config::from_file("./config_e2e.yaml");
    fabric::new(config);

    let rocket = rocket::build().mount("/users", routes![user::login]);

    if let Err(err) = rocket.launch().await {
        println!("Rocket Err: {}", err);
    }
    Ok(())
}
