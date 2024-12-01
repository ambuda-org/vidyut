extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;

#[test]
fn sutra_1_4_3() {
    // IkArAnta
    assert_has_sup_1s(&nyap("kumArI"), Stri, &["kumArI"]);
    assert_has_sup_1s(&nyap("gOrI"), Stri, &["gOrI"]);
    assert_has_sup_1s("lakzmI", Stri, &["lakzmIH"]);
    assert_has_sup_1s(&nyap("SArNgaravI"), Stri, &["SArNgaravI"]);
    // UkArAnta
    assert_has_sup_1s("brahmabanDU", Stri, &["brahmabanDUH"]);
    assert_has_sup_1s("yavagU", Stri, &["yavagUH"]);
    // yU?
    assert_has_sup_4s("mAtf", Stri, &["mAtre"]);
    assert_has_sup_4s("duhitf", Stri, &["duhitre"]);
    // stryAKyO
    assert_has_sup_1s("grAmaRI", Pum, &["grAmaRIH"]);
    assert_has_sup_1s("senAnI", Pum, &["senAnIH"]);
    assert_has_sup_1s("KalapU", Pum, &["KalapUH"]);
}

#[test]
fn sutra_1_4_4() {
    let shri = krdanta(&[], &d("SriY", Bhvadi), Krt::kvip);
    assert_has_sup_ss(&shri, Stri, &["SrIH"]);
    assert_has_sup_ss("BrU", Stri, &["BrUH"]);
    // astrI
    assert_has_sup_ss("strI", Stri, &["stri"]);
}

#[test]
fn sutra_1_4_5() {
    let shri = create_krdanta("SrI", &[], &d("SriY", Bhvadi), Krt::kvip);
    assert_has_sup_6p(&shri, Stri, &["SriyAm", "SrIRAm"]);
    assert_has_sup_6p("BrU", Stri, &["BruvAm", "BrURAm"]);
    // astrI
    assert_has_sup_6p("strI", Stri, &["strIRAm"]);
}

#[test]
fn sutra_1_4_6() {
    let shri = create_krdanta("SrI", &[], &d("SriY", Bhvadi), Krt::kvip);

    assert_has_sup_4s("kfti", Stri, &["kftyE", "kftaye"]);
    assert_has_sup_4s("Denu", Stri, &["DenvE", "Denave"]);
    assert_has_sup_4s(&shri, Stri, &["SriyE", "Sriye"]);
    assert_has_sup_4s("BrU", Stri, &["BruvE", "Bruve"]);
    // astrI
    assert_has_sup_4s("strI", Stri, &["striyE"]);
    // stryAKyO
    assert_has_sup_4s("agni", Pum, &["agnaye"]);
    assert_has_sup_4s("vAyu", Pum, &["vAyave"]);
    assert_has_sup_4s("Banu", Pum, &["Banave"]);
}

#[test]
fn sutra_1_4_7() {
    assert_has_sup_4s("agni", Pum, &["agnaye"]);
    assert_has_sup_4s("vAyu", Pum, &["vAyave"]);
    assert_has_sup_4s("kfti", Stri, &["kftyE", "kftaye"]);
    assert_has_sup_4s("Denu", Stri, &["DenvE", "Denave"]);
    // asaKi
    assert_has_sup_3s("saKi", Pum, &["saKyA"]);
    assert_has_sup_4s("saKi", Pum, &["saKye"]);
    assert_has_sup_5s("saKi", Pum, &["saKyuH"]);
    assert_has_sup_7s("saKi", Pum, &["saKyO"]);
}

#[test]
fn sutra_1_4_8() {
    assert_has_sup_3s("pati", Pum, &["patyA"]);
    assert_has_sup_4s("pati", Pum, &["patye"]);
    assert_has_sup_5s("pati", Pum, &["patyuH"]);
    assert_has_sup_6s("pati", Pum, &["patyuH"]);
    assert_has_sup_7s("pati", Pum, &["patyO"]);
    // TODO: test when in samasa.
}

#[test]
fn sutra_1_4_10() {
    assert_has_krdanta(&[], &d("Bi\\di~^r", Rudhadi), Krt::tfc, &["Bettf"]);
    assert_has_krdanta(&[], &d("Ci\\di~^r", Rudhadi), Krt::tfc, &["Cettf"]);
    assert_has_tip(&[], &nic(&d("qukf\\Y", Tanadi)), Lun, &["acIkarat"]);
    assert_has_tip(&[], &nic(&d("hf\\Y", Bhvadi)), Lun, &["ajIharat"]);
}

