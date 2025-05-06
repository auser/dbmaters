#[derive(Debug)]
pub enum DbMatersError {
    DatabaseError(String),
    MigrationError(String),
}

impl DbMatersError {
    pub fn new(message: &str) -> Self {
        DbMatersError::DatabaseError(message.to_string())
    }
}
