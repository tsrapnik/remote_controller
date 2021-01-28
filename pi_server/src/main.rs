#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod command;

use command::Command;
use rocket::response;
use rocket_contrib::json::Json;
use std::{io, io::Write, net::TcpStream};
use wakey::WolPacket;

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

#[post("/", format = "application/json", data = "<command>")]
fn execute_command(command: Json<Command>) -> () {
    fn send_tcp_message(message: &[u8]) -> Result<(), io::Error> {
        let mut stream = TcpStream::connect("192.168.1.5:5000")?; //TODO: put address in config file.
        stream.write(message)?;
        Ok(())
    }

    fn wake_pc() {
        let wol_pc = WolPacket::from_string("4c:cc:6a:b0:b4:8c", ':'); //TODO: put address in config file.
        if wol_pc.send_magic().is_err() {
            println!("waking pc failed.")
        }
    }

    fn wake_monitor() {
        let wol_monitor = WolPacket::from_string("18:65:71:9f:a4:27", ':'); //TODO: put address in config file.
        if wol_monitor.send_magic().is_err() {
            println!("waking monitor failed.")
        }
    }

    fn send_to_pc(command: Command) -> () {
        // Keep sending until it succeeds or maximum tries reached.
        let max_tries = 10usize;
        for _ in 0..max_tries {
            let response = ureq::post("http://192.168.1.2/") //TODO: put address in config file.
                .send_json(serde_json::to_value(command.clone()).unwrap());

            match response {
                Ok(_) => {
                    return;
                }
                Err(error) => {
                    println!("{}", error);
                }
            }
        }
    
        // If previous loop never returned we failed to get a proper response.
        println!("Sending command failed.");
    }

    let command = command.into_inner();
    match command {
        Command::Brightness { value } => {
            let mut message = [
                0xa6u8, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x01, 0x32, value, 0x37, 0x32, 0x14, 0x32,
                0x32, 0x01, 0x00,
            ];
            let checksum = message[..message.len() - 1]
                .iter()
                .fold(0x00, |acc, x| acc ^ x);
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
        Command::Netflix | Command::VrtNuTvGuide | Command::VrtNuLive => {
            // These commands are all meant for the pc (with monitor turned on). So we check the pc
            // is awake and if so forward the command.

            // Always wake, if already awake the pc or monitor just ignores the wol packet.
            wake_monitor();
            wake_pc();

            send_to_pc(command);
        }
        // TODO => {
        //     // These commands are all meant for the pc (with monitor turned off). So we check the pc
        //     // is awake and if so forward the command.

        //     // Always wake, if already awake the pc just ignores the wol packet.
        //     wake_pc();

        //     send_to_pc(command);
        // }
        Command::Shutdown => {
            // This command is meant to turn of the pc. So just send without checking if it arrives,
            // since if the pc is already off, we won't get any response.

            send_to_pc(command);
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
