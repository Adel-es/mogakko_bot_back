extern crate bcrypt;

use diesel::result::{Error, DatabaseErrorKind};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{self, Build};
use bcrypt::{DEFAULT_COST, hash};
use crate::connection::DbConn;
use model::{NewUser, User, UserResponse, UpdateUser};
use format::{check_form, check_username, check_pw};

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
    NewUser::create(user, &connection)
        .map(|_| Status::Created)
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
fn read(id: i32, connection: DbConn) -> Result<Json<UserResponse>, (Status, String)> {
    // check mkko_login_session
    User::read(id, &connection)
        .map(|user| Json(user.to_response()))
        .map_err(|e| error_status(e))
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
        .map_err(|e| error_status(e))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, (Status, String)> {
    // check mkko_login_session
    match User::delete(id, &connection) {
        Ok(row_cnt) => {
            if row_cnt == 0 {
                Err(error_status(Error::NotFound))
            } else {
                Ok(Status::Ok)
            }
        },
        Err(e) => Err(error_status(e))
    }
}

fn error_status(error: Error) -> (Status, String) {
    let status = match error {
        Error::NotFound => Status::NotFound,
        Error::QueryBuilderError(_) => Status::UnprocessableEntity,
        Error::DatabaseError(kind, ref info) => {
            match kind {
                DatabaseErrorKind::UniqueViolation => Status::Conflict,
                DatabaseErrorKind::ForeignKeyViolation => Status::UnprocessableEntity,
                _ => {
                    match info.constraint_name() {
                        Some(_) => Status::UnprocessableEntity,
                        None => Status::InternalServerError
                    }
                }
            }
        }
        _ => Status::InternalServerError,
    };
    
    (status, format!("{}: {}", status.code.to_string(), error.to_string()))
}

pub fn mount(rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    rocket.mount("/users", routes![create, read, update, delete])
}