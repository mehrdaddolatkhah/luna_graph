use crate::adapters::outbound::repositories::user::db_entities::UserDb;
use crate::domain::errors::repository_error::RepositoryError;
use crate::domain::models::user::{NewUser, User};
use uuid::Uuid;

pub trait UserOutboundPort: Send + Sync {
    fn get_by_id(&self, id: Uuid) -> Option<User>;
    fn get_all(&self) -> Option<Vec<User>>;
    fn add_user(&self, new_user: NewUser) -> Option<User>;
    fn update_user_token(&self, user_id: Uuid, new_token: &str) -> Result<(), RepositoryError>;
    fn delete_user(&self, id: Uuid) -> bool;
    fn find_user_by_mobile(&self, mobile: &str) -> Option<UserDb>;
    fn find_user_by_email(&self, email: &str) -> Option<UserDb>;
}
