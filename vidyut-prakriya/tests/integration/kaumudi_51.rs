extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn sk_2554() {
    let kri = d("qukrI\\Y", Kryadi);
    assert_has_tip(&[], &kri, Lat, &["krIRAti"]);
    assert_has_jhi(&[], &kri, Lat, &["krIRanti"]);
    assert_has_ta(&[], &kri, Lat, &["krIRIte"]);
    assert_has_aataam(&[], &kri, Lat, &["krIRAte"]);

    assert_has_tip(&[], &kri, Lit, &["cikrAya"]);
    assert_has_tas(&[], &kri, Lit, &["cikriyatuH"]);
    // SK has "cikriyiTa" which is almost certainly a typo.
    assert_has_sip(&[], &kri, Lit, &["cikrayiTa", "cikreTa"]);
    assert_has_vas(&[], &kri, Lit, &["cikriyiva"]);
    assert_has_thaas(&[], &kri, Lit, &["cikriyize"]);

    assert_has_tip(&[], &kri, Lut, &["kretA"]);
    assert_has_tip(&[], &kri, Lrt, &["krezyati"]);
    // krIRAtu is not listed (typo?)
    // krItAt is listed (typo?)
    assert_has_tip(&[], &kri, Lot, &["krIRAtu", "krIRItAt"]);
    assert_has_ta(&[], &kri, AshirLin, &["krezIzwa"]);
    assert_has_tip(&[], &kri, Lun, &["akrEzIt"]);
    assert_has_ta(&[], &kri, Lun, &["akrezwa"]);

    let pri = d("prI\\Y", Kryadi);
    assert_has_tip(&[], &pri, Lat, &["prIRAti"]);
    assert_has_ta(&[], &pri, Lat, &["prIRIte"]);

    let mi = d("mI\\Y", Kryadi);
    assert_has_tip(&["pra"], &mi, Lat, &["pramIRAti"]);
    assert_has_ta(&["pra"], &mi, Lat, &["pramIRIte"]);

    assert_has_tip(&[], &mi, Lit, &["mamO"]);
    assert_has_tas(&[], &mi, Lit, &["mimyatuH"]);
    assert_has_sip(&[], &mi, Lit, &["mamiTa", "mamATa"]);
    assert_has_ta(&[], &mi, Lit, &["mimye"]);
    assert_has_tip(&[], &mi, Lut, &["mAtA"]);
    assert_has_tip(&[], &mi, Lrt, &["mAsyati"]);
    assert_has_tip(&[], &mi, AshirLin, &["mIyAt"]);
    assert_has_ta(&[], &mi, AshirLin, &["mAsIzwa"]);
    assert_has_tip(&[], &mi, Lun, &["amAsIt"]);
    assert_has_tas(&[], &mi, Lun, &["amAsizwAm"]);
    assert_has_ta(&[], &mi, Lun, &["amAsta"]);

    let si = d("zi\\Y", Kryadi);
    assert_has_tip(&[], &si, Lat, &["sinAti"]);
    assert_has_ta(&[], &si, Lat, &["sinIte"]);
    assert_has_tip(&[], &si, Lit, &["sizAya"]);
    assert_has_ta(&[], &si, Lit, &["sizye"]);
    assert_has_tip(&[], &si, Lut, &["setA"]);
}

#[test]
fn sk_2555() {
    let sku = d("sku\\Y", Kryadi);
    assert_has_tip(&[], &sku, Lat, &["skunoti", "skunAti"]);
    assert_has_ta(&[], &sku, Lat, &["skunute", "skunIte"]);
    assert_has_tip(&[], &sku, Lit, &["cuskAva"]);
    assert_has_ta(&[], &sku, Lit, &["cuskuve"]);
    assert_has_tip(&[], &sku, Lut, &["skotA"]);
    assert_has_tip(&[], &sku, Lun, &["askOzIt"]);
    assert_has_ta(&[], &sku, Lun, &["askozwa"]);

    let stambh = d("stanBu~", Kryadi);
    assert_has_tip(&["vi"], &stambh, Lat, &["vizwaBnoti", "vizwaBnAti"]);
    assert_has_tip(
        &["ava"],
        &stambh,
        Lat,
        &["avazwaBnoti", "avazwaBnAti", "avastaBnoti", "avastaBnAti"],
    );
    assert_has_tip(&["ava"], &stambh, Lit, &["avatazwamBa", "avatastamBa"]);
    assert_has_tip(&["vi"], &stambh, Lun, &["vyazwaBat", "vyazwamBIt"]);

    let stumbh = d("stunBu~", Kryadi);
    assert_has_tip(&[], &stumbh, Lat, &["stuBnoti", "stuBnAti"]);
}

