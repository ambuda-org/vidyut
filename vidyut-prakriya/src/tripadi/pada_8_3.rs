use crate::args::Gana;
use crate::core::char_view::{get_term_and_offset_indices, xy, CharPrakriya};
use crate::core::iterators::xy_rule;
use crate::core::operators as op;
use crate::core::Prakriya;
use crate::core::Rule;
use crate::core::Rule::Varttika;
use crate::core::Tag as T;
use crate::core::Term;
use crate::it_samjna;
use crate::sounds as al;
use crate::sounds::{s, Set};
use lazy_static::lazy_static;

lazy_static! {
    static ref IN1: Set = s("iR");
    static ref IN2: Set = s("iR2");
    static ref IN_KU: Set = s("iR2 ku~");
    static ref KU_PU: Set = s("ku~ pu~");
    static ref KHAR: Set = s("Kar");
    static ref JHAL: Set = s("Jal");
    static ref SHAR: Set = s("Sar");
    static ref HAL: Set = s("hal");
    static ref AC: Set = s("ac");
    static ref IK: Set = s("ik");
    static ref AA: Set = s("a");
    static ref ASH: Set = s("aS");
}

fn try_ra_lopa(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let is_avasane = p.find_next_where(i, |t| !t.is_empty()).is_none();
        let is_khari = match p.find_next_where(i, |t| !t.is_empty()) {
            Some(j) => p.has(j, |t| t.has_adi(&*KHAR)),
            None => false,
        };

        // 8.3.15
        // TODO: next pada
        let c = p.get(i)?;
        let has_ru = c.has_antya('r') && !c.has_tag(T::FlagAntyaAcSandhi);
        if !has_ru {
            continue;
        }

        if p.has_next_non_empty(i, |t| t.has_adi('r')) {
            p.run_at("8.3.14", i, op::antya(""));
            let c = p.get(i)?;
            if c.is_hrasva() {
                let sub = al::to_dirgha(p.get(i)?.antya()?)?;
                // Placed here, otherwise this is vyartha. See `8.3.13` for the Qa case of
                // this rule.
                p.run_at("6.3.111", i, op::antya(&sub.to_string()));
            }
        } else if c.ends_with("rr") {
            // Special implementation for apaspAH, etc.
            p.run_at("8.3.14", i, op::antya(""));
            let sub = al::to_dirgha(p.get(i)?.upadha()?)?;
            // Placed here, otherwise this is vyartha. See `8.3.13` for the Qa case of
            // this rule.
            p.run_at("6.3.111", i, |t| t.set_upadha(&sub.to_string()));
        }

        if p.is_pada(i) && (is_avasane || is_khari) {
            if p.find_next_where(i, |t| !t.is_empty() && t.is_sup())
                .is_some()
            {
                if p.has(i, |t| t.has_tag(T::Ru)) {
                    // payaHsu, ...
                    p.run_at("8.3.16", i, |t| t.set_antya("H"));
                }
                // Otherwise, we have gIrzu, etc.
            } else {
                // vfkzaH, ...
                p.run_at("8.3.15", i, |t| t.set_antya("H"));
            }
        }
    }

    xy_rule(
        p,
        |x, y| {
            x.has_antya('r')
                && x.has_tag(T::Ru)
                && (x.has_u_in(&["Bos", "Bagos", "aGos"]) || x.has_upadha(&*AA))
                && y.has_adi(&*ASH)
        },
        |p, i, j| {
            p.run_at("8.3.17", i, |t| t.set_antya("y"));
            if p.has(j, |t| t.has_adi(&*AC)) {
                // Though technically optional, avoid including other rules to prevent creating
                // noisy output.
                p.run_at("8.3.19", i, |t| t.set_antya(""));
            } else {
                p.run_at("8.3.22", i, |t| t.set_antya(""));
            }
        },
    );

    Some(())
}

