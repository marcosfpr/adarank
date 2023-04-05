/// Copyright (c) 2021 Marcos Pontes
/// MIT License
///
use rand;

///! Utility function to generating random data.
pub fn randomize<D, T>(distribution: D, times: usize) -> Vec<T>
where
    D: rand::distributions::Distribution<T>,
{
    let mut rng = rand::thread_rng();
    let mut vec = Vec::new();
    for _ in 0..times {
        vec.push(distribution.sample(&mut rng));
    }
    vec
}

///
/// Generates a random uniform distribution of length `len`.
///
pub fn randomize_uniform<T>(min: T, max: T, times: usize) -> Vec<T>
where
    T: rand::distributions::uniform::SampleUniform + Copy,
{
    let d = rand::distributions::Uniform::new(min, max);
    randomize(d, times)
}
