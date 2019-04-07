#[derive(Debug)]
pub enum ChallengeError {
    DatabaseError,
    NotFoundError,
    SerializationError,
}
