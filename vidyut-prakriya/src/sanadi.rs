//! Runs rules that add *sanādi pratyaya*s to the end of a dhatu or subanta.
use crate::args::Agama as A;
use crate::args::Gana::*;
use crate::args::Sup;
use crate::args::{Namadhatu, Pratipadika, Sanadi};
use crate::core::errors::*;
use crate::core::operators as op;
use crate::core::Tag as T;
use crate::core::Term;
use crate::core::{Prakriya, Rule};
use crate::dhatu_gana;
use crate::ganapatha as gana;
use crate::it_samjna;
use crate::pratipadika_karya;
use crate::sounds::HAL;
use crate::Rule::Varttika;

// These dhatus use their pratyaya optionally if followed by an ArdhadhAtuka-pratyaya.
const AYADAYA: &[&str] = &[
    "gupU~", "DUpa~", "viCa~", "paRa~\\", "pana~\\", "fti", "kamu~\\",
];

struct SanadiPrakriya<'a> {
    p: &'a mut Prakriya,
    /// The index after which we will insert the *sanādi pratyaya*.
    i_base: usize,
}

impl<'a> SanadiPrakriya<'a> {
    fn new(p: &'a mut Prakriya, i_base: usize) -> Self {
        Self { p, i_base }
    }

    fn run_for(
        p: &mut Prakriya,
        i_base: usize,
        rule: impl Into<Rule>,
        aupadeshika: &str,
        func: impl Fn(&mut Prakriya),
    ) {
        p.run(rule, |p| {
            // TODO: do others. (Refactoring.) These are the most important.
            let sanadi = match aupadeshika {
                "san" => Term::from(Sanadi::san),
                "yaN" => Term::from(Sanadi::yaN),
                "Ric" => Term::from(Sanadi::Ric),
                "kyaN" => Term::from(Sanadi::kyaN),
                "kyac" => Term::from(Sanadi::kyac),
                "kAmyac" => Term::from(Sanadi::kAmyac),
                _ => {
                    let mut t = Term::make_upadesha(aupadeshika);
                    t.add_tags(&[T::Pratyaya]);
                    t
                }
            };

            p.insert_after(i_base, sanadi);
            func(p);

            if !p.has(i_base, |t| t.is_dhatu()) {
                p.set(i_base + 1, |t| t.add_tag(T::FlagNoArdhadhatuka));
            }
        });

        let i_pratyaya = i_base + 1;
        p.add_tag_at("3.1.32", i_pratyaya, T::Dhatu);
        it_samjna::run(p, i_pratyaya).expect("ok")
    }

    /// Adds `upadesha` as a pratyaya after the dhatu at index `i_dhatu`.
    fn add(&mut self, rule: impl Into<Rule>, upadesha: &str) {
        SanadiPrakriya::run_for(self.p, self.i_base, rule, upadesha, |_| {});
    }

    fn add_with(&mut self, rule: impl Into<Rule>, upadesha: &str, func: impl Fn(&mut Prakriya)) {
        SanadiPrakriya::run_for(self.p, self.i_base, rule, upadesha, func);
    }

    fn optional_add(&mut self, rule: impl Into<Rule>, upadesha: &str) {
        self.optional_add_with(rule, upadesha, |_| {});
    }

    fn optional_add_with(
        &mut self,
        rule: impl Into<Rule>,
        upadesha: &str,
        func: impl Fn(&mut Prakriya),
    ) {
        let i_base = self.i_base;
        self.p.optionally(rule.into(), |rule, p| {
            SanadiPrakriya::run_for(p, i_base, rule, upadesha, func);
        });
    }
}

