use rocket::{self, Build};
use rocket::http::{Status, ContentType};

pub mod model;
use model::Schedule;

#[get("/<id>")]
fn read(id: i32) -> (Status, (ContentType, &'static str)) {
    (Status::ImATeapot, (ContentType::JSON, "{ \"data\": \" schedule data\" }"))
}

pub fn mount(rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    rocket.mount("/schedules", routes![read])
}