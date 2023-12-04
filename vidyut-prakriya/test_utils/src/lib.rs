/*!
Various test utils.

vidyut-prakriya has thousands of assert statements. This module contains various functions to
manage the boilerplate required for these assertions.
*/
extern crate vidyut_prakriya;

use vidyut_prakriya::args::Antargana;
use vidyut_prakriya::args::DhatuPada::*;
use vidyut_prakriya::args::Prayoga::*;
use vidyut_prakriya::args::Purusha as P;
use vidyut_prakriya::args::SamasaType::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Prakriya;
use vidyut_prakriya::Rule;
use vidyut_prakriya::Vyakarana;

fn pum_s(pratipadika: Pratipadika, vibhakti: Vibhakti) -> Subanta {
    Subanta::new(pratipadika, Linga::Pum, vibhakti, Vacana::Eka)
}

/// A handy way to manage various assertions.
///
/// `Vyakarana` with its default settings will ignore certain rules. Some of these rules are
/// chAndasa rules, meaning that they apply only in Vedic usage. Other rules are valid options but
/// will overgenerate, which adds too much noise to our other tests.
///
/// To test these ignored rules, we can create a new `Tester` that uses the `Vyakarana` settings we
/// specify. For convenience, all of these methods have wrapper functions that assume the
/// default settings for `Vyakarana`.
#[derive(Debug)]
pub struct Tester {
    vyakarana: Vyakarana,
}

impl Tester {
    /// Creates a tester with our default settings.
    pub fn new(ashtadhyayi: Vyakarana) -> Self {
        Self {
            vyakarana: ashtadhyayi,
        }
    }

    /// Creates a tester that enables chAndasa rules.
    pub fn with_chaandasa() -> Self {
        Self {
            vyakarana: Vyakarana::builder().is_chandasi(true).build(),
        }
    }

    /// Creates a tester that enables svara rules.
    pub fn with_svara_rules() -> Self {
        Self {
            vyakarana: Vyakarana::builder().use_svaras(true).build(),
        }
    }
}

impl Default for Tester {
    fn default() -> Self {
        Self::new(Vyakarana::new())
    }
}

// Shorthand
// ---------

pub fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::mula(u, g)
}

pub fn d_kutadi(u: &str, g: Gana) -> Dhatu {
    let mula = Muladhatu::new(u, g).with_antargana(Antargana::Kutadi);
    Dhatu::Mula(mula)
}

pub fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::san])
}

pub fn yan(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::yaN])
}

pub fn nic(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Ric])
}

pub fn san_nic(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::san, Sanadi::Ric])
}

pub fn nic_san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Ric, Sanadi::san])
}

pub fn yan_nic(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::yaN, Sanadi::Ric])
}

/// Marks a dhatu as taking yaN-pratyaya with luk.
pub fn yan_luk(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::yaNluk])
}

pub fn krdanta(prefixes: &[&str], d: &Dhatu, krt: impl Into<Krt>) -> Krdanta {
    Krdanta::builder()
        .dhatu(d.clone().with_prefixes(prefixes))
        .krt(krt)
        .build()
        .unwrap()
}

pub fn upapada_krdanta(
    upapada: impl Into<Pratipadika>,
    prefixes: &[&str],
    d: &Dhatu,
    krt: impl Into<Krt>,
) -> Krdanta {
    let upapada = Subanta::new(upapada.into(), Linga::Pum, Vibhakti::Prathama, Vacana::Eka);
    Krdanta::builder()
        .dhatu(d.clone().with_prefixes(prefixes))
        .krt(krt)
        .upapada(upapada)
        .build()
        .unwrap()
}

pub fn taddhitanta(prati: impl Into<Pratipadika>, taddhita: Taddhita) -> Taddhitanta {
    Taddhitanta::builder()
        .pratipadika(prati.into())
        .taddhita(taddhita)
        .build()
        .unwrap()
}

/// Shorthand for building a pratipadika that ends with NI/Ap.
pub fn nyap(text: &str) -> Pratipadika {
    Pratipadika::nyap(text)
}

