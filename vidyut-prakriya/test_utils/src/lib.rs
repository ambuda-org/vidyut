/*!
Various test utils.

vidyut-prakriya has thousands of assert statements. This module contains various functions to
manage the boilerplate required for these assertions.
*/
extern crate vidyut_prakriya;

use std::convert::TryInto;
use vidyut_prakriya::args::Antargana;
use vidyut_prakriya::args::DhatuPada::*;
use vidyut_prakriya::args::Prayoga::*;
use vidyut_prakriya::args::Purusha as P;
use vidyut_prakriya::args::SamasaType::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Rule;
use vidyut_prakriya::Vyakarana;
use vidyut_prakriya::{Decision, Prakriya};

fn pum_s(pratipadika: Pratipadika, vibhakti: Vibhakti) -> Subanta {
    Subanta::new(pratipadika, Linga::Pum, vibhakti, Vacana::Eka)
}

/// A wrapper for `Pratipadika` that supports From<&str> (as opposed to TryFrom<&str>).
#[derive(Clone)]
pub struct SafePratipadika(pub Pratipadika);

impl From<&str> for SafePratipadika {
    fn from(val: &str) -> Self {
        Self(Pratipadika::basic(Slp1String::from(val).expect("ok")))
    }
}

impl From<&Krdanta> for SafePratipadika {
    fn from(val: &Krdanta) -> Self {
        Self(val.into())
    }
}

impl From<Krdanta> for SafePratipadika {
    fn from(val: Krdanta) -> Self {
        Self(val.into())
    }
}

impl From<&Pratipadika> for SafePratipadika {
    fn from(val: &Pratipadika) -> Self {
        Self(val.into())
    }
}

impl From<Pratipadika> for SafePratipadika {
    fn from(val: Pratipadika) -> Self {
        Self(val)
    }
}

impl From<Samasa> for SafePratipadika {
    fn from(val: Samasa) -> Self {
        Self(val.into())
    }
}

impl From<&Samasa> for SafePratipadika {
    fn from(val: &Samasa) -> Self {
        Self(val.into())
    }
}

impl From<&Taddhitanta> for SafePratipadika {
    fn from(val: &Taddhitanta) -> Self {
        Self(val.into())
    }
}

impl From<Taddhitanta> for SafePratipadika {
    fn from(val: Taddhitanta) -> Self {
        Self(val.into())
    }
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
    ignore_va_padantasya: bool,
}

impl Tester {
    /// Creates a tester with our default settings.
    pub fn new(vyakarana: Vyakarana) -> Self {
        Self {
            vyakarana,
            ignore_va_padantasya: true,
        }
    }

    /// Creates a tester that enables chAndasa rules.
    pub fn with_chaandasa() -> Self {
        Self::new(Vyakarana::builder().is_chandasi(true).build())
    }

    /// Creates a tester that enables svara rules.
    pub fn with_svara_rules() -> Self {
        Self::new(Vyakarana::builder().use_svaras(true).build())
    }

    pub fn with_nlp_mode() -> Self {
        Self::new(Vyakarana::builder().nlp_mode(true).build())
    }

    pub fn with_ignore_va_padantasya(mut self: Self, val: bool) -> Self {
        self.ignore_va_padantasya = val;
        self
    }

    /// Derives tinantas from the given conditions.
    pub fn derive_tinantas(&self, args: &Tinanta) -> Vec<Prakriya> {
        self.vyakarana.derive_tinantas(args)
    }

    /// Derives subantas from the given conditions.
    pub fn derive_subantas(&self, args: &Subanta) -> Vec<Prakriya> {
        self.vyakarana.derive_subantas(args)
    }

    /// Derives krdantas from the given conditions.
    pub fn derive_krdantas(&self, args: &Krdanta) -> Vec<Prakriya> {
        self.vyakarana.derive_krdantas(args)
    }

