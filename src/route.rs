use rocket::{self, Build};

use super::schedule;
use super::user;

pub fn mount(rocket: rocket::Rocket<Build>) -> rocket::Rocket<Build> {
    let rocket = schedule::mount(rocket);
    user::mount(rocket)
}