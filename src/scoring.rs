//! Scores a tagged Sanskrit sentence.

use crate::segmenting::Phrase;
use core::str::FromStr;
use modular_bitfield::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::num::ParseIntError;
use std::path::Path;
use vidyut_kosha::semantics::POSTag;
use vidyut_kosha::semantics::*;

/// Models a Markov transition state.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct State(u16);

impl State {
    pub fn initial() -> Self {
        PadaState::initial_state().into_state()
    }
    pub fn from_pada(s: &Pada) -> Self {
        PadaState::from_pada(s).into_state()
    }
}

impl ToString for State {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl FromStr for State {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

/// Models the transition state for some *subanta*.
///
/// The state describes linga, vibhakti, and vacana, which are sufficient for our current needs.
#[bitfield(bits = 14)]
pub struct SubantaState {
    #[skip(getters)]
    unused: B5,
    #[skip(getters)]
    linga: Linga,
    #[skip(getters)]
    vacana: Vacana,
    #[skip(getters)]
    vibhakti: Vibhakti,
    #[skip(getters)]
    is_purvapada: bool,
}

/// Models the transition state for some *tinanta*.
///
/// The state describes purusha and vacana, which are sufficient for our current needs.
#[bitfield(bits = 14)]
pub struct TinantaState {
    #[skip(getters)]
    unused: B10,
    #[skip(getters)]
    purusha: Purusha,
    #[skip(getters)]
    vacana: Vacana,
}

/// Models the transition state for some *pada*.
#[bitfield(bits = 16)]
pub struct PadaState {
    #[skip(getters)]
    pos: POSTag,
    #[skip(getters)]
    payload: B14,
}

impl PadaState {
    /// Creates the initial state.
    pub fn initial_state() -> Self {
        PadaState::new().with_pos(POSTag::None)
    }

    /// Creates a state label for the given pada.
    pub fn from_pada(p: &Pada) -> Self {
        let zero = [0_u8; 2];
        let (pos_tag, payload) = match p {
            Pada::None => (POSTag::None, zero),
            Pada::Subanta(s) => {
                let bytes = SubantaState::new()
                    .with_linga(s.linga)
                    .with_vacana(s.vacana)
                    .with_vibhakti(s.vibhakti)
                    .with_is_purvapada(s.is_purvapada)
                    .into_bytes();
                (POSTag::Subanta, bytes)
            }
            Pada::Tinanta(s) => {
                let bytes = TinantaState::new()
                    .with_purusha(s.purusha)
                    .with_vacana(s.vacana)
                    .into_bytes();
                (POSTag::Tinanta, bytes)
            }
            Pada::Avyaya(_) => (POSTag::Avyaya, zero),
        };
        PadaState::new()
            .with_pos(pos_tag)
            .with_payload(u16::from_le_bytes(payload))
    }

    /// Packs the given semantics into a `State`.
    pub fn into_state(self) -> State {
        let val = u16::from_le_bytes(self.into_bytes());
        State(val)
    }
}

/// Calculates the log probability given some numerator and denominator.
fn log_prob(num: f64, denom: i32) -> f32 {
    let prob = num / f64::from(denom);
    prob.log10() as f32
}

type LemmaMap<T> = HashMap<(String, POSTag), T>;

struct LemmaModel {
    /// Log probability that a (lemma, pos) appears.
    log_probs: HashMap<(String, POSTag), f32>,
    /// The log probability of a token not seen anywhere in the training data.
    log_p_unknown: f32,
}

impl LemmaModel {
    fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut counts = HashMap::new();

        let mut rdr = csv::Reader::from_path(&path)?;
        for maybe_row in rdr.records() {
            let r = maybe_row?;
            let lemma = &r[0];
            let pos_tag = r[1].parse()?;
            let count = r[2].parse::<i32>()?;
            counts.insert((lemma.to_string(), pos_tag), count);
        }

        let (log_probs, log_p_unknown) = Self::to_log_probs(counts);
        Ok(Self {
            log_probs,
            log_p_unknown,
        })
    }

    fn to_log_probs(counts: LemmaMap<i32>) -> (LemmaMap<f32>, f32) {
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

        (log_probs, log_p_epsilon)
    }

    fn log_prob(&self, lemma: String, pos_tag: POSTag) -> f32 {
        match self.log_probs.get(&(lemma, pos_tag)) {
            Some(log_prob) => *log_prob,
            None => self.log_p_unknown,
        }
    }
}

struct TransitionModel {
    /// Log probability of (state[n] | state[n-1])
    log_probs: HashMap<(State, State), f32>,
    /// The log probability of a transition state not seen anywhere in the training data.
    log_epsilon: f32,
}

impl TransitionModel {
    fn new(path: &Path) -> Result<Self, Box<dyn Error>> {
        type Key = (State, State);

        let mut log_probs = HashMap::new();
        let mut rdr = csv::Reader::from_path(path)?;
        for maybe_row in rdr.records() {
            let row = maybe_row?;

            let key: Key = (row[0].parse()?, row[1].parse()?);
            let prob = row[2].parse::<f32>()?;

            log_probs.insert(key, prob.log10());
        }

        Ok(Self {
            log_probs,
            // FIXME: calculate this properly.
            log_epsilon: -5.0,
        })
    }

    fn log_prob(&self, prev: &State, cur: &State) -> f32 {
        let key = (*prev, *cur);
        match self.log_probs.get(&key) {
            Some(s) => *s,
            None => self.log_epsilon,
        }
    }
}

pub struct Model {
    lemmas: LemmaModel,
    transitions: TransitionModel,
}

impl Model {
    pub fn new(lemma_counts_path: &Path, transitions_path: &Path) -> Result<Self, Box<dyn Error>> {
        let lemmas = LemmaModel::new(lemma_counts_path)?;
        let transitions = TransitionModel::new(transitions_path)?;

        Ok(Model {
            lemmas,
            transitions,
        })
    }

    /// Scores the given phrase by using lemma probabilities.
    ///
    /// We return our float score as an i32 because floats aren't hashed by default in Rust. To
    /// represent "floatness," multiply the float score by 100 so that the ones and tens places
    /// represent the tenths and hundredths places, respectively.
    pub fn score(&self, phrase: &Phrase) -> i32 {
        let n = phrase.words.len();
        let delta = if let Some(last) = phrase.words.last() {
            let prev_state = if n >= 2 {
                State::from_pada(&phrase.words[n - 2].semantics)
            } else {
                State::initial()
            };

            let cur_state = State::from_pada(&last.semantics);

            let pada = &last.semantics;
            let lemma_log_prob = self
                .lemmas
                .log_prob(pada.lemma(), pada.part_of_speech_tag());
            let transition_log_prob = self.transitions.log_prob(&prev_state, &cur_state);
            lemma_log_prob + transition_log_prob
        } else {
            0.0
        };

        // To simplify the scoring, assume that:
        //
        //     P(W[0], ..., W[n]) = P(W[0], ..., W[n-1]) * P(W[n] | W[0], ..., W[n-1])
        phrase.score + (100_f32 * delta) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_prob() {
        assert_eq!(log_prob(10.0, 10), 0.0);
        assert_eq!(log_prob(10.0, 100), -1.0);
        assert_eq!(log_prob(10.0, 1000), -2.0);
    }
}
