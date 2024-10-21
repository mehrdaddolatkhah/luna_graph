use crate::infrastructure::schema::users;
use diesel::prelude::*;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd};
use uuid::Uuid;

#[derive(Clone, Queryable, QueryableByName, Selectable, Ord, Eq, PartialEq, PartialOrd, Debug)]
#[diesel(table_name = users)]
pub struct UserDb {
    #[diesel(column_name = "id")]
    pub id: Uuid,

    #[diesel(column_name = "first_name")]
    pub first_name: String,

    #[diesel(column_name = "last_name")]
    pub last_name: Option<String>,

    #[diesel(column_name = "birth_date")]
    pub birth_date: Option<chrono::NaiveDate>,

    #[diesel(column_name = "email")]
    pub email: Option<String>,

    #[diesel(column_name = "mobile")]
    pub mobile: Option<String>,

    #[diesel(column_name = "password")]
    pub password: String,

    #[diesel(column_name = "email_verified")]
    pub email_verified: Option<bool>,

    #[diesel(column_name = "token")]
    pub token: Option<String>,

    #[diesel(column_name = "provider_id")]
    pub provider_id: Option<String>,

    #[diesel(column_name = "provider_access_token")]
    pub provider_access_token: Option<String>,

    #[diesel(column_name = "provider_refresh_token")]
    pub provider_refresh_token: Option<String>,

    #[diesel(column_name = "provider_user_id")]
    pub provider_user_id: Option<String>,

    #[diesel(column_name = "public_key_credential")]
    pub public_key_credential: Option<String>,

    #[diesel(column_name = "credential_type")]
    pub credential_type: Option<String>,

    #[diesel(column_name = "role_id")]
    pub role_id: Option<Uuid>,

    #[diesel(column_name = "status")]
    pub status: Option<String>,
    #[diesel(column_name = "last_login")]
    pub last_login: Option<chrono::NaiveDateTime>,

    #[diesel(column_name = "created_at")]
    pub created_at: chrono::NaiveDateTime,

    #[diesel(column_name = "updated_at")]
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUserDb {
    #[diesel(column_name = "first_name")]
    pub first_name: String,

    #[diesel(column_name = "last_name")]
    pub last_name: Option<String>,

    #[diesel(column_name = "birth_date")]
    pub birth_date: Option<chrono::NaiveDate>,

    #[diesel(column_name = "email")]
    pub email: Option<String>,

    #[diesel(column_name = "mobile")]
    pub mobile: Option<String>,

    #[diesel(column_name = "password")]
    pub password: String,
}
