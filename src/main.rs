#[macro_use] extern crate rocket;

#[get("/")]
async fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/hi", routes![index])
}