pub fn karmadharaya(x: impl Into<Pratipadika>, y: impl Into<Pratipadika>) -> Samasa {
    use Vibhakti::*;
    Samasa::builder()
        .padas(vec![pum_s(x.into(), Prathama), pum_s(y.into(), Prathama)])
        .samasa_type(SamasaType::Karmadharaya)
        .build()
        .unwrap()
}

pub fn tatpurusha(
    x: impl Into<Pratipadika>,
    y: impl Into<Pratipadika>,
    vibhakti: Vibhakti,
) -> Samasa {
    use Vibhakti::*;
    Samasa::builder()
        .padas(vec![pum_s(x.into(), vibhakti), pum_s(y.into(), Prathama)])
        .samasa_type(SamasaType::Tatpurusha)
        .build()
        .unwrap()
}

pub fn avyaya_tatpurusha(x: impl Into<Pratipadika>, y: impl Into<Pratipadika>) -> Samasa {
    let padas = vec![
        Subanta::avyaya(x.into()),
        Subanta::new(y.into(), Linga::Pum, Vibhakti::Prathama, Vacana::Eka),
    ];
    Samasa::builder()
        .padas(padas)
        .samasa_type(Tatpurusha)
        .build()
        .unwrap()
}

pub fn bahuvrihi(x: impl Into<Pratipadika>, y: impl Into<Pratipadika>) -> Samasa {
    use Vibhakti::*;
    Samasa::builder()
        .padas(vec![pum_s(x.into(), Prathama), pum_s(y.into(), Prathama)])
        .samasa_type(SamasaType::Bahuvrihi)
        .build()
        .unwrap()
}

/// ------------------------------------------------------------------------------------
/// Tinantas
/// ------------------------------------------------------------------------------------

impl Tester {
    /// Derives tinantas from the given initial conditions.
    pub fn derive_tinantas(&self, args: &Tinanta) -> Vec<Prakriya> {
        self.vyakarana.derive_tinantas(args)
    }

    /// Asserts that the given input conditions produce the tinantas `expected`.
    fn assert_has_tinantas(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        lakara: Lakara,
        purusha: Purusha,
        vacana: Vacana,
        expected: &[&str],
    ) {
        let args = Tinanta::builder()
            .dhatu(dhatu.clone().with_prefixes(prefixes))
            .prayoga(Prayoga::Kartari)
            .purusha(purusha)
            .vacana(vacana)
            .lakara(lakara)
            .build()
            .unwrap();
        let actual = self.derive_tinantas(&args);
        let actual = sanitize_results(actual);
        assert_has_results(actual, expected);
    }
}

/// Checks the given combination of `dhatu` and `prefixes` produces the `expected` results given
/// this `lakara`/`purusha`/`vacana` combination.
pub fn assert_has_tinantas(
    prefixes: &[&str],
    dhatu: &Dhatu,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
    expected: &[&str],
) {
    let t = Tester::default();
    t.assert_has_tinantas(prefixes, dhatu, lakara, purusha, vacana, expected);
}

/// Creates a function alias `fn_name` that points to the method of the same name on a default
/// `Tester` instance.
macro_rules! test_tin {
    ($fn_name:ident, $prayoga:expr, $purusha:expr, $vacana:expr, $pada:expr) => {
        impl Tester {
            pub fn $fn_name(
                &self,
                prefixes: &[&str],
                dhatu: &Dhatu,
                la: Lakara,
                expected: &[&str],
            ) {
                let args = Tinanta::builder()
                    .dhatu(dhatu.clone().with_prefixes(prefixes))
                    .prayoga($prayoga)
                    .purusha($purusha)
                    .vacana($vacana)
                    .lakara(la)
                    .pada($pada)
                    .build()
                    .unwrap();
                let actual = self.derive_tinantas(&args);
                let actual = sanitize_results(actual);
                assert_has_results(actual, expected);
            }
        }

        pub fn $fn_name(prefixes: &[&str], dhatu: &Dhatu, la: Lakara, expected: &[&str]) {
            let t = Tester::default();
            t.$fn_name(prefixes, dhatu, la, expected);
        }
    };
}

