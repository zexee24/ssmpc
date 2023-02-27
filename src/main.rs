use rocket::http::ContentType;
use std::{fs, env};

#[macro_use]extern crate rocket;

#[get("/")]
fn index() -> (ContentType, String){
    (ContentType::HTML, String::from(fs::read_to_string("src/pages/index.html").expect("error opening file")))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
