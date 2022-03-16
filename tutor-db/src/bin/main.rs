use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;
use std::sync::Mutex;
use tera::Tera;
use tutor_db::routes::*;
use tutor_db::state::AppState;

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good. You've already asked me ".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let app = move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static/**/*")).unwrap();
        App::new()
            .data(tera)
            .app_data(shared_data.clone())
            .configure(course_routes)
            .configure(tutor_routes)
            .configure(static_routes)
            .configure(general_routes)
    };

    let host_port = env::var("HOST_PORT").expect("HOST_PORT is not set in .env file");
    HttpServer::new(app).bind(&host_port)?.run().await
}