test_tin!(assert_has_tip, Kartari, P::Prathama, Eka, Parasmai);
test_tin!(assert_has_tas, Kartari, P::Prathama, Dvi, Parasmai);
test_tin!(assert_has_jhi, Kartari, P::Prathama, Bahu, Parasmai);
test_tin!(assert_has_sip, Kartari, P::Madhyama, Eka, Parasmai);
test_tin!(assert_has_thas, Kartari, P::Madhyama, Dvi, Parasmai);
test_tin!(assert_has_tha, Kartari, P::Madhyama, Bahu, Parasmai);
test_tin!(assert_has_mip, Kartari, P::Uttama, Eka, Parasmai);
test_tin!(assert_has_vas, Kartari, P::Uttama, Dvi, Parasmai);
test_tin!(assert_has_mas, Kartari, P::Uttama, Bahu, Parasmai);

test_tin!(assert_has_ta, Kartari, P::Prathama, Eka, Atmane);
test_tin!(assert_has_aataam, Kartari, P::Prathama, Dvi, Atmane);
test_tin!(assert_has_jha, Kartari, P::Prathama, Bahu, Atmane);
test_tin!(assert_has_thaas, Kartari, P::Madhyama, Eka, Atmane);
test_tin!(assert_has_aathaam, Kartari, P::Madhyama, Dvi, Atmane);
test_tin!(assert_has_dhvam, Kartari, P::Madhyama, Bahu, Atmane);
test_tin!(assert_has_iw, Kartari, P::Uttama, Eka, Atmane);
test_tin!(assert_has_vahi, Kartari, P::Uttama, Dvi, Atmane);
test_tin!(assert_has_mahin, Kartari, P::Uttama, Bahu, Atmane);

test_tin!(assert_has_ta_k, Karmani, P::Prathama, Eka, Atmane);
test_tin!(assert_has_aataam_k, Karmani, P::Prathama, Dvi, Atmane);
test_tin!(assert_has_jha_k, Karmani, P::Prathama, Bahu, Atmane);
test_tin!(assert_has_thaas_k, Karmani, P::Madhyama, Eka, Atmane);
test_tin!(assert_has_aathaam_k, Karmani, P::Madhyama, Dvi, Atmane);
test_tin!(assert_has_dhvam_k, Karmani, P::Madhyama, Bahu, Atmane);
test_tin!(assert_has_iw_k, Karmani, P::Uttama, Eka, Atmane);
test_tin!(assert_has_vahi_k, Karmani, P::Uttama, Dvi, Atmane);
test_tin!(assert_has_mahin_k, Karmani, P::Uttama, Bahu, Atmane);

