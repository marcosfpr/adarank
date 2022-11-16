// Copyright (c) 2021 Marcos Pontes
// MIT License
//

///
/// Byte representation trait
///
pub mod byte_rpr;

///
/// External device segments mapping
///
pub mod segment;

///
/// Defines a core primitiive for the library: `Feature`.
/// A `Feature` is a float value useful to represent `DataPoint`s.
///
pub mod feature;

///
/// Defines a core primitive for the library: `DataPoint`.  
/// A `DataPoint` is a element of a `RankList` in a `DataSet`.
///
pub mod datapoint;
///
/// Defines a core primitive for the library: `RankList`.
/// A `RankList` is a list of `DataPoint`s and provides methods for
/// ranking them.
///
pub mod ranklist;
