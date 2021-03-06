extern crate diesel;

use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::controllers::index;
use crate::models::users::User;
use serde::{Deserialize, Serialize};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(index::index));
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    name: String,
}

pub async fn create(item: web::Json<UserData>) -> HttpResponse {
    let user = User::create(&(item.name));
    println!("{:?}",user);
    HttpResponse::Created().body("Inserting")

}