    /// Derives taddhitantas from the given conditions.
    pub fn derive_taddhitantas(&self, args: &Taddhitanta) -> Vec<Prakriya> {
        self.vyakarana.derive_taddhitantas(args)
    }

    /// Derives taddhitantas in a specific meaning context from the given conditions.
    fn derive_artha_taddhitantas(
        &self,
        p: impl Into<SafePratipadika>,
        t: Taddhita,
        a: Option<TaddhitaArtha>,
    ) -> Vec<Prakriya> {
        let args = if let Some(a) = a {
            Taddhitanta::builder()
                .pratipadika(p.into().0)
                .taddhita(t)
                .artha(a)
                .build()
                .unwrap()
        } else {
            taddhitanta(p.into(), t)
        };
        self.derive_taddhitantas(&args)
    }

    /// Derives vakyas from the given initial conditions.
    fn derive_vakyas(&self, padas: &[Pada]) -> Vec<Prakriya> {
        self.vyakarana.derive_vakyas(padas)
    }

    /// Asserts that the given input conditions produce the tinantas `expected`.
    pub fn assert_has_tinantas(&self, args: &Tinanta, expected: &[&str]) {
        let mut actual = self.derive_tinantas(args);
        if self.ignore_va_padantasya {
            actual.retain(|p| !uses_va_padantasya(p) && !is_noisy_pada(p) && !has_bad_final(p));
        }
        sort_and_dedup(&mut actual);
        assert_has_results(actual, expected);
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
        let mut actual = self.derive_subantas(&args);
        if self.ignore_va_padantasya {
            actual.retain(|p| !uses_va_padantasya(p) && !is_noisy_pada(p) && !has_bad_final(p));
        }
        sort_and_dedup(&mut actual);
        assert_has_results(actual, expected);
    }

    pub fn assert_has_krdanta(
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
        let mut actual = self.derive_krdantas(&spec);
        if self.ignore_va_padantasya {
            actual.retain(|p| !uses_va_padantasya(p) && !is_noisy_pada(p));
        }
        sort_and_dedup(&mut actual);
        assert_has_results(actual, expected);
    }

    pub fn assert_has_upapada_krdanta(
        &self,
        upapada: impl Into<SafePratipadika>,
        prefixes: &[&str],
        dhatu: &Dhatu,
        krt: impl Into<Krt>,
        expected: &[&str],
    ) {
        let args = upapada_krdanta(upapada, prefixes, dhatu, krt);
        let mut actual = self.derive_krdantas(&args);
        if self.ignore_va_padantasya {
            actual.retain(|p| !uses_va_padantasya(p) && !is_noisy_pada(p));
        }
        assert_has_results(actual, expected);
    }

    pub fn assert_has_taddhita(
        &self,
        prati: impl Into<SafePratipadika>,
        t: Taddhita,
        expected: &[&str],
    ) {
        let pratipadika = prati.into();
        let mut actual = self.derive_artha_taddhitantas(pratipadika.clone(), t, None);
        if self.ignore_va_padantasya {
            actual.retain(|p| !uses_va_padantasya(p));
        }
        assert_has_results(actual, expected);
    }

    pub fn assert_has_artha_taddhita(
        &self,
        prati: impl Into<SafePratipadika>,
        requested_artha: TaddhitaArtha,
        t: Taddhita,
        expected: &[&str],
    ) {
        let mut actual = self.derive_artha_taddhitantas(prati.into(), t, Some(requested_artha));
        actual.retain(|p| {
            if let Some(Artha::Taddhita(prakriya_artha)) = p.artha() {
                requested_artha.is_type_of(prakriya_artha)
            } else {
                false
            }
        });

        if self.ignore_va_padantasya {
            actual.retain(|p| !uses_va_padantasya(p) && !is_noisy_pada(p));
        }
        sort_and_dedup(&mut actual);
        assert_has_results(actual, expected);
    }

