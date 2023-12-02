use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub struct Signer {
    pub id: u64
}

#[get("/round1")]
async fn round1() -> impl Responder {
    HttpResponse::Ok().body("Round 1")
}

#[get("/round2")]
async fn round2() -> impl Responder {
    HttpResponse::Ok().body("Round 2")
}

#[get("/round3")]
async fn round3() -> impl Responder {
    HttpResponse::Ok().body("Round 3")
}

#[get("/new")]
async fn new_signer() -> impl Responder {
    HttpResponse::Ok().body("New Signer")
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/signers")
            .service(round1)
            .service(round2)
            .service(round3)
    );
}