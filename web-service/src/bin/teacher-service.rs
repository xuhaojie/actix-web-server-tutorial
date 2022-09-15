use actix_web::{http,web, App, HttpServer};
use std::io;
use std::sync::Mutex;
use dotenv::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;
use crate::errors::MyError;
use actix_cors::Cors;
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
		let cors = Cors::default()
		.allowed_origin("http://localhost:8080/")
		.allowed_origin_fn(|origin,_req_head| {
			origin.as_bytes().starts_with(b"http://localhost")
		})
		.allowed_methods(vec!["GET", "POST"])
		.allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
		.allowed_header(http::header::CONTENT_TYPE)
		.max_age(3600);

		App::new()
		.app_data(shared_data.clone())
		.app_data(web::JsonConfig::default().error_handler(|_err, _req| {
			MyError::InvalidInput("Please provide valid Json input".to_string()).into()
		}))
		.configure(general_routes)
		.configure(course_routes)
		.configure(teacher_routes)
		.wrap(cors)

	};
	let host_port = env::var("HOST_PORT").expect("HOST:PORT address is ");
	println!("Listening on: {}", &host_port);
	HttpServer::new(app).bind(&host_port)?.run().await
}