/// Converts "m" and "n" to the anusvara when a consonant follows.
///
/// Example: Sankate -> SaMkate
fn try_mn_to_anusvara(p: &mut Prakriya) -> Option<()> {
    // Disable certain sandhi changes if deriving a dhatu since we'll run these rules again when
    // deriving the final word.
    //
    // Example: sam + cekrIya should not become saYcekrIya so that we can later derive
    // samacekrIyata, etc.
    let is_dhatu_prakriya = p.terms().last()?.is_dhatu();

    for i in 0..p.terms().len() {
        if p.has(i, |t| t.has_antya('m')) && p.is_pada(i) {
            if let Some(i_next) = p.find_next_where(i, |t| !t.is_empty()) {
                if p.has(i_next, |t| t.has_adi(&*HAL)) && !is_dhatu_prakriya {
                    p.run_at("8.3.23", i, |t| t.set_antya("M"));
                }
            }
        }
    }

    // TODO: a-padAnta
    let mut cp = CharPrakriya::new(p);
    cp.for_chars(
        xy(|x, y| (x == 'm' || x == 'n') && JHAL.contains(y)),
        |p, _, i| {
            if let Some((i_term, i_offset)) = get_term_and_offset_indices(p, i) {
                let t = p.get(i_term).expect("ok");
                if t.is_pada() && i_offset + 1 == t.len() {
                    false
                } else if t.is_unadi() && t.has_u("qumsu~n") {
                    // for qumsun-pratyaya
                    false
                } else if t.has_text("pums") && !p.has(i_term + 1, |t| t.is_pada()) {
                    // Don't make this change for "m" in a pratipadika, so that we can derive
                    // "supums".
                    false
                } else {
                    p.run("8.3.24", |p| p.set_char_at(i, "M"));
                    true
                }
            } else {
                false
            }
        },
    );

    Some(())
}

fn try_add_dhut_agama(p: &mut Prakriya) -> Option<()> {
    for i in 0..p.terms().len() {
        let j = p.find_next_not_empty(i)?;
        let x = p.get(i)?;
        let y = p.get(j)?;
        if p.is_pada(i) && x.has_antya('q') && y.has_adi('s') {
            p.optionally("8.3.29", |rule, p| {
                op::insert_agama_before(p, j, "Du~w");
                p.step(rule);
                it_samjna::run(p, j).ok();
            });
        }
    }

    Some(())
}

/// Runs rules that modify the visarjanIya (visarga).
/// (8.3.34 - 8.3.54)
fn try_visarjaniyasya(p: &mut Prakriya) -> Option<()> {
    let is_it_ut_upadha = |x: &Term| x.has_upadha('i') || x.has_upadha('u');
    let is_samasa = |p: &Prakriya, i_x| p.has(i_x + 1, |t| t.is_empty() && t.is_pada());

    for i_x in 0..p.terms().len() {
        let x = match p.get_if(i_x, |t| t.has_antya('H')) {
            Some(t) => t,
            None => continue,
        };

        let i_y = p.find_next_where(i_x, |t| !t.is_empty())?;
        let y = p.get(i_y)?;

        if y.has_adi(&*SHAR) {
            p.optional_run_at("8.3.36", i_x, |t| t.set_antya("s"));
        } else if y.has_at(1, &*SHAR) {
            p.run_at("8.3.35", i_x, |_| {});
        } else if y.has_adi(&*KU_PU) {
            if x.has_text_in(&["namas", "puras"]) && x.is_gati() {
                p.run_at("8.3.40", i_x, |t| t.set_antya("s"));
            } else if is_it_ut_upadha(x) && !x.is_pratyaya() {
                p.run_at("8.3.41", i_x, |t| t.set_antya("z"));
            } else if x.has_u("tiras") && x.is_gati() {
                p.optional_run_at("8.3.42", i_x, |t| t.set_antya("s"));
            } else if x.ends_with("aH") && is_samasa(p, i_x) && !x.is_avyaya() && y.has_u("qukf\\Y")
            {
                p.run_at("8.3.46", i_x, |t| t.set_antya("s"));
            } else if x.has_text("BAH") && y.has_text("kar") {
                // TODO: rest of kaskAdi
                p.run_at("8.3.48", i_x, |t| t.set_antya("s"));
            } else if x.has_tag(T::Ru) && y.is_pratyaya() {
                if x.has_antya(&*IN1) {
                    // sarpizpASam, ...
                    p.run_at("8.3.39", i_x, |t| t.set_antya("z"));
                } else {
                    // yaSaskAmyati, svaHkAmyati,
                    p.run_at("8.3.38", i_x, |t| t.set_antya("s"));
                }
            } else {
                // TODO: jihvamuliya and upadhmaniya
                p.run_at("8.3.37", i_x, |t| t.set_antya("H"));
            }
        } else {
            p.run_at("8.3.34", i_x, |t| t.set_antya("s"));
        }
    }

    Some(())
}

