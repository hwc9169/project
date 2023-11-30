#[macro_use]
extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn launch() -> _ {
    rocket::build().mount("/", routes![index])
}
