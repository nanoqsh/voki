use rocket::{
    get,
    serde::{json::Json, Serialize},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Message<'a> {
    text: &'a str,
}

#[get("/")]
pub async fn index() -> Json<Message<'static>> {
    let message = Message { text: "hello" };
    message.into()
}
