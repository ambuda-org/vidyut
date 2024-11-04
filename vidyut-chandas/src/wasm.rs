use crate as rs;
extern crate console_error_panic_hook;
use serde::Serialize;

use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

#[derive(Serialize)]
pub enum Weight {
    G,
    L,
}

impl From<rs::Weight> for Weight {
    fn from(w: rs::Weight) -> Self {
        match w {
            rs::Weight::G => Self::G,
            rs::Weight::L => Self::L,
        }
    }
}

#[derive(Serialize)]
pub struct Akshara {
    text: String,
    weight: Weight,
}

impl From<rs::Akshara> for Akshara {
    fn from(a: rs::Akshara) -> Self {
        Self {
            text: a.text().to_string(),
            weight: a.weight().into(),
        }
    }
}

#[derive(Serialize)]
pub enum MatchType {
    None,
    Prefix,
    Pada,
    Full,
}

impl From<rs::MatchType> for MatchType {
    fn from(m: rs::MatchType) -> Self {
        match m {
            rs::MatchType::None => Self::None,
            rs::MatchType::Prefix => Self::Prefix,
            rs::MatchType::Pada => Self::Pada,
            rs::MatchType::Full => Self::Full,
        }
    }
}

#[allow(non_snake_case)]
#[wasm_bindgen]
#[derive(Serialize)]
pub struct Match {
    vrtta: Option<String>,
    matchType: MatchType,
    aksharas: Vec<Vec<Akshara>>,
}

impl From<rs::Match> for Match {
    fn from(m: rs::Match) -> Self {
        let mut aksharas = Vec::new();
        for rs_row in m.aksharas() {
            let mut row = Vec::new();
            for a in rs_row {
                row.push(a.clone().into());
            }
            aksharas.push(row);
        }
        Match {
            vrtta: m.padya().as_ref().map(|v| v.name().to_string()),
            matchType: m.match_type().into(),
            aksharas,
        }
    }
}

#[wasm_bindgen]
extern "C" {
    /// Exposes `console.error` in case we need to log anything to the JS console.
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn error(s: &str);
}

#[wasm_bindgen]
pub struct Chandas(rs::Chandas);

#[wasm_bindgen]
impl Chandas {
    pub fn init(vrtta_text: &str) -> Self {
        // Logs panics to the console. Without this, panics are logged as "RuntimeError:
        // Unreachable executed", which is not useful.
        console_error_panic_hook::set_once();

        Self(rs::Chandas::from_text(vrtta_text).expect("should be well-formed"))
    }

    pub fn classify(&self, text: &str) -> JsValue {
        let res: Match = self.0.classify(text).into();
        serde_wasm_bindgen::to_value(&res).expect("wasm")
    }
}
