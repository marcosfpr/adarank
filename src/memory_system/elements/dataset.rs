/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)
use super::storage::Storage;

///
/// A LTR dataset structure
///
pub struct DataSet2 {
    ///
    /// RankLists ids
    ///
    pub(crate) ranklists: Vec<usize>,
    ///
    /// Memory storage for the ranklists
    ///
    pub(crate) storage: Storage,
}

///
/// For simplicity, we will use a DataSet as a vector of RankLists.
///
pub type DataSet = Vec<super::ranklist::RankList>;
