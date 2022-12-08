/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)
use std::cell::{Ref, RefCell};
use std::fmt;
use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

use crate::error::LtrError;
use crate::memory_system::elements::byte_rpr::{ByteRpr, FixedByteLen};
use crate::memory_system::elements::datapoint::DataPoint;

use super::byte_rpr::DynamicByteLen;

/// A RankList is the object to be ranked by `Learner`s.
///
/// The RankList primitive represents a collections of `DataPoint`s
/// corresponding to the same query id. This property is checked at runtime.
///
/// RankLists are used by `Learner`s to rank `DataPoint`s and offer a way to
/// evaluate the performance of the `Learner`.
///
/// It's important to notice that RankList offers interior mutability,
/// which means that it's possible to modify a RankList
/// without mutable borrowing it. This is particularly useful when
/// shuffling the `DataPoint`s inside the RankList.
///
/// TODO: the comments above leads to unefficient code. I'll work in some refactoring.
///
#[derive(Clone, Serialize, Deserialize)]
pub struct RankList {
    ///
    /// The list of `DataPoint`s.
    ///
    data_points: RefCell<Vec<DataPoint>>,
}

#[derive(Clone)]
pub struct RankListPermutation<'a> {
    ///
    /// The list of pairs (Original position, Score, Label)
    ///
    pub(crate) permutation: Vec<usize>,

    pub(crate) ranklist: &'a RankList,
}

impl RankList {
    /// Creates a new `RankList` with the given `DataPoint`s.
    ///
    /// # Arguments
    ///
    /// * `data_points` - The list of `DataPoint`s.
    ///
    pub fn new(data_points: Vec<DataPoint>) -> RankList {
        RankList {
            data_points: RefCell::new(data_points),
        }
    }

    /// Get the length of the `RankList`.
    ///
    /// # Returns
    ///
    /// The length of the `RankList`.
    ///
    pub fn len(&self) -> usize {
        self.data_points.borrow().len()
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
    pub fn get(&self, index: usize) -> Result<Ref<DataPoint>, LtrError> {
        if index < self.len() {
            Ok(Ref::map(self.data_points.borrow(), |dp| &dp[index]))
        } else {
            Err(LtrError::RankListIndexOutOfBounds(index))
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
    pub fn set(&self, index: usize, data_point: DataPoint) -> Result<(), LtrError> {
        if index < self.len() {
            self.data_points.borrow_mut()[index] = data_point;
            Ok(())
        } else {
            Err(LtrError::RankListIndexOutOfBounds(index))
        }
    }

    ///
    /// Rank the `RankList` according to the given `DataPoint`s.
    ///
    pub fn rank(&self) -> Result<(), LtrError> {
        // Reverse sorting
        self.data_points
            .borrow_mut()
            .sort_by(|a, b| b.partial_cmp(&a).unwrap());
        Ok(())
    }

    ///
    /// Rank the `RankList` according to the given `DataPoint`s and a given feature index.
    ///
    /// # Arguments
    /// * `feature_index` - The index of the feature to be used to sort the `RankList`.
    ///
    pub fn rank_by_feature(&self, feature_index: usize) -> Result<(), LtrError> {
        self.data_points.borrow_mut().sort_by(|a, b| {
            b.get_feature(feature_index)
                .unwrap()
                .partial_cmp(&a.get_feature(feature_index).unwrap())
                .unwrap()
        });
        Ok(())
    }

    ///
    /// Permute the `RankList` according to the given permutation vector.Performs poorly.
    ///
    /// # Arguments
    /// * `permutation` - The permutation vector.
    ///
    pub fn permute(&self, permutation: Vec<usize>) -> Result<(), LtrError> {
        let mut new_data_points = Vec::with_capacity(self.data_points.borrow().len());
        for i in permutation {
            match self.data_points.borrow().get(i) {
                Some(dp) => new_data_points.push(dp.clone()),
                None => return Err(LtrError::RankListIndexOutOfBounds(i)),
            }
        }
        self.data_points.replace(new_data_points);
        Ok(())
    }
}

///
/// A `RankList` iterator.
/// It's possible to iterate over a `RankList` using the `Iterator` trait.
/// Still a in-development feature.
///
pub struct RankListIter<'a> {
    rank_list: &'a RankList,
    index: usize,
}

impl<'a> Iterator for RankListIter<'a> {
    type Item = Ref<'a, DataPoint>;

    fn next(&mut self) -> Option<Ref<'a, DataPoint>> {
        if self.index < self.rank_list.len() {
            self.index += 1;
            Some(Ref::map(self.rank_list.data_points.borrow(), |dp| {
                &dp[self.index - 1]
            }))
        } else {
            None
        }
    }
}

///
/// `RankList`s are iterable over `&DataPoint`s.
/// This allows for easy iteration over the `RankList`.
///
impl<'a> IntoIterator for &'a RankList {
    type Item = Ref<'a, DataPoint>;
    type IntoIter = RankListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        RankListIter {
            rank_list: self,
            index: 0,
        }
    }
}

///
/// We can interpret a `RankList` as a `Vec` of `DataPoint`s.
///
impl From<Vec<DataPoint>> for RankList {
    fn from(data_points: Vec<DataPoint>) -> RankList {
        RankList {
            data_points: RefCell::new(data_points),
        }
    }
}

///
/// Displaying a `RankList` is done by displaying the `DataPoint`s.
///
impl fmt::Display for RankList {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "RankList object with {} data points",
            self.data_points.borrow().len()
        )
    }
}