#[derive(Debug)]
struct ShaPrakriya<'a> {
    p: &'a mut Prakriya,
    i_term: usize,
    i_char: usize,
    done: bool,
}

impl<'a> ShaPrakriya<'a> {
    fn new(p: &'a mut Prakriya, i_char: usize) -> Self {
        let i_term = p.find_for_char_at(i_char).expect("ok");
        Self {
            p,
            i_term,
            i_char,
            done: false,
        }
    }

    fn prev(&self) -> Option<&Term> {
        let i_prev = self.p.find_prev_where(self.i_term, |t| !t.is_empty())?;
        self.p.get(i_prev)
    }

    /// Returns a reference to the current term.
    fn term(&self) -> &Term {
        self.p.get(self.i_term).expect("ok")
    }

    fn i_prev(&self) -> Option<usize> {
        self.p.find_prev_where(self.i_term, |t| !t.is_empty())
    }

    fn has_upasarga_in(&self, text: &[&str]) -> bool {
        if let Some(i_prev) = self.i_prev() {
            self.p.has(i_prev, |t| t.has_text_in(text))
        } else {
            false
        }
    }

    fn try_block(&mut self, rule: impl Into<Rule>) {
        if !self.done {
            self.p.step(rule);
        }
        self.done = true;
    }

    fn try_shatva(&mut self, rule: impl Into<Rule>) {
        if !self.done {
            self.p.run(rule, |p| p.set_char_at(self.i_char, "z"));
        }
        self.done = true;
    }

    fn try_run_with(&mut self, rule: impl Into<Rule>, func: impl Fn(&mut Prakriya)) {
        if !self.done {
            self.p.run(rule.into(), func);
        }
        self.done = true;
    }

    fn optional_try_shatva(&mut self, rule: impl Into<Rule>) -> bool {
        if !self.done {
            let done = self
                .p
                .optional_run(rule, |p| p.set_char_at(self.i_char, "z"));
            self.done = done;
            done
        } else {
            false
        }
    }
}

