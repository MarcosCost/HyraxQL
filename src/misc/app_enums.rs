pub enum AppError {
    InvalidArgs(String),
    DatabaseError(String),
    NotFound,
}