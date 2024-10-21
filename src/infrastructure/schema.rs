// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Nullable<Varchar>,
        birth_date -> Nullable<Date>,
        #[max_length = 50]
        email -> Nullable<Varchar>,
        #[max_length = 20]
        mobile -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Varchar,
        email_verified -> Nullable<Bool>,
        token -> Nullable<Text>,
        #[max_length = 100]
        provider_id -> Nullable<Varchar>,
        provider_access_token -> Nullable<Text>,
        provider_refresh_token -> Nullable<Text>,
        #[max_length = 100]
        provider_user_id -> Nullable<Varchar>,
        public_key_credential -> Nullable<Text>,
        #[max_length = 50]
        credential_type -> Nullable<Varchar>,
        role_id -> Nullable<Uuid>,
        #[max_length = 50]
        status -> Nullable<Varchar>,
        last_login -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
