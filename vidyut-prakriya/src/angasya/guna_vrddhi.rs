use crate::args::Agama as A;
use crate::args::Aupadeshika as Au;
use crate::args::Gana;
use crate::args::Lakara::*;
use crate::args::Sanadi as S;
use crate::args::Unadi;
use crate::args::Upasarga as U;
use crate::args::Vikarana as V;
use crate::core::operators as op;
use crate::core::Rule::Varttika;
use crate::core::{Code, Prakriya, Rule, Tag as T, Term, TermView};
use crate::sounds as al;
use crate::sounds::{Set, AC, HAL, IK};

const HRASVA: Set = Set::from("aiufx");

impl Term {
    /// Tries guna of the term's penultimate sound.
    fn try_upadha_guna(&mut self) {
        if let Some(a) = self.upadha() {
            if let Some(sub) = al::to_guna(a) {
                self.set_upadha(sub);
                self.add_tag(T::FlagGuna);
            }
        }
    }

    /// Tries guna of the term's last sound.
    fn try_antya_guna(&mut self) {
        if let Some(a) = self.antya() {
            if let Some(sub) = al::to_guna(a) {
                self.set_antya(sub);
                self.add_tag(T::FlagGuna);
            }
        }
    }

    /// Tries vrddhi of the term's penultimate sound.
    fn try_upadha_vrddhi(&mut self) {
        if let Some(a) = self.upadha() {
            if let Some(sub) = al::to_vrddhi(a) {
                self.set_upadha(sub);
                self.add_tag(T::FlagVrddhi);
            }
        }
    }

    /// Tries vrddhi of the term's last sound.
    fn try_antya_vrddhi(&mut self) {
        if let Some(a) = self.antya() {
            if let Some(sub) = al::to_vrddhi(a) {
                self.set_antya(sub);
                self.add_tag(T::FlagVrddhi);
            }
        }
    }
}

struct GunaVrddhiPrakriya<'a> {
    p: &'a mut Prakriya,
    /// The index of the term that potentially receives guna.
    i_anga: usize,
    /// The index of the start of the view that potentially conditions guna.
    i_next: usize,
    /// The index of the end of the view that potentially conditions guna.
    i_p: usize,
    /// Whether guna/vrddhi has been decided for this anga.
    locked: bool,
}

impl<'a> GunaVrddhiPrakriya<'a> {
    /// Creates a new prakriya wrapper that specializes in guna/vrddhi changes.
    fn new(p: &'a mut Prakriya, i_anga: usize, i_next: usize, i_p: usize) -> Self {
        debug_assert!(i_next <= i_p);
        Self {
            p,
            i_anga,
            i_next,
            i_p,
            locked: false,
        }
    }

    /// Returns the anga that might receive guna/vrddhi.
    fn anga(&self) -> &Term {
        self.p.get(self.i_anga).expect("ok")
    }

    /// Returns the term that might condition guna/vrddhi.
    fn next_view(&self) -> TermView {
        self.p.view(self.i_next, self.i_p).expect("ok")
    }

    /// Checks a standard list of rules that block guna/vrddhi
    fn check_blocking_rules(&mut self) {
        if self.locked {
            return;
        }

        let anga = self.anga();
        let n = self.next_view();

        if anga.has_tag_in(&[T::FlagAtLopa, T::FlagGunaApavada]) {
            self.locked = true;
        } else if self.p.has(self.i_anga + 1, |t| {
            t.is_dhatu() && t.is_empty() && t.has_tag(T::FlagAtLopa)
        }) && n.has_tag(T::Ardhadhatuka)
        {
            self.try_block("1.1.4");
        } else if n.is_knit() {
            self.try_block("1.1.5");
        } else if anga.is_any_u(&[Au::dIDIN, Au::vevIN]) || anga.is_it_agama() {
            self.try_block("1.1.6");
        } else if !n.has_tag(T::Pratyaya) {
            self.locked = true;
        }
    }

    /// Blocks guna/vrddhi with the given `rule`.
    fn try_block(&mut self, rule: Code) {
        self.try_run(rule, |_| {});
    }

    /// Tries `func` and blocks further guna/vrddhi changes if it succeeds.
    fn try_run(&mut self, rule: Code, func: impl Fn(&mut Term)) {
        if !self.locked {
            self.p.run_at(rule, self.i_anga, func);
            self.locked = true;
        }
    }

