use std::{self, fmt};
use heck::SnakeCase;

#[derive(Debug)]
pub enum PredicateType {
    Intersects,
    Crosses,
}

impl fmt::Display for PredicateType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_snake_case())
    }
}

#[derive(Fail, Debug)]
pub enum Error {
    #[fail(display = "Invalid geometry, {}", _0)]
    InvalidGeometry(String),
    #[fail(display = "Impossible operation, {}", _0)]
    ImpossibleOperation(String),
    #[fail(display = "error while calling libgeos while {}", _0)]
    GeosError(String),
    #[fail(display = "error while calling libgeos method {} (error number = {})", _0, _1)]
    GeosFunctionError(PredicateType, i32),
    #[fail(display = "impossible to build a geometry from a nullptr")]
    NoConstructionFromNullPtr,
}

pub type Result<T> = std::result::Result<T, Error>;