    fn assert_has_vakya(&self, padas: &[Pada], expected: &[&str]) {
        let mut prakriyas = self.vyakarana.derive_vakyas(padas);
        prakriyas.retain(|p| !is_noisy_pada(p) && !has_bad_final(p));
        assert_has_results(prakriyas, &expected);
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
    Dhatu::mula(u.try_into().expect("ok"), g)
}

pub fn d_kutadi(u: &str, g: Gana) -> Dhatu {
    let mula = Muladhatu::new(u.try_into().expect("ok"), g).with_antargana(Antargana::Kutadi);
    Dhatu::Mula(mula)
}

pub fn d_akusmiya(u: &str, g: Gana) -> Dhatu {
    let mula = Muladhatu::new(u.try_into().expect("ok"), g).with_antargana(Antargana::Akusmiya);
    Dhatu::Mula(mula)
}

pub fn d_adhrshiya(u: &str, g: Gana) -> Dhatu {
    let mula = Muladhatu::new(u.try_into().expect("ok"), g).with_antargana(Antargana::Adhrshiya);
    Dhatu::Mula(mula)
}

pub fn d_ghatadi(u: &str, g: Gana) -> Dhatu {
    let mula = Muladhatu::new(u.try_into().expect("ok"), g).with_antargana(Antargana::Ghatadi);
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

pub fn kyac(pratipadika: Pratipadika) -> Dhatu {
    Dhatu::nama(pratipadika, Some(Sanadi::kyac))
}

pub fn kyan(pratipadika: Pratipadika) -> Dhatu {
    Dhatu::nama(pratipadika, Some(Sanadi::kyaN))
}

/// Marks a dhatu as taking yaN-pratyaya with luk.
pub fn yan_luk(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::yaNluk])
}

pub fn phit(s: &str) -> Pratipadika {
    Pratipadika::basic(Slp1String::from(s).expect("ok"))
}

pub fn krdanta(prefixes: &[&str], d: &Dhatu, krt: impl Into<Krt>) -> Krdanta {
    Krdanta::builder()
        .dhatu(d.clone().with_prefixes(prefixes))
        .krt(krt)
        .build()
        .unwrap()
}

pub fn upapada_krdanta(
    upapada: impl Into<SafePratipadika>,
    prefixes: &[&str],
    d: &Dhatu,
    krt: impl Into<Krt>,
) -> Krdanta {
    let upapada = Subanta::new(
        upapada.into().0,
        Linga::Pum,
        Vibhakti::Prathama,
        Vacana::Eka,
    );
    Krdanta::builder()
        .dhatu(d.clone().with_prefixes(prefixes))
        .krt(krt)
        .upapada(upapada)
        .build()
        .unwrap()
}

pub fn taddhitanta(prati: impl Into<SafePratipadika>, taddhita: Taddhita) -> Taddhitanta {
    Taddhitanta::builder()
        .pratipadika(prati.into().0)
        .taddhita(taddhita)
        .build()
        .unwrap()
}

pub fn artha_taddhitanta(
    prati: impl Into<SafePratipadika>,
    artha: TaddhitaArtha,
    taddhita: Taddhita,
) -> Taddhitanta {
    Taddhitanta::builder()
        .pratipadika(prati.into().0)
        .artha(artha)
        .taddhita(taddhita)
        .build()
        .unwrap()
}

/// Shorthand for building a pratipadika that ends with NI/Ap.
pub fn nyap(text: &str) -> Pratipadika {
    Pratipadika::nyap(text.try_into().expect("ok"))
}

pub fn karmadharaya(x: impl Into<SafePratipadika>, y: impl Into<SafePratipadika>) -> Samasa {
    use Vibhakti::*;
    Samasa::builder()
        .padas(vec![
            pum_s(x.into().0, Prathama),
            pum_s(y.into().0, Prathama),
        ])
        .samasa_type(SamasaType::Karmadharaya)
        .build()
        .unwrap()
}

