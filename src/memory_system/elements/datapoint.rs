/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)
use std::cmp::Ordering;
use std::fmt;

use std::ops::Index;

use serde::{Deserialize, Serialize};

use crate::error::LtrError;

use super::{
    byte_rpr::{ByteRpr, FixedByteLen},
    feature::Feature,
};

///
/// A DataPoint is a single training instance (Like in RankLib).
/// A DataPoint represents a pair `[item, query]` extracted
/// from a LTR-valid data format. A common format is the SVM-Light
/// format:
/// `<label> qid:<query_id> <feature_1>:<value_1> <feature_2>:<value_2> ...<feature_n>:<value_n>`
///
/// where `<label>` is the target value, `<query_id>` is the query ID,  <feature_i> is the feature
/// and `<value_i>` is the value of the feature.
///
///
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    /// The label of the DataPoint.
    label: u8,
    /// The query id of the DataPoint.
    /// This is the identifier of the query that the DataPoint belongs to.
    query_id: u64,
    /// The features of the DataPoint.
    /// This is a vector of `Feature`s.
    features: Vec<Feature>,
    /// Optional description of the DataPoint.
    /// This is a string that can be used to describe the DataPoint.
    description: Option<String>,
}

impl DataPoint {
    ///
    /// Creates an empty DataPoint
    ///
    pub fn empty() -> DataPoint {
        DataPoint {
            label: 0,
            query_id: 0,
            features: Vec::new(),
            description: None,
        }
    }

    ///
    /// Creates a new DataPoint.
    ///
    /// # Arguments
    ///
    /// * `label` - The label of the DataPoint.
    /// * `query_id` - The query id of the DataPoint.
    /// * `features` - The features of the DataPoint.
    /// * `description` - Optional description of the DataPoint.
    ///
    pub fn new(
        label: u8,
        query_id: u64,
        features: Vec<Feature>,
        description: Option<&str>,
    ) -> DataPoint {
        DataPoint {
            label,
            query_id,
            features,
            description: description.map(|s| s.to_string()), // None or Some(s)
        }
    }

    ///
    /// Returns the label of the DataPoint.
    ///
    pub fn get_label(&self) -> u8 {
        self.label
    }

    ///
    /// Returns the query id of the DataPoint.
    ///
    pub fn get_query_id(&self) -> u64 {
        self.query_id
    }

    ///
    /// Returns the features of the DataPoint.
    ///
    pub fn get_features(&self) -> &Vec<Feature> {
        &self.features
    }

    ///
    /// Get a specific feature of the DataPoint.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the feature to be returned. The index starts at 1, because
    /// it's common to use the feature label as the index when indexing the features. For example,
    /// the SVM-Light format indexes each feature with a label starting at 1. In order to avoid
    /// confusion, the index starts at 1.
    ///
    ///
    /// # Returns
    ///
    /// The feature at the given index.
    ///
    pub fn get_feature(&self, index: usize) -> Result<&Feature, LtrError> {
        if index == 0 || index > self.features.len() {
            return Err(LtrError::FeatureIndexOutOfBounds(index));
        }
        Ok(&self.features[index - 1])
    }

    ///
    /// Returns the description of the DataPoint.
    ///
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    ///
    /// Set the label of the DataPoint.
    ///
    /// # Arguments
    ///
    /// * `label` - The new label of the DataPoint.
    ///
    pub fn set_label(&mut self, label: u8) {
        self.label = label;
    }

    ///
    /// Set the query id of the DataPoint.
    ///
    /// # Arguments
    ///
    /// * `query_id` - The new query id of the DataPoint.
    ///
    pub fn set_query_id(&mut self, query_id: u64) {
        self.query_id = query_id;
    }

    ///
    /// Add a feature to the DataPoint.
    ///
    /// # Arguments
    ///
    /// * `feature` - The feature to be added.
    ///
    pub fn add_feature(&mut self, feature: Feature) -> Result<(), LtrError> {
        // Sanity check
        self.features.push(feature);
        Ok(())
    }

