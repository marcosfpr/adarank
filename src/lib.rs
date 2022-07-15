/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)
pub mod datapoint;
pub mod error;
pub mod eval;
pub mod loader;
pub mod ranklist;
pub mod utils;

///
/// For simplicity, we will use a DataSet as a vector of RankLists.
///
pub type DataSet = Vec<ranklist::RankList>;
