use rocket::{self, Build};
use rocket::http::{Status, ContentType};

pub mod model;
use model::User;

#[get("/")]
fn read() -> (Status, (ContentType, &'static str)) {
    (Status::ImATeapot, (ContentType::JSON, "{ \"data\": \" user data\" }"))
}

pub fn mount(rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    rocket.mount("/users", routes![read])
}