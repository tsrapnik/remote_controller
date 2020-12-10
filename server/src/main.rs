#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::{response};
use system_shutdown;
use std::str;

#[get("/")]
fn load_html() -> Option<response::NamedFile> {
    response::NamedFile::open("../client/index.html").ok()
}

#[get("/styles.css")]
fn load_styles() -> Option<response::NamedFile> {
    response::NamedFile::open("../client/styles.css").ok()
}

#[get("/script.js")]
fn load_script() -> Option<response::NamedFile> {
    response::NamedFile::open("../client/script.js").ok()
}

#[post("/", format = "text/plain", data = "<command>")]
fn execute_command(command: rocket::Data) -> () {
    if let Ok(command) = str::from_utf8(command.peek()) {
        match command {
            "shutdown" =>
                println!("shutting down."),
                // system_shutdown::shutdown(),
            _ =>
                println!("unknown command: \"{}\"", command),
        }
    }
}

fn main() {
    rocket::ignite().mount("/", routes![load_html, load_styles, load_script, execute_command]).launch();
}