use crate::state::AppState;
use actix_web::{web,HttpResponse};

use crate::models::course::{CreateCourse, UpdateCourse};
use crate::dbaccess::course::*;
use crate::errors::MyError;

pub async fn post_new_course(app_state: web::Data<AppState>, new_course: web::Json<CreateCourse>) -> Result<HttpResponse,MyError>  {
	post_new_course_db(&app_state.db, new_course.try_into()?).await
	.map(|courses|HttpResponse::Ok().json(courses))
}

pub async fn get_courses_for_teacher(app_state: web::Data<AppState>, params: web::Path<i32>) -> Result<HttpResponse,MyError> {
	let teacher_id = params.into_inner();
	get_courses_for_teacher_db(&app_state.db, teacher_id)
	.await
	.map(|courses|HttpResponse::Ok().json(courses))
}

pub async fn get_course_detail(app_state: web::Data<AppState>, params: web::Path<(i32,i32)>) -> Result<HttpResponse,MyError> {
	let (teacher_id, course_id) = params.into_inner();
	get_course_detail_db(&app_state.db, teacher_id, course_id).await
	.map(|course| HttpResponse::Ok().json(course))
}

pub async fn delete_course(app_state: web::Data<AppState>, params: web::Path<(i32,i32)>) -> Result<HttpResponse,MyError> {
	let (teacher_id, course_id) = params.into_inner();
	delete_course_db(&app_state.db, teacher_id, course_id).await
	.map(|course| HttpResponse::Ok().json(course))
}

pub async fn update_course_details(app_state: web::Data<AppState>, update_course: web::Json<UpdateCourse>, params: web::Path<(i32,i32)>) -> Result<HttpResponse,MyError> {
	let (teacher_id, course_id) = params.into_inner();
	update_course_details_db(&app_state.db, teacher_id, course_id, update_course.into()).await
	.map(|course| HttpResponse::Ok().json(course))
}

#[cfg(test)]
mod tests {
	use super::*;
	use actix_web::http::StatusCode;
	use actix_web::ResponseError;
	use std::sync::Mutex;
	use dotenv::dotenv;
	use sqlx::postgres::PgPoolOptions;
	use std::env;
	//#[ignore]
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

		let course = web::Json(CreateCourse{
			teacher_id: 1,
			name: "Test course".into(),
			description: Some("This is a course".into()),
			format: None,
			structure: None,
			duration: None,
			price: None,
			language: Some("English".into()),
			level: Some("Beginner".into()),
		});
		let resp = post_new_course(app_state, course).await.unwrap();
		assert_eq!(resp.status(), StatusCode::OK);
	}
	#[actix_rt::test]
	async fn get_all_courses_success(){
		dotenv().ok();
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

		let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

		let app_state: web::Data<AppState> = web::Data::new(AppState { 
			health_check_response: "".to_string(),
			visit_count: Mutex::new(0),
			//course: Mutex::new(Vec::new()) 
			db: db_pool,
		});

		let teacher_id : web::Path<i32> = web::Path::from(1);
		let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap();
		assert_eq!(resp.status(), StatusCode::OK);
	}
	#[actix_rt::test]
	async fn get_one_course_success(){
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

	#[actix_rt::test]
	async fn get_one_course_failure(){
		dotenv().ok();
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

		let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

		let app_state: web::Data<AppState> = web::Data::new(AppState { 
			health_check_response: "".to_string(),
			visit_count: Mutex::new(0),
			//course: Mutex::new(Vec::new()) 
			db: db_pool,
		});

		let params : web::Path<(i32,i32)> = web::Path::from((1,100));
		let resp = get_course_detail(app_state, params).await.unwrap();
		assert_eq!(resp.status(), StatusCode::OK);
	}

	#[actix_rt::test]
	async fn delete_course_success(){
		dotenv().ok();
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

		let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

		let app_state: web::Data<AppState> = web::Data::new(AppState { 
			health_check_response: "".to_string(),
			visit_count: Mutex::new(0),
			//course: Mutex::new(Vec::new()) 
			db: db_pool,
		});

		let params : web::Path<(i32,i32)> = web::Path::from((1,3));
		let resp = delete_course(app_state, params).await.unwrap();
		assert_eq!(resp.status(), StatusCode::OK);
	}
	// #[ignore]
	#[actix_rt::test]
	async fn delete_course_failure(){
		dotenv().ok();
		let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set.");

		let db_pool = PgPoolOptions::new().connect(&database_url).await.unwrap();

		let app_state: web::Data<AppState> = web::Data::new(AppState { 
			health_check_response: "".to_string(),
			visit_count: Mutex::new(0),
			//course: Mutex::new(Vec::new()) 
			db: db_pool,
		});

		let params : web::Path<(i32,i32)> = web::Path::from((1,101));
		let resp = delete_course(app_state, params).await;
		match resp {
			Ok(_) => println!("Something wrong"),
			Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
		}
	
	}
}

/*
curl -X POST localhost:8080/courses/ -H "Content-Type: application/json" -d '{"teacher_id" : 1, "name": "First course"}'
curl -X POST localhost:8080/courses/ -H "Content-Type: application/json" -d '{"teacher_id" : 1, "name": "Second course"}'
curl -X POST localhost:8080/courses/ -H "Content-Type: application/json" -d '{"teacher_id" : 1, "name": "Third course"}'

curl localhost:8080/courses/1
*/