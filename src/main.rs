// /src/main.rs

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::form::Form;
// TODO - make a cookie with the hashed password
use rocket::http::{ Cookie };

mod structs;
use self::structs::structs::{ User };
mod utils;
// use self::utils::redirects::{ homepage_redirect };

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
fn user_sign_up(user: Form<User>) -> rocket::response::content::RawHtml<&'static str> {
    use self::utils::hash_and_salt::{generate_salt, hash_password, extract_hash};
    use self::utils::database_connection::{establish_connection, find_user_by_username};

    let conn = establish_connection();

    if find_user_by_username(&conn, &user.username).is_some() {
        return rocket::response::content::RawHtml(
            r#"
                <h2>Username already exists. Please choose a different one.</h2>
            "#,
        );
    }


    // println!("User info: {:?}", user);
    let generated_salt = generate_salt();
    match hash_password(user.password.as_bytes(), &generated_salt) {
        Ok(password_hash) => {
            println!("Salt: {}", generated_salt.as_str());
            println!("Hashed password: {}", password_hash);

            let extracted_hash: std::string::String = extract_hash(&password_hash);

            println!("Extracted hash: {}", extracted_hash);

            let login_cookie = Cookie::build(extracted_hash.clone())
                .path("/")
                .secure(true)
                .build();

            println!("Connected to the database.");
            println!("Cookie (not for eating): {}", login_cookie);
            println!("Username: {}", &user.username.to_string());
            println!("Password: {}", &extracted_hash);
            println!("Current working directory: {:?}", std::env::current_dir());
            let _ = conn
                .expect("User Info")
                .execute(
                "INSERT INTO users (username, password_hash) VALUES (?1, ?2)",
                &[&user.username.to_string(), &extracted_hash],
            );
            // I'm so lazy to write anything more meaningful than that
            return rocket::response::content::RawHtml(
                r#"
                    <h2>Signed up!</h2>
                "#,
            );
        }
        Err(err) => {
            eprintln!("Error generating the hashed password: {:?}", err);
            return rocket::response::content::RawHtml(r#"
                <h2>Internal error signing up...</h2>
            "#);
        }
    }
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