#[test]
fn sutra_1_4_11() {
    assert_has_krdanta(&[], &d("kuqi~\\", Bhvadi), Krt::a, &["kuRqA"]);
    assert_has_krdanta(&[], &d("huqi~\\", Bhvadi), Krt::a, &["huRqA"]);
    assert_has_krdanta(&[], &d("Sikza~\\", Bhvadi), Krt::a, &["SikzA"]);
    assert_has_krdanta(&[], &d("Bikza~\\", Bhvadi), Krt::a, &["BikzA"]);
}

#[test]
fn sutra_1_4_13() {
    let kf = d("qukf\\Y", Tanadi);
    let hf = d("hf\\Y", Bhvadi);
    assert_has_krdanta(&[], &kf, Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &hf, Krt::tfc, &["hartf"]);
    assert_has_tip(&[], &kf, Lrt, &["karizyati"]);
    assert_has_tip(&[], &hf, Lrt, &["harizyati"]);
    assert_has_tip(&[], &kf, Lrn, &["akarizyat"]);
    assert_has_taddhita("upagu", T::aR, &["Opagava"]);
    assert_has_taddhita("kapawu", T::aR, &["kApawava"]);

    // pratyayagrahana
    assert_has_lan(&["ni"], &d("vi\\Sa~", Tudadi), &["nyaviSata"]);
    assert_has_lan(&["vi"], &d("qukrI\\Y", Kryadi), &["vyakrIRIta"]);

    assert_has_vas(&[], &kf, Lrt, &["karizyAvaH"]);
    assert_has_mas(&[], &kf, Lrt, &["karizyAmaH"]);
    assert_has_sup_1p("kuRqa", Napumsaka, &["kuRqAni"]);
    // TODO: others;
}

#[test]
fn sutra_1_4_14() {
    assert_has_sup_1p("brAhmaRa", Pum, &["brAhmaRAH"]);
}

#[test]
fn sutra_1_4_16() {
    assert_has_taddhita("Bavat", T::Cas, &["BavadIya"]);
    assert_has_taddhita("UrRA", T::yus, &["UrRAyu"]);
    assert_has_taddhita("ftu", T::Gas, &["ftviya"]);
}

#[test]
fn skip_sutra_1_4_17() {}

#[test]
fn skip_sutra_1_4_18() {}

#[test]
fn sutra_1_4_19() {
    assert_has_taddhita("udaSvit", T::matup, &["udaSvitvat"]);
    assert_has_taddhita("vidyut", T::matup, &["vidyutvat"]);
    assert_has_taddhita("payas", T::vini, &["payasvin"]);
    assert_has_taddhita("yaSas", T::vini, &["yaSasvin"]);
    // tasau
    assert_has_taddhita("takzan", T::matup, &["takzavat"]);
}

#[test]
fn sutra_1_4_21() {
    assert_has_sup_1p("brAhmaRa", Pum, &["brAhmaRAH"]);
    assert_has_jhi(&[], &d("paWa~", Bhvadi), Lat, &["paWanti"]);
}

#[test]
fn sutra_1_4_22() {
    let path = d("paWa~", Bhvadi);
    assert_has_sup_1d("brAhmaRa", Pum, &["brAhmaRO"]);
    assert_has_tas(&[], &path, Lat, &["paWataH"]);

    assert_has_sup_1s("brAhmaRa", Pum, &["brAhmaRaH"]);
    assert_has_tip(&[], &path, Lat, &["paWati"]);
}

#[test]
fn skip_sutra_1_4_58() {}

#[test]
fn sutra_1_4_59() {
    let ni = d("RI\\Y", Bhvadi);
    assert_has_tip(&["pra"], &ni, Lat, &["praRayati"]);
    assert_has_tip(&["pari"], &ni, Lat, &["pariRayati"]);
    assert_has_krdanta(&["pra"], &ni, Krt::Rvul, &["praRAyaka"]);
    assert_has_krdanta(&["pari"], &ni, Krt::Rvul, &["pariRAyaka"]);
}

#[ignore]
#[test]
fn sutra_1_4_63() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_upapada_krdanta("sat", &[], &kr, Krt::ktvA, &["satkftya", "sat kftvA"]);
    assert_has_upapada_krdanta("sat", &[], &kr, Krt::kta, &["satkfta"]);
    assert_has_tip(&["sat"], &kr, Lit, &["satkaroti"]);
    assert_has_upapada_krdanta("asat", &[], &kr, Krt::ktvA, &["asatkftya", "asat kftvA"]);
    assert_has_upapada_krdanta("asat", &[], &kr, Krt::kta, &["asatkfta"]);
    assert_has_tip(&["asat"], &kr, Lit, &["asatkaroti"]);
}

