use http::index;
use rocket::{fs::FileServer, launch, routes};
use std::process::Command;

#[launch]
async fn rocket() -> _ {
    Command::new("./server").spawn().expect("run server");

    rocket::build()
        .mount("/", FileServer::from("./static"))
        .mount("/api", routes![index])
}
