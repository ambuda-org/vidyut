/*!
Various test utils.

vidyut-prakriya has thousands of assert statements. This module contains various functions to
manage the boilerplate required for these assertions.
*/
extern crate vidyut_prakriya;

use vidyut_prakriya::args::Antargana;
use vidyut_prakriya::args::Purusha as P;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;
use vidyut_prakriya::Prakriya;
use vidyut_prakriya::Rule;

/// A handy way to manage various assertions.
///
/// `Ashtadhyayi` with its default settings will ignore certain rules. Some of these rules are
/// chAndasa rules, meaning that they apply only in Vedic usage. Other rules are valid options but
/// will overgenerate, which adds too much noise to our other tests.
///
/// To test these ignored rules, we can create a new `Tester` that uses the `Ashtadhyayi` settings we
/// specify. For convenience, all of these methods have wrapper functions that assume the
/// default settings for `Ashtadhyayi`.
pub struct Tester {
    ashtadhyayi: Ashtadhyayi,
}

impl Tester {
    /// Creates a tester with our default settings.
    pub fn new(ashtadhyayi: Ashtadhyayi) -> Self {
        Self { ashtadhyayi }
    }

    /// Creates a tester that enables chAndasa rules.
    pub fn with_chaandasa() -> Self {
        Self {
            ashtadhyayi: Ashtadhyayi::builder().is_chandasa(true).build(),
        }
    }

    /// Asserts that the given input conditions produce the tinantas `expected`.
    fn assert_has_tin(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        lakara: Lakara,
        purusha: Purusha,
        vacana: Vacana,
        expected: &[&str],
    ) {
        let args = TinantaArgs::builder()
            .prayoga(Prayoga::Kartari)
            .purusha(purusha)
            .vacana(vacana)
            .lakara(lakara)
            .build()
            .unwrap();
        let actual = derive_tinantas(
            &self.ashtadhyayi,
            &dhatu.clone().with_prefixes(prefixes),
            &args,
        );
        assert_padas(actual, expected);
    }

    /// Asserts that the given input conditions produce the parasmaipada tinantas in `expected`.
    fn assert_has_parasmai_tin(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        lakara: Lakara,
        purusha: Purusha,
        vacana: Vacana,
        expected: &[&str],
    ) {
        let args = TinantaArgs::builder()
            .prayoga(Prayoga::Kartari)
            .purusha(purusha)
            .vacana(vacana)
            .lakara(lakara)
            .pada(Pada::Parasmai)
            .build()
            .unwrap();
        let actual = derive_tinantas(
            &self.ashtadhyayi,
            &dhatu.clone().with_prefixes(prefixes),
            &args,
        );
        assert_padas(actual, expected);
    }

    /// Asserts that the given input conditions produce the Atmanepada tinantas in `expected`.
    fn assert_has_atmane_tin(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        lakara: Lakara,
        purusha: Purusha,
        vacana: Vacana,
        expected: &[&str],
    ) {
        let args = TinantaArgs::builder()
            .prayoga(Prayoga::Kartari)
            .purusha(purusha)
            .vacana(vacana)
            .lakara(lakara)
            .pada(Pada::Atmane)
            .build()
            .unwrap();
        let actual = derive_tinantas(
            &self.ashtadhyayi,
            &dhatu.clone().with_prefixes(prefixes),
            &args,
        );
        assert_padas(actual, expected);
    }

    /// Asserts that the given input conditions produce the karmaNi-prayoga tinantas in `expected`.
    pub fn assert_has_karmani_tin(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        lakara: Lakara,
        purusha: Purusha,
        vacana: Vacana,
        expected: &[&str],
    ) {
        let args = TinantaArgs::builder()
            .prayoga(Prayoga::Karmani)
            .purusha(purusha)
            .vacana(vacana)
            .lakara(lakara)
            .build()
            .unwrap();
        let actual = derive_tinantas(
            &self.ashtadhyayi,
            &dhatu.clone().with_prefixes(prefixes),
            &args,
        );
        assert_padas(actual, expected);
    }

