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
//! ```ignore
//! use ltrs::{
//! ensemble::adarank::AdaRank,
//! eval::map::MAP,
//! learner::Learner,
//! loader::{LtrFormat, svmlight::SVMLight},
//! ranker::Ranker,
//! };
//!
//! fn main() {
//!  
//!      // Let's load a dataset from the ohsumed corpus
//!      let corpus = std::path::Path::new("benchmarks/OHSUMED").join("Data/All/OHSUMED.txt");
//!
//!
//!      // Load a SVMLight dataset.
//!      let ohsumed_dataset = SVMLight::load(corpus.to_str().unwrap()).unwrap();  
//!      // Clone a `RankList` to test later...
//!      let test_sample = ohsumed_dataset[0].clone();
//!         
//!      // Create an AdaRank learner with MAP as the evaluation metric, 50 iterations,
//!      // 3 max consecutive selections, and 0.003 tolerance.
//!      let mut adarank = AdaRank::new(ohsumed_dataset, Box::new(MAP), 50, 3, 0.003, None, None);
//!         
//!
//!      // Fit the learner to the dataset.
//!      adarank.fit().unwrap();
//!         
//!      // Get the test `DataPoint` from the `RankList`.
//!      let dp = test_sample.get(0).unwrap();
//!
//!      
//!      // Predict the score for the test `DataPoint`.
//!      let doc_label = adarank.predict(&test_sample.get(0).unwrap());
//!      println!("Document {} has the score {:.2} for query {}",
//!                dp.get_description().unwrap(),
//!                doc_label,
//!                dp.get_query_id());
//!      
//! }
//!
//! ```
//!
//!
//!
//! A good place for you to get started is to check out
//! the example code (
//! [source code](https://github.com/marcosfpr/ltrs/blob/master/examples/ohsumed.rs))

///
/// Define a the core memory definitions for  this LTR framework.
///
pub mod memory_system;

///
/// Define the error type for the library.
///
pub mod error;

///
/// Define evaluators for the library.
/// Evaluators are used to evaluate the performance of a `Learner`.
///
pub mod eval;

///
/// Utility functions for the library.  
/// These functions are not part of the core API, but are useful inside the library.
///
pub mod utils;

///
/// Define the loader for the library. A `Loader` is used to load a `DataSet` from a
/// IO stream.
///
pub mod loader;

///
/// Define the `Ranker` primitive. All AI algorithms in the library are `Ranker`s,
/// which means they can be used to predict the score of `DataPoint`s in `RankList`s .
///
pub mod ranker;

///
/// Define the `Learner` primitive. A `Learner` is define operationss required
/// to train a `Ranker`.
///
pub mod learner;

///
/// Define a class of `Ranker`s based on ensemble methods.
///
pub mod ensemble;
