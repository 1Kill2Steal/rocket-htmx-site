// /src/main.rs

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::form::Form;

use rusqlite::{Connection, Result};

mod structs;
use self::structs::structs::{ User };
mod utils;
use self::utils::utils::{ generate_salt, hash_password };

#[get("/nav_content")]
fn nav_content() -> rocket::response::content::RawHtml<&'static str> {
    rocket::response::content::RawHtml(r#"
        <nav>
            <div class="nav_item">
                <a href="./index.html">Homepage</a>
            </div>
            <div class="nav_item">
                <a href="./sign_up.html">Sign Up</a>
            </div>
            <div class="nav_item">
                <a href="./login.html">Log In</a>
            </div>
        </nav>
    "#)
}

#[get("/update")]
fn update() -> &'static str {
    "Updated content now!"
}

#[put("/user_sign_up", data = "<user>")]
fn user_sign_up(
    user: Form<User>
) -> rocket::response::content::RawHtml<&'static str> {
    //TODO...
    let generated_salt = utils::utils::generate_salt();
    match hash_password(user.password.as_bytes(), &generated_salt) {
        Ok(password_hash) => {
            println!("Salt: {}", generated_salt.as_str());
            println!("Hashed password: {}", password_hash);
        }
        Err(err) => eprintln!("Error generating the hashed password: {:?}", err),
    }

    // I'm so lazy to write anything more meaningful than that
    rocket::response::content::RawHtml(r#"
        <h2>Signed up!</h2>
    "#)
}
#[put("/user_login", data = "<user>")]
fn user_login(user: Form<User>) {
    //TODO...
}


#[launch]
fn rocket() -> _ {
    use rocket::fs::FileServer;
    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/", routes![nav_content, update, user_sign_up, user_login])
}
