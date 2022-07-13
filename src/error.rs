/// Copyright (c) 2021 Marcos Pontes
/// MIT License

///
/// The library's error enum.
///
#[derive(Debug, Clone)]
pub enum LtrError {
    FeatureIndexOutOfBounds,
    RankListIndexOutOfBounds,
    InvalidDataPoint(&'static str),
    MetricError(&'static str),
    ParseError,
}
