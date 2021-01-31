use actix_web::{web, HttpRequest, HttpResponse, Responder};
use crate::models::users::User;

pub async fn index() -> impl Responder {
    let results = User::all();
    let mut res = format!("Displaying {} users\n\n", results.len());
    for user in results {
        let s = format!("id: {}, name: {}\n", user.id, user.name);
        res.push_str(&s);
    }

    HttpResponse::Ok().body(res)
}