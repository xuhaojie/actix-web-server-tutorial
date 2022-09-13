use super::state::AppState;
use actix_web::{web,HttpResponse};

use super::models::Course;
use super::db_access::*;
use super::errors::MyError;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
	let health_check_response = &app_state.health_check_response;
	let mut visit_count = app_state.visit_count.lock().unwrap();
	let response = format!("{} {} times", health_check_response, visit_count);
	*visit_count += 1;
	HttpResponse::Ok().json(&response)
}

pub async fn new_course(app_state: web::Data<AppState>, new_course: web::Json<Course>) -> Result<HttpResponse,MyError>  {
	post_new_course_db(&app_state.db, new_course.into()).await
	.map(|courses|HttpResponse::Ok().json(courses))
}

pub async fn get_courses_for_teacher(app_state: web::Data<AppState>,  params: web::Path<(i32,)>) -> Result<HttpResponse,MyError> {
	let teacher_id = params.0;
	get_courses_for_teacher_db(&app_state.db, teacher_id)
	.await
	.map(|courses|HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(app_state: web::Data<AppState>, params: web::Path<(i32,i32)>) -> Result<HttpResponse,MyError> {
	let teacher_id = params.0;
	let  course_id = params.1;
	get_course_detail_db(&app_state.db, teacher_id, course_id).await
	.map(|course| HttpResponse::Ok().json(course))
}
#[cfg(test)]
mod tests {
	use super::*;
	use actix_web::http::StatusCode;
	use std::sync::Mutex;
	use dotenv::dotenv;
	use sqlx::postgres::PgPoolOptions;
	use std::env;

	#[actix_rt::test]
	async fn post_course_test(){
		dotenv().ok();
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

		let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

		let app_state: web::Data<AppState> = web::Data::new(AppState { 
			health_check_response: "".to_string(),
			visit_count: Mutex::new(0),
			//course: Mutex::new(Vec::new()) 
			db: db_pool,
		});

		let course = web::Json(Course{
			teacher_id: 1,
			name: "Test course".into(),
			id: Some(3),
			time: None,
		});
		let resp = new_course(app_state, course).await.unwrap();
		assert_eq!(resp.status(), StatusCode::OK);
	}
	#[actix_rt::test]
	async fn get_all_courses(){
		dotenv().ok();
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

		let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

		let app_state: web::Data<AppState> = web::Data::new(AppState { 
			health_check_response: "".to_string(),
			visit_count: Mutex::new(0),
			//course: Mutex::new(Vec::new()) 
			db: db_pool,
		});

		let teacher_id : web::Path<(i32,)> = web::Path::from((1,));
		let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap();
		assert_eq!(resp.status(), StatusCode::OK);
	}
	#[actix_rt::test]
	async fn get_one_course(){
		dotenv().ok();
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

		let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

		let app_state: web::Data<AppState> = web::Data::new(AppState { 
			health_check_response: "".to_string(),
			visit_count: Mutex::new(0),
			//course: Mutex::new(Vec::new()) 
			db: db_pool,
		});

		let params : web::Path<(i32,i32)> = web::Path::from((1,1));
		let resp = get_course_detail(app_state, params).await.unwrap();
		assert_eq!(resp.status(), StatusCode::OK);
	}
}

/*
curl -X POST localhost:8080/courses/ -H "Content-Type: application/json" -d '{"teacher_id" : 1, "name": "First course"}'
curl -X POST localhost:8080/courses/ -H "Content-Type: application/json" -d '{"teacher_id" : 1, "name": "Second course"}'
curl -X POST localhost:8080/courses/ -H "Content-Type: application/json" -d '{"teacher_id" : 1, "name": "Third course"}'

curl localhost:8080/courses/1
*/