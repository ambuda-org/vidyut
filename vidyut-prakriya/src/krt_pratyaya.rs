/*!
Runs rules that add krt-pratyayas after a dhAtu.

*Status: in progress, with support for major pratyayas.*


### Scope and role of krt-pratyayas

All of the rules that add a pratyaya after a dhAtu are defined within the scope of the adhikAra
rule 3.1.91 (dhAtoH), which extends to the end of 3.4. These pratyayas are of two types:

1. tiN-pratyayas, as well as the lakAras (lat, lit, lut) that those tiN-pratyayas replace. These
   create tinantas like gacCati, jagmuH, etc.

2. All other affixes, which are called *krt*-pratyayas. These create prAtipadikas, including
   nominal stems (gacCan, gata, gantavya) and indeclinables (gantum, gatvA).

Because *krt*-pratyayas create prAtipadikas, they can be followed by sup-pratyayas to create
subantas. However, the following krt-pratyayas are avyayas:

- any krt-pratyaya ending in "m", "e", "E", "o", or "O", per 1.1.39.
- ktvA, tosun, and kasun, per 1.1.40.

And after these krt-pratyayas, sup pratyayas cannot be added.

Once we add a krt-pratyaya, the derivation proceeds in the normal way. Most krt-pratyayas are
called `Ardhadhatuka` by rule 3.4.114, but a few that are `Sit` are called `Sarvadhatuka` by rule
3.4.113.


### Design

We want our code to answer two kinds of questions:

1. *Can this krt pratyaya apply to the given dhatu?* -- This is trivial for common pratyayas like
   `tavya`, but many pratyayas have complex exceptions that we must also check for. For an extreme
   example of this, see the implementation for 3.2.142.

2. *Given this dhatu, which krt pratyayas can we use with it?* -- We can answer this by running (1)
   for each of the 100+ krt-pratyayas we have, but a naive implementation could be unacceptably
   slow.

We focus on (1), not only because we expect this case to be more common but also -- and more
importantly -- because some of these rules have additional side effects beyond merely adding a
pratyaya. For example, 3.1.108 "hanas ta ca", which adds the pratyaya *kyap* while also changing
the last letter of the dhatu to *t*. Therefore, we cannot simply focus on (2) and return a list of
rules and pratyayas: with such an approach, modeling side effects is more cumbersome and less local.


### Internal API

We use a `KrtPrakriya` object that wraps `Prakriya` in a lightweight way and remembers which
pratyaya the caller wants to add. We add a pratyaya to `KrtPrakriya` only if both of the following
are true:

- There is a rule that allows the given `krt`.
- There is no rule that has blocked the given `krt`.

The main functions here are `try_add_krt` and `try_add_krt_for_tacchila_etc`, which each have their
own control flow. For details, please see the docs for those functions.
*/

use crate::args::Krt;
use crate::dhatu_gana as gana;
use crate::it_samjna;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
use crate::sounds::{s, SoundSet};
use crate::tag::Tag as T;
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: SoundSet = s("ac");
    static ref IK: SoundSet = s("ik");
    static ref PU: SoundSet = s("pu~");
    static ref HAL: SoundSet = s("hal");
}

impl Krt {
    fn to_term(self) -> Term {
        use Krt as K;
        let mut krt = Term::make_upadesha(self.as_str());
        krt.add_tags(&[T::Pratyaya, T::Krt]);

        // Any rule that adds `krtya` also includes the `krtya` samjna by adhikara from 3.1.95.
        // Other samjnas, such as `Nistha`, are added in separate rules and are thus modeled
        // separately.
        if matches!(
            self,
            K::tavyat | K::tavya | K::anIyar | K::yat | K::kyap | K::Ryat
        ) {
            krt.add_tag(T::Krtya);
        }

        krt
    }
}

