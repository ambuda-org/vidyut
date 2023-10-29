use crate::args::Gana;
use crate::operators as op;
use crate::prakriya::{Code, Prakriya};
use crate::sounds as al;
use crate::sounds::{s, Set};
use crate::tag::Tag as T;
use crate::term::{Term, TermView};
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
    static ref HRASVA: Set = Set::from("aiufx");
    static ref IK: Set = s("ik");
    static ref HAL: Set = s("hal");
}

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
            }
        }
    }

    /// Tries vrddhi of the term's last sound.
    fn try_antya_vrddhi(&mut self) {
        if let Some(a) = self.antya() {
            if let Some(sub) = al::to_vrddhi(a) {
                self.set_antya(sub);
            }
        }
    }
}

struct GunaVrddhiPrakriya<'a> {
    p: &'a mut Prakriya,
    /// The index of the term that potentially receives guna.
    i_anga: usize,
    /// The index of the term that potentially conditions guna.
    i_next: usize,
    /// Whether guna/vrddhi has been decided for this anga.
    done: bool,
}

impl<'a> GunaVrddhiPrakriya<'a> {
    /// Creates a new prakriya wrapper that specializes in guna/vrddhi changes.
    fn new(p: &'a mut Prakriya, i_anga: usize, i_next: usize) -> Self {
        Self {
            p,
            i_anga,
            i_next,
            done: false,
        }
    }

    /// Returns the anga that might receive guna/vrddhi.
    fn anga(&self) -> &Term {
        self.p.get(self.i_anga).expect("ok")
    }

    /// Returns the term that might condition guna/vrddhi.
    fn next(&self) -> TermView {
        self.p.view(self.i_next).expect("ok")
    }

    /// Checks a standard list of rules that block guna/vrddhi
    fn check_guna_vrddhi_blocks(&mut self) {
        if self.done {
            return;
        }

        let anga = self.anga();
        let n = self.next();

        if anga.has_tag_in(&[T::FlagAtLopa, T::FlagGunaApavada]) {
            self.done = true;
        } else if self.p.has(self.i_anga + 1, |t| {
            t.is_dhatu() && t.is_empty() && t.has_tag(T::FlagAtLopa)
        }) && n.has_tag(T::Ardhadhatuka)
        {
            self.block("1.1.4");
        } else if n.is_knit() {
            self.block("1.1.5");
        } else if anga.has_u_in(&["dIDIN", "vevIN"]) || anga.is_it_agama() {
            self.block("1.1.6");
        } else {
            if !n.has_tag(T::Pratyaya) {
                self.done = true;
            }
        }
    }

    /// Blocks guna/vrddhi with the given `rule`.
    fn block(&mut self, rule: Code) {
        self.try_run(rule, |_| {});
    }

    /// Tries `func` and blocks further guna/vrddhi changes if it succeeds.
    fn try_run(&mut self, rule: Code, func: impl Fn(&mut Term)) {
        if !self.done {
            self.p.run_at(rule, self.i_anga, func);
            self.done = true;
        }
    }

    /// Tries `func` optionally and blocks further guna/vrddhi changes if it succeeds.
    fn run_optional(&mut self, rule: Code, func: impl Fn(&mut Term)) {
        if !self.done {
            let ran = self.p.run_optional_at(rule, self.i_anga, func);
            self.done = ran;
        }
    }
}

/// Tries rules that cause vrddhi when a taddhita-pratyaya follows.
fn try_taddhita_vrddhi(p: &mut Prakriya, i: usize) -> Option<()> {
    const DVARA_ADI: &[&str] = &[
        "dvAra", "svara", "svADyAya", "vyalkaSa", "svasti", "svar", "sPyakfta", "Svas", "Svan",
        "sva",
    ];

    let anga = p.get(i)?;
    let n = p.get_if(i + 1, |t| t.is_taddhita())?;

    let rule = if n.has_tag_in(&[T::Yit, T::Rit]) {
        "7.2.117"
    } else if n.has_tag(T::kit) {
        "7.2.118"
    } else {
        return None;
    };

    if anga.has_text_in(&["devikA", "SiMSapA", "dityavAh", "dIrGasatra", "Sreyas"]) {
        // dAvikA, ...
        let adi_ac = anga.text.find(al::is_ac)?;
        p.run_at("7.3.1", i, |t| t.set_at(adi_ac, "A"));
    } else if anga.has_text_in(&["kekaya", "mitrayu", "pralaya"]) {
        p.run_at("7.3.2", i, |t| t.find_and_replace_text("y", "iy"));
    } else if anga.text.starts_with("vy") {
        // HACK: should properly be only with vi-upasarga.
        // TODO: also apply for sv-, .etc.
        p.run_at("7.3.3", i, |t| t.text.replace_range(..2, "vEy"));
    } else if anga.has_u_in(DVARA_ADI) {
        // dvAra -> dOvArika, ...
        p.run_at("7.3.4", i, |t| {
            let i_yan = t.text.rfind(|c| c == 'y' || c == 'v').expect("ok");
            if t.get_at(i_yan) == Some('y') {
                t.text.insert(i_yan, 'E');
            } else {
                t.text.insert(i_yan, 'O');
            }
        });
    } else if anga.has_text("nyagroDa") {
        p.run_at("7.3.5", i, |t| t.text.replace_range(..2, "nEy"));
    } else {
        let adi_ac = anga.text.find(al::is_ac)?;
        let ac = anga.get_at(adi_ac)?;
        let vrddhi = al::to_vrddhi(ac)?;
        p.run_at(rule, i, |t| {
            t.set_at(adi_ac, vrddhi);
        });
    }

    Some(())
}