#[test]
fn sk_2556() {
    let skambh = d("skanBu~", Kryadi);
    assert_has_tip(&["vi"], &skambh, Lat, &["vizkaBnoti", "vizkaBnAti"]);

    let skumbh = d("skunBu~", Kryadi);
    assert_has_tip(&[], &skumbh, Lat, &["skuBnoti", "skuBnAti"]);
}

#[test]
fn sk_2557() {
    let stambh = d("stanBu~", Kryadi);
    assert_has_sip(
        &[],
        &stambh,
        Lot,
        &["staBAna", "staBnItAt", "staBnuhi", "staBnutAt"],
    );

    let stumbh = d("stunBu~", Kryadi);
    assert_has_sip(
        &[],
        &stumbh,
        Lot,
        &["stuBAna", "stuBnItAt", "stuBnuhi", "stuBnutAt"],
    );

    let skambh = d("skanBu~", Kryadi);
    assert_has_sip(
        &[],
        &skambh,
        Lot,
        &["skaBAna", "skaBnItAt", "skaBnuhi", "skaBnutAt"],
    );

    let skumbh = d("skunBu~", Kryadi);
    assert_has_sip(
        &[],
        &skumbh,
        Lot,
        &["skuBAna", "skuBnItAt", "skuBnuhi", "skuBnutAt"],
    );

    let yu = d("yu\\Y", Kryadi);
    assert_has_tip(&[], &yu, Lat, &["yunAti"]);
    assert_has_ta(&[], &yu, Lat, &["yunIte"]);
    assert_has_tip(&[], &yu, Lut, &["yotA"]);

    let knu = d("knUY", Kryadi);
    assert_has_tip(&[], &knu, Lat, &["knUnAti"]);
    assert_has_ta(&[], &knu, Lat, &["knUnIte"]);
    assert_has_tip(&[], &knu, Lut, &["knavitA"]);

    let dru = d("drUY", Kryadi);
    assert_has_tip(&[], &dru, Lat, &["drURAti"]);
    assert_has_ta(&[], &dru, Lat, &["drURIte"]);
}

