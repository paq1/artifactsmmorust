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
    pub status: Option<i32>,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Simple(title) => write!(f, "{title}"),
            Error::WithCode(error_with_code) =>
                write!(
                    f,
                    "(code: {}, title: {}, description: {})",
                    error_with_code.code,
                    error_with_code.title,
                    error_with_code.description.clone().unwrap_or("".to_string())
                )
        }
    }
}

impl std::error::Error for Error {}