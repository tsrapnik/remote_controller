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
        let result = process::Command::new("sudo")
            .arg("-u")
            .arg("tsrapnik")
            .arg("chromium")
            .arg(site)
            .spawn();

        if let Err(error) = result {
            println!("Failed to open \"{}\". error: \"{}\"", site, error);
        }
    }

    match command.into_inner() {
        Command::Shutdown => {
            if system_shutdown::shutdown().is_err() {
                println!("Shutting down failed.");
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
