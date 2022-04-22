use dotenv_codegen::dotenv;
use rpassword::read_password;
use serde::Deserialize;
use std::collections::HashMap;
use text_io::read;

#[derive(Deserialize)]
struct AdminToken {
    token: String,
}

fn main() {
    let mut map = HashMap::new();

    print!("Admin email: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let admin_email: String = read!();
    map.insert("email", &admin_email);

    print!("Admin password: ");
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
    let admin_password: String = read_password().unwrap();
    map.insert("password", &admin_password);

    let client = reqwest::blocking::Client::new();
    let res = client.post(dotenv!("URI")).json(&map).send();

    let response_text = res.unwrap().text().unwrap();

    let admin_token: AdminToken = match serde_json::from_str(&response_text) {
        Ok(admin_token) => admin_token,
        Err(_) => {
            println!("Invalid credentials");
            return;
        }
    };

    println!("{:?}", admin_token.token);
}
