pub mod types;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

use crate::{models::roles::{NewRole, Role}, Pool};
use diesel::{prelude::*};

#[post("/add")]
async fn add_roles(body: web::Json<types::NewRolesInput>, db: web::Data<Pool>) -> impl Responder {
    let new_roles_input = body.into_inner();
    let result = add_new_roles(new_roles_input.roles, db);
    match result {
        Ok(total_inserted) => HttpResponse::Ok().body(format!("New roles created: {:?}", total_inserted)),
        Err(e) => HttpResponse::BadRequest().body(format!("Error creating roles: {:?}", e))
    }
}

fn add_new_roles(roles: Vec<NewRole>, db: web::Data<Pool>) -> Result<usize, diesel::result::Error> {
    let mut connection = db.get().unwrap();
    diesel::insert_into(crate::schema::roles::table)
            .values(&roles)
            .execute(&mut connection)
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/roles")
            .service(add_roles)
    );
}