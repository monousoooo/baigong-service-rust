use actix_web::{get, HttpResponse, post, Responder, web};
use crate::models::UserData;
use crate::persistence::create_user;

#[get("/")]
pub(crate) async fn index() -> impl Responder {
    String::new()
}

#[post("/user")]
pub(crate) async fn add_user(web::Json(user_data): web::Json<UserData>, data: web::Data<mysql::Pool>) -> actix_web::Result<impl Responder> {
    let name = user_data.name;

    web::block(move || create_user(&data, name)).await??;
    Ok(HttpResponse::NoContent())
}