/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use crate::datapoint::DataPoint;
use crate::error::LtrError;
use crate::ranklist::RankList;
use crate::DataSet;

///
/// The default implementation of SVMLight parsing
/// is based on the SVM-light format.
///
/// The format is as follows:
/// <label> qid:<qid> <index1>:<value1> <index2>:<value2> ... # <info>
pub struct SVMLight;

impl SVMLight {
    pub fn load_datapoint(buffer: &str) -> Result<DataPoint, LtrError> {
        // Convert the buffer into a string
        let mut data_point = DataPoint::empty();

        // Find # to extract the  optional description.
        let mut buffer_iter = buffer.split('#');
        let buffer_str = buffer_iter.next().ok_or(LtrError::ParseError("Error in SVMLight::load_datapoint: Description processing failure."))?;
        if let Some(info) = buffer_iter.next() {
            data_point.set_description(info.trim());
        }

        let mut iter = buffer_str.trim().split(' '); // Split on spaces

        // Get the label
        let label = iter
            .next()
            .ok_or(LtrError::InvalidDataPoint("Missing the label parameter."))?;
        data_point.set_label(
            label
                .parse::<u8>()
                .map_err(|_| LtrError::InvalidDataPoint("Invalid label parameter."))?,
        );

        // Get the qid:<qid>
        let qid = iter
            .next()
            .ok_or(LtrError::InvalidDataPoint("Missing the qid parameter."))?;

        let mut qid_iter = qid.split(':');
        qid_iter.next().ok_or(LtrError::ParseError("Error in SVMLight::load_datapoint: Query ID processing failure."))?; // Skip the qid:
        let qid_str = qid_iter.next().ok_or(LtrError::ParseError("Error in SVMLight::load_datapoint: Query ID processing failure"))?;
        let qid = qid_str
            .parse::<u32>()
            .map_err(|_| LtrError::InvalidDataPoint("Invalid qid parameter."))?;

        data_point.set_query_id(qid);

        // Get the features
        let mut feature_values = Vec::new();
        for feature in iter {
            let mut feature_iter = feature.split(':');

            let index = feature_iter
                .next()
                .ok_or(LtrError::InvalidDataPoint("Missing feature index."))?
                .parse::<usize>()
                .map_err(|_| LtrError::InvalidDataPoint("Invalid feature index."))?;

            let value = feature_iter
                .next()
                .ok_or(LtrError::InvalidDataPoint("Missing feature value."))?
                .parse::<f32>()
                .map_err(|_| LtrError::InvalidDataPoint("Invalid feature value."))?;

            if index > feature_values.len() {
                feature_values.resize(index as usize, 0.0);
            }

            feature_values[index - 1] = value;
        }

        data_point.set_features(feature_values)?;

        Ok(data_point)
    }

    ///
    /// Load a RankList from a SVM-Light buffer.
    /// Notice that this method DOES NOT check whether the RankList has
    /// different query ids. If you're not sure, use the `load_dataset` method.
    ///
    /// The format is as follows:
    /// <datapoint1>\n
    /// <datapoint2>\n
    /// ...
    /// <datapointN>\n
    ///
    pub fn load_ranklist(buffer: &str) -> Result<RankList, LtrError> {
        let mut data_points = Vec::new();
        let mut buffer_iter = buffer.split('\n');
        while let Some(line) = buffer_iter.next() {
            if line.is_empty() {
                continue;
            }

            let data_point = SVMLight::load_datapoint(line)?;
            data_points.push(data_point);
        }

        Ok(RankList::new(data_points))
    }

    ///
    /// Load a DataSet from a SVM-Light buffer.
    /// May panic if the buffer is invalid.
    ///
    pub fn load_dataset(buffer: &str) -> Result<DataSet, LtrError> {
        let mut buffer_iter = buffer.split('\n');
        let mut dataset: DataSet = DataSet::new();

        let mut current_query_id = 0;
        let mut current_rank_list = Vec::new();

        while let Some(line) = buffer_iter.next() {
            if line.is_empty() {
                continue;
            }
            let dp = SVMLight::load_datapoint(line)?;
            if dp.get_query_id() != current_query_id || current_query_id == 0 {
                current_query_id = dp.get_query_id();
                current_rank_list.push(dp);
            } else {
                // Different query id, so we need to add the current rank list to the dataset
                let ranklist = RankList::new(current_rank_list.clone());
                dataset.push(ranklist);
                current_rank_list.clear();

                current_query_id = dp.get_query_id();
                current_rank_list.push(dp);
            }
        }
        // Add the last rank list
        let ranklist = RankList::new(current_rank_list);
        dataset.push(ranklist);

        Ok(dataset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_svm_light_parser() {
        let buffer = "1 qid:10 1:21.00 2:2.30 3:4.50 # desc";
        let data_point = SVMLight::load_datapoint(buffer).unwrap();

        assert_eq!(data_point.get_label(), 1);
        assert_eq!(data_point.get_query_id(), 10);
        assert_eq!(data_point.get_description(), Some(&"desc".to_string()));
        assert_eq!(data_point.get_features().len(), 3);
        assert_eq!(*data_point.get_feature(1).unwrap(), 21.0f32);
        assert_eq!(*data_point.get_feature(2).unwrap(), 2.3f32);
        assert_eq!(*data_point.get_feature(3).unwrap(), 4.5f32);
    }
}
