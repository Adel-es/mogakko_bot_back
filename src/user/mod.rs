extern crate bcrypt;

use diesel::result::Error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{self, Build};
use bcrypt::{DEFAULT_COST, hash};
use crate::connection::DbConn;
use model::{NewUser, User, UserResponse, UpdateUser};

pub mod model;

#[post("/", format = "application/json", data = "<new_user>")]
pub fn create(
    new_user: Json<NewUser>,
    connection: DbConn,
) -> Result<Status, Status> {
    let mut user = new_user.into_inner();
    user.pw = hash(user.pw, DEFAULT_COST)
        .map_err(|_| Status::InternalServerError)?;
    NewUser::create(user, &connection)
        .map(|_| Status::Created)
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
fn read(id: i32, connection: DbConn) -> Result<Json<UserResponse>, Status> {
    // check mkko_login_session
    User::read(id, &connection)
        .map(|user| Json(user.to_response()))
        .map_err(|_| Status::NotFound)
}

#[patch("/<id>", format = "application/json", data = "<user>")]
pub fn update(
    id: i32,
    user: Json<UpdateUser>,
    connection: DbConn,
) -> Result<Json<UserResponse>, Status> {
    // check pw_double_check session
    let mut user = user.into_inner();
    if let Some(pw) = user.pw {
        user.pw = hash(pw, DEFAULT_COST)
            .map(|hashed| Some(hashed))
            .map_err(|_| Status::InternalServerError)?;
    }
    UpdateUser::update(id, user, &connection)
        .map(|user| Json(user.to_response()))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    // check mkko_login_session
    User::delete(id, &connection)
        .map(|_| Status::NoContent)
        .map_err(|error| error_status(error))
}

fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError,
    }
}

pub fn mount(rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    rocket.mount("/users", routes![create, read, update, delete])
}