/// Wrapper for `Prakriya` with the following features:
///
/// - remembers which `krt` pratyaya the caller wishes to add, which simplifies the calling API.
/// - records whether a `krt` pratyaya has been added or not, which simplifies the control flow for
///   optional rules.
struct KrtPrakriya<'a> {
    p: &'a mut Prakriya,
    krt: Krt,
    tried: bool,
    has_krt: bool,
}

impl<'a> KrtPrakriya<'a> {
    /// Creates a new `KrtPrakriya` struct.
    fn new(p: &'a mut Prakriya, krt: Krt) -> Self {
        KrtPrakriya {
            p,
            krt,
            tried: false,
            has_krt: false,
        }
    }

    /// Wraps `Prakriya::get`.
    fn get(&self, i: usize) -> Option<&Term> {
        self.p.get(i)
    }

    fn has_prefix(&self, value: &str) -> bool {
        match self.p.find_last_where(|t| !t.has_tag(T::Dhatu)) {
            Some(i) => self.p.terms()[i].has_text(value),
            None => false,
        }
    }

    fn has_prefixes(&self, values: &[&str; 2]) -> bool {
        match self.p.find_last_where(|t| !t.has_tag(T::Dhatu)) {
            Some(i) => {
                i > 0
                    && self.p.has(i - 1, |t| t.has_text(values[0]))
                    && self.p.has(i, |t| t.has_text(values[1]))
            }
            None => false,
        }
    }

    fn has_prefix_in(&self, values: &[&str]) -> bool {
        match self.p.find_last_where(|t| !t.has_tag(T::Dhatu)) {
            Some(i) => self.p.terms()[i].has_text_in(values),
            None => false,
        }
    }

    /// If there's a match, adds the given `krt` pratyaya.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    fn try_add(&mut self, rule: Rule, krt: Krt) -> bool {
        self.try_add_with(rule, krt, |_p, _i| {})
    }

    /// If there's a match, optionally adds the given `krt` pratyaya.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    fn optional_try_add(&mut self, rule: Rule, krt: Krt) -> bool {
        if krt == self.krt && !self.has_krt {
            if self.p.is_allowed(rule) {
                self.try_add_with(rule, krt, |_p, _i| {});
                return true;
            } else {
                self.p.decline(rule);
            }
        }
        false
    }

    /// If there's a match, adds the given `krt` pratyaya then runs `func`.
    ///
    /// This method does nothing if a krt pratyaya has already been added.
    fn try_add_with(&mut self, rule: Rule, krt: Krt, func: impl Fn(&mut Prakriya, usize)) -> bool {
        self.tried = true;
        if self.krt == krt && !self.has_krt {
            let i = self.p.terms().len();
            self.p.op(rule, |p| {
                p.push(krt.to_term());
                func(p, i);
            });
            it_samjna::run(self.p, i).expect("should never fail");

            self.has_krt = true;
            true
        } else {
            false
        }
    }
}

fn is_nandi_grahi_pacadi(p: &KrtPrakriya, i: usize) -> bool {
    let dhatu = p.get(i).expect("should be present");

    // TODO: add the others.
    // TODO: at this length, maybe a set? sorted vec?
    dhatu.has_text_in(&[
        // pac-Adi
        "pac", "vac", "vap", "vad", "cal", "tap", "pat", "nadaw", "Bazaw", "vas", "garat", "plavaw",
        "cIraw", "mAhaw", "jara", "mara", "kzara", "kzama", "sUdaw", "devaw", "moraw", "seva",
        "meza", "kroDa", "vraRa", "daMSa", "daSa", "damBa", "jAraBara", "Svapaca", "meGa", "kIza",
        "kzapa", "mada", "raja", "dIzaw", "caraw",
    ])
}

