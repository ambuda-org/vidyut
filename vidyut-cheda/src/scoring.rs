//! A simple statistical model.

use crate::chedaka::{Config, Phrase, TokenPool};
use crate::errors::Result;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};
use std::path::Path;
use vidyut_kosha::entries::PadaEntry;
use vidyut_prakriya::args::{Linga, Pada, Purusha, Vacana, Vibhakti};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum POSTag {
    /// A verb.
    Tinanta,
    /// A nominal.
    Subanta,
    /// An indeclinable.
    Avyaya,
    /// Unknown.
    Unknown,
}

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub(crate) struct StateCode(u8);

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum State {
    #[default]
    Initial,
    Unknown,
    Avyaya,
    Tinanta(Purusha, Vacana),
    Subanta(Linga, Vibhakti, Vacana),
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_pada(artha: &Pada) -> Self {
        match artha {
            Pada::Subanta(s) => {
                if s.is_avyaya() {
                    Self::Avyaya
                } else {
                    Self::Subanta(s.linga(), s.vibhakti(), s.vacana())
                }
            }
            Pada::Tinanta(t) => Self::Tinanta(t.purusha(), t.vacana()),
            _ => Self::Unknown,
        }
    }

    pub fn pos_tag(&self) -> POSTag {
        match self {
            Self::Subanta(_, _, _) => POSTag::Subanta,
            Self::Tinanta(_, _) => POSTag::Tinanta,
            Self::Avyaya => POSTag::Avyaya,
            _ => POSTag::Unknown,
        }
    }
}

