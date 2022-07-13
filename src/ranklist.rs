use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use crate::datapoint::DataPoint;
use crate::error::LtrError;

/// Copyright (c) 2021 Marcos Pontes
/// MIT License

/// A RankList is the object to be ranked by models.
///
/// A RankList is a list of `DataPoint`s corresponding to the same query id.
/// This property must be checked at runtime.
///
#[derive(Clone, Serialize, Deserialize)]
pub struct RankList {
    /// The list of `DataPoint`s.
    ///
    data_points: Vec<DataPoint>,
}

impl RankList {
    /// Creates a new `RankList` with the given `DataPoint`s.
    ///
    /// # Arguments
    ///
    /// * `data_points` - The list of `DataPoint`s.
    ///
    pub fn new(data_points: Vec<DataPoint>) -> RankList {
        RankList { data_points }
    }

    /// Get the length of the `RankList`.
    ///
    /// # Returns
    ///
    /// The length of the `RankList`.
    ///
    pub fn len(&self) -> usize {
        self.data_points.len()
    }

    ///
    /// Get the `DataPoint` at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the `DataPoint` to be returned.
    ///
    /// # Returns
    ///
    /// The `DataPoint` at the given index.
    ///
    pub fn get(&self, index: usize) -> Result<&DataPoint, LtrError> {
        if index < self.data_points.len() {
            Ok(&self.data_points[index])
        } else {
            Err(LtrError::RankListIndexOutOfBounds)
        }
    }

    ///
    /// Set the `DataPoint` at the given index.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the `DataPoint` to be set.
    /// * `data_point` - The `DataPoint` to be set.
    ///
    pub fn set(&mut self, index: usize, data_point: DataPoint) -> Result<(), LtrError> {
        if index < self.data_points.len() {
            self.data_points[index] = data_point;
            Ok(())
        } else {
            Err(LtrError::RankListIndexOutOfBounds)
        }
    }

    ///
    /// Rank the `RankList` according to the given `DataPoint`s.
    ///
    pub fn rank(&mut self) -> Result<(), LtrError> {
        // Reverse sorting
        self.data_points.sort_by(|a, b| b.partial_cmp(&a).unwrap());
        Ok(())
    }

    ///
    /// Rank the `RankList` according to the given `DataPoint`s and a given feature index.
    ///
    /// # Arguments
    /// * `feature_index` - The index of the feature to be used to sort the `RankList`.
    ///
    pub fn rank_by_feature(&mut self, feature_index: usize) -> Result<(), LtrError> {
        self.data_points.sort_by(|a, b| {
            b.get_feature(feature_index)
                .unwrap()
                .partial_cmp(&a.get_feature(feature_index).unwrap())
                .unwrap()
        });
        Ok(())
    }

    ///
    /// Permute the `RankList` according to the given permutation vector.
    ///
    /// # Arguments
    /// * `permutation` - The permutation vector.
    ///
    pub fn permute(&mut self, permutation: Vec<usize>) {
        let mut new_data_points = Vec::with_capacity(self.data_points.len());
        for i in permutation {
            new_data_points.push(self.data_points[i].clone());
        }
        self.data_points = new_data_points;
    }
}

impl IntoIterator for RankList {
    type Item = DataPoint;
    type IntoIter = std::vec::IntoIter<DataPoint>;

    fn into_iter(self) -> Self::IntoIter {
        self.data_points.into_iter()
    }
}

impl From<Vec<DataPoint>> for RankList {
    fn from(data_points: Vec<DataPoint>) -> RankList {
        RankList { data_points }
    }
}

impl fmt::Display for RankList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RankList object with {} data points",
            self.data_points.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::svmlight::*;

    #[test]
    fn test_ranklist() {
        let data_points = vec![
            SVMLight::load_datapoint("0 qid:9 1:10 2:1.2 3:4.3 4:5.4 # doc1").unwrap(),
            SVMLight::load_datapoint("1 qid:9 1:11 2:2.2 3:4.5 4:5.6 # doc2").unwrap(),
            SVMLight::load_datapoint("0 qid:9 1:12 2:2.5 3:4.7 4:5.2 # doc3").unwrap(),
        ];

        let rank_list = RankList::new(data_points);
        assert_eq!(rank_list.len(), 3);

        let another_rank_list = rank_list.clone();
        assert_eq!(another_rank_list.len(), 3);

        assert!(rank_list.get(0).is_ok());
        assert!(rank_list.get(1).is_ok());
        assert!(rank_list.get(2).is_ok());
        assert!(rank_list.get(3).is_err());

        // checking the first data point just for sanity
        let first_data_point = rank_list.get(0).unwrap();
        assert_eq!(first_data_point.get_label(), 0);
        assert_eq!(first_data_point.get_query_id(), 9);
        assert_eq!(*first_data_point.get_feature(1).unwrap(), 10.0f32);

        // checking the second data point just for sanity
        let second_data_point = rank_list.get(1).unwrap();
        assert_eq!(second_data_point.get_label(), 1);
        assert_eq!(second_data_point.get_query_id(), 9);
        assert_eq!(*second_data_point.get_feature(2).unwrap(), 2.2f32);

        // checking the third data point just for sanity
        let third_data_point = rank_list.get(2).unwrap();
        assert_eq!(third_data_point.get_label(), 0);
        assert_eq!(third_data_point.get_query_id(), 9);
        assert_eq!(*third_data_point.get_feature(3).unwrap(), 4.7f32);

        let string_representation = format!("{}", rank_list);
        assert_eq!(string_representation, "RankList object with 3 data points");

        // Ranking
        let mut partial_rank_list = rank_list.clone();
        partial_rank_list.rank_by_feature(1).unwrap();
        assert_eq!(partial_rank_list.len(), 3);
        assert_eq!(
            partial_rank_list.get(0).unwrap().get_description().unwrap(),
            "doc3"
        );
        assert_eq!(
            partial_rank_list.get(1).unwrap().get_description().unwrap(),
            "doc2"
        );
        assert_eq!(
            partial_rank_list.get(2).unwrap().get_description().unwrap(),
            "doc1"
        );

        let mut full_rank_list = rank_list.clone();
        full_rank_list.rank().unwrap();
        assert_eq!(full_rank_list.len(), 3);
        assert_eq!(
            full_rank_list.get(0).unwrap().get_description().unwrap(),
            "doc2"
        );
        assert_eq!(
            full_rank_list.get(1).unwrap().get_description().unwrap(),
            "doc1"
        );
        assert_eq!(
            full_rank_list.get(2).unwrap().get_description().unwrap(),
            "doc3"
        );

        // Permutation
        let mut permuted_rank_list = rank_list.clone();
        let permutation = vec![1, 2, 0];
        permuted_rank_list.permute(permutation);
        assert_eq!(
            permuted_rank_list
                .get(0)
                .unwrap()
                .get_description()
                .unwrap(),
            "doc2"
        );
        assert_eq!(
            permuted_rank_list
                .get(1)
                .unwrap()
                .get_description()
                .unwrap(),
            "doc3"
        );
        assert_eq!(
            permuted_rank_list
                .get(2)
                .unwrap()
                .get_description()
                .unwrap(),
            "doc1"
        );

        //  Set
        let mut set_rank_list = rank_list.clone();
        set_rank_list
            .set(
                0,
                SVMLight::load_datapoint("2 qid:9 1:10 2:1.2 3:4.3 4:5.4 # doc23").unwrap(),
            )
            .unwrap();
        assert_eq!(
            set_rank_list.get(0).unwrap().get_description().unwrap(),
            "doc23"
        );
    }
}
