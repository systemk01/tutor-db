use crate::dbaccess::tutor::*;
use crate::errors::EzyTutorError;
use crate::models::tutor::{NewTutor, UpdateTutor};
use crate::state::AppState;
use actix_web::{web, HttpResponse};


pub async fn get_all_tutors(app_state: web::Data<AppState>) -> 
    Result<HttpResponse, EzyTutorError> {
    get_all_tutors_db(&app_state.db)
    .await
    .map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn get_tutor_details(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = path.into_inner();
    get_tutor_details_db(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn post_new_tutor(
    new_tutor: web::Json<NewTutor>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    post_new_tutor_db(&app_state.db, NewTutor::from(new_tutor))
    .await
    .map(|tutor|HttpResponse::Ok().json(tutor))
}

pub async fn update_tutor_details(
    app_state: web::Data<AppState>,
    path: web::Path<i32>,
    update_tutor: web::Json<UpdateTutor>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = path.into_inner();
    update_tutor_details_db(&app_state.db, tutor_id, UpdateTutor::from(update_tutor))
    .await
    .map(|tutor|HttpResponse::Ok().json(tutor))
}

pub async fn delete_tutor(app_state: web::Data<AppState>,
    path: web::Path<i32>,)
    -> Result<HttpResponse, EzyTutorError> {
        let tutor_id = path.into_inner();
        delete_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
    }


    #[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use actix_web::ResponseError;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_tutors_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file!");
        let pool = PgPool::connect(&database_url).await.unwrap();
        let app_state = web::Data::new(AppState{
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool
        });
        let resp = get_all_tutors(app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }



}