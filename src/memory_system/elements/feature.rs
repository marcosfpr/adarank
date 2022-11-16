/// Copyright (c) 2021 Marcos Pontes
// This code is licensed under MIT license (see LICENSE for details)
use super::byte_rpr::{ByteRpr, FixedByteLen};
use rand::distributions::uniform::{SampleBorrow, SampleUniform, UniformFloat, UniformSampler};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::num::ParseFloatError;
use std::ops::Deref;
use std::ops::DerefMut;
use std::str::FromStr;

///
/// A particular Feature for lt.rs is just a floating point value.
/// The feature_value is the value of the feature.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Feature(pub f32);

impl Feature {
    ///
    /// Creates a new feature given a f32 value
    ///
    pub fn new(value: f32) -> Feature {
        Feature(value)
    }
}

impl PartialEq for Feature {
    fn eq(&self, other: &Self) -> bool {
        self.0 - other.0 < f32::EPSILON
    }
}

impl PartialOrd for Feature {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Deref for Feature {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Feature {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl ByteRpr for Feature {
    fn as_byte_rpr(&self, buff: &mut dyn std::io::Write) -> usize {
        self.0.as_byte_rpr(buff)
    }

    fn from_byte_rpr(bytes: &[u8]) -> Self {
        Feature(f32::from_byte_rpr(bytes))
    }
}

impl FixedByteLen for Feature {
    fn segment_len() -> usize {
        f32::segment_len()
    }
}

impl From<f32> for Feature {
    fn from(v: f32) -> Self {
        Feature(v)
    }
}

impl FromStr for Feature {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match f32::from_str(s) {
            Ok(v) => Ok(Feature(v)),
            Err(e) => Err(e),
        }
    }
}

impl Into<f32> for Feature {
    fn into(self) -> f32 {
        self.0
    }
}

///
/// Generates uniform distribution for `Feature`s.
///
#[derive(Clone, Copy, Debug)]
pub struct UniformFeature(UniformFloat<f32>);

impl UniformSampler for UniformFeature {
    type X = Feature;
    fn new<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        UniformFeature(UniformFloat::<f32>::new(low.borrow().0, high.borrow().0))
    }
    fn new_inclusive<B1, B2>(low: B1, high: B2) -> Self
    where
        B1: SampleBorrow<Self::X> + Sized,
        B2: SampleBorrow<Self::X> + Sized,
    {
        UniformFeature(UniformFloat::<f32>::new_inclusive(
            low.borrow().0,
            high.borrow().0,
        ))
    }
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Self::X {
        Feature(self.0.sample(rng))
    }
}

impl SampleUniform for Feature {
    type Sampler = UniformFeature;
}

///
/// Creates a feature vector containing the arguments
///
#[macro_export]
macro_rules! fvec {
    () => {
        std::vec::Vec::<Feature>::new()
    };
    ($elem:expr; $n:expr) => {
        std::vec::from_elem(Feature($elem), $n)
    };
    ($($x:expr),+ $(,)?) => { (vec![$(crate::memory_system::elements::feature::Feature($x)),*]) }
}