fn run_shatva_rules_at_char_index(sp: &mut ShaPrakriya, text: &str) -> Option<()> {
    sp.p.debug(format!("shatva, term={} char={}", sp.i_term, sp.i_char));
    use Gana::*;

    let i = sp.i_term;
    let t = sp.term();

    // Skip abhyasas since we handle these as part of the dhatu rules.
    // Check for SaAdeshadi for Irzyi[zi]zati
    if t.is_abhyasa() && !t.has_tag(T::FlagSaAdeshadi) {
        return None;
    }

    // Skip unAdi-pratyayas that don't allow satva.
    if t.is_krt() && t.has_u("isan") {
        return None;
    }

    // Various niyamas on shatva
    // -------------------------
    let maybe_i_upasarga = sp.p.find_prev_where(sp.i_term, |t| t.is_upasarga());
    let inku = if let Some(i) = maybe_i_upasarga {
        let upasarga = sp.p.get(i).expect("ok");
        upasarga.has_antya(&*IN_KU)
    } else {
        false
    };

    if t.has_at(1, 'r') || t.has_u_in(&["sf\\px", "sf\\ja~", "sf\\ja~\\", "spf\\Sa~", "spfha"]) {
        // visfpa
        // TODO: savanAdi
        sp.try_block("8.3.110");
    } else if (t.has_u("sAt") && t.is_pratyaya()) || sp.i_char == 0 {
        // daDisAt, daDi siYcati
        sp.try_block("8.3.111");
    } else if t.has_u("zi\\ca~^") && sp.p.has(i + 1, |t| t.has_u("yaN")) {
        // aBisesicyate
        sp.try_block("8.3.112");
    } else if t.has_u_in(&["ziDa~"]) && sp.has_upasarga_in(&["pari", "aBi", "vi"]) {
        // Based on commentary, this seems to apply only for abhi-sidh and
        // pari-sidh. SK has vi-sidh.
        sp.try_block("8.3.113");
    } else if t.has_text("saQ") {
        // parisoQa, parisoQum, ...
        //
        // The rule says "soQ" but "soQ" appears only after rule 8.3.13, which itself can occur
        // only after rule 8.4.41.
        sp.try_block("8.3.115");
    } else if inku
        && t.has_u_in(&["stanBu~", "zivu~", "zaha~\\"])
        && sp.p.has(i + 1, |t| t.has_u("caN"))
    {
        // paryatasTabat, paryazIsivat, ...
        //
        // Per varttika, this is specifically for the abhyasa, not the dhatu.
        sp.try_block("8.3.116");
    } else if t.has_u("zu\\Y") && sp.p.has(i + 1, |t| t.has_u_in(&["sya", "san"])) {
        // aBisozyati, ...
        sp.try_block("8.3.117");
    } else if inku && sp.p.terms().last()?.has_lakshana("li~w") && sp.i_term > 0 {
        let i_abhyasa = sp.i_term - 1;
        let has_abhyasa = sp.p.has(i_abhyasa, |t| t.is_abhyasa() && !t.is_empty());
        if t.has_u("za\\dx~") && has_abhyasa {
            // nizasAda, ...
            sp.try_run_with("8.3.118", |p| p.set(i_abhyasa, |t| t.set_adi("z")));
        } else if t.has_u("zva\\nja~\\") && has_abhyasa {
            // parizasvaje, ...
            sp.try_run_with(Varttika("8.3.118.1"), |p| {
                p.set(i_abhyasa, |t| t.set_adi("z"))
            });
        }
    }

    // Rules that condition on an upasarga
    // -----------------------------------
    // In general, various terms might intervene between the upasarga and the dhatu. Examples:
    //
    // 1. an abhyAsa (upa-sa-sAra)
    // 2. aw-Agama (sam-a-smarat)
    // 3. suw-Agama (pari-z-karoti, pary-a-z-karot)
    //
    // The terms here have three main sections:
    //
    // a. All terms -- no vyayAya
    // b. Up to sita -- aw-vyavAya OK
    // c. stha to sita -- abhyAsa-vyavAya and aw-vyavAya OK

    let term = sp.term();
    let maybe_i_upasarga = sp.p.find_prev_where(sp.i_term, |t| t.is_upasarga());
    if maybe_i_upasarga.is_some() {
        let i_upasarga = maybe_i_upasarga?;

        // No gap between upasarga and dhatu.
        let no_vyavaya = i_upasarga + 2 == sp.i_term;
        // Gap between upasarga and dhatu is aw-Agama.
        let at_vyavaya = i_upasarga + 3 == sp.i_term
            && sp.p.has(i_upasarga + 2, |t| t.is_agama() && t.has_u("aw"));
        // Gap between upasarga and dhatu is just an abhyasa.
        let abhyasa_vyavaya = i_upasarga + 3 == sp.i_term
            && sp.p.has(i_upasarga + 2, |t| t.is_abhyasa())
            && !sp.p.has(i_upasarga, |t| t.has_tag(T::FlagAntyaAcSandhi));

        // Check both upadesha and gana to avoid matching dhatus in other ganas.
        const SU_TO_STUBH: &[(&str, Gana)] = &[
            ("zu\\Y", Svadi),
            ("zU", Tudadi),
            ("zo\\", Divadi),
            ("zwu\\Y", Adadi),
            ("zwuBu~\\", Bhvadi),
        ];
        const SU_ADI: &[(&str, Gana)] = &[
            ("zu\\Y", Svadi),
            ("zU", Tudadi),
            ("zo\\", Divadi),
            ("zwu\\Y", Adadi),
            ("zwuBu~\\", Bhvadi),
            ("zWA\\", Bhvadi),
            // TODO: senaya (senA + Ric)
            ("ziDa~", Bhvadi),
            ("ziDU~", Bhvadi),
            ("zi\\ca~^", Tudadi),
            ("za\\nja~", Bhvadi),
            ("zva\\nja~\\", Bhvadi),
        ];
        const SEV_ADI: &[(&str, Gana)] = &[
            ("zevf~\\", Bhvadi),
            // TODO: sita, saya
            ("zivu~", Divadi),
            ("zaha~\\", Bhvadi),
            // TODO: suw-Agama
            ("zwu\\Y", Adadi),
            ("zva\\nja~\\", Bhvadi),
        ];

        let upasarga = sp.p.get(i_upasarga)?;
        let inku = upasarga.has_antya(&*IN_KU);
        let has_dhatu_in = |d: &Term, items: &[(&str, Gana)]| {
            items.iter().any(|(u, g)| d.has_gana(*g) && d.has_u(u))
        };

        // By 8.3.64, zatva also occurs for the abhyasa of dhatus starting with sthA in 8.3.65 and
        // ending with 8.3.70 inclusive.
        if no_vyavaya || at_vyavaya || abhyasa_vyavaya {
            if has_dhatu_in(term, SEV_ADI) || term.has_u("su~w") {
                // Rules 8.3.70 and 8.3.71 take priority over 8.3.65 so that we can handle `stu` correctly.
                if upasarga.has_u_in(&["pari", "ni", "vi"]) {
                    // TODO: also exclude sita
                    let is_siv_adi = !term.has_u("zevf~\\");
                    if is_siv_adi {
                        // "siv" follows "sita" which means we lose "at_vyavaya" and
                        // "abhyava_vyavaya". But 8.3.71 allows "at_vyavaya" again.
                        //
                        // The rule says "aw-vyavAye 'pi" but the KV heavily implies this is
                        // optional strictly for "aw" and mandatory otherwise.
                        if at_vyavaya {
                            sp.optional_try_shatva("8.3.71");
                            sp.done = true;
                        } else if no_vyavaya {
                            sp.try_shatva("8.3.70");
                        }
                    } else {
                        // Other terms (sev, sita) are allowed in any condition.
                        sp.try_shatva("8.3.70");
                    }
                }
            }

            // Avoid `else if` here so we can match svanj again (pratyazvaNkta)
            let term = sp.term();
            let upasarga = sp.p.get(i_upasarga)?;
            if inku && has_dhatu_in(term, SU_ADI) {
                let is_stha_adi = !has_dhatu_in(term, SU_TO_STUBH);
                if no_vyavaya || at_vyavaya || (abhyasa_vyavaya && is_stha_adi) {
                    sp.try_shatva("8.3.65");
                }
            } else if inku && term.has_u("za\\dx~") {
                let code = "8.3.66";
                if upasarga.has_u("prati") {
                    sp.try_block(code);
                } else {
                    sp.try_shatva(code);
                }
            } else if term.has_u("stanBu~") {
                if upasarga.has_u("ava") {
                    sp.optional_try_shatva("8.3.68");
                } else if inku {
                    // > aprateḥ ityetadiha na anuvartate, tena etadapi bhavati, pratiṣṭabhnāti,
                    // > prayaṣṭabhnāt, pratitaṣṭambha
                    // -- Kashika on 8.3.67
                    sp.try_shatva("8.3.67");
                }
            } else if term.has_u("svana~") && upasarga.has_u_in(&["ava", "vi"]) {
                // vizvanati, visvanati
                sp.optional_try_shatva("8.3.69");
            }

            let dhatu = sp.term();
            if sp.done && dhatu.has_adi('z') {
                let i_abhyasa = sp.i_term - 1;
                let is_stha_adi = !has_dhatu_in(dhatu, SU_TO_STUBH);
                if sp.p.has(i_abhyasa, |t| t.is_abhyasa() && t.has_adi('s')) && is_stha_adi {
                    sp.p.run_at("8.3.64", i_abhyasa, |t| t.set_adi("z"));
                }
            }
        }

        if no_vyavaya || at_vyavaya {
            let dhatu = sp.term();
            let upasarga = sp.p.get(i_upasarga)?;
            if upasarga.has_u_in(&["anu", "vi", "pari", "aBi", "ni"]) && dhatu.has_u("syandU~\\") {
                sp.optional_try_shatva("8.3.72");
                sp.done = true;
            } else if upasarga.has_u_in(&["vi", "pari"]) && dhatu.has_u("ska\\ndi~r") {
                if upasarga.has_u("vi") {
                    if !sp.p.has(sp.i_term + 1, |t| t.is_nistha()) {
                        sp.optional_try_shatva("8.3.73");
                    }
                } else {
                    sp.optional_try_shatva("8.3.74");
                }
            } else if upasarga.has_u_in(&["nis", "ni", "vi"])
                && dhatu.has_u_in(&["sPura~", "sPula~"])
            {
                sp.optional_try_shatva("8.3.76");
            } else if upasarga.has_u("vi") && dhatu.has_u("skanBu~") {
                sp.try_shatva("8.3.77");
            }
        }
        let term = sp.term();
        if term.has_tag(T::FlagSaAdeshadi) && !abhyasa_vyavaya && !term.is_abhyasa() {
            sp.try_block("8.3.111");
        }
    }

    let term = sp.term();
    if sp.i_term > 0 && term.is_samasa() {
        // TODO: these rules assume one term per pratipadika.
        let i_purva = sp.p.find_prev_where(sp.i_term, |t| !t.is_empty())?;
        let purva = sp.p.get(i_purva)?;
        let uttara = term;

        if purva.has_text("aNguli") && uttara.has_u("saNga") {
            // aNgulizaNga
            sp.try_shatva("8.3.80");
        } else if purva.has_text("BIru") && uttara.has_u("sTAna") {
            // BIruzWAna
            sp.try_shatva("8.3.81");
        } else if purva.has_text("agni") && uttara.has_u_in(&["stut", "stoma", "soma"]) {
            // BIruzWAna
            sp.try_shatva("8.3.82");
        } else if purva.has_text_in(&["mAtf", "pitf"]) && uttara.has_u("svasf") {
            // mAtfzvasA, ...
            sp.try_shatva("8.3.84");
        } else if purva.has_text_in(&["vi", "ku", "Sami", "pari"]) && uttara.has_u("sTala") {
            // vizWala, ...
            sp.try_shatva("8.3.96");
        } else if purva.has_text_in(&[
            "amba", "Amba", "go", "BUmi", "savya", "apa", "dvi", "tri", "ku", "Seku", "SaNku",
            "aNgu", "maYji", "puYji", "parame", "barhis", "divi", "agni",
        ]) && uttara.has_u("sTa")
        {
            // ambazWa, ...
            sp.try_shatva("8.3.97");
        }
    } else if term.has_upadha(&*IK)
        && term.has_antya('s')
        && sp
            .p
            .has(sp.i_term + 1, |t| t.is_taddhita() && t.has_adi('t'))
    {
        // sarpizwama, ...
        sp.try_shatva("8.3.101");
    }

    // General rules
    // -------------

    let term_view = sp.p.pratyaya(sp.i_term);
    let is_apadanta = match &term_view {
        Some(v) => !v.is_empty(),
        None => false,
    };
    let adesha_pratyaya = match &term_view {
        Some(v) => v.has_tag_in(&[T::Pratyaya, T::FlagSaAdeshadi]),
        None => false,
    };
    let in_koh = match sp.prev() {
        Some(t) => t.has_antya(&*IN_KU),
        None => false,
    };
    if in_koh && is_apadanta && adesha_pratyaya {
        const SVIDI_SVADI: &[&str] = &[
            "YizvidA~\\",
            "YizvidA~",
            "zvi\\dA~",
            "zvada~\\",
            "zvada~",
            "zaha~",
            "zaha~\\",
        ];
        let term = sp.term();
        // Use `find_next_where` to find `san` because position of `san` is uncertain due to iw-Agama
        // and ni-pratyaya. `z` is part of the rule.
        let shan =
            sp.p.find_next_where(sp.i_term, |t| t.has_u("san") && t.has_adi('z'));

        let prev_is_abhyasa = sp.i_term > 0 && sp.p.has(sp.i_term - 1, |t| t.is_abhyasa());
        if shan.is_some() && prev_is_abhyasa {
            let nau = sp.p.has(sp.i_term + 1, |t| t.is_ni_pratyaya());

            // Prefer `has_u_in` over `has_text_in` because `has_u_in` is more reliable and doesn't
            // include sound changes.
            // TODO: does this overgenerate?
            if nau && term.has_u_in(SVIDI_SVADI) {
                sp.try_block("8.3.62");
            } else {
                let code = "8.3.61";
                if term.has_u("zwu\\Y") || nau {
                    // stu -> tuzwUsati
                    // siv -> sizevayizati
                    sp.try_shatva(code);
                } else {
                    // sisikzati
                    sp.try_block(code);
                }
            }
        }
    }

    let mut prev_chars = text[..sp.i_char].chars().rev();
    let inku = if let Some(y) = prev_chars.next() {
        if IN_KU.contains(y) {
            true
        } else {
            let i_term = sp.p.find_for_char_at(sp.i_char - 1).expect("ok");
            let is_num = y == 'M' && sp.p.has(i_term, |t| t.has_tag(T::FlagNum));
            let is_num_visarjaniya_shar = is_num || y == 'H' || SHAR.contains(y);

            if is_num_visarjaniya_shar {
                // Per commentaries, allow at most one num-visarjaniya-shar sound in-between.
                prev_chars.next().map_or(false, |c| IN_KU.contains(c))
            } else {
                false
            }
        }
    } else {
        false
    };

    let is_antya = sp.i_char + 1 == text.len();

    let term = sp.p.pratyaya(sp.i_term)?;
    if inku && term.has_tag_in(&[T::Pratyaya, T::FlagSaAdeshadi]) && !is_antya {
        // For zRus, etc. -- we want to change the first s here, not the second.
        let is_first_s_of_term = if sp.i_term == 0 {
            sp.i_char == 0
        } else {
            let num_chars_before: usize = sp.p.terms()[..sp.i_term].iter().map(|t| t.len()).sum();
            num_chars_before == sp.i_char
        };
        let t = sp.p.get(sp.i_term)?;
        if t.is_dhatu() && !is_first_s_of_term {
            return None;
        }

        if term.has_u("sAti~") {
            // agnisAt ...
            sp.try_block("8.3.111");
        } else if term.last().is_unadi()
            && (term.last().has_u("sara")
                && sp.i_term > 0
                && sp.p.has(sp.i_term - 1, |t| t.has_text_in(&["kf", "DU"])))
        {
            // Do nothing.
        } else {
            // General case.
            sp.try_shatva("8.3.59");
        }
    }

    sp.p.debug("shatva complete");
    Some(())
}

