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
enum Command {
    Shutdown,
    Brightness {value: u8},
    ShutdownMonitor,
    Netflix,
    VrtNuTvGuide,
    VrtNuLive,
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

    match command.into_inner() {
        Command::Shutdown => {
            if system_shutdown::shutdown().is_err() {
                println!("shutting down failed.");
            }
        }
        Command::Brightness {value} => {
            let mut message = [
                0xa6u8, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x01, 0x32, value, 0x37, 0x32, 0x14, 0x32, 0x32,
                0x01, 0xea, 0x00
            ];
            let checksum = message[.. message.len() - 1].iter().fold(0x00, |acc, x| acc ^ x);
            message[message.len() - 1] = checksum;
            if send_tcp_message(&message).is_err() {
                println!("changing brightness failed.")
            }
        }
        Command::ShutdownMonitor => {
            let message: &[u8] = &[0xa6, 0x01, 0x00, 0x00, 0x00, 0x04, 0x01, 0x18, 0x01, 0xbb];
            if send_tcp_message(message).is_err() {
                println!("shutting down monitor failed.")
            }
        }
        Command::Netflix => {
            open_site("https://netflix.com");
        }
        Command::VrtNuTvGuide => {
            open_site("https://www.vrt.be/vrtnu/tv-gids/");
        }
        Command::VrtNuLive => {
            open_site("https://www.vrt.be/vrtnu/livestream/");
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
