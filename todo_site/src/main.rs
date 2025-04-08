#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Tervetuloa Rocket-sovellukseen!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
