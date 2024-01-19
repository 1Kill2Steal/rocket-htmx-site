// /src/main.rs
#[macro_use]
extern crate rocket;

/*
#[get("/")]
fn index() -> &'static str {
    "Rust + HTMX test website!"
}
*/

#[get("/update")]
fn update() -> &'static str {
    "Updated content now!"
}

#[launch]
fn rocket() -> _ {
    use rocket::fs::FileServer;
    rocket::build()
        .mount("/", FileServer::from("static"))
        .mount("/", routes![/*index, */update])
}
