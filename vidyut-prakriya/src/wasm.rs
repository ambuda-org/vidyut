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
use crate::core::Error;
use crate::core::Rule;
use crate::core::{Prakriya, Step, StepTerm};
use serde::{Deserialize, Serialize};
extern crate console_error_panic_hook;

use crate::Vyakarana;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Exposes `console.error` in case we need to log anything to the JS console.
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn error(s: &str);

    /// Exposes `console.error` in case we need to log anything to the JS console.
    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    fn debug(s: &str);
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

// For now, mula-dhatus only.
#[derive(Serialize, Deserialize)]
struct DhatuArgs {
    aupadeshika: String,
    gana: Gana,
    antargana: Option<Antargana>,
    sanadi: Vec<Sanadi>,
    prefixes: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct KrdantaArgs {
    dhatu: DhatuArgs,
    krt: Option<BaseKrt>,
    unadi: Option<Unadi>,
    lakara: Option<Lakara>,
    prayoga: Option<Prayoga>,
    #[serde(skip_serializing_if = "Option::is_none")]
    upapada: Option<UpapadadArgs>,
}

#[derive(Serialize, Deserialize)]
struct UpapadadArgs {
    stem: Option<String>,
    linga: Option<Linga>,
    vibhakti: Option<Vibhakti>,
    vacana: Option<Vacana>,
}

// rust-wasm does not support enums, so fake enum-like behavior through a struct with optional
// fields.
//
// The API expects that exactly one field is set. Otherwise, the API will throw an error.
#[derive(Serialize, Deserialize)]
struct PratipadikaArgs {
    basic: Option<String>,
    nyap: Option<String>,
    krdanta: Option<KrdantaArgs>,
    taddhitanta: Option<TaddhitantaArgsInner>,
}

#[derive(Serialize, Deserialize)]
struct TaddhitantaArgsInner {
    stem: String,
    taddhita: Taddhita,
}

#[derive(Serialize, Deserialize)]
struct SubantaArgs {
    pratipadika: PratipadikaArgs,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
}

#[derive(Serialize, Deserialize)]
struct TinantaArgs {
    dhatu: DhatuArgs,
    lakara: Lakara,
    prayoga: Prayoga,
    purusha: Purusha,
    vacana: Vacana,
    skip_at_agama: bool,
    pada: Option<DhatuPada>,
}

#[derive(Serialize, Deserialize)]
struct TaddhitantaArgs {
    pratipadika: PratipadikaArgs,
    taddhita: Taddhita,
}

/// Shorthand for result type
pub type Result<T> = std::result::Result<T, Error>;

impl DhatuArgs {
    fn into_rust(self) -> Result<Dhatu> {
        let aupadeshika = Slp1String::from(self.aupadeshika)?;
        let mut dhatu = match self.antargana {
            Some(antargana) => Dhatu::mula_with_antargana(aupadeshika, self.gana, antargana),
            None => Dhatu::mula(aupadeshika, self.gana),
        };

        dhatu = dhatu
            .with_prefixes(&self.prefixes)
            .with_sanadi(&self.sanadi);

        Ok(dhatu)
    }
}

impl KrdantaArgs {
    fn into_rust(self) -> Result<Krdanta> {
        let dhatu: Dhatu = self.dhatu.into_rust()?;
        let mut builder = Krdanta::builder().dhatu(dhatu);

        if let Some(unadi) = self.unadi {
            builder = builder.krt(unadi);
        } else if let Some(krt) = self.krt {
            builder = builder.krt(krt);
        }
        if let Some(la) = self.lakara {
            builder = builder.lakara(la);
        }
        if let Some(prayoga) = self.prayoga {
            builder = builder.prayoga(prayoga);
        }
        if let Some(upapada) = self.upapada {
            if let Some(stem_val) = upapada.stem {
                debug(&format!("[vidyut debug] upapada:v={}, l={}", upapada.vibhakti.unwrap(), upapada.linga.unwrap()));
                let pratipadika = Pratipadika::basic(Slp1String::from(stem_val)?);

                let subanta =
                    Subanta::new(pratipadika, upapada.linga.unwrap(), upapada.vibhakti.unwrap(), upapada.vacana.unwrap());
                builder = builder.upapada(subanta);
            }
        }
        builder.build()
    }
}

impl SubantaArgs {
    fn into_rust(self) -> Result<Subanta> {
        let pratipadika = match self.pratipadika {
            PratipadikaArgs {
                basic: Some(basic),
                nyap: None,
                krdanta: None,
                taddhitanta: None,
            } => Pratipadika::basic(Slp1String::from(basic)?),
            PratipadikaArgs {
                basic: None,
                nyap: Some(nyap),
                krdanta: None,
                taddhitanta: None,
            } => Pratipadika::nyap(Slp1String::from(nyap).expect("ok")),
            PratipadikaArgs {
                basic: None,
                nyap: None,
                krdanta: Some(krt),
                taddhitanta: None,
            } => Pratipadika::Krdanta(Box::new(krt.into_rust()?)),
            PratipadikaArgs {
                basic: None,
                nyap: None,
                krdanta: None,
                taddhitanta: Some(tad),
            } => {
                let base = Pratipadika::basic(Slp1String::from(tad.stem).expect("ok"));
                Pratipadika::Taddhitanta(Box::new(Taddhitanta::new(base, tad.taddhita)))
            }
            // TODO: improve error handling, remove placeholder
            _ => Pratipadika::basic(Slp1String::from("doza").expect("ok")),
        };
        Subanta::builder()
            .pratipadika(pratipadika)
            .linga(self.linga)
            .vacana(self.vacana)
            .vibhakti(self.vibhakti)
            .build()
    }
}

impl TinantaArgs {
    fn into_rust(self) -> Result<Tinanta> {
        let mut args = Tinanta::builder()
            .dhatu(self.dhatu.into_rust()?)
            .lakara(self.lakara)
            .prayoga(self.prayoga)
            .purusha(self.purusha)
            .vacana(self.vacana)
            .skip_at_agama(self.skip_at_agama);
        if let Some(pada) = self.pada {
            args = args.pada(pada);
        }
        args.build()
    }
}

impl TaddhitantaArgs {
    fn into_rust(self) -> Result<Taddhitanta> {
        let pratipadika = match self.pratipadika {
            PratipadikaArgs {
                basic: Some(basic),
                nyap: None,
                krdanta: None,
                taddhitanta: None,
            } => Pratipadika::basic(Slp1String::from(basic)?),
            PratipadikaArgs {
                basic: None,
                nyap: Some(nyap),
                krdanta: None,
                taddhitanta: None,
            } => Pratipadika::nyap(Slp1String::from(nyap).expect("ok")),
            PratipadikaArgs {
                basic: None,
                nyap: None,
                krdanta: Some(krt),
                taddhitanta: None,
            } => Pratipadika::Krdanta(Box::new(krt.into_rust()?)),
            _ => Pratipadika::basic(Slp1String::from("doza").expect("ok")),
        };
        Ok(Taddhitanta::new(pratipadika, self.taddhita))
    }
}

/// WebAssembly API for vidyut-prakriya.
///
/// Within reason, we have tried to mimic a native JavaScript API. At some point, we wish to
/// support optional arguments, perhaps by using `Reflect`.
#[wasm_bindgen]
pub struct Vidyut {}

#[wasm_bindgen]
impl Vidyut {
    /// Creates a new API manager.
    ///
    /// This constructor is not called `new` because `new` is a reserved word in JavaScript.
    pub fn init() -> Self {
        // Logs panics to the console. Without this, panics are logged as "RuntimeError:
        // Unreachable executed", which is not useful.
        console_error_panic_hook::set_once();

        Self {}
    }

