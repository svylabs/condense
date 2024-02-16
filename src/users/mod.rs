pub mod types;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::{connection, Connection, PgConnection, RunQueryDsl};

use crate::{state::app_state::AppState, users::types::NewUserInput};
use crate::schema::users;
use crate::models::{NewUser, User};

#[post("/new")]
async fn new_user(body: web::Json<NewUserInput>, state: web::Data<AppState>) -> impl Responder {
    let mut pool = state.pool.get().unwrap();
    let new_user = body.into_inner();
    let new_user = NewUser {
        username: new_user.username,
    };
    let inserted_user = diesel::insert_into(users::table)
        .values(&new_user)
        .get_result::<User>(&mut pool)
        .unwrap();
    HttpResponse::Ok().body(format!("New user created: {:?}", inserted_user))
}

#[get("/round2")]
async fn round2() -> impl Responder {
    HttpResponse::Ok().body("Round 2")
}

#[get("/round3")]
async fn round3() -> impl Responder {
    HttpResponse::Ok().body("Round 3")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(new_user)
            .service(round2)
            .service(round3)
    );
}