    /// Asserts that the given input conditions produce the prathamA-eka tinantas in `expected`
    fn assert_has_lakara(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_tin(prefixes, dhatu, la, P::Prathama, Eka, expected);
    }

    pub fn assert_has_tip(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, dhatu, la, P::Prathama, Eka, expected);
    }

    pub fn assert_has_tas(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, dhatu, la, P::Prathama, Dvi, expected);
    }

    pub fn assert_has_jhi(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, dhatu, la, P::Prathama, Bahu, expected);
    }

    pub fn assert_has_sip(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, dhatu, la, P::Madhyama, Eka, expected);
    }

    pub fn assert_has_thas(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, dhatu, la, P::Madhyama, Dvi, expected);
    }

    pub fn assert_has_tha(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, dhatu, la, P::Madhyama, Bahu, expected);
    }

    pub fn assert_has_mip(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, dhatu, la, P::Uttama, Eka, expected);
    }

    pub fn assert_has_vas(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, dhatu, la, P::Uttama, Dvi, expected);
    }

    pub fn assert_has_mas(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, dhatu, la, P::Uttama, Bahu, expected);
    }

    pub fn assert_has_ta(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_atmane_tin(prefixes, dhatu, la, P::Prathama, Eka, expected);
    }

    pub fn assert_has_aataam(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        la: Lakara,
        expected: &[&str],
    ) {
        self.assert_has_atmane_tin(prefixes, dhatu, la, P::Prathama, Dvi, expected);
    }

    pub fn assert_has_jha(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_atmane_tin(prefixes, dhatu, la, P::Prathama, Bahu, expected);
    }

    pub fn assert_has_thaas(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        la: Lakara,
        expected: &[&str],
    ) {
        self.assert_has_atmane_tin(prefixes, dhatu, la, P::Madhyama, Eka, expected);
    }

    pub fn assert_has_aathaam(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        la: Lakara,
        expected: &[&str],
    ) {
        self.assert_has_atmane_tin(prefixes, dhatu, la, P::Madhyama, Dvi, expected);
    }

    pub fn assert_has_dhvam(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        la: Lakara,
        expected: &[&str],
    ) {
        self.assert_has_atmane_tin(prefixes, dhatu, la, P::Madhyama, Bahu, expected);
    }

    pub fn assert_has_iw(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_atmane_tin(prefixes, dhatu, la, P::Uttama, Eka, expected);
    }

    pub fn assert_has_vahi(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_atmane_tin(prefixes, dhatu, la, P::Uttama, Dvi, expected);
    }

    pub fn assert_has_mahin(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        la: Lakara,
        expected: &[&str],
    ) {
        self.assert_has_atmane_tin(prefixes, dhatu, la, P::Uttama, Bahu, expected);
    }

    pub fn assert_has_ta_k(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_karmani_tin(prefixes, dhatu, la, P::Prathama, Eka, expected);
    }

    pub fn assert_has_aataam_k(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        la: Lakara,
        expected: &[&str],
    ) {
        self.assert_has_karmani_tin(prefixes, dhatu, la, P::Prathama, Dvi, expected);
    }

    pub fn assert_has_jha_k(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        la: Lakara,
        expected: &[&str],
    ) {
        self.assert_has_karmani_tin(prefixes, dhatu, la, P::Prathama, Bahu, expected);
    }

    pub fn assert_has_thaas_k(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        la: Lakara,
        expected: &[&str],
    ) {
        self.assert_has_karmani_tin(prefixes, dhatu, la, P::Madhyama, Eka, expected);
    }

    pub fn assert_has_aathaam_k(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        la: Lakara,
        expected: &[&str],
    ) {
        self.assert_has_karmani_tin(prefixes, dhatu, la, P::Madhyama, Dvi, expected);
    }

    pub fn assert_has_dhvam_k(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        la: Lakara,
        expected: &[&str],
    ) {
        self.assert_has_karmani_tin(prefixes, dhatu, la, P::Madhyama, Bahu, expected);
    }

    pub fn assert_has_iw_k(&self, prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_karmani_tin(prefixes, dhatu, la, P::Uttama, Eka, expected);
    }

    pub fn assert_has_vahi_k(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        la: Lakara,
        expected: &[&str],
    ) {
        self.assert_has_karmani_tin(prefixes, dhatu, la, P::Uttama, Dvi, expected);
    }

    pub fn assert_has_mahin_k(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        la: Lakara,
        expected: &[&str],
    ) {
        self.assert_has_karmani_tin(prefixes, dhatu, la, P::Uttama, Bahu, expected);
    }

    fn assert_has_sup(
        &self,
        pratipadika: &Pratipadika,
        linga: Linga,
        vibhakti: Vibhakti,
        vacana: Vacana,
        expected: &[&str],
    ) {
        let args = SubantaArgs::builder()
            .linga(linga)
            .vacana(vacana)
            .vibhakti(vibhakti)
            .build()
            .unwrap();

        let results = self.ashtadhyayi.derive_subantas(pratipadika, &args);
        let actual = sanitize_results(results);
        assert_padas(actual, expected);
    }

    pub fn assert_has_krt(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        krt: impl Into<Krt>,
        expected: &[&str],
    ) {
        let args = KrdantaArgs::builder().krt(krt.into()).build().unwrap();
        let actual = derive_krdantas(
            &self.ashtadhyayi,
            &dhatu.clone().with_prefixes(prefixes),
            args,
        );
        assert_padas(actual, expected);
    }
}

