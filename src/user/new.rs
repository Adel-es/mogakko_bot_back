
use rocket::{self, Build, State, form};
use rocket::http::{Status, ContentType};
use serde::Serialize;
use rocket_session_store::Session;
use rand; 

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex, TryLockError, PoisonError}; 

use super::connection; 
use super::RedisStruct; 
use super::super::discord_bot::config; 
use super::super::session::session::{self, SessionData};

fn create_verify_code() -> u64 {
    rand::random::<u64>() % 1000000
}

#[get("/discord-id-verification/<discord_id>")]
async fn send_discord_dm(session: Session<'_, SessionData>, redis : &State<RedisStruct>, discord_id : &str) -> Status {

    // let guild_id : u64 = config::GUILD_ID.parse::<u64>().unwrap();  
    let discord_user_id : u64 = discord_id.parse::<u64>().unwrap(); 
    let stream = Arc::clone(&redis.discord_bot_stream);

    let code = create_verify_code(); 
    session::set_session_auth_code(&session, code).await.unwrap(); 
    //TODO! Fill discord id 
    let msg = connection::msg_struct::Message::SendAuthCode{discord_id : 538799426479849472, code : format!("{:06}", code)}; 
    let serialized_msg = serde_json::to_string(&msg).unwrap();

    loop{
        let lock = stream.try_lock(); 
        match lock {
            Err(TryLockError::Poisoned(_)) => { panic!() }, 
            Err(TryLockError::WouldBlock) => {continue},
            Ok(mut mutex) =>{
                println!("[send] discord bot msg"); 
                if let Err(e) = mutex.write(serialized_msg.as_bytes()){
                    println!("{:?}", e); 
                }
                break; 
            }
        }
    }
    Status::Ok
} 


#[get("/discord-id-verification-code/<code>")]
async fn verify_discord_code(session: Session<'_, SessionData>, redis : &State<RedisStruct>, code : &str) -> Status {
    let res_code = code.parse::<u64>(); 
    
    let session_code = session::get_session_auth_code(&session).await;  
    match res_code {
        Ok(code) => {
                if code == session_code {
                    Status::Ok 
                }
                else {
                    Status::Unauthorized 
                }
            }, 
        Err(_) => Status::UnprocessableEntity 
    }
}

pub fn mount(rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    let rocket = rocket.mount("/users/new", routes![send_discord_dm, verify_discord_code]); 
    rocket
}