    /// Wrapper for `Vyakarana::derive_krdantas`.
    #[allow(non_snake_case)]
    pub fn deriveKrdantas(&self, val: JsValue) -> JsValue {
        let js_args: KrdantaArgs = serde_wasm_bindgen::from_value(val).unwrap();

        match js_args.into_rust() {
            Ok(args) => {
                let v = Vyakarana::new();
                let prakriyas = v.derive_krdantas(&args);

                let web_prakriyas = to_web_prakriyas(&prakriyas);
                serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
            }
            Err(_) => {
                error(&format!("[vidyut] Derivation error"));
                serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm")
            }
        }
    }

    /// Wrapper for `Vyakarana::derive_dhatus`.
    #[allow(non_snake_case)]
    pub fn deriveDhatus(&self, val: JsValue) -> JsValue {
        let v = Vyakarana::new();
        let js_args: DhatuArgs = serde_wasm_bindgen::from_value(val).unwrap();

        match js_args.into_rust() {
            Ok(args) => {
                let prakriyas = v.derive_dhatus(&args);
                let web_prakriyas = to_web_prakriyas(&prakriyas);
                serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
            }
            Err(_) => {
                error(&format!("[vidyut] Derivation error"));
                serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm")
            }
        }
    }

    /// Wrapper for `Vyakarana::derive_subantas`.
    #[allow(non_snake_case)]
    pub fn deriveSubantas(&self, val: JsValue) -> JsValue {
        let v = Vyakarana::new();
        let js_args: SubantaArgs = serde_wasm_bindgen::from_value(val).unwrap();

        match js_args.into_rust() {
            Ok(args) => {
                let prakriyas = v.derive_subantas(&args);
                let web_prakriyas = to_web_prakriyas(&prakriyas);
                serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
            }
            Err(_) => {
                error(&format!("[vidyut] Derivation error"));
                serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm")
            }
        }
    }