impl Default for Tester {
    fn default() -> Self {
        Self::new(Ashtadhyayi::new())
    }
}

// Derivation helpers
// ------------------

/// Sanitizes our test results by making them deterministic and predictable.
fn sanitize_results(mut results: Vec<Prakriya>) -> Vec<Prakriya> {
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());

    results
        .into_iter()
        .filter(|p| {
            let text = p.text();
            !text.ends_with('d')
                && !text.ends_with('q')
                && !text.ends_with('g')
                && !text.ends_with('b')
        })
        .collect()
}

/// Derives tinantas from the given initial conditions.
fn derive_tinantas(a: &Ashtadhyayi, dhatu: &Dhatu, args: &TinantaArgs) -> Vec<Prakriya> {
    let results = a.derive_tinantas(dhatu, args);
    sanitize_results(results)
}

/// Derives krdantas from the given initial conditions.
fn derive_krdantas(a: &Ashtadhyayi, dhatu: &Dhatu, args: KrdantaArgs) -> Vec<Prakriya> {
    let mut results = a.derive_krdantas(dhatu, &args);
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());
    // Allowed in pada sandhi, but noisy here.
    results.retain(|p| !p.text().contains("cS"));
    results
}

/// Derives taddhitantas from the given initial conditions.
fn derive_taddhitantas(p: &Pratipadika, t: Taddhita, a: Option<TaddhitaArtha>) -> Vec<Prakriya> {
    let args = if let Some(a) = a {
        TaddhitantaArgs::builder()
            .taddhita(t)
            .artha(a)
            .build()
            .unwrap()
    } else {
        TaddhitantaArgs::builder().taddhita(t).build().unwrap()
    };
    let a = Ashtadhyayi::new();
    let results = a.derive_taddhitantas(p, &args);
    sanitize_results(results)
}

/// Derives vakyas from the given initial conditions.
fn derive_vakyas(first: &str, second: &str) -> Vec<Prakriya> {
    let a = Ashtadhyayi::new();
    let mut results = a.derive_vakyas(&first, &second);
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());
    results
}

fn debug_text(rule: Rule) -> String {
    match rule {
        Rule::Ashtadhyayi(x) => x.to_string(),
        Rule::Dhatupatha(x) => format!("DA {x}"),
        Rule::Kashika(x) => format!("kA {x}"),
        Rule::Kaumudi(x) => format!("kO {x}"),
        Rule::Linganushasana(x) => format!("Li {x}"),
        Rule::UP(x) => format!("uRA {x}"),
    }
}

/// Nicely prints out the given prakriyas.
fn print_all_prakriyas(prakriyas: &[Prakriya]) {
    for p in prakriyas {
        for step in p.history() {
            println!("{} --> {}", debug_text(step.rule()), step.result());
        }
        println!("{:?}", p.rule_choices());
        println!();
    }
}