/// Runs rules that try to add `krt` in one of three specific senses. (3.2.134 - 3.2.179)
///
/// The three senses of these pratyayas are:
/// - tacchIla (nature)
/// - taddharma (duty)
/// - tatsAdhukAra (ability)
///
///
/// ### Control flow
///
/// In this section, we ignore rule 3.1.94, which allows multiple pratyayas to optionally apply:
///
/// > ṇvulaiva siddhe vuñvidhānaṃ jñāpanārthaṃ, tācchīlikeṣu vā'sarūpanyāyena tṛjādayo na bhavantīti
/// > -- Kashika vrtti on 3.2.146
///
/// > tacchīlādiṣu vā'sarūpavidhirnāstīti। tenālaṅkārakaḥ, nirākartetyādīnāṃ tacchīlādiṣu sādhutvaṃ
/// > na bhavati॥
/// > -- Nyasa on 3.2.146
///
/// Therefore, the pratyayas in this section are blocking: for a given dhatu, exactly one of these
/// pratyayas can be used.
///
/// That said, there are some minor exceptions here and there where multiple pratyayas can apply.
fn try_add_krt_for_tacchila_etc(p: &mut KrtPrakriya, i: usize, krt: Krt) -> Option<()> {
    use Krt as K;
    let dhatu = p.get(i)?;

    let prev = if i > 0 { p.get(i - 1) } else { None };
    let upasarge = prev.map_or(false, |t| t.has_tag(T::Upasarga));

    let has_prefix_and_text = |x, y| p.has_prefix(x) && dhatu.has_text(y);
    let has_nic = p.p.terms().last()?.has_u("Ric");
    let has_yan = p.p.terms().last()?.has_u("yaN");
    let has_san = p.p.terms().last()?.has_u("san");

    if has_prefix_and_text("alam", "kf")
        || (p.has_prefixes(&["nis", "A"]) && dhatu.has_text("kf"))
        || has_prefix_and_text("pra", "jan")
        || has_prefix_and_text("ud", "pac")
        || has_prefix_and_text("ud", "pat")
        || has_prefix_and_text("ud", "mad")
        || dhatu.has_text("ruc")
        || has_prefix_and_text("apa", "trap")
        || dhatu.has_text_in(&["vft", "vfD", "sah", "car"])
    {
        p.try_add("3.2.136", K::izRuc);
    } else if dhatu.has_text_in(&["glE", "ji", "sTA", "BU"]) {
        p.try_add("3.2.139", K::ksnu);
    } else if dhatu.has_text_in(&["tras", "gfD", "Dfz", "kzip"]) {
        p.try_add("3.2.140", krt);
    } else if dhatu.has_u_in(gana::SHAM_ADI) {
        p.try_add("3.2.141", K::GinuR);
    } else if has_prefix_and_text("sam", "pfc")
        || has_prefix_and_text("anu", "ruD")
        || has_prefix_and_text("A", "yam")
        || has_prefix_and_text("A", "yas")
        || has_prefix_and_text("pari", "sf")
        || has_prefix_and_text("sam", "sfj")
        || has_prefix_and_text("pari", "dev")
        || has_prefix_and_text("sam", "jvar")
        || has_prefix_and_text("pari", "kzip")
        || has_prefix_and_text("pari", "raw")
        || has_prefix_and_text("pari", "vad")
        || has_prefix_and_text("pari", "dah")
        || has_prefix_and_text("pari", "muh")
        || dhatu.has_text_in(&["duz", "dviz", "druh", "duh", "yuj"])
        || has_prefix_and_text("A", "krIq")
        || has_prefix_and_text("vi", "vij")
        || dhatu.has_text_in(&["tyaj", "raj", "Baj"])
        || has_prefix_and_text("ati", "car")
        || has_prefix_and_text("apa", "car")
        || has_prefix_and_text("A", "muz")
        || (p.has_prefixes(&["aBi", "A"]) && dhatu.has_text("han"))
    {
        p.try_add("3.2.142", K::GinuR);
    } else if p.has_prefix("vi") && dhatu.has_text_in(&["kaz", "las", "katT", "sranB"]) {
        p.try_add("3.2.143", K::GinuR);
    } else if p.has_prefix_in(&["vi", "apa"]) && dhatu.has_text("laz") {
        p.try_add("3.2.144", K::GinuR);
    } else if p.has_prefix("pra") && dhatu.has_text_in(&["lap", "sf", "dru", "maT", "vad", "vas"]) {
        p.try_add("3.2.145", K::GinuR);
    }

    // Break the `if` chain so that pari-kzip and pari-raw can match again here.
    let dhatu = p.get(i)?;
    let has_prefix_and_text = |x, y| p.has_prefix(x) && dhatu.has_text(y);
    if dhatu.has_text_in(&["nind", "hins", "kliS", "KAd"])
        || (has_prefix_and_text("vi", "naS") && has_nic)
        || has_prefix_and_text("pari", "kzip")
        || has_prefix_and_text("pari", "raw")
        || (has_prefix_and_text("pari", "vad") && has_nic)
        || (p.has_prefixes(&["vi", "A"]) && dhatu.has_text("BAz"))
        // TODO: not in dhatupatha -- how does the prakriya start?
        || dhatu.has_text("asUy")
    {
        p.try_add("3.2.146", K::vuY);
    } else if upasarge && dhatu.has_text_in(&["div", "kruS"]) {
        p.try_add("3.2.147", K::vuY);
    } else if dhatu.has_text_in(&[
        "laz", "pat", "pad", "sTA", "BU", "vfz", "han", "kam", "gam", "SF",
    ]) {
        p.try_add("3.2.154", K::ukaY);
    } else if dhatu.has_text_in(&["jalp", "Bikz", "kuww", "lunw", "vf"]) {
        p.try_add("3.2.154", K::zAkan);
    } else if dhatu.has_text_in(&["sf", "Gas", "ad"]) {
        p.try_add("3.2.160", K::kmarac);
    } else if dhatu.has_text_in(&["Banj", "BAs", "mid"]) {
        p.try_add("3.2.161", K::Gurac);
    } else if (dhatu.has_u("vida~") && dhatu.has_gana(2)) || dhatu.has_text_in(&["Bid", "Cid"]) {
        // Per commentaries, allow only this specific `vid`.
        p.try_add("3.2.162", K::kurac);
    } else if dhatu.has_u("i\\R") || dhatu.has_text_in(&["naS", "ji", "sf"]) {
        p.try_add("3.2.163", krt);
    } else if dhatu.has_text("jAgf") {
        p.try_add("3.2.165", K::Uka);
    } else if dhatu.has_text_in(&["yaj", "jap", "daS"]) && has_yan {
        p.try_add("3.2.166", K::Uka);
    } else if dhatu.has_text_in(&["nam", "kamp", "smi", "jas", "kam", "hins", "dIp"]) {
        p.try_add("3.2.167", K::ra);
    } else if has_san || has_prefix_and_text("A", "Sans") || dhatu.has_text("Bikz") {
        p.try_add("3.2.168", K::u);
    } else if dhatu.has_text_in(&["svap", "tfz"]) {
        p.try_add("3.2.172", K::najiN);
    } else if dhatu.has_text_in(&["Sf", "vand"]) {
        p.try_add("3.2.173", K::Aru);
    } else if dhatu.has_text("BI") {
        if krt == K::kruka {
            p.try_add("3.2.174.v1", K::kruka);
        } else if krt == K::kru {
            p.try_add("3.2.174", K::kru);
        } else {
            p.try_add("3.2.174", K::klukan);
        }
    } else if dhatu.has_text_in(&["sTA", "IS", "BAs", "pis", "kas"]) {
        p.try_add("3.2.175", K::varac);
    } else if dhatu.has_text("yA") && has_yan {
        p.try_add("3.2.176", K::varac);
    } else if dhatu.has_text_in(&["BrAj", "BAs", "Durv", "dyut", "Urj", "pF", "ju"]) {
        // TODO: grAva-stut
        p.try_add("3.2.177", K::kvip);
    } else if dhatu.has_text_in(&["yuj", "Cid", "Bid"]) {
        // anyebhyo 'pi dRzyate -- so, include what the commentators mention.
        p.try_add("3.2.178", K::kvip);
    }

    if !p.tried {
        p.try_add("3.2.135", K::tfn);
    }

    Some(())
}

