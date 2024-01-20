// /src/structs/structs.rs

use rocket::form::Form;

#[derive(FromForm, Debug)]
pub struct User {
    id: i32,
    username: String,
    password: String,
}
