use actix_web::{App, HttpServer, web};
use condenser::{ckg, sign, signers, users, state};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap();
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(state::app_state::AppState::new()))
            .configure(ckg::init_routes)
            .configure(sign::init_routes)
            .configure(signers::init_routes)
            .configure(users::init_routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}