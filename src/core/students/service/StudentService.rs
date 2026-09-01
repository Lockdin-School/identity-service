use crate::core::students::models::StudentProfile::{StudentProfile, StudentProfileNew};
use crate::core::students::repository::StudentRepository::StudentRepository;
use std::sync::Arc;
use tokio::io;
use uuid::Uuid;

pub struct StudentService {
    repo: Arc<dyn StudentRepository>,
}

impl StudentService {
    pub fn new(repo: Arc<dyn StudentRepository>) -> Self {
        Self { repo }
    }

    pub async fn create_student_profile(
        &self,
        student_profile: StudentProfileNew,
    ) -> Result<StudentProfile, io::Error> {
        log::info!(
            "student.create | service | create_student_profile | started | Creating student profile..."
        );
        match self.repo.create_student_profile(student_profile).await {
            Ok(student_profile) => {
                log::info!(
                    "student.create | service | create_student_profile | success | Student profile created successfully!"
                );
                Ok(student_profile)
            }
            Err(e) => {
                log::error!(
                    "student.create | service | create_student_profile | failed | Something wrong happened | {}",
                    e
                );
                Err(io::Error::other(e))
            }
        }
    }

    pub async fn get_student_profile_by_id(&self, id: Uuid) -> Result<StudentProfile, io::Error> {
        log::info!(
            "student.get | service | get_student_profile_by_id | started | Getting student profile by id..."
        );
        match self.repo.get_student_profile_by_id(id).await {
            Ok(some_student_profile) => {
                log::info!(
                    "student.get | service | get_student_profile_by_id | in-progress | Continuing to check if the student profile is available"
                );
                match some_student_profile {
                    Some(student_profile) => {
                        log::info!(
                            "student.get | service | get_student_profile_by_id | success | Student profile retrieved successfully!"
                        );
                        Ok(student_profile)
                    }
                    None => {
                        log::error!(
                            "student.get | service | get_student_profile_by_id | failed | Student profile not found."
                        );
                        Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            "Student profile not found",
                        ))
                    }
                }
            }
            Err(e) => {
                log::error!(
                    "student.get | service | get_student_profile_by_id | failed | Something wrong happened | {}",
                    e
                );
                Err(io::Error::other(e))
            }
        }
    }

    pub async fn get_student_profile_by_account_id(
        &self,
        account_id: Uuid,
    ) -> Result<StudentProfile, io::Error> {
        log::info!(
            "student.get | service | get_student_profile_by_account_id | started | Getting student profile by account_id..."
        );
        match self
            .repo
            .get_student_profile_by_account_id(account_id)
            .await
        {
            Ok(some_student_profile) => {
                log::info!(
                    "student.get | service | get_student_profile_by_account_id | in-progress | Continuing to check if the student profile is available."
                );
                match some_student_profile {
                    Some(student_profile) => {
                        log::info!(
                            "student.get | service | get_student_profile_by_account_id | success | Student profile retrieved successfully!"
                        );
                        Ok(student_profile)
                    }
                    None => {
                        log::error!(
                            "student.get | service | get_student_profile_by_account_id | failed | Student profile not found."
                        );
                        Err(io::Error::new(
                            io::ErrorKind::NotFound,
                            "Student profile not found",
                        ))
                    }
                }
            }
            Err(e) => {
                log::error!(
                    "student.get | service | get_student_profile_by_account_id | failed | Something wrong happened | {}",
                    e
                );
                Err(io::Error::other(e))
            }
        }
    }

    pub async fn update_student_profile(
        &self,
        id: Uuid,
        student_profile: StudentProfileNew,
    ) -> Result<StudentProfile, io::Error> {
        log::info!(
            "student.update | service | update_student_profile | started | Updating student profile..."
        );
        match self.repo.update_student_profile(id, student_profile).await {
            Ok(student_profile) => {
                log::info!(
                    "student.update | service | update_student_profile | success | Student profile updated successfully!"
                );
                Ok(student_profile)
            }
            Err(e) => {
                log::error!(
                    "student.update | service | update_student_profile | failed | Something wrong happened | {}",
                    e
                );
                Err(io::Error::other(e))
            }
        }
    }
}
