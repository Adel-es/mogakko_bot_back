pub mod model;

use chrono::NaiveDateTime;
use diesel::result::{Error, DatabaseErrorKind};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{self, Build};

use crate::connection::DbConn;
use model::{Schedule, DisplaySchedule, NewSchedule, UpdateSchedule};

#[get("/?<start>&<end>&<user_id>")]
fn read(
    start: Option<&str>,
    end: Option<&str>,
    user_id: Option<i32>,
    connection: DbConn,
) -> Result<Json<Vec<DisplaySchedule>>, (Status, String)> {
    let fmt = "%Y-%m-%dT%H:%M:%S";
    let parse_from_str = NaiveDateTime::parse_from_str;
    
    let start_dt = match start {
        Some(date) => Some(parse_from_str(date, fmt)
            .map_err(|_| (Status::BadRequest, format!("Format of 'start' must be {}", fmt)))?),
        None => None
    };
    let end_dt = match end {
        Some(date) => Some(parse_from_str(date, fmt)
            .map_err(|_| (Status::BadRequest, format!("Format of 'end' must be {}", fmt)))?),
        None => None
    };

    Schedule::read_by_cond(start_dt, end_dt, user_id, &connection)
        .map(|schedule| Json(schedule))
        .map_err(|e| error_status(e))
}

#[get("/<id>")]
fn read_one(id: i32, connection: DbConn) -> Result<Json<DisplaySchedule>, (Status, String)> {
    // 로그인 확인하는 부분 추가하기

    Schedule::read_by_id(id, &connection)
        .map(|schedule| Json(schedule))
        .map_err(|e| error_status(e))
}

#[post("/", format = "application/json", data = "<new_schedule>")]
pub fn create(
    new_schedule: Json<NewSchedule>,
    connection: DbConn,
) -> Result<Json<Schedule>, (Status, String)> {
    NewSchedule::create(new_schedule.into_inner(), &connection)
        .map(|schedule| Json(schedule))
        .map_err(|e| error_status(e))
}

#[patch("/<id>", format = "application/json", data = "<schedule>")]
pub fn update(
    id: i32,
    schedule: Json<UpdateSchedule>,
    connection: DbConn,
) -> Result<Json<Schedule>, (Status, String)> {
    UpdateSchedule::update(id, schedule.into_inner(), &connection)
        .map(|schedule| Json(schedule))
        .map_err(|e| error_status(e))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, (Status, String)> {
    match Schedule::delete(id, &connection) {
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
    rocket.mount(
        "/schedules",
        routes![read, read_one, create, update, delete],
    )
}