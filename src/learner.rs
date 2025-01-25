use crate::{error::LtrError, eval::Evaluator, ranker::Ranker, DataSet};

/// This trait represents the basic behavior for
/// all models implemented in the lt.rs crate.
///
/// The models should be able to fit a `DataSet` and
/// rank a `DataPoint`.
pub trait Learner:
    Ranker + DatasetConfigurable + MetricConfigurable + FeaturesConfigurable
{
    /// Fit a `DataSet` to the model.
    ///
    /// # Returns
    /// * `Ok(())` if the model was fitted successfully.
    ///
    /// # Errors
    /// `LtrError` if the model could not be fitted.
    fn fit(&mut self) -> Result<(), LtrError>;

    /// The `Learner`s should retrieve the training process score.
    ///
    /// # Returns
    /// The training process score.
    ///
    /// # Errors
    /// `LtrError` if the score could not be calculated.
    fn score(&self) -> Result<f32, LtrError>;

    /// The `Learner`s should retrieve the validation process score.
    ///
    /// # Returns
    /// The validation process score.
    ///
    /// # Errors
    /// `LtrError` if the score could not be calculated.
    fn validation_score(&self) -> Result<f32, LtrError>;
}

/// The `Learner`s should have a training and a optional validation `DataSet`.
///
/// The training `DataSet` is used to train the model.
/// The validation `DataSet` is used to validate the model during training.
pub trait DatasetConfigurable {
    /// Set the training `DataSet`.
    ///
    /// # Arguments
    /// * `dataset` - The training `DataSet`.
    fn set_train_dataset(&mut self, dataset: DataSet);

    /// The `Learner`s should have a training and a optional validation `DataSet`.
    ///
    /// The training `DataSet` is used to train the model.
    /// The validation `DataSet` is used to validate the model during training.
    fn set_validation_dataset(&mut self, dataset: DataSet);
}

/// The `Learner`s should allow the user to customize
/// the metrics used to evaluate the model.
pub trait MetricConfigurable {
    /// Set the metric used to evaluate the model.
    ///
    /// # Arguments
    /// * `metric` - The metric used to evaluate the model.
    ///
    /// # Errors
    /// `LtrError` if the metric could not be set.
    fn set_metric(&mut self, metric: Box<dyn Evaluator>);
}

/// The `Learner`s should allow the user to customize
/// the features used to train the model.
pub trait FeaturesConfigurable {
    /// Set the features used to train the model.
    ///
    /// # Arguments
    /// * `features` - The features used to train the model.
    fn set_features(&mut self, features: Vec<usize>);
}

/// The `Learner`s should allow the user to save the model to a file.
pub trait FileSerializable {
    /// Save the model to a file.
    ///
    /// # Arguments
    /// * `path` - The path to save the model.
    ///
    /// # Returns
    /// `Ok(())` if the model was saved successfully.
    ///
    /// # Errors
    /// `LtrError` if the model could not be saved.
    ///
    fn save_to_file(&self, path: &str) -> Result<(), LtrError>;

    /// Load the model from a file.
    ///
    /// # Arguments
    /// * `path` - The path to load the model.
    ///
    /// # Returns
    /// `Ok(())` if the model was loaded successfully.
    ///
    /// # Errors
    /// `LtrError` if the model could not be loaded.
    fn load_from_file(&mut self, path: &str) -> Result<(), LtrError>;
}