macro_rules! test_la {
    ($fn_name:ident, $la:expr) => {
        pub fn $fn_name(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
            let t = Tester::default();
            t.assert_has_tinantas(
                prefixes,
                dhatu,
                $la,
                Purusha::Prathama,
                Vacana::Eka,
                expected,
            );
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

/// ------------------------------------------------------------------------------------
/// Subantas
/// ------------------------------------------------------------------------------------

impl Tester {
    /// Derives tinantas from the given initial conditions.
    fn derive_subantas(&self, args: &Subanta) -> Vec<Prakriya> {
        self.vyakarana.derive_subantas(args)
    }

    fn assert_has_subantas(
        &self,
        prati: &Pratipadika,
        linga: Linga,
        vibhakti: Vibhakti,
        vacana: Vacana,
        expected: &[&str],
    ) {
        let args = Subanta::builder()
            .pratipadika(prati.clone())
            .linga(linga)
            .vacana(vacana)
            .vibhakti(vibhakti)
            .build()
            .unwrap();
        let prakriyas = self.derive_subantas(&args);
        let actual = sanitize_results(prakriyas);
        assert_has_results(actual, expected);
    }
}

macro_rules! assert_sup {
    ($fn_name:ident, $vibhakti:expr, $vacana:expr) => {
        impl Tester {
            pub fn $fn_name(&self, prati: impl Into<Pratipadika>, linga: Linga, expected: &[&str]) {
                self.assert_has_subantas(&prati.into(), linga, $vibhakti, $vacana, &expected);
            }
        }

        pub fn $fn_name(prati: impl Into<Pratipadika>, linga: Linga, expected: &[&str]) {
            let t = Tester::default();
            t.assert_has_subantas(&prati.into(), linga, $vibhakti, $vacana, &expected);
        }
    };
}

assert_sup!(assert_has_sup_1s, Prathama, Eka);
assert_sup!(assert_has_sup_1d, Prathama, Dvi);
assert_sup!(assert_has_sup_1p, Prathama, Bahu);
assert_sup!(assert_has_sup_2s, Dvitiya, Eka);
assert_sup!(assert_has_sup_2d, Dvitiya, Dvi);
assert_sup!(assert_has_sup_2p, Dvitiya, Bahu);
assert_sup!(assert_has_sup_3s, Trtiya, Eka);
assert_sup!(assert_has_sup_3d, Trtiya, Dvi);
assert_sup!(assert_has_sup_3p, Trtiya, Bahu);
assert_sup!(assert_has_sup_4s, Caturthi, Eka);
assert_sup!(assert_has_sup_4d, Caturthi, Dvi);
assert_sup!(assert_has_sup_4p, Caturthi, Bahu);
assert_sup!(assert_has_sup_5s, Panchami, Eka);
assert_sup!(assert_has_sup_5d, Panchami, Dvi);
assert_sup!(assert_has_sup_5p, Panchami, Bahu);
assert_sup!(assert_has_sup_6s, Sasthi, Eka);
assert_sup!(assert_has_sup_6d, Sasthi, Dvi);
assert_sup!(assert_has_sup_6p, Sasthi, Bahu);
assert_sup!(assert_has_sup_7s, Saptami, Eka);
assert_sup!(assert_has_sup_7d, Saptami, Dvi);
assert_sup!(assert_has_sup_7p, Saptami, Bahu);
assert_sup!(assert_has_sup_ss, Sambodhana, Eka);
assert_sup!(assert_has_sup_sd, Sambodhana, Dvi);
assert_sup!(assert_has_sup_sp, Sambodhana, Bahu);

macro_rules! create_sup {
    ($fn_name:ident, $vibhakti:expr, $vacana:expr) => {
        pub fn $fn_name(_expected: &str, prati: impl Into<Pratipadika>, linga: Linga) -> Pada {
            Subanta::builder()
                .pratipadika(prati.into())
                .linga(linga)
                .vibhakti($vibhakti)
                .vacana($vacana)
                .build()
                .unwrap()
                .into()
        }
    };
}

create_sup!(sup_1s, Prathama, Eka);
create_sup!(sup_1d, Prathama, Dvi);
create_sup!(sup_1p, Prathama, Bahu);
create_sup!(sup_2s, Dvitiya, Eka);
create_sup!(sup_2d, Dvitiya, Dvi);
create_sup!(sup_2p, Dvitiya, Bahu);
create_sup!(sup_3s, Trtiya, Eka);
create_sup!(sup_3d, Trtiya, Dvi);
create_sup!(sup_3p, Trtiya, Bahu);
create_sup!(sup_4s, Caturthi, Eka);
create_sup!(sup_4d, Caturthi, Dvi);
create_sup!(sup_4p, Caturthi, Bahu);
create_sup!(sup_5s, Panchami, Eka);
create_sup!(sup_5d, Panchami, Dvi);
create_sup!(sup_5p, Panchami, Bahu);
create_sup!(sup_6s, Sasthi, Eka);
create_sup!(sup_6d, Sasthi, Dvi);
create_sup!(sup_6p, Sasthi, Bahu);
create_sup!(sup_7s, Saptami, Eka);
create_sup!(sup_7d, Saptami, Dvi);
create_sup!(sup_7p, Saptami, Bahu);
create_sup!(sup_ss, Sambodhana, Eka);
create_sup!(sup_sd, Sambodhana, Dvi);
create_sup!(sup_sp, Sambodhana, Bahu);

/// Like `assert_has_subantas` but without any filtering on the last sound.
/// (Needed for 8.4.56.)
pub fn assert_has_subantas_raw(
    pratipadika_text: &str,
    linga: Linga,
    vibhakti: Vibhakti,
    vacana: Vacana,
    expected: &[&str],
) {
    let pratipadika = Pratipadika::basic(pratipadika_text);
    let v = Vyakarana::new();
    let args = Subanta::builder()
        .pratipadika(pratipadika)
        .linga(linga)
        .vacana(vacana)
        .vibhakti(vibhakti)
        .build()
        .unwrap();

    let mut results = v.derive_subantas(&args);
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());
    let actual: Vec<_> = results.into_iter().collect();
    assert_has_results(actual, expected);
}

/// ------------------------------------------------------------------------------------
/// Krdantas
/// ------------------------------------------------------------------------------------

impl Tester {
    fn derive_krdantas(&self, args: &Krdanta) -> Vec<Prakriya> {
        self.vyakarana.derive_krdantas(args)
    }

    pub fn assert_has_krt(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        krt: impl Into<Krt>,
        expected: &[&str],
    ) {
        let spec = Krdanta::builder()
            .dhatu(dhatu.clone().with_prefixes(prefixes))
            .krt(krt.into())
            .build()
            .unwrap();
        let mut prakriyas = self.derive_krdantas(&spec);
        prakriyas.sort_by_key(|p| p.text());
        prakriyas.dedup_by_key(|p| p.text());
        // Allowed in pada sandhi, but noisy here.
        prakriyas.retain(|p| !p.text().contains("cS"));

        assert_has_results(prakriyas, expected);
    }
}

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
    let v = Vyakarana::new();
    let krdanta = Krdanta::builder()
        .dhatu(dhatu.clone().with_prefixes(upapadas))
        .krt(krt.into())
        .artha(requested_artha)
        .build()
        .unwrap();
    let mut prakriyas = derive_krdantas(&v, &krdanta);

    prakriyas.retain(|p| {
        if let Some(Artha::Krt(prakriya_artha)) = p.artha() {
            requested_artha == prakriya_artha
        } else {
            false
        }
    });
    assert_has_results(prakriyas, expected);
}

pub fn assert_has_upapada_krdanta(
    upapada: impl Into<Pratipadika>,
    prefixes: &[&str],
    dhatu: &Dhatu,
    krt: impl Into<Krt>,
    expected: &[&str],
) {
    let v = Vyakarana::new();
    let args = upapada_krdanta(upapada, prefixes, dhatu, krt);
    assert_has_results(derive_krdantas(&v, &args), expected);
}

/// Creates a krdanta as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_krdanta(text: &str, prefixes: &[&str], d: &Dhatu, krt: impl Into<Krt>) -> Krdanta {
    Krdanta::builder()
        .dhatu(d.clone().with_prefixes(prefixes))
        .krt(krt)
        .require(text)
        .build()
        .unwrap()
}

