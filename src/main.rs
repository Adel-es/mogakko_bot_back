use rocket::serde::{Deserialize, Serialize, json};
use rocket::http::{Status, ContentType, Header};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Error; 
use tokio::sync::{RwLock}; 

mod route;
mod schedule;
mod user;
mod discord_bot; 
use std::thread;

#[macro_use] extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct MyObj {
    data: String,
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

#[rocket::main]
async fn rocket_server_start() {
    let mut rocket = rocket::build()
        .attach(CORS);
    rocket = route::mount(rocket);
    rocket.launch().await.unwrap(); 
}

// https://api.rocket.rs/v0.5-rc/rocket/config/struct.Config.html#method.figment
#[tokio::main]
async fn main() {
    /* tokio::spawn(move || {
        // rocket_server_start(); 
        discord_bot::bot::start_discord_bot().await; 
    }); */ 
    let handle = thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            let mut rocket = rocket::build()
            .attach(CORS);
            rocket = route::mount(rocket);
            // rocket.launch().await.unwrap(); 

        });
    });
    // tokio::task::spawn(rocket_server_start()); 
    /*    tokio::spawn(async move {
        rocket_server_start().await;
        // discord_bot::bot::start_discord_bot().await; 
    }); */  
    
    discord_bot::bot::start_discord_bot().await; 
    /* 
    let mut rocket = rocket::build()
        .attach(CORS);
    rocket = route::mount(rocket);
    rocket.launch().await.unwrap();  */ 
}