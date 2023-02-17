use reqwest::Error as RqError;
use std::error::Error;
use std::fmt::{Display, Formatter};

///
///  The error kind used by `CasError`
///
/// Use `match` to process different kind.
///
#[derive(Copy, Clone, Debug)]
pub enum ErrorKind {
    UserInfoIncorrect,
    ServiceUrlIncorrect,
    NetworkError,
}

///
/// The error type.
///
/// There are only three kinds of errors,
/// use `kind()` method to get `ErrorKind`.
///
/// Use `get_ref()`, `get_mut()`, `into_inner()`, or `source()` method to get
/// the underlying error.
///
#[derive(Debug)]
pub struct CasError {
    kind: ErrorKind,
    inner: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl CasError {
    pub(crate) fn with_source<E>(kind: ErrorKind, inner: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        Self {
            kind,
            inner: Some(Box::new(inner)),
        }
    }

    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self { kind, inner: None }
    }

    pub fn get_ref(&self) -> Option<&(dyn Error + Send + Sync + 'static)> {
        self.inner.as_ref().map(|e| e.as_ref())
    }

    pub fn get_mut(&mut self) -> Option<&mut (dyn Error + Send + Sync + 'static)> {
        self.inner.as_mut().map(|e| e.as_mut())
    }

    pub fn into_inner(self) -> Option<Box<dyn Error + Send + Sync + 'static>> {
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
            }
            ServiceUrlIncorrect => {
                write!(f, "Service url incorrect")
            }
            NetworkError => {
                write!(f, "Network failed")
            }
        }
    }
}

impl Error for CasError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.get_ref().map(|e| e as &(dyn Error + 'static))
    }
}

impl From<RqError> for CasError {
    fn from(value: RqError) -> Self {
        Self::with_source(ErrorKind::NetworkError, value)
    }
}
