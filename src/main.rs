#![allow(non_snake_case)]
use colored::Colorize;
use dotenv_codegen::dotenv;
use reqwest::header::CONTENT_TYPE;
use rpassword::read_password;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io;
use text_io::read;

#[derive(Deserialize)]
struct AdminToken {
    token: String,
}

#[derive(Debug, Serialize)]
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
    let client = reqwest::blocking::Client::new();

    let mut map = HashMap::new();

    print!("Admin email: ");
    flush();
    let admin_email: String = read!();
    map.insert("email", &admin_email);

    print!("Admin password: ");
    flush();
    let admin_password: String = read_password().unwrap();
    map.insert("password", &admin_password);

    let res = client
        .post(format!("{}api/auth", dotenv!("URI")))
        .json(&map)
        .send();

    let response_text = res.unwrap().text().unwrap();

    let admin_token: AdminToken = match serde_json::from_str(&response_text) {
        Ok(admin_token) => admin_token,
        Err(_) => {
            println!("{}", "Invalid credentials".red().bold());
            return;
        }
    };

    println!("{}", "Admin log in successful".green());
    println!();

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
    println!("{}", "- Professional Registration -".blue().bold());

    print!("Email: ");
    flush();
    io::stdin().read_line(&mut professional.email).unwrap();
    professional.email.pop();

    print!("Password: ");
    flush();
    io::stdin().read_line(&mut professional.password).unwrap();
    professional.password.pop();

    print!("Name: ");
    flush();
    io::stdin().read_line(&mut professional.name).unwrap();
    professional.name.pop();

    print!("Clinic: ");
    flush();
    io::stdin().read_line(&mut professional.clinic).unwrap();
    professional.clinic.pop();

    print!("Gender: ");
    flush();
    io::stdin().read_line(&mut professional.gender).unwrap();
    professional.gender.pop();

    print!("Year Of Birth: ");
    flush();
    io::stdin()
        .read_line(&mut professional.yearOfBirth)
        .unwrap();
    professional.yearOfBirth.pop();

    print!("Description: ");
    flush();
    io::stdin()
        .read_line(&mut professional.description)
        .unwrap();
    professional.description.pop();

    print!("Language(fr/en): ");
    flush();
    io::stdin().read_line(&mut professional.language).unwrap();
    professional.language.pop();

    print!("Phone: ");
    flush();
    io::stdin().read_line(&mut professional.phone).unwrap();
    professional.phone.pop();

    let res = client
        .post(format!("{}api/users/admin", dotenv!("URI")))
        .header(CONTENT_TYPE, "application/json")
        .header("x-auth-token", &admin_token.token)
        .json(&professional)
        .send();

    let response_text = res.unwrap().text().unwrap();

    println!("{}", response_text);
}

fn flush() {
    std::io::Write::flush(&mut std::io::stdout()).expect("flush failed!");
}
