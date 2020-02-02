use std::fmt;

pub enum InternalOpenError {
    NoSuchObject,
}

impl fmt::Display for InternalOpenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::NoSuchObject => write!(f, "The destination object does not exist on this server or does not support this protocol"),
        }
    }
}

pub enum OpenError {
    Internal(InternalOpenError),
    ChallengeFailed,
    Timeout,
    Io(String),
}

impl From<InternalOpenError> for OpenError {
    fn from(from: InternalOpenError) -> Self {
        Self::Internal(from)
    }
}

impl From<String> for OpenError {
    fn from(from: String) -> Self {
        Self::Io(from)
    }
}

impl fmt::Display for OpenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Internal(err) => err.fmt(f),
            Self::ChallengeFailed => write!(f, "Challenge failed"),
            Self::Io(err) => write!(f, "IO error: {}", err),
            Self::Timeout => write!(f, "Connection timeout"),
        }
    }
}
