use crate::core::accounts::handlers::{
    create_account, get_account_by_cognito_sub, get_account_by_id,
};
use crate::core::students::handlers::{
    create_student_profile, get_student_profile_by_account_id, get_student_profile_by_id,
    update_student_profile,
};
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    log::info!("Configuring routes...");
    cfg.service(
        // ------------- configure routes ------------
        web::scope("/api/v1")
            .service(web::scope("/auth"))
            .service(
                web::scope("/accounts")
                    .service(create_account)
                    .service(get_account_by_id)
                    .service(get_account_by_cognito_sub),
            )
            .service(
                web::scope("/students")
                    .service(create_student_profile)
                    .service(get_student_profile_by_id)
                    .service(get_student_profile_by_account_id)
                    .service(update_student_profile),
            ),
    );
}