    ///
    /// Set a feature value with a particular index to the DataPoint.
    /// This is useful when updating the features of a DataPoint.
    ///
    /// # Arguments
    ///
    /// * `index` - The index of the feature to be updated.
    /// * `feature` - The new feature value.
    ///
    pub fn set_feature(&mut self, index: usize, feature: Feature) -> Result<(), LtrError> {
        // Sanity check
        if index > self.features.len() {
            return Err(LtrError::FeatureIndexOutOfBounds(index));
        }
        self.features[index - 1] = feature;
        Ok(())
    }

    ///
    /// Set all feature values.
    ///
    /// # Arguments
    ///
    /// * `features` - The new feature values.
    ///
    pub fn set_features(&mut self, features: Vec<Feature>) -> Result<(), LtrError> {
        self.features = features;
        Ok(())
    }

    ///
    /// Set the description of the DataPoint.
    ///
    /// # Arguments
    ///
    /// * `description` - The new description of the DataPoint.
    ///
    pub fn set_description(&mut self, description: &str) {
        self.description = Some(description.to_string());
    }
}

impl fmt::Display for DataPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "DataPoint: label={}, query_id={}, features={:?}, description={:?}",
            self.label, self.query_id, self.features, self.description
        )
    }
}

///
/// A DataPoint comparison is symmetric, transitive and reflexive.
/// However, notice that the comparison is not total!
/// For example, if two DataPoints have the same label and the same query_id, but different features,
/// they are still considered equal.
impl Eq for DataPoint {}

impl PartialEq for DataPoint {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label && self.query_id == other.query_id
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

///
/// A DataPoint can be partial compared using its label.
/// This is useful when sorting DataPoints with the same query_id.
///
impl PartialOrd for DataPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.label.cmp(&other.label))
    }
}

///
/// A DataPoint can be totally compared using its label.
/// This is useful when sorting DataPoints with the same query_id.
///
impl Ord for DataPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.label.cmp(&other.label)
    }
}

///
/// Get a Feature given its feature_id.
/// Notice that this trait is unsafe because it does not check whether the feature_id is valid.
/// Be careful when using this trait.
///
impl Index<usize> for DataPoint {
    type Output = Feature;

    fn index(&self, index: usize) -> &Self::Output {
        self.get_feature(index).unwrap()
    }
}

impl ByteRpr for DataPoint {
    fn as_byte_rpr(&self, buff: &mut dyn std::io::Write) -> usize {
        let mut size = self.label.as_byte_rpr(buff) + self.query_id.as_byte_rpr(buff);

        size += (self.features.len() as u64).as_byte_rpr(buff);
        size += self.features.as_byte_rpr(buff);

        if let Some(d) = &self.description {
            size += (d.len() as u64).as_byte_rpr(buff) + d.as_byte_rpr(buff);
        } else {
            size += (0u64).as_byte_rpr(buff);
        }

        size
    }

    fn from_byte_rpr(bytes: &[u8]) -> Self {
        let label = bytes[0];

        let (start, end) = (1, 1 + u64::segment_len());
        let query_id = u64::from_byte_rpr(&bytes[start..end]);

        let (start, end) = (end, end + u64::segment_len());
        let features_len = u64::from_byte_rpr(&bytes[start..end]);

        let (start, end) = (end, end + features_len as usize);
        let features = Vec::<Feature>::from_byte_rpr(&bytes[start..end]);

        let (start, end) = (end, end + u64::segment_len());
        let description_len = u64::from_byte_rpr(&bytes[start..end]);

        let description = if description_len > 0 {
            let (start, end) = (end, end + description_len as usize);
            let description = String::from_byte_rpr(&bytes[start..end]);
            Some(description)
        } else {
            None
        };

        DataPoint {
            label,
            query_id,
            features,
            description,
        }
    }
}