#[test]
fn sk_2558() {
    let pu = d("pUY", Kryadi);
    assert_has_tip(&[], &pu, Lat, &["punAti"]);
    assert_has_ta(&[], &pu, Lat, &["punIte"]);
    assert_has_tip(&[], &pu, Lut, &["pavitA"]);

    let lu = d("lUY", Kryadi);
    assert_has_tip(&[], &lu, Lat, &["lunAti"]);
    assert_has_ta(&[], &lu, Lat, &["lunIte"]);

    let str = d("stFY", Kryadi);
    assert_has_ta(&[], &str, Lat, &["stfRIte"]);
    assert_has_tip(&[], &str, Lit, &["tastAra"]);
    assert_has_tas(&[], &str, Lit, &["tastaratuH"]);
    assert_has_tip(&[], &str, Lut, &["staritA", "starItA"]);
    assert_has_tip(&[], &str, VidhiLin, &["stfRIyAt"]);
    assert_has_ta(&[], &str, VidhiLin, &["stfRIta"]);
    assert_has_tip(&[], &str, AshirLin, &["stIryAt"]);
    assert_has_ta(&[], &str, AshirLin, &["starizIzwa", "stIrzIzwa"]);
    assert_has_tip(&[], &str, Lun, &["astArIt"]);
    assert_has_tas(&[], &str, Lun, &["astArizwAm"]);
    assert_has_ta(&[], &str, Lun, &["astarizwa", "astarIzwa", "astIrzwa"]);

    let kr = d("kFY", Kryadi);
    assert_has_tip(&[], &kr, Lat, &["kfRAti"]);
    assert_has_ta(&[], &kr, Lat, &["kfRIte"]);
    assert_has_tip(&[], &kr, Lit, &["cakAra"]);
    assert_has_ta(&[], &kr, Lit, &["cakare"]);

    let vr = d("vFY", Kryadi);
    assert_has_tip(&[], &vr, Lat, &["vfRAti"]);
    assert_has_ta(&[], &vr, Lat, &["vfRIte"]);
    assert_has_tip(&[], &vr, Lit, &["vavAra"]);
    assert_has_ta(&[], &vr, Lit, &["vavare"]);
    assert_has_tip(&[], &vr, Lut, &["varitA", "varItA"]);
    assert_has_ta(&[], &vr, AshirLin, &["varizIzwa", "vUrzIzwa"]);
    assert_has_tas(&[], &vr, Lun, &["avArizwAm"]);
    assert_has_ta(&[], &vr, Lun, &["avarizwa", "avarIzwa", "avUrzwa"]);

    let dhu = d("DUY", Kryadi);
    assert_has_tip(&[], &dhu, Lat, &["DunAti"]);
    assert_has_ta(&[], &dhu, Lat, &["DunIte"]);
    assert_has_sip(&[], &dhu, Lit, &["duDaviTa", "duDoTa"]);
    assert_has_vas(&[], &dhu, Lit, &["duDuviva"]);
    assert_has_tip(&[], &dhu, Lut, &["DotA", "DavitA"]);
    assert_has_tip(&[], &dhu, Lun, &["aDAvIt"]);
    assert_has_ta(&[], &dhu, Lun, &["aDavizwa", "aDozwa"]);

    let shr = d("SF", Kryadi);
    assert_has_tas(&[], &shr, Lit, &["SaSratuH", "SaSaratuH"]);
    assert_has_vas(&[], &shr, Lit, &["SaSariva", "SaSriva"]);
    assert_has_tip(&[], &shr, Lut, &["SaritA", "SarItA"]);
    assert_has_sip(&[], &shr, Lot, &["SfRIhi", "SfRItAt"]);
    assert_has_tip(&[], &shr, AshirLin, &["SIryAt"]);
    assert_has_tas(&[], &shr, Lun, &["aSArizwAm"]);

    let pr = d("pF", Kryadi);
    assert_has_tas(&[], &pr, Lit, &["papratuH", "paparatuH"]);
    assert_has_tip(&[], &pr, AshirLin, &["pUryAt"]);

    let mr = d("mF", Kryadi);
    assert_has_tip(&[], &mr, Lat, &["mfRAti"]);
    assert_has_tip(&[], &mr, Lit, &["mamAra"]);

    let dr = d("dF", Kryadi);
    assert_has_tas(&[], &dr, Lit, &["dadaratuH", "dadratuH"]);

    let r = d("F", Kryadi);
    assert_has_tip(&[], &r, Lat, &["fRAti"]);
    assert_has_tip(&[], &r, Lit, &["arAYcakAra", "arAmbaBUva", "arAmAsa"]);
    assert_has_tip(&[], &r, Lut, &["aritA", "arItA"]);
    assert_has_tip(&[], &r, Lan, &["ArRAt"]);
    assert_has_tas(&[], &r, Lan, &["ArRItAm"]);
    assert_has_tip(&[], &r, AshirLin, &["IryAt"]);
    assert_has_tip(&[], &r, Lun, &["ArIt"]);
    assert_has_tas(&[], &r, Lun, &["ArizwAm"]);
}

