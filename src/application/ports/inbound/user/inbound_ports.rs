use crate::domain::models::user::{NewUser, NewUserCredentials, User, UserCredentials};
use crate::domain::errors::user_error::UserError;
use uuid::Uuid;

pub trait UserInboundPort: Send + Sync {
    fn get_by_id(&self, id: Uuid) -> Option<User>;
    fn get_all(&self) -> Option<Vec<User>>;
    fn register_user(&self, new_user: NewUser) -> Result<User, UserError>;
    fn delete_user(&self, id: Uuid) -> bool;
    fn login_by_user_credentials(
        &self,
        user_credentials: NewUserCredentials,
    ) -> Result<UserCredentials, UserError>;
}