/// Runs rules that try to add the given `krt` suffix to the dhatu. (3.1.91 - 3.4.76)
///
///
/// ### API
///
/// This function receives a `Prakriya` and the `krt` suffix the caller wishes to add. It returns
/// one of the following vowels:
///
/// - `Some(true)`, which denotes that `krt` was successfully added to the prakriya.
/// - `Some(false)`, which denotes that there is no rule that can add `krt` to the prakriya.
/// - `None`, which denotes that the prakriya is ineligible to receive a krt pratyaya, e.g. if the
///    prakriya does not contain a dhatu.
///
///
/// ### Control flow
///
/// Most of the rules here are constrained by the following paribhasha:
///
/// > 3.1.94 vā́sarūpo'striyām
///
/// In plain English: the pratyayas introduced from 3.1.95 up to 3.3.94 (striyām ktiṅ) block each
/// other if they have the same form (sarūpa). Otherwise, there is no blocking and each rule is
/// optional (vā).
///
/// For example, if one rule adds *ghañ* and another adds *ap*, both pratyayas have the same form
/// ("a") if their it letters are removed, and therefore the *ap* rule will block the *ghañ* rule.
///
/// For details, see: <https://ashtadhyayi.com/sutraani/3/1/94>
fn try_add_krt(p: &mut Prakriya, krt: Krt) -> Option<bool> {
    use Krt as K;

    let i = p.find_last(T::Dhatu)?;
    let prev = if i > 0 { p.get(i - 1) } else { None };

    // Pre-calculate some common properties.
    let upasarge = prev.map_or(false, |t| t.has_tag(T::Upasarga));
    let supi = prev.map_or(false, |t| t.has_tag(T::Sup));

    // For convenience below, wrap `Prakriya` in a new `KrtPrakriya` type that contains `krt` and
    // records whether or not any of these rules were applied.
    let mut p = KrtPrakriya::new(p, krt);
    let dhatu = p.get(i)?;

    match krt {
        // ------------------------------------------
        // krtyAH
        // ------------------------------------------
        K::tavyat | K::tavya | K::anIyar => {
            p.try_add("3.1.96", krt);
        }

        // "ya" (3.1.97 - 3.1.132)
        K::yat | K::kyap | K::Ryat => {
            // TODO: nipatana:
            // - 3.1.101 - 3.1.105
            // - 3.1.114 - 3.1.119
            // - 3.1.121 - 3.1.123
            // - 3.1.127 - 3.1.132
            let vapi_rapi = &["vap", "rap", "lap", "trap", "cam"];

            // If true, skip 3.1.110.
            let mut skip_3_1_110 = false;

            // Specific rules (optional)
            let dhatu = p.get(i)?;
            if dhatu.has_u("quBf\\Y") {
                p.optional_try_add("3.1.112", K::kyap);
            } else if dhatu.has_u_in(&["qukf\\Y", "vfzu~"]) {
                if dhatu.has_text("kf") {
                    p.optional_try_add("3.1.120", K::kyap);
                } else {
                    // This rule makes rule 3.1.110 optional for vfz.
                    skip_3_1_110 = p.p.op_optional("3.1.120", |_| {});
                }
            } else if dhatu.has_text("mfj") {
                // This rule makes rule 3.1.110 optional for mfj.
                skip_3_1_110 = p.p.op_optional("3.1.113", |_| {});
            }

            // Specific rules (required)
            let dhatu = p.get(i)?;
            if dhatu.has_u_in(&["Sa\\kx~", "zaha~\\"]) {
                p.try_add("3.1.99", K::yat);
            } else if dhatu.has_u_in(&["gada~", "madI~", "cara~", "ya\\ma~"]) && !upasarge {
                p.try_add("3.1.100", K::yat);
            } else if !upasarge && dhatu.has_text("vad") && supi {
                p.try_add("3.1.106", K::yat);
                p.try_add("3.1.106", K::kyap);
            } else if !upasarge && supi && dhatu.has_u("BU") {
                // The condition here is bhAve, but per the kashika, it is always bhAve per 3.4.70,
                // so it is effectively nitya.
                p.try_add("3.1.107", K::kyap);
            } else if !upasarge && supi && dhatu.has_u("ha\\na~") {
                p.try_add_with("3.1.108", K::kyap, |p, i| {
                    p.set(i, |t| t.set_antya("t"));
                });
            } else if dhatu.has_u("i\\R") || dhatu.has_text_in(&["stu", "SAs", "vf", "df", "juz"]) {
                p.try_add("3.1.109", K::kyap);
            } else if dhatu.has_upadha('f') && !dhatu.has_text_in(&["kfp", "cft"]) && !skip_3_1_110
            {
                p.try_add("3.1.110", K::kyap);
            } else if dhatu.has_text("Kan") {
                p.try_add_with("3.1.111", K::kyap, |p, i| {
                    p.set(i, |t| t.set_antya("I"));
                });
            } else if (p.has_prefix("A") && dhatu.has_text("su")) || dhatu.has_text_in(vapi_rapi) {
                p.try_add("3.1.126", K::Ryat);
            }

            // General rules (optional)
            let dhatu = p.get(i)?;
            if dhatu.has_antya('u') || dhatu.has_antya('U') {
                p.optional_try_add("3.1.125", K::Ryat);
            }

            // General rules (obligatory)
            let dhatu = p.get(i)?;
            if !p.tried {
                if dhatu.has_upadha('a') && dhatu.has_antya(&*PU) {
                    // Sapya, laBya, japya
                    p.try_add("3.1.98", K::yat);
                } else if dhatu.has_antya('f') || dhatu.has_antya('F') || dhatu.has_antya(&*HAL) {
                    p.try_add("3.1.124", K::Ryat);
                } else if dhatu.has_antya(&*AC) {
                    p.try_add("3.1.97", K::yat);
                }
            }
        }

        K::Rvul | K::tfc => {
            p.try_add("3.1.133", krt);
        }

        K::lyu | K::Rini => {
            if is_nandi_grahi_pacadi(&p, i) {
                p.try_add("3.1.134", krt);
            }
        }

        // "a" (3.1.134 - ???)
        K::ac | K::Sa | K::ka | K::Ra => {
            // TODO: 3.1.138 - 3.1.144
            let pa_ghra = &["pA\\", "GrA\\", "DmA\\", "De\\w", "df\\Si~r"];

            if dhatu.has_u_in(pa_ghra) && dhatu.has_gana(1) {
                // These are all bhvAdi dhAtus, so enforce `has_gana(1)` to avoid other dhatus.
                p.try_add("3.1.137", K::Sa);
            } else if dhatu.has_upadha(&*IK) || dhatu.has_u_in(&["JYA\\", "prI\\Y", "kF"]) {
                // vikzipa, viliKa, buDa
                p.try_add("3.1.135", K::ka);
            } else if upasarge && dhatu.has_antya('A') {
                p.try_add("3.1.136", K::ka);
            } else if is_nandi_grahi_pacadi(&p, i) {
                p.try_add("3.1.134", K::ac);
            }
        }

        K::zvun | K::vun => {
            if dhatu.has_text_in(&["nft", "Kan", "ranj"]) {
                p.try_add("3.1.145", K::zvun);
            } else if dhatu.has_text_in(&["pru", "sf", "lU"]) {
                p.try_add("3.1.149", K::vun);
            }

            // Allowed for all dhatus in the sense of ASIH.
            p.try_add("3.1.150", K::vun);
        }

        K::Takan => {
            if dhatu.has_text("gE") {
                p.try_add("3.1.146", krt);
            }
        }

        K::Ryuw => {
            if dhatu.has_text("gE") {
                p.try_add("3.1.146", krt);
            } else if dhatu.has_text("hA") {
                p.try_add("3.1.148", krt);
            }
        }

        K::kta | K::ktavatu => {
            if dhatu.has_tag(T::YIt) {
                p.try_add("3.2.187", Krt::kta);
            }
            p.try_add("3.2.102", krt);

            if p.has_krt {
                p.p.op_term("1.1.26", i + 1, op::add_tag(T::Nistha));
            }
        }

        K::Nvanip => {
            if dhatu.has_u("zu\\Y") || dhatu.has_text("yaj") {
                p.try_add("3.2.103", krt);
            }
        }

        K::atfn => {
            if dhatu.has_text("jF") {
                p.try_add("3.2.104", krt);
            }
        }

        K::kvasu => {
            // Although the rule states "chandasi," kvasu~ has wide application:
            //
            // > kavayastu bahulaṃ prayuñjate । taṃ tasthivāṃsaṃ nagaropakaṇṭhe iti । śreyāṃsi
            // > sarvāṇyadhijagmuṣaste ityādi ॥
            // -- Siddhanta Kaumudi on 3.2.107.
            //
            if dhatu.has_text_in(&["sad", "vas", "Sru"]) {
                p.try_add("3.2.108", krt);
            } else {
                p.try_add("3.2.107", krt);
            }
        }

        // 3.2.111 - 3.2.123 are various lakaras.
        // TODO: 3.3.130 - 3.2.133
        K::Satf | K::SAnac => {
            // 3.2.125 and 3.2.126 define other semantics conditions for Satf and SAnac.
            if p.try_add("3.2.124", krt) {
                p.p.op_term("3.2.127", i + 1, op::add_tag(T::Sat));
            }
        }

        // These are treated separately from SAnac because they are not `Lat` replacements.
        K::SAnan | K::cAnaS => {
            if dhatu.has_text_in(&["pU", "yaj"]) {
                p.try_add("3.2.128", K::SAnan);
            } else {
                p.try_add("3.2.129", K::cAnaS);
            }
        }

        // Rules 3.2.134 - 3.2.179 have a separate control flow and are defined in
        // `try_add_krt_for_tacchila_etc`.

        // The normal control flow resumes from 3.2.180 onward.
        K::qu => {
            if p.has_prefix_in(&["vi", "pra", "sam"]) && dhatu.has_text("BU") {
                p.try_add("3.2.180", krt);
            }
        }

        K::zwran => {
            let ni_shasa = &[
                "nI", "Sas", "yu", "yuj", "stu", "tud", "si", "sic", "mih", "pat", "danS", "nah",
            ];
            if dhatu.has_text("DA") {
                p.try_add("3.2.181", krt);
            } else if dhatu.has_u("dA\\p") || dhatu.has_text_in(ni_shasa) {
                p.try_add("3.2.182", krt);
            } else if dhatu.has_text("pU") {
                p.try_add("3.2.183", krt);
            }
        }

        K::itra => {
            if dhatu.has_text_in(&["f", "lU", "DU", "sU", "Kan", "sah", "car"]) {
                p.try_add("3.2.184", krt);
            } else if dhatu.has_text("pU") {
                // Also allowed by 3.2.186.
                p.try_add("3.2.185", krt);
            }
        }

        K::tumun => {
            p.try_add("3.3.10", krt);
        }

        K::ktri => {}

        K::Tuc => {}

        K::naN => {
            if dhatu.has_text_in(&["yaj", "yAc", "yat", "viC", "praC", "rakz"]) {
                p.try_add("3.3.90", krt);
            }
        }

        K::nan => {
            if dhatu.has_text("svap") {
                p.try_add("3.3.91", krt);
            }
        }

        // ------------------------------------------
        // striyAm
        // ------------------------------------------
        K::ktin => {
            // TODO: striyam
            p.try_add("3.3.94", krt);
        }

        K::Rvuc => {}

        K::ani => {}

        K::lyuw => {
            p.try_add("3.3.115", krt);
        }

        K::Ga => {}

        K::Kal => {}

        K::ktic => {}

        K::GaY => {
            // TODO: move with other a-pratyayas?
            let dhatu = p.get(i)?;
            if dhatu.has_text_in(&["pad", "ruj", "viS"]) {
                p.try_add("3.3.16", K::GaY);
            } else if dhatu.has_text("sf") {
                p.try_add("3.3.17", K::GaY);
            } else {
                p.try_add("3.3.18", K::GaY);
            }
        }
        K::ktvA => {
            p.try_add("3.4.21", krt);
        }
        _ => {
            try_add_krt_for_tacchila_etc(&mut p, i, krt);
        }
    }

    Some(p.has_krt)
}

