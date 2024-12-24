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

use crate::args::Aupadeshika as Au;
use crate::args::KrtArtha::*;
use crate::args::Sanadi as S;
use crate::args::Upasarga as U;
use crate::args::{BaseKrt, Gana, Lakara, Taddhita};
use crate::core::operators as op;
use crate::core::{Prakriya, PrakriyaTag as PT, Rule, Tag as T, Term};
use crate::dhatu_gana as gana;
use crate::it_samjna;
use crate::krt::utils::KrtPrakriya;
use crate::sounds::{s, Set, AC, HAL, IK};
use crate::stem_gana::TYAD_ADI;
use crate::Rule::Varttika;
use lazy_static::lazy_static;

const II: Set = s(&["i"]);
const UU: Set = s(&["u"]);
const PU: Set = s(&["pu~"]);

lazy_static! {
    static ref EMPTY_TERM: Term = Term::make_text("");
}

/// Tries to add various pratyayas that are just "a."
fn try_add_various_pratyayas(kp: &mut KrtPrakriya) {
    use BaseKrt::*;

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
    // However, there are some minor exceptions where multiple pratyayas can apply for the given
    // dhatu.
    kp.with_context(TacchilaTaddharmaTatsadhukara, |kp| {
        let dhatu = kp.dhatu_end();
        let i_dhatu = kp.i_dhatu;

        let has_prefix_and_text = |x, y| kp.has_upapada(x) && dhatu.has_text(y);
        let has_upasarga_and_u = |x, y| kp.has_upasarga(x) && dhatu.has_u(y);
        let has_upasarga_and_text = |x, y| kp.has_upasarga(x) && dhatu.has_text(y);
        let last = kp.p.terms().last().expect("present");
        let has_yan = last.is(S::yaN);
        let has_san = last.is(S::san);

        if has_prefix_and_text("alam", "kf")
            || (kp.has_both_upasargas(U::nis, U::AN) && dhatu.has_text("kf"))
            || has_upasarga_and_text(U::pra, "jan")
            || has_upasarga_and_text(U::ud, "pac")
            || has_upasarga_and_text(U::ud, "pat")
            || has_upasarga_and_text(U::ud, "mad")
            || dhatu.has_text("ruc")
            || has_upasarga_and_text(U::apa, "trap")
            || dhatu.has_text_in(&["vft", "vfD", "sah", "car"])
        {
            kp.try_add("3.2.136", izRuc);
        } else if kp.dhatu_end().is_nic() && kp.p.is_chandasi() {
            // DArayizRu, pArayizRu, ...
            kp.try_add("3.2.137", izRuc);
        } else if dhatu.has_u_in(&["BU", "BrAjf~\\"]) && kp.p.is_chandasi() {
            if !dhatu.has_u("BU") {
                kp.p.step(Rule::Kashika("3.2.138"));
            }
            // BavizRu
            kp.try_add("3.2.138", izRuc);
        } else if dhatu.has_text_in(&["glE", "ji", "sTA", "BU"]) {
            // glAsnu, jizRu, sTAsnu, BUzRu
            kp.try_add("3.2.139", ksnu);
        } else if dhatu.has_text_in(&["tras", "gfD", "Dfz", "kzip"]) {
            kp.try_add("3.2.140", knu);
        } else if dhatu.has_u_in(gana::SHAM_ADI) {
            kp.try_add("3.2.141", GinuR);
        } else if has_upasarga_and_text(U::sam, "pfc")
            || has_upasarga_and_text(U::anu, "ruD")
            || has_upasarga_and_text(U::AN, "yam")
            || has_upasarga_and_text(U::AN, "yas")
            || has_upasarga_and_text(U::pari, "sf")
            || has_upasarga_and_text(U::sam, "sfj")
            || has_upasarga_and_text(U::pari, "dev")
            || has_upasarga_and_text(U::sam, "jvar")
            || has_upasarga_and_text(U::pari, "kzip")
            || has_upasarga_and_text(U::pari, "raw")
            || has_upasarga_and_text(U::pari, "vad")
            || has_upasarga_and_text(U::pari, "dah")
            || has_upasarga_and_text(U::pari, "muh")
            || dhatu.has_text_in(&["duz", "dviz", "druh", "duh", "yuj"])
            || has_upasarga_and_text(U::AN, "krIq")
            || has_upasarga_and_text(U::vi, "vij")
            || dhatu.has_text_in(&["tyaj", "ranj", "Baj"])
            || has_upasarga_and_text(U::ati, "car")
            || has_upasarga_and_text(U::apa, "car")
            || has_upasarga_and_text(U::AN, "muz")
            || (kp.has_both_upasargas(U::aBi, U::AN) && dhatu.has_text("han"))
        {
            kp.try_add("3.2.142", GinuR);
        } else if kp.has_upasarga(U::vi) && dhatu.has_text_in(&["kaz", "las", "katT", "sranB"]) {
            kp.try_add("3.2.143", GinuR);
        } else if kp.has_upasarga_in(&[U::vi, U::apa]) && dhatu.has_text("laz") {
            // vilAzin, apalAzin
            kp.try_add("3.2.144", GinuR);
        } else if kp.has_upasarga(U::pra)
            && dhatu.has_u_in(&["lapa~", "sf\\", "dru\\", "maTe~", "vada~", "va\\sa~"])
        {
            kp.try_add("3.2.145", GinuR);
        } else if dhatu.has_text_in(&["sf", "Gas", "ad"]) {
            // sfmara, Gasmara, admara
            // (placed here because 'sf' matches elsewhere.)
            kp.try_add("3.2.160", kmarac);
        } else if dhatu.has_u("ga\\mx~") {
            // gatvara
            // ('gam' matches elsewhere.)
            kp.try_add_with("3.2.164", kvarap, |p| p.set(i_dhatu, |t| t.set_antya("t")));
        } else if has_san || has_upasarga_and_u(U::AN, "Sasi~\\") || dhatu.has_u("Bikza~\\") {
            // cikIrzu, jihIrzu, ...
            // ('Bikz' matches elsewhere.)
            kp.try_add("3.2.168", u);
        }

        // Break the `if` chain so that pari-kzip and pari-raw can match again here.
        let dhatu = kp.dhatu_end();
        let has_upasarga_and_u = |x, y| kp.has_upasarga(x) && dhatu.has_u(y);
        if dhatu.has_text_in(&["nind", "hins", "kliS", "KAd"])
            || (kp.has_upasarga(U::vi) && kp.has_sanadi("Ra\\Sa~", S::Ric))
            || has_upasarga_and_u(U::pari, "kzi\\pa~")
            || has_upasarga_and_u(U::pari, "rawa~")
            || (kp.has_upasarga(U::pari) && kp.has_sanadi("vada~", S::Ric))
            || (kp.has_both_upasargas(U::vi, U::AN) && dhatu.has_u("BAza~\\"))
            || (kp.dhatu_start().has_text("asU") && kp.dhatu_end().has_u("yak"))
        {
            // nindaka, hiMsaka
            kp.try_add("3.2.146", vuY);
        } else if upasarge
            && ((dhatu.has_u("divu~") && dhatu.has_gana(Gana::Curadi)) || dhatu.has_u("kru\\Sa~"))
        {
            // Adevaka, parikraSoka
            kp.try_add("3.2.147", vuY);
        } else if dhatu.has_u_in(&["cala~", "cupa~", "ru"]) || kp.has_sanadi("Sabda~", S::Ric) {
            // calana, copana, ...
            kp.try_add("3.2.148", yuc);
        } else if dhatu.has_u_in(&[
            "laza~^",
            "patx~",
            "pa\\da~\\",
            "zWA\\",
            "BU",
            "vfzu~",
            "ha\\na~",
            "kamu~\\",
            "ga\\mx~",
            "SF",
        ]) {
            // apalAzuka, prapAtuka, ...
            kp.try_add("3.2.154", ukaY);
        } else if dhatu.has_u_in(&["jalpa~", "Bikza~\\", "vfN"])
            || kp.has_sanadi_in(&["kuwwa~", "lunwa~"], S::Ric)
        {
            // jalpAka, ...
            kp.try_add("3.2.154", zAkan);
        } else if kp.has_upasarga(U::pra) && dhatu.has_u("ju") {
            // prajavin
            kp.try_add("3.2.155", ini);
        } else if kp.has_sanadi_in(&["spfha", "gfha", "pata"], S::Ric) || dhatu.has_u("daya~\\") {
            // Per kashika, first 3 are curAdi.
            kp.try_add("3.2.158", Aluc);
            // TODO: others
        } else if dhatu.has_text_in(&["Banj", "BAs", "mid"]) {
            kp.try_add("3.2.161", Gurac);
        } else if (dhatu.is_u(Au::vida_2) && dhatu.has_gana(Gana::Adadi))
            || dhatu.has_text_in(&["Bid", "Cid"])
        {
            // Per commentaries, allow only this specific `vid`.
            kp.try_add("3.2.162", kurac);
        } else if dhatu.has_u_in(&["i\\R", "Ra\\Sa~", "ji\\", "sf\\"]) {
            // naSvara, ...
            kp.try_add("3.2.163", kvarap);
        } else if dhatu.has_text("jAgf") {
            // jAgarUka
            kp.try_add("3.2.165", Uka);
        } else if kp.has_sanadi_in(&["ya\\ja~", "japa~", "da\\nSa~"], S::yaN) {
            // yAyajUka, ...
            kp.try_add("3.2.166", Uka);
        } else if dhatu.has_text_in(&["nam", "kanp", "smi", "jas", "kam", "hins", "dIp"]) {
            kp.try_add("3.2.167", ra);
        } else if dhatu.has_text_in(&["svap", "tfz"]) {
            kp.try_add("3.2.172", najiN);
        } else if dhatu.has_text_in(&["SF", "vand"]) {
            // SarAru, vandAru
            kp.try_add("3.2.173", Aru);
        } else if dhatu.has_text("BI") {
            if kp.expects_krt(krukan) {
                kp.try_add(Varttika("3.2.174.1"), krukan);
            } else if kp.expects_krt(kru) {
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
        let dhatu = kp.dhatu_end();
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
        } else if kp.has_upasarga(U::AN) && dhatu.has_u_in(&["ru", "plu\\N"]) {
            // ArAva, Arava, AplAva, Aplava
            // (block 3.3.22)
            kp.optional_try_add("3.3.50", GaY);
        } else if upasarge && dhatu.has_u("ru") {
            // saMrAva, uparAva
            kp.try_add("3.3.22", GaY);
        } else if kp.has_upasarga(U::sam) && dhatu.has_u_in(&["yu", "dru\\", "du\\"]) {
            // samyAva, sandrAva, sandAva
            kp.try_add("3.3.23", GaY);
        } else if !upasarge && dhatu.has_u_in(&["SriY", "RI\\Y", "BU"]) {
            // SRAya, nAya, BAva
            kp.try_add("3.3.24", GaY);
        } else if kp.has_upasarga(U::vi) && dhatu.has_u_in(&["wukzu", "Sru\\"]) {
            // vikzAva, viSrAva
            kp.try_add("3.3.25", GaY);
        } else if kp.has_upasarga_in(&[U::ava, U::ud]) && dhatu.has_u("RI\\Y") {
            // avanAya, unnAya
            kp.try_add("3.3.26", GaY);
        } else if kp.has_upasarga(U::pra) && dhatu.has_u_in(&["dru\\", "zwu\\Y", "sru\\"]) {
            // pradrAva, prastAva, prasrAva
            kp.try_add("3.3.27", GaY);
        } else if (kp.has_upasarga(U::nir) && dhatu.has_u_in(&["pUY", "pUN"]))
            || (kp.has_upasarga(U::aBi) && dhatu.has_u("lUY"))
        {
            // nizpAva, aBilAva, ...
            // "pū iti pūṅpūñoḥ sāmānyena grahaṇam" (KV)
            kp.try_add("3.3.28", GaY);
        } else if kp.has_upasarga_in(&[U::ud, U::ni]) && dhatu.has_u("gF") {
            // "gṝ śabde, gṝ nigaraṇe, dvayorapi grahaṇam" (KV)
            kp.try_add("3.3.29", GaY);
        } else if kp.has_upasarga_in(&[U::ud, U::ni])
            && dhatu.has_u("kF")
            && dhatu.has_gana(Gana::Tudadi)
        {
            // utkAra, utkara
            // "vikṣepārthasya kiratergrahaṇaṃ, na hiṃsārthasya" (KV)
            kp.try_artha_add("3.3.30", GaY);
        } else if kp.has_upasarga(U::sam) && dhatu.has_u("zwu\\Y") {
            // saMstAva, saMstava
            kp.try_artha_add("3.3.31", GaY);
        } else if dhatu.has_u("stFY") {
            if kp.has_upasarga(U::pra) {
                // prastAra, prastara
                kp.try_artha_add("3.3.32", GaY);
            } else if kp.has_upasarga(U::vi) {
                // vistAra, vistara
                kp.try_artha_add("3.3.33", GaY);
                // vizwAra
                kp.try_artha_add("3.3.34", GaY);
            }
        } else if dhatu.has_u("graha~^") {
            // Collect all "grah" rules here, mainly for 3.3.45 + 3.3.51.
            if kp.has_upasarga(U::ud) {
                // udgrAha
                kp.try_add("3.3.35", GaY);
            } else if kp.has_upasarga(U::sam) {
                // saNgrAha
                kp.try_artha_add("3.3.36", GaY);
            } else if kp.has_upasarga_in(&[U::ava, U::ni]) {
                // avagrAha, nigrAha
                kp.try_artha_add("3.3.45", GaY);
            } else if kp.has_upasarga(U::pra) {
                // pragrAha
                kp.try_artha_add("3.3.46", GaY);
            } else if kp.has_upasarga(U::pari) {
                // parigrAha
                kp.try_artha_add("3.3.47", GaY);
            }
        } else if (kp.has_upasarga(U::pari) && dhatu.has_u("RI\\Y"))
            || (kp.has_upasarga(U::ni) && dhatu.has_u("i\\R"))
        {
            // parinAya, nyAya
            kp.try_artha_add("3.3.37", GaY);
        } else if kp.has_upasarga(U::pari) && dhatu.has_u("i\\R") {
            kp.try_artha_add("3.3.38", GaY);
        } else if kp.has_upasarga_in(&[U::vi, U::upa]) && dhatu.is_u(Au::SIN) {
            kp.try_artha_add("3.3.39", GaY);
        } else if dhatu.has_u("ci\\Y") {
            if !kp.try_artha_add("3.3.40", GaY) {
                kp.try_artha_add_with("3.3.41", GaY, |p| p.set(i_dhatu, op::adi("k")));
                kp.try_artha_add_with("3.3.42", GaY, |p| p.set(i_dhatu, op::adi("k")));
            }
        } else if kp.has_upasarga(U::ni) && dhatu.has_u_in(&["vfN", "vfY"]) {
            // nIvAra
            // "vṛ iti vṛṅvṛñoḥ sāmānyena grahaṇam" (KV)
            kp.try_artha_add("3.3.48", GaY);
        } else if kp.has_upasarga(U::ud) && dhatu.has_u_in(&["SriY", "yu", "pUY", "dru\\"]) {
            // ucCrAya, udyAva, utpAva, uddrAva
            kp.try_add("3.3.49", GaY);
        }

        // TODO: vibhasha
        let dhatu = kp.dhatu_end();
        if kp.had_match {
        } else if dhatu.has_u("graha~^") {
            if kp.has_upasarga(U::ava) {
                kp.try_artha_add("3.3.51", GaY);
            } else if kp.has_upasarga(U::pra) {
                kp.try_artha_add("3.3.52", GaY);
                kp.try_artha_add("3.3.53", GaY);
            }
        } else if kp.has_upasarga(U::pra) && dhatu.has_u("vfY") {
            // pravAra, pravara
            kp.optional_try_add("3.3.54", GaY);
        } else if kp.has_upasarga(U::pari) && dhatu.has_u("BU") {
            // pariBAva, pariBava
            kp.optional_try_add("3.3.55", GaY);
        }

        let dhatu = kp.dhatu_end();
        if kp.had_match {
            // Skip.
        } else if dhatu.has_u_in(&["graha~^", "vfY", "df", "ga\\mx~"])
            || (kp.has_upasarga_in(&[U::nir, U::nis]) && dhatu.has_u("ci\\Y"))
        {
            kp.try_add("3.3.58", ap);
        } else if upasarge && dhatu.has_u("a\\da~") {
            if kp.has_upasarga(U::ni) {
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
        } else if kp.has_upasarga_in(&[U::sam, U::upa, U::ni, U::vi]) && dhatu.has_u("ya\\ma~") {
            // saMyAma, saMyama, ...
            kp.optional_try_add("3.3.63", ap);
        } else if kp.has_upasarga(U::ni) && dhatu.has_u_in(&["gada~", "Rada~", "paWa~", "svana~"]) {
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
        } else if kp.has_upasarga_in(&[U::sam, U::ud]) && dhatu.has_u("aja~") {
            // samaja, udaja
            kp.try_artha_add("3.3.69", ap);
        } else if kp.has_upasarga(U::upa) && dhatu.has_u("sf\\") {
            // upasara
            // (The KV's examples all use upa-.)
            kp.try_artha_add("3.3.71", ap);
        } else if dhatu.has_u("hve\\Y") {
            if kp.has_upasarga_in(&[U::ni, U::aBi, U::upa, U::vi]) {
                // nihava, aBihava, upahava, vihava
                kp.try_add_with("3.3.72", ap, |p| p.set(i_dhatu, op::text("hu")));
            } else if kp.has_upasarga(U::AN) {
                // Ahava
                kp.try_artha_add_with("3.3.73", ap, |p| p.set(i_dhatu, op::text("hu")));
            } else if !upasarge {
                // hava
                kp.try_add_with("3.3.75", ap, |p| p.set(i_dhatu, op::text("hu")));
            }
        } else if dhatu.is_u(Au::hana) {
            if !upasarge {
                kp.try_add_with("3.3.76", ap, |p| p.set(i_dhatu, op::text("vaD")));
            }
        }

        // Base cases
        let dhatu = kp.dhatu_end();
        if kp.had_match {
        } else if dhatu.has_antya(II) {
            // caya, ...
            kp.try_add("3.3.56", ac);
        } else if dhatu.has_antya('F') || dhatu.has_antya(UU) {
            // kara, yava, ...
            kp.try_add("3.3.57", ap);
        } else {
            kp.try_add("3.3.18", GaY);
        }
    });

    let i_dhatu = kp.i_dhatu;
    let dhatu = kp.dhatu_end();
    let is_han = dhatu.is_u(Au::hana);
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
        let dhatu = kp.dhatu_end();
        if kp.has_upasarga(U::ava) && dhatu.has_u_in(&["tF", "stFY"]) {
            kp.try_add("3.3.120", GaY);
        } else if dhatu.has_u("Kanu~^") {
            kp.optional_try_add("3.3.125", Ga);
            kp.optional_try_add(Varttika("3.3.125.1:1"), qa);
            kp.optional_try_add(Varttika("3.3.125.1:2"), qara);
            kp.optional_try_add(Varttika("3.3.125.1:3"), ika);
            kp.optional_try_add(Varttika("3.3.125.1:4"), ikavaka);
        }

        // Base case
        let dhatu = kp.dhatu_end();
        if dhatu.has_antya(HAL) {
            kp.try_add("3.3.121", GaY);
        }
    })
}

