//! # kensa
//! 
//! Kensa is a HTTP REST service for inspecting GitHub Pull Requests and assigning "features"
//! based on configurable rules. Kensa receives Pull Requests, applies the configured rules to the
//! PR, and then responds with the set of matching features. Features can be anything from relative
//! patch size, to the functional area of the project being modified. The goal of Kensa is to
//! provide a tool for automating inspection and categorization of PRs.

#![deny(missing_debug_implementations, missing_copy_implementations,
        trivial_numeric_casts,
        unsafe_code,
        unused_import_braces)]

#![feature(plugin, proc_macro)]
#![plugin(rocket_codegen)]
#![recursion_limit = "1024"]

extern crate chrono;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
#[macro_use] extern crate error_chain;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate uuid;

pub mod handlers;
pub mod schema;
pub mod models;
pub mod errors;
pub mod util;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

pub fn start_server() {
    rocket::ignite()
        .mount("/", routes![index])
        .mount("/features", routes![handlers::features::index, handlers::features::show, handlers::features::create, handlers::features::destroy])
        .catch(errors![handlers::not_found])
        .launch();
}