// Heavy assert helpers
// --------------------
// These functions are too heavy for regular use. Instead, use the smaller assert functions below.

/// Asserts that the given `prakriyas` produce the `expected` results.
///
/// If there is any difference, this function will nicely print out each prakriya in `prakriyas`.
fn assert_padas(prakriyas: Vec<Prakriya>, expected: &[&str]) {
    let actuals: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();

    if actuals.len() != expected.len() {
        print_all_prakriyas(&prakriyas);
    }

    assert_eq!(
        actuals.len(),
        expected.len(),
        "expected: {expected:?}, actual: {actuals:?}"
    );

    let mut expected = Vec::from(expected);
    expected.sort();
    expected.dedup();

    for (i, p) in prakriyas.iter().enumerate() {
        let actual = p.text();

        if actual != expected[i] {
            print_all_prakriyas(&prakriyas);
        }
        assert_eq!(
            actual, expected[i],
            "expected: {expected:?}, actual: {actuals:?}"
        );
    }
}

/// Checks the given combination of `dhatu` and `prefixes` produces the `expected` results given
/// this `lakara`/`purusha`/`vacana` combination.
pub fn assert_has_tinanta(
    prefixes: &[&str],
    dhatu: &Dhatu,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
    expected: &[&str],
) {
    let t = Tester::default();
    t.assert_has_tin(prefixes, dhatu, lakara, purusha, vacana, expected);
}

pub fn assert_has_sandhi(first: &str, second: &str, expected: &[&str]) {
    let prakriyas = derive_vakyas(&first, &second);

    let actuals: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    if actuals.len() != expected.len() {
        print_all_prakriyas(&prakriyas);
    }

    assert_eq!(
        actuals.len(),
        expected.len(),
        "expected: {expected:?}, actual: {actuals:?}"
    );

    let mut expected: Vec<_> = expected.iter().map(|text| text.replace(" ", "")).collect();
    expected.sort();
    expected.dedup();

    for (i, p) in prakriyas.iter().enumerate() {
        let actual = p.text();

        if actual != expected[i] {
            print_all_prakriyas(&prakriyas);
        }
        assert_eq!(
            actual, expected[i],
            "expected: {expected:?}, actual: {actuals:?}"
        );
    }
}

// Dhatu helpers
// -------------

/// Shorthand for `Dhatu::new`.
///
/// Our tests create dhatus thousands of times, so we defined this function to save some typing and
/// make our tests easier to scan.
pub fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

pub fn d_kutadi(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g).with_antargana(Some(Antargana::Kutadi))
}

/// Marks a dhatu as taking san-pratyaya.
pub fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San])
}

/// Marks a dhatu as taking yaN-pratyaya.
pub fn yan(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Yan])
}

/// Marks a dhatu as taking Ric-pratyaya.
pub fn nic(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Nic])
}

/// Marks a dhatu as taking san-pratyaya followed by Nic-pratyaya.
pub fn san_nic(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San, Sanadi::Nic])
}

/// Marks a dhatu as taking Nic-pratyaya followed by san-pratyaya.
pub fn nic_san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Nic, Sanadi::San])
}

/// Marks a dhatu as taking yaN-pratyaya with luk.
pub fn yan_luk(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::YanLuk])
}

// Pratipadika helpers
// -------------------

/// Shorthand for building a pratipadika.
pub fn prati(text: &str) -> Pratipadika {
    Pratipadika::new(text)
}

/// Shorthand for building a pratipadika that ends with NI/Ap.
pub fn nyap(text: &str) -> Pratipadika {
    Pratipadika::builder()
        .text(text)
        .is_nyap(true)
        .build()
        .unwrap()
}

/// Shorthand for building a pratipadika from a dhatu.
pub fn dhatu_prati(text: &str) -> Pratipadika {
    Pratipadika::builder()
        .text(text)
        .is_dhatu(true)
        .build()
        .unwrap()
}

