use crate::configuration::state::AppState;
use crate::core::students::models::StudentProfile::StudentProfileNew;
use actix_web::web::Data;
use actix_web::{HttpResponse, get, post, put, web};
use uuid::Uuid;

#[post("")]
pub async fn create_student_profile(
    state: Data<AppState>,
    student_profile: web::Json<StudentProfileNew>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "student.create | handler | create_student_profile | started | Creating student profile..."
    );
    match state
        .student_service
        .create_student_profile(student_profile.into_inner())
        .await
    {
        Ok(student_profile) => {
            log::info!(
                "student.create | handler | create_student_profile | success | Student profile created successfully!"
            );
            Ok(HttpResponse::Ok().json(student_profile))
        }
        Err(e) => {
            log::error!(
                "student.create | handler | create_student_profile | failed | Student profile creation failed | {}",
                e
            );
            Ok(HttpResponse::from_error(e))
        }
    }
}

#[get("/{id}")]
pub async fn get_student_profile_by_id(
    state: Data<AppState>,
    id: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "student.get | handler | get_student_profile_by_id | started | Getting student profile by id..."
    );
    match state
        .student_service
        .get_student_profile_by_id(id.into_inner())
        .await
    {
        Ok(student_profile) => {
            log::info!(
                "student.get | handler | get_student_profile_by_id | success | Student profile retrieved successfully!"
            );
            Ok(HttpResponse::Ok().json(student_profile))
        }
        Err(e) => {
            log::error!(
                "student.get | handler | get_student_profile_by_id | failed | Student profile retrieval failed | {}",
                e
            );
            Ok(HttpResponse::from_error(e))
        }
    }
}

#[get("/account/{account_id}")]
pub async fn get_student_profile_by_account_id(
    state: Data<AppState>,
    account_id: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "student.get | handler | get_student_profile_by_account_id | started | Getting student profile by account_id..."
    );
    match state
        .student_service
        .get_student_profile_by_account_id(account_id.into_inner())
        .await
    {
        Ok(student_profile) => {
            log::info!(
                "student.get | handler | get_student_profile_by_account_id | success | Student profile retrieved successfully!"
            );
            Ok(HttpResponse::Ok().json(student_profile))
        }
        Err(e) => {
            log::error!(
                "student.get | handler | get_student_profile_by_account_id | failed | Student profile retrieval failed | {}",
                e
            );
            Ok(HttpResponse::from_error(e))
        }
    }
}

#[put("/{id}")]
pub async fn update_student_profile(
    state: Data<AppState>,
    id: web::Path<Uuid>,
    student_profile: web::Json<StudentProfileNew>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "student.update | handler | update_student_profile | started | Updating student profile..."
    );
    match state
        .student_service
        .update_student_profile(id.into_inner(), student_profile.into_inner())
        .await
    {
        Ok(student_profile) => {
            log::info!(
                "student.update | handler | update_student_profile | success | Student profile updated successfully!"
            );
            Ok(HttpResponse::Ok().json(student_profile))
        }
        Err(e) => {
            log::error!(
                "student.update | handler | update_student_profile | failed | Student profile update failed | {}",
                e
            );
            Ok(HttpResponse::from_error(e))
        }
    }
}