/*
fn find_shatva_spans(text: &str) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    let mut i_inku = None;
    for (i, c) in text.chars().enumerate() {
        if IN_KU.contains(c) {
            // Start of span.
            i_inku = Some(i);
        } else if c == 's' {
            if let Some(inku) = i_inku {
                // End of span.
                ret.push((inku, i));
            }
            i_inku = None;
        } else if c == '~' {
            // Ignore nasal vowel markings.
        } else if !"MH".contains(c) {
            // By 8.3.58, reset if we see a sound that is not in num-visarjanIya.
            i_inku = None;
        }
    }
    ret
}
*/

fn run_shatva_rules(p: &mut Prakriya) -> Option<()> {
    // Iterate in reverse order so that we can produce `san` -> `zan`, which can then trigger
    // 8.3.61.
    //
    // Use a `bytes()` iterator because `chars()` doesn't support `.enumerate().rev()`
    let text = p.compact_text();
    for (i, c) in text.bytes().enumerate().rev() {
        if (c as char) == 's' {
            let mut sp = ShaPrakriya::new(p, i);
            run_shatva_rules_at_char_index(&mut sp, &text);
        }
    }

    // Other s -> z rules
    xy_rule(
        p,
        |x, _| {
            // Also include SAsu~\\ for ASizO, etc.
            x.has_u_in(&["va\\sa~", "SAsu~", "SAsu~\\", "Gasx~"])
                && ((x.has_upadha(&*IN_KU) && x.has_antya('s'))
                // HACK for UsatuH (U + s + atuH)
                || x.has_text("s"))
        },
        |p, i, _| {
            let x = p.get(i).expect("present");
            let code = "8.3.60";
            if x.has_text("s") {
                p.run_at(code, i, op::text("z"));
            } else {
                p.run_at(code, i, op::antya("z"));
            }
        },
    );

    Some(())
}