#[test]
fn sutra_1_4_60() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&["pra"], &kr, Krt::ktvA, &["prakftya"]);
    assert_has_krdanta(&["pra"], &kr, Krt::kta, &["prakfta"]);
    assert_has_tip(&["pra"], &kr, Lat, &["prakaroti"]);

    assert_has_krdanta(&["pra"], &d("RI\\Y", Bhvadi), Krt::kta, &["praRIta"]);
    assert_has_krdanta(&["aBi"], &d("zi\\ca~^", Tudadi), Krt::kta, &["aBizikta"]);
}

#[test]
fn sutra_1_4_67() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&["puras"], &kr, Krt::ktvA, &["puraskftya"]);
    assert_has_krdanta(&["puras"], &kr, Krt::kta, &["puraskfta"]);
}

#[test]
fn sutra_1_4_68() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_krdanta(&["astam"], &gam, Krt::ktvA, &["astaNgatya", "astaNgamya"]);
    assert_has_krdanta(&["astam"], &gam, Krt::kta, &["astaNgata"]);
}

#[ignore]
#[test]
fn sutra_1_4_69() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_krdanta(&["acCa"], &gam, Krt::ktvA, &["acCagatya", "acCagamya"]);
    assert_has_krdanta(&["acCa"], &gam, Krt::kta, &["acCagata"]);
    assert_has_tip(&["acCa"], &gam, Lat, &["acCagacCati"]);

    let vad = d("vada~", Bhvadi);
    assert_has_krdanta(&["acCa"], &vad, Krt::ktvA, &["acCodya"]);
    assert_has_krdanta(&["acCa"], &vad, Krt::kta, &["acCodita"]);
    assert_has_tip(&["acCa"], &vad, Lat, &["acCavadati"]);
}

#[ignore]
#[test]
fn sutra_1_4_70() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&["adas"], &kr, Krt::ktvA, &["adaHkftya"]);
    assert_has_krdanta(&["adas"], &kr, Krt::kta, &["adaHkfta"]);
    assert_has_tip(&["adas"], &kr, Lat, &["adaHkaroti"]);
}

#[ignore]
#[test]
fn sutra_1_4_74() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&["maDye"], &kr, Krt::ktvA, &["maDyekftya", "maDyekftvA"]);
    assert_has_krdanta(&["pade"], &kr, Krt::ktvA, &["padekftya", "padekftvA"]);
    assert_has_krdanta(
        &["nivacane"],
        &kr,
        Krt::ktvA,
        &["nivacanekftya", "nivacanekftvA"],
    );
}

#[test]
fn sutra_1_4_80() {
    assert_has_lat(&["vi"], &d("liKa~", Tudadi), &["viliKati"]);
    assert_has_tip(
        &["tiras"],
        &d("qukf\\Y", Tanadi),
        Lat,
        &["tiraskaroti", "tiraHkaroti"],
    );
}

#[test]
fn skip_sutra_1_4_99_to_sutra_1_4_104() {}

#[test]
fn sutra_1_4_105() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_sip(&[], &pac, Lat, &["pacasi"]);
    assert_has_thas(&[], &pac, Lat, &["pacaTaH"]);
    assert_has_tha(&[], &pac, Lat, &["pacaTa"]);
}

#[test]
fn sutra_1_4_107() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_mip(&[], &pac, Lat, &["pacAmi"]);
    assert_has_vas(&[], &pac, Lat, &["pacAvaH"]);
    assert_has_mas(&[], &pac, Lat, &["pacAmaH"]);
}

#[test]
fn sutra_1_4_108() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tip(&[], &pac, Lat, &["pacati"]);
    assert_has_tas(&[], &pac, Lat, &["pacataH"]);
    assert_has_jhi(&[], &pac, Lat, &["pacanti"]);
}

#[test]
fn sutra_1_4_109() {
    assert_has_sandhi("daDi", "atra", &["daDyatra"]);
    assert_has_sandhi("maDu", "atra", &["maDvatra"]);
}

#[test]
fn sutra_1_4_110() {
    // TODO: ashtadhyayi.com has daDiM, maDuM -- why?
    assert_has_sup_1s("daDi", Napumsaka, &["daDi"]);
    assert_has_sup_1s("maDu", Napumsaka, &["maDu"]);
    assert_has_sup_1s("vfkza", Pum, &["vfkzaH"]);
    assert_has_sup_1s("plakza", Pum, &["plakzaH"]);
}