    /// Tries `func` optionally and blocks further guna/vrddhi changes if it succeeds.
    fn run_optional(&mut self, rule: Code, func: impl Fn(&mut Term)) {
        if !self.locked {
            let ran = self.p.optional_run_at(rule, self.i_anga, func);
            self.locked = ran;
        }
    }
}

/// Tries rules that cause vrddhi when a taddhita-pratyaya follows.
fn try_taddhita_vrddhi(p: &mut Prakriya, i_anga: usize, i_n: usize) -> Option<()> {
    const DVARA_ADI: &[&str] = &[
        "dvAra", "svara", "svADyAya", "vyalkaSa", "svasti", "svar", "sPyakfta", "Svas", "Svan",
        "sva",
    ];

    let anga = p.get(i_anga)?;
    let n = p.get_if(i_n, |t| t.is_taddhita())?;

    let rule = if n.has_tag_in(&[T::Yit, T::Rit]) {
        "7.2.117"
    } else if n.has_tag(T::kit) {
        "7.2.118"
    } else {
        return None;
    };

    if anga.is_any_phit(&["kekaya", "mitrayu", "pralaya"]) {
        p.run_at("7.3.2", i_anga, |t| t.find_and_replace_text("y", "iy"));
    }

    let anga = p.get(i_anga)?;
    if anga.is_any_phit(&["devikA", "SiMSapA", "dityavAh", "dIrGasatra", "Sreyas"]) {
        // dAvikA, ...
        let adi_ac = anga.text.find(al::is_ac)?;
        p.run_at("7.3.1", i_anga, |t| t.set_at(adi_ac, "A"));
    } else if anga.starts_with("vy") {
        // HACK: should properly be only with vi-upasarga.
        // TODO: also apply for sv-, .etc.
        p.run_at("7.3.3", i_anga, |t| t.text.replace_range(..2, "vEy"));
    } else if anga.is_any_phit(DVARA_ADI) {
        // dvAra -> dOvArika, ...
        p.run_at("7.3.4", i_anga, |t| {
            let i_yan = t.text.rfind(|c| c == 'y' || c == 'v').expect("ok");
            if t.get(i_yan) == Some('y') {
                t.text.insert(i_yan, 'E');
            } else {
                t.text.insert(i_yan, 'O');
            }
        });
    } else if anga.has_text("nyagroDa") {
        p.run_at("7.3.5", i_anga, |t| t.text.replace_range(..2, "nEy"));
    } else {
        let i_ac = p.find_first_where(|t| t.has_ac())?;
        let adi = p.get(i_ac)?;
        let adi_ac = adi.text.find(al::is_ac)?;
        let ac = adi.get(adi_ac)?;
        let vrddhi = al::to_vrddhi(ac)?;
        p.run_at(rule, i_ac, |t| {
            t.set_at(adi_ac, vrddhi);
        });
    }

    Some(())
}

