use crate::core::students::models::StudentProfile::{StudentProfile, StudentProfileNew};
use crate::core::students::repository::StudentRepository::StudentRepository;
use sqlx::{Error, PgPool};
use uuid::Uuid;

pub struct PostgresStudentRepository {
    pool: PgPool,
}

impl PostgresStudentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl StudentRepository for PostgresStudentRepository {
    async fn create_student_profile(
        &self,
        student_profile: StudentProfileNew,
    ) -> sqlx::Result<StudentProfile, Error> {
        sqlx::query_as::<_, StudentProfile>(
            r#"
            INSERT INTO student_profiles (
                account_id,
                first_name,
                last_name,
                avatar_url,
                grade,
                curriculum,
                school_name,
                province,
                onboarding_completed
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                id,
                account_id,
                first_name,
                last_name,
                avatar_url,
                grade,
                curriculum,
                school_name,
                province,
                onboarding_completed,
                created_at,
                updated_at
            "#,
        )
        .bind(student_profile.account_id)
        .bind(student_profile.first_name)
        .bind(student_profile.last_name)
        .bind(student_profile.avatar_url)
        .bind(student_profile.grade)
        .bind(student_profile.curriculum)
        .bind(student_profile.school_name)
        .bind(student_profile.province)
        .bind(student_profile.onboarding_completed)
        .fetch_one(&self.pool)
        .await
    }

    async fn get_student_profile_by_id(
        &self,
        id: Uuid,
    ) -> sqlx::Result<Option<StudentProfile>, Error> {
        sqlx::query_as::<_, StudentProfile>(
            r#"
            SELECT
                id,
                account_id,
                first_name,
                last_name,
                avatar_url,
                grade,
                curriculum,
                school_name,
                province,
                onboarding_completed,
                created_at,
                updated_at
            FROM student_profiles
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn get_student_profile_by_account_id(
        &self,
        account_id: Uuid,
    ) -> Result<Option<StudentProfile>, Error> {
        sqlx::query_as::<_, StudentProfile>(
            r#"
            SELECT
                id,
                account_id,
                first_name,
                last_name,
                avatar_url,
                grade,
                curriculum,
                school_name,
                province,
                onboarding_completed,
                created_at,
                updated_at
            FROM student_profiles
            WHERE account_id = $1
            "#,
        )
        .bind(account_id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn update_student_profile(
        &self,
        id: Uuid,
        student_profile: StudentProfileNew,
    ) -> Result<StudentProfile, Error> {
        sqlx::query_as::<_, StudentProfile>(
            r#"
            UPDATE student_profiles
            SET
                first_name = $2,
                last_name = $3,
                avatar_url = $4,
                grade = $5,
                curriculum = $6,
                school_name = $7,
                province = $8,
                onboarding_completed = $9
            WHERE id = $1
            RETURNING
                id,
                account_id,
                first_name,
                last_name,
                avatar_url,
                grade,
                curriculum,
                school_name,
                province,
                onboarding_completed,
                created_at,
                updated_at
            "#,
        )
        .bind(id)
        .bind(student_profile.first_name)
        .bind(student_profile.last_name)
        .bind(student_profile.avatar_url)
        .bind(student_profile.grade)
        .bind(student_profile.curriculum)
        .bind(student_profile.school_name)
        .bind(student_profile.province)
        .bind(student_profile.onboarding_completed)
        .fetch_one(&self.pool)
        .await
    }
}
