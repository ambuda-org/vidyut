/*!
WebAssembly bindings for vidyut-sandhi.

Exposes sandhi rule generation and a simple join function for applying
external sandhi to pairs of SLP1-encoded words.
*/
use crate::generator::generate_rules;
use serde::Serialize;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn error(s: &str);
}

#[derive(Serialize)]
struct WebRule {
    first: String,
    second: String,
    result: String,
}

/// Sandhi engine that joins SLP1-encoded words.
#[wasm_bindgen]
pub struct Sandhi {
    rules: Vec<crate::Rule>,
}

#[wasm_bindgen]
impl Sandhi {
    /// Initialize the sandhi engine by generating all rules.
    pub fn init() -> Sandhi {
        console_error_panic_hook::set_once();
        Sandhi {
            rules: generate_rules(),
        }
    }

    /// Join two SLP1-encoded words using external sandhi rules.
    /// Returns the combined string after applying the best matching rule.
    pub fn join(&self, first: &str, second: &str) -> String {
        if first.is_empty() {
            return second.to_string();
        }
        if second.is_empty() {
            return first.to_string();
        }

        // Try matching rules with longest first/second parts first
        let mut best: Option<(usize, usize, &str)> = None;

        for rule in &self.rules {
            let rf = rule.first();
            let rs = rule.second();

            if rf.is_empty() || rs.is_empty() {
                continue;
            }

            if first.ends_with(rf) && second.starts_with(rs) {
                let total_match = rf.len() + rs.len();
                if best.is_none() || total_match > best.unwrap().0 + best.unwrap().1 {
                    best = Some((rf.len(), rs.len(), rule.result()));
                }
            }
        }

        match best {
            Some((flen, slen, result)) => {
                let prefix = &first[..first.len() - flen];
                let suffix = &second[slen..];
                format!("{}{}{}", prefix, result, suffix)
            }
            None => {
                // No sandhi rule applies; just concatenate
                format!("{}{}", first, second)
            }
        }
    }

    /// Get all sandhi rules as JSON.
    pub fn rules(&self) -> JsValue {
        let web_rules: Vec<WebRule> = self
            .rules
            .iter()
            .map(|r| WebRule {
                first: r.first().to_string(),
                second: r.second().to_string(),
                result: r.result().to_string(),
            })
            .collect();
        serde_wasm_bindgen::to_value(&web_rules).unwrap_or(JsValue::NULL)
    }
}