#[allow(unused)]
pub fn run(p: &mut Prakriya, krt: Krt) {
    try_add_krt(p, krt);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::args::Dhatu;
    use crate::dhatu_karya;

    fn make_prakriya(dhatu: &str) -> Prakriya {
        let dhatu = Dhatu::new(dhatu, 1);
        let mut p = Prakriya::new();
        dhatu_karya::run(&mut p, &dhatu).unwrap();

        p
    }

    fn allows(dhatu: &str, krt: Krt) -> bool {
        let mut p = make_prakriya(dhatu);
        run(&mut p, krt);
        p.terms().last().unwrap().has_u(krt.as_str())
    }

    #[test]
    fn test_common_pratyayas() {
        use Krt::*;

        assert!(allows("BU", tavyat));
        assert!(allows("BU", tavya));
        assert!(allows("BU", anIyar));
        assert!(allows("BU", Rvul));
        assert!(allows("BU", tfc));
        assert!(allows("BU", kta));
        assert!(allows("BU", ktavatu));
        assert!(allows("BU", ktvA));
        assert!(allows("BU", Satf));
        assert!(allows("BU", lyuw));
        assert!(allows("BU", tumun));

        assert!(allows("BU", ktin));

        assert!(allows("BU", GaY));
    }

    #[test]
    fn test_ya_pratyaya() {
        use Krt::*;

        assert!(allows("BU", yat));
        // Allowed by "or AvAzyake"
        assert!(allows("BU", Ryat));

        assert!(allows("gada~", yat));
        assert!(!allows("gada~", Ryat));

        assert!(allows("Sa\\kx~", yat));
        assert!(!allows("Sa\\kx~", Ryat));

        assert!(allows("a\\da~", Ryat));
        assert!(!allows("a\\da~", yat));
    }

    #[test]
    fn test_tacchila() {
        use Krt::*;

        // 3.2.161
        assert!(allows("sf\\", kmarac));
        assert!(allows("Gasx~", kmarac));
        assert!(allows("a\\da~", kmarac));
        assert!(!allows("sf\\", tfn));

        // 3.2.162
        assert!(allows("Ba\\njo~", Gurac));
        assert!(allows("BAsf~\\", Gurac));
        assert!(allows("YimidA~\\", Gurac));
        assert!(!allows("Ba\\njo~", tfn));

        // 3.2.173
        assert!(allows("Sf", Aru));
        assert!(allows("vadi~\\", Aru));
        assert!(!allows("Sf", tfn));

        // 3.2.174
        assert!(allows("YiBI\\", kru));
        assert!(allows("YiBI\\", kruka));
        assert!(allows("YiBI\\", klukan));
        assert!(!allows("YiBI\\", tfn));
    }
}
