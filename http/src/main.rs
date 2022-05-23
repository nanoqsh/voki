use rocket::{
    fs::FileServer,
    get, launch, routes,
    serde::{json::Json, Serialize},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Message<'a> {
    text: &'a str,
}

#[get("/")]
fn index() -> Json<Message<'static>> {
    let message = Message { text: "hello" };
    message.into()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("./static"))
        .mount("/api", routes![index])
}