/// Creates a function alias `fn_name` that points to the method of the same name on a default
/// `Tester` instance.
macro_rules! test_tin {
    ($fn_name:ident) => {
        pub fn $fn_name(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
            let t = Tester::default();
            t.$fn_name(prefixes, dhatu, la, expected);
        }
    };
}

test_tin!(assert_has_tip);
test_tin!(assert_has_tas);
test_tin!(assert_has_jhi);
test_tin!(assert_has_sip);
test_tin!(assert_has_thas);
test_tin!(assert_has_tha);
test_tin!(assert_has_mip);
test_tin!(assert_has_vas);
test_tin!(assert_has_mas);

test_tin!(assert_has_ta);
test_tin!(assert_has_aataam);
test_tin!(assert_has_jha);
test_tin!(assert_has_thaas);
test_tin!(assert_has_aathaam);
test_tin!(assert_has_dhvam);
test_tin!(assert_has_iw);
test_tin!(assert_has_vahi);
test_tin!(assert_has_mahin);

test_tin!(assert_has_ta_k);
test_tin!(assert_has_aataam_k);
test_tin!(assert_has_jha_k);
test_tin!(assert_has_thaas_k);
test_tin!(assert_has_aathaam_k);
test_tin!(assert_has_dhvam_k);
test_tin!(assert_has_iw_k);
test_tin!(assert_has_vahi_k);
test_tin!(assert_has_mahin_k);

macro_rules! test_la {
    ($fn_name:ident, $la:expr) => {
        pub fn $fn_name(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
            let t = Tester::default();
            t.assert_has_lakara(prefixes, dhatu, $la, expected);
        }
    };
}

test_la!(assert_has_lat, Lakara::Lat);
test_la!(assert_has_lit, Lakara::Lit);
test_la!(assert_has_lut, Lakara::Lut);
test_la!(assert_has_lrt, Lakara::Lrt);
test_la!(assert_has_lot, Lakara::Lot);
test_la!(assert_has_lan, Lakara::Lan);
test_la!(assert_has_ashirlin, Lakara::AshirLin);
test_la!(assert_has_vidhilin, Lakara::VidhiLin);
test_la!(assert_has_lun, Lakara::Lun);
test_la!(assert_has_lrn, Lakara::Lrn);

// Krdanta helpers
// ---------------

pub fn assert_has_krdanta(
    prefixes: &[&str],
    dhatu: &Dhatu,
    krt: impl Into<Krt>,
    expected: &[&str],
) {
    let t = Tester::default();
    t.assert_has_krt(prefixes, dhatu, krt, expected);
}

pub fn assert_has_artha_krdanta(
    upapadas: &[&str],
    dhatu: &Dhatu,
    requested_artha: KrtArtha,
    krt: impl Into<Krt>,
    expected: &[&str],
) {
    let a = Ashtadhyayi::new();
    let args = KrdantaArgs::builder()
        .krt(krt.into())
        .artha(requested_artha)
        .build()
        .unwrap();
    let mut prakriyas = derive_krdantas(&a, &dhatu.clone().with_prefixes(upapadas), args);

    prakriyas.retain(|p| {
        if let Some(Artha::Krt(prakriya_artha)) = p.artha() {
            requested_artha == prakriya_artha
        } else {
            false
        }
    });
    assert_padas(prakriyas, expected);
}

pub fn assert_has_upapada_krdanta(
    upapada: &str,
    prefixes: &[&str],
    dhatu: &Dhatu,
    krt: impl Into<Krt>,
    expected: &[&str],
) {
    let a = Ashtadhyayi::new();
    let args = KrdantaArgs::builder()
        .krt(krt.into())
        .upapada(Upapada::make_subanta(upapada))
        .build()
        .unwrap();
    assert_padas(
        derive_krdantas(&a, &dhatu.clone().with_prefixes(prefixes), args),
        expected,
    );
}

pub fn assert_has_upapada_krdanta_raw(
    upapada: Upapada,
    prefixes: &[&str],
    dhatu: &Dhatu,
    krt: impl Into<Krt>,
    expected: &[&str],
) {
    let a = Ashtadhyayi::new();
    let args = KrdantaArgs::builder()
        .krt(krt.into())
        .upapada(upapada)
        .build()
        .unwrap();
    assert_padas(
        derive_krdantas(&a, &dhatu.clone().with_prefixes(prefixes), args),
        expected,
    );
}

