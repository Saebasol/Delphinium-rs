use std::{error::Error, fmt};

#[derive(Debug)]
pub enum DelphiniumError {
    RequestError(reqwest::Error),
    Http { status: u16, message: String },
}

impl fmt::Display for DelphiniumError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DelphiniumError::RequestError(e) => write!(f, "{}", e),
            DelphiniumError::Http { status, message } => {
                write!(f, "HTTP Error {}: {}", status, message)
            }
        }
    }
}

impl Error for DelphiniumError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DelphiniumError::RequestError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for DelphiniumError {
    fn from(e: reqwest::Error) -> Self {
        if let Some(status) = e.status() {
            DelphiniumError::Http {
                status: status.as_u16(),
                message: e.to_string(),
            }
        } else {
            DelphiniumError::RequestError(e)
        }
    }
}
