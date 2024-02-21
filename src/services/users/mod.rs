pub mod types;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::{prelude::*};
use diesel::{Connection, PgConnection, RunQueryDsl};

use crate::services::users::types::NewUserInput;
use crate::{Pool};
use crate::schema::{users, roles};
use crate::models::{users::{NewUser, User}, user_roles::NewUserRole, roles::Role};

#[post("/new")]
async fn new_user(body: web::Json<NewUserInput>, db: web::Data<Pool>, user: AuthedUser) -> impl Responder {
    let new_user_input = body.into_inner();
    let new_user = NewUser {
        username: new_user_input.username,
    };
    let result = add_new_user(new_user, new_user_input.roles, db);
    match result {
        Ok(inserted_user) => HttpResponse::Ok().body(format!("New user created: {:?}", inserted_user)),
        Err(e) => HttpResponse::BadRequest().body(format!("Error creating user: {:?}", e))
    }
}

fn add_new_user(user: NewUser, roles_user: Option<Vec<String>>, db: web::Data<Pool>) -> Result<User, diesel::result::Error> {
    let mut connection = db.get().unwrap();
    connection.transaction::<User, diesel::result::Error, _>(|conn| {
        let inserted_user = diesel::insert_into(users::table)
            .values(&user)
            .get_result::<User>(conn)
            .unwrap();
        let all_roles = roles_user.unwrap();
        let total_roles = all_roles.len();
        let user_roles = roles::table
            .filter(roles::name.eq_any(all_roles))
            .load::<Role>(conn)
            .expect("Error loading roles");
        let user_roles_to_insert = NewUserRole::new(inserted_user.id, &user_roles);
        diesel::insert_into(crate::schema::user_roles::table)
            .values(&user_roles_to_insert)
            .execute(conn)
            .expect("Error inserting user roles");
        match total_roles != user_roles.len() {
            true =>  Err(diesel::result::Error::NotFound),
            false => Ok(inserted_user)
        }
    })
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