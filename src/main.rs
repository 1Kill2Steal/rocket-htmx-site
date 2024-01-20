// /src/main.rs

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::response::content::RawHtml;
use rocket::form::Form;

use rusqlite::{Connection, Result};

mod structs;
use self::structs::structs::{ User };

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

#[post("/user_sign_up", data = "<user>")]
fn user_sign_up(user: Form<User>) {
    //TODO...
}
#[post("/user_login", data = "<user>")]
fn user_login(user: Form<User>) {
    //TODO...
}


#[launch]
fn rocket() -> _ {
    use rocket::fs::FileServer;
    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/", routes![nav_content])
        .mount("/", routes![update])
}
