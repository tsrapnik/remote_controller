#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket::response;
use rocket_contrib::json::Json;
use serde::Deserialize;
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

#[derive(Deserialize)]
struct Command {
    command: String,
}

#[post("/", format = "application/json", data = "<command>")]
fn execute_command(command: Json<Command>) -> () {
    fn send_tcp_message(message: &[u8]) -> Result<(), io::Error> {
        let mut stream = TcpStream::connect("192.168.1.5:5000")?;
        stream.write(message)?;
        Ok(())
    }

    fn open_site(site: &str) {
        let result = open::that(site);
        if let Err(error) = result {
            println!("Failed to open \"{}\". error: \"{}\"", site, error);
        }
    }

    match command.command.as_str() {
        "shutdown" => {
            if system_shutdown::shutdown().is_err() {
                println!("shutting down failed.");
            }
        }
        "brightness_100" => {
            let message: &[u8] = &[
                0xa6, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x01, 0x32, 0x64, 0x37, 0x32, 0x14, 0x32, 0x32,
                0x01, 0xea,
            ];
            if send_tcp_message(message).is_err() {
                println!("changing brightness failed.")
            }
        }
        "brightness_50" => {
            let message: &[u8] = &[
                0xa6, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x01, 0x32, 0x37, 0x37, 0x32, 0x14, 0x32, 0x32,
                0x01, 0xb9,
            ];
            if send_tcp_message(message).is_err() {
                println!("changing brightness failed.")
            }
        }
        "brightness_0" => {
            let message: &[u8] = &[
                0xa6, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x01, 0x32, 0x00, 0x37, 0x32, 0x14, 0x32, 0x32,
                0x01, 0x8e,
            ];
            if send_tcp_message(message).is_err() {
                println!("changing brightness failed.")
            }
        }
        "shutdown_monitor" => {
            let message: &[u8] = &[0xa6, 0x01, 0x00, 0x00, 0x00, 0x04, 0x01, 0x18, 0x01, 0xbb];
            if send_tcp_message(message).is_err() {
                println!("shutting down monitor failed.")
            }
        }
        "netflix" => {
            open_site("https://netflix.com");
        }
        "vrt_nu_tv_guide" => {
            open_site("https://www.vrt.be/vrtnu/tv-gids/");
        }
        "vrt_nu_live" => {
            open_site("https://www.vrt.be/vrtnu/livestream/");
        }
        _ => {
            println!("unknown command: \"{}\"", command.command);
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