fn is_nandi_grahi_pacadi(kp: &KrtPrakriya) -> bool {
    let dhatu = kp.dhatu_end();

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

fn try_add_upapada_krt(kp: &mut KrtPrakriya) -> Option<bool> {
    use BaseKrt::*;

    const DIVA_ADI: &[&str] = &[
        "divA", "viBA", "niSA", "praBA", "BAs", "kAra", "anta", "ananta", "Adi", "bahu", "nAndI",
        "kim", "lipi", "libi", "bali", "Bakti", "kartf", "citra", "kzetra", "jaNGA", "bAhu",
        "ahas", "yat", "tat", "Danus", "arus",
    ];
    const ADHYA_ADI: &[&str] = &[
        "AQya", "suBaga", "sTUla", "palita", "nagna", "anDa", "priya",
    ];

    let i_dhatu = kp.p.find_first_where(|t| t.is_dhatu())?;
    let dhatu = kp.dhatu_end();

    let upapada = match kp.p.get_if(0, |t| t.has_tag(T::Pratipadika)) {
        Some(t) => t,
        None => &EMPTY_TERM,
    };
    let upapade = kp.p.has(0, |t| t.has_tag(T::Pratipadika));

    let nau = kp.p.has(i_dhatu + 1, |t| t.is(S::Ric));
    let upasarge = kp.p.has_prev_non_empty(i_dhatu, |t| t.is_upasarga());

    let krt = kp.krt;
    match krt {
        aR | ka | ac | wa | wak if upapade => {
            if upapada.has_text_in(&["kzema", "priya", "madre"]) && dhatu.is_u(Au::qukfY) {
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
                kp.try_add(Varttika("3.2.8.1"), wak);
            } else if dhatu.has_u("hf\\Y") {
                if kp.has_upasarga(U::AN) {
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
                    kp.try_add(Varttika("3.2.9.1"), ac);
                } else if upapada.has_text("sUtra") {
                    // dhAri-arthe
                    kp.optional_try_add(Varttika("3.2.9.2"), ac);
                }
            } else if !upasarge && dhatu.has_antya('A') {
                kp.try_add("3.2.3", ka);
            } else if kp.has_upasarga_dhatu(i_dhatu, "pari", "mfjU~")
                || kp.has_upasarga_dhatu(i_dhatu, "apa", "Ru\\da~^")
            {
                kp.try_add("3.2.5", ka);
            } else if kp.has_upasarga(U::pra) && dhatu.has_u_in(&["qudA\\Y", "jYA\\"]) {
                kp.try_add("3.2.6", ka);
            } else if kp.has_upasarga(U::AN) && dhatu.has_u("hf\\Y") {
                kp.try_add("3.2.11", ac);
            } else if dhatu.has_u("arha~") {
                kp.try_add("3.2.12", ac);
            } else if upapada.has_text("Sam") {
                kp.try_add("3.2.14", ac);
            } else if dhatu.is_u(Au::SIN) {
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
            } else if dhatu.is_u(Au::qukfY) {
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
            } else if dhatu.is_u(Au::hana) {
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

        in_ if upapade => {
            if upapada.has_text_in(&["stamba", "Sakft"]) && dhatu.is_u(Au::qukfY) {
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
            } else if kp.p.is_chandasi() && dhatu.has_u_in(&["vanu~\\", "zaRa~", "rakza~", "maTe~"])
            {
                // brahmavani, ...
                kp.try_add("3.2.27", krt);
            }
        }
        KaS if upapade => {
            let nasika = upapada.has_text("nAsikA");
            let stana = upapada.has_text("stana");
            let dhma = dhatu.has_u("DmA\\");
            let dhe = dhatu.has_u("De\\w");
            if dhatu.has_u("ejf~\\") && nau {
                // aNgamejaya, janamejaya
                kp.try_add("3.2.28", krt);
            } else if (nasika && (dhma || dhe)) || (stana && dhe) {
                kp.try_add("3.2.29", krt);
            } else if (dhma || dhe) && upapada.has_text_in(&["nAqI", "muzwi"]) {
                kp.try_add("3.2.30", krt);
            } else if upapada.has_text("kUla")
                && kp.has_upasarga(U::ud)
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

        Kac if upapade => {
            if upapada.has_text_in(&["priya", "vaSa"]) && dhatu.has_u("vada~") {
                kp.try_add("3.2.38", krt);
            } else if upapada.has_text_in(&["dvizat", "para"])
                && kp.has_sanadi_in(&["ta\\pa~", "tapa~"], S::Ric)
            {
                // dvizantapa, parantapa
                kp.try_add("3.2.39", krt);
            } else if upapada.has_text("vAc") && dhatu.has_text("yam") {
                kp.try_add("3.2.40", krt);
            } else if upapada.has_text_in(&["pur", "sarva"])
                && (kp.has_sanadi("dF", S::Ric) || dhatu.has_text("sah"))
            {
                // purantara, sarvaMsaha
                kp.try_add("3.2.41", krt);
            } else if upapada.has_text_in(&["sarva", "kUla", "aBra", "karIza"])
                && dhatu.has_text("kaz")
            {
                kp.try_add("3.2.42", krt);
            } else if dhatu.is_u(Au::qukfY) {
                if upapada.has_text_in(&["meGa", "fti", "Baya"]) {
                    kp.try_add("3.2.43", krt);
                } else if upapada.has_text_in(&["kzema", "priya", "madra"]) {
                    kp.try_add("3.2.44", krt);
                }
            } else if upapada.has_text("ASita") && dhatu.has_u("BU") {
                kp.try_add("3.2.45", krt);
            } else if upapada.has_text("suta") && dhatu.has_u("ga\\mx~") {
                // sutaNgama
                kp.try_add("3.2.47", krt);
            }
        }

        qa if upapade => {
            if dhatu.has_u("ga\\mx~") {
                if upapada.has_text_in(&[
                    "anta", "atyanta", "aDvan", "dUra", "pAra", "sarva", "ananta",
                ]) {
                    kp.try_add("3.2.48", krt);
                }
            } else if dhatu.is_u(Au::hana) {
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

        Rini if upapade => {
            if upapada.has_text_in(&["kumAra", "Sirza"]) && dhatu.is_u(Au::hana) {
                kp.try_add("3.2.51", krt);
            } else if upapada.has_text("vrata") {
                kp.try_add("3.2.80", krt);
            } else if dhatu.has_u("ma\\na~\\") {
                kp.try_add("3.2.82", krt);
            } else if dhatu.has_u("ya\\ja~^") {
                kp.try_add("3.2.85", krt);
            } else if dhatu.is_u(Au::hana) {
                kp.try_add("3.2.86", krt);
            } else {
                kp.try_add("3.2.78", krt);
            }
        }

        Kyun if upapade => {
            if upapada.has_text_in(ADHYA_ADI) && dhatu.is_u(Au::qukfY) {
                kp.try_add("3.2.56", krt);
            }
        }

        KizRuc | KukaY if upapade => {
            if upapada.has_text_in(ADHYA_ADI) && dhatu.has_u("BU") {
                kp.try_add("3.2.57", krt);
            }
        }

        kvin | kaY => {
            if upapada.is_any_phit(TYAD_ADI) && dhatu.has_u("df\\Si~r") {
                kp.try_add("3.2.60", krt);
            } else if upapada.has_text_in(&["samAna", "anya"]) && dhatu.has_u("df\\Si~r") {
                kp.try_add(Varttika("3.2.60.1"), krt);
            } else if krt == kvin {
                if !upapada.has_text("udaka") && dhatu.has_text("spfS") {
                    kp.try_add("3.2.58", kvin);
                } else if dhatu.has_u("df\\Si~r") {
                    kp.try_add("3.2.60", kvin);
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
                    } else if kp.has_upasarga(U::ud) && dhatu.has_u("zRi\\ha~") {
                        kp.do_nipatana(code, "uzRih");
                    } else if dhatu.has_u("ancu~") {
                        kp.try_add(code, kvin);
                    } else if dhatu.has_u("yu\\ji~^r") {
                        kp.do_nipatana(code, "yuj");
                    } else if dhatu.has_u("krunca~") {
                        kp.do_nipatana(code, "kruYc");
                    }
                }
            }
        }

        viw if upapade => {
            if dhatu.has_u_in(&["janI~\\", "zaRu~^", "Kanu~^", "kramu~", "ga\\mx~"]) {
                // abjAH, ...
                kp.try_add("3.2.67", krt);
            } else if dhatu.has_u("a\\da~") {
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
            if dhatu.is_u(Au::hana) {
                if upapada.has_text_in(&["brahman", "BrURa", "vftra"]) {
                    kp.try_add("3.2.87", krt);
                }
            } else if dhatu.has_u("zWA\\") {
                kp.try_add("3.2.77", krt);
            } else if upapada.has_text("soma") && dhatu.has_u("zu\\Y") {
                kp.try_add("3.2.90", krt);
            } else if upapada.has_text("agni") && dhatu.has_u("ci\\Y") {
                kp.try_add("3.2.91", krt);
            } else if !dhatu.has_text("ay") {
                // Exclude 'ay' because it produces weird output.
                kp.try_add("3.2.76", krt);
            }
        }

        ini => {
            if kp.has_upasarga(U::vi) && dhatu.has_u("qukrI\\Y") {
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

    if kp.has_krt && krt == kvip {
        let dhatu = kp.dhatu_end();
        if dhatu.has_text("Sri") {
            // SrI
            // TODO: others
            kp.p.run_at(Varttika("3.2.178.1"), i_dhatu, |t| t.set_antya("I"));
        }
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
fn try_add_krt(kp: &mut KrtPrakriya) -> Option<bool> {
    use BaseKrt as K;

    if kp.krt == K::ki && kp.has_upapada("antar") {
        let i_upapada = kp.i_upapada()?;
        kp.p.run_at(Varttika("1.4.65"), i_upapada, op::add_tag(T::Upasarga));
    }

    let i_start = kp.p.find_first_with_tag(T::Dhatu)?;
    let i_end = kp.p.find_last_with_tag(T::Dhatu)?;

    // Pre-calculate some common properties.
    let i_upasarga = kp.p.find_prev_where(i_start, |t| t.is_upasarga());
    let upasarge = i_upasarga.is_some();
    let supi = i_end > 0 && kp.p.has(i_end - 1, |t| t.is_sup());

    // For convenience below, wrap `Prakriya` in a new `KrtPrakriya` type that contains `krt` and
    // records whether or not any of these rules were applied.
    let dhatu = kp.dhatu_end();
    let i_dhatu = kp.i_dhatu;

    let krt = kp.krt;
    match krt {
        // ------------------------------------------
        // krtyAH
        // ------------------------------------------
        K::tavyat | K::tavya | K::anIyar => {
            let added = kp.try_add("3.1.96", krt);
            if added && krt == K::tavyat && kp.dhatu_end().has_u("va\\sa~") {
                // vAstavya
                kp.p.optional_run_at(Varttika("3.1.96.1"), i_end + 1, |t| t.add_tag(T::Rit));
            }
        }

        K::kelimar => {
            kp.try_add(Varttika("3.1.96.2"), krt);
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
            let dhatu = kp.dhatu_end();
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
                    skip_3_1_110 = kp.p.optional_run("3.1.120", |_| {});
                }
            } else if dhatu.has_text("mfj") {
                // This rule makes rule 3.1.110 optional for mfj.
                skip_3_1_110 = kp.p.optional_run("3.1.113", |_| {});
            } else if !upasarge && dhatu.has_u("va\\ha~^") {
                // vahya
                kp.optional_try_add("3.1.102", K::yat);
            } else if !upasarge && dhatu.has_u("f\\") {
                // arya
                kp.optional_try_add("3.1.103", K::yat);
            } else if kp.has_upasarga(U::upa) && dhatu.has_u("sf\\") {
                // upasaryA
                kp.optional_try_add_with("3.1.104", K::yat, |p| p.add_tag(PT::Stri));
            }

            // Specific rules (required)
            let dhatu = kp.dhatu_end();
            let mut avashyaka_blocked = false;
            if dhatu.has_u_in(&["Sa\\kx~", "zaha~\\"]) {
                kp.try_add("3.1.99", K::yat);
            } else if dhatu.has_u_in(&["gada~", "madI~", "cara~", "ya\\ma~"]) && !upasarge {
                kp.try_add("3.1.100", K::yat);
            } else if dhatu.has_u("cara~") && kp.has_upasarga(U::AN) {
                kp.optional_try_add(Varttika("3.1.100.1"), K::yat);
            } else if !upasarge && dhatu.has_text("vad") && supi {
                kp.try_add("3.1.106", K::yat);
                kp.try_add("3.1.106", K::kyap);
            } else if !upasarge && supi && dhatu.has_u("BU") {
                // The condition here is bhAve, but per the kashika, it is always bhAve per 3.4.70,
                // so it is effectively nitya.
                kp.try_add("3.1.107", K::kyap);
            } else if !upasarge && supi && dhatu.is_u(Au::hana) {
                // brahmahatya, ...
                kp.try_add_with("3.1.108", K::kyap, |p| {
                    p.set(i_dhatu, |t| t.set_antya("t"));
                });
            } else if dhatu.has_u_in(&["i\\R", "zwu\\Y", "SAsu~", "vfY", "df", "juzI~\\"]) {
                if dhatu.has_u("i\\R") {
                    // Optional so we can apply 3.3.99 later.
                    let ok = kp.optional_try_add("3.1.109", K::kyap);
                    avashyaka_blocked = ok;
                } else {
                    kp.try_add("3.1.109", K::kyap);
                    avashyaka_blocked = true;
                }
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
            let dhatu = kp.dhatu_end();
            if (dhatu.has_antya('u') || dhatu.has_antya('U')) && !avashyaka_blocked {
                kp.optional_try_add("3.1.125", K::Ryat);
            } else if dhatu.is_u(Au::hana) {
                kp.optional_try_add_with(Varttika("3.1.97.2"), K::yat, |p| {
                    p.set(i_dhatu, |t| t.set_text("vaD"));
                });
            }

            // General rules (obligatory)
            let dhatu = kp.dhatu_end();
            if !kp.had_match {
                if dhatu.has_upadha('a') && dhatu.has_antya(PU) {
                    // Sapya, laBya, japya
                    kp.try_add("3.1.98", K::yat);
                } else if dhatu.has_u_in(&["taka~", "Sasu~\\", "cate~^", "yatI~\\", "janI~\\"]) {
                    kp.try_add(Varttika("3.1.97.1"), K::yat);
                } else if dhatu.has_antya('f') || dhatu.has_antya('F') || dhatu.has_antya(HAL) {
                    kp.try_add("3.1.124", K::Ryat);
                } else if dhatu.has_antya(AC) {
                    kp.try_add("3.1.97", K::yat);
                }
            }
        }

        K::Rvul | K::tfc => {
            kp.try_add("3.1.133", krt);
        }

        K::lyu | K::Rini => {
            if is_nandi_grahi_pacadi(kp) {
                kp.try_add("3.1.134", krt);
            } else if krt == K::Rini {
                // TODO: supi
                kp.try_add("3.2.78", krt);
            }
        }

        // "a" (3.1.134 - ???)
        K::ac | K::Sa | K::ka | K::Ra => {
            // These are all bhvAdi dhAtus, so also check for `Bhvadi` to avoid other dhatus.
            const PA_GHRA: &[&str] = &["pA\\", "GrA\\", "DmA\\", "De\\w", "df\\Si~r"];
            const JVAL_ADI: &[&str] = &[
                "jvala~",
                "cala~",
                "jala~",
                "wala~",
                "wvala~",
                "zWala~",
                "hala~",
                "Rala~",
                "pala~",
                "bala~",
                "pula~",
                "kula~",
                "Sala~",
                "hula~",
                "patx~",
                "kzala~",
                "kvaTe~",
                "paTe~",
                "maTe~",
                "wuvama~",
                "Bramu~",
                "kzara~",
                "dvf\\",
                "zaha~\\",
                "ra\\mu~\\",
                "za\\dx~",
                "Sa\\dx~",
                "kru\\Sa~",
                "kuca~",
                "buDa~",
                "ru\\ha~",
                "kasa~",
            ];

            let dhatu = kp.dhatu_end();

            if !upasarge && dhatu.is_any_u(&[Au::qudAY, Au::quDAY]) {
                // dada, daDa
                // (if rejected: dAya, DAya by 3.1.141)
                kp.optional_try_add("3.1.139", K::Sa);
            } else if !upasarge && dhatu.has_u_in(JVAL_ADI) {
                // jvAla, cAla
                kp.optional_try_add("3.1.140", K::Ra);
            } else if dhatu.has_u("graha~^") {
                kp.optional_try_add("3.1.143", K::Ra);
                kp.optional_try_add("3.1.144", K::ka);
            }

            let dhatu = kp.dhatu_end();
            if dhatu.has_u("tanu~^") {
                // avatAna
                kp.try_add(Varttika("3.1.140.1"), K::Ra);
            } else if upasarge && dhatu.has_u_in(PA_GHRA) && dhatu.has_gana(Gana::Bhvadi) {
                kp.try_add("3.1.137", K::Sa);
            } else if (!upasarge && dhatu.has_u_in(&["li\\pa~^", "vi\\dx~^"]))
                || (!upasarge
                    && kp.has_sanadi_in(&["Df\\Y", "pF", "vida~", "cita~", "zaha~\\"], S::Ric))
                || (kp.has_upasarga(U::ud) && kp.has_sanadi(&"ejf~\\", S::Ric))
                || (!upasarge && dhatu.has_u("sAti"))
            {
                // limpa, vinda, DAraya, pAraya, vedaya, udejaya, cetaya, sAtaya, sAhaya
                kp.try_add("3.1.138", K::Sa);
            } else if upasarge && dhatu.has_antya('A') {
                // prasTa, sugla, sumla
                kp.try_add("3.1.136", K::ka);
            } else if dhatu.has_u_in(&["jYA\\", "prI\\Y", "kF"]) {
                // Part 1 of 2 (jYa, ...)
                kp.try_add("3.1.135", K::ka);
            } else if dhatu.has_u_in(&["SyE\\N", "vya\\Da~", "li\\ha~^", "Sli\\za~", "Svasa~"])
                || (kp.has_upasarga_in(&[U::AN, U::sam]) && dhatu.has_u("sru\\"))
                || (kp.has_upasarga(U::ati) && dhatu.has_u("i\\R"))
                || (kp.has_upasarga(U::ava) && dhatu.has_u("zo\\"))
                || (kp.has_upasarga(U::ava) && dhatu.has_u("hf\\Y"))
                || dhatu.has_antya('A')
            {
                // avaSyAya, pratiSyAya, ...
                kp.try_add("3.1.141", K::Ra);
            } else if dhatu.has_upadha(IK) {
                // Part 2 of 2 (vikzipa, viliKa, buDa)
                kp.try_add("3.1.135", K::ka);
            } else if dhatu.has_u_in(&["wudu\\", "RI\\Y"]) && !upasarge {
                kp.try_add("3.1.142", K::Ra);
            } else if krt == K::ac {
                // ajvidhiḥ sarvadhātubhyaḥ paṭhyante ca pacādayaḥ। aṇbādhanārtham eva
                // syāt sidhyanti śvapacādayaḥ।
                kp.try_add(Rule::Kashika("3.1.134"), K::ac);
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
            } else if !dhatu.has_text("ay") {
                // Exclude 'ay' because it produces weird output.
                kp.try_add("3.2.76", krt);
            }
        }

        K::Rvi => {
            if dhatu.has_u("Ba\\ja~^") {
                kp.try_add("3.2.62", krt);
            } else if i_end > 0 && dhatu.has_u("va\\ha~^") {
                kp.try_add("3.2.64", krt);
            }
        }

        K::Yyuw => {
            if kp.p.is_chandasi() {
                if kp.has_upapada_in(&["kavya", "purIza", "purIzya"]) && dhatu.has_u("va\\ha~^") {
                    // kavyavAhana, ...
                    kp.try_add("3.2.65", krt);
                } else if kp.has_upapada("havya") {
                    // havyavAhana
                    kp.try_add("3.2.66", krt);
                }
            }
        }

        K::manin | K::kvanip | K::vanip | K::vic => {
            let code = "3.2.75";
            if krt == K::manin && dhatu.has_text("Bas") {
                kp.try_add(code, krt);
            } else if krt == K::vic && dhatu.has_text("riz") {
                kp.try_add(code, krt);
            } else if !dhatu.has_text_in(&["ay", "av"]) {
                // suSarmA, prAtaritvan, vijAvA, rez, ...
                // Exclude 'ay' and 'av' because they produce weird output.
                kp.try_add("3.2.75", krt);
            }
        }

        K::kta | K::ktavatu => {
            if dhatu.has_tag(T::YIt) {
                kp.try_add("3.2.187", BaseKrt::kta);
            }
            kp.try_add("3.2.102", krt);

            if kp.has_krt {
                let i_last = kp.p.terms().len() - 1;
                kp.p.add_tag_at("1.1.26", i_last, T::Nistha);
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
                K::Satf => kp.p.has_tag(PT::Parasmaipada),
                // taṅānāv ātmanepadam (1.4.100)
                K::SAnac => kp.p.has_tag(PT::Atmanepada),
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
                if kp.p.has(i_la, |t| t.has_lakara(Lakara::Lat)) {
                    // pacantam, ...
                    kp.try_replace_lakara("3.2.124", i_la, krt);
                } else if kp.p.has(i_la, |t| t.has_lakara(Lakara::Lrt)) {
                    // karizyantam, ...
                    kp.try_replace_lakara("3.3.14", i_la, krt);
                }
                if kp.has_krt {
                    kp.p.add_tag_at("3.2.127", i_la, T::Sat);
                }
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
            if kp.has_upasarga_in(&[U::vi, U::pra, U::sam]) && dhatu.has_text("BU") {
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
                "patx~",
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
                        p.push(Taddhita::map.into());
                    });
                    it_samjna::run(kp.p, i_end + 2).expect("should never fail");
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

        K::ki => {
            if upasarge && dhatu.has_tag(T::Ghu) {
                kp.try_add("3.3.92", krt);
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
            if kp.upapada().is_some() {
                kp.try_add("3.3.126", krt);
            }
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

fn try_add_strilinga_pratyayas(kp: &mut KrtPrakriya) -> Option<bool> {
    use BaseKrt::*;

    let dhatu = kp.dhatu_end();
    let i_start = kp.p.find_first_with_tag(T::Dhatu)?;
    let i_end = kp.p.find_last_with_tag(T::Dhatu)?;
    let i_upasarga = kp.p.find_prev_where(i_start, |t| t.is_upasarga());
    let upasarge = i_upasarga.is_some();

    if dhatu.has_u_in(&["zwA\\", "gE\\", "pA\\", "qupa\\ca~^z"]) {
        // prasTiti, udgIti, ...
        // BAve
        kp.optional_try_add_with("3.3.95", ktin, |p| p.add_tag(PT::Stri));
    } else if dhatu.has_u_in(&["vraja~", "ya\\ja~^"]) {
        // vrajyA, ijyA
        // BAve
        kp.optional_try_add_with("3.3.98", kyap, |p| p.add_tag(PT::Stri));
    } else if kp.has_upa_u(U::sam, "aja~")
        || kp.has_upa_u(U::ni, "za\\dx~")
        || kp.has_upa_u(U::ni, "patx~")
        || dhatu.has_u_in(&["ma\\na~\\", "vida~", "zu\\Y", "SIN", "Bf\\Y", "i\\R"])
    {
        // samajyA, nizadyA, ...
        kp.try_add_with("3.3.99", kyap, |p| p.add_tag(PT::Stri));
    } else if dhatu.has_u("qukf\\Y") {
        // kfti
        kp.optional_try_add_with("3.3.100:1", ktin, |p| {
            p.add_tag(PT::Stri);
            p.add_tag(PT::Bhave);
        });
        // kriyA
        kp.optional_try_add_with("3.3.100:2", Sa, |p| {
            p.add_tag(PT::Stri);
            p.add_tag(PT::Bhave);
        });
        // kftyA
        kp.try_add_with("3.3.100:3", kyap, |p| {
            p.add_tag(PT::Stri);
            p.add_tag(PT::Bhave);
        });
    }

    fn map_uti_yuti(t: &Term) -> Option<&'static str> {
        let ret = match t.u.as_ref()?.as_str() {
            "ava~" => "U",
            "yu" => "yU",
            "ju" => "jU",
            "zo\\" => "sA",
            "zaRa~" => "sA",
            "ha\\na~" => "he",
            "hi\\" => "he",
            "kFta~" => "kIr",
            _ => return None,
        };
        Some(ret)
    }

    let dhatu = kp.dhatu_end();
    let i_dhatu_end = kp.i_dhatu_end;
    if dhatu.has_u_in(&[
        "A\\px~",
        "rA\\Da~",
        "dIpI~\\",
        "sransu~\\",
        "Dvansu~\\",
        "qula\\Ba~\\z",
    ]) {
        // Apti, ...
        kp.try_add_with(Varttika("3.3.94.1"), ktin, |p| p.add_tag(PT::Stri));
    } else if dhatu.has_u_in(&["Sru\\", "ya\\ja~^", "zwu\\Y"]) {
        // Sruti, izwi, stuti
        kp.try_add_with(Varttika("3.3.94.2"), ktin, |p| p.add_tag(PT::Stri));
    } else if dhatu.has_u_in(&["glE\\", "mlE\\", "jyA\\", "o~hA\\k"]) {
        // glAni, ...
        kp.try_add_with(Varttika("3.3.94.3"), ni, |p| p.add_tag(PT::Stri));
    } else if dhatu.has_antya('F') || dhatu.has_u_in(gana::LU_ADI) {
        // kIrRi, gIrRi, ...
        kp.try_add_with(Varttika("3.3.94.4"), ktin, |p| {
            p.add_tag(PT::Stri);
            p.terms_mut().last_mut().expect("ok").add_tag(T::Nistha);
        });
    } else if kp.has_upasarga(U::sam) && dhatu.has_u("pa\\da~\\") {
        // sampat, sampatti, vipat, vipatti
        kp.try_add_with(Varttika("3.3.94.5"), kvip, |p| p.add_tag(PT::Stri));
    } else if dhatu.has_u_in(&["zWA\\", "gE\\", "pA\\", "qupa\\ca~^z"]) {
        // prasTiti, ...
        kp.try_add_with(Varttika("3.3.96"), ktin, |p| p.add_tag(PT::Stri));
    } else if let Some(sub) = map_uti_yuti(kp.dhatu_start()) {
        let i_dhatu_start = kp.i_dhatu;
        let matches = (i_dhatu_start == i_dhatu_end)
            || (i_dhatu_start + 1 == i_dhatu_end && kp.dhatu_start().has_u("kFta~"));

        if matches {
            // Uti, yUti, ...
            kp.try_add_with(Varttika("3.3.97"), ktin, |p| {
                p.set(i_dhatu_start, |t| {
                    t.set_text(sub);
                    t.add_tag(T::Complete)
                });
                p.add_tag(PT::Stri)
            });
        }
    } else if dhatu.has_u("izu~") && kp.krt == Sa {
        // icCA
        kp.do_nipatana("3.3.101", "icCA");
    } else if dhatu.is(S::Ric) || dhatu.has_u_in(&["Asa~\\", "SranTa~"]) {
        // kAraRA, hAraRA, ...
        // (blocks 3.3.102)
        kp.try_add_with("3.3.107", yuc, |p| p.add_tag(PT::Stri));
    } else if kp.p.has(i_end, |t| t.is_pratyaya()) {
        // cikIrzA, jihIrzA, ...
        kp.try_add_with("3.3.102", a, |p| p.add_tag(PT::Stri));
    } else if dhatu.is_guru() && dhatu.has_antya(HAL) {
        // kuRqA, huRqA, ...
        kp.try_add_with("3.3.103", a, |p| p.add_tag(PT::Stri));
    } else if dhatu.has_tag(T::zit) {
        // TODO: bhid-Adi
        kp.try_add_with("3.3.104", aN, |p| p.add_tag(PT::Stri));
    } else if dhatu.has_antya('A') {
        let ok = if kp.has_upapada_in(&["Srad", "antar"]) {
            // SradDA, antarDA
            kp.p.run_at(Varttika("3.3.106.1"), kp.i_upapada()?, |_| {});
            true
        } else {
            false
        };
        if upasarge || ok {
            // pradA, upadA, ...
            kp.try_add_with("3.3.106", aN, |p| p.add_tag(PT::Stri));
        }
    }

    // kfti, citi, mati, ...
    // TODO: why not `else` here?
    kp.try_add_with("3.3.94", ktin, |p| p.add_tag(PT::Stri));
    Some(kp.has_krt)
}

/// Runs the rules that add a krt-pratyaya to a given dhatu. Returns whether a pratyaya was added.
pub fn run(p: &mut Prakriya, krt: BaseKrt) -> bool {
    let mut kp = KrtPrakriya::new(p, krt);

    try_add_various_pratyayas(&mut kp);
    if try_add_upapada_krt(&mut kp).unwrap_or(false) {
        return true;
    }

    let mut kp = KrtPrakriya::new(p, krt);
    if try_add_krt(&mut kp).unwrap_or(false) {
        return true;
    }

    let mut kp = KrtPrakriya::new(p, krt);
    try_add_strilinga_pratyayas(&mut kp).unwrap_or(false)
}