// Taddhitanta helpers
// -------------------

pub fn assert_has_taddhitanta(prati: &Pratipadika, t: Taddhita, expected: &[&str]) {
    assert_padas(derive_taddhitantas(prati, t, None), expected);
}

pub fn assert_has_artha_taddhita(
    prati: &str,
    requested_artha: TaddhitaArtha,
    t: Taddhita,
    expected: &[&str],
) {
    let pratipadika = Pratipadika::new(prati);
    let mut prakriyas = derive_taddhitantas(&pratipadika, t, Some(requested_artha));
    prakriyas.retain(|p| {
        if let Some(Artha::Taddhita(prakriya_artha)) = p.artha() {
            requested_artha.is_type_of(prakriya_artha)
        } else {
            false
        }
    });
    assert_padas(prakriyas, expected);
}

// Subanta helpers
// ---------------

/// Like `assert_has_subantas_p` but without any filtering on the last sound.
/// (Needed for 8.4.56.)
pub fn assert_has_subantas_raw(
    pratipadika_text: &str,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
    expected: &[&str],
) {
    let pratipadika = Pratipadika::new(pratipadika_text);
    let a = Ashtadhyayi::new();
    let args = SubantaArgs::builder()
        .linga(linga)
        .vacana(vacana)
        .vibhakti(vibhakti)
        .build()
        .unwrap();

    let mut results = a.derive_subantas(&pratipadika, &args);
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());
    let actual: Vec<_> = results.into_iter().collect();
    assert_padas(actual, expected);
}

pub trait Prati {
    fn to_p(self) -> Pratipadika;
}

impl Prati for &Pratipadika {
    fn to_p(self) -> Pratipadika {
        self.clone()
    }
}

impl Prati for &str {
    fn to_p(self) -> Pratipadika {
        Pratipadika::new(self)
    }
}

macro_rules! test_sup {
    ($fn_name:ident, $vibhakti:expr, $vacana:expr) => {
        pub fn $fn_name(prati: impl Prati, linga: Linga, expected: &[&str]) {
            let t = Tester::default();
            t.assert_has_sup(&prati.to_p(), linga, $vibhakti, $vacana, &expected);
        }
    };
}

// TODO: move these to Tester.
test_sup!(assert_has_sup_1s, Prathama, Eka);
test_sup!(assert_has_sup_1d, Prathama, Dvi);
test_sup!(assert_has_sup_1p, Prathama, Bahu);
test_sup!(assert_has_sup_2s, Dvitiya, Eka);
test_sup!(assert_has_sup_2d, Dvitiya, Dvi);
test_sup!(assert_has_sup_2p, Dvitiya, Bahu);
test_sup!(assert_has_sup_3s, Trtiya, Eka);
test_sup!(assert_has_sup_3d, Trtiya, Dvi);
test_sup!(assert_has_sup_3p, Trtiya, Bahu);
test_sup!(assert_has_sup_4s, Caturthi, Eka);
test_sup!(assert_has_sup_4d, Caturthi, Dvi);
test_sup!(assert_has_sup_4p, Caturthi, Bahu);
test_sup!(assert_has_sup_5s, Panchami, Eka);
test_sup!(assert_has_sup_5d, Panchami, Dvi);
test_sup!(assert_has_sup_5p, Panchami, Bahu);
test_sup!(assert_has_sup_6s, Sasthi, Eka);
test_sup!(assert_has_sup_6d, Sasthi, Dvi);
test_sup!(assert_has_sup_6p, Sasthi, Bahu);
test_sup!(assert_has_sup_7s, Saptami, Eka);
test_sup!(assert_has_sup_7d, Saptami, Dvi);
test_sup!(assert_has_sup_7p, Saptami, Bahu);
test_sup!(assert_has_sup_ss, Sambodhana, Eka);
test_sup!(assert_has_sup_sd, Sambodhana, Dvi);
test_sup!(assert_has_sup_sp, Sambodhana, Bahu);
