use actix_web::{HttpResponse, ResponseError};
use std::fmt;

#[derive(Debug)]
pub enum UserError {
    HashError(String),
    TokenGenerationError(String),
    TokenValidationError(String),
    MissingToken,
    InvalidToken(String),
    TokenExpired(String),
    InvalidCredentials,
    Unauthorized,
    RegistrationError(String),
    LoginError(String),
    UnknownError(String),
}

impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserError::HashError(e) => write!(f, "Hashing error: {}", e),
            UserError::TokenGenerationError(e) => write!(f, "Token generation error: {}", e),
            UserError::TokenValidationError(e) => write!(f, "Token validation error: {}", e),
            UserError::MissingToken => write!(f, "MissingToken"),
            UserError::InvalidToken(e) => write!(f, "Invalid token: {}", e),
            UserError::TokenExpired(e) => write!(f, "Token expired: {}", e),
            UserError::InvalidCredentials => write!(f, "Invalid credentials"),
            UserError::Unauthorized => write!(f, "Unauthorized access"),
            UserError::RegistrationError(e) => write!(f, "Registration error: {}", e),
            UserError::LoginError(e) => write!(f, "Login error: {}", e),
            UserError::UnknownError(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        match self {
            UserError::Unauthorized => HttpResponse::Unauthorized().body("Unauthorized"),
            UserError::InvalidCredentials => HttpResponse::BadRequest().body("Invalid credentials"),
            UserError::MissingToken => HttpResponse::BadRequest().body("Missing token"),
            UserError::InvalidToken(_) => HttpResponse::BadRequest().body("Invalid token"),
            UserError::TokenExpired(_) => HttpResponse::Unauthorized().body("Token expired"),
            _ => HttpResponse::InternalServerError().body("Internal server error"),
        }
    }
}

impl std::error::Error for UserError {}
