/*!
WebAssembly bindings for vidyut-prakriya.

Since vidyut-prakriya is lightweight and has few dependencies, we can create WebAssembly bindings
for it quite easily. In development, we recommend using these bindings through the `make debugger`
command, which will also start a small Python webserver.

The main struct of interest is `Vidyut`, which wraps all of vidyut-prakriya's high-level APIs.

Although these bindings are usable and reliable, we want to improve their ergonomics so that
JavaScript callers can use them more idiomatically.

Useful links:
- Rust and WebAssembly book: <https://rustwasm.github.io/docs/book/introduction.html>
- wasm-pack book: <https://rustwasm.github.io/docs/wasm-pack/>
- wasm-bindgen book: <https://rustwasm.github.io/wasm-bindgen/introduction.html>
*/
use crate::args::*;
use crate::core::Rule;
use crate::core::{Prakriya, Step, StepTerm};
use crate::dhatupatha::Dhatupatha;
use serde::{Deserialize, Serialize};
extern crate console_error_panic_hook;

use crate::Vyakarana;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Exposes `console.error` in case we need to log anything to the JS console.
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn error(s: &str);
}

/// A rule that was applied in the derivation.
///
/// We use this data to look up the rule's text in the frontend.
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct WebRule {
    /// The source of this rule (sutrapatha, varttika, etc.)
    source: String,
    /// The (numeric) code that was applied for this step of the derivation.
    code: String,
}

/// A single term in the derivation history.
#[allow(non_snake_case)]
#[derive(Serialize)]
pub struct WebStepTerm {
    /// The text in the term.
    text: String,
    /// Whether or not this term was changed from the previous step.
    wasChanged: bool,
}

impl From<&StepTerm> for WebStepTerm {
    fn from(x: &StepTerm) -> Self {
        Self {
            text: x.text().to_string(),
            wasChanged: x.was_changed(),
        }
    }
}

/// A lightweight `Step` that exposes fewer private fields than the native `Step` struct.
#[derive(Serialize)]
pub struct WebStep {
    /// The rule that created this step.
    rule: WebRule,
    /// The result of applying the given rule.
    result: Vec<WebStepTerm>,
}

/// A lightweight `Prakriya` that exposes fewer private fields than the native `Prakriya` struct.
#[derive(Serialize)]
pub struct WebPrakriya {
    /// The final text produced by this prakriya.
    text: String,
    /// The list of steps that was applied to produce `text`.
    history: Vec<WebStep>,
}

impl Rule {
    /// The text this rule comes from.
    fn source(&self) -> &str {
        use Rule::*;
        match self {
            Ashtadhyayi(_) => "ashtadhyayi",
            Varttika(_) => "varttika",
            Dhatupatha(_) => "dhatupatha",
            Kashika(_) => "kashika",
            Linganushasana(_) => "linganushasanam",
            Kaumudi(_) => "kaumudi",
            Unadipatha(_) => "unadi",
            Phit(_) => "phit",
        }
    }
}

/// Converts the native `Step` array to a format that wasm_bindgen can serialize.
fn to_web_history(history: &[Step]) -> Vec<WebStep> {
    history
        .iter()
        .map(|step| WebStep {
            rule: WebRule {
                source: step.rule().source().to_string(),
                code: step.rule().code().to_string(),
            },
            result: step.result().iter().map(|t| t.into()).collect(),
        })
        .collect()
}

/// Converts the native `Prakriya` struct to a format that wasm_bindgen can serialize.
fn to_web_prakriyas(prakriyas: &[Prakriya]) -> Vec<WebPrakriya> {
    prakriyas
        .iter()
        .map(|p| WebPrakriya {
            text: p.text(),
            history: to_web_history(p.history()),
        })
        .collect()
}

/// Expands a mula dhatu by adding *sanādi pratyaya*s and upasargas, as needed.
fn try_expand_dhatu(dhatu: &Dhatu, sanadi: &[Sanadi], upasargas: &[String]) -> Dhatu {
    dhatu.clone().with_sanadi(sanadi).with_prefixes(upasargas)
}

