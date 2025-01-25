use crate::{error::LtrError, DataSet};

/// SVM-light format loader.
pub mod svmlight;

/// Defines the interface for loading and saving a dataset given a file path
/// It's useful because models can load datasets directly from a file path.
pub trait LtrFormat {
    /// Loads a dataset from a file path.
    ///
    /// # Arguments
    /// * `path` - The path to the file.
    ///
    /// # Returns
    /// A `DataSet` with the data loaded from the file.
    fn load(path: &str) -> Result<DataSet, LtrError>;

    /// Saves a dataset to a file path.
    ///
    /// # Arguments
    /// * `path` - The path to the file.
    /// * `dataset` - The dataset to be saved.
    ///
    /// # Returns
    /// A `Result` with the success of the operation.
    fn save(path: &str, dataset: &DataSet) -> Result<(), LtrError>;
}
