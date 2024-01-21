// /src/structs/structs.rs

#[derive(FromForm, Debug)]
pub struct User {
    // id: i32, it's AUTO_INCREMENT in the schema.
    pub username: String,
    pub password: String,
}
