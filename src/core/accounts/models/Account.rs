use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "camelCase")]
#[sqlx(type_name = "account_status", rename_all = "lowercase")]
pub enum AccountStatus {
    Active,
    Inactive,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub cognito_sub: String,
    pub email: String,
    pub email_verified: bool,
    pub status: AccountStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
