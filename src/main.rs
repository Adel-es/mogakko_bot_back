#[macro_use] 
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate bcrypt;

use rocket::http::{Header};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use dotenv::dotenv;

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex}; 

mod connection;
mod schema;
mod route;
mod schedule;
mod user;
mod connection_bot; 
mod discord_bot; 

pub struct RedisStruct {
    discord_bot_stream : Option<Arc<Mutex<TcpStream>>> 
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn connect_discord_bot() -> Result<TcpStream, ()> {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            Ok(stream)
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
            Err(())
        }
    }
    // println!("Terminated.");
}

#[rocket::main]
async fn main() {
    dotenv().ok();

    let stream = if discord_bot::config::DISCORD_BOT_CONNECTION {
        Some(Arc::new(Mutex::new(connect_discord_bot().unwrap())))
    } else{
        None
    }; 

    // let stream = Arc::new(Mutex::new(stream));
    let redis = RedisStruct{discord_bot_stream : stream}; 

    let mut rocket = rocket::build()
        .attach(CORS)
        .manage(connection::init_pool())
        .manage(redis); 
        
    rocket = route::mount(rocket);
    rocket.launch().await.unwrap();
}