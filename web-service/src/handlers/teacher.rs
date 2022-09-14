use crate::dbaccess::teacher::*;
use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, UpdateTeacher};
use crate::state::AppState;

use actix_web::{web, HttpResponse};

pub async fn get_all_techers(app_state: web::Data<AppState>) -> Result<HttpResponse, MyError> {
	get_all_teacher_db(&app_state.db).await.map(|teachers| HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_details(app_state: web::Data<AppState>, params: web::Path<i32>) -> Result<HttpResponse, MyError> {
	let teacher_id = params.into_inner();
	get_teacher_details_db(&app_state.db, teacher_id).await.map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn post_new_teacher(app_state: web::Data<AppState>, new_teacher: web::Json<CreateTeacher>) -> Result<HttpResponse, MyError> {
	post_new_teacher_db(&app_state.db, CreateTeacher::from(new_teacher)).await.map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_details(app_state: web::Data<AppState>, params: web::Path<i32>, update_teacher: web::Json<UpdateTeacher>) -> Result<HttpResponse, MyError> {
	let teacher_id = params.into_inner();
	update_teacher_details_db(&app_state.db, teacher_id, UpdateTeacher::from(update_teacher)).await.map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher(app_state: web::Data<AppState>, params: web::Path<i32>) ->Result<HttpResponse, MyError> {
	let teacher_id = params.into_inner();
	delete_teacher_db(&app_state.db, teacher_id).await.map(|teacher| HttpResponse::Ok().json(teacher))
}

#[cfg(test)]
mod tests{
	use super::*;
	use actix_web::http::StatusCode;
	use dotenv::dotenv;
	use sqlx::postgres::PgPoolOptions;
	use std::env;
	use std::sync::Mutex;

	#[actix_rt::test]
	async fn get_all_techers_success_test(){
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

		let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

		let app_state: web::Data<AppState> = web::Data::new(AppState { 
			health_check_response: "".to_string(),
			visit_count: Mutex::new(0),
			//course: Mutex::new(Vec::new()) 
			db: db_pool,
		});

		let resp = get_all_techers(app_state).await.unwrap();
		assert_eq!(resp.status(), StatusCode::OK);
	}

	#[actix_rt::test]
	async fn get_teacher_detail_success_test(){
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

		let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

		let app_state: web::Data<AppState> = web::Data::new(AppState { 
			health_check_response: "".to_string(),
			visit_count: Mutex::new(0),
			//course: Mutex::new(Vec::new()) 
			db: db_pool,
		});

		let params: web::Path<i32> = web::Path::from(1);
		let resp = get_teacher_details(app_state, params).await.unwrap();
		assert_eq!(resp.status(), StatusCode::OK);
	}

	#[actix_rt::test]
	async fn post_techer_success_test(){
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

		let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

		let app_state: web::Data<AppState> = web::Data::new(AppState { 
			health_check_response: "".to_string(),
			visit_count: Mutex::new(0),
			//course: Mutex::new(Vec::new()) 
			db: db_pool,
		});

		let new_teacher = CreateTeacher {
			name: "Third Teacher".into(),
			picture_url: "http://yangxu.pro".into(),
			profile: "A teacher in Machine Learning".into(),
		};
		
		let params = web::Json(new_teacher);
		let resp = post_new_teacher(app_state, params).await.unwrap();
		assert_eq!(resp.status(), StatusCode::OK);
	}

	#[actix_rt::test]
	async fn delete_techer_success_test(){
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

		let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

		let app_state: web::Data<AppState> = web::Data::new(AppState { 
			health_check_response: "".to_string(),
			visit_count: Mutex::new(0),
			//course: Mutex::new(Vec::new()) 
			db: db_pool,
		});
		
		//let params: web::Path<i32> = web::Path::from(1);
		let params = web::Path::from(1);
		let resp = delete_teacher(app_state, params).await.unwrap();
		assert_eq!(resp.status(), StatusCode::OK);
	}
}

/*
curl -X POST localhost:8080/teachers/ -H "Content-Type: application/json" -d '{"name": "zhangsan", "picture_url" : "http://", "profile": "aaaa"}'
curl -X POST localhost:8080/teachers/ -H "Content-Type: application/json" -d '{"name": "lisi", "picture_url" : "http://", "profile": "bbbb"}'
curl -X POST localhost:8080/teachers/ -H "Content-Type: application/json" -d '{"name": "wangwu", "picture_url" : "http://", "profile": "cccc"}'

curl localhost:8080/teachers/1
*/