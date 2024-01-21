// /src/structs/structs.rs

use rocket::form::Form;

#[derive(FromForm, Debug)]
pub struct User {
    // id: i32, it's AUTO_INCREMENT in the schema.
    pub username: String,
    pub password: String,
}
