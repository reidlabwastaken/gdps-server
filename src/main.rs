#![feature(decl_macro)]

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String {
    return String::from("index | coming soon to a localhost:8000 near u");
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}