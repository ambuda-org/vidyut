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
use crate::dhatupatha::Dhatupatha;
use crate::prakriya::{Prakriya, Step};
use serde::Serialize;
extern crate console_error_panic_hook;

use crate::Ashtadhyayi;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};

/// A lightweight `Step` that exposes fewer private fields.
#[derive(Serialize)]
pub struct WebStep {
    rule: String,
    result: String,
}

/// A lightweight `Prakriya` that exposes fewer private fields.
#[derive(Serialize)]
pub struct WebPrakriya {
    text: String,
    history: Vec<WebStep>,
}

fn to_web_history(history: &[Step]) -> Vec<WebStep> {
    history
        .iter()
        .map(|x| WebStep {
            rule: x.rule(),
            result: x.result().clone(),
        })
        .collect()
}

fn to_web_prakriyas(prakriyas: &[Prakriya]) -> Vec<WebPrakriya> {
    prakriyas
        .iter()
        .map(|p| WebPrakriya {
            text: p.text(),
            history: to_web_history(p.history()),
        })
        .collect()
}

/// Expands a mula dhatu by adding sanadi-pratyayas and upasargas, as needed.
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

#[wasm_bindgen]
extern "C" {
    /// Exposes `console.error` in case we need to log anything to the JS console.
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn error(s: &str);
}

/// WebAssembly API for vidyut-prakriya.
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
            #[allow(clippy::unwrap_used)]
            dhatupatha: Dhatupatha::from_text(dhatupatha).unwrap(),
        }
    }

    /// Wrapper for `Ashtadhyayi::derive_tinantas`.
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
        pada: Option<Pada>,
        sanadi: Option<Sanadi>,
        upasarga: Option<String>,
    ) -> JsValue {
        if let Some(raw_dhatu) = self.dhatupatha.get(code) {
            let mut args = TinantaArgs::builder()
                .lakara(lakara)
                .prayoga(prayoga)
                .purusha(purusha)
                .vacana(vacana);

            if let Some(pada) = pada {
                args = args.pada(pada);
            }

            let args = args.build().expect("should be well-formed");

            let a = Ashtadhyayi::new();
            let dhatu = try_expand_dhatu(raw_dhatu, sanadi, upasarga);
            let prakriyas = a.derive_tinantas(&dhatu, &args);

            let web_prakriyas = to_web_prakriyas(&prakriyas);
            serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
        } else {
            error(&format!("[vidyut] Dhatu code not found: {code}"));
            serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm")
        }
    }

    /// Wrapper for `Ashtadhyayi::derive_subantas`.
    #[allow(non_snake_case)]
    pub fn deriveSubantas(
        &self,
        pratipadika: &str,
        linga: Linga,
        vacana: Vacana,
        vibhakti: Vibhakti,
    ) -> JsValue {
        let args = SubantaArgs::builder()
            .linga(linga)
            .vacana(vacana)
            .vibhakti(vibhakti)
            .build()
            .expect("should be well-formed");

        let a = Ashtadhyayi::new();
        let pratipadika = Pratipadika::new(pratipadika);
        let prakriyas = a.derive_subantas(&pratipadika, &args);

        let web_prakriyas = to_web_prakriyas(&prakriyas);
        serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
    }

    /// Wrapper for `Ashtadhyayi::derive_krdantas`.
    ///
    /// TODO: how might we reduce the number of arguments here?
    #[allow(non_snake_case)]
    pub fn deriveKrdantas(
        &self,
        code: &str,
        krt: Krt,
        sanadi: Option<Sanadi>,
        upasarga: Option<String>,
    ) -> JsValue {
        if let Some(raw_dhatu) = self.dhatupatha.get(code) {
            let args = KrdantaArgs::builder()
                .krt(krt)
                .build()
                .expect("should be well-formed");

            let a = Ashtadhyayi::new();
            let dhatu = try_expand_dhatu(raw_dhatu, sanadi, upasarga);
            let prakriyas = a.derive_krdantas(&dhatu, &args);

            let web_prakriyas = to_web_prakriyas(&prakriyas);
            serde_wasm_bindgen::to_value(&web_prakriyas).expect("wasm")
        } else {
            error(&format!("[vidyut] Dhatu code not found: {code}"));
            serde_wasm_bindgen::to_value(&Vec::<WebPrakriya>::new()).expect("wasm")
        }
    }
}
