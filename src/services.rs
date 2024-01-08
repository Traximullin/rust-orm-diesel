use actix_web::{
    get, post,
    web::{Data, Json, Path},
    Responder, HttpServer, HttpResponse
};
use serde::Deserialize;
use crate::{
    messages::{FetchUser},
    AppState, DbActor
};
use actix::Addr;

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub title: String,
    pub content: String,
}

#[get("/users")]
pub async fn fetch_users(state: Data<AppState>) -> impl Responder {
    let db: Addr<DbActor> = state.as_ref().db.clone();

    match db.send(FetchUser).await {
        Ok(Ok(info)) => HttpResponse::Ok().json(info),
        Ok(Err(_)) => HttpResponse::NotFound().json("No user found"),
        _ => HttpResponse::InternalServerError().json("Unable to retrieve users"),
    }
}

// #[get("/users/{id}/articles")]
// pub async fn fetch_users_articles(path: Path<i32>) -> impl Responder {
//     "test".to_string();
// }

// #[post("/users/{id}/articles")]
// pub async fn create_user(path: Path<i32>, body: Json<CreateUserBody>) -> impl Responder {
//     "test".to_string();
// }