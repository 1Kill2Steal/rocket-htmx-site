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
    use rocket::response::content::RawHtml;


    if user.password.len() < 8 {
        return RawHtml(
            r#"
                <h2>Please use at least 8 symbols</h2>
                <p>Also, make sure there's 1 upper case, 1 lower case and 1 digit as well...</p>
            "#,
        );
    }

    let mut has_uppercase = false;
    let mut has_lowercase = false;
    let mut has_digit = false;

    for c in user.password.chars() {
        if c.is_ascii_uppercase() {
            has_uppercase = true;
        } else if c.is_ascii_lowercase() {
            has_lowercase = true;
        } else if c.is_ascii_digit() {
            has_digit = true;
        }
    }

    if !has_uppercase || !has_lowercase || !has_digit {
        return RawHtml(
            r#"
                <h2>
                    Please include an upper case character,<br>
                    a lower case character and a digit to your password
                </h2>
            "#,
        );
    }

    let conn = establish_connection();

    if find_user_by_username(&conn, &user.username).is_some() {
        return RawHtml(
            r#"
                <h2>Username already exists. Please choose a different one.</h2>
            "#,
        );
    }


    // println!("User info: {:?}", user);
    let generated_salt = generate_salt();
    match hash_password(user.password.as_bytes(), &generated_salt) {
        Ok(password_hash) => {
            println!("Salt: {}", &generated_salt.to_string());
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
            println!("User Salt: {}", &generated_salt.to_string());
            println!("Current working directory: {:?}", std::env::current_dir());
            let _ = conn
                .expect("User Info")
                .execute(
                "INSERT INTO users (username, password_hash, password_salt) VALUES (?1, ?2, ?3)",
                &[&user.username.to_string().to_lowercase(), &extracted_hash, &generated_salt.to_string()],
            );
            // I'm so lazy to write anything more meaningful than that
            return RawHtml(
                r#"
                    <h2>Signed up!</h2>
                "#,
            );
        }
        Err(err) => {
            eprintln!("Internal error signing up: {:?}", err);
            return RawHtml(r#"
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
