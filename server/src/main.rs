#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use system_shutdown;

#[get("/")]
fn hello() -> String {
    format!("Hello!")
}

#[get("/kill")]
fn bye() -> String {
    system_shutdown::shutdown();
    format!("Shutting down!")
}

fn main() {
    rocket::ignite().mount("/", routes![hello, bye]).launch();
}