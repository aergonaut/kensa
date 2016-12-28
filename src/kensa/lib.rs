#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

pub fn start_server() {
    rocket::ignite().mount("/", routes![index]).launch();
}
