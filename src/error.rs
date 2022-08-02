/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use std::fmt::Display;

///
/// All errors specific to the LTR library.
/// These errors can  be called for all modules in the LTR library,
/// and provide a way to identify, semantically well-defined, the source of the error.
///
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LtrError {
    ///
    /// Error raised when a feature is not found in a `DataPoint`.
    /// It's important to notice that the first index of the datapoint's features is 1, not 0.
    /// 
    FeatureIndexOutOfBounds(usize),

    ///
    /// Error raised when a `DataPoint` is not found in a `RankList`.
    /// 
    RankListIndexOutOfBounds(usize),
    
    ///
    /// Error raised when a `DataPoint` is invalid.
    /// 
    InvalidDataPoint(&'static str),
    
    ///
    /// Error raised when an evaluating a `RankList` or a `DataSet` fails.
    /// 
    EvaluationError(&'static str),
    
    ///
    /// Error raised when the parsing from a stream to a `DataPoint` fails.
    /// 
    ParseError(&'static str),
    
    ///
    /// General IO error.
    ///
    IOError(String),


    ///
    /// Error raised when no `Ranker` was found in a `Learner`.
    /// 
    NoRankers,
}

impl Display for LtrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LtrError::FeatureIndexOutOfBounds(i) => write!(f, "Feature index out of bounds: {}", i),
            LtrError::RankListIndexOutOfBounds(i) => write!(f, "RankList index out of bounds: {}", i),
            LtrError::InvalidDataPoint(msg) => write!(f, "Invalid datapoint: {}", msg),
            LtrError::EvaluationError(msg) => write!(f, "Evaluation error: {}", msg),
            LtrError::ParseError(msg) => write!(f, "Error while parsing an input: {}", msg),
            LtrError::IOError(msg) => write!(f, "Error while reading or writing an input: {}", msg),
            LtrError::NoRankers => write!(f, "No rankers were built. Run `fit` first."),
        }
    }
}
