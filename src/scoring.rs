//! Scores a parse state (higher scores are better).

use crate::segmenting::Phrase;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

pub struct Model {
    log_probs: HashMap<String, f32>,
    /// The log probability of a token not seen anywhere in the training data.
    log_p_epsilon: f32,
}

/// Calculates the log probability given some numerator and denominator.
fn log_prob(num: f64, denom: i32) -> f32 {
    let prob = num / f64::from(denom);
    prob.log10() as f32
}

impl Model {
    pub fn from_counts(counts: HashMap<String, i32>) -> Model {
        // Use a very small smoothing factor because most out-of-vocabulary tokens are errors.
        let eps: f64 = 1e-100;

        let n: i32 = counts.values().sum();
        let num_keys = counts.len() as i32;

        let log_probs: HashMap<_, f32> = counts
            .iter()
            .map(|(k, c)| {
                let smoothed_v = log_prob(f64::from(*c) + eps, n + num_keys);
                (k.clone(), smoothed_v)
            })
            .collect();

        let log_p_epsilon = log_prob(eps, n + num_keys);
        Model {
            log_probs,
            log_p_epsilon,
        }
    }

    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut counts = HashMap::new();

        let mut rdr = csv::Reader::from_path(&path)?;
        for maybe_row in rdr.records() {
            let r = maybe_row?;
            let lemma = &r[0];
            let count = r[1].parse::<i32>()?;
            counts.insert(lemma.to_string(), count);
        }

        Ok(Model::from_counts(counts))
    }

    /// Scores the given phrase by using lemma probabilities.
    ///
    /// We return our float score as an i32 because floats aren't hashed by default in Rust. To
    /// represent "floatness," multiply the float score by 100 so that the ones and tens places
    /// represent the tenths and hundredths places, respectively.
    pub fn score(&self, phrase: &Phrase) -> i32 {
        let delta = match phrase.words.last() {
            Some(last) => {
                let lemma = last.lemma();
                match self.log_probs.get(&lemma) {
                    Some(log_prob) => *log_prob,
                    None => self.log_p_epsilon,
                }
            }
            None => self.log_p_epsilon,
        };

        // To simplify the scoring, assume that:
        //
        //     P(W[0], ..., W[n]) = P(W[0], ..., W[n-1]) * P(W[n] | W[0], ..., W[n-1])
        phrase.score + (100_f32 * delta) as i32
    }
}
