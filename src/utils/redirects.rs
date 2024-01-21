// /src/utils/redirects.rs

use rocket::response::Redirect;

pub fn homepage_redirect() -> Redirect {
    Redirect::to("/")
}
