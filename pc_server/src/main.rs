#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[path = "../../pi_server/src/command.rs"] // Use same definition as in pi_server.
mod command;

use command::Command;
use rocket_contrib::json::Json;
use std::process;
use system_shutdown;

#[post("/", format = "application/json", data = "<command>")]
fn execute_command(command: Json<Command>) -> () {
    fn open_site(site: &str) {
        let result = process::Command::new("google-chrome")
            .arg(site)
            .spawn();

        if let Err(error) = result {
            println!("Failed to open \"{}\". error: \"{}\"", site, error);
        }
    }

    fn set_volume(volume: u8) -> () {
        let result = process::Command::new("amixer")
            .arg("-D")
            .arg("pulse")
            .arg("sset")
            .arg("Master")
            .arg(format!("{}%", volume))
            .spawn();

        if let Err(error) = result {
            println!("Failed to change volume. error: \"{}\"", error);
        }
    }

    fn open_spotify() {
        let result = process::Command::new("spotify")
            .spawn();

        if let Err(error) = result {
            println!("Failed to open spotify. error: \"{}\"", error);
        }
    }

    match command.into_inner() {
        Command::Shutdown => {
            if system_shutdown::shutdown().is_err() {
                println!("Shutting down failed.");
            }
        }
        Command::Volume { value } => {
            set_volume(value);
        }
        Command::Netflix => {
            open_site("https://netflix.com");
        }
        Command::VrtMax => {
            open_site("https://www.vrt.be/vrtnu/livestream/");
        }
        Command::Spotify => {
            open_spotify();
        }
        _ => {
            println!("Not a pc command.")
        }
    }
}

fn main() {
    rocket::ignite()
        .mount("/", routes![execute_command])
        .launch();
}
