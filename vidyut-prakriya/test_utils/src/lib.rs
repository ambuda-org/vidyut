/*!
Various test utils.

vidyut-prakriya has thousands of assert statements. This module contains various functions to
manage the boilerplate required for these assertions.
*/
extern crate vidyut_prakriya;

use vidyut_prakriya::args::Antargana;
use vidyut_prakriya::args::Purusha as P;
use vidyut_prakriya::args::SamasaPada;
use vidyut_prakriya::args::SamasaType::*;
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

    /// Core derivation methods
    /// -----------------------

    /// Derives tinantas from the given initial conditions.
    fn derive_tinantas(&self, dhatu: &Dhatu, args: &TinantaArgs) -> Vec<Prakriya> {
        self.ashtadhyayi.derive_tinantas(dhatu, args)
    }

    /// Derives tinantas from the given initial conditions.
    fn derive_subantas(&self, p: &Pratipadika, args: &SubantaArgs) -> Vec<Prakriya> {
        self.ashtadhyayi.derive_subantas(p, args)
    }

    fn derive_krdantas(&self, dhatu: &Dhatu, args: &KrdantaArgs) -> Vec<Prakriya> {
        self.ashtadhyayi.derive_krdantas(dhatu, args)
    }

    /// Derives taddhitantas from the given initial conditions.
    fn derive_taddhitantas(&self, p: &Pratipadika, args: &TaddhitantaArgs) -> Vec<Prakriya> {
        self.ashtadhyayi.derive_taddhitantas(p, args)
    }

    /// Derives samasas from the given initial conditions.
    fn derive_stryantas(&self, p: &Pratipadika) -> Vec<Prakriya> {
        self.ashtadhyayi.derive_stryantas(p)
    }

    /// Derives samasas from the given initial conditions.
    fn derive_samasas(&self, args: &SamasaArgs) -> Vec<Prakriya> {
        self.ashtadhyayi.derive_samasas(args)
    }

    /// Derives vakyas from the given initial conditions.
    fn derive_vakyas(&self, padas: &[Pada]) -> Vec<Prakriya> {
        self.ashtadhyayi.derive_vakyas(padas)
    }

    /// Creation methods
    /// ----------------

    pub fn create_sup(
        &self,
        expected: &str,
        prati: &Pratipadika,
        linga: Linga,
        vibhakti: Vibhakti,
        vacana: Vacana,
    ) -> Pada {
        let args = SubantaArgs::builder()
            .linga(linga)
            .vibhakti(vibhakti)
            .vacana(vacana)
            .build()
            .unwrap();
        let prakriyas = self.derive_subantas(prati, &args);
        for p in prakriyas.iter() {
            if p.text() == expected {
                let pada: Option<Pada> = p.into();
                return pada.unwrap();
            }
        }
        print_all_prakriyas(&prakriyas);
        let alts: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        panic!("Could not create \"{}\". Alternatives: {alts:?}", expected);
    }

    /// Core assertion methods
    /// ----------------------

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
        let actual = self.derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
        let actual = sanitize_results(actual);
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
            .pada(DhatuPada::Parasmai)
            .build()
            .unwrap();
        let actual = self.derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
        let actual = sanitize_results(actual);
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
            .pada(DhatuPada::Atmane)
            .build()
            .unwrap();
        let actual = self.derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
        let actual = sanitize_results(actual);
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
        let actual = self.derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
        let actual = sanitize_results(actual);
        assert_padas(actual, expected);
    }

    fn assert_has_sup(
        &self,
        prati: &Pratipadika,
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
        let prakriyas = self.derive_subantas(prati, &args);
        let actual = sanitize_results(prakriyas);
        assert_padas(actual, expected);
    }

    fn assert_has_vakya(&self, padas: &[Pada], expected: &[&str]) {
        let prakriyas = self.ashtadhyayi.derive_vakyas(padas);
        let prakriyas = sanitize_results(prakriyas);

        let expected: Vec<_> = expected.iter().map(|text| text.replace(" ", "")).collect();
        let actuals: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        assert_results(&expected, &actuals, prakriyas);
    }

    /// Fine-grained assertion methods
    /// ------------------------------

    /// Asserts that the given input conditions produce the prathamA-eka tinantas in `expected`
    fn assert_has_lakara(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_tin(prefixes, d, la, P::Prathama, Eka, expected);
    }

    pub fn assert_has_tip(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, d, la, P::Prathama, Eka, expected);
    }

    pub fn assert_has_tas(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, d, la, P::Prathama, Dvi, expected);
    }

    pub fn assert_has_jhi(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, d, la, P::Prathama, Bahu, expected);
    }

    pub fn assert_has_sip(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, d, la, P::Madhyama, Eka, expected);
    }

    pub fn assert_has_thas(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, d, la, P::Madhyama, Dvi, expected);
    }

    pub fn assert_has_tha(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, d, la, P::Madhyama, Bahu, expected);
    }

    pub fn assert_has_mip(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, d, la, P::Uttama, Eka, expected);
    }

    pub fn assert_has_vas(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, d, la, P::Uttama, Dvi, expected);
    }

    pub fn assert_has_mas(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_parasmai_tin(prefixes, d, la, P::Uttama, Bahu, expected);
    }

    pub fn assert_has_ta(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_atmane_tin(prefixes, d, la, P::Prathama, Eka, expected);
    }

    pub fn assert_has_aataam(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_atmane_tin(prefixes, d, la, P::Prathama, Dvi, expected);
    }

    pub fn assert_has_jha(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_atmane_tin(prefixes, d, la, P::Prathama, Bahu, expected);
    }

    pub fn assert_has_thaas(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_atmane_tin(prefixes, d, la, P::Madhyama, Eka, expected);
    }

    pub fn assert_has_aathaam(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_atmane_tin(prefixes, d, la, P::Madhyama, Dvi, expected);
    }

    pub fn assert_has_dhvam(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_atmane_tin(prefixes, d, la, P::Madhyama, Bahu, expected);
    }

    pub fn assert_has_iw(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_atmane_tin(prefixes, d, la, P::Uttama, Eka, expected);
    }

    pub fn assert_has_vahi(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_atmane_tin(prefixes, d, la, P::Uttama, Dvi, expected);
    }

    pub fn assert_has_mahin(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_atmane_tin(prefixes, d, la, P::Uttama, Bahu, expected);
    }

    pub fn assert_has_ta_k(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_karmani_tin(prefixes, d, la, P::Prathama, Eka, expected);
    }

    pub fn assert_has_aataam_k(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_karmani_tin(prefixes, d, la, P::Prathama, Dvi, expected);
    }

    pub fn assert_has_jha_k(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_karmani_tin(prefixes, d, la, P::Prathama, Bahu, expected);
    }

    pub fn assert_has_thaas_k(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_karmani_tin(prefixes, d, la, P::Madhyama, Eka, expected);
    }

    pub fn assert_has_aathaam_k(
        &self,
        prefixes: &[&str],
        d: &Dhatu,
        la: Lakara,
        expected: &[&str],
    ) {
        self.assert_has_karmani_tin(prefixes, d, la, P::Madhyama, Dvi, expected);
    }

    pub fn assert_has_dhvam_k(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_karmani_tin(prefixes, d, la, P::Madhyama, Bahu, expected);
    }

    pub fn assert_has_iw_k(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_karmani_tin(prefixes, d, la, P::Uttama, Eka, expected);
    }

    pub fn assert_has_vahi_k(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_karmani_tin(prefixes, d, la, P::Uttama, Dvi, expected);
    }

    pub fn assert_has_mahin_k(&self, prefixes: &[&str], d: &Dhatu, la: Lakara, expected: &[&str]) {
        self.assert_has_karmani_tin(prefixes, d, la, P::Uttama, Bahu, expected);
    }

    pub fn assert_has_krt(
        &self,
        prefixes: &[&str],
        dhatu: &Dhatu,
        krt: impl Into<Krt>,
        expected: &[&str],
    ) {
        let args = KrdantaArgs::builder().krt(krt.into()).build().unwrap();
        let mut prakriyas = self.derive_krdantas(&dhatu.clone().with_prefixes(prefixes), &args);
        prakriyas.sort_by_key(|p| p.text());
        prakriyas.dedup_by_key(|p| p.text());
        // Allowed in pada sandhi, but noisy here.
        prakriyas.retain(|p| !p.text().contains("cS"));

        assert_padas(prakriyas, expected);
    }

    pub fn assert_has_samasas(&self, args: &SamasaArgs, expected: &[&str]) {
        let mut prakriyas = self.derive_samasas(&args);
        prakriyas.sort_by_key(|p| p.text());
        prakriyas.dedup_by_key(|p| p.text());
        assert_padas(prakriyas, expected);
    }

    /// A simpler interface to `assert_has_samasas` that accepts exactly two items.
    fn assert_samasa_of_type(
        &self,
        padas: &[SamasaPada],
        samasa_type: SamasaType,
        expected: &[&str],
    ) {
        let args = SamasaArgs::builder()
            .padas(Vec::from(padas))
            .samasa_type(samasa_type)
            .build()
            .unwrap();
        let t = Tester::default();
        t.assert_has_samasas(&args, expected);
    }

    fn assert_has_bahuvrihi(
        &self,
        a: impl IntoPratipadika,
        b: impl IntoPratipadika,
        expected: &[&str],
    ) {
        self.assert_samasa_of_type(
            &[
                SamasaPada::new(a.to_p(), Vibhakti::Prathama),
                SamasaPada::new(b.to_p(), Vibhakti::Prathama),
            ],
            Bahuvrihi,
            expected,
        );
    }

    fn assert_has_avyayibhava(
        &self,
        a: impl IntoPratipadika,
        b: impl IntoPratipadika,
        expected: &[&str],
    ) {
        self.assert_samasa_of_type(
            &[
                SamasaPada::avyaya(a.to_p()),
                SamasaPada::new(b.to_p(), Vibhakti::Prathama),
            ],
            Avyayibhava,
            expected,
        );
    }

    fn assert_has_karmadharaya(
        &self,
        a: impl IntoPratipadika,
        b: impl IntoPratipadika,
        expected: &[&str],
    ) {
        self.assert_samasa_of_type(
            &[
                SamasaPada::new(a.to_p(), Vibhakti::Prathama),
                SamasaPada::new(b.to_p(), Vibhakti::Prathama),
            ],
            Karmadharaya,
            expected,
        );
    }

    fn assert_has_dvitiya_tatpurusha(
        &self,
        a: impl IntoPratipadika,
        b: impl IntoPratipadika,
        expected: &[&str],
    ) {
        self.assert_samasa_of_type(
            &[
                SamasaPada::new(a.to_p(), Vibhakti::Dvitiya),
                SamasaPada::new(b.to_p(), Vibhakti::Prathama),
            ],
            Tatpurusha,
            expected,
        );
    }

    fn assert_has_trtiya_tatpurusha(
        &self,
        a: impl IntoPratipadika,
        b: impl IntoPratipadika,
        expected: &[&str],
    ) {
        self.assert_samasa_of_type(
            &[
                SamasaPada::new(a.to_p(), Vibhakti::Trtiya),
                SamasaPada::new(b.to_p(), Vibhakti::Prathama),
            ],
            Tatpurusha,
            expected,
        );
    }

    fn assert_has_caturthi_tatpurusha(
        &self,
        a: impl IntoPratipadika,
        b: impl IntoPratipadika,
        expected: &[&str],
    ) {
        self.assert_samasa_of_type(
            &[
                SamasaPada::new(a.to_p(), Vibhakti::Caturthi),
                SamasaPada::new(b.to_p(), Vibhakti::Prathama),
            ],
            Tatpurusha,
            expected,
        );
    }

    fn assert_has_panchami_tatpurusha(
        &self,
        a: impl IntoPratipadika,
        b: impl IntoPratipadika,
        expected: &[&str],
    ) {
        self.assert_samasa_of_type(
            &[
                SamasaPada::new(a.to_p(), Vibhakti::Panchami),
                SamasaPada::new(b.to_p(), Vibhakti::Prathama),
            ],
            Tatpurusha,
            expected,
        );
    }

    fn assert_has_sasthi_tatpurusha(
        &self,
        a: impl IntoPratipadika,
        b: impl IntoPratipadika,
        expected: &[&str],
    ) {
        self.assert_samasa_of_type(
            &[
                SamasaPada::new(a.to_p(), Vibhakti::Sasthi),
                SamasaPada::new(b.to_p(), Vibhakti::Prathama),
            ],
            Tatpurusha,
            expected,
        );
    }

    fn assert_has_saptami_tatpurusha(
        &self,
        a: impl IntoPratipadika,
        b: impl IntoPratipadika,
        expected: &[&str],
    ) {
        self.assert_samasa_of_type(
            &[
                SamasaPada::new(a.to_p(), Vibhakti::Saptami),
                SamasaPada::new(b.to_p(), Vibhakti::Prathama),
            ],
            Tatpurusha,
            expected,
        );
    }

    /// Derives taddhitantas from the given initial conditions.
    fn derive_artha_taddhitantas(
        &self,
        p: &Pratipadika,
        t: Taddhita,
        a: Option<TaddhitaArtha>,
    ) -> Vec<Prakriya> {
        let args = if let Some(a) = a {
            TaddhitantaArgs::builder()
                .taddhita(t)
                .artha(a)
                .build()
                .unwrap()
        } else {
            TaddhitantaArgs::builder().taddhita(t).build().unwrap()
        };
        let results = self.derive_taddhitantas(p, &args);
        sanitize_results(results)
    }

    pub fn assert_has_artha_taddhita(
        &self,
        prati: &str,
        requested_artha: TaddhitaArtha,
        t: Taddhita,
        expected: &[&str],
    ) {
        let pratipadika = Pratipadika::from(prati);
        let mut prakriyas = self.derive_artha_taddhitantas(&pratipadika, t, Some(requested_artha));
        prakriyas.retain(|p| {
            if let Some(Artha::Taddhita(prakriya_artha)) = p.artha() {
                requested_artha.is_type_of(prakriya_artha)
            } else {
                false
            }
        });
        assert_padas(prakriyas, expected);
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

pub fn create_pratipadika(text: &str, prakriyas: &[Prakriya]) -> Pratipadika {
    for p in prakriyas.iter() {
        if p.text() == text {
            let prati: Option<Pratipadika> = p.into();
            return prati.unwrap();
        }
    }
    print_all_prakriyas(&prakriyas);
    let alts: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    panic!("Could not create \"{}\". Alternatives: {alts:?}", text);
}

pub fn create_pada(text: &str, prakriyas: &[Prakriya]) -> Pada {
    for p in prakriyas.iter() {
        if p.text() == text {
            let pada: Option<Pada> = p.into();
            return pada.unwrap();
        }
    }
    print_all_prakriyas(&prakriyas);
    let alts: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    panic!("Could not create \"{}\". Alternatives: {alts:?}", text);
}

/// Creates a krdanta as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_krdanta(
    text: &str,
    prefixes: &[&str],
    d: &Dhatu,
    krt: impl Into<Krt>,
) -> Pratipadika {
    let args = KrdantaArgs::builder().krt(krt).build().unwrap();
    let d = d.clone().with_prefixes(prefixes);

    let tester = Tester::default();
    let prakriyas = tester.derive_krdantas(&d, &args);
    create_pratipadika(text, &prakriyas)
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
) -> Pratipadika {
    let args = KrdantaArgs::builder()
        .krt(krt)
        .upapada(Upapada::make_subanta(upapada))
        .build()
        .unwrap();
    let d = d.clone().with_prefixes(prefixes);

    let tester = Tester::default();
    let prakriyas = tester.derive_krdantas(&d, &args);
    create_pratipadika(text, &prakriyas)
}