pub fn tatpurusha(
    x: impl Into<SafePratipadika>,
    y: impl Into<SafePratipadika>,
    vibhakti: Vibhakti,
) -> Samasa {
    use Vibhakti::*;
    Samasa::builder()
        .padas(vec![
            pum_s(x.into().0, vibhakti),
            pum_s(y.into().0, Prathama),
        ])
        .samasa_type(SamasaType::Tatpurusha)
        .build()
        .unwrap()
}

pub fn avyaya_tatpurusha(x: impl Into<SafePratipadika>, y: impl Into<SafePratipadika>) -> Samasa {
    let padas = vec![
        Subanta::avyaya(x.into().0),
        Subanta::new(y.into().0, Linga::Pum, Vibhakti::Prathama, Vacana::Eka),
    ];
    Samasa::builder()
        .padas(padas)
        .samasa_type(Tatpurusha)
        .build()
        .unwrap()
}

pub fn bahuvrihi(x: impl Into<SafePratipadika>, y: impl Into<SafePratipadika>) -> Samasa {
    use Vibhakti::*;
    Samasa::builder()
        .padas(vec![
            pum_s(x.into().0, Prathama),
            pum_s(y.into().0, Prathama),
        ])
        .samasa_type(SamasaType::Bahuvrihi)
        .build()
        .unwrap()
}

/// ------------------------------------------------------------------------------------
/// Tinantas
/// ------------------------------------------------------------------------------------

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
    let args = Tinanta::builder()
        .dhatu(dhatu.clone().with_prefixes(prefixes))
        .prayoga(Prayoga::Kartari)
        .purusha(purusha)
        .vacana(vacana)
        .lakara(lakara)
        .build()
        .unwrap();
    t.assert_has_tinantas(&args, expected);
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
                self.assert_has_tinantas(&args, expected);
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
            let args = Tinanta::builder()
                .dhatu(dhatu.clone().with_prefixes(prefixes))
                .prayoga(Prayoga::Kartari)
                .purusha(Purusha::Prathama)
                .vacana(Vacana::Eka)
                .lakara($la)
                .build()
                .unwrap();
            t.assert_has_tinantas(&args, expected);
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

