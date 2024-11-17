/*!
WebAssembly bindings for vidyut-prakriya.

Since vidyut-prakriya is lightweight and has few dependencies, we can create WebAssembly bindings
for it quite easily. In development, we recommend using these bindings through the `make debugger`
command, which will also start a small Python webserver.

The main struct of interest is `Vidyut`, which wraps all of vidyut-prakriya's high-level APIs.

Although these bindings are usable and reliable, we want to improve their ergonomics so that
JavaScript callers can use them more idiomatically.
*/
use crate::args::*;
use crate::core::Rule;
use crate::core::{Prakriya, Step, StepTerm};
use crate::dhatupatha::Dhatupatha;
use serde::Serialize;
extern crate console_error_panic_hook;

use crate::Vyakarana;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

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
fn try_expand_dhatu(dhatu: &Dhatu, sanadi: Option<Sanadi>, upasarga: Option<String>) -> Dhatu {
    let mut ret = dhatu.clone();
    if let Some(s) = sanadi {
        ret = ret.with_sanadi(&[s]);
    }
    if let Some(u) = upasarga {
        ret = ret.with_prefixes(&[u]);
    }
    ret
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
    #[allow(clippy::too_many_arguments)]
    #[allow(non_snake_case)]
    pub fn deriveTinantas(
        &self,
        code: &str,
        lakara: Lakara,
        prayoga: Prayoga,
        purusha: Purusha,
        vacana: Vacana,
        pada: Option<DhatuPada>,
        sanadi: Option<Sanadi>,
        upasarga: Option<String>,
    ) -> JsValue {
        if let Some(raw_dhatu) = self.dhatupatha.get(code) {
            let dhatu = try_expand_dhatu(raw_dhatu, sanadi, upasarga);
            let mut args = Tinanta::builder()
                .dhatu(dhatu)
                .lakara(lakara)
                .prayoga(prayoga)
                .purusha(purusha)
                .vacana(vacana);
            if let Some(pada) = pada {
                args = args.pada(pada);
            }
            let args = args.build().expect("should be well-formed");

            let v = Vyakarana::new();
            let prakriyas = v.derive_tinantas(&args);

            let web_prakriyas = to_web_prakriyas(&prakriyas);
            serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
        } else {
            error(&format!("[vidyut] Dhatu code not found: {code}"));
            serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm")
        }
    }

    /// Wrapper for `Vyakarana::derive_subantas`.
    #[allow(non_snake_case)]
    pub fn deriveSubantas(
        &self,
        pratipadika: &str,
        linga: Linga,
        vibhakti: Vibhakti,
        vacana: Vacana,
    ) -> JsValue {
        let args = Subanta::builder()
            .pratipadika(Pratipadika::basic(pratipadika))
            .linga(linga)
            .vacana(vacana)
            .vibhakti(vibhakti)
            .build()
            .expect("should be well-formed");

        let v = Vyakarana::new();
        let prakriyas = v.derive_subantas(&args);

        let web_prakriyas = to_web_prakriyas(&prakriyas);
        serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
    }

    /// Wrapper for `Vyakarana::derive_krdantas`.
    #[allow(non_snake_case)]
    pub fn deriveKrdantas(
        &self,
        code: &str,
        krt: BaseKrt,
        sanadi: Option<Sanadi>,
        upasarga: Option<String>,
    ) -> JsValue {
        if let Some(raw_dhatu) = self.dhatupatha.get(code) {
            let dhatu = try_expand_dhatu(raw_dhatu, sanadi, upasarga);
            let args = Krdanta::builder()
                .dhatu(dhatu)
                .krt(krt)
                .build()
                .expect("should be well-formed");

            let v = Vyakarana::new();
            let prakriyas = v.derive_krdantas(&args);

            let web_prakriyas = to_web_prakriyas(&prakriyas);
            serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
        } else {
            error(&format!("[vidyut] Dhatu code not found: {code}"));
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