    /// Wrapper for `Vyakarana::derive_tinantas`.
    ///
    /// TODO: how might we reduce the number of arguments here?
    #[allow(non_snake_case)]
    pub fn deriveTinantas(&self, val: JsValue) -> JsValue {
        let v = Vyakarana::new();
        let js_args: TinantaArgs = serde_wasm_bindgen::from_value(val).unwrap();

        match js_args.into_rust() {
            Ok(args) => {
                let prakriyas = v.derive_tinantas(&args);
                let web_prakriyas = to_web_prakriyas(&prakriyas);
                serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
            }
            Err(_) => {
                error(&format!("[vidyut] Derivation error"));
                serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm")
            }
        }
    }

    /// Wrapper for `Vyakarana::derive_taddhitantas`.
    #[allow(non_snake_case)]
    pub fn deriveTaddhitantas(&self, val: JsValue) -> JsValue {
        let v = Vyakarana::new();
        let js_args: TaddhitantaArgs = serde_wasm_bindgen::from_value(val).unwrap();

        match js_args.into_rust() {
            Ok(args) => {
                let prakriyas = v.derive_taddhitantas(&args);
                let web_prakriyas = to_web_prakriyas(&prakriyas);
                serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
            }
            Err(_) => {
                error(&format!("[vidyut] Derivation error"));
                serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm")
            }
        }
    }

    /// Wrapper for `Vyakarana::derive_stryantas`.
    #[allow(non_snake_case)]
    pub fn deriveStryantas(&self, val: JsValue) -> JsValue {
        let v = Vyakarana::new();
        let js_args: PratipadikaArgs = match serde_wasm_bindgen::from_value(val) {
            Ok(args) => args,
            Err(e) => {
                error(&format!("[vidyut] deriveStryantas parse error: {:?}", e));
                return serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm");
            }
        };

        debug(&format!(
            "[vidyut] deriveStryantas js_args: basic={:?}, nyap={:?}, krdanta={:?}",
            js_args.basic,
            js_args.nyap,
            js_args.krdanta.is_some()
        ));

        let pratipadika = match js_args {
            PratipadikaArgs {
                basic: Some(basic),
                nyap: None,
                krdanta: None,
                taddhitanta: None,
            } => Pratipadika::basic(Slp1String::from(basic).expect("ok")),
            PratipadikaArgs {
                basic: None,
                nyap: None,
                krdanta: Some(krt),
                taddhitanta: None,
            } => match krt.into_rust() {
                Ok(k) => {
                    debug(&format!(
                        "[vidyut] deriveStryantas krdanta conversion successful"
                    ));
                    Pratipadika::Krdanta(Box::new(k))
                }
                Err(e) => {
                    error(&format!("[vidyut] Krdanta conversion error: {:?}", e));
                    return serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm");
                }
            },
            _ => {
                error("[vidyut] Invalid pratipadika args for stryantas");
                return serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm");
            }
        };

        let prakriyas = v.derive_stryantas(&pratipadika);
        debug(&format!(
            "[vidyut] deriveStryantas produced {} results",
            prakriyas.len()
        ));
        let web_prakriyas = to_web_prakriyas(&prakriyas);
        serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
    }
}