/// Creates a krdanta as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_upapada_krdanta(
    text: &str,
    upapada: &str,
    prefixes: &[&str],
    d: &Dhatu,
    krt: impl Into<Krt>,
) -> Krdanta {
    upapada_krdanta(upapada, prefixes, d, krt).with_require(text)
}

/// Derives krdantas from the given initial conditions.
fn derive_krdantas(a: &Vyakarana, krdanta: &Krdanta) -> Vec<Prakriya> {
    let mut results = a.derive_krdantas(krdanta);
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());
    // Allowed in pada sandhi, but noisy here.
    results.retain(|p| !p.text().contains("cS"));
    results
}

/// ------------------------------------------------------------------------------------
/// Taddhitantas
/// ------------------------------------------------------------------------------------

impl Tester {
    /// Derives taddhitantas from the given initial conditions.
    fn derive_taddhitantas(&self, args: &Taddhitanta) -> Vec<Prakriya> {
        self.vyakarana.derive_taddhitantas(args)
    }

    /// Derives taddhitantas from the given initial conditions.
    fn derive_artha_taddhitantas(
        &self,
        p: impl Into<Pratipadika>,
        t: Taddhita,
        a: Option<TaddhitaArtha>,
    ) -> Vec<Prakriya> {
        let args = if let Some(a) = a {
            Taddhitanta::builder()
                .pratipadika(p.into())
                .taddhita(t)
                .artha(a)
                .build()
                .unwrap()
        } else {
            taddhitanta(p.into(), t)
        };
        let results = self.derive_taddhitantas(&args);
        sanitize_results(results)
    }