fn try_murdhanya_for_dha_in_tinanta(p: &mut Prakriya) -> Option<()> {
    let i = p.terms().len() - 1;
    if i == 0 {
        return None;
    }

    let tin = p.get(i)?;

    let dha = tin.has_adi('D');
    let shidhvam_lun_lit = p.get(i - 1)?.has_text("zI") || tin.has_lakshana_in(&["lu~N", "li~w"]);

    let i_anga = p.find_prev_where(i, |t| !t.is_empty() && !t.is_agama())?;
    let allow_it_agama_as_in = if shidhvam_lun_lit && dha {
        false // p.op_optional(Rule::Kaumudi("2258"), |_| {})
    } else {
        false
    };

    let inah = if allow_it_agama_as_in {
        p.find_prev_where(i, |t| t.is_it_agama()).is_some()
    } else {
        let anga = p.get(i_anga)?;
        anga.has_antya(&*IN2)
    };

    if inah && shidhvam_lun_lit && dha {
        if p.has(i_anga + 1, |t| t.is_it_agama()) {
            p.optional_run_at("8.3.79", i, op::adi("Q"));
        } else {
            p.run_at("8.3.78", i, op::adi("Q"));
        }
    }

    Some(())
}

pub fn run(p: &mut Prakriya) {
    try_ra_lopa(p);
    try_mn_to_anusvara(p);
    try_add_dhut_agama(p);
    try_visarjaniyasya(p);

    // Runs rules that make a sound mUrdhanya when certain sounds precede.
    //
    // Example: `nesyati -> nezyati`
    //
    // (8.3.55 - 8.3.119)
    run_shatva_rules(p);
    try_murdhanya_for_dha_in_tinanta(p);
}
