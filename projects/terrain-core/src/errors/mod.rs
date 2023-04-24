use std::error::Error;
use std::fmt::{Display, Formatter};
mod display;

pub type TerrainResult<T> = Result<T, TerrainError>;

#[derive(Clone, Debug, PartialEq)]
pub struct TerrainError {
    kind: Box<TerrainErrorKind>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TerrainErrorKind {
    UnknownError
}

