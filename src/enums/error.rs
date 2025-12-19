use std::fmt::{self, Display};

use crate::enums::token::Token;

pub enum Error {
    ParseError(usize, Token),
    CompilerError(usize, String),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> fmt::Result {
        let error = match self {
            Error::CompilerError(error, _) => error.to_string(),
            Error::ParseError(usize, token) => {
                format!("[line {}] Error at '{}': Expect expression.", usize, token)
            }
        };
        write!(fmt, "{}", error)
    }
}
