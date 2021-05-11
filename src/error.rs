use thiserror::Error;
#[derive(Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    #[error("side count must be greater than 2 but got {0}")]
    InvalidNumberSides(usize),
    #[error("move that is used is not effective")]
    UneffectiveMove,
    #[error("cannot convert inner axis to axis (if you see this, something went really wrong)")]
    AxisConvert,
}