#[test]
fn sk_2559() {
    let jya = d("jyA\\", Kryadi);
    assert_has_tip(&[], &jya, Lat, &["jinAti"]);
    assert_has_tip(&[], &jya, Lit, &["jijyO"]);
    assert_has_tas(&[], &jya, Lit, &["jijyatuH"]);

    let li = d("lI\\", Kryadi);
    assert_has_tip(&[], &li, Lit, &["lalO", "lilAya"]);
    assert_has_tip(&[], &li, Lut, &["lAtA", "letA"]);

    let vli = d("vlI\\", Kryadi);
    assert_has_tip(&[], &vli, Lat, &["vlinAti"]);

    let jna = d("jYA\\", Kryadi);
    assert_has_tip(&[], &jna, Lat, &["jAnAti"]);

    let bandh = d("ba\\nDa~", Kryadi);
    assert_has_tip(&[], &bandh, Lat, &["baDnAti"]);
    // babandDa, etc. are supported by other sources.
    assert_has_sip(&[], &bandh, Lit, &["babanDiTa", "babandDa", "babanDa"]);
    assert_has_tip(&[], &bandh, Lut, &["banDA", "bandDA"]);
    assert_has_tas(&[], &bandh, Lut, &["banDArO", "bandDArO"]);
    assert_has_tip(&[], &bandh, Lrt, &["Bantsyati"]);
    assert_has_sip(&[], &bandh, Lot, &["baDAna", "baDnItAt"]);
    assert_has_tip(&[], &bandh, Lun, &["aBAntsIt"]);
    assert_has_tas(&[], &bandh, Lun, &["abAnDAm", "abAndDAm"]);
    assert_has_jhi(&[], &bandh, Lun, &["aBAntsuH"]);

    let vr = d("vfN", Kryadi);
    assert_has_ta(&[], &vr, Lat, &["vfRIte"]);
    assert_has_ta(&[], &vr, Lit, &["vavre"]);
    assert_has_thaas(&[], &vr, Lit, &["vavfze"]);
    assert_has_dhvam(&[], &vr, Lit, &["vavfQve"]);
    assert_has_ta(&[], &vr, Lut, &["varitA", "varItA"]);
    assert_has_ta(&[], &vr, Lun, &["avarizwa", "avarIzwa", "avfta"]);

    let shranth = d("SranTa~", Kryadi);
    assert_has_tip(&[], &shranth, Lat, &["SraTnAti"]);
    // SaSranTatuH and SasranTuH are not supported by KV or SK, but they are allowed by other
    // grammarians.
    assert_has_tas(&[], &shranth, Lit, &["SreTatuH", "SaSranTatuH"]);
    assert_has_jhi(&[], &shranth, Lit, &["SreTuH", "SaSranTuH"]);
    assert_has_sip(&[], &shranth, Lit, &["SaSranTiTa"]);
    // TODO: SaSrATa, SaSraTa, SreTiTa by sudhAkara-mata. But, adding this makes the derivation of
    // SaSrATa unclear.

    let kunth = d("kunTa~", Kryadi);
    assert_has_tip(&[], &kunth, Lat, &["kuTnAti"]);
    assert_has_tip(&[], &kunth, Lit, &["cukunTa"]);

    let kuth = d("kuTa~", Kryadi);
    assert_has_tip(&[], &kuth, Lit, &["cukoTa"]);

    let mrd = d("mfda~", Kryadi);
    assert_has_tip(&[], &mrd, Lat, &["mfdnAti"]);
    assert_has_sip(&[], &mrd, Lot, &["mfdAna", "mfdnItAt"]);

    let mrd = d("mfqa~", Kryadi);
    assert_has_tip(&[], &mrd, Lat, &["mfqRAti"]);

    let gudh = d("guDa~", Kryadi);
    assert_has_tip(&[], &gudh, Lat, &["guDnAti"]);

    let kush = d("kuza~", Kryadi);
    assert_has_tip(&[], &kush, Lat, &["kuzRAti"]);
    assert_has_tip(&[], &kush, Lut, &["kozitA"]);
}