/// Runs rules for vrddhi conditioned on following Nit-Yit.
///
/// (7.2.115 - 7.3.35)
/// Taddhita rules: 7.2.117 - 7.3.31
fn try_nnit_vrddhi(p: &mut Prakriya, i_anga: usize, i_n: usize) -> Option<()> {
    let n = p.get(i_n)?;
    if !n.has_tag_in(&[T::Yit, T::Rit]) {
        return None;
    }

    let anga_has_kit_agama = p.has(i_anga + 1, |t| {
        t.is_agama() && t.is_knit() && !t.is_it_agama()
    });
    if anga_has_kit_agama {
        return None;
    }

    // Allow RiN even though it is Nit. Without this check, RiN will be excluded by
    // `can_use_guna_or_vrddhi`.
    if !n.has_u("RiN") {
        let mut gp = GunaVrddhiPrakriya::new(p, i_anga, i_n, i_n);
        gp.check_blocking_rules();
        if gp.locked {
            p.debug("locked, returning");
            return None;
        }
    }

    let anga = p.get(i_anga)?;
    let n = p.get(i_n)?;
    let is_cin = n.is(V::ciR) || n.has_tag(T::Cinvat);
    let is_cin_krt = is_cin || n.has_tag(T::Krt);
    let has_udatta = !anga.has_tag(T::Anudatta);

    let is_aacam_adi = {
        let is_aacam = anga.has_u("camu~")
            && anga.has_gana(Gana::Bhvadi)
            && p.find_prev_where(i_anga, |t| t.is(U::AN)).is_some();
        is_aacam || anga.has_u_in(&["kamu~\\", "wuvama~"])
    };

    if is_cin_krt && has_udatta && anga.has_antya('m') && !is_aacam_adi {
        p.step("7.3.34");
    } else if is_cin_krt && anga.has_text_in(&["jan", "vaD"]) && !n.is(Unadi::YuR) {
        // ajani, avaDi, ...
        p.step("7.3.35");
    } else if is_cin_krt && anga.has_antya('A') {
        op::insert_after("7.3.33", p, i_anga, A::yuk);
    } else if anga.is_u(Au::hana) && !is_cin && !n.has_u("Ral") {
        p.run("7.3.32", |p| {
            p.set(i_anga, op::upadha("A"));
            p.set(i_anga, op::antya("t"));
        });
    } else if anga.has_antya(AC) {
        // The use of "acaH" in 7.2.115 indicates that we should ignore "iko guNavRddhI" which
        // ordinarily restricts vrddhi to ik vowels only. By ignoring this restriction, we can
        // correctly generate `vye -> vivyAya` etc.
        let antya = anga.antya()?;
        if !al::is_vrddhi(antya) {
            p.run_at("7.2.115", i_anga, |t| t.try_antya_vrddhi());
        }
    } else if anga.has_upadha('a') {
        if anga.has_u_in(&["kamu~\\", "wuvama~"]) {
            // akAmi, avAmi
            p.step(Varttika("7.3.34.1"))
        }

        // pAcayati
        p.run_at("7.2.116", i_anga, op::upadha("A"));
    }

    Some(())
}

/// Tries rules that replace an anga's vowel with a vrddhi substitute.
///
/// Example: kf + i + ta -> kArita
fn try_vrddhi_adesha(p: &mut Prakriya, i_anga: usize, i_n: usize) -> Option<()> {
    let anga = p.get_if(i_anga, |t| !t.has_tag(T::FlagGunaApavada))?;
    let n = p.view(i_anga + 1, i_n)?;
    let i_p = n.end();

    if anga.has_text("mfj") && !n.last().is_knit() {
        let mut gp = GunaVrddhiPrakriya::new(p, i_anga, i_n, i_p);
        gp.check_blocking_rules();
        gp.try_run("7.2.114", |t| t.try_upadha_vrddhi());
    } else if anga.has_text("mfj") && n.last().is_knit() && n.has_adi(AC) && !n.last().is_krt() {
        // mfjanti, mArjanti, ...
        p.optional_run_at(Rule::Kaumudi("2473"), i_anga, |t| t.try_upadha_vrddhi());
    } else if n.first().is_taddhita() {
        try_taddhita_vrddhi(p, i_anga, i_n);
    } else {
        try_nnit_vrddhi(p, i_anga, i_n);
    }

    Some(())
}

