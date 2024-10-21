use std::fmt;

#[derive(Debug)]
pub enum RepositoryError {
    NotFound(String),
    InsertFailed(String),
    UpdateFailed(String),
    DeleteFailed(String),
    DatabaseError(String),
    ConnectionError(String),
    QueryError(String),
}

impl fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepositoryError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            RepositoryError::InsertFailed(msg) => write!(f, "Insert Failed: {}", msg),
            RepositoryError::UpdateFailed(msg) => write!(f, "Update Failed: {}", msg),
            RepositoryError::DeleteFailed(msg) => write!(f, "Delete Failed: {}", msg),
            RepositoryError::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            RepositoryError::ConnectionError(msg) => write!(f, "Connection Error: {}", msg),
            RepositoryError::QueryError(msg) => write!(f, "Query Error: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}
