/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)

#[macro_use]
extern crate lazy_static;

pub mod datapoint;
pub mod error;
pub mod eval;
pub mod loader;
pub mod ranklist;
pub mod utils;
pub mod ranker;
pub mod learner;
pub mod ensemble;


///
/// A particular Feature for lt.rs is just a floating point value.
/// The feature_value is the value of the feature.
type Feature = f32;

///
/// For simplicity, we will use a DataSet as a vector of RankLists.
///
pub type DataSet = Vec<ranklist::RankList>;