/// Tries to add a *sanādi pratyaya* to the prakriya `p`.
///
/// This function supports two different use cases:
///
/// 1. If `sanadi` is defined, the function tries to add the specific pratyaya and does nothing if
///    pratyaya cannot be added. Examples: cikIrzati, putrakAmyati, ...
///
/// 2. If `sanadi` is `None`, the function tries to add the first possible sanadi pratyaya and
///    does nothing if no such rule can be found. Examples: bAzpayate, muRqayati, ...
fn try_add(p: &mut Prakriya, sanadi: &Option<Sanadi>, is_ardhadhatuka: bool) -> Option<()> {
    use Sanadi::*;

    let i_last = p.terms().len() - 1;
    let i_base = p.find_last_where(|t| !t.is_empty())?;

    let mut sp = SanadiPrakriya::new(p, i_last);
    let base = sp.p.get(i_base)?;
    let sup = sp.p.has(i_base + 1, |t| t.is_sup())
        || (sp.p.has(i_base + 1, |t| t.is_nyap_pratyaya()) && sp.p.has(i_base + 2, |t| t.is_sup()));

    // `Gana` is required so that we can exclude "03.0021 kita~".
    if base.is_dhatu() && base.has_u_in(&["gupa~\\", "tija~\\", "kita~"]) && base.has_gana(Bhvadi) {
        // jugupsate, titikzate, cikitsati
        sp.add_with("3.1.5", san.as_str(), |p| {
            p.set(i_base + 1, |t| t.add_tag(T::FlagNoArdhadhatuka));
        });
    } else if base.is_any_u(dhatu_gana::MAN_BADHA) {
        // mImAMsate, etc.
        sp.add_with("3.1.6", san.as_str(), |p| {
            // TODO: optional by extension of "vA" from 3.1.7 per Kashika?
            p.set(i_base + 1, |t| t.add_tag(T::FlagNoArdhadhatuka));
        });
    } else if matches!(sanadi, Some(san)) {
        sp.add("3.1.7", san.as_str());
    } else if sup && matches!(sanadi, Some(kyac)) {
        // putrIyati, ...
        sp.add("3.1.8", kyac.as_str());
    } else if sup && matches!(sanadi, Some(kAmyac)) {
        // putrakAmyati, ...
        sp.add("3.1.9", kAmyac.as_str());
    } else if sup && matches!(sanadi, Some(kyaN)) {
        // SyenAyate, ..
        if base.has_text_in(&["ojas", "apsaras"]) {
            // ojAyate, apsarAyate
            sp.add_with("3.1.11", kyaN.as_str(), |p| {
                p.set(i_base, |t| t.set_antya(""));
            });
        } else {
            // SyenAyate, ..
            sp.add("3.1.11", kyaN.as_str());
            if sp.p.has(i_base, |t| t.has_antya('s')) {
                // payAyate, payasyate, ...
                sp.p.optional_run_at(Varttika("3.1.11.1"), i_base, |t| t.set_antya(""));
            }
        }
    } else if sup && matches!(sanadi, Some(Ric)) {
        // pawayati, ...
        sp.add_with(Rule::Dhatupatha("10.0502"), Ric.as_str(), |p| {
            p.set(i_base + 1, |t| t.add_tag(T::FlagNoArdhadhatuka));
        });
    } else if sup && base.has_text_in(gana::BHRSHA_ADI) {
        // BfSAyate, ..
        sp.add_with("3.1.12", kyaN.as_str(), |p| {
            p.set(i_base, |t| {
                if t.has_antya(HAL) {
                    t.set_antya("");
                }
            })
        });
    } else if sup && base.has_text_in(gana::LOHITA_ADI) || base.has_u("qAc") {
        // lohitAyati, lohitAyate, ..
        sp.add("3.1.13", "kyaz");
    } else if sup && base.has_text("kazwa") {
        // kazwAyate, ...
        sp.add("3.1.14", kyaN.as_str());
    } else if sup && base.has_text_in(&["satra", "kakza", "kfcCra", "gahana"]) {
        // kazwAyate, ...
        sp.add(Varttika("3.1.14.1"), kyaN.as_str());
    } else if sup && base.has_text_in(&["romanTa", "tapas"]) {
        let is_tapas = base.has_text("tapas");
        // romanTAyate, ...
        sp.add("3.1.15", kyaN.as_str());
        // tapasyati, ...
        if is_tapas {
            sp.p.run_at(Varttika("3.1.15.2"), sp.p.terms().len() - 1, |t| {
                t.remove_tag(T::Nit)
            });
        }
    } else if sup && base.has_text_in(&["bAzpa", "uzma"]) {
        // bAzpAyate, ...
        sp.add("3.1.16", kyaN.as_str());
    } else if sup && base.has_text("Pena") {
        // PenAyate, ...
        sp.add(Varttika("3.1.16.1"), kyaN.as_str());
    } else if sup && base.has_text_in(&["Sabda", "vEra", "kalaha", "aBra", "kaRva", "meGa"]) {
        // SabdAyate, ...
        sp.add("3.1.17", kyaN.as_str());
    } else if sup && base.has_text_in(&["sudina", "durdina", "nIhAra"]) {
        // sudinAyate, ...
        sp.add(Varttika("3.1.17.1"), kyaN.as_str());
    } else if sup
        && base.has_text_in(&[
            "awA", "awwA", "SIkA", "kowA", "powA", "sowA", "kazwA", "pruzwA", "pluzwA",
        ])
    {
        // awAyate, ...
        sp.add(Varttika("3.1.17.2"), kyaN.as_str());
    } else if sup && base.has_text_in(gana::SUKHA_ADI) {
        // suKAyate, ...
        sp.add("3.1.18", kyaN.as_str());
    } else if sup && base.has_text_in(&["namas", "varivas", "citra"]) {
        // namasyati, ...
        sp.add_with("3.1.19", kyac.as_str(), |p| {
            if p.has(i_base, |t| t.has_text("citra")) {
                p.set(i_base + 2, |t| t.add_tag(T::Nit));
            }
        });
    } else if sup && base.has_text_in(&["pucCa", "BARqa", "cIvara"]) {
        // utpucCayate, ...
        sp.add("3.1.20", "RiN");
    } else if sup
        && base.has_text_in(&[
            "muRqa", "miSra", "SlakzRa", "lavaRa", "vrata", "vastra", "hali", "kali", "kfta",
            "tUsta",
        ])
    {
        // muRqayati, ...
        sp.add_with("3.1.21", Ric.as_str(), |p| {
            p.set(i_base, |t| {
                if t.has_text_in(&["hali", "kali"]) {
                    t.set_antya("a");
                }
            })
        });
    } else if matches!(sanadi, Some(Sanadi::yaN) | Some(Sanadi::yaNluk)) {
        if base.has_text_in(&["lup", "sad", "car", "jap", "jaB", "dah", "daS", "gF"]) {
            sp.add("3.1.24", yaN.as_str());
        } else if (i_base > 0
            && sp
                .p
                .has(i_base - 1, |t| t.has_u_in(&["sUca", "sUtra", "mUtra"])))
            || base.has_u_in(&["awa~", "f\\", "aSa~", "aSU~\\", "UrRuY"])
        {
            sp.add(Varttika("3.1.22.1"), yaN.as_str());
        } else if base.is_ekac() && base.has_adi(HAL) {
            sp.add("3.1.22", yaN.as_str());
        }

        if matches!(sanadi, Some(Sanadi::yaNluk)) {
            use Rule::Dhatupatha as DP;

            let i_yan = p.find_last_where(|t| t.is_pratyaya() && t.is(yaN))?;

            // Apply luk.
            p.run_at("2.4.74", i_yan, op::luk);

            // "carkarItam ca" declares that yan-luk dhatus are part of ad-Adi gaNa.
            // As a result, we will see lopa of Sap-vikarana per 2.4.72.
            p.run_at(DP("02.0076"), i_yan, |d| d.set_gana(Adadi));
        }
    } else if (sup
        && base.has_text_in(&[
            "satya", "pASa", "rUpa", "vIRA", "tUla", "Sloka", "senA", "loman", "tvaca", "varman",
            "varRa", "cUrRa", "arTa", "veda",
        ]))
        || (base.has_gana(Curadi) && !base.has_tag(T::FlagNoNic))
    {
        // satyApayati, ..., corayati, ...
        sp.add("3.1.25", Ric.as_str());

        let base = sp.p.get(i_base)?;
        if sup && base.has_text_in(&["satya", "arTa", "veda"]) {
            // satyApayati, arTApayati, vedApayati
            op::insert_before(Varttika("3.1.25.1"), sp.p, i_base + 2, A::Apuk);
        }
    } else if matches!(sanadi, Some(Ric)) {
        // kArayati, ...
        sp.add("3.1.26", Ric.as_str());
    } else if base.has_gana(Kandvadi) {
        // kaNDUyati, ...
        //
        // "dvivadhāḥ kaṇḍvādayo, dhātavaḥ prātipādikāni ca. tatra dhātvadhikārād
        // dhātubhyaḥ eva pratyayo vidhīyate, na tu prātipadikebhyaḥ"
        //
        // -- KV on 3.1.27.
        //
        // We avoid `has_u_in` because it turns up too many false positives, e.g. "asu~" (asyati).
        sp.add("3.1.27", "yak");
    } else if base.has_u_in(AYADAYA) {
        let mut can_add_pratyaya = true;

        if is_ardhadhatuka {
            can_add_pratyaya = !sp.p.optionally("3.1.31", |rule, p| {
                p.run_at(rule, i_base, |t| {
                    // TODO: not sure where to do this.
                    if t.has_u("fti") {
                        t.set_text("ft")
                    }
                });
            });
        }

        if can_add_pratyaya {
            let base = sp.p.get(i_base)?;
            if base.has_u_in(&["gupU~", "DUpa~", "viCa~", "paRa~\\", "pana~\\"]) {
                let rule = "3.1.28";
                if base.has_u("paRa~\\") {
                    // > stutyarthena paninā sāhacaryāt tadarthaḥ paṇiḥ pratyayamutpādayati na
                    // > vyavahārārthaḥ. śatasya paṇate. sahasrasaya paṇate
                    // -- KV on 3.1.28
                    sp.optional_add(rule, "Aya");
                } else {
                    sp.add(rule, "Aya");
                }
            } else if base.has_u("fti") {
                // ftIyate
                //
                // From the bAlamanorAma:
                //
                // takārānto dhāturayamikā nirdiṣṭo na tvikārāntaḥ । idantatva hi
                // savarṇadīrgheṇaiva siddhe īyaṅiti īkāroccāraṇavaiyarthyāt । naca idantatve sati
                // 'eranekācaḥ' iti yaṇ syāditi vācyam, evamapi 'ṛterṅyaḥ' iti ṅyapratyaye
                // kṛte "akṛtsārvadhātukayordīrgha" iti dīrgheṇaiva siddhe iyaṅvidhivaiyarthyāt
                sp.add_with("3.1.29", "IyaN", |p| {
                    p.set(i_base, |t| t.set_antya(""));
                });
            } else if base.has_u("kamu~\\") {
                sp.add("3.1.30", "RiN");
            }
        }
    }

    Some(())
}