impl<'a> From<&PadaEntry<'a>> for State {
    fn from(val: &PadaEntry) -> Self {
        match val {
            PadaEntry::Subanta(s) => State::Subanta(s.linga(), s.vibhakti(), s.vacana()),
            PadaEntry::Tinanta(t) => State::Tinanta(t.purusha(), t.vacana()),
            PadaEntry::Avyaya(_) => State::Avyaya,
            PadaEntry::Unknown => State::Unknown,
        }
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Model {
    /// Internal map of states to small codes.
    states: FxHashMap<State, StateCode>,

    // Transition model
    // --------------------------
    /// Log probability of `(state[n] | state[n-1])`
    transition_probs: FxHashMap<(StateCode, StateCode), f32>,
    /// The log probability of a transition state not seen anywhere in the training data.
    transition_log_epsilon: f32,

    // Lemma model
    // --------------------------
    string_table: FxHashMap<String, usize>,
    lemma_log_probs: FxHashMap<(usize, POSTag), f32>,
    lemma_log_p_unknown: f32,
}

/// Calculates the log probability given some numerator and denominator.
fn log_prob(num: f64, denom: f64) -> f32 {
    (num.log10() - denom.log10()) as f32
}

impl Model {
    pub fn new() -> Self {
        let mut ret = Self::default();

        let mut states = vec![State::Initial, State::Unknown, State::Avyaya];
        {
            for purusha in Purusha::iter() {
                for vacana in Vacana::iter() {
                    states.push(State::Tinanta(purusha, vacana));
                }
            }
            for linga in Linga::iter() {
                for vibhakti in Vibhakti::iter() {
                    for vacana in Vacana::iter() {
                        states.push(State::Subanta(linga, vibhakti, vacana));
                    }
                }
            }
        }
        assert!(states.len() < 256);

        ret.states = states
            .iter()
            .enumerate()
            .map(|(i, s)| (*s, StateCode(i as u8)))
            .collect();

        ret
    }

    pub fn read(model_path: &Path) -> Result<Self> {
        let file = File::open(model_path)?;
        let reader = BufReader::new(file);
        let ret: Model = rmp_serde::from_read(reader)?;

        Ok(ret)
    }

    pub fn write(&self, model_path: &Path) -> Result<()> {
        let out = rmp_serde::to_vec(&self)?;
        let file = File::create(model_path)?;
        let mut writer = BufWriter::new(file);
        writer.write_all(&out)?;

        Ok(())
    }

    pub fn lemma_log_probability(&self, text: &str, pos_tag: POSTag) -> Option<f32> {
        let id = self.string_table.get(text)?;
        let log_prob = self.lemma_log_probs.get(&(*id, pos_tag))?;
        Some(*log_prob)
    }

    /// Scores the given phrase by using lemma probabilities.
    ///
    /// We return our float score as an i32 because floats aren't hashed by default in Rust. To
    /// represent "floatness," multiply the float score by 100 so that the ones and tens places
    /// represent the tenths and hundredths places, respectively.
    pub(crate) fn score(&self, phrase: &Phrase, pool: &TokenPool) -> i32 {
        let delta = if let Some(i_last) = phrase.tokens.last() {
            let n = phrase.tokens.len();
            let prev_state = if n >= 2 {
                let i = phrase.tokens[n - 2];
                self.to_state_code(pool.get(i).expect("present").data())
            } else {
                StateCode(0)
            };

            let last = pool.get(*i_last).expect("present");
            let cur_state = self.to_state_code(last.data());

            let maybe_lemma_log_prob = (|| {
                let pada = last.data();
                let state: State = pada.into();
                self.lemma_log_probability(&pada.lemma()?, state.pos_tag())
            })();

            let lemma_log_prob = match maybe_lemma_log_prob {
                Some(p) => p,
                None => self.lemma_log_p_unknown,
            };

            let transition_log_prob = {
                let key = (prev_state, cur_state);
                match self.transition_probs.get(&key) {
                    Some(s) => *s,
                    None => self.transition_log_epsilon,
                }
            };

            lemma_log_prob + transition_log_prob
        } else {
            0.0
        };

        // To simplify the scoring, assume that:
        //
        //     P(W[0], ..., W[n]) = P(W[0], ..., W[n-1]) * P(W[n] | W[0], ..., W[n-1])
        phrase.score + (100_f32 * delta) as i32
    }

    /// Packs a PadaEntry into a `StateCode`.
    pub(crate) fn to_state_code(&self, entry: &PadaEntry) -> StateCode {
        let state: State = entry.into();
        match self.states.get(&state) {
            Some(c) => *c,
            None => StateCode(0),
        }
    }
}

#[derive(Default, Debug)]
pub struct ModelBuilder {
    /// Freq(`state[n]` | `state[n-1]`).
    ///
    /// Order in the map is `(state[n-1], state[n])`
    transitions: FxHashMap<(State, State), u32>,

    /// Freq(`lemma[n]` | `state[n]`).
    ///
    /// Ideally, we should use `token[n]` instead of `lemma[n]`. However, the DCS data doesn't
    /// realiably expose the inflected word for a given entry. Additionally, using the lemma helps us
    /// work around data sparsity.
    emissions: FxHashMap<(String, State), u32>,
}

impl ModelBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Count a transitions occurrrence.
    pub fn count_transition(&mut self, prev_state: State, cur_state: State) {
        let key = (prev_state, cur_state);
        let val = self.transitions.entry(key).or_insert(0);
        *val += 1;
    }

    /// Count an emission occurrrence.
    pub fn count_emission(&mut self, cur_state: State, lemma: String) {
        let key = (lemma, cur_state);
        let val = self.emissions.entry(key).or_insert(0);
        *val += 1;
    }

    pub fn merge(&mut self, other: ModelBuilder) {
        for (key, value) in other.transitions.into_iter() {
            let v = self.transitions.entry(key).or_insert(0);
            *v += value;
        }

        for (key, value) in other.emissions.into_iter() {
            let v = self.emissions.entry(key).or_insert(0);
            *v += value;
        }
    }

    /// Writes the model to disk.
    pub fn write_model(&self, base_path: &Path) -> Result<()> {
        let mut m = Model::new();

        // Transitions
        {
            m.transition_log_epsilon = -5.0;

            let mut priors = FxHashMap::default();
            for ((prev, _cur), value) in self.transitions.iter() {
                let v = priors.entry(prev).or_insert(0);
                *v += value;
            }

            let mut transition_probs = FxHashMap::default();
            for ((prev, cur), numerator) in self.transitions.iter() {
                let denominator = priors[prev];
                let prob = log_prob(f64::from(*numerator), f64::from(denominator));
                let prev_code = m.states[prev];
                let cur_code = m.states[cur];
                transition_probs.insert((prev_code, cur_code), prob);
            }
            m.transition_probs = transition_probs;
        }

        // Emissions
        {
            // Use a very small smoothing factor because most out-of-vocabulary tokens are errors.
            const LEMMA_EPSILON: f64 = 1e-100;

            let mut string_table = FxHashMap::default();
            for (lemma, _state) in self.emissions.keys() {
                let n = string_table.len();
                string_table.insert(lemma.to_string(), n);
            }
            m.string_table = string_table;

            let mut lemma_counts_by_pos = FxHashMap::default();
            for ((lemma, state), count) in &self.emissions {
                let id = m.string_table[lemma];
                let pos = state.pos_tag();
                let key = (id, pos);

                let val = lemma_counts_by_pos.entry(key).or_insert(0);
                *val += count;
            }

            let mut lemma_log_probs = FxHashMap::default();
            let n: u32 = lemma_counts_by_pos.values().sum();
            let num_keys = lemma_counts_by_pos.len() as u32;
            let denominator = f64::from(n + num_keys);
            for (key, numerator) in lemma_counts_by_pos {
                let smoothed_prob = log_prob(f64::from(numerator) + LEMMA_EPSILON, denominator);
                lemma_log_probs.insert(key, smoothed_prob);
            }

            m.lemma_log_p_unknown = log_prob(LEMMA_EPSILON, denominator);
            m.lemma_log_probs = lemma_log_probs;
        }

        let model_path = Config::new(base_path).model_path();
        let model_dir = model_path.parent().unwrap();

        std::fs::create_dir_all(model_dir)?;
        m.write(&model_path)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_prob() {
        assert_eq!(log_prob(10.0, 10.0), 0.0);
        assert_eq!(log_prob(10.0, 100.0), -1.0);
        assert_eq!(log_prob(10.0, 1000.0), -2.0);
    }
}