macro_rules! assert_sup {
    ($fn_name:ident, $vibhakti:expr, $vacana:expr) => {
        impl Tester {
            pub fn $fn_name(
                &self,
                prati: impl Into<SafePratipadika>,
                linga: Linga,
                expected: &[&str],
            ) {
                self.assert_has_subantas(&prati.into().0, linga, $vibhakti, $vacana, &expected);
            }
        }

        pub fn $fn_name(prati: impl Into<SafePratipadika>, linga: Linga, expected: &[&str]) {
            let t = Tester::default();
            t.assert_has_subantas(&prati.into().0, linga, $vibhakti, $vacana, &expected);
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
        pub fn $fn_name(_expected: &str, prati: impl Into<SafePratipadika>, linga: Linga) -> Pada {
            Subanta::builder()
                .pratipadika(prati.into().0)
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
    let pratipadika = Pratipadika::basic(pratipadika_text.try_into().expect("ok"));
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

pub fn assert_has_krdanta(
    prefixes: &[&str],
    dhatu: &Dhatu,
    krt: impl Into<Krt>,
    expected: &[&str],
) {
    let t = Tester::default();
    t.assert_has_krdanta(prefixes, dhatu, krt, expected);
}

pub fn assert_has_artha_krdanta(
    upapadas: &[&str],
    dhatu: &Dhatu,
    requested_artha: KrtArtha,
    krt: impl Into<Krt>,
    expected: &[&str],
) {
    let krdanta = Krdanta::builder()
        .dhatu(dhatu.clone().with_prefixes(upapadas))
        .krt(krt.into())
        .artha(requested_artha)
        .build()
        .unwrap();

    let t = Tester::default();
    let mut actual = t.derive_krdantas(&krdanta);
    actual.retain(|p| {
        if let Some(Artha::Krt(prakriya_artha)) = p.artha() {
            requested_artha == prakriya_artha
        } else {
            false
        }
    });

    actual.retain(|p| !uses_va_padantasya(p) && !is_noisy_pada(p));
    assert_has_results(actual, expected);
}

pub fn assert_has_upapada_krdanta(
    upapada: impl Into<SafePratipadika>,
    prefixes: &[&str],
    dhatu: &Dhatu,
    krt: impl Into<Krt>,
    expected: &[&str],
) {
    let t = Tester::default();
    t.assert_has_upapada_krdanta(upapada, prefixes, dhatu, krt, expected);
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

/// ------------------------------------------------------------------------------------
/// Taddhitantas
/// ------------------------------------------------------------------------------------

/// Creates a krdanta as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_taddhitanta(
    text: &str,
    base: impl Into<SafePratipadika>,
    taddhita: Taddhita,
) -> Taddhitanta {
    taddhitanta(base, taddhita).with_require(text)
}

/// Creates a krdanta as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_artha_taddhita(
    _text: &str,
    base: impl Into<SafePratipadika>,
    artha: TaddhitaArtha,
    taddhita: Taddhita,
) -> Taddhitanta {
    Taddhitanta::builder()
        .pratipadika(base.into().0)
        .taddhita(taddhita)
        .artha(artha)
        .build()
        .unwrap()
}

pub fn assert_has_taddhita(
    prati: impl Into<SafePratipadika>,
    taddhita: Taddhita,
    expected: &[&str],
) {
    let t = Tester::default();
    t.assert_has_taddhita(prati.into(), taddhita, expected);
}

pub fn assert_has_artha_taddhita(
    prati: impl Into<SafePratipadika>,
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
        let mut actual = self.vyakarana.derive_samasas(&args);
        if self.ignore_va_padantasya {
            actual.retain(|p| !uses_va_padantasya(p));
        }
        actual.sort_by_key(|p| p.text());
        actual.dedup_by_key(|p| p.text());
        assert_has_results(actual, expected);
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
        a: impl Into<SafePratipadika>,
        b: impl Into<SafePratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&bahuvrihi(a, b), expected);
    }

    fn assert_has_avyayibhava(
        &self,
        a: impl Into<SafePratipadika>,
        b: impl Into<SafePratipadika>,
        expected: &[&str],
    ) {
        let padas = vec![
            Subanta::avyaya(a.into().0),
            Subanta::new(b.into().0, Linga::Pum, Vibhakti::Prathama, Vacana::Eka),
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
        a: impl Into<SafePratipadika>,
        b: impl Into<SafePratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&karmadharaya(a, b), expected);
    }

    pub fn assert_has_dvitiya_tatpurusha(
        &self,
        a: impl Into<SafePratipadika>,
        b: impl Into<SafePratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&tatpurusha(a, b, Vibhakti::Dvitiya), expected);
    }

    pub fn assert_has_trtiya_tatpurusha(
        &self,
        a: impl Into<SafePratipadika>,
        b: impl Into<SafePratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&tatpurusha(a, b, Vibhakti::Trtiya), expected);
    }

    fn assert_has_caturthi_tatpurusha(
        &self,
        a: impl Into<SafePratipadika>,
        b: impl Into<SafePratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&tatpurusha(a, b, Vibhakti::Caturthi), expected);
    }

    fn assert_has_panchami_tatpurusha(
        &self,
        a: impl Into<SafePratipadika>,
        b: impl Into<SafePratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&tatpurusha(a, b, Vibhakti::Panchami), expected);
    }

    fn assert_has_sasthi_tatpurusha(
        &self,
        a: impl Into<SafePratipadika>,
        b: impl Into<SafePratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&tatpurusha(a, b, Vibhakti::Sasthi), expected);
    }

    fn assert_has_saptami_tatpurusha(
        &self,
        a: impl Into<SafePratipadika>,
        b: impl Into<SafePratipadika>,
        expected: &[&str],
    ) {
        self.assert_has_samasas(&tatpurusha(a, b, Vibhakti::Saptami), expected);
    }
}