    pub fn assert_has_artha_taddhita(
        &self,
        prati: impl Into<Pratipadika>,
        requested_artha: TaddhitaArtha,
        t: Taddhita,
        expected: &[&str],
    ) {
        let mut prakriyas = self.derive_artha_taddhitantas(prati.into(), t, Some(requested_artha));
        prakriyas.retain(|p| {
            if let Some(Artha::Taddhita(prakriya_artha)) = p.artha() {
                requested_artha.is_type_of(prakriya_artha)
            } else {
                false
            }
        });
        assert_has_results(prakriyas, expected);
    }

    pub fn assert_has_taddhita(&self, prati: &str, t: Taddhita, expected: &[&str]) {
        let pratipadika = Pratipadika::basic(prati);
        let prakriyas = self.derive_artha_taddhitantas(pratipadika.clone(), t, None);
        assert_has_results(prakriyas, expected);
    }
}

/// Creates a krdanta as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_taddhitanta(
    text: &str,
    base: impl Into<Pratipadika>,
    taddhita: Taddhita,
) -> Taddhitanta {
    taddhitanta(base, taddhita).with_require(text)
}

/// Creates a krdanta as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_artha_taddhita(
    _text: &str,
    base: impl Into<Pratipadika>,
    artha: TaddhitaArtha,
    taddhita: Taddhita,
) -> Taddhitanta {
    Taddhitanta::builder()
        .pratipadika(base.into())
        .taddhita(taddhita)
        .artha(artha)
        .build()
        .unwrap()
}

pub fn assert_has_taddhita(prati: impl Into<Pratipadika>, t: Taddhita, expected: &[&str]) {
    let tester = Tester::default();
    let results = tester.derive_artha_taddhitantas(prati.into(), t, None);
    let results = sanitize_results(results);
    assert_has_results(results, expected);
}

pub fn assert_has_artha_taddhita(
    prati: impl Into<Pratipadika>,
    requested_artha: TaddhitaArtha,
    taddhita: Taddhita,
    expected: &[&str],
) {
    let t = Tester::default();
    t.assert_has_artha_taddhita(prati, requested_artha, taddhita, expected);
}

/// ------------------------------------------------------------------------------------
/// Samasas
/// ------------------------------------------------------------------------------------

impl Tester {
    pub fn assert_has_samasas(&self, args: &Samasa, expected: &[&str]) {
        let mut prakriyas = self.vyakarana.derive_samasas(&args);
        prakriyas.sort_by_key(|p| p.text());
        prakriyas.dedup_by_key(|p| p.text());
        assert_has_results(prakriyas, expected);
    }

    /// A simpler interface to `assert_has_samasas` that accepts exactly two items.
    fn assert_samasa_of_type(&self, padas: &[Subanta], samasa_type: SamasaType, expected: &[&str]) {
        let args = Samasa::builder()
            .padas(Vec::from(padas))
            .samasa_type(samasa_type)
            .build()
            .unwrap();
        self.assert_has_samasas(&args, expected);
    }

