use std::fmt::{Debug, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Simple(String),
    WithCode(ErrorWithCode),
}

#[derive(Debug)]
pub struct ErrorWithCode {
    pub code: String,
    pub title: String,
    pub description: Option<String>,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Simple(title) => write!(f, "{title}"),
            Error::WithCode(errorWithCode) =>
                write!(
                    f,
                    "(code: {}, title: {}, description: {})",
                    errorWithCode.code,
                    errorWithCode.title,
                    errorWithCode.description.clone().unwrap_or("".to_string())
                )
        }
    }
}

impl std::error::Error for Error {}