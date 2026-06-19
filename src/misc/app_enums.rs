#[derive(Debug)]
pub enum AppError {
    InvalidArgs(String),
    DatabaseError(String),
    NotFound,
}