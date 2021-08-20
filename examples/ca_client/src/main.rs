use fabric::msp::ca::Client;
use std::fs::read_to_string;

fn main() {
    dotenv::dotenv().ok();
    println!(
        "{:?}\n",
        dotenv::var("FABRIC_CA_CLIENT_HOME").unwrap_or_else(|_| {
            let home = dotenv::var("HOME").unwrap();

            "sss".to_string()
        })
    );
    let s = Client::from_file("./config_e2e.yaml");
    println!("{:?}\n", s);
}
