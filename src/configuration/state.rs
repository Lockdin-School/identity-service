use crate::core::accounts::repository::AccountRepositoryImpl::PostgresAccountRepository;
use crate::core::accounts::service::AccountService::AccountService;
use crate::core::sessions::service::SessionsService::SessionService;
use crate::core::students::repository::StudentRepositoryImpl::PostgresStudentRepository;
use crate::core::students::service::StudentService::StudentService;
use crate::infrastructure::db::database::{init_postgres, run_migrations};
use actix_web::web::Data;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub account_service: Data<AccountService>,
    pub student_service: Data<StudentService>,
    pub session_service: Data<SessionService>,
}

pub fn app_state(pool: sqlx::PgPool) -> AppState {
    AppState {
        account_service: Data::new(AccountService::new(Arc::new(
            PostgresAccountRepository::new(pool.clone()),
        ))),
        student_service: Data::new(StudentService::new(Arc::new(
            PostgresStudentRepository::new(pool.clone()),
        ))),
        session_service: Data::new(SessionService {}),
    }
}

pub async fn init_state() -> AppState {
    log::info!("Initializing state...");
    let pg_pool = init_postgres().await;
    // let redis = init_redis().await.expect("Failed to initialize redis");
    run_migrations(&pg_pool).await;
    app_state(pg_pool)
}
