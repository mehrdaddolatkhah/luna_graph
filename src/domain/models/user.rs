use uuid::Uuid;

#[derive(Debug)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,

    pub last_name: Option<String>,

    pub birth_date: Option<chrono::NaiveDate>,

    pub email: Option<String>,

    pub mobile: Option<String>,

    pub password: String,

    pub email_verified: Option<bool>,

    pub token: Option<String>,

    pub provider_id: Option<String>,

    pub provider_access_token: Option<String>,

    pub provider_refresh_token: Option<String>,

    pub provider_user_id: Option<String>,

    pub public_key_credential: Option<String>,

    pub credential_type: Option<String>,

    pub role_id: Option<Uuid>,

    pub status: Option<String>,

    pub last_login: Option<chrono::NaiveDateTime>,
}

#[derive(Debug)]
pub struct NewUser {
    pub first_name: String,

    pub last_name: Option<String>,

    pub birth_date: Option<chrono::NaiveDate>,

    pub email: Option<String>,

    pub mobile: Option<String>,

    pub password: String,

    pub email_verified: Option<bool>,

    pub provider_id: Option<String>,

    pub provider_access_token: Option<String>,

    pub provider_refresh_token: Option<String>,

    pub provider_user_id: Option<String>,

    pub public_key_credential: Option<String>,

    pub credential_type: Option<String>,

    pub role_id: Option<Uuid>,

    pub status: Option<String>,

    pub last_login: Option<chrono::NaiveDateTime>,
}

#[derive(Debug)]
pub struct UserCredentials {
    pub user_id: Uuid,
    pub token: String,
}

#[derive(Debug)]
pub struct NewUserCredentials {
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub password: String,
}