///
/// A macro to create a new DataPoint.
/// This macro is useful when creating a new DataPoint with a given label, the query_id, the
/// features and the description.
/// The features are given as a vector of `Feature`s.
/// The description is optional.
///
#[macro_export]
macro_rules! dp {
    ($label:expr, $query_id:expr, $features:expr) => {
        DataPoint::new($label, $query_id, $features, None)
    };
    ($label:expr, $query_id:expr, $features:expr, $description:expr) => {
        DataPoint::new($label, $query_id, $features, Some($description))
    };
}

#[cfg(test)]
mod tests {

    use crate::fvec;

    use super::*;

    #[test]
    fn test_data_point_new() {
        let features = fvec![1.2, 3.4, 5.6];
        let mut data_point = dp!(1, 2, features.clone(), "This is a test");
        assert_eq!(data_point.get_label(), 1);
        assert_eq!(data_point.get_query_id(), 2);
        assert_eq!(data_point.get_features(), &features);
        assert_eq!(
            data_point.get_description(),
            Some(&"This is a test".to_string())
        );

        // Assert formatting
        let formatted_data_point = format!("{}", data_point);
        assert_eq!(formatted_data_point, "DataPoint: label=1, query_id=2, features=[Feature(1.2), Feature(3.4), Feature(5.6)], description=Some(\"This is a test\")");

        // Assert cloning
        let cloned_data_point = data_point.clone();

        // Assert equality
        assert_eq!(cloned_data_point, data_point);
        assert_eq!(data_point, data_point);
        assert_eq!(
            cloned_data_point,
            DataPoint::new(1, 2, fvec![0.0], Some("This is a test"))
        );
        //                            ^-- Equal to the previous DataPoint!

        // Assert inequality
        assert_ne!(
            cloned_data_point,
            DataPoint::new(2, 4, fvec![1.2, 3.4, 5.6], Some("This is a test"))
        );

        // Setting
        data_point.set_label(2);
        data_point.set_query_id(4);
        data_point.set_description("This is another test");

        assert_eq!(data_point.get_label(), 2);
        assert_eq!(data_point.get_query_id(), 4);
        assert_eq!(
            data_point.get_description(),
            Some(&"This is another test".to_string())
        );
    }

    #[test]
    fn test_update_features() {
        let mut mydp = dp!(1, 2, fvec![1.2, 3.4, 5.6], "This is a test");

        // Assert that the features are correct
        assert_eq!(mydp.get_features(), &fvec![1.2, 3.4, 5.6]);

        match mydp.get_feature(0) {
            Ok(_) => assert!(false),
            Err(er) => assert_eq!(er, LtrError::FeatureIndexOutOfBounds(0 as usize)),
        }

        mydp.add_feature(Feature::from(20.0)).unwrap();

        assert_eq!(mydp.get_feature(4), Ok(&Feature::from(20.0)));

        let snapshot = mydp.clone();

        mydp.set_feature(4, Feature::from(21.0)).unwrap();

        assert_eq!(mydp.get_feature(4), Ok(&Feature::from(21.0)));

        assert_ne!(mydp.get_features(), snapshot.get_features());
        assert_eq!(mydp, snapshot); // equal because the label is the same.

        mydp.set_label(2);

        assert!(mydp > snapshot);
    }

    #[test]
    fn test_byte_rpr() {
        let features: Vec<Feature> = fvec![1.2, 3.4, 5.6];
        {
            let original = dp!(1, 2, features.clone(), "This is a test");

            let result = DataPoint::from_byte_rpr(&original.alloc_byte_rpr());

            assert_eq!(original, result);

            assert_eq!(original.get_features(), result.get_features());

            assert_eq!(original.get_description(), result.get_description());
        }
        // {
        //     let original = dp!(2, 15, features.clone());

        //     let result = DataPoint::from_byte_rpr(&original.alloc_byte_rpr());

        //     assert_eq!(original, result);

        //     assert_eq!(original.get_features(), result.get_features());

        //     assert_eq!(original.get_description(), result.get_description());
        // }
    }
}