#[ignore]
#[test]
fn sk_2560() {
    let kush = d("kuza~", Kryadi);
    assert_has_tip(&["nir"], &kush, Lut, &["nizkozitA", "nizkozwA"]);
    // aniT -> ksa -> nirakukzat
    assert_has_tip(&["nir"], &kush, Lun, &["nirakozIt", "nirakukzat"]);

    let kshubh = d("kzuBa~", Kryadi);
    assert_has_tip(&[], &kshubh, Lat, &["kzuBnAti"]);
    assert_has_tas(&[], &kshubh, Lat, &["kzuBnItaH"]);
    assert_has_tip(&[], &kshubh, Lut, &["kzoBitA"]);
    assert_has_sip(&[], &kshubh, Lot, &["kzuBAna", "kzuBnItAt"]);

    let nabh = d("RaBa~", Kryadi);
    assert_has_tip(&[], &nabh, Lat, &["naBnAti"]);

    let tubh = d("tuBa~", Kryadi);
    assert_has_tip(&[], &tubh, Lat, &["tuBnAti"]);

    assert_has_ta(&[], &d("RaBa~\\", Bhvadi), Lat, &["naBate"]);
    assert_has_ta(&[], &d("tuBa~\\", Bhvadi), Lat, &["toBate"]);
    assert_has_tip(&[], &d("RaBa~", Divadi), Lat, &["naByati"]);
    assert_has_tip(&[], &d("tuBa~", Divadi), Lat, &["tuByati"]);

    let klish = d("kliSU~", Kryadi);
    assert_has_tip(&[], &klish, Lat, &["kliSnAti"]);
    assert_has_tip(&[], &klish, Lut, &["kleSitA", "klezwA"]);
    assert_has_tip(&[], &klish, Lun, &["akleSIt", "aklikzat"]);

    let ash = d("aSa~", Kryadi);
    assert_has_tip(&[], &ash, Lat, &["aSnAti"]);
    assert_has_tip(&[], &ash, Lit, &["ASa"]);

    let dhras = d("u~Drasa~", Kryadi);
    assert_has_tip(&[], &dhras, Lat, &["DrasnAti"]);

    let x = d("uDrasa~", Kryadi);
    assert_has_tip(
        &[],
        &x,
        Lit,
        &["uDrasAYcakAra", "uDrasAmAsa", "uDrasAmbaBUva"],
    );

    let ish = d("iza~", Kryadi);
    assert_has_tip(&[], &ish, Lat, &["izRAti"]);
    assert_has_tip(&[], &ish, Lut, &["ezitA"]);
    // TODO: iw-vikalpa?

    let x = d("vi\\za~", Kryadi);
    assert_has_tip(&[], &x, Lat, &["vizRAti"]);
    assert_has_tip(&[], &x, Lut, &["vezwA"]);

    let prush = d("pruza~", Kryadi);
    assert_has_tip(&[], &prush, Lat, &["pruzRAti"]);

    let plush = d("pluza~", Kryadi);
    assert_has_tip(&[], &plush, Lat, &["pluzRAti"]);

    let push = d("puza~", Kryadi);
    assert_has_tip(&[], &push, Lut, &["pozitA"]);

    let mush = d("muza~", Kryadi);
    assert_has_tip(&[], &mush, Lut, &["mozitA"]);

    let khac = d("Kaca~", Kryadi);
    assert_has_tip(&[], &khac, Lat, &["KacYAti"]);
}

#[test]
fn sk_2561() {
    let khav = d("Kava~", Kryadi);
    assert_has_tip(&[], &khav, Lat, &["KOnAti"]);
    assert_has_tip(&[], &khav, Lit, &["caKAva"]);
    assert_has_tip(&[], &khav, Lut, &["KavitA"]);
    assert_has_sip(&[], &khav, Lot, &["KOnIhi", "KOnItAt"]);

    let heth = d("heWa~", Kryadi);
    // ashtadhyayi.com text has hiWRAti here.
    assert_has_tip(&[], &heth, Lat, &["heWRAti"]);

    let grah = d("graha~^", Kryadi);
    assert_has_tip(&[], &grah, Lat, &["gfhRAti"]);
    assert_has_ta(&[], &grah, Lat, &["gfhRIte"]);
}

#[test]
fn sk_2562() {
    let grah = d("graha~^", Kryadi);
    assert_has_tip(&[], &grah, Lut, &["grahItA"]);
    assert_has_tip(&[], &grah, AshirLin, &["gfhyAt"]);
    assert_has_ta(&[], &grah, AshirLin, &["grahIzIzwa"]);
    assert_has_tip(&[], &grah, Lun, &["agrahIt"]);
    assert_has_tas(&[], &grah, Lun, &["agrahIzwAm"]);
    assert_has_ta(&[], &grah, Lun, &["agrahIzwa"]);
    assert_has_aataam(&[], &grah, Lun, &["agrahIzAtAm"]);
    assert_has_jha(&[], &grah, Lun, &["agrahIzata"]);
}
