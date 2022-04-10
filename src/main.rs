use rocket::serde::{Deserialize, Serialize, json};
use rocket::http::{Status, ContentType, Header};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

#[macro_use] extern crate rocket;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct MyObj {
    data: String,
}

#[get("/")]
fn index() -> (Status, (ContentType, &'static str)) {
    (Status::ImATeapot, (ContentType::JSON, "{ \"data\": \" hello world from rust\" }"))
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Attaching CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS).mount("/", routes![index])
}