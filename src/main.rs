use rocket::serde::{Deserialize, Serialize, json};
use rocket::http::{Status, ContentType, Header};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::Error;

use rocket_session_store::{
	memory::MemoryStore,
	SessionStore,
	SessionResult,
	Session,
	CookieConfig,
};

//===== tcp streaming ======
//use tokio::sync::{RwLock};
use std::net::{TcpStream};
use std::io::{Read, Write};

//===== global varaible ===== 
use std::sync::{Arc, Mutex}; 

//========other std ==========
use std::time::Duration;
use std::thread;

mod route;
mod schedule;
mod user;
mod discord_bot; 
mod connection; 
mod session; 

use session::session::SessionData; 

#[macro_use] extern crate rocket;

pub struct RedisStruct {
    discord_bot_stream : Arc<Mutex<TcpStream>> 
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

    let stream =  connect_discord_bot().unwrap(); 
    let stream = Arc::new(Mutex::new(stream));
    let redis = RedisStruct{discord_bot_stream : stream}; 
    
    let memory_store:MemoryStore::<SessionData> = MemoryStore::default(); 
	let store: SessionStore<SessionData> = SessionStore {
		store: Box::new(memory_store),
		name: "token".into(),
		duration: Duration::from_secs(3600),
		cookie: CookieConfig::default(),
	};

    let mut rocket = rocket::build()
        .attach(CORS)
        .attach(store.fairing())
        .manage(redis); 
    rocket = route::mount(rocket);
    rocket.launch().await.unwrap(); 
}
