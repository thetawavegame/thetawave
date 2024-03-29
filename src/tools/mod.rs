//! Helper functions
use rand::{thread_rng, Rng};

/// Randomly picks index of vector using weights
/// Takes in a vector of weights
pub fn weighted_rng(probs: Vec<f32>) -> usize {
    let prob_space = probs.iter().fold(0.0, |sum, prob| sum + prob);
    let pos = thread_rng().gen::<f32>() * prob_space;
    let mut sum = 0.0;
    for (idx, prob) in probs.iter().enumerate() {
        sum += prob;
        if sum > pos {
            return idx;
        }
    }
    unreachable!("Error in probabilities.");
}

/// Signed modulo function
pub fn signed_modulo(a: f32, n: f32) -> f32 {
    a - (a / n).floor() * n
}