    pub fn assert_has_bahuvrihi(
        &self,
        a: impl Into<Pratipadika>,
        b: impl Into<Pratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&bahuvrihi(a, b), expected);
    }

    fn assert_has_avyayibhava(
        &self,
        a: impl Into<Pratipadika>,
        b: impl Into<Pratipadika>,
        expected: &[&str],
    ) {
        let padas = vec![
            Subanta::avyaya(a.into()),
            Subanta::new(b.into(), Linga::Pum, Vibhakti::Prathama, Vacana::Eka),
        ];
        let args = Samasa::builder()
            .padas(padas)
            .samasa_type(Avyayibhava)
            .build()
            .unwrap();

        let mut prakriyas = self.vyakarana.derive_samasas(&args);
        prakriyas.sort_by_key(|p| p.text());
        prakriyas.dedup_by_key(|p| p.text());
        let prakriyas: Vec<_> = prakriyas
            .into_iter()
            .filter(|p| {
                let text = p.text();
                !text.ends_with("d")
            })
            .collect();
        assert_has_results(prakriyas, expected);
    }

    pub fn assert_has_karmadharaya(
        &self,
        a: impl Into<Pratipadika>,
        b: impl Into<Pratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&karmadharaya(a, b), expected);
    }

    pub fn assert_has_dvitiya_tatpurusha(
        &self,
        a: impl Into<Pratipadika>,
        b: impl Into<Pratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&tatpurusha(a, b, Vibhakti::Dvitiya), expected);
    }

    pub fn assert_has_trtiya_tatpurusha(
        &self,
        a: impl Into<Pratipadika>,
        b: impl Into<Pratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&tatpurusha(a, b, Vibhakti::Trtiya), expected);
    }

    fn assert_has_caturthi_tatpurusha(
        &self,
        a: impl Into<Pratipadika>,
        b: impl Into<Pratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&tatpurusha(a, b, Vibhakti::Caturthi), expected);
    }

    fn assert_has_panchami_tatpurusha(
        &self,
        a: impl Into<Pratipadika>,
        b: impl Into<Pratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&tatpurusha(a, b, Vibhakti::Panchami), expected);
    }

    fn assert_has_sasthi_tatpurusha(
        &self,
        a: impl Into<Pratipadika>,
        b: impl Into<Pratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&tatpurusha(a, b, Vibhakti::Sasthi), expected);
    }

    fn assert_has_saptami_tatpurusha(
        &self,
        a: impl Into<Pratipadika>,
        b: impl Into<Pratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&tatpurusha(a, b, Vibhakti::Saptami), expected);
    }
}

macro_rules! assert_samasa {
    ($fn_name:ident) => {
        pub fn $fn_name(
            purva: impl Into<Pratipadika>,
            uttara: impl Into<Pratipadika>,
            expected: &[&str],
        ) {
            let t = Tester::default();
            t.$fn_name(purva, uttara, expected);
        }
    };
}

assert_samasa!(assert_has_bahuvrihi);
assert_samasa!(assert_has_avyayibhava);
assert_samasa!(assert_has_karmadharaya);
assert_samasa!(assert_has_dvitiya_tatpurusha);
assert_samasa!(assert_has_trtiya_tatpurusha);
assert_samasa!(assert_has_caturthi_tatpurusha);
assert_samasa!(assert_has_panchami_tatpurusha);
assert_samasa!(assert_has_sasthi_tatpurusha);
assert_samasa!(assert_has_saptami_tatpurusha);

pub fn assert_has_avyaya_tatpurusha(
    first: impl Into<Pratipadika>,
    second: impl Into<Pratipadika>,
    expected: &[&str],
) {
    let t = Tester::default();
    t.assert_samasa_of_type(
        &[
            Subanta::avyaya(first.into()),
            pum_s(second.into(), Prathama),
        ],
        Tatpurusha,
        expected,
    );
}

pub fn assert_has_misc_tatpurusha(
    first: impl Into<Pratipadika>,
    second: impl Into<Pratipadika>,
    expected: &[&str],
) {
    assert_has_sasthi_tatpurusha(first, second, expected);
}

pub fn assert_has_dvandva(items: &[&str], expected: &[&str]) {
    let args = Samasa::builder()
        .padas(
            items
                .iter()
                .map(|s| pum_s(Pratipadika::basic(s), Vibhakti::Prathama))
                .collect(),
        )
        .samasa_type(Dvandva)
        .build()
        .unwrap();
    let t = Tester::default();
    t.assert_has_samasas(&args, expected);
}

pub fn assert_has_samahara_dvandva(items: &[&str], expected: &[&str]) {
    let args = Samasa::builder()
        .padas(
            items
                .iter()
                .map(|s| pum_s(Pratipadika::basic(s), Vibhakti::Prathama))
                .collect(),
        )
        .samasa_type(SamaharaDvandva)
        .build()
        .unwrap();
    let t = Tester::default();
    t.assert_has_samasas(&args, expected);
}