/// Creates a krdanta as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_taddhitanta(
    text: &str,
    base: impl IntoPratipadika,
    taddhita: Taddhita,
) -> Pratipadika {
    let args = TaddhitantaArgs::builder()
        .taddhita(taddhita)
        .build()
        .unwrap();

    let tester = Tester::default();
    let prakriyas = tester.derive_taddhitantas(&base.to_p(), &args);
    create_pratipadika(text, &prakriyas)
}

/// Creates a krdanta as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_stryanta(text: &str, base: &str) -> Pratipadika {
    let tester = Tester::default();
    let prakriyas = tester.derive_stryantas(&Pratipadika::from(base));
    create_pratipadika(text, &prakriyas)
}

/// Creates a samasa as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_samasa(text: &str, items: &[&str], samasa_type: SamasaType) -> Pratipadika {
    let padas: Vec<_> = items
        .iter()
        .map(|s| {
            let prati = s.to_p();
            SamasaPada::new(prati, Vibhakti::Prathama)
        })
        .collect();
    let args = SamasaArgs::builder()
        .padas(padas)
        .samasa_type(samasa_type)
        .build()
        .unwrap();

    let tester = Tester::default();
    let prakriyas = tester.derive_samasas(&args);
    create_pratipadika(text, &prakriyas)
}

