use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    UserInfoIncorrect,
    ServiceUrlIncorrect,
    NetworkError,
    Other,
}

#[derive(Debug)]
pub struct CasError {
    kind: ErrorKind,
    inner: Box<dyn Error + Send + Sync + 'static>,
}


impl CasError {
    fn new<E>(kind: ErrorKind, inner: E) -> Self
    where E: Error + Send + Sync + 'static
    {
        Self {kind, inner: Box::new(inner)}
    }

    fn other<E>(error: E) -> Self
    where E: Error + Send + Sync + 'static
    {
        Self::new(ErrorKind::Other, error)
    }


    pub fn get_ref(&self) -> &(dyn Error + Send + Sync + 'static) {
        self.inner.as_ref()
    }

    pub fn get_mut(&mut self) -> &mut (dyn Error + Send + Sync + 'static) {
        self.inner.as_mut()
    }

    pub fn into_inner(self) -> Box<dyn Error + Send + Sync + 'static> {
        self.inner
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
}

impl Display for CasError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use ErrorKind::*;
        match self.kind {
            UserInfoIncorrect => {
                write!(f, "Username or password incorrect")
            },
            ServiceUrlIncorrect => {
                write!(f, "Service url incorrect")
            },
            NetworkError => {
                write!(f, "Network failed")
            },
            Other => {
                write!(f, "{}", self.get_ref())
            }
        }
    }
}


impl Error for CasError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(self.get_ref())
    }
}