#[derive(Serialize, Deserialize)]
struct TinantaArgs {
    code: String,
    lakara: Lakara,
    prayoga: Prayoga,
    purusha: Purusha,
    vacana: Vacana,
    pada: Option<DhatuPada>,
    sanadi: Vec<Sanadi>,
    upasarga: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct KrdantaArgs {
    code: String,
    krt: BaseKrt,
    sanadi: Vec<Sanadi>,
    upasarga: Vec<String>,
    lakara: Option<Lakara>,
    prayoga: Option<Prayoga>,
}

#[derive(Serialize, Deserialize)]
struct SubantaArgs {
    pratipadika: String,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
}

impl TinantaArgs {
    fn into_rust(self, raw_dhatu: &Dhatu) -> Tinanta {
        let dhatu = try_expand_dhatu(raw_dhatu, &self.sanadi, &self.upasarga);
        let mut args = Tinanta::builder()
            .dhatu(dhatu)
            .lakara(self.lakara)
            .prayoga(self.prayoga)
            .purusha(self.purusha)
            .vacana(self.vacana);
        if let Some(pada) = self.pada {
            args = args.pada(pada);
        }
        args.build().expect("should be well-formed")
    }
}

impl KrdantaArgs {
    fn into_rust(self, raw_dhatu: &Dhatu) -> Krdanta {
        let dhatu = try_expand_dhatu(raw_dhatu, &self.sanadi, &self.upasarga);
        let mut builder = Krdanta::builder().dhatu(dhatu).krt(self.krt);
        if let Some(la) = self.lakara {
            builder = builder.lakara(la);
        }
        if let Some(prayoga) = self.prayoga {
            builder = builder.prayoga(prayoga);
        }

        builder.build().expect("should be well-formed")
    }
}

impl SubantaArgs {
    fn into_rust(self) -> Subanta {
        Subanta::builder()
            .pratipadika(Pratipadika::basic(
                Slp1String::from(self.pratipadika).expect("ok"),
            ))
            .linga(self.linga)
            .vacana(self.vacana)
            .vibhakti(self.vibhakti)
            .build()
            .expect("should be well-formed")
    }
}

/// WebAssembly API for vidyut-prakriya.
///
/// Within reason, we have tried to mimic a native JavaScript API. At some point, we wish to
/// support optional arguments, perhaps by using `Reflect`.
#[wasm_bindgen]
pub struct Vidyut {
    /// An internal reference to a dhatupatha.
    /// (This dhatupatha is sourced from ashtadhyayi.com.)
    dhatupatha: Dhatupatha,
}

#[wasm_bindgen]
impl Vidyut {
    /// Creates a new API manager.
    ///
    /// This constructor is not called `new` because `new` is a reserved word in JavaScript.
    pub fn init(dhatupatha: &str) -> Self {
        // Logs panics to the console. Without this, panics are logged as "RuntimeError:
        // Unreachable executed", which is not useful.
        console_error_panic_hook::set_once();

        Vidyut {
            dhatupatha: Dhatupatha::from_text(dhatupatha).expect("should be well-formed"),
        }
    }

    /// Wrapper for `Vyakarana::derive_tinantas`.
    ///
    /// TODO: how might we reduce the number of arguments here?
    #[allow(non_snake_case)]
    pub fn deriveTinantas(&self, val: JsValue) -> JsValue {
        let js_args: TinantaArgs = serde_wasm_bindgen::from_value(val).unwrap();

        if let Some(raw_dhatu) = self.dhatupatha.get(&js_args.code) {
            let v = Vyakarana::new();
            let args = js_args.into_rust(raw_dhatu);
            let prakriyas = v.derive_tinantas(&args);

            let web_prakriyas = to_web_prakriyas(&prakriyas);
            serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
        } else {
            error(&format!("[vidyut] Dhatu code not found: {}", js_args.code));
            serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm")
        }
    }

    /// Wrapper for `Vyakarana::derive_subantas`.
    #[allow(non_snake_case)]
    pub fn deriveSubantas(&self, val: JsValue) -> JsValue {
        let v = Vyakarana::new();
        let js_args: SubantaArgs = serde_wasm_bindgen::from_value(val).unwrap();
        let args = js_args.into_rust();
        let prakriyas = v.derive_subantas(&args);

        let web_prakriyas = to_web_prakriyas(&prakriyas);
        serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
    }

    /// Wrapper for `Vyakarana::derive_krdantas`.
    #[allow(non_snake_case)]
    pub fn deriveKrdantas(&self, val: JsValue) -> JsValue {
        let js_args: KrdantaArgs = serde_wasm_bindgen::from_value(val).unwrap();

        if let Some(raw_dhatu) = self.dhatupatha.get(&js_args.code) {
            let v = Vyakarana::new();
            let args = js_args.into_rust(raw_dhatu);
            let prakriyas = v.derive_krdantas(&args);

            let web_prakriyas = to_web_prakriyas(&prakriyas);
            serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
        } else {
            error(&format!("[vidyut] Dhatu code not found: {}", js_args.code));
            serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm")
        }
    }

    /// Wrapper for `Vyakarana::derive_dhatus`.
    #[allow(non_snake_case)]
    pub fn deriveDhatus(&self, code: &str) -> JsValue {
        if let Some(dhatu) = self.dhatupatha.get(code) {
            let v = Vyakarana::new();
            let prakriyas = v.derive_dhatus(dhatu);

            let web_prakriyas = to_web_prakriyas(&prakriyas);
            serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
        } else {
            error(&format!("[vidyut] Dhatu code not found: {code}"));
            serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm")
        }
    }
}
