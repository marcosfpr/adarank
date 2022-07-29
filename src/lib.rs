#![doc(html_logo_url = "")]
#![cfg_attr(all(feature = "unstable", test), feature(test))]
#![cfg_attr(
    feature = "cargo-clippy",
    allow(
        clippy::module_inception,
        clippy::needless_range_loop,
        clippy::bool_assert_comparison
    )
)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![warn(missing_docs)]
#![allow(clippy::len_without_is_empty)]

//! # `lt.rs`
//!
//! ltrs is a Learning to Rank library.
//! Think `RankLib`, but in Rust.
//!
//! ```rust
//! use ltrs::{
//!     ensemble::adarank::AdaRank,
//!     eval::{map::MAP, precision::Precision},
//!     learner::Learner,
//!     loader::LtrFormat,
//!     utils::logging::init_logger,
//!     DataSet,
//! };
//! 
//! fn main() {
//!      init_logger();
//!      // Let's load a dataset from the ohsumed corpus
//!      let corpus = std::path::Path::new("benchmarks/OHSUMED").join("Data/All/OHSUMED.txt");
//! 
//!      if corpus.exists() {
//!         log::info!("Loading corpus from {}", corpus.display());
//! 
//!         // Load the dataset from the SVMLight format
//!         let ohsumed_dataset: DataSet = ltrs::loader::svmlight::SVMLight::load(
//!             corpus.to_str().unwrap()
//!         ).unwrap();
//!
//!         // Cloning a `RankList` to test later...
//!         let test_sample = ohsumed_dataset[0].clone();
//!         
//!         // Create an AdaRank learner with MAP as the evaluation metric, 50 iterations,
//!         // 3 max consecutive selections, and 0.003 tolerance.
//!         let mut adarank = AdaRank::new(ohsumed_dataset, Box::new(MAP), 50, 3, 0.003, None, None);
//!         
//!         // Fit the learner to the dataset.
//!         adarank.fit().unwrap();
//! 
//!         log::debug!("Finished fitting.");
//!         
//!         // Get the test `DataPoint` from the `RankList`.
//!         let dp = test_sample.get(0).unwrap();
//!
//!         // Predict the score for the test `DataPoint`.
//!         let doc_label = adarank.predict(&test_sample.get(0).unwrap());
//!         log::info!("Document {} has the score {:.2} for query {}",
//!                     dp.get_description().unwrap(),
//!                     doc_label, 
//!                     dp.get_query_id());
//!      }
//! }
//! 
//! ```
//!
//!
//!
//! A good place for you to get started is to check out
//! the example code (
//! [source code](https://github.com/marcosfpr/ltrs/blob/master/examples/ohsumed.rs))

#[macro_use]
extern crate lazy_static;

pub mod datapoint;
pub mod ensemble;
pub mod error;
pub mod eval;
pub mod learner;
pub mod loader;
pub mod ranker;
pub mod ranklist;
pub mod utils;

///
/// A particular Feature for lt.rs is just a floating point value.
/// The feature_value is the value of the feature.
type Feature = f32;

///
/// For simplicity, we will use a DataSet as a vector of RankLists.
///
pub type DataSet = Vec<ranklist::RankList>;
