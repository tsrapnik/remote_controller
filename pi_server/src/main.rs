#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod command;

use command::Command;
use futures::executor::block_on;
use hyper::{Body, Client, Method, Request};
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
        let mut stream = TcpStream::connect("192.168.1.5:5000")?;
        stream.write(message)?;
        Ok(())
    }

    fn wake_pc() {
        let wol_pc = WolPacket::from_string("4c:cc:6a:b0:b4:8c", ':');
        if wol_pc.send_magic().is_err() {
            println!("waking pc failed.")
        }
    }

    fn wake_monitor() {
        let wol_monitor = WolPacket::from_string("18:65:71:9f:a4:27", ':');
        if wol_monitor.send_magic().is_err() {
            println!("waking monitor failed.")
        }
    }

    async fn send_to_pc(command: Command) -> () {
        // Keep sending until it succeeds or maximum tries reached.
        let max_tries = 10usize;
        for _ in 0..max_tries {
            let command = serde_json::to_string(&command);
            if command.is_err() {
                break;
            }
            let command = command.unwrap();
            let request = Request::builder()
                .method(Method::POST)
                .uri("http://192.168.1.25") //TODO
                .header("content-type", "application/json")
                .body(Body::from(command));
            if request.is_err() {
                break;
            }
            let request = request.unwrap();
            let client = Client::new();
            let response = client.request(request).await;
            if response.is_ok() {
                return;
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
                0x32, 0x01, 0xea, 0x00,
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

            let future = send_to_pc(command);
            block_on(future);
        }
        // TODO => {
        //     // These commands are all meant for the pc (with monitor turned off). So we check the pc
        //     // is awake and if so forward the command.

        //     // Always wake, if already awake the pc just ignores the wol packet.
        //     wake_pc();

        //     let future = send_to_pc(command);
        //     block_on(future);
        // }
        Command::Shutdown => {
            // This command is meant to turn of the pc. So just send without checking if it arrives,
            // since if the pc is already off, we won't get any response.

            let future = send_to_pc(command);
            block_on(future);
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
