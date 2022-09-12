use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../routers.rs"]
mod routers;
#[path = "../models.rs"]
mod models;
#[path = "../state.rs"]
mod state;

use routers::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {
	let shared_data = web::Data::new(AppState {
		health_check_response: "I'm Ok.".to_string(),
		visit_count: Mutex::new(0),
		course: Mutex::new(Vec::new()),
	});

	let app = move || {
		App::new()
		.app_data(shared_data.clone())
		.configure(general_routes)
		.configure(course_routes)
	};

	HttpServer::new(app).bind("127.0.0.1:8080")?.run().await
}