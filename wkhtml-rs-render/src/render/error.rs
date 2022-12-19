use thiserror::Error;

#[derive(Error, Debug)]
pub enum RenderError {
    #[error("Format error: {0}")]
    FormatError(#[from] std::fmt::Error),
    #[error("Unsupported tag: {0}")]
    UnsupportedTag(String),
}
