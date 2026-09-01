use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Type)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(type_name = "curriculum_enum", rename_all = "UPPERCASE")]
pub enum Curriculum {
    CAPS,
    IEB,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct StudentProfile {
    pub id: Uuid,
    pub account_id: Uuid,

    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,

    pub grade: i32,
    pub curriculum: Curriculum,
    pub school_name: Option<String>,
    pub province: Option<String>,

    pub onboarding_completed: bool,

    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudentProfileNew {
    pub account_id: Uuid,

    pub first_name: String,
    pub last_name: String,
    pub avatar_url: Option<String>,

    pub grade: i32,
    pub curriculum: Curriculum,
    pub school_name: Option<String>,
    pub province: Option<String>,

    pub onboarding_completed: bool,
}
