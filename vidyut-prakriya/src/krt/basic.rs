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
*/

use crate::args::Gana;
use crate::args::Krt;
use crate::args::KrtArtha::*;
use crate::args::Taddhita;
use crate::dhatu_gana as gana;
use crate::it_samjna;
use crate::krt::utils::KrtPrakriya;
use crate::operators as op;
use crate::prakriya::{Prakriya, Rule};
use crate::sounds::{s, Set};
use crate::stem_gana::TYAD_ADI;
use crate::tag::Tag as T;
use crate::term::Term;
use lazy_static::lazy_static;

lazy_static! {
    static ref AC: Set = s("ac");
    static ref II: Set = s("i");
    static ref IK: Set = s("ik");
    static ref UU: Set = s("u");
    static ref PU: Set = s("pu~");
    static ref HAL: Set = s("hal");
    static ref EMPTY_TERM: Term = Term::make_text("");
}

/// Tries to add various pratyayas that are just "a."
fn try_add_various_pratyayas(kp: &mut KrtPrakriya) {
    use Krt::*;

    let upapada = kp.upapada();
    let upasarge = upapada.map(|t| t.is_upasarga()).unwrap_or(false);

    // "Kartari" comes from 3.4.67 and applies to all krt-pratyayas if not otherwise specified.
    kp.with_context(Karta, |_| {});

    // In this section, we ignore rule 3.1.94, which allows multiple pratyayas to optionally apply:
    //
    // > ṇvulaiva siddhe vuñvidhānaṃ jñāpanārthaṃ, tācchīlikeṣu vā'sarūpanyāyena tṛjādayo na bhavantīti
    // > -- Kashika vrtti on 3.2.146
    //
    // > tacchīlādiṣu vā'sarūpavidhirnāstīti। tenālaṅkārakaḥ, nirākartetyādīnāṃ tacchīlādiṣu sādhutvaṃ
    // > na bhavati॥
    // > -- Nyasa on 3.2.146
    //
    // Therefore, the pratyayas in this section are blocking: for a given dhatu, exactly one of these
    // pratyayas can be used.
    //
    // That said, there are some minor exceptions here and there where multiple pratyayas can apply.
    kp.with_context(TacchilaTaddharmaTatsadhukara, |kp| {
        let dhatu = kp.dhatu();
        let i_dhatu = kp.i_dhatu;

        let has_prefix_and_text = |x, y| kp.has_upapada(x) && dhatu.has_text(y);
        let last = kp.p.terms().last().expect("present");
        let has_nic = last.has_u("Ric");
        let has_yan = last.has_u("yaN");
        let has_san = last.has_u("san");

        if has_prefix_and_text("alam", "kf")
            || (kp.has_prefixes(&["nis", "A"]) && dhatu.has_text("kf"))
            || has_prefix_and_text("pra", "jan")
            || has_prefix_and_text("ud", "pac")
            || has_prefix_and_text("ud", "pat")
            || has_prefix_and_text("ud", "mad")
            || dhatu.has_text("ruc")
            || has_prefix_and_text("apa", "trap")
            || dhatu.has_text_in(&["vft", "vfD", "sah", "car"])
        {
            kp.try_add("3.2.136", izRuc);
        } else if dhatu.has_text_in(&["glE", "ji", "sTA", "BU"]) {
            kp.try_add("3.2.139", ksnu);
        } else if dhatu.has_text_in(&["tras", "gfD", "Dfz", "kzip"]) {
            kp.try_add("3.2.140", knu);
        } else if dhatu.has_u_in(gana::SHAM_ADI) {
            kp.try_add("3.2.141", GinuR);
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
            || dhatu.has_text_in(&["tyaj", "ranj", "Baj"])
            || has_prefix_and_text("ati", "car")
            || has_prefix_and_text("apa", "car")
            || has_prefix_and_text("A", "muz")
            || (kp.has_prefixes(&["aBi", "A"]) && dhatu.has_text("han"))
        {
            kp.try_add("3.2.142", GinuR);
        } else if kp.has_upapada("vi") && dhatu.has_text_in(&["kaz", "las", "katT", "sranB"]) {
            kp.try_add("3.2.143", GinuR);
        } else if kp.has_upapada_in(&["vi", "apa"]) && dhatu.has_text("laz") {
            kp.try_add("3.2.144", GinuR);
        } else if kp.has_upapada("pra")
            && dhatu.has_u_in(&["lapa~", "sf\\", "dru\\", "maTe~", "vada~", "va\\sa~"])
        {
            kp.try_add("3.2.145", GinuR);
        }

        // Break the `if` chain so that pari-kzip and pari-raw can match again here.
        let dhatu = kp.dhatu();
        let has_prefix_and_text = |x, y| kp.has_upapada(x) && dhatu.has_text(y);
        if dhatu.has_text_in(&["nind", "hins", "kliS", "KAd"])
        || (has_prefix_and_text("vi", "naS") && has_nic)
        || has_prefix_and_text("pari", "kzip")
        || has_prefix_and_text("pari", "raw")
        || (has_prefix_and_text("pari", "vad") && has_nic)
        || (kp.has_prefixes(&["vi", "A"]) && dhatu.has_text("BAz"))
        // TODO: not in dhatupatha -- how does the prakriya start?
        || dhatu.has_text("asUy")
        {
            kp.try_add("3.2.146", vuY);
        } else if upasarge && dhatu.has_text_in(&["div", "kruS"]) {
            kp.try_add("3.2.147", vuY);
        } else if dhatu.has_text_in(&[
            "laz", "pat", "pad", "sTA", "BU", "vfz", "han", "kam", "gam", "SF",
        ]) {
            kp.try_add("3.2.154", ukaY);
        } else if dhatu.has_text_in(&["jalp", "Bikz", "kuww", "lunw", "vf"]) {
            kp.try_add("3.2.154", zAkan);
        } else if dhatu.has_u_in(&["spfha", "gfha", "pata", "daya~\\"]) {
            // Per kashika, first 3 are curAdi.
            kp.try_add("3.2.158", Aluc);
            // TODO: others
        } else if dhatu.has_text_in(&["sf", "Gas", "ad"]) {
            kp.try_add("3.2.160", kmarac);
        } else if dhatu.has_text_in(&["Banj", "BAs", "mid"]) {
            kp.try_add("3.2.161", Gurac);
        } else if (dhatu.has_u("vida~") && dhatu.has_gana(Gana::Adadi))
            || dhatu.has_text_in(&["Bid", "Cid"])
        {
            // Per commentaries, allow only this specific `vid`.
            kp.try_add("3.2.162", kurac);
        } else if dhatu.has_u("i\\R") || dhatu.has_text_in(&["naS", "ji", "sf"]) {
            kp.try_add("3.2.163", kvarap);
        } else if dhatu.has_text("jAgf") {
            kp.try_add("3.2.165", Uka);
        } else if dhatu.has_text_in(&["yaj", "jap", "daS"]) && has_yan {
            kp.try_add("3.2.166", Uka);
        } else if dhatu.has_text_in(&["nam", "kanp", "smi", "jas", "kam", "hins", "dIp"]) {
            kp.try_add("3.2.167", ra);
        } else if has_san || has_prefix_and_text("A", "Sans") || dhatu.has_text("Bikz") {
            kp.try_add("3.2.168", u);
        } else if dhatu.has_text_in(&["svap", "tfz"]) {
            kp.try_add("3.2.172", najiN);
        } else if dhatu.has_text_in(&["Sf", "vand"]) {
            kp.try_add("3.2.173", Aru);
        } else if dhatu.has_text("BI") {
            if kp.krt == kruka {
                kp.try_add("3.2.174.v1", kruka);
            } else if kp.krt == kru {
                kp.try_add("3.2.174", kru);
            } else {
                kp.try_add("3.2.174", klukan);
            }
        } else if dhatu.has_text_in(&["sTA", "IS", "BAs", "pis", "kas"]) {
            kp.try_add("3.2.175", varac);
        } else if i_dhatu > 0 && kp.p.has(i_dhatu - 1, |t| t.has_text("yA")) && has_yan {
            // yAyAvara
            kp.try_add("3.2.176", varac);
        } else if dhatu.has_text_in(&["BrAj", "BAs", "Durv", "dyut", "Urj", "pF", "ju"]) {
            // TODO: grAva-stut
            kp.try_add("3.2.177", kvip);
        } else if dhatu.has_text_in(&["yuj", "Cid", "Bid"]) {
            // anyebhyo 'pi dRzyate -- so, include what the commentators mention.
            kp.try_add("3.2.178", kvip);
        }

        if !kp.had_match {
            kp.try_add("3.2.135", tfn);
        }
    });

    kp.with_context(Bhava, |kp| {
        let dhatu = kp.dhatu();
        let i_dhatu = kp.i_dhatu;
        if dhatu.has_text_in(&["pad", "ruj", "viS", "spfS"]) {
            // pAda, roga, veSa, sparSa
            kp.try_add("3.3.16", GaY);
        } else if !upasarge && dhatu.has_text("sf") {
            // sAra
            kp.try_add("3.3.17", GaY);
        } else if dhatu.has_u("i\\N") {
            // aDyAya
            kp.try_add("3.3.21", GaY);
        } else if kp.has_upapada("AN") && dhatu.has_u_in(&["ru", "plu\\N"]) {
            // ArAva, Arava, AplAva, Aplava
            // (block 3.3.22)
            kp.optional_try_add("3.3.50", GaY);
        } else if upasarge && dhatu.has_u("ru") {
            // saMrAva, uparAva
            kp.try_add("3.3.22", GaY);
        } else if kp.has_upapada("sam") && dhatu.has_u_in(&["yu", "dru\\", "du\\"]) {
            // samyAva, sandrAva, sandAva
            kp.try_add("3.3.23", GaY);
        } else if !upasarge && dhatu.has_u_in(&["SriY", "RI\\Y", "BU"]) {
            // SRAya, nAya, BAva
            kp.try_add("3.3.24", GaY);
        } else if kp.has_upapada("vi") && dhatu.has_u_in(&["wukzu", "Sru\\"]) {
            // vikzAva, viSrAva
            kp.try_add("3.3.25", GaY);
        } else if kp.has_upapada_in(&["ava", "ud"]) && dhatu.has_u("RI\\Y") {
            // avanAya, unnAya
            kp.try_add("3.3.26", GaY);
        } else if kp.has_upapada("pra") && dhatu.has_u_in(&["dru\\", "zwu\\Y", "sru\\"]) {
            // pradrAva, prastAva, prasrAva
            kp.try_add("3.3.27", GaY);
        } else if (kp.has_upapada("nir") && dhatu.has_u_in(&["pUY", "pUN"]))
            || (kp.has_upapada("aBi") && dhatu.has_u("lUY"))
        {
            // nizpAva, aBilAva, ...
            // "pū iti pūṅpūñoḥ sāmānyena grahaṇam" (KV)
            kp.try_add("3.3.28", GaY);
        } else if kp.has_upapada_in(&["ud", "ni"]) && dhatu.has_u("gF") {
            // "gṝ śabde, gṝ nigaraṇe, dvayorapi grahaṇam" (KV)
            kp.try_add("3.3.29", GaY);
        } else if kp.has_upapada_in(&["ud", "ni"])
            && dhatu.has_u("kF")
            && dhatu.has_gana(Gana::Tudadi)
        {
            // utkAra, utkara
            // "vikṣepārthasya kiratergrahaṇaṃ, na hiṃsārthasya" (KV)
            kp.try_artha_add("3.3.30", GaY);
        } else if kp.has_upapada("sam") && dhatu.has_u("zwu\\Y") {
            // saMstAva, saMstava
            kp.try_artha_add("3.3.31", GaY);
        } else if dhatu.has_u("stFY") {
            if kp.has_upapada("pra") {
                // prastAra, prastara
                kp.try_artha_add("3.3.32", GaY);
            } else if kp.has_upapada("vi") {
                // vistAra, vistara
                kp.try_artha_add("3.3.33", GaY);
                // vizwAra
                kp.try_artha_add("3.3.34", GaY);
            }
        } else if dhatu.has_u("graha~^") {
            // Collect all "grah" rules here, mainly for 3.3.45 + 3.3.51.
            if kp.has_upapada("ud") {
                // udgrAha
                kp.try_add("3.3.35", GaY);
            }
            if kp.has_upapada("sam") {
                // saNgrAha
                kp.try_artha_add("3.3.36", GaY);
            }
            if kp.has_upapada_in(&["ava", "ni"]) {
                // avagrAha, nigrAha
                kp.try_artha_add("3.3.45", GaY);
            }
            if kp.has_upapada("pra") {
                // pragrAha
                kp.try_artha_add("3.3.46", GaY);
            }
            if kp.has_upapada("pari") {
                // parigrAha
                kp.try_artha_add("3.3.47", GaY);
            }
        } else if (kp.has_upapada("pari") && dhatu.has_u("RI\\Y"))
            || (kp.has_upapada("ni") && dhatu.has_u("i\\R"))
        {
            // parinAya, nyAya
            kp.try_artha_add("3.3.37", GaY);
        } else if kp.has_upapada("pari") && dhatu.has_u("i\\R") {
            kp.try_artha_add("3.3.38", GaY);
        } else if kp.has_upapada_in(&["vi", "upa"]) && dhatu.has_u("SIN") {
            kp.try_artha_add("3.3.39", GaY);
        } else if dhatu.has_u("ci\\Y") {
            if !kp.try_artha_add("3.3.40", GaY) {
                kp.try_artha_add_with("3.3.41", GaY, |p| p.set(i_dhatu, op::adi("k")));
                kp.try_artha_add_with("3.3.42", GaY, |p| p.set(i_dhatu, op::adi("k")));
            }
        } else if kp.has_upapada("ni") && dhatu.has_u_in(&["vfN", "vfY"]) {
            // nIvAra
            // "vṛ iti vṛṅvṛñoḥ sāmānyena grahaṇam" (KV)
            kp.try_artha_add("3.3.48", GaY);
        } else if kp.has_upapada("ud") && dhatu.has_u_in(&["SriY", "yu", "pUY", "dru\\"]) {
            // ucCrAya, udyAva, utpAva, uddrAva
            kp.try_add("3.3.49", GaY);
        }

        // TODO: vibhasha
        let dhatu = kp.dhatu();
        if kp.had_match {
        } else if dhatu.has_u("graha~^") {
            if kp.has_upapada("ava") {
                kp.try_artha_add("3.3.51", GaY);
            }
            if kp.has_upapada("pra") {
                kp.try_artha_add("3.3.52", GaY);
                kp.try_artha_add("3.3.53", GaY);
            }
        } else if kp.has_upapada("pra") && dhatu.has_u("vfY") {
            // pravAra, pravara
            kp.optional_try_add("3.3.54", GaY);
        } else if kp.has_upapada("pari") && dhatu.has_u("BU") {
            // pariBAva, pariBava
            kp.optional_try_add("3.3.55", GaY);
        }

        let dhatu = kp.dhatu();
        if kp.had_match {
            // Skip.
        } else if dhatu.has_u_in(&["graha~^", "vfY", "df", "ga\\mx~"])
            || (kp.has_upapada_in(&["nir", "nis"]) && dhatu.has_u("ci\\Y"))
        {
            kp.try_add("3.3.58", ap);
        } else if upasarge && dhatu.has_u("a\\da~") {
            if kp.has_upapada("ni") {
                // nyAda, niGasa
                kp.optional_try_add("3.3.60", Ra);
            }
            // viGasa, praGasa
            kp.try_add("3.3.59", ap);
        } else if !upasarge && dhatu.has_u_in(&["vya\\Da~", "japa~"]) {
            // vyaDa, japa
            kp.try_add("3.3.61", ap);
        } else if !upasarge && dhatu.has_u_in(&["svana~", "hase~"]) {
            // svana, svAna, hasa, hAsa
            kp.optional_try_add("3.3.62", ap);
        } else if kp.has_upapada_in(&["sam", "upa", "ni", "vi"]) && dhatu.has_u("ya\\ma~") {
            // saMyAma, saMyama, ...
            kp.optional_try_add("3.3.63", ap);
        } else if kp.has_upapada("ni") && dhatu.has_u_in(&["gada~", "Rada~", "paWa~", "svana~"]) {
            // nigada, nigAda, ...
            kp.optional_try_add("3.3.64", ap);
        } else if dhatu.has_u("kvaRa~") {
            // kvaRa
            kp.optional_try_add("3.3.65", ap);
        } else if dhatu.has_u("paRa~\\") {
            // paRa
            kp.try_artha_add("3.3.66", ap);
        } else if !upasarge && dhatu.has_u("madI~") {
            kp.try_add("3.3.67", ap);
        } else if kp.has_upapada_in(&["sam", "ud"]) && dhatu.has_u("aja~") {
            // samaja, udaja
            kp.try_artha_add("3.3.69", ap);
        } else if kp.has_upapada("upa") && dhatu.has_u("sf\\") {
            // upasara
            // (The KV's examples all use upa-.)
            kp.try_artha_add("3.3.71", ap);
        } else if dhatu.has_u("hve\\Y") {
            if kp.has_upapada_in(&["ni", "aBi", "upa", "vi"]) {
                // nihava, aBihava, upahava, vihava
                kp.try_add_with("3.3.72", ap, |p| p.set(i_dhatu, op::text("hu")));
            } else if kp.has_upapada("AN") {
                // Ahava
                kp.try_artha_add_with("3.3.73", ap, |p| p.set(i_dhatu, op::text("hu")));
            } else if !upasarge {
                // hava
                kp.try_add_with("3.3.75", ap, |p| p.set(i_dhatu, op::text("hu")));
            }
        } else if dhatu.has_u("ha\\na~") {
            if !upasarge {
                kp.try_add_with("3.3.76", ap, |p| p.set(i_dhatu, op::text("vaD")));
            }
        }

        // Base cases
        let dhatu = kp.dhatu();
        if kp.had_match {
        } else if dhatu.has_antya(&*II) {
            // caya, ...
            kp.try_add("3.3.56", ac);
        } else if dhatu.has_antya('F') || dhatu.has_antya(&*UU) {
            // kara, yava, ...
            kp.try_add("3.3.57", ap);
        } else {
            kp.try_add("3.3.18", GaY);
        }
    });

    let i_dhatu = kp.i_dhatu;
    let dhatu = kp.dhatu();
    let is_han = dhatu.has_u("ha\\na~");
    kp.with_context(Murti, |kp| {
        if is_han {
            kp.try_artha_add_with("3.3.77", ap, |p| p.set(i_dhatu, op::text("Gan")));
        }
    });
    kp.with_context(Desha, |kp| {
        if is_han && kp.has_upapada("antar") {
            kp.try_artha_add_with("3.3.78", ap, |p| p.set(i_dhatu, op::text("Gan")));
        }
    });

    kp.with_context(Samjna, |kp| {
        let dhatu = kp.dhatu();
        if kp.has_upapada("ava") && dhatu.has_u_in(&["tF", "stFY"]) {
            kp.try_add("3.3.120", GaY);
        } else if dhatu.has_u("Kanu~^") {
            kp.optional_try_add("3.3.125", Ga);
        }

        // Base case
        let dhatu = kp.dhatu();
        if dhatu.has_antya(&*HAL) {
            kp.try_add("3.3.121", GaY);
        }
    })
}

fn is_nandi_grahi_pacadi(kp: &KrtPrakriya) -> bool {
    let dhatu = kp.dhatu();

    // TODO: add the others.
    const NAND_ADI: &[&str] = &["nand", "jalp", "ram", "dfp"];

    #[allow(unused)]
    const GRAH_ADI: &[[&str; 2]] = &[
        ["", "grah"],
        ["ud", "sah"],
        ["ud", "das"],
        ["ud", "BAs"],
        ["", "sTA"],
        ["", "mantr"],
        ["sam", "mard"],
        ["ni", "rakz"],
        ["ni", "Sru"],
        ["ni", "vas"],
        ["ni", "vap"],
        ["ni", "SA"],
    ];

    const PAC_ADI: &[&str] = &[
        "pac", "vac", "vap", "vad", "cal", "tap", "pat", "nadaw", "Bazaw", "vas", "garat",
        "plavaw", "cIraw", "mAhaw", "jara", "mara", "kzara", "kzama", "sUdaw", "devaw", "moraw",
        "seva", "meza", "kroDa", "vraRa", "daMSa", "daSa", "damBa", "jAraBara", "Svapaca", "meGa",
        "kIza", "kzapa", "mada", "raja", "dIzaw", "caraw",
    ];

    // TODO: add the others.
    // TODO: at this length, maybe a set? sorted vec?
    dhatu.has_text_in(NAND_ADI) || dhatu.has_text_in(PAC_ADI)
}

fn try_add_upapada_krt(p: &mut Prakriya, krt: Krt) -> Option<bool> {
    use Krt::*;

    const DIVA_ADI: &[&str] = &[
        "divA", "viBA", "niSA", "praBA", "BAs", "kAra", "anta", "ananta", "Adi", "bahu", "nAndI",
        "kim", "lipi", "libi", "bali", "Bakti", "kartf", "citra", "kzetra", "jaNGA", "bAhu",
        "ahas", "yat", "tat", "Danus", "arus",
    ];
    const ADHYA_ADI: &[&str] = &[
        "AQya", "suBaga", "sTUla", "palita", "nagna", "anDa", "priya",
    ];

    // For convenience below, wrap `Prakriya` in a new `KrtPrakriya` type that contains `krt` and
    // records whether or not any of these rules were applied.
    let mut kp = KrtPrakriya::new(p, krt);
    try_add_various_pratyayas(&mut kp);

    let i_dhatu = kp.p.find_first_where(|t| t.is_dhatu())?;
    let dhatu = kp.dhatu();

    let upapada = match kp.p.get_if(0, |t| t.has_tag(T::Pratipadika)) {
        Some(t) => t,
        None => &EMPTY_TERM,
    };

    let nau = kp.p.has(i_dhatu + 1, |t| t.has_u("Ric"));
    let upasarge = i_dhatu > 0 && kp.p.has(i_dhatu - 1, |t| t.is_upasarga());

    match krt {
        aR | ka | ac | wa | wak => {
            if upapada.has_text_in(&["kzema", "priya", "madre"]) && dhatu.has_u("qukf\\Y") {
                // Also repeated for khac below.
                kp.try_add("3.2.44", aR);
            } else if dhatu.has_u_in(&["hve\\Y", "ve\\Y", "mA\\N"]) {
                kp.try_add("3.2.2", aR);
            } else if dhatu.has_u("zWA\\") {
                // Also 3.2.77
                kp.try_add("3.2.4", ka);
            } else if dhatu.has_u("KyAY") {
                // gosaNKya
                kp.try_add("3.2.7", ka);
            } else if !upasarge && dhatu.has_u("gE\\") {
                // for pA, see 3.2.8.v1 below.
                kp.try_add("3.2.8", wak);
            } else if upapada.has_text_in(&["surA", "SIDu"])
                && !upasarge
                && dhatu.has_u("pA\\")
                && dhatu.has_gana(Gana::Bhvadi)
            {
                kp.try_add("3.2.8.v1", wak);
            } else if dhatu.has_u("hf\\Y") {
                if kp.has_upapada("AN") {
                    // tAcCIlye
                    kp.optional_try_add("3.2.11", ac);
                } else {
                    // an-udyamane, vayasi
                    kp.optional_try_add("3.2.9", ac);
                }
            } else if dhatu.has_u("graha~^") {
                if upapada.has_text_in(&[
                    "Sakti", "lANgala", "aNkuSa", "yazwi", "tomara", "Gawa", "GaWI", "Danus",
                ]) {
                    kp.try_add("3.2.9.v1", ac);
                } else if upapada.has_text("sUtra") {
                    // dhAri-arthe
                    kp.optional_try_add("3.2.9.v2", ac);
                }
            } else if !upasarge && dhatu.has_antya('A') {
                kp.try_add("3.2.3", ka);
            } else if kp.has_upasarga_dhatu(i_dhatu, "pari", "mfjU~")
                || kp.has_upasarga_dhatu(i_dhatu, "apa", "Ru\\da~^")
            {
                kp.try_add("3.2.5", ka);
            } else if kp.has_upapada("pra") && dhatu.has_u_in(&["qudA\\Y", "jYA\\"]) {
                kp.try_add("3.2.6", ka);
            } else if kp.has_upapada("AN") && dhatu.has_u("hf\\Y") {
                kp.try_add("3.2.11", ac);
            } else if dhatu.has_u("arha~") {
                kp.try_add("3.2.12", ac);
            } else if upapada.has_text("Sam") {
                kp.try_add("3.2.14", ac);
            } else if dhatu.has_u("SIN") {
                kp.try_add("3.2.15", ac);
            } else if dhatu.has_u("cara~") {
                if upapada.has_text_in(&["BikzA", "senA", "AdAya"]) {
                    kp.try_add("3.2.17", wa);
                } else {
                    kp.try_add("3.2.16", wa);
                }
            } else if dhatu.has_u("sf\\") {
                if upapada.has_text_in(&["puraH", "agrataH", "agre"]) {
                    kp.try_add("3.2.18", wa);
                } else if upapada.has_text("pUrva") {
                    kp.try_add("3.2.19", wa);
                }
            } else if dhatu.has_u("qukf\\Y") {
                if upapada.has_text_in(DIVA_ADI) || upapada.has_tag(T::Sankhya) {
                    kp.try_add("3.2.21", wa);
                } else if upapada.has_text("karman") {
                    kp.optional_try_add("3.2.22", wa);
                } else if upapada.has_text_in(&[
                    "Sabda", "Sloka", "kalaha", "gATA", "vEra", "cAwu", "sUtra", "mantra", "pada",
                ]) {
                    kp.p.step("3.2.23");
                } else {
                    kp.try_add("3.2.20", wa);
                }
            } else if dhatu.has_u("ha\\na~") {
                if upapada.has_text_in(&["jAyA", "pati"]) {
                    // Sense is "lakshane".
                    kp.optional_try_add("3.2.52", krt);
                } else if upapada.has_text_in(&["hastin", "kapAwa"]) {
                    // Sense is "shaktau".
                    kp.optional_try_add("3.2.54", krt);
                } else if upapada.has_text_in(&["pARi", "tAqa"]) && krt == ka {
                    let sub = if upapada.has_text("pARi") {
                        "pARiGa"
                    } else {
                        "tAqaGa"
                    };
                    kp.optional_do_nipatana("3.2.55", sub);
                } else {
                    // Sense is "a-manuSya-kartrke".
                    kp.optional_try_add("3.2.53", krt);
                }
            }

            // (base case)
            if !kp.has_krt {
                // kumBakAra, ...
                kp.try_add("3.2.1", aR);
            }
        }

        in_ => {
            if upapada.has_text_in(&["stamba", "Sakft"]) && dhatu.has_u("qukf\\Y") {
                kp.try_add("3.2.24", krt);
            } else if dhatu.has_u("hf\\Y") {
                if upapada.has_text_in(&["dfti", "nATa"]) {
                    kp.try_add("3.2.25", krt);
                }
            } else if (upapada.has_text("Pala") && dhatu.has_u("graha~^"))
                || (upapada.has_text("Atman") && dhatu.has_u("quBf\\Y"))
            {
                let sub = if upapada.has_text("Pala") {
                    "Palegrahi"
                } else {
                    "AtmamBari"
                };
                kp.do_nipatana("3.2.26", sub);
            }
        }
        KaS => {
            let nasika = upapada.has_text("nAsikA");
            let stana = upapada.has_text("stana");
            let dhma = dhatu.has_u("DmA\\");
            let dhe = dhatu.has_u("De\\w");
            if dhatu.has_text("ej") && nau {
                kp.try_add("3.2.28", krt);
            } else if (nasika && (dhma || dhe)) || (stana && dhe) {
                kp.try_add("3.2.29", krt);
            } else if (dhma || dhe) && upapada.has_text_in(&["nAqI", "muzwi"]) {
                kp.try_add("3.2.30", krt);
            } else if upapada.has_text("kUla")
                && kp.has_upapada("ud")
                && dhatu.has_u_in(&["ru\\jo~", "va\\ha~^"])
            {
                kp.try_add("3.2.31", krt);
            } else if upapada.has_text_in(&["vaha", "aBra"]) && dhatu.has_u("li\\ha~^") {
                kp.try_add("3.2.32", krt);
            } else if dhatu.has_u("qupa\\ca~^z") {
                if upapada.has_text_in(&["mita", "naKa"]) {
                    kp.try_add("3.2.34", krt);
                } else {
                    kp.try_add("3.2.33", krt);
                }
            } else if upapada.has_text_in(&["viDu", "arus"]) && dhatu.has_u("tu\\da~^") {
                kp.try_add("3.2.35", krt);
            } else if (upapada.has_text("asUrya") && dhatu.has_u("df\\Si~r"))
                || (upapada.has_text("lalAwa") && dhatu.has_u("ta\\pa~"))
            {
                kp.try_add("3.2.36", krt);
            } else if (upapada.has_text("ugra") && dhatu.has_u("df\\Si~r"))
                || (upapada.has_text("irA") && dhatu.has_u("madI~"))
                || (upapada.has_text("pARi") && dhatu.has_u("DmA\\"))
            {
                let sub = if upapada.has_text("ugra") {
                    "ugrampaSya"
                } else if upapada.has_text("irA") {
                    "irammada"
                } else {
                    "pARinDama"
                };
                kp.do_nipatana("3.2.37", sub);
            } else if dhatu.has_u("ma\\na~\\") {
                kp.try_add("3.2.83", krt);
            }
        }

        Kac => {
            if upapada.has_text_in(&["priya", "vaSa"]) && dhatu.has_u("vada~") {
                kp.try_add("3.2.38", krt);
            } else if upapada.has_text_in(&["dvizat", "para"]) && dhatu.has_text("tAp") && nau {
                kp.try_add("3.2.39", krt);
            } else if upapada.has_text("vAc") && dhatu.has_text("yam") {
                kp.try_add("3.2.40", krt);
            } else if upapada.has_text_in(&["pur", "sarva"])
                && ((dhatu.has_text("dAr") && nau) || dhatu.has_text("sah"))
            {
                kp.try_add("3.2.41", krt);
            } else if upapada.has_text_in(&["sarva", "kUla", "aBra", "karIza"])
                && dhatu.has_text("kaz")
            {
                kp.try_add("3.2.42", krt);
            } else if dhatu.has_u("qukf\\Y") {
                if upapada.has_text_in(&["meGa", "fti", "Baya"]) {
                    kp.try_add("3.2.43", krt);
                } else if upapada.has_text_in(&["kzema", "priya", "madra"]) {
                    kp.try_add("3.2.44", krt);
                }
            } else if upapada.has_text("ASita") && dhatu.has_u("BU") {
                kp.try_add("3.2.45", krt);
            }
        }

        qa => {
            if dhatu.has_u("ga\\mx~") {
                if upapada.has_text_in(&[
                    "anta", "atyanta", "aDvan", "dUra", "pAra", "sarva", "ananta",
                ]) {
                    kp.try_add("3.2.48", krt);
                }
            } else if dhatu.has_u("ha\\na~") {
                if upapada.has_text_in(&["kleSa", "tamas"]) && kp.has_upapada("apa") {
                    kp.try_add("3.2.50", krt);
                } else {
                    kp.try_add("3.2.49", krt);
                }
            } else if dhatu.has_u("janI~\\") {
                // TODO: upapada
                kp.try_add("3.2.97", krt);
            }
        }

        Rini => {
            if upapada.has_text_in(&["kumAra", "Sirza"]) && dhatu.has_u("ha\\na~") {
                kp.try_add("3.2.51", krt);
            } else if upapada.has_text("vrata") {
                kp.try_add("3.2.80", krt);
            } else if dhatu.has_u("ma\\na~\\") {
                kp.try_add("3.2.82", krt);
            } else if dhatu.has_u("ya\\ja~^") {
                kp.try_add("3.2.85", krt);
            } else if dhatu.has_u("ha\\na~") {
                kp.try_add("3.2.86", krt);
            } else {
                kp.try_add("3.2.78", krt);
            }
        }

        Kyun => {
            if upapada.has_text_in(ADHYA_ADI) && dhatu.has_u("qukf\\Y") {
                kp.try_add("3.2.56", krt);
            }
        }

        KizRuc | KukaY => {
            if upapada.has_text_in(ADHYA_ADI) && dhatu.has_u("BU") {
                kp.try_add("3.2.57", krt);
            }
        }

        kvin | kaY => {
            if upapada.has_text_in(TYAD_ADI) && dhatu.has_u("df\\Si~r") {
                kp.try_add("3.2.60", krt);
            } else if krt == kvin {
                if !upapada.has_text("udaka") && dhatu.has_text("spfS") {
                    kp.try_add("3.2.58", kvin);
                } else {
                    let code = "3.2.59";
                    if upapada.has_text("ftu") && dhatu.has_u("ya\\ja~^") {
                        kp.do_nipatana(code, "ftvij");
                    } else if dhatu.has_u("YiDfzA~") {
                        kp.do_nipatana(code, "daDfz");
                    } else if dhatu.has_u("sf\\ja~") {
                        kp.do_nipatana(code, "sraj");
                    } else if dhatu.has_u("di\\Sa~^") {
                        kp.do_nipatana(code, "diS");
                    } else if kp.has_upapada("ud") && dhatu.has_u("zRi\\ha~") {
                        kp.do_nipatana(code, "uzRih");
                    } else if dhatu.has_u("ancu~") {
                        kp.p.run_at(code, i_dhatu, |t| {
                            t.set_text("aYc");
                            t.add_tag(T::Krt);
                        });
                        // HACK: update bookkeeping here.
                        kp.has_krt = true;
                        kp.had_match = true;
                    } else if dhatu.has_u("yu\\ji~^r") {
                        kp.do_nipatana(code, "yuj");
                    } else if dhatu.has_u("krunca~") {
                        kp.do_nipatana(code, "kruYc");
                    }
                }
            }
        }

        viw => {
            if dhatu.has_u("a\\da~") {
                if upapada.has_text("kravya") {
                    kp.try_add("3.2.69", krt);
                } else if !upapada.has_text("anna") {
                    kp.try_add("3.2.68", krt);
                }
            }
        }

        kap => {
            if dhatu.has_u("du\\ha~^") {
                kp.try_add_with("3.2.70", krt, |p| {
                    p.set(i_dhatu, |t| t.set_antya("G"));
                });
            }
        }

        kvip => {
            if dhatu.has_u("ha\\na~") {
                if upapada.has_text_in(&["brahman", "BrURa", "vftra"]) {
                    kp.try_add("3.2.87", krt);
                }
            } else if dhatu.has_u("zWA\\") {
                kp.try_add("3.2.77", krt);
            } else if upapada.has_text("soma") && dhatu.has_u("zu\\Y") {
                kp.try_add("3.2.90", krt);
            } else if upapada.has_text("agni") && dhatu.has_u("ci\\Y") {
                kp.try_add("3.2.91", krt);
            } else {
                kp.try_add("3.2.76", krt);
            }
        }

        ini => {
            if kp.has_upapada("vi") && dhatu.has_u("qukrI\\Y") {
                if upapada.has_text("DAnya") {
                    kp.p.step(Rule::Kashika("3.2.93"));
                } else {
                    kp.try_add("3.2.93", krt);
                }
            }
        }

        kvanip => {
            if dhatu.has_u("df\\Si~r") {
                kp.try_add("3.2.94", krt);
            } else if dhatu.has_u_in(&["qukf\\Y", "yu\\Da~\\"]) {
                if upapada.has_text("rAjan") {
                    kp.try_add("3.2.95", krt);
                } else if upapada.has_text("saha") {
                    kp.try_add("3.2.96", krt);
                }
            }
        }
        _ => {}
    }

    Some(kp.has_krt)
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
    let upasarge = prev.map_or(false, |t| t.is_upasarga());
    let supi = prev.map_or(false, |t| t.has_tag(T::Sup));

    // For convenience below, wrap `Prakriya` in a new `KrtPrakriya` type that contains `krt` and
    // records whether or not any of these rules were applied.
    let mut kp = KrtPrakriya::new(p, krt);
    let dhatu = kp.dhatu();
    let i_dhatu = kp.i_dhatu;

    match krt {
        // ------------------------------------------
        // krtyAH
        // ------------------------------------------
        K::tavyat | K::tavya | K::anIyar => {
            let added = kp.try_add("3.1.96", krt);
            if added && krt == K::tavyat && kp.dhatu().has_u("va\\sa~") {
                // vAstavya
                kp.p.run_optional_at("3.1.96.v1", i + 1, |t| t.add_tag(T::Rit));
            }
        }

        K::kelimar => {
            kp.try_add("3.1.96.v2", krt);
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
            let dhatu = kp.dhatu();
            if dhatu.has_u("fca~") {
                // ṛdupadhādapi ṛcerata eva nipātanāt ṇyat bhavati
                kp.try_add("7.3.66", K::Ryat);
            } else if dhatu.has_text("Bf") {
                kp.optional_try_add("3.1.112", K::kyap);
            } else if dhatu.has_u_in(&["qukf\\Y", "vfzu~"]) {
                if dhatu.has_text("kf") {
                    kp.optional_try_add("3.1.120", K::kyap);
                } else {
                    // This rule makes rule 3.1.110 optional for vfz.
                    skip_3_1_110 = kp.p.run_optional("3.1.120", |_| {});
                }
            } else if dhatu.has_text("mfj") {
                // This rule makes rule 3.1.110 optional for mfj.
                skip_3_1_110 = kp.p.run_optional("3.1.113", |_| {});
            }

            // Specific rules (required)
            let dhatu = kp.dhatu();
            let mut avashyaka_blocked = false;
            if dhatu.has_u_in(&["Sa\\kx~", "zaha~\\"]) {
                kp.try_add("3.1.99", K::yat);
            } else if dhatu.has_u_in(&["gada~", "madI~", "cara~", "ya\\ma~"]) && !upasarge {
                kp.try_add("3.1.100", K::yat);
            } else if dhatu.has_u("cara~") && kp.has_upapada("AN") {
                kp.optional_try_add("3.1.100.v1", K::yat);
            } else if !upasarge && dhatu.has_text("vad") && supi {
                kp.try_add("3.1.106", K::yat);
                kp.try_add("3.1.106", K::kyap);
            } else if !upasarge && supi && dhatu.has_u("BU") {
                // The condition here is bhAve, but per the kashika, it is always bhAve per 3.4.70,
                // so it is effectively nitya.
                kp.try_add("3.1.107", K::kyap);
            } else if !upasarge && supi && dhatu.has_u("ha\\na~") {
                kp.try_add_with("3.1.108", K::kyap, |p| {
                    p.set(i_dhatu, |t| t.set_antya("t"));
                });
            } else if dhatu.has_u_in(&["i\\R", "zwu\\Y", "SAsu~", "vfY", "df", "juzI~\\"]) {
                kp.try_add("3.1.109", K::kyap);
                avashyaka_blocked = true;
            } else if dhatu.has_upadha('f') && !dhatu.has_text_in(&["kfp", "cft"]) && !skip_3_1_110
            {
                kp.try_add("3.1.110", K::kyap);
            } else if dhatu.has_text("Kan") {
                kp.try_add_with("3.1.111", K::kyap, |p| {
                    p.set(i_dhatu, |t| t.set_antya("I"));
                });
            } else if (kp.has_upapada("A") && dhatu.has_text("su")) || dhatu.has_text_in(vapi_rapi)
            {
                kp.try_add("3.1.126", K::Ryat);
            }

            // General rules (optional)
            let dhatu = kp.dhatu();
            if (dhatu.has_antya('u') || dhatu.has_antya('U')) && !avashyaka_blocked {
                kp.optional_try_add("3.1.125", K::Ryat);
            } else if dhatu.has_u("ha\\na~") {
                kp.optional_try_add_with("3.1.97.v2", K::yat, |p| {
                    p.set(i_dhatu, |t| t.set_text("vaD"));
                });
            }

            // General rules (obligatory)
            let dhatu = kp.dhatu_end();
            if !kp.had_match {
                if dhatu.has_upadha('a') && dhatu.has_antya(&*PU) {
                    // Sapya, laBya, japya
                    kp.try_add("3.1.98", K::yat);
                } else if dhatu.has_u_in(&["taka~", "Sasu~\\", "cate~^", "yatI~\\", "janI~\\"]) {
                    kp.try_add("3.1.97.v1", K::yat);
                } else if dhatu.has_antya('f') || dhatu.has_antya('F') || dhatu.has_antya(&*HAL) {
                    kp.try_add("3.1.124", K::Ryat);
                } else if dhatu.has_antya(&*AC) {
                    kp.try_add("3.1.97", K::yat);
                }
            }
        }

        K::Rvul | K::tfc => {
            kp.try_add("3.1.133", krt);
        }

        K::lyu | K::Rini => {
            if is_nandi_grahi_pacadi(&kp) {
                kp.try_add("3.1.134", krt);
            } else if krt == K::Rini {
                // TODO: supi
                kp.try_add("3.2.78", krt);
            }
        }

        // "a" (3.1.134 - ???)
        K::ac | K::Sa | K::ka | K::Ra => {
            // These are all bhvAdi dhAtus, so also check for `Bhvadi` to avoid other dhatus.
            let pa_ghra = &["pA\\", "GrA\\", "DmA\\", "De\\w", "df\\Si~r"];
            let dhatu = kp.dhatu_end();

            if upasarge && dhatu.has_u_in(pa_ghra) && dhatu.has_gana(Gana::Bhvadi) {
                kp.try_add("3.1.137", K::Sa);
            } else if dhatu.has_u_in(&[
                "li\\pa~^", "vi\\dx~^", "pF", "vida~", "cita~", "sAti", "zaha~\\",
            ]) {
                kp.try_add("3.1.138", K::Sa);
            } else if dhatu.has_upadha(&*IK) || dhatu.has_u_in(&["JYA\\", "prI\\Y", "kF"]) {
                // vikzipa, viliKa, buDa
                kp.try_add("3.1.135", K::ka);
            } else if upasarge && dhatu.has_antya('A') {
                kp.try_add("3.1.136", K::ka);
            } else if krt == K::ac {
                // ajvidhiḥ sarvadhātubhyaḥ paṭhyante ca pacādayaḥ। aṇbādhanārtham eva
                // syāt sidhyanti śvapacādayaḥ।
                kp.try_add(Rule::Kashika("3.1.134"), K::ac);
            } else if dhatu.has_u_in(&["wudu\\", "RI\\Y"]) && !upasarge {
                kp.try_add("3.1.142", K::Ra);
            } else if dhatu.has_u("graha~^") {
                kp.optional_try_add("3.1.143", K::aR);
                kp.optional_try_add("3.1.144", K::ka);
            }
        }

        K::zvun | K::vun => {
            if dhatu.has_text_in(&["nft", "Kan", "ranj"]) {
                kp.try_add_with("3.1.145", K::zvun, |p| {
                    if p.has(i_dhatu, |t| t.has_text("ranj")) {
                        // per kashika
                        p.set(i_dhatu, |t| t.set_upadha(""));
                    }
                });
            } else if dhatu.has_text_in(&["pru", "sf", "lU"]) {
                kp.try_add("3.1.149", K::vun);
            }

            // Allowed for all dhatus in the sense of ASIH.
            kp.try_add("3.1.150", K::vun);
        }

        K::Takan => {
            if dhatu.has_text("gE") {
                kp.try_add("3.1.146", krt);
            }
        }

        K::Ryuw => {
            if dhatu.has_text("gE") {
                kp.try_add("3.1.146", krt);
            } else if dhatu.has_text("hA") {
                kp.try_add("3.1.148", krt);
            }
        }

        K::kvip => {
            if dhatu.has_text_in(&[
                "sad", "sU", "dviz", "druh", "duh", "yuj", "vid", "Bid", "Cid", "ji", "nI", "rAj",
            ]) {
                // Exclude some of the dhatus above:
                //
                // > sū iti dviṣā sāhacaryāt sūteḥ ādādikasya grahaṇaṃ, na suvateḥ taudādikasya।
                // > yujiryoge, yuja samādhau, dvayorapi grahaṇam। vida jñāne, vida sattāyām, vida
                // > vicāraṇe, trayāṇāmapi grahaṇam। na lābhārthasya videḥ, akārasya vivakṣatatvāt
                // -- KV
                let skip = (dhatu.has_text("sU") && !dhatu.has_gana(Gana::Adadi))
                    || (dhatu.has_text("vid") && dhatu.has_tag(T::xdit));
                if !skip {
                    kp.try_add("3.2.61", krt);
                }
            } else {
                kp.try_add("3.2.76", krt);
            }
        }

        K::Rvi => {
            if dhatu.has_u("Ba\\ja~^") {
                kp.try_add("3.2.62", krt);
            }
        }

        K::manin | K::kvanip | K::vanip | K::vic => {
            let code = "3.2.75";
            if krt == K::manin && dhatu.has_text("Bas") {
                kp.try_add(code, krt);
            } else if krt == K::vic && dhatu.has_text("riz") {
                kp.try_add(code, krt);
            }
        }

        K::kta | K::ktavatu => {
            if dhatu.has_tag(T::YIt) {
                kp.try_add("3.2.187", Krt::kta);
            }
            kp.try_add("3.2.102", krt);

            if kp.has_krt {
                let i_last = kp.p.terms().len() - 1;
                kp.p.run_at("1.1.26", i_last, op::add_tag(T::Nistha));
            }
        }

        K::Nvanip => {
            if dhatu.has_u("zu\\Y") || dhatu.has_text("yaj") {
                kp.try_add("3.2.103", krt);
            }
        }

        K::atfn => {
            if dhatu.has_text("jF") {
                kp.try_add("3.2.104", krt);
            }
        }

        K::kAnac | K::kvasu => {
            // TODO: does this check make sense for kvasu and kAnac? Or should we relax this check
            // because these are mainly chAndasa? For now, disable it.
            /*
            let has_pada_match = match krt {
                K::kvasu => wrap.p.has_tag(T::Parasmaipada),
                // taṅānāv ātmanepadam (1.4.100)
                K::kAnac => wrap.p.has_tag(T::Atmanepada),
                _ => false,
            };
            */

            // Although the rule has "chandasi" by anuvrtti from 3.2.105, kvasu~ has wide
            // application:
            //
            // > kavayastu bahulaṃ prayuñjate । taṃ tasthivāṃsaṃ nagaropakaṇṭhe iti । śreyāṃsi
            // > sarvāṇyadhijagmuṣaste ityādi ॥
            // -- Siddhanta Kaumudi on 3.2.107.
            if !kp.has_krt {
                let i_la = kp.p.terms().len() - 1;
                if krt == K::kvasu && dhatu.has_text_in(&["sad", "vas", "Sru"]) {
                    kp.try_replace_lakara("3.2.108", i_la, krt);
                } else {
                    kp.try_replace_lakara("3.2.107", i_la, krt);
                }
            }
        }

        // 3.2.111 - 3.2.123 are various lakaras.
        // TODO: 3.3.130 - 3.2.133
        K::Satf | K::SAnac => {
            let has_pada_match = match krt {
                K::Satf => kp.p.has_tag(T::Parasmaipada),
                // taṅānāv ātmanepadam (1.4.100)
                K::SAnac => kp.p.has_tag(T::Atmanepada),
                _ => false,
            };

            /*
            // TODO: not sure if these block 3.2.127 below.
            if krt == K::Satf {
                if dhatu.has_text("dviz") {
                    wrap.try_add("3.2.131", krt);
                } else if dhatu.has_u("zu\\Y") {
                    wrap.try_add("3.2.132", krt);
                } else if dhatu.has_text("arh") {
                    wrap.try_add("3.2.133", krt);
                }
            }
            */

            // 3.2.125 and 3.2.126 define other semantics conditions for Satf and SAnac.
            if has_pada_match && !kp.has_krt {
                let i_la = kp.p.terms().len() - 1;
                kp.try_replace_lakara("3.2.128", i_la, krt);
                kp.p.run_at("3.2.127", i_la, op::add_tag(T::Sat));
            }
        }

        // These are treated separately from SAnac because they are not `Lat` replacements.
        K::SAnan | K::cAnaS => {
            if dhatu.has_text_in(&["pU", "yaj"]) {
                kp.try_add("3.2.128", K::SAnan);
            } else {
                kp.try_add("3.2.129", K::cAnaS);
            }
        }

        // Rules 3.2.134 - 3.2.179 have a separate control flow and are defined in
        // `try_add_krt_for_tacchila_etc`.

        // The normal control flow resumes from 3.2.180 onward.
        K::qu => {
            if kp.has_upapada_in(&["vi", "pra", "sam"]) && dhatu.has_text("BU") {
                kp.try_add("3.2.180", krt);
            }
        }

        K::zwran => {
            // TODO: move others over
            const DAP_ADI: &[&str] = &[
                "dA\\p",
                "RI\\Y",
                "Sasu~",
                "yu",
                "yu\\ji~^r",
                "zwu\\Y",
                "tu\\da~^",
                "zi\\Y",
                "zi\\ca~^",
                "mi\\ha~",
                "da\\nSa~",
                "Ra\\ha~^",
            ];
            if dhatu.has_u("De\\w") {
                kp.try_add("3.2.181", krt);
            } else if dhatu.has_u_in(DAP_ADI) {
                kp.try_add("3.2.182", krt);
            } else if dhatu.has_text("pU") {
                kp.try_add("3.2.183", krt);
            }
        }

        K::itra => {
            if dhatu.has_text_in(&["f", "lU", "DU", "sU", "Kan", "sah", "car"]) {
                kp.try_add("3.2.184", krt);
            } else if dhatu.has_text("pU") {
                // Also allowed by 3.2.186.
                kp.try_add("3.2.185", krt);
            }
        }

        K::tumun => {
            kp.try_add("3.3.10", krt);
        }

        K::ktri => {
            if dhatu.has_tag(T::qvit) {
                if kp.try_add("3.3.88", krt) {
                    // TODO: put this somewhere else?
                    kp.p.run("4.4.20", |p| {
                        p.push(Taddhita::map.to_term());
                    });
                    it_samjna::run(kp.p, i + 2).expect("should never fail");
                }
            }
        }

        K::aTuc => {
            if dhatu.has_tag(T::wvit) {
                kp.try_add("3.3.89", krt);
            }
        }

        K::naN => {
            if dhatu.has_text_in(&["yaj", "yAc", "yat", "viC", "praC", "rakz"]) {
                kp.try_add("3.3.90", krt);
            }
        }

        K::nan => {
            if dhatu.has_u("Yizva\\pa~") {
                kp.try_add("3.3.91", krt);
            }
        }

        // ------------------------------------------
        // striyAm
        // ------------------------------------------
        K::ktin => {
            kp.try_add_with("3.3.94", krt, |p| p.add_tag(T::Stri));
        }

        K::a => {
            if kp.p.has(i, |t| t.is_pratyaya()) {
                kp.try_add_with("3.3.102", krt, |p| p.add_tag(T::Stri));
            } else if dhatu.is_guru() && dhatu.has_antya(&*HAL) {
                kp.try_add_with("3.3.103", krt, |p| p.add_tag(T::Stri));
            }
        }

        K::yuc => {
            if dhatu.has_u_in(&["Ric", "Asa~\\", "SranTa~"]) {
                kp.try_add_with("3.3.107", krt, |p| p.add_tag(T::Stri));
            }
        }

        K::Rvuc => {}

        K::ani => {}

        K::lyuw => {
            kp.try_add("3.3.115", krt);
        }

        K::Ga => {
            kp.try_add("3.3.118", krt);
        }

        K::Kal => {
            // TODO: restrict
            kp.try_add("3.3.126", krt);
        }

        K::ktic => {
            kp.try_add("3.3.174", krt);
        }
        K::kamul | K::Ramul => {
            if krt == K::kamul {
                kp.try_add("3.4.12", krt);
            } else {
                // TODO: move this rule somewhere else?
                kp.try_add("3.4.22", krt);
            }
        }
        K::kasun => {
            if dhatu.has_u_in(&["sf\\px~", "u~tfdi~^r"]) {
                kp.try_add("3.4.17", krt);
            }
        }
        K::ktvA => {
            kp.try_add("3.4.21", krt);
        }
        _ => (),
    }

    Some(kp.has_krt)
}

/// Runs the rules that add a krt-pratyaya to a given dhatu. Returns whether a pratyaya was added.
pub fn run(p: &mut Prakriya, krt: Krt) -> bool {
    if try_add_upapada_krt(p, krt).unwrap_or(false) {
        return true;
    }
    try_add_krt(p, krt).unwrap_or(false)
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;
    use crate::args::{Dhatu, Gana};
    use crate::dhatu_karya;

    fn make_prakriya(dhatu: &str) -> Prakriya {
        let dhatu = Dhatu::new(dhatu, Gana::Bhvadi);
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
