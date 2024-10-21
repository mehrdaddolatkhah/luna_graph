use crate::application::ports::inbound::user::inbound_ports::UserInboundPort;
use std::sync::Arc;
use uuid::Uuid;

pub struct UserService {}

impl UserInboundPort for UserService {
    fn get_by_id(&self, id: Uuid) -> Option<crate::domain::models::user::User> {
        todo!()
    }

    fn get_all(&self) -> Option<Vec<crate::domain::models::user::User>> {
        todo!()
    }

    fn register_user(
        &self,
        new_user: crate::domain::models::user::NewUser,
    ) -> Result<crate::domain::models::user::User, crate::domain::errors::user_error::UserError>
    {
        todo!()
    }

    fn delete_user(&self, id: Uuid) -> bool {
        todo!()
    }

    fn login_by_user_credentials(
        &self,
        user_credentials: crate::domain::models::user::NewUserCredentials,
    ) -> Result<
        crate::domain::models::user::UserCredentials,
        crate::domain::errors::user_error::UserError,
    > {
        todo!()
    }
}
