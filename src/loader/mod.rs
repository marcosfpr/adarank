/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use crate::{error::LtrError, DataSet};
pub mod svmlight;

pub trait LtrFormat {
    fn load(path: &str) -> Result<DataSet, LtrError>;
    fn save(path: &str, dataset: &DataSet) -> Result<(), LtrError>;
}
