use rocket::{self, Build};
use rocket::http::{Status, ContentType};

pub mod model;
use super::discord_bot::{auth, config}; 
use model::User;
use crate::route;

#[get("/")]
fn read() -> (Status, (ContentType, &'static str)) {
    (Status::ImATeapot, (ContentType::JSON, "{ \"data\": \" user data\" }"))
}

#[get("/discord-id-verification/<discord_id>")]
async fn send_discord_dm(discord_id : &str) -> Status {
    print!("{}", discord_id);
    
    let guild_id : u64 = config::GUILD_ID.parse::<u64>().unwrap();  
    let discord_user_id : u64 = discord_id.parse::<u64>().unwrap(); 
    let user_exist_check =  auth::is_guild_user(discord_user_id, guild_id).await;  
    if user_exist_check {
        print!("exist!"); 
    } 
    Status::Ok
} 

pub fn mount(rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    let rocket = rocket.mount("/users", routes![read]); 
    rocket.mount("/users/new", routes![send_discord_dm])
}