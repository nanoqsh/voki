use http::index;
use rocket::{fs::FileServer, launch, routes};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", FileServer::from("./static"))
        .mount("/api", routes![index])
}
