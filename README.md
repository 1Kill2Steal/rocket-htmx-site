# Rust Rocket + HTMX site
It's a work in progress without much functionality.
Made this project to test out how to work with Rust for websites using Rocket.

## Prerequisites
1. [Rust language w/ cargo](https://rustup.rs/)
- This project uses - **rustup nightly** - `rustup default nightly`

## To use the application
1. Run `cargo build` (You only need to run it once)
2. Run `cargo run` OR `cargo run --release`

## TODO:
- Implement a password regex pattern.
  Most likely this: `Regex::new(r"^(?=.*[A-Z])(?=.*[a-z])(?=.*\d)(?=.*[^A-Za-z0-9]).{8,}$").unwrap();`
  1 upper case char, 1 lower case char, 1 digit, 1 special char, at least 8 characters long.
- Implement the `rusqlite` functionality for the log in system.
- Implement a **private** and **secure** cookie (of the hashed password) to persistently store the session.
- Implement a **log out** system that removes the cookie.

### A lot of UI Improvement
- Boring. I don't know when I'll do it...

### Buying a real domain and hosting on it- (probably not soon)
