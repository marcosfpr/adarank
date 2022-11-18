/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use crate::{error::LtrError, memory_system::elements::dataset::DataSet};

///
/// SVM-light format loader.
///
pub mod svmlight;

///
/// Defines the interface for loading and saving a dataset given a file path
/// It's useful because models can load datasets directly from a file path.
///
pub trait LtrFormat {
    ///
    /// Loads a dataset from a file path.
    ///
    fn load(path: &str) -> Result<DataSet, LtrError>;

    ///
    /// Saves a dataset to a file path.
    ///
    fn save(path: &str, dataset: &DataSet) -> Result<(), LtrError>;
}
