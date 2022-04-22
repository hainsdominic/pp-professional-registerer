#![allow(non_snake_case)]
use dotenv_codegen::dotenv;
use rpassword::read_password;
use serde::Deserialize;
use std::collections::HashMap;
use text_io::read;

#[derive(Deserialize)]
struct AdminToken {
    token: String,
}

#[derive(Debug)]
struct Professional {
    r#type: String,
    email: String,
    password: String,
    name: String,
    clinic: String,
    gender: String,
    yearOfBirth: String,
    description: String,
    language: String,
    phone: String,
    profession: String,
}

fn main() {
    let mut map = HashMap::new();

    print!("Admin email: ");
    flush();
    let admin_email: String = read!();
    map.insert("email", &admin_email);

    print!("Admin password: ");
    flush();
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

    println!("Admin log in successful");

    let mut professional = Professional {
        r#type: String::from("professional"),
        email: String::new(),
        password: String::new(),
        name: String::new(),
        clinic: String::new(),
        gender: String::new(),
        yearOfBirth: String::new(),
        description: String::new(),
        language: String::new(),
        phone: String::new(),
        profession: String::from("chiropractor"),
    };
    println!("- Professional Registration -");
    print!("Email: ");
    flush();
    professional.email = read!();
    print!("Password: ");
    flush();
    professional.password = read!();

    print!("Name: ");
    flush();
    professional.name = read!();

    print!("Clinic: ");
    flush();
    professional.clinic = read!();

    print!("Gender: ");
    flush();
    professional.gender = read!();

    print!("Year Of Birth: ");
    flush();
    professional.yearOfBirth = read!();

    print!("Description: ");
    flush();
    professional.description = read!();

    print!("Language(fr/en): ");
    flush();
    professional.language = read!();

    print!("Phone: ");
    flush();
    professional.phone = read!();

    println!("{:#?}", professional);
}

fn flush() {
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
}