/// Runs rules that replace an anga's vowel with its corresponding guna.
/// Example: buD + a + ti -> boDati
fn try_guna_adesha(p: &mut Prakriya, i_anga: usize, i_n: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    if anga.has_tag(T::FlagGunaApavada)
        || !anga.has_tag_in(&[T::Dhatu, T::Sarvadhatuka, T::Ardhadhatuka])
    {
        return None;
    }

    let n = p.get_if(i_n, |t| !t.is_empty())?;
    let is_sarva_ardha = n.has_tag_in(&[T::Sarvadhatuka, T::Ardhadhatuka]);
    if !is_sarva_ardha {
        return None;
    }

    let has_only_wit_agamas = p.terms()[i_anga + 1..i_n]
        .iter()
        .all(|t| t.is_empty() || t.has_tag(T::wit));
    let is_puganta = p.has(i_anga + 1, |t| t.is(A::puk));
    if !has_only_wit_agamas && !is_puganta {
        return None;
    }

    let i_n_start = p.find_next_where(i_anga, |t| !t.is_empty())?;
    let mut gp = GunaVrddhiPrakriya::new(p, i_anga, i_n_start, i_n);

    let anga = gp.anga();
    let n_view = gp.next_view();
    let n_adi = n_view.first().adi()?;
    let n = gp.p.get(i_n)?;
    let piti_sarvadhatuke = n.has_all_tags(&[T::pit, T::Sarvadhatuka]);

    if anga.is_any_u(&[Au::Divi, Au::kfvi]) {
        // Per commentary on 3.1.81, these roots don't take guna.
    } else if anga.has_text("mid") && n.has_tag(T::Sit) {
        // medyati
        gp.try_run("7.3.82", |t| t.try_upadha_guna());
    } else if anga.has_antya(IK) && n_view.first().has_u("jus") {
        // ajuhavuH
        gp.try_run("7.3.83", |t| t.try_antya_guna());
    } else if anga.has_text("tfnah")
        && HAL.contains(n_adi)
        && piti_sarvadhatuke
        && !n.has_tag(T::Nit)
    {
        // tfneQi; otherwise, tfRahAni, tfRQaH.
        // HACK: check for absence of `Nit` on first term to prevent tfnhyAt -> tfRihyAt
        p.run_at("7.3.92", i_anga, op::mit("i"));
    } else {
        let is_yan_luk = || gp.p.has(i_anga + 1, |t| t.is(S::yaN) && t.is_lupta());
        // Exceptions
        if anga.has_text_in(&["BU", "sU"])
            && n.is_tin()
            && piti_sarvadhatuke
            // See KV on 7.3.88 for why we exclude yaN-luk forms (boBoti, etc.)
            && !is_yan_luk()
        {
            // aBUt, ...
            gp.try_block("7.3.88");
        } else if anga.has_antya('u') && HAL.contains(n_adi) && piti_sarvadhatuke {
            gp.check_blocking_rules();
            let n = gp.p.get(i_n)?;

            let anga = gp.anga();
            if anga.has_u("UrRuY") {
                if n.is_aprkta() {
                    // prOrRot
                    gp.try_run("7.3.91", |t| t.try_antya_guna());
                } else {
                    // UrROti, UrRoti
                    // If vrddhi is declined, UrRu will take guna by 7.3.84 below.
                    let sub = al::to_vrddhi(anga.antya()?)?;
                    gp.run_optional("7.3.90", op::antya(sub));
                }
            } else if gp.p.get(i_anga + 1)?.has_tag(T::Luk) && !anga.is_abhyasta() {
                // Why check for abhyasta?
                //
                // > na abhyastasya ityetadiha anuvartate, yoyoti, roroti ityevamādyartham.
                //
                // -- KV on 7.3.89.
                gp.try_run("7.3.89", |t| t.try_antya_vrddhi());
            };
        }

        // Main guna rules.
        let anga = gp.anga();
        let is_laghu_upadha = anga.has_upadha(HRASVA);

        // HACK to ignore antya A and avoid applying guna to it.
        if is_puganta || is_laghu_upadha {
            if anga.is_abhyasta() && piti_sarvadhatuke && AC.contains(n_adi) {
                // e.g. nenijAma
                gp.try_block("7.3.87");
            } else {
                let code = "7.3.86";
                if is_puganta {
                    let sub = al::to_guna(anga.antya()?)?;
                    // Ignore 'a/A' by "iko gunavRddhI"
                    if !(sub == "a" || sub == "A") {
                        gp.check_blocking_rules();
                        gp.try_run(code, |t| t.try_antya_guna());
                    }
                } else {
                    let sub = al::to_guna(anga.upadha()?)?;
                    if !(sub == "a" || sub == "A") {
                        gp.check_blocking_rules();
                        gp.try_run(code, |t| t.try_upadha_guna());
                    }
                }
            }
        } else if anga.has_antya(IK) {
            gp.check_blocking_rules();
            gp.try_run("7.3.84", |t| t.try_antya_guna());
        }
    }

    Some(())
}

