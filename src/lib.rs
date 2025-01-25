//! # AdaRank: a boosting algorithm for information retrieval
//!
//! AdaRank is a popular learning to rank algorithm that is based on the AdaBoost algorithm.
//! See the [original paper](https://dl.acm.org/doi/10.1145/1277741.1277809) for more details.
//!
//! AdaRank is a boosting algorithm that is used to learn a ranking function from a set of
//! features. The algorithm is based on the AdaBoost algorithm, which is a popular ensemble
//! method that is used to learn a strong classifier from a set of weak classifiers.
//!
//!
//! ```ignore
//! use adarank::AdaRank;
//! use adarank::eval::map::MAP;
//! use adarank::loader::svmlight::SVMLight;
//!
//! let corpus = std::path::Path::new("benchmarks/OHSUMED").join("Data/All/OHSUMED.txt");
//!
//!
//! // Load a SVMLight dataset.
//! let ohsumed_dataset = SVMLight::load(corpus.to_str().unwrap()).unwrap();  
//! // Clone a `RankList` to test later...
//! let test_sample = ohsumed_dataset[0].clone();
//!    
//! // Create an AdaRank learner with MAP as the evaluation metric, 50 iterations,
//! // 3 max consecutive selections, and 0.003 tolerance.
//! let mut adarank = AdaRank::new(ohsumed_dataset, Box::new(MAP), 50, 3, 0.003, None, None);
//!    
//!
//! // Fit the learner to the dataset.
//! adarank.fit().unwrap();
//!    
//! // Get the test `DataPoint` from the `RankList`.
//! let dp = test_sample.get(0).unwrap();
//!
//!
//! // Predict the score for the test `DataPoint`.
//! let doc_label = adarank.predict(&test_sample.get(0).unwrap());
//! println!("Document {} has the score {:.2} for query {}",
//!                dp.get_description().unwrap(),
//!                doc_label,
//!                dp.get_query_id());
//! ```
//!
//!
//!
//! A good place for you to get started is to check out
//! the example [source code](https://github.com/marcosfpr/ltrs/blob/master/examples/ohsumed.rs))

/// Define a core primitive for the library: `DataPoint`.  
/// A `DataPoint` is a element of a `RankList` in a `DataSet`.
pub mod datapoint;

/// Define a core primitive for the library: `RankList`.
/// A `RankList` is a list of `DataPoint`s and provides methods for
/// ranking them.
pub mod ranklist;

/// Define the error type for the library.
pub mod error;

/// Define evaluators for the library.
/// Evaluators are used to evaluate the performance of a `Learner`.
pub mod eval;

/// Utility functions for the library.  
/// These functions are not part of the core API, but are useful inside the library.
pub mod utils;

/// Define the loader for the library. A `Loader` is used to load a `DataSet` from a
/// IO stream.
pub mod loader;

/// Define the `Ranker` primitive. All AI algorithms in the library are `Ranker`s,
/// which means they can be used to predict the score of `DataPoint`s in `RankList`s .
pub mod ranker;

/// Define the `Learner` primitive. A `Learner` is define operationss required
/// to train a `Ranker`.
pub mod learner;

/// Define a class of `Ranker`s based on ensemble methods.
pub mod ensemble;

/// A particular Feature for lt.rs is just a floating point value.
/// The feature_value is the value of the feature.
type Feature = f32;

/// For simplicity, we will use a DataSet as a vector of RankLists.
pub type DataSet = Vec<ranklist::RankList>;
