pub mod model;

use rocket::{self, Build};
use rocket::http::{Status};
use rocket::serde::json::Json;

use crate::connection::DbConn;
use model::{Schedule, NewSchedule};

#[get("/<sched_id>")]
fn read_one(sched_id: i32, connection: DbConn) -> Result<Json<Schedule>, Status> {
    // 로그인 확인하는 부분 추가하기

    Schedule::read(sched_id, &connection)
        .map(|schedule| Json(schedule))
        .map_err(|_| Status::NotFound)
}

pub fn mount(rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    rocket.mount("/schedules", routes![read_one])
}