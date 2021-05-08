use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("side count must be greater than 2 but got {0}")]
    InvalidNumberSides(usize),
}
