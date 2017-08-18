#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
use rocket::http::ContentType;
use rocket::response::content::Content;

#[get("/generate")]
fn generate() -> Content<String> {
    let flag = "flag_j29FJ9zi9rjFI832uFfjE";
    Content(
        ContentType::HTML,
        format!(
            "<!DOCTYPE html><html><body><form><input type=\"password\" value=\"{}\" /></form></body></html>",
            flag
        ),
    )
}

fn main() {
    rocket::ignite().mount("/", routes![generate]).launch();
}
