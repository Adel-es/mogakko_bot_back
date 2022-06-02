use rocket::{self, Build, State};
use rocket::http::{Status, ContentType};
use rocket::response::Redirect; 
use serde::Serialize;

use std::net::{TcpStream};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex, TryLockError, PoisonError}; 


use rocket_session_store::Session; 
pub mod model;
pub mod new; 
use super::connection; 
use super::RedisStruct;
use super::session::session::{self, SessionData};   


#[get("/")]
async fn read(session: Session<'_, SessionData> ) -> (Status, (ContentType, &'static str)) {
    (Status::ImATeapot, (ContentType::JSON, "{ \"data\": \" user is log out \" }"))
}

#[get("/state")]
async fn printstate(session: Session<'_, SessionData> ) -> (Status, (ContentType, String)) {
    let button1 = "<input type=\"button\" onclick=\"location.href='/users/sample_login';\" value=\"Login\"/>"; 
    let button2 = "<input type=\"button\" onclick=\"location.href='/users/sample_logout';\" value=\"Logout\"/>"; 
    

    let content : String = match session::get_session_login(&session).await {
        true => String::from("LOGIN SUCCESS"), 
        false => String::from("YOU ARE NOT IN")
    }; 
    (Status::Ok, (ContentType::HTML, format!("{}{}{}", content, button1, button2))) 
}

#[get("/sample_login")]
async fn sample_login(session: Session<'_, SessionData> ) -> Redirect {
    session::set_session_login(&session, true).await.unwrap(); 
    Redirect::temporary("/users/state")
}

#[get("/sample_logout")]
async fn sample_logout(session: Session<'_, SessionData> ) -> Redirect {
    session::set_session_login(&session, false).await.unwrap(); 
    Redirect::temporary("/users/state")
}


pub fn mount(rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    let rrocket = rocket.mount("/users", routes![read, printstate, sample_login, sample_logout]); 
    new::mount(rrocket)
}