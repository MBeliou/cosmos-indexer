use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum LibError {
    #[error("Parse Error")]
    ParseError {},

    #[error("Unknown type url")]
    UnknownTypeUrl { type_url: String },
}
