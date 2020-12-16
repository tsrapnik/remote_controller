#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response;
use std::{io, io::Write, net::TcpStream, str};
use system_shutdown;

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
    fn send_tcp_message(message: &[u8]) -> Result<(), io::Error> {
        let mut stream = TcpStream::connect("192.168.1.5:5000")?;
        stream.write(message)?;
        Ok(())
    }

    if let Ok(command) = str::from_utf8(command.peek()) {
        match command {
            "shutdown" => {
                println!("shutting down.");
                // system_shutdown::shutdown();
            }
            "brightness_50" => {
                let message: &[u8] = &[
                    0xa6, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x01, 0x32, 0x37, 0x37, 0x32, 0x14, 0x32,
                    0x32, 0x01, 0xb9,
                ];
                if send_tcp_message(message).is_err() {
                    println!("changing brightness failed.")
                }
            }
            "brightness_0" => {
                let message: &[u8] = &[
                    0xa6, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x01, 0x32, 0x00, 0x37, 0x32, 0x14, 0x32,
                    0x32, 0x01, 0x8e,
                ];
                if send_tcp_message(message).is_err() {
                    println!("changing brightness failed.")
                }
            }
            _ => {
                println!("unknown command: \"{}\"", command);
            }
        }
    }
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![load_html, load_styles, load_script, execute_command],
        )
        .launch();
}