/// Runs rules that condition on a following liT-pratyaya.
///
/// Per commentaries, these rules apply for cases where guna does not otherwise obtain.
///
/// Constraints:
/// - must run after guna/vrddhi have been tried.
///
/// (7.4.10 - 7.4.12)
fn try_r_guna_before_lit(p: &mut Prakriya, i: usize) -> Option<()> {
    if !p.has(i, |t| t.is_dhatu()) {
        return None;
    }

    if !p.terms().last()?.has_lakara(Lit) {
        return None;
    }

    if p.get(i + 1)?.has_tag(T::Krt) {
        // Skip if this is kvasu~ or kAnac. Since these are the only krt-pratyayas that replace
        // li~w, just check if the pratyaya is `Krt`.
        //
        // > ṛkārāntānāṃ guṇapratiṣedhārthaṃ tarhi kittvaṃ vaktavyam। ayaṃ hi liṭi ṛkārāntānāṃ
        // > pratiṣedhaviṣaye guṇa ārabhyate। sa yathaiveha pratiṣedhaṃ bādhitvā guṇo bhavati -
        // > teratuḥ teruriti। evamihāpi syāt - titīrvān, titirāṇa iti। punaḥ kitkaraṇāt
        // > pratiṣidhyate। tasmātkittvaṃ kartavyam।
        //
        // -- Mahabhashya on 3.2.107.
        return None;
    }

    let do_ar_guna = |t: &mut Term| {
        t.add_tag(T::FlagGuna);
        t.set_antya("ar");
    };

    let anga = p.get(i)?;
    let is_skr = || anga.has_u("qukf\\Y") && i > 0 && p.has(i - 1, |t| t.is(A::suw));
    if anga.has_antya('f') && (anga.is_samyogadi() || is_skr()) {
        p.run_at("7.4.10", i, do_ar_guna);
    } else if anga.has_antya('F') || (anga.has_u_in(&["fCa~", "f\\"]) && anga.has_adi('f')) {
        if anga.has_u("fCa~") {
            p.run_at("7.4.11", i, op::adi("ar"));
        } else {
            let mut skipped = false;
            if anga.has_text_in(&["SF", "dF", "pF"]) && !anga.has_gana(Gana::Curadi) {
                skipped = p.optional_run_at("7.4.12", i, |t| {
                    t.set_antya("f");
                    t.add_tag(T::FlagGunaApavada);
                });
            }

            if !skipped {
                p.run_at("7.4.11", i, do_ar_guna);
            }
        }
    }

    Some(())
}

fn run_for_index(p: &mut Prakriya, i_anga: usize, i_n: usize) -> Option<()> {
    let anga = p.get(i_anga)?;
    let n = p.get(i_n)?;

    if anga.is_dhatu()
        && anga.has_gana(Gana::Tanadi)
        && anga.has_u_in(&["kziRu~^", "fRu~^", "tfRu~^", "GfRu~^"])
        && n.is(V::u)
    {
        // kziRoti, kzeRoti, ...
        p.optional_add_tag_at(Rule::Kaumudi("2547"), i_anga, T::FlagGunaApavada);
    }

    let anga = p.get(i_anga)?;
    let n = p.get(i_n)?;
    if anga.is_u(Au::jAgf)
        && !(n.is(V::ciR) || n.is(Unadi::kvin) || n.has_u("Ral"))
        && !p.pratyaya(i_n)?.has_tag(T::Nit)
    {
        // jAgf-guna takes priority over vrddhi. Skip if already applied (e.g. for jAgf + Ric).
        if anga.has_antya('f') {
            p.run_at("7.3.85", i_anga, |t| {
                t.set_antya("ar");
                t.add_tag(T::FlagGuna);
            });
        }
    } else {
        // Vrddhi takes priority over guna. For example, Ric is Ardhadhatuka (guna)
        // and Rit (vrddhi), but it will cause vrddhi if possible.
        try_vrddhi_adesha(p, i_anga, i_n);
        try_guna_adesha(p, i_anga, i_n);
        // TODO: 7.4.23-4
    }

    try_r_guna_before_lit(p, i_anga);

    Some(())
}

pub fn run(p: &mut Prakriya) -> Option<()> {
    let mut i = p.find_first_where(|t| t.is_anga())?;

    loop {
        let j = p.find_next_where(i, |t| !t.is_agama() && (!t.is_empty() || !t.is_lupta()))?;
        let t = p.get(i)?;

        if t.is_anga() && !t.has_tag_in(&[T::FlagAntyaAcSandhi, T::FlagPratipadikaTiLopa]) {
            run_for_index(p, i, j);
        }

        i = j;
    }
}
