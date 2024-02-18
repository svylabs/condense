use actix_web::{App, HttpServer, web};
use condenser::{ckg, sign, signers, users, state, roles};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let port: u16 = std::env::var("PORT")
        .unwrap_or("8080".to_string())
        .parse()
        .unwrap();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
                .max_size(10)
                .build(manager)
                .expect("Failed to create pool.");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(ckg::init_routes)
            .configure(sign::init_routes)
            .configure(signers::init_routes)
            .configure(users::init_routes)
            .configure(roles::init_routes)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}