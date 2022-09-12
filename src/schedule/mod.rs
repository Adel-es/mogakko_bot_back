pub mod model;

use chrono::NaiveDateTime;
use diesel::result::Error;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{self, Build};

use crate::connection::DbConn;
use crate::error::err_to_response;
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
        .map_err(|e| err_to_response(e))
}

#[get("/<id>")]
fn read_one(id: i32, connection: DbConn) -> Result<Json<DisplaySchedule>, (Status, String)> {
    // 로그인 확인하는 부분 추가하기

    Schedule::read_by_id(id, &connection)
        .map(|schedule| Json(schedule))
        .map_err(|e| err_to_response(e))
}

#[post("/", format = "application/json", data = "<new_schedule>")]
pub fn create(
    new_schedule: Json<NewSchedule>,
    connection: DbConn,
) -> Result<Json<Schedule>, (Status, String)> {
    NewSchedule::create(new_schedule.into_inner(), &connection)
        .map(|schedule| Json(schedule))
        .map_err(|e| err_to_response(e))
}

#[patch("/<id>", format = "application/json", data = "<schedule>")]
pub fn update(
    id: i32,
    schedule: Json<UpdateSchedule>,
    connection: DbConn,
) -> Result<Json<Schedule>, (Status, String)> {
    UpdateSchedule::update(id, schedule.into_inner(), &connection)
        .map(|schedule| Json(schedule))
        .map_err(|e| err_to_response(e))
}

#[delete("/<id>")]
pub fn delete(id: i32, connection: DbConn) -> Result<Status, (Status, String)> {
    match Schedule::delete(id, &connection) {
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
    rocket.mount(
        "/schedules",
        routes![read, read_one, create, update, delete],
    )
}