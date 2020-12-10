#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use system_shutdown;
use std::str;

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
    rocket::ignite().mount("/", routes![execute_command]).launch();
}