/// Tries to create a *nāmadhātu* using the given arguments.
pub fn try_create_namadhatu(p: &mut Prakriya, dhatu: &Namadhatu) -> Option<()> {
    match dhatu.pratipadika() {
        Pratipadika::Basic(basic) => {
            pratipadika_karya::add_basic(p, basic);
        }
        _ => panic!("Unsupported type for namadhatu"),
    }

    let mut su = Term::from(Sup::su);
    su.set_text("");
    su.add_tags(&[T::Vibhakti, T::V1, T::Luk]);
    p.push(su);

    try_add(p, dhatu.nama_sanadi(), false);

    Some(())
}

pub fn try_add_required(p: &mut Prakriya, is_ardhadhatuka: bool) {
    try_add(p, &None, is_ardhadhatuka);
}

/// Tries to add the given `sanadi`-pratyaya and returns an error if the addition failed.
pub fn try_add_optional(p: &mut Prakriya, sanadi: Sanadi) -> Result<()> {
    try_add(p, &Some(sanadi), false);

    // For now, raise errors only for yan and yan-luk, since these are the only two public values
    // that can fail.
    if matches!(sanadi, Sanadi::yaN | Sanadi::yaNluk) {
        if let Some(t) = p.terms().last() {
            if !(t.is(Sanadi::yaN) && t.is_pratyaya()) {
                return Err(Error::Abort(p.rule_choices().to_vec()));
            }
        }
    }

    Ok(())
}
