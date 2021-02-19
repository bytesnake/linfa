use thiserror::Error;
use ndarray_stats::errors::MinMaxError;

pub type Result<T> = std::result::Result<T, BayesError>;

#[derive(Error, Debug)]
pub enum BayesError {
    /// Error when performing Max operation on data
    #[error("invalid statistical operation {0}")]
    Stats(#[from] MinMaxError),
    #[error(transparent)]
    BaseCrate(#[from] linfa::Error),
}
