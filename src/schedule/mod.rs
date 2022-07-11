pub mod model;

use chrono::NaiveDateTime;
use diesel::result::Error;
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
) -> Result<Json<Vec<DisplaySchedule>>, Status> {
    let fmt = "%Y-%m-%dT%H:%M:%S";
    let parse_from_str = NaiveDateTime::parse_from_str;
    
    let start_dt = match start {
        Some(date) => Some(parse_from_str(date, fmt)
            .map_err(|_| Status::BadRequest)?),
        None => None
    };
    let end_dt = match end {
        Some(date) => Some(parse_from_str(date, fmt)
            .map_err(|_| Status::BadRequest)?),
        None => None
    };

    Schedule::read_by_cond(start_dt, end_dt, user_id, &connection)
        .map(|schedule| Json(schedule))
        .map_err(|_| Status::NotFound)
}

#[get("/<id>")]
fn read_one(id: i32, connection: DbConn) -> Result<Json<DisplaySchedule>, Status> {
    // 로그인 확인하는 부분 추가하기

    Schedule::read_by_id(id, &connection)
        .map(|schedule| Json(schedule))
        .map_err(|_| Status::NotFound)
}

#[post("/", format = "application/json", data = "<new_schedule>")]
pub fn create(
    new_schedule: Json<NewSchedule>,
    connection: DbConn,
) -> Result<Json<Schedule>, Status> {
    NewSchedule::create(new_schedule.into_inner(), &connection)
        .map(|schedule| Json(schedule))
        .map_err(|error| error_status(error))
}

#[patch("/<id>", format = "application/json", data = "<schedule>")]
pub fn update(
    id: i32,
    schedule: Json<UpdateSchedule>,
    connection: DbConn,
) -> Result<Json<Schedule>, Status> {
    UpdateSchedule::update(id, schedule.into_inner(), &connection)
        .map(|schedule| Json(schedule))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, Status> {
    Schedule::delete(id, &connection)
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
    rocket.mount(
        "/schedules",
        routes![read, read_one, create, update, delete],
    )
}