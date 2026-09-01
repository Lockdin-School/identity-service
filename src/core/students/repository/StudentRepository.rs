use crate::core::students::models::StudentProfile::{StudentProfile, StudentProfileNew};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait StudentRepository: Send + Sync {
    async fn create_student_profile(
        &self,
        student_profile: StudentProfileNew,
    ) -> Result<StudentProfile, sqlx::Error>;

    async fn get_student_profile_by_id(
        &self,
        id: Uuid,
    ) -> sqlx::Result<Option<StudentProfile>, sqlx::Error>;

    async fn get_student_profile_by_account_id(
        &self,
        account_id: Uuid,
    ) -> sqlx::Result<Option<StudentProfile>, sqlx::Error>;

    async fn update_student_profile(
        &self,
        id: Uuid,
        student_profile: StudentProfileNew,
    ) -> sqlx::Result<StudentProfile, sqlx::Error>;
}
