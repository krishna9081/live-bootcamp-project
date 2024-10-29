pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    UnexpectedError,
    UnprocessableEntity,
    IncorrectCredentials,
    MissingToken,
    InvalidToken,
}