impl DynamicByteLen for RankList {
    fn segment_len(&self) -> usize {
        let vec_len: usize = self
            .data_points
            .borrow()
            .iter()
            .map(|dp| dp.segment_len())
            .sum();

        u64::segment_len() + vec_len
    }
}

impl ByteRpr for RankList {
    fn as_byte_rpr(&self, buff: &mut dyn std::io::Write) -> usize {
        let dps = self.data_points.borrow();
        let mut size: usize = (dps.len() as u64).as_byte_rpr(buff);
        dps.iter().for_each(|dp| {
            size += (dp.segment_len() as u64).as_byte_rpr(buff) + dp.as_byte_rpr(buff);
        });
        size
    }

    fn from_byte_rpr(bytes: &[u8]) -> Self {
        let (mut start, mut end) = (0, u64::segment_len() as usize);
        let len = u64::from_byte_rpr(&bytes[start..end]);
        let mut data_points = Vec::new();
        for _ in 0..len {
            start = end;
            end = end + u64::segment_len();

            let nbytes = u64::from_byte_rpr(&bytes[start..end]) as usize;

            start = end;
            end = end + nbytes;

            data_points.push(DataPoint::from_byte_rpr(&bytes[start..end]));
        }
        RankList::new(data_points)
    }
}

impl<'a> From<&'a RankList> for RankListPermutation<'a> {
    fn from(rl: &'a RankList) -> RankListPermutation<'a> {
        RankListPermutation {
            permutation: (0..rl.len()).collect(),
            ranklist: rl,
        }
    }
}

///
/// A macro to create a `RankList` from a vector of
/// `DataPoint`s represented by a tuple of label,  query_id,
/// features and the optional description.
///
#[macro_export]
macro_rules! rl {
    ($(($label:expr, $query_id:expr, $features:expr)),*) => {
        {
            let mut data_points = Vec::new();
            $(
                data_points.push(crate::dp!($label, $query_id, $features));
            )*
            RankList::new(data_points)
        }
    };
    ($(($label:expr, $query_id:expr, $features:expr, $description:expr)),*) => {
        {
            let mut data_points = Vec::new();
            $(
                data_points.push(crate::dp!($label, $query_id, $features, $description));
            )*
            RankList::new(data_points)
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{dp, fvec, loader::svmlight::*};

    #[test]
    fn test_ranklist() {
        let rank_list = rl!(
            (0, 9, fvec![10.0, 1.2, 4.3, 5.4], "doc1"),
            (1, 9, fvec![11.0, 2.2, 4.5, 5.6], "doc2"),
            (0, 9, fvec![12.0, 2.5, 4.7, 5.2], "doc3")
        );

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
        assert_eq!(**first_data_point.get_feature(1).unwrap(), 10.0f32);

        // checking the second data point just for sanity
        let second_data_point = rank_list.get(1).unwrap();
        assert_eq!(second_data_point.get_label(), 1);
        assert_eq!(second_data_point.get_query_id(), 9);
        assert_eq!(**second_data_point.get_feature(2).unwrap(), 2.2f32);

        // checking the third data point just for sanity
        let third_data_point = rank_list.get(2).unwrap();
        assert_eq!(third_data_point.get_label(), 0);
        assert_eq!(third_data_point.get_query_id(), 9);
        assert_eq!(**third_data_point.get_feature(3).unwrap(), 4.7f32);

        let string_representation = format!("{}", rank_list);
        assert_eq!(string_representation, "RankList object with 3 data points");

        // Ranking
        let partial_rank_list = rank_list.clone();
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

        let full_rank_list = rank_list.clone();
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
        let permuted_rank_list = rank_list.clone();
        let permutation = vec![1, 2, 0];

        let invalid_permutation = vec![1, 2, 3];
        assert!(permuted_rank_list.permute(invalid_permutation).is_err());

        permuted_rank_list.permute(permutation).unwrap();
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
        let set_rank_list = rank_list.clone();

        let new_dp = SVMLight::load_datapoint("2 qid:9 1:10 2:1.2 3:4.3 4:5.4 # doc23").unwrap();

        set_rank_list.set(0, new_dp.clone()).unwrap();
        assert_eq!(
            set_rank_list.get(0).unwrap().get_description().unwrap(),
            "doc23"
        );

        match set_rank_list.set(100, new_dp) {
            Err(er) => assert_eq!(er, LtrError::RankListIndexOutOfBounds(100 as usize)),
            _ => unreachable!(),
        };
    }

    #[test]
    fn test_ranklist_iterator() {
        let rank_list: RankList = RankList::from(vec![
            dp!(0, 9, fvec![10.0, 1.2, 4.3, 5.4], "doc1"),
            dp!(1, 9, fvec![11.0, 2.2, 4.5, 5.6], "doc2"),
            dp!(0, 9, fvec![12.0, 2.5, 4.7, 5.2], "doc3"),
        ]);

        assert_eq!(rank_list.len(), 3);

        for (i, data_point) in rank_list.into_iter().enumerate() {
            assert_eq!(
                data_point.get_label(),
                rank_list.get(i).unwrap().get_label()
            );
            assert_eq!(
                data_point.get_query_id(),
                rank_list.get(i).unwrap().get_query_id()
            );
            assert_eq!(
                *data_point.get_feature(1).unwrap(),
                *rank_list.get(i).unwrap().get_feature(1).unwrap()
            );
        }
    }
}
