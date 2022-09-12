extern crate bcrypt;

use diesel::result::Error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{self, Build};
use bcrypt::{DEFAULT_COST, hash};
use chrono::{Local, NaiveDateTime};
use crate::error::err_to_response;
use crate::connection::DbConn;
use model::{NewUser, User, UserResponse, UpdateUser};
use format::{check_form, check_username, check_pw};

pub mod sample; 
pub mod new; 
pub mod model;
pub mod format;

#[post("/", format = "application/json", data = "<new_user>")]
pub fn create(
    new_user: Json<NewUser>,
    connection: DbConn,
) -> Result<Status, (Status, String)> {
    let mut user = new_user.into_inner();
    
    check_form(&user).map_err(|e| (Status::BadRequest, e.to_string()))?;
    
    user.pw = hash(user.pw, DEFAULT_COST)
        .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    
    let now = Local::now().to_string();
    user.created = Some(NaiveDateTime::parse_from_str(&now, "%Y-%m-%d %H:%M:%S%.f %z")
        .map_err(|e| (Status::InternalServerError, e.to_string()))?);

    NewUser::create(user, &connection)
        .map(|_| Status::Created)
        .map_err(|error| err_to_response(error))
}

#[get("/<id>")]
fn read(id: i32, connection: DbConn) -> Result<Json<UserResponse>, (Status, String)> {
    // check mkko_login_session
    User::read(id, &connection)
        .map(|user| Json(user.to_response()))
        .map_err(|e| err_to_response(e))
}

#[patch("/<id>", format = "application/json", data = "<user>")]
pub fn update(
    id: i32,
    user: Json<UpdateUser>,
    connection: DbConn,
) -> Result<Json<UserResponse>, (Status, String)> {
    // check pw_double_check session
    let mut user = user.into_inner();
    
    if let Some(username) = user.username {
        check_username(&username).map_err(|e| (Status::BadRequest, e.to_string()))?;
    }
    if let Some(pw) = user.pw {
        check_pw(&pw).map_err(|e| (Status::BadRequest, e.to_string()))?;
        user.pw = hash(pw, DEFAULT_COST)
            .map(|hashed| Some(hashed))
            .map_err(|e| (Status::InternalServerError, e.to_string()))?;
    }
    UpdateUser::update(id, user, &connection)
        .map(|user| Json(user.to_response()))
        .map_err(|e| err_to_response(e))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, (Status, String)> {
    // check mkko_login_session
    match User::delete(id, &connection) {
        Ok(row_cnt) => {
            if row_cnt == 0 {
                Err(err_to_response(Error::NotFound))
            } else {
                Ok(Status::Ok)
            }
        },
        Err(e) => Err(err_to_response(e))
    }
}

pub fn mount(rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    let mut rrocket = rocket.mount("/users", routes![create, read, update, delete]); 
    // rrocket = sample::mount(rrocket); 
    new::mount(rrocket) 
}