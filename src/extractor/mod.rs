pub mod agent;
mod error;
pub mod git;
pub mod package;

pub use error::ExtractError;

pub trait Extractor: Sized {
    type Error: Into<ExtractError>;

    fn extract() -> Result<Self, Self::Error>;
}
