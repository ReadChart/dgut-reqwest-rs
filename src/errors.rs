#[derive(Debug, Clone)]
pub enum GetSessionIDError {
    RequestFailure,
    KeywordMatchingFailure,
    OtherFailure,
}

impl std::fmt::Display for GetSessionIDError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Failed To Retract Session ID Due To: {}",
            self.value()
        )
    }
}

impl GetSessionIDError {
    pub fn value(&self) -> &str {
        match *self {
            GetSessionIDError::RequestFailure => "Failed to make request",
            GetSessionIDError::KeywordMatchingFailure => "Failed to match session keyword",
            _ => "Unknown Error",
        }
    }
}
impl std::error::Error for GetSessionIDError {}

#[derive(Debug, Clone)]
pub enum AuthenticationError {
    TokenExtractionFailure,
    EmptyCredentialFailure,
}

impl std::fmt::Display for AuthenticationError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "Failed To Retract Personal Cas Token Due To: {}",
            self.value()
        )
    }
}

impl AuthenticationError {
    pub fn value(&self) -> &str {
        match *self {
            AuthenticationError::TokenExtractionFailure => "Failed To Extract Personal Token",
            AuthenticationError::EmptyCredentialFailure => "Username Or Password Is Empty",
            _ => "Other Error",
        }
    }
}

impl std::error::Error for AuthenticationError {}
