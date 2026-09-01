use crate::configuration::state::AppState;
use crate::core::accounts::models::AccountNew::AccountNew;
use actix_web::web::Data;
use actix_web::{HttpResponse, get, post, web};
use uuid::Uuid;

#[post("")]
pub async fn create_account(
    state: Data<AppState>,
    new_account: web::Json<AccountNew>,
) -> actix_web::Result<HttpResponse> {
    log::info!("account.create | handler | create_account | started | Creating account...");
    match state
        .account_service
        .create_account(new_account.into_inner())
        .await
    {
        Ok(account) => {
            log::info!(
                "account.create | handler | create_account | success | Account created successfully!"
            );
            Ok(HttpResponse::Ok().json(account))
        }
        Err(e) => {
            log::error!(
                "account.create | handler | create_account | failed | Account creation failed | {}",
                e
            );
            Ok(HttpResponse::from_error(e))
        }
    }
}

#[get("/{id}")]
pub async fn get_account_by_id(
    state: Data<AppState>,
    id: web::Path<Uuid>,
) -> actix_web::Result<HttpResponse> {
    log::info!("account.get | handler | get_account_by_id | started | Getting account by id...");
    match state
        .account_service
        .get_account_by_id(id.into_inner())
        .await
    {
        Ok(account) => {
            log::info!(
                "account.get | handler | get_account_by_id | success | Account retrieved successfully!"
            );
            Ok(HttpResponse::Ok().json(account))
        }
        Err(e) => {
            log::error!(
                "account.get | handler | get_account_by_id | failed | Account retrieval failed | {}",
                e
            );
            Ok(HttpResponse::from_error(e))
        }
    }
}

#[get("/cognito/{cognito_sub}")]
pub async fn get_account_by_cognito_sub(
    state: Data<AppState>,
    cognito_sub: web::Path<String>,
) -> actix_web::Result<HttpResponse> {
    log::info!(
        "account.get | handler | get_account_by_cognito_sub | started | Getting account by cognito_sub..."
    );
    match state
        .account_service
        .get_account_by_cognito_sub(cognito_sub.into_inner())
        .await
    {
        Ok(account) => {
            log::info!(
                "account.get | handler | get_account_by_cognito_sub | success | Account retrieved successfully!"
            );
            Ok(HttpResponse::Ok().json(account))
        }
        Err(e) => {
            log::error!(
                "account.get | handler | get_account_by_cognito_sub | failed | Account retrieval failed | {}",
                e
            );
            Ok(HttpResponse::from_error(e))
        }
    }
}