/// Creates a samasa as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_avyaya_tatpurusha(
    text: &str,
    first: impl IntoPratipadika,
    second: impl IntoPratipadika,
) -> Pratipadika {
    let padas = vec![
        SamasaPada::avyaya(first.to_p()),
        SamasaPada::new(second.to_p(), Vibhakti::Prathama),
    ];
    let args = SamasaArgs::builder()
        .padas(padas)
        .samasa_type(Tatpurusha)
        .build()
        .unwrap();

    let tester = Tester::default();
    let prakriyas = tester.derive_samasas(&args);
    create_pratipadika(text, &prakriyas)
}

/// Creates a samasa as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_tatpurusha(text: &str, items: &[&str], vibhakti: Vibhakti) -> Pratipadika {
    debug_assert!(items.len() == 2);

    let padas = vec![
        SamasaPada::new(Pratipadika::from(items[0]), vibhakti),
        SamasaPada::new(Pratipadika::from(items[1]), Vibhakti::Prathama),
    ];
    let args = SamasaArgs::builder()
        .padas(padas)
        .samasa_type(Tatpurusha)
        .build()
        .unwrap();

    let tester = Tester::default();
    let prakriyas = tester.derive_samasas(&args);
    create_pratipadika(text, &prakriyas)
}

/// Creates a samasa as a pratipadika.
///
/// This function is a shorthand that lets us test certain subanta forms more easily.
pub fn create_samasa_p(text: &str, pratis: &[Pratipadika], samasa_type: SamasaType) -> Pratipadika {
    let padas: Vec<_> = pratis
        .iter()
        .map(|s| {
            let prati = s.to_p();
            SamasaPada::new(prati, Vibhakti::Prathama)
        })
        .collect();
    let args = SamasaArgs::builder()
        .padas(padas)
        .samasa_type(samasa_type)
        .build()
        .unwrap();

    let tester = Tester::default();
    let prakriyas = tester.derive_samasas(&args);
    create_pratipadika(text, &prakriyas)
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
    let tester = Tester::default();
    let results = tester.derive_artha_taddhitantas(p, t, a);
    sanitize_results(results)
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

fn debug_text(rule: Rule) -> String {
    match rule {
        Rule::Ashtadhyayi(x) => x.to_string(),
        Rule::Dhatupatha(x) => format!("DA {x}"),
        Rule::Kashika(x) => format!("kA {x}"),
        Rule::Kaumudi(x) => format!("kO {x}"),
        Rule::Linganushasana(x) => format!("Li {x}"),
        Rule::Unadipatha(x) => format!("uRA {x}"),
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

fn assert_results(expected: &[String], actuals: &[String], prakriyas: Vec<Prakriya>) {
    let mut expected = Vec::from(expected);
    expected.sort();
    expected.dedup();

    let mut actuals = Vec::from(actuals);
    actuals.sort();
    actuals.dedup();

    if actuals.len() != expected.len() {
        print_all_prakriyas(&prakriyas);
    }

    // Before comparing, confirm lengths are the same.
    assert_eq!(
        actuals.len(),
        expected.len(),
        "expected: {expected:?}, actual: {actuals:?}"
    );

    for (i, p) in prakriyas.iter().enumerate() {
        let actual = p.text();

        if actual != expected[i] {
            print_all_prakriyas(&prakriyas);
        }

        assert_eq!(
            actual, expected[i],
            "expected: {expected:?}, actual: {actual:?}"
        );
    }
}

/// Asserts that the given `prakriyas` produce the `expected` results.
///
/// If there is any difference, this function will nicely print out each prakriya in `prakriyas`.
fn assert_padas(prakriyas: Vec<Prakriya>, expected: &[&str]) {
    let expected: Vec<_> = expected.iter().map(|t| t.to_string()).collect();
    let actuals: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    assert_results(&expected, &actuals, prakriyas);
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

pub fn assert_has_vakya(first: &Pada, second: &Pada, expected: &[&str]) {
    let t = Tester::default();
    t.assert_has_vakya(&vec![first.to_owned(), second.to_owned()], expected);
}

pub fn assert_has_sandhi(first: &str, second: &str, expected: &[&str]) {
    let prakriyas = derive_vakyas(&first, &second);

    let expected: Vec<_> = expected.iter().map(|text| text.replace(" ", "")).collect();
    let actuals: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
    assert_results(&expected, &actuals, prakriyas);
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
    Pratipadika::from(text)
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

pub fn assert_has_taddhitanta(prati: impl IntoPratipadika, t: Taddhita, expected: &[&str]) {
    assert_padas(derive_taddhitantas(&prati.to_p(), t, None), expected);
}

pub fn assert_has_artha_taddhita(
    prati: &str,
    requested_artha: TaddhitaArtha,
    taddhita: Taddhita,
    expected: &[&str],
) {
    let t = Tester::default();
    t.assert_has_artha_taddhita(prati, requested_artha, taddhita, expected);
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
    let pratipadika = Pratipadika::from(pratipadika_text);
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

pub trait IntoPratipadika {
    fn to_p(self) -> Pratipadika;
}

impl IntoPratipadika for &Pratipadika {
    fn to_p(self) -> Pratipadika {
        self.clone()
    }
}

impl IntoPratipadika for &str {
    fn to_p(self) -> Pratipadika {
        Pratipadika::from(self)
    }
}

macro_rules! assert_sup {
    ($fn_name:ident, $vibhakti:expr, $vacana:expr) => {
        pub fn $fn_name(prati: impl IntoPratipadika, linga: Linga, expected: &[&str]) {
            let t = Tester::default();
            t.assert_has_sup(&prati.to_p(), linga, $vibhakti, $vacana, &expected);
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
        pub fn $fn_name(expected: &str, prati: impl IntoPratipadika, linga: Linga) -> Pada {
            let t = Tester::default();
            t.create_sup(&expected, &prati.to_p(), linga, $vibhakti, $vacana)
        }
    };
}

create_sup!(create_sup_1s, Prathama, Eka);
create_sup!(create_sup_1d, Prathama, Dvi);
create_sup!(create_sup_1p, Prathama, Bahu);
create_sup!(create_sup_2s, Dvitiya, Eka);
create_sup!(create_sup_2d, Dvitiya, Dvi);
create_sup!(create_sup_2p, Dvitiya, Bahu);
create_sup!(create_sup_3s, Trtiya, Eka);
create_sup!(create_sup_3d, Trtiya, Dvi);
create_sup!(create_sup_3p, Trtiya, Bahu);
create_sup!(create_sup_4s, Caturthi, Eka);
create_sup!(create_sup_4d, Caturthi, Dvi);
create_sup!(create_sup_4p, Caturthi, Bahu);
create_sup!(create_sup_5s, Panchami, Eka);
create_sup!(create_sup_5d, Panchami, Dvi);
create_sup!(create_sup_5p, Panchami, Bahu);
create_sup!(create_sup_6s, Sasthi, Eka);
create_sup!(create_sup_6d, Sasthi, Dvi);
create_sup!(create_sup_6p, Sasthi, Bahu);
create_sup!(create_sup_7s, Saptami, Eka);
create_sup!(create_sup_7d, Saptami, Dvi);
create_sup!(create_sup_7p, Saptami, Bahu);
create_sup!(create_sup_ss, Sambodhana, Eka);
create_sup!(create_sup_sd, Sambodhana, Dvi);
create_sup!(create_sup_sp, Sambodhana, Bahu);

macro_rules! assert_samasa {
    ($fn_name:ident) => {
        pub fn $fn_name(
            purva: impl IntoPratipadika,
            uttara: impl IntoPratipadika,
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
    first: impl IntoPratipadika,
    second: impl IntoPratipadika,
    expected: &[&str],
) {
    let t = Tester::default();
    t.assert_samasa_of_type(
        &[
            SamasaPada::avyaya(first.to_p()),
            SamasaPada::new(second.to_p(), Vibhakti::Prathama),
        ],
        Tatpurusha,
        expected,
    );
}

pub fn assert_has_misc_tatpurusha(
    first: impl IntoPratipadika,
    second: impl IntoPratipadika,
    expected: &[&str],
) {
    let t = Tester::default();
    t.assert_samasa_of_type(
        &[
            SamasaPada::new(first.to_p(), Vibhakti::Sasthi),
            SamasaPada::new(second.to_p(), Vibhakti::Prathama),
        ],
        Tatpurusha,
        expected,
    );
}

pub fn assert_has_dvandva(items: &[&str], expected: &[&str]) {
    let args = SamasaArgs::builder()
        .padas(
            items
                .iter()
                .map(|s| SamasaPada::new(Pratipadika::from(s), Vibhakti::Prathama))
                .collect(),
        )
        .samasa_type(Dvandva)
        .build()
        .unwrap();
    let t = Tester::default();
    t.assert_has_samasas(&args, expected);
}

pub fn assert_has_samahara_dvandva(items: &[&str], expected: &[&str]) {
    let args = SamasaArgs::builder()
        .padas(
            items
                .iter()
                .map(|s| SamasaPada::new(Pratipadika::from(s), Vibhakti::Prathama))
                .collect(),
        )
        .samasa_type(SamaharaDvandva)
        .build()
        .unwrap();
    let t = Tester::default();
    t.assert_has_samasas(&args, expected);
}
