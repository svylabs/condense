use actix_web::{App, HttpServer, web};
use condenser::{ckg, sign, signers, users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(ckg::init_routes)
            .configure(sign::init_routes)
            .configure(signers::init_routes)
            .configure(users::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}