/// Runs rules for vrddhi conditioned on following Nit-Yit.
///
/// (7.2.115 - 7.3.35)
/// Taddhita rules: 7.2.117 - 7.3.31
fn try_nnit_vrddhi(p: &mut Prakriya, i: usize) -> Option<()> {
    let i_next = p.find_next_where(i, |t| !t.is_empty())?;

    let mut gp = GunaVrddhiPrakriya::new(p, i, i_next);
    let n = gp.next();
    let is_nnit = n.has_tag_in(&[T::Yit, T::Rit]);
    // Allow RiN even though it is Nit. Without this check, RiN will be excluded by
    // `can_use_guna_or_vrddhi`.
    if !n.has_u("RiN") {
        gp.check_guna_vrddhi_blocks();
    }

    if !is_nnit || gp.done {
        return None;
    }

    let anga = p.get(i)?;
    let n = p.view(i_next)?;
    let is_cin = n.has_u("ciR") || n.has_tag(T::Cinvat);
    let is_cin_krt = is_cin || n.has_tag(T::Krt);
    let has_udatta = !anga.has_tag(T::Anudatta);

    let is_aacam_adi = {
        let is_aacam = anga.has_u("camu~")
            && anga.has_gana(Gana::Bhvadi)
            && p.find_prev_where(i, |t| t.is_upasarga() && t.has_u("AN"))
                .is_some();
        is_aacam || anga.has_u_in(&["kamu~\\", "wuvama~"])
    };

    if is_cin_krt && has_udatta && anga.has_antya('m') && !is_aacam_adi {
        p.step("7.3.34");
    } else if is_cin_krt && anga.has_text_in(&["jan", "vaD"]) {
        // ajani, avaDi, ...
        p.step("7.3.35");
    } else if is_cin_krt && anga.has_antya('A') {
        op::append_agama("7.3.33", p, i, "yu~k");
    } else if anga.has_u("ha\\na~") && !is_cin && !n.has_u("Ral") {
        p.run("7.3.32", |p| {
            p.set(i, op::upadha("A"));
            p.set(i, op::antya("t"));
        });
    } else if anga.has_antya(&*AC) {
        // The use of "acaH" in 7.2.115 indicates that we should ignore "iko guNavRddhI" which
        // ordinarily restricts vrddhi to ik vowels only. By ignoring this restriction, we can
        // correctly generate `vye -> vivyAya` etc.
        let antya = anga.antya()?;
        if !al::is_vrddhi(antya) {
            let sub = al::to_vrddhi(antya)?;
            p.run_at("7.2.115", i, op::antya(sub));
        }
    } else if anga.has_upadha('a') {
        if anga.has_u_in(&["kamu~\\", "wuvama~"]) {
            // akAmi, avAmi
            p.step("7.3.34.v1")
        }

        // pAcayati
        p.run_at("7.2.116", i, op::upadha("A"));
    }

    Some(())
}

/// Tries rules that replace an anga's vowel with a vrddhi substitute.
///
/// Example: kf + i + ta -> kArita
fn try_vrddhi_adesha(p: &mut Prakriya, i: usize) -> Option<()> {
    let dhatu = p.get_if(i, |t| !t.has_tag(T::FlagGunaApavada))?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;
    let n = p.view(i_n)?;

    if dhatu.has_text("mfj") && !n.is_knit() {
        let mut gp = GunaVrddhiPrakriya::new(p, i, i_n);
        gp.check_guna_vrddhi_blocks();
        gp.try_run("7.2.114", |t| t.try_upadha_vrddhi());
    } else if n.first()?.is_taddhita() {
        try_taddhita_vrddhi(p, i);
    } else {
        try_nnit_vrddhi(p, i);
    }

    Some(())
}

