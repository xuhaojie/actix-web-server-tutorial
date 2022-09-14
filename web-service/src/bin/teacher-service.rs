use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;
use crate::errors::MyError;

#[path = "../handlers/mod.rs"]
mod handlers;

#[path = "../models/mod.rs"]
mod models;

#[path = "../routers.rs"]
mod routers;

#[path = "../state.rs"]
mod state;

#[path = "../dbaccess/mod.rs"]
mod dbaccess;

#[path = "../errors.rs"]
mod errors;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
	dotenv().ok();

	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

	let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();
	let shared_data = web::Data::new(AppState {
		health_check_response: "I'm Ok.".to_string(),
		visit_count: Mutex::new(0),
		//course: Mutex::new(Vec::new()),
		db: db_pool,
	});

	let app = move || {
		App::new()
		.app_data(shared_data.clone())
		.app_data(web::JsonConfig::default().error_handler(|_err, _req| {
			MyError::InvalidInput("Please provide valid Json input".to_string()).into()
		}))
		.configure(general_routes)
		.configure(course_routes)
		.configure(teacher_routes)
	};
	let host_port = env::var("HOST_PORT").expect("HOST:PORT address is ");
	println!("Listening on: {}", &host_port);
	HttpServer::new(app).bind(&host_port)?.run().await
}
