use crate::errors::Result;
use crate::normalize_text;
use crate::scoring::{Model, POSTag};
use crate::sounds;
use crate::strict_mode;
use compact_str::CompactString;
use priority_queue::PriorityQueue;
use rustc_hash::FxHashMap;
use std::path::{Path, PathBuf};
use vidyut_kosha::entries::PadaEntry;
use vidyut_kosha::Kosha;
use vidyut_sandhi::{Split, Splitter};

/// A Sanskrit word and its data.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Token<'a> {
    /// The underlying text of the given word.
    pub(crate) text: CompactString,
    /// The data associated with this word.
    pub(crate) data: PadaEntry<'a>,
}

/// A small cache that stores all tokens seen during a segmentation.
#[derive(Debug, Default)]
pub(crate) struct TokenPool<'a> {
    tokens: Vec<Token<'a>>,
}

/// Represents an in-progress segment of a phrase.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Phrase {
    /// The words that we've recognized so far.
    pub tokens: Vec<usize>,
    /// The text we still need to process.
    pub remaining: String,
    /// The score associated with this in-progress solution.
    pub score: i32,
}

pub(crate) struct Config {
    vidyut_base_path: PathBuf,
}

pub struct Chedaka {
    sandhi: Splitter,
    kosha: Kosha,
    model: Model,
}

impl<'a> TokenPool<'a> {
    /// Creates a new pool.
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts the given token into the pool and returns its index.
    pub fn insert(&mut self, token: Token<'a>) -> usize {
        self.tokens.push(token);
        self.tokens.len() - 1
    }

    /// Returns the token corresponding to the given index, if one exists.
    pub fn get(&self, index: usize) -> Option<&Token> {
        self.tokens.get(index)
    }
}

impl Phrase {
    /// Create a new state.
    pub const fn new(text: String) -> Self {
        Self {
            tokens: Vec::new(),
            remaining: text,
            // log_10(1) = 0
            score: 0,
        }
    }
}

impl<'a> Token<'a> {
    /// Creates a new `Token`.
    pub fn new(text: &str, data: PadaEntry<'a>) -> Self {
        Self {
            text: CompactString::from(text),
            data,
        }
    }

    /// The plain text of this word.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// The information we have about this word.
    pub fn data(&self) -> &PadaEntry {
        &self.data
    }

    pub fn lemma(&self) -> &str {
        self.data.lemma().unwrap_or("")
    }

    pub fn pos_tag(&self) -> POSTag {
        match &self.data {
            PadaEntry::Subanta(_) => POSTag::Subanta,
            PadaEntry::Tinanta(_) => POSTag::Tinanta,
            PadaEntry::Avyaya(_) => POSTag::Avyaya,
            PadaEntry::Unknown => POSTag::Unknown,
        }
    }
}

impl Config {
    pub fn new(path: &Path) -> Self {
        Self {
            vidyut_base_path: path.to_path_buf(),
        }
    }

    pub fn sandhi_rules(&self) -> PathBuf {
        self.vidyut_base_path.join("sandhi-rules.csv")
    }

    pub fn kosha_path(&self) -> PathBuf {
        self.vidyut_base_path.join("kosha")
    }

    pub fn model_path(&self) -> PathBuf {
        self.vidyut_base_path.join("cheda/model.msgpack")
    }
}

impl Chedaka {
    /// Creates a segmenter from the given input data.
    pub fn new(path: &Path) -> Result<Self> {
        let config = Config {
            vidyut_base_path: path.to_path_buf(),
        };

        Ok(Chedaka {
            sandhi: Splitter::from_csv(config.sandhi_rules())?,
            kosha: Kosha::new(config.kosha_path())?,
            model: Model::read(&config.model_path())?,
        })
    }

    /// Returns a reference to this chedaka's underlying kosha.
    ///
    /// We provide this method so that callers who want direct access to a kosha can reuse the
    /// instance here instead of creating a new one.
    pub fn kosha(&self) -> &Kosha {
        &self.kosha
    }

    /// Returns a reference to this chedaka's underlying model.
    pub fn model(&self) -> &Model {
        &self.model
    }