/// Runs rules that replace an anga's vowel with its corresponding guna.
/// Example: buD + a + ti -> boDati
fn try_guna_adesha(p: &mut Prakriya, i: usize) -> Option<()> {
    let j = p.find_next_where(i, |t| !t.is_empty() && !t.has_u("pu~k"))?;

    let anga = p.get_if(i, |t| !t.is_agama() && !t.has_tag(T::FlagGunaApavada))?;
    let n = p.view(j)?;

    let is_sarva_ardha = n.has_tag_in(&[T::Sarvadhatuka, T::Ardhadhatuka]);
    let piti_sarvadhatuke = n.all(&[T::pit, T::Sarvadhatuka]);
    let is_ik = anga.has_antya(&*IK);

    let mut gp = GunaVrddhiPrakriya::new(p, i, j);
    let anga = gp.anga();
    let n = gp.next();

    if anga.has_u_in(&["Divi~", "kfvi~"]) {
        // Per commentary on 3.1.81, these roots don't take guna.
    } else if anga.has_text("mid") && n.has_tag(T::Sit) {
        // medyati
        gp.try_run("7.3.82", |t| t.try_upadha_guna());
    } else if is_ik && n.has_u("jus") {
        gp.try_run("7.3.83", |t| t.try_antya_guna());
    } else if anga.has_text("tfnah") && n.has_adi(&*HAL) && piti_sarvadhatuke && !n.has_tag(T::Nit)
    {
        // tfneQi; otherwise, tfRahAni, tfRQaH.
        // HACK: check for absence of `Nit` on first term to prevent tfnhyAt -> tfRihyAt
        p.run_at("7.3.92", i, op::mit("i"));
    } else if is_sarva_ardha {
        // Exceptions
        if anga.has_text_in(&["BU", "sU"]) && n.has_tag(T::Tin) && piti_sarvadhatuke {
            // aBUt, ...
            gp.block("7.3.88");
        } else if anga.has_antya('u') && n.has_adi(&*HAL) && piti_sarvadhatuke {
            gp.check_guna_vrddhi_blocks();

            let anga = gp.anga();
            let n = gp.next();
            let sub = al::to_vrddhi(anga.antya()?)?;
            if anga.has_u("UrRuY") {
                if n.last()?.is_aprkta() {
                    // prOrRot
                    gp.try_run("7.3.91", |t| t.try_antya_guna());
                } else {
                    // UrROti, UrRoti
                    // If vrddhi is declined, UrRu will take guna by 7.3.84 below.
                    gp.run_optional("7.3.90", op::antya(sub));
                }
            } else if gp.p.get(i + 1)?.has_tag(T::Luk) && !anga.has_tag(T::Abhyasta) {
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
        let n = gp.next();
        let is_laghu_upadha = anga.has_upadha(&*HRASVA);
        let is_puganta = gp.p.has(i + 1, |t| t.has_u("pu~k"));

        // HACK to ignore antya A and avoid applying guna to it.
        if is_puganta || is_laghu_upadha {
            if anga.is_abhyasta() && piti_sarvadhatuke && n.has_adi(&*AC) {
                // e.g. nenijAma
                gp.block("7.3.87");
            } else {
                let code = "7.3.86";
                if is_puganta {
                    let sub = al::to_guna(anga.antya()?)?;
                    // Ignore 'a/A' by "iko gunavRddhI"
                    if !(sub == "a" || sub == "A") {
                        gp.check_guna_vrddhi_blocks();
                        gp.try_run(code, |t| t.try_antya_guna());
                    }
                } else {
                    let sub = al::to_guna(anga.upadha()?)?;
                    if !(sub == "a" || sub == "A") {
                        gp.check_guna_vrddhi_blocks();
                        gp.try_run(code, |t| t.try_upadha_guna());
                    }
                }
            }
        } else if is_ik {
            gp.check_guna_vrddhi_blocks();
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

    if !p.terms().last()?.has_lakshana("li~w") {
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
    if anga.has_antya('f') && anga.is_samyogadi() {
        p.run_at("7.4.10", i, do_ar_guna);
    } else if anga.has_antya('F') || (anga.has_u_in(&["fCa~", "f\\"]) && anga.has_adi('f')) {
        if anga.has_u("fCa~") {
            p.run_at("7.4.11", i, op::adi("ar"));
        } else {
            let mut skipped = false;
            if anga.has_text_in(&["SF", "dF", "pF"]) && !anga.has_gana(Gana::Curadi) {
                skipped = p.run_optional_at("7.4.12", i, |t| {
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

fn run_for_index(p: &mut Prakriya, i: usize) -> Option<()> {
    let anga = p.get_if(i, |t| !t.is_agama())?;
    let i_n = p.find_next_where(i, |t| !t.is_empty())?;
    let n = p.get(i_n)?;

    if anga.has_u("jAgf") && !n.has_u_in(&["kvin", "ciR", "Ral"]) && !p.view(i_n)?.has_tag(T::Nit) {
        // jAgf-guna takes priority over vrddhi. Skip if already applied (e.g. for jAgf + Ric).
        if anga.has_antya('f') {
            p.run_at("7.3.85", i, |t| {
                t.set_antya("ar");
                t.add_tag(T::FlagGuna);
            });
        }
    } else {
        // Vrddhi takes priority over guna. For example, Ric is Ardhadhatuka (guna)
        // and Rit (vrddhi), but it will cause vrddhi if possible.
        try_vrddhi_adesha(p, i);
        try_guna_adesha(p, i);
        // TODO: 7.4.23-4
    }

    Some(())
}

pub fn run(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        run_for_index(p, i);
        try_r_guna_before_lit(p, i);
    }

    p.maybe_save_sthanivat();
    Some(())
}
