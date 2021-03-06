use thiserror::Error;
#[derive(Error, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    #[error("side count must be greater than 2 but got {0}")]
    InvalidNumberSides(usize),
    #[error("cannot convert inner axis to axis (if you see this, something went really wrong)")]
    AxisConvert,
    #[error("move is invalid because the layer(s) specified is out of range")]
    InvalidMoveLayer,
}