    /// Segments the input text into tokens.
    pub fn run<'a>(&'a self, input_text: &str) -> Result<Vec<Token<'a>>> {
        if !input_text.is_ascii() {
            return Err(crate::Error::NonAsciiText);
        }

        let normalized_text = normalize_text::normalize(input_text);
        let initial_state = Phrase::new(normalized_text.to_string());

        let mut pq = PriorityQueue::new();
        let mut word_cache: FxHashMap<String, Vec<PadaEntry<'a>>> = FxHashMap::default();
        let mut token_pool: TokenPool<'a> = TokenPool::new();

        // viterbi_cache[remainder][state] = the best result that ends with $state and has $remainder
        // text remaining in the input.
        let mut viterbi_cache: FxHashMap<String, FxHashMap<String, Phrase>> = FxHashMap::default();

        let score = initial_state.score;
        pq.push(initial_state, score);

        while let Some((cur, cur_score)) = pq.pop() {
            // The best solution remaining is complete, so we can stop here.
            //
            // Our current scoring model is a probabilistic model that adjusts the probability of a
            // solution by multiplying it by other probabilities. Since a probability is at most 1, a
            // partial score in a probabilistic model can never increase; in practice, it will only
            // decrease as more and more terms are added.
            //
            // In other words, a solution's score can only decrease as we add more words to it.
            //
            // If we see a complete solution in our priority queue with score C, we thus know that all
            // solutions following it both (a) have a score equal or lower to C due to the nature of
            // priority queues and (b) cannot possibly produce a result better than C per our result
            // above.
            //
            // So once we find a finished solution in our priority queue, we can suspend execution.
            //
            // NOTE: this doesn't hold if using an actual Viterbi algorithm as we can suspend only once
            // we've seen each of our N possible states.
            if cur.remaining.is_empty() {
                break;
            }

            // Non-Sanskrit token: emit and continue.
            if cur.remaining.starts_with(|c| !sounds::is_sanskrit(c)) {
                let mut new = match cur.remaining.split_once(' ') {
                    Some((first, second)) => {
                        let mut new = Phrase {
                            tokens: cur.tokens.clone(),
                            remaining: second.to_string(),
                            // HACK: this is buggy -- scoring based on cur score set here?
                            score: cur_score,
                        };
                        let i = token_pool.insert(Token {
                            text: CompactString::from(first),
                            data: PadaEntry::Unknown,
                        });
                        new.tokens.push(i);
                        new
                    }
                    None => {
                        let mut new = Phrase {
                            tokens: cur.tokens.clone(),
                            remaining: "".to_string(),
                            // HACK: this is buggy -- scoring based on cur score set here?
                            score: cur_score,
                        };
                        let i = token_pool.insert(Token {
                            text: CompactString::from(cur.remaining),
                            data: PadaEntry::Unknown,
                        });
                        new.tokens.push(i);
                        new
                    }
                };

                new.score = self.model.score(&new, &token_pool);
                viterbi_cache
                    .entry(new.remaining.clone())
                    .or_default()
                    .insert("STATE".to_string(), new.clone());

                let new_score = new.score;
                pq.push(new, new_score);
                continue;
            }

            // A clumsy workaround because I'm not sure how to set up the iterator types here.
            let no_results = Vec::new();

            for split in self.sandhi.split_all(&cur.remaining) {
                if !split.is_valid() || split.is_recursive(&cur.remaining) {
                    continue;
                }

                let first = split.first();
                let second = split.second();
                self.analyze_pada(first, &split, &mut word_cache)?;

                for artha in word_cache.get(first).unwrap_or(&no_results) {
                    if !strict_mode::is_valid_word(&cur, &token_pool, &split, artha) {
                        continue;
                    }

                    let mut new = Phrase {
                        tokens: cur.tokens.clone(),
                        remaining: second.to_string(),
                        // HACK: this is buggy -- scoring based on cur score set here?
                        score: cur_score,
                    };
                    let i = token_pool.insert(Token {
                        text: CompactString::from(first),
                        data: artha.clone(),
                    });
                    new.tokens.push(i);
                    new.score = self.model.score(&new, &token_pool);

                    // Use state "STATE" for now since we don't have any states implemented.
                    let maybe_rival = viterbi_cache
                        .entry(new.remaining.clone())
                        .or_default()
                        .get("STATE");
                    let new_score = new.score;
                    if let Some(rival) = maybe_rival {
                        if rival.score >= new.score {
                            continue;
                        }
                    };
                    viterbi_cache
                        .entry(new.remaining.clone())
                        .or_default()
                        .insert("STATE".to_string(), new.clone());
                    pq.push(new, new_score);
                }
            }
        }

        // Return the best result we could find above.
        if let Some(solutions) = viterbi_cache.get("") {
            if let Some(best) = solutions.values().max_by_key(|s| s.score) {
                let mut ret = Vec::new();
                for i in &best.tokens {
                    let token = token_pool.get(*i).expect("present").clone();

                    // Do this goofy round-trip to avoid lifetime complaints.
                    let data = if token.data == PadaEntry::Unknown {
                        PadaEntry::Unknown
                    } else {
                        let packed_data = self.kosha.pack(&token.data).expect("ok");
                        self.kosha.unpack(packed_data).expect("ok")
                    };

                    let token = Token {
                        text: token.text,
                        data,
                    };
                    ret.push(token);
                }
                return Ok(ret);
            }
        }
        Ok(Vec::new())
    }

    // FIXME: better as an iterator, but hard to implement. For now, update statefully then iterate in
    // caller.
    fn analyze_pada<'a>(
        &'a self,
        text: &str,
        split: &Split,
        cache: &mut FxHashMap<String, Vec<PadaEntry<'a>>>,
    ) -> Result<()> {
        if !cache.contains_key(text) {
            let mut res = self.kosha.get_all(text);

            // Add the option to skip an entire chunk. (For typos, junk, etc.)
            if split.is_end_of_chunk() || text.starts_with(|c| !sounds::is_sanskrit(c)) {
                res.push(PadaEntry::Unknown);
            }

            cache.insert(text.to_string(), res);
        };
        Ok(())
    }
}