/// Creates a samasa as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_avyaya_tatpurusha(
    _text: &str,
    first: impl Into<Pratipadika>,
    second: impl Into<Pratipadika>,
) -> Samasa {
    avyaya_tatpurusha(first, second)
}

/// Creates a samasa as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_bahuvrihi(_text: &str, first: &str, second: &str) -> Samasa {
    bahuvrihi(first, second)
}

pub fn assert_has_samasas(args: &Samasa, expected: &[&str]) {
    let t = Tester::default();
    t.assert_has_samasas(&args, expected);
}

/// ------------------------------------------------------------------------------------
/// Vakyas
/// ------------------------------------------------------------------------------------

impl Tester {
    /// Derives vakyas from the given initial conditions.
    fn derive_vakyas(&self, padas: &[Pada]) -> Vec<Prakriya> {
        self.vyakarana.derive_vakyas(padas)
    }

    fn assert_has_vakya(&self, padas: &[Pada], expected: &[&str]) {
        let prakriyas = self.vyakarana.derive_vakyas(padas);
        let prakriyas = sanitize_results(prakriyas);
        assert_has_results(prakriyas, &expected);
    }
}

pub fn assert_has_vakya(first: &Pada, second: &Pada, expected: &[&str]) {
    let t = Tester::default();
    t.assert_has_vakya(&vec![first.to_owned(), second.to_owned()], expected);
}

pub fn assert_has_sandhi(first: &str, second: &str, expected: &[&str]) {
    let prakriyas = derive_vakyas(&first, &second);
    assert_has_results(prakriyas, &expected);
}

/// Derives vakyas from the given initial conditions.
fn derive_vakyas(first: &str, second: &str) -> Vec<Prakriya> {
    let padas = vec![Pada::from_text(first), Pada::from_text(second)];

    let tester = Tester::default();
    let mut results = tester.derive_vakyas(&padas);
    results.sort_by_key(|p| p.text());
    results.dedup_by_key(|p| p.text());
    results
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
            !['d', 'q', 'g', 'b'].iter().any(|c| text.ends_with(*c)) && !text.contains("cS")
        })
        .collect()
}

fn debug_text(rule: Rule) -> String {
    match rule {
        Rule::Ashtadhyayi(x) => x.to_string(),
        Rule::Varttika(x, y) => format!("{x} v{y}"),
        Rule::Dhatupatha(x) => format!("DA {x}"),
        Rule::Kashika(x) => format!("kA {x}"),
        Rule::Kaumudi(x) => format!("kO {x}"),
        Rule::Linganushasana(x) => format!("liNga {x}"),
        Rule::Phit(x) => format!("Piw {x}"),
        Rule::Unadipatha(x) => format!("uRA {x}"),
    }
}

/// Nicely prints out the given prakriyas.
pub fn print_all_prakriyas(prakriyas: &[Prakriya]) {
    for p in prakriyas {
        for step in p.history() {
            let mut result = String::new();
            for (i, text) in step.result().iter().enumerate() {
                if i != 0 {
                    result += " + ";
                }
                if step.active() == Some(i) {
                    result += &format!("[{text}]");
                } else {
                    result += text;
                }
            }
            println!("{} --> {}", debug_text(step.rule()), result);
        }
        println!("{:?}", p.rule_choices());
        println!();
    }
}

// Heavy assert helpers
// --------------------
// These functions are too heavy for regular use. Instead, use the smaller assert functions below.

/// Asserts that the given `prakriyas` produce the `expected` results.
pub fn assert_has_results(prakriyas: Vec<Prakriya>, expected: &[&str]) {
    let expected: Vec<_> = expected.iter().map(|text| text.replace(" ", "")).collect();
    let actuals: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();

    let mut expected = Vec::from(expected);
    expected.sort();
    expected.dedup();

    let mut actuals = Vec::from(actuals);
    actuals.sort();
    actuals.dedup();

    if actuals != expected {
        print_all_prakriyas(&prakriyas);
    }

    assert_eq!(
        actuals, expected,
        "expected: {expected:#?}, actual: {actuals:#?}"
    );
}