macro_rules! assert_samasa {
    ($fn_name:ident) => {
        pub fn $fn_name(
            purva: impl Into<SafePratipadika>,
            uttara: impl Into<SafePratipadika>,
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
    first: impl Into<SafePratipadika>,
    second: impl Into<SafePratipadika>,
    expected: &[&str],
) {
    let t = Tester::default();
    t.assert_samasa_of_type(
        &[
            Subanta::avyaya(first.into().0),
            pum_s(second.into().0, Prathama),
        ],
        Tatpurusha,
        expected,
    );
}

pub fn assert_has_misc_tatpurusha(
    first: impl Into<SafePratipadika>,
    second: impl Into<SafePratipadika>,
    expected: &[&str],
) {
    assert_has_sasthi_tatpurusha(first, second, expected);
}

pub fn assert_has_dvandva(items: &[&str], expected: &[&str]) {
    let args = Samasa::builder()
        .padas(
            items
                .iter()
                .map(|s| {
                    pum_s(
                        Pratipadika::basic((*s).try_into().expect("ok")),
                        Vibhakti::Prathama,
                    )
                })
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
                .map(|s| {
                    pum_s(
                        Pratipadika::basic((*s).try_into().expect("ok")),
                        Vibhakti::Prathama,
                    )
                })
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
    first: impl Into<SafePratipadika>,
    second: impl Into<SafePratipadika>,
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

pub fn has_bad_final(p: &Prakriya) -> bool {
    let text = p.text();
    ['d', 'q', 'g', 'b'].iter().any(|c| text.ends_with(*c))
}

fn is_noisy_pada(p: &Prakriya) -> bool {
    p.text().contains("cS")
}

fn uses_va_padantasya(p: &Prakriya) -> bool {
    p.rule_choices()
        .iter()
        .any(|r| r.rule() == Rule::Ashtadhyayi("8.4.59") && r.decision() == Decision::Accept)
}

pub fn sort_and_dedup(p: &mut Vec<Prakriya>) {
    p.sort_by_key(|p| p.text());
    p.dedup_by_key(|p| p.text());
}

fn debug_text(rule: Rule) -> String {
    match rule {
        Rule::Ashtadhyayi(x) => x.to_string(),
        Rule::Varttika(x) => format!("vA {x}"),
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
        print_prakriya(p);
    }
}

pub fn print_prakriya(p: &Prakriya) {
    for step in p.history() {
        let mut result = String::new();
        for (i, t) in step.result().iter().enumerate() {
            if i != 0 {
                result += " + ";
            }
            if t.was_changed() {
                result += &format!("[{}]", t.text());
            } else {
                result += t.text();
            }
        }
        println!("{} --> {}", debug_text(step.rule()), result);
    }
    println!("{:?}", p.rule_choices());
    println!();
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

    assert_eq!(expected, actuals);
}

pub fn assert_matches_prakriya(p: &Prakriya, expected: &[(Rule, Vec<&str>)]) {
    let mut i_start = 0;
    for (e_rule, e_result) in expected {
        let i = p.history()[i_start..]
            .iter()
            .enumerate()
            .find(|(_, x)| x.rule() == *e_rule)
            .map(|x| x.0);

        if i.is_none() {
            print_prakriya(p);
            assert!(
                false,
                "Could not find expected rule {:?} in prakriya.",
                e_rule
            );
        }

        i_start += i.unwrap();

        let step = &p.history()[i_start];
        let items: Vec<_> = step.result().iter().map(|x| x.text()).collect();
        if &items != e_result {
            print_prakriya(p);
            assert!(
                false,
                "Mismatch for prakriya on code {:?}, {}.",
                e_rule, i_start
            );
        }
    }
}
