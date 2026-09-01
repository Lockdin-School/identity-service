use crate::core::accounts::models::Account::AccountStatus;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AccountNew {
    pub cognito_sub: String,
    pub email: String,
    pub email_verified: bool,
    pub status: AccountStatus,
}
