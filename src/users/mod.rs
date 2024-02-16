pub mod types;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::prelude::*;
use diesel::{connection, Connection, PgConnection, RunQueryDsl};

use crate::{state::app_state::AppState, users::types::NewUserInput};
use crate::schema::{users, roles};
use crate::models::{NewUser, NewUserRole, Role, User};

#[post("/new")]
async fn new_user(body: web::Json<NewUserInput>, state: web::Data<AppState>) -> impl Responder {
    let mut pool = state.pool.get().unwrap();
    let new_user_input = body.into_inner();
    let new_user = NewUser {
        username: new_user_input.username,
    };
    let result = pool.transaction::<User, diesel::result::Error, _>(|pool| {
        let inserted_user = diesel::insert_into(users::table)
            .values(&new_user)
            .get_result::<User>(pool)
            .unwrap();
        let all_roles = new_user_input.roles.unwrap();
        let total_roles = all_roles.len();
        let user_roles = roles::table
            .filter(roles::name.eq_any(all_roles))
            .load::<Role>(pool)
            .expect("Error loading roles");
        let user_roles_to_insert = NewUserRole::new(inserted_user.id, &user_roles);
        diesel::insert_into(crate::schema::user_roles::table)
            .values(&user_roles_to_insert)
            .execute(pool)
            .expect("Error inserting user roles");
        match total_roles != user_roles.len() {
            true =>  Err(diesel::result::Error::NotFound),
            false => Ok(inserted_user)
        }
    });
    match result {
        Ok(inserted_user) => HttpResponse::Ok().body(format!("New user created: {:?}", inserted_user)),
        Err(e) => HttpResponse::BadRequest().body(format!("Error creating user: {:?}", e))
    }
}

#[get("/list")]
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