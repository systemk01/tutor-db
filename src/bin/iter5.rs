use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use errors::EzyTutorError;
use sqlx::postgres::PgPool;
use std::env;
use std::io;
use std::sync::Mutex;

#[path = "../iter5/dbaccess/mod.rs"]
mod dbaccess;
#[path = "../iter5/errors.rs"]
mod errors;
#[path = "../iter5/handlers/mod.rs"]
mod handlers;
#[path = "../iter5/models/mod.rs"]
mod models;
#[path = "../iter5/routes.rs"]
mod routes;
#[path = "../iter5/state.rs"]
mod state;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::connect(&database_url).await.unwrap();
    //Construct AppState
    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good, you already asked me".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });
    //Construct App and configure routes
    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .app_data(web::JsonConfig::default().error_handler(|_err, _req| {
                EzyTutorError::InvalidInput("please provide valid JSON input".to_string()).into()
            }))
            .configure(general_routes)
            .configure(course_routes)
            .configure(tutor_routes)
    };
    //start HTTP server
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    println!("listening on {}", &host_port);
    HttpServer::new(app).bind(&host_port)?.run().await
    
}
