/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use crate::{error::LtrError, DataSet};


///
/// Defines a loader for the SVM-Light input format.
/// 
pub mod svmlight;

///
/// The trait `LtrFormat` defines operations required for
/// implementing a valid LTR input source.
/// 
pub trait LtrFormat {
    ///
    /// Load a `DataSet` from a file given its path.
    /// 
    fn load(path: &str) -> Result<DataSet, LtrError>;
    
    ///
    /// Save a `DataSet` into a file given its path.
    /// 
    fn save(path: &str, dataset: &DataSet) -> Result<(), LtrError>;
}
