extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_3_1_1() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::tavyat, &["kartavya"]);
    assert_has_krdanta(&[], &kf, Krt::anIyar, &["karaRIya"]);
}

#[test]
fn sutra_3_1_2() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::tavyat, &["kartavya"]);
    // TODO: tEtttirIya
}

#[test]
fn sutra_3_1_5() {
    assert_has_lat(&[], &d("gupa~\\", Bhvadi), &["jugupsate"]);
    assert_has_lat(&[], &d("tija~\\", Bhvadi), &["titikzate"]);
    assert_has_lat(&[], &d("kita~", Bhvadi), &["cikitsati"]);
}

#[test]
fn sutra_3_1_6() {
    assert_has_lat(&[], &d("mAna~\\", Bhvadi), &["mImAMsate"]);
    assert_has_lat(&[], &d("baDa~\\", Bhvadi), &["bIBatsate"]);
    assert_has_lat_a(&[], &d("dAna~^", Bhvadi), &["dIdAMsate"]);
    assert_has_lat_a(&[], &d("SAna~^", Bhvadi), &["SISAMsate"]);

    // TODO: mAnayati, etc.
}

#[test]
fn sutra_3_1_7() {
    let san_d = |u, gana| san(&d(u, gana));
    let kf = san_d("qukf\\Y", Tanadi);
    let hf = san_d("hf\\Y", Bhvadi);

    assert_has_lat_p(&[], &kf, &["cikIrzati"]);
    assert_has_lat_p(&[], &hf, &["jihIrzati"]);
    assert_has_lan_p(&["pra"], &kf, &["prAcikIrzat"]);

    assert_has_lat_p(&[], &san_d("patx~", Bhvadi), &["pipatizati", "pitsati"]);
    assert_has_lat_p(&[], &san_d("mf\\N", Tudadi), &["mumUrzati"]);
}

#[test]
fn sutra_3_1_22() {
    assert_has_lat(&[], &yan(&d("qupa\\ca~^z", Bhvadi)), &["pApacyate"]);
    assert_has_lat(&[], &yan(&d("ya\\ja~^", Bhvadi)), &["yAyajyate"]);
    assert_has_lat(&[], &yan(&d("jvala~", Bhvadi)), &["jAjvalyate"]);
    assert_has_lat(&[], &yan(&d("dIpI~\\", Divadi)), &["dedIpyate"]);
}

#[test]
fn sutra_3_1_22_v1() {
    assert_has_lat(&[], &yan(&d("sUca", Curadi)), &["sosUcyate"]);
    assert_has_lat(&[], &yan(&d("sUtra", Curadi)), &["sosUtryate"]);
    assert_has_lat(&[], &yan(&d("mUtra", Curadi)), &["momUtryate"]);
    assert_has_lat(&[], &yan(&d("awa~", Bhvadi)), &["awAwyate"]);
    assert_has_lat(&[], &yan(&d("f\\", Juhotyadi)), &["arAryate"]);
    assert_has_lat(&[], &yan(&d("aSa~", Kryadi)), &["aSASyate"]);
    assert_has_lat(&["pra"], &yan(&d("UrRuY", Adadi)), &["prorRonUyate"]);
}

#[test]
fn sutra_3_1_23() {
    assert_has_lat(&[], &yan(&d("kramu~", Bhvadi)), &["caNkramyate"]);
    assert_has_lat(&[], &yan(&d("drama~", Bhvadi)), &["dandramyate"]);
}

#[test]
fn sutra_3_1_24() {
    assert_has_lat(&[], &yan(&d("lu\\px~^", Tudadi)), &["lolupyate"]);
    assert_has_lat(&[], &yan(&d("za\\dx~", Bhvadi)), &["sAsadyate"]);
    assert_has_lat(&[], &yan(&d("cara~", Bhvadi)), &["caYcUryate"]);
    assert_has_lat(&[], &yan(&d("japa~", Bhvadi)), &["jaYjapyate"]);
    assert_has_lat(&[], &yan(&d("jaBI~\\", Bhvadi)), &["jaYjaByate"]);
    assert_has_lat(&[], &yan(&d("da\\ha~", Bhvadi)), &["dandahyate"]);
    assert_has_lat(&[], &yan(&d("da\\nSa~", Bhvadi)), &["dandaSyate"]);
    assert_has_lat(&["ni"], &yan(&d("gF", Tudadi)), &["nijegilyate"]);
}

#[test]
fn sutra_3_1_25() {
    assert_has_lat_p(&[], &d("cura~", Curadi), &["corayati"]);
    assert_has_lat_p(&[], &d("citi~", Curadi), &["cintayati"]);
}

#[test]
fn sutra_3_1_26() {
    let san = |u, gana| d(u, gana).with_sanadi(&[Sanadi::Nic]);
    assert_has_lat_p(&[], &san("qukf\\Y", Tanadi), &["kArayati"]);
    assert_has_lat_p(&[], &san("qupa\\ca~^z", Bhvadi), &["pAcayati"]);

    // TODO: add others
}

#[test]
fn sutra_3_1_28() {
    let pan = &d("paRa~\\", Bhvadi);
    assert_has_lat_p(&[], &d("gupU~", Bhvadi), &["gopAyati"]);
    assert_has_lat_p(&[], &d("DUpa~", Bhvadi), &["DUpAyati"]);
    assert_has_lat_p(&[], &d("viCa~", Tudadi), &["vicCAyati"]);
    assert_has_lat_p(&[], &pan, &["paRAyati"]);
    assert_has_lat_p(&[], &d("pana~\\", Bhvadi), &["panAyati"]);
    assert_has_lat_a(&[], &pan, &["paRate"]);
}

#[test]
fn sutra_3_1_29() {
    assert_has_lat_a(&[], &d("fti", Bhvadi), &["ftIyate"]);
}

#[test]
fn sutra_3_1_30() {
    assert_has_lat_a(&[], &d("kamu~\\", Bhvadi), &["kAmayate"]);
}

#[test]
fn sutra_3_1_31() {
    let assert_has_tfc = |u, gana, expected| {
        assert_has_krdanta(&[], &d(u, gana), Krt::tfc, expected);
    };
    assert_has_tfc("gupU~", Bhvadi, &["goptf", "gopitf", "gopAyitf"]);
    assert_has_tfc("fti", Bhvadi, &["artitf", "ftIyitf"]);
    assert_has_tfc("kamu~\\", Bhvadi, &["kamitf", "kAmayitf"]);
}

#[test]
fn sutra_3_1_33() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_lrt_p(&[], &kf, &["karizyati"]);
    assert_has_lrn_p(&[], &kf, &["akarizyat"]);
    assert_has_lut(&[], &kf, &["kartA"]);
    assert_has_lut(&[], &d("ma\\na~\\", Divadi), &["mantA"]);
    assert_has_lut(&["sam"], &d("ga\\mx~", Bhvadi), &["saNgantA"]);
}

#[test]
fn sutra_3_1_35() {
    assert_has_lit(
        &[],
        &d("kAsf~\\", Bhvadi),
        &["kAsAYcakre", "kAsAmAsa", "kAsAmbaBUva"],
    );
    assert_has_lit(
        &[],
        &yan(&d("lUY", Kryadi)),
        &["lolUyAYcakre", "lolUyAmAsa", "lolUyAmbaBUva"],
    );
    // TODO: amantre
}
#[test]
fn sutra_3_1_36() {
    assert_has_lit(
        &[],
        &d("Iha~\\", Bhvadi),
        &["IhAYcakre", "IhAmAsa", "IhAmbaBUva"],
    );
    assert_has_lit(
        &[],
        &d("Uha~\\", Bhvadi),
        &["UhAYcakre", "UhAmAsa", "UhAmbaBUva"],
    );

    // ijAdeH?
    assert_has_tip(&[], &d("takzU~", Bhvadi), Lit, &["tatakza"]);
    assert_has_tip(&[], &d("rakza~", Bhvadi), Lit, &["rarakza"]);
    // gurumataH
    assert_has_mip(&[], &d("ya\\ja~^", Bhvadi), Lit, &["iyaja", "iyAja"]);
    assert_has_mip(&[], &d("quva\\pa~^", Bhvadi), Lit, &["uvapa", "uvApa"]);
    // anfcCaH
    assert_has_tip(&[], &d("fCa~", Tudadi), Lit, &["AnarcCa", "AnarCa"]);
    assert_has_tas(&[], &d("fCa~", Tudadi), Lit, &["AnarcCatuH", "AnarCatuH"]);
    assert_has_jhi(&[], &d("fCa~", Tudadi), Lit, &["AnarcCuH", "AnarCuH"]);
}

#[test]
fn sutra_3_1_36_v1() {
    assert_has_tip(&["pra"], &d("UrRuY", Adadi), Lit, &["prorRunAva"]);
}

#[test]
fn sutra_3_1_37() {
    assert_has_lit(
        &[],
        &d("daya~\\", Bhvadi),
        &["dayAYcakre", "dayAmAsa", "dayAmbaBUva"],
    );
    assert_has_lit(
        &[],
        &d("aya~\\", Bhvadi),
        &["ayAYcakre", "ayAmAsa", "ayAmbaBUva"],
    );
    assert_has_lit(
        &[],
        &d("Asa~\\", Adadi),
        &["AsAYcakre", "AsAmAsa", "AsAmbaBUva"],
    );
}

#[test]
fn sutra_3_1_38() {
    assert_has_lit(
        &[],
        &d("uza~", Bhvadi),
        &["ozAYcakAra", "ozAmAsa", "ozAmbaBUva", "uvoza"],
    );
    assert_has_lit(
        &[],
        &d("vida~", Adadi),
        &["vidAYcakAra", "vidAmAsa", "vidAmbaBUva", "viveda"],
    );
    assert_has_lit(
        &[],
        &d("jAgf", Adadi),
        &["jAgarAYcakAra", "jAgarAmAsa", "jAgarAmbaBUva", "jajAgAra"],
    );
}

#[test]
fn sutra_3_1_39() {
    assert_has_lit(
        &[],
        &d("YiBI\\", Juhotyadi),
        &["biBayAYcakAra", "biBayAmAsa", "biBayAmbaBUva", "biBAya"],
    );
    assert_has_lit(
        &[],
        &d("hrI\\", Juhotyadi),
        &["jihrayAYcakAra", "jihrayAmAsa", "jihrayAmbaBUva", "jihrAya"],
    );
    assert_has_tip(
        &[],
        &d("quBf\\Y", Juhotyadi),
        Lit,
        &["biBarAYcakAra", "biBarAmAsa", "biBarAmbaBUva", "baBAra"],
    );
    assert_has_lit(
        &[],
        &d("hu\\", Juhotyadi),
        &["juhavAYcakAra", "juhavAmAsa", "juhavAmbaBUva", "juhAva"],
    );
}

#[test]
fn sutra_3_1_40() {
    assert_has_tip(
        &[],
        &nic(&d("qupa\\ca~^z", Bhvadi)),
        Lit,
        &["pAcayAYcakAra", "pAcayAmAsa", "pAcayAmbaBUva"],
    );
}

#[test]
fn sutra_3_1_41() {
    let vid = d("vida~", Adadi);
    assert_has_jhi(&[], &vid, Lot, &["vidANkurvantu", "vidantu"]);
    // Also include other purusha+vacana combinations.
    assert_has_tip(
        &[],
        &vid,
        Lot,
        &["vidANkarotu", "vidANkurutAt", "vettu", "vittAt"],
    );
    assert_has_tas(&[], &vid, Lot, &["vidANkurutAm", "vittAm"]);
    assert_has_sip(
        &[],
        &vid,
        Lot,
        &["vidANkuru", "vidANkurutAt", "vidDi", "vittAt"],
    );
    assert_has_thas(&[], &vid, Lot, &["vidANkurutam", "vittam"]);
}

#[test]
fn sutra_3_1_44() {
    assert_has_lun_p(&[], &d("qukf\\Y", Tanadi), &["akArzIt"]);
    assert_has_lun_p(&[], &d("hf\\Y", Bhvadi), &["ahArzIt"]);
}

#[test]
fn sutra_3_1_44_v1() {
    assert_has_lun_p(
        &[],
        &d("spf\\Sa~", Tudadi),
        &["asprAkzIt", "aspArkzIt", "aspfkzat"],
    );
    assert_has_lun_p(
        &[],
        &d("mf\\Sa~", Tudadi),
        &["amrAkzIt", "amArkzIt", "amfkzat"],
    );
    assert_has_lun_p(
        &[],
        &d("kf\\za~", Bhvadi),
        &["akArkzIt", "akrAkzIt", "akfkzat"],
    );
    assert_has_lun_p(
        &[],
        &d("tf\\pa~", Divadi),
        &["atrApsIt", "atArpsIt", "atfpat", "atarpIt"],
    );
    assert_has_lun_p(
        &[],
        &d("df\\pa~", Divadi),
        &["adrApsIt", "adArpsIt", "adfpat", "adarpIt"],
    );
}

#[test]
fn sutra_3_1_45() {
    assert_has_lun_p(&[], &d("du\\ha~^", Adadi), &["aDukzat"]);
    assert_has_lun_p(&[], &d("li\\ha~^", Adadi), &["alikzat"]);
    // SalaH
    assert_has_lun_p(&[], &d("Bi\\di~^r", Rudhadi), &["aBEtsIt", "aBidat"]);
    assert_has_lun_p(&[], &d("Ci\\di~^r", Rudhadi), &["acCEtsIt", "acCidat"]);
    // igupaDAt
    assert_has_lun_p(&[], &d("da\\ha~", Bhvadi), &["aDAkzIt"]);
    // aniwaH
    assert_has_lun_p(&[], &d("kuza~", Kryadi), &["akozIt"]);
    assert_has_lun_p(&[], &d("muza~", Kryadi), &["amozIt"]);
}

#[test]
fn sutra_3_1_46() {
    assert_has_lun_p(&["AN"], &d("Sli\\za~", Divadi), &["ASlikzat", "ASlizat"]);
}

#[test]
fn sutra_3_1_47() {
    assert_has_lun_p(&[], &d("df\\Si~r", Bhvadi), &["adarSat", "adrAkzIt"]);
}

#[test]
fn sutra_3_1_48() {
    assert_has_lun_p(&[], &nic(&d("qukf\\Y", Tanadi)), &["acIkarat"]);
    assert_has_lun_p(&[], &nic(&d("hf\\Y", Bhvadi)), &["ajIharat"]);
    assert_has_lun_p(&[], &d("SriY", Bhvadi), &["aSiSriyat"]);
    assert_has_lun_p(&[], &d("dru\\", Bhvadi), &["adudruvat"]);
    assert_has_lun_p(&[], &d("sru\\", Bhvadi), &["asusruvat"]);
}

#[test]
fn sutra_3_1_49() {
    assert_has_lun_p(&[], &d("De\\w", Bhvadi), &["adaDat", "aDAt", "aDAsIt"]);
    assert_has_lun_p(
        &[],
        &d("wuo~Svi", Bhvadi),
        &["aSiSviyat", "aSvat", "aSvayIt"],
    );
}

#[test]
fn sutra_3_1_52() {
    let asu = d("asu~", Divadi);
    assert_has_ta(&["pari"], &asu, Lun, &["paryAsTata"]);
    assert_has_aataam(&["pari"], &asu, Lun, &["paryAsTetAm"]);
    assert_has_jha(&["pari"], &asu, Lun, &["paryAsTanta"]);

    let vac = d("va\\ca~", Adadi);
    assert_has_tip(&[], &vac, Lun, &["avocat"]);
    assert_has_tas(&[], &vac, Lun, &["avocatAm"]);
    assert_has_jhi(&[], &vac, Lun, &["avocan"]);

    let khya = d("KyA\\", Adadi);
    assert_has_tip(&[], &khya, Lun, &["aKyat"]);
    assert_has_tas(&[], &khya, Lun, &["aKyatAm"]);
    assert_has_jhi(&[], &khya, Lun, &["aKyan"]);
}

#[test]
fn sutra_3_1_53() {
    assert_has_lun_p(&[], &d("li\\pa~^", Tudadi), &["alipat"]);
    assert_has_lun_p(&[], &d("zi\\ca~^", Tudadi), &["asicat"]);
    assert_has_lun_p(&[], &d("hve\\Y", Bhvadi), &["ahvat"]);
}

#[test]
fn sutra_3_1_54() {
    assert_has_lun_a(&[], &d("li\\pa~^", Tudadi), &["alipata", "alipta"]);
    assert_has_lun_a(&[], &d("zi\\ca~^", Tudadi), &["asicata", "asikta"]);
    assert_has_lun_a(&[], &d("hve\\Y", Bhvadi), &["ahvata", "ahvAsta"]);
}

#[test]
fn sutra_3_1_55() {
    assert_has_lun_p(&[], &d("pu\\za~", Divadi), &["apuzat"]);
    assert_has_lun_p(&[], &d("dyuta~\\", Bhvadi), &["adyutat"]);
    assert_has_lun_p(&[], &d("SvitA~\\", Bhvadi), &["aSvitat"]);
    assert_has_lun_p(&[], &d("ga\\mx~", Bhvadi), &["agamat"]);
    assert_has_lun_p(&[], &d("Sa\\kx~", Svadi), &["aSakat"]);
    // parasmEpadezu
    assert_has_lun_a(&["vi"], &d("dyuta~\\", Bhvadi), &["vyadyotizwa"]);
    assert_has_lun_a(&[], &d("luwa~\\", Bhvadi), &["alowizwa"]);
}

#[test]
fn sutra_3_1_56() {
    assert_has_lun_p(&[], &d("sf\\", Juhotyadi), &["asarat"]);
    assert_has_lun_p(&[], &d("SAsu~", Adadi), &["aSizat"]);
    assert_has_lun_p(&[], &d("f\\", Juhotyadi), &["Arat"]);
    assert_has_lun_a(&["sam"], &d("f\\", Juhotyadi), &["samArata"]);
}

#[test]
fn sutra_3_1_57() {
    assert_has_lun_p(&[], &d("Bi\\di~^r", Rudhadi), &["aBidat", "aBEtsIt"]);
    assert_has_lun_p(&[], &d("Ci\\di~^r", Rudhadi), &["acCidat", "acCEtsIt"]);
    // parasmEpadezu
    assert_has_lun_a(&[], &d("Bi\\di~^r", Rudhadi), &["aBitta"]);
    assert_has_lun_a(&[], &d("Ci\\di~^r", Rudhadi), &["acCitta"]);
}

#[test]
fn sutra_3_1_58() {
    assert_has_lun_p(&[], &d("jF", Kryadi), &["ajarat", "ajArIt"]);
    assert_has_lun_p(&[], &d("stanBu~", Kryadi), &["astaBat", "astamBIt"]);
    assert_has_lun_p(&[], &d("mrucu~", Bhvadi), &["amrucat", "amrocIt"]);
    assert_has_lun_p(&[], &d("mlucu~", Bhvadi), &["amlucat", "amlocIt"]);
    assert_has_lun_p(&[], &d("grucu~", Bhvadi), &["agrucat", "agrocIt"]);
    assert_has_lun_p(&[], &d("glucu~", Bhvadi), &["aglucat", "aglocIt"]);
    assert_has_lun_p(&[], &d("gluncu~", Bhvadi), &["aglucat", "agluYcIt"]);
    assert_has_lun_p(
        &[],
        &d("wuo~Svi", Bhvadi),
        &["aSvat", "aSvayIt", "aSiSviyat"],
    );
}

#[test]
fn sutra_3_1_60() {
    let pad = d("pa\\da~\\", Divadi);
    assert_has_ta(&["ud"], &pad, Lun, &["udapAdi"]);
    assert_has_ta(&["sam"], &pad, Lun, &["samapAdi"]);
    // te?
    assert_has_aataam(&["ud"], &pad, Lun, &["udapatsAtAm"]);
    assert_has_jha(&["ud"], &pad, Lun, &["udapatsata"]);
}

#[test]
fn sutra_3_1_61() {
    assert_has_lun_karmani(&[], &d("dIpI~\\", Divadi), &["adIpi", "adIpizwa"]);
    assert_has_lun_karmani(&[], &d("janI~\\", Divadi), &["ajani", "ajanizwa"]);
    assert_has_lun_karmani(&[], &d("buDa~", Bhvadi), &["aboDi", "aboDizwa"]);
    assert_has_lun_karmani(&[], &d("pUrI~\\", Divadi), &["apUri", "apUrizwa"]);
    assert_has_lun_karmani(&[], &d("tAyf~\\", Bhvadi), &["atAyi", "atAyizwa"]);
    assert_has_lun_karmani(&[], &d("o~pyAyI~\\", Bhvadi), &["apyAyi", "apyAyizwa"]);
}

#[test]
fn sutra_3_1_66() {
    assert_has_lun_karmani(&[], &d("SE\\", Bhvadi), &["aSAyi"]);
    assert_has_lun_karmani(&[], &d("qukf\\Y", Tanadi), &["akAri"]);
    assert_has_lun_karmani(&[], &d("hf\\Y", Bhvadi), &["ahAri"]);
}

#[test]
fn sutra_3_1_67() {
    assert_has_lat_karmani(&[], &d("Asa~\\", Adadi), &["Asyate"]);
    assert_has_lat_karmani(&[], &d("SIN", Adadi), &["Sayyate"]);
    assert_has_lat_karmani(&[], &d("qukf\\Y", Tanadi), &["kriyate"]);
    assert_has_lat_karmani(&[], &d("ga\\mx~", Bhvadi), &["gamyate"]);
    assert_has_lat_karmani(&[], &d("qupa\\ca~^z", Bhvadi), &["pacyate"]);
}

#[test]
fn sutra_3_1_68() {
    assert_has_lat(&[], &d("BU", Bhvadi), &["Bavati"]);
    assert_has_lat(&[], &d("qupa\\ca~^z", Bhvadi), &["pacati", "pacate"]);
}

#[test]
fn sutra_3_1_69() {
    assert_has_lat(&[], &d("divu~", Divadi), &["dIvyati"]);
    assert_has_lat(&[], &d("zivu~", Divadi), &["sIvyati"]);
}

#[test]
fn sutra_3_1_70() {
    assert_has_lat(&[], &d("wuBrASf~\\", Bhvadi), &["BrASyate", "BrASate"]);
    assert_has_lat(&[], &d("wuBlASf~\\", Bhvadi), &["BlASyate", "BlASate"]);
    assert_has_lat(&[], &d("Bramu~", Divadi), &["BrAmyati", "Bramati"]);
    assert_has_lat_p(&[], &d("kramu~", Bhvadi), &["krAmyati", "krAmati"]);
    assert_has_lat(&[], &d("klamu~", Divadi), &["klAmyati", "klAmati"]);
    assert_has_lat(&[], &d("trasI~", Divadi), &["trasyati", "trasati"]);
    assert_has_lat(&[], &d("truwa~", Tudadi), &["truwyati", "truwati"]);
    assert_has_lat_p(&[], &d("laza~^", Bhvadi), &["lazyati", "lazati"]);
}

#[test]
fn sutra_3_1_71() {
    let yas = d("yasu~", Divadi);
    assert_has_lat(&[], &yas, &["yasyati", "yasati"]);
    assert_has_lat(&["AN"], &yas, &["Ayasyati"]);
    assert_has_lat(&["pra"], &yas, &["prayasyati"]);
}

#[test]
fn sutra_3_1_72() {
    let yas = d("yasu~", Divadi);
    assert_has_lat(&["sam"], &yas, &["saMyasyati", "saMyasati"]);
}

#[test]
fn sutra_3_1_73() {
    assert_has_lat_p(&[], &d("zu\\Y", Svadi), &["sunoti"]);
    assert_has_lat_p(&[], &d("zi\\Y", Svadi), &["sinoti"]);
}

#[test]
fn sutra_3_1_74() {
    assert_has_lat_p(&[], &d("Sru\\", Bhvadi), &["SfRoti"]);
}

#[test]
fn sutra_3_1_75() {
    assert_has_lat_p(&[], &d("akzU~", Bhvadi), &["akzRoti", "akzati"]);
}

#[test]
fn sutra_3_1_76() {
    assert_has_lat_p(&[], &d("takzU~", Bhvadi), &["takzRoti", "takzati"]);
}

#[test]
fn sutra_3_1_77() {
    assert_has_lat_p(&[], &d("tu\\da~^", Tudadi), &["tudati"]);
    assert_has_lat_p(&[], &d("Ru\\da~^", Tudadi), &["nudati"]);
}

#[test]
fn sutra_3_1_78() {
    assert_has_lat_p(&[], &d("ru\\Di~^r", Rudhadi), &["ruRadDi"]);
    assert_has_lat_p(&[], &d("Bi\\di~^r", Rudhadi), &["Binatti"]);
}

#[test]
fn sutra_3_1_79() {
    assert_has_lat_p(&[], &d("tanu~^", Tanadi), &["tanoti"]);
    assert_has_lat_p(&[], &d("zaRu~^", Tanadi), &["sanoti"]);
    assert_has_lat_p(&[], &d("kzaRu~^", Tanadi), &["kzaRoti"]);
    assert_has_lat_p(&[], &d("qukf\\Y", Tanadi), &["karoti"]);

    assert_has_lun_a(&[], &d("qukf\\Y", Tanadi), &["akfta"]);
}

#[test]
fn sutra_3_1_80() {
    assert_has_lat(&[], &d("Divi~", Bhvadi), &["Dinoti"]);
    assert_has_lat(&[], &d("kfvi~", Bhvadi), &["kfRoti"]);
}

#[test]
fn sutra_3_1_81() {
    assert_has_lat_p(&[], &d("qukfI\\Y", Kryadi), &["krIRAti"]);
    assert_has_lat_p(&[], &d("prI\\Y", Kryadi), &["prIRAti"]);
}

#[test]
fn sutra_3_1_82() {
    assert_has_lat(&[], &d("stanBu~", Kryadi), &["staBnAti", "staBnoti"]);
    assert_has_lat(&[], &d("stunBu~", Kryadi), &["stuBnAti", "stuBnoti"]);
    assert_has_lat(&[], &d("skanBu~", Kryadi), &["skaBnAti", "skaBnoti"]);
    assert_has_lat(&[], &d("skunBu~", Kryadi), &["skuBnAti", "skuBnoti"]);
    assert_has_lat_p(&[], &d("sku\\Y", Kryadi), &["skunAti", "skunoti"]);
}

#[test]
fn sutra_3_1_83() {
    let muz = d("muza~", Kryadi);
    assert_has_sip(&[], &muz, Lot, &["muzARa", "muzRItAt"]);
    assert_has_sip(&[], &d("puza~", Kryadi), Lot, &["puzARa", "puzRItAt"]);
    // halaH
    assert_has_sip(&[], &d("qukrI\\Y", Kryadi), Lot, &["krIRIhi", "krIRItAt"]);
    // hO
    assert_has_tip(&[], &muz, Lat, &["muzRAti"]);
}

#[test]
fn sutra_3_1_91() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::tavya, &["kartavya"]);
    assert_has_krdanta(&[], &kf, Krt::anIyar, &["karaRIya"]);
}

#[test]
fn sutra_3_1_92() {
    // example: 3.2.1
    let kf = d("qukf\\Y", Tanadi);
    assert_has_upapada_krdanta("kumBa", &[], &kf, Krt::aR, &["kumBakAra"]);

    // TODO: stamberama, karRejapa

    // example: 3.2.97
    let jan = d("janI~\\", Divadi);
    assert_has_upapada_krdanta("upasara", &[], &jan, Krt::qa, &["upasaraja"]);
    assert_has_upapada_krdanta("mandura", &[], &jan, Krt::qa, &["manduraja"]);
}

#[test]
fn sutra_3_1_93() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::tavya, &["kartavya"]);
    assert_has_krdanta(&[], &kf, Krt::anIyar, &["karaRIya"]);
    assert_has_ashirlin_p(&[], &d("ci\\Y", Svadi), &["cIyAt"]);
    assert_has_ashirlin_p(&[], &d("zwu\\Y", Adadi), &["stUyAt"]);
}

// krt-pratyayas
// -------------

#[test]
fn sutra_3_1_94() {
    let kzip = d("kzi\\pa~^", Tudadi);
    assert_has_krdanta(&["vi"], &kzip, Krt::Rvul, &["vikzepaka"]);
    assert_has_krdanta(&["vi"], &kzip, Krt::tfc, &["vikzeptf"]);
    assert_has_krdanta(&["vi"], &kzip, Krt::ka, &["vikzipa"]);

    // asarUpa?
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_upapada_krdanta("go", &[], &daa, Krt::ka, &["goda"]);
    assert_has_upapada_krdanta("kambala", &[], &daa, Krt::ka, &["kambalada"]);

    // striyAm?
    assert_has_krdanta(&[], &san(&d("qukf\\Y", Tanadi)), Krt::a, &["cikIrzA"]);
    assert_has_krdanta(&[], &san(&d("hf\\Y", Bhvadi)), Krt::a, &["jihIrzA"]);
}

#[test]
fn sutra_3_1_95() {
    // No examples from KV.
}

#[test]
fn sutra_3_1_96() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::tavyat, &["kartavya"]);
    assert_has_krdanta(&[], &kf, Krt::tavya, &["kartavya"]);
    assert_has_krdanta(&[], &kf, Krt::anIyar, &["karaRIya"]);
}

#[test]
fn sutra_3_1_96_v1() {
    let vas = d("va\\sa~", Bhvadi);
    assert_has_krdanta(&[], &vas, Krt::tavyat, &["vastavya", "vAstavya"]);
}

#[test]
fn sutra_3_1_96_v2() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::kelimar, &["pacelima"]);
    assert_has_krdanta(&[], &d("Bi\\di~^r", Rudhadi), Krt::kelimar, &["Bidelima"]);
}

#[test]
fn sutra_3_1_97() {
    assert_has_krdanta(&[], &d("gE\\", Bhvadi), Krt::yat, &["geya"]);
    assert_has_krdanta(&[], &d("pA\\", Bhvadi), Krt::yat, &["peya"]);
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::yat, &["ceya"]);
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Krt::yat, &["jeya"]);

    // ajanta-bhUta-pUrva
    assert_has_krdanta(&[], &san(&d("do\\", Divadi)), Krt::yat, &["ditsya"]);
    assert_has_krdanta(&[], &san(&d("quDA\\Y", Juhotyadi)), Krt::yat, &["Ditsya"]);
}

#[test]
fn sutra_3_1_97_v1() {
    // TODO: not sure which "tak" or "Sas"
    assert_has_krdanta(&[], &d("taka~", Bhvadi), Krt::yat, &["takya"]);
    assert_has_krdanta(&[], &d("Sasu~\\", Bhvadi), Krt::yat, &["Sasya"]);
    assert_has_krdanta(&[], &d("cate~^", Bhvadi), Krt::yat, &["catya"]);
    assert_has_krdanta(&[], &d("yatI~\\", Bhvadi), Krt::yat, &["yatya"]);
    assert_has_krdanta(&[], &d("janI~\\", Divadi), Krt::yat, &["janya"]);
}

#[test]
fn sutra_3_1_97_v2() {
    let han = d("ha\\na~", Adadi);
    assert_has_krdanta(&[], &han, Krt::yat, &["vaDya"]);
    assert_has_krdanta(&[], &han, Krt::Ryat, &["GAtya"]);
}

#[test]
fn sutra_3_1_98() {
    assert_has_krdanta(&[], &d("Sa\\pa~^", Bhvadi), Krt::yat, &["Sapya"]);
    assert_has_krdanta(&[], &d("qula\\Ba~^z", Bhvadi), Krt::yat, &["laBya"]);
    // poH
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::yat, &[]);
    assert_has_krdanta(&[], &pac, Krt::Ryat, &["pAkya", "pAcya"]);
    let vac = d("va\\ca~", Adadi);
    assert_has_krdanta(&[], &vac, Krt::yat, &[]);
    assert_has_krdanta(&[], &vac, Krt::Ryat, &["vAkya", "vAcya"]);
    // ad-upaDAt
    let kup = d("kupa~", Divadi);
    assert_has_krdanta(&[], &kup, Krt::yat, &[]);
    assert_has_krdanta(&[], &kup, Krt::Ryat, &["kopya"]);
    let gup = d("gupa~", Divadi);
    assert_has_krdanta(&[], &gup, Krt::yat, &[]);
    assert_has_krdanta(&[], &gup, Krt::Ryat, &["gopya"]);
    let aap = d("A\\px~", Svadi);
    assert_has_krdanta(&[], &aap, Krt::yat, &[]);
    assert_has_krdanta(&[], &aap, Krt::Ryat, &["Apya"]);
}

#[test]
fn sutra_3_1_99() {
    assert_has_krdanta(&[], &d("Sa\\kx~", Svadi), Krt::yat, &["Sakya"]);
    assert_has_krdanta(&[], &d("zaha~\\", Bhvadi), Krt::yat, &["sahya"]);
}

#[test]
fn sutra_3_1_100() {
    let gad = d("gada~", Bhvadi);
    let mad = d("madI~", Divadi);
    let car = d("cara~", Bhvadi);
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_krdanta(&[], &gad, Krt::yat, &["gadya"]);
    assert_has_krdanta(&[], &mad, Krt::yat, &["madya"]);
    assert_has_krdanta(&[], &car, Krt::yat, &["carya"]);
    assert_has_krdanta(&[], &yam, Krt::yat, &["yamya"]);
    assert_has_krdanta(&["pra"], &gad, Krt::yat, &[]);
    assert_has_krdanta(&["pra"], &gad, Krt::Ryat, &["pragAdya"]);
    assert_has_krdanta(&["pra"], &mad, Krt::yat, &[]);
    assert_has_krdanta(&["pra"], &mad, Krt::Ryat, &["pramAdya"]);
}

#[test]
fn sutra_3_1_100_v1() {
    let car = d("cara~", Bhvadi);
    assert_has_krdanta(&["AN"], &car, Krt::yat, &["Acarya"]);
    assert_has_krdanta(&["AN"], &car, Krt::Ryat, &["AcArya"]);
}

#[test]
fn sutra_3_1_106() {
    let vad = d("vada~", Bhvadi);
    assert_has_upapada_krdanta("brahma", &[], &vad, Krt::yat, &["brahmavadya"]);
    assert_has_upapada_krdanta("brahma", &[], &vad, Krt::kyap, &["brahmodya"]);
}

#[test]
fn sutra_3_1_107() {
    let bhu = d("BU", Bhvadi);
    assert_has_upapada_krdanta("brahma", &[], &bhu, Krt::kyap, &["brahmaBUya"]);
    assert_has_upapada_krdanta("deva", &[], &bhu, Krt::kyap, &["devaBUya"]);
    // supi?
    assert_has_krdanta(&[], &bhu, Krt::yat, &["Bavya"]);
    // an-upasarge?
    assert_has_krdanta(&["pra"], &bhu, Krt::yat, &["praBavya"]);
}

#[ignore]
#[test]
fn sutra_3_1_108() {
    let han = d("ha\\na~", Adadi);
    assert_has_upapada_krdanta("brahma", &[], &han, Krt::kyap, &["brahmahatya"]);
    assert_has_upapada_krdanta("aSva", &[], &han, Krt::kyap, &["aSvahatya"]);
    // supi?
    assert_has_krdanta(&[], &han, Krt::GaY, &["GAta"]);
    assert_has_krdanta(&[], &han, Krt::Ryat, &[]);
}

#[test]
fn sutra_3_1_109() {
    assert_has_krdanta(&[], &d("i\\R", Adadi), Krt::kyap, &["itya"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), Krt::kyap, &["stutya"]);
    assert_has_krdanta(&[], &d("SAsu~", Adadi), Krt::kyap, &["Sizya"]);
    assert_has_krdanta(&[], &d("vfY", Svadi), Krt::kyap, &["vftya"]);
    assert_has_krdanta(&["AN"], &d("df", Svadi), Krt::kyap, &["Adftya"]);
    assert_has_krdanta(&[], &d("juzI~\\", Tudadi), Krt::kyap, &["juzya"]);

    // blocks Ryat
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), Krt::Ryat, &[]);

    // vfY, not vfN
    let vrnite = &d("vfN", Kryadi);
    assert_has_krdanta(&[], &vrnite, Krt::kyap, &[]);
    assert_has_krdanta(&[], &vrnite, Krt::Ryat, &["vArya"]);
}

#[test]
fn sutra_3_1_110() {
    assert_has_krdanta(&[], &d("vftu~\\", Bhvadi), Krt::kyap, &["vftya"]);
    assert_has_krdanta(&[], &d("vfDu~\\", Bhvadi), Krt::kyap, &["vfDya"]);
    // a-kxpi-cfteh?
    assert_has_krdanta(&[], &d("kfpU~\\", Bhvadi), Krt::kyap, &[]);
    assert_has_krdanta(&[], &d("cftI~", Tudadi), Krt::kyap, &[]);
}

#[test]
fn sutra_3_1_111() {
    assert_has_krdanta(&[], &d("Kanu~^", Bhvadi), Krt::kyap, &["Keya"]);
}

#[test]
fn sutra_3_1_112() {
    let bhf = d("Bf\\Y", Bhvadi);
    assert_has_krdanta(&[], &bhf, Krt::kyap, &["Bftya"]);
    assert_has_krdanta(&[], &bhf, Krt::Ryat, &["BArya"]);
}

#[test]
fn sutra_3_1_113() {
    let mfj = d("mfjU~", Adadi);
    assert_has_krdanta(&["pari"], &mfj, Krt::kyap, &["parimfjya"]);
    assert_has_krdanta(&["pari"], &mfj, Krt::Ryat, &["parimArgya", "parimArjya"]);
}

#[test]
fn sutra_3_1_120() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::kyap, &["kftya"]);
    assert_has_krdanta(&[], &kf, Krt::Ryat, &["kArya"]);
    let vfz = d("vfzu~", Bhvadi);
    assert_has_krdanta(&[], &vfz, Krt::kyap, &["vfzya"]);
    assert_has_krdanta(&[], &vfz, Krt::Ryat, &["varzya"]);
}

#[test]
fn sutra_3_1_124() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::Ryat, &["kArya"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), Krt::Ryat, &["hArya"]);
    assert_has_krdanta(&[], &d("Df\\Y", Bhvadi), Krt::Ryat, &["DArya"]);
    assert_has_krdanta(&[], &d("va\\ca~", Adadi), Krt::Ryat, &["vAkya", "vAcya"]);
    assert_has_krdanta(
        &[],
        &d("qupa\\ca~^z", Bhvadi),
        Krt::Ryat,
        &["pAkya", "pAcya"],
    );
}

#[test]
fn sutra_3_1_125() {
    let lu = d("lUY", Kryadi);
    assert_has_krdanta(&[], &lu, Krt::Ryat, &["lAvya"]);
    assert_has_krdanta(&[], &lu, Krt::yat, &["lavya"]);
    let pu = d("pUY", Kryadi);
    assert_has_krdanta(&[], &pu, Krt::Ryat, &["pAvya"]);
    assert_has_krdanta(&[], &pu, Krt::yat, &["pavya"]);
}

#[test]
fn sutra_3_1_126() {
    assert_has_krdanta(&["AN"], &d("zu\\Y", Svadi), Krt::Ryat, &["AsAvya"]);
    assert_has_krdanta(&[], &d("yu", Adadi), Krt::Ryat, &["yAvya"]);
    assert_has_krdanta(&[], &d("quva\\pa~^", Bhvadi), Krt::Ryat, &["vApya"]);
    assert_has_krdanta(&[], &d("rapa~", Bhvadi), Krt::Ryat, &["rApya"]);
    assert_has_krdanta(&[], &d("lapa~", Adadi), Krt::Ryat, &["lApya"]);
    assert_has_krdanta(&[], &d("trapU~\\z", Bhvadi), Krt::Ryat, &["trApya"]);
    assert_has_krdanta(&["AN"], &d("camu~", Bhvadi), Krt::Ryat, &["AcAmya"]);
}

#[test]
fn sutra_3_1_133() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::Rvul, &["kAraka"]);
    assert_has_krdanta(&[], &kf, Krt::tfc, &["kartf"]);
    let hf = d("hf\\Y", Bhvadi);
    assert_has_krdanta(&[], &hf, Krt::Rvul, &["hAraka"]);
    assert_has_krdanta(&[], &hf, Krt::tfc, &["hartf"]);
}

#[test]
fn sutra_3_1_135() {
    let kzip = &d("kzi\\pa~^", Tudadi);
    assert_has_krdanta(&["vi"], &kzip, Krt::ka, &["vikzipa"]);
    assert_has_krdanta(&[], &d("bu\\Da~\\", Divadi), Krt::ka, &["buDa"]);
    assert_has_krdanta(&[], &d("kfSa~", Divadi), Krt::ka, &["kfSa"]);
    assert_has_krdanta(&[], &d("jYA\\", Kryadi), Krt::ka, &["jYa"]);
    assert_has_krdanta(&[], &d("prI\\Y", Kryadi), Krt::ka, &["priya"]);
    assert_has_krdanta(&[], &d("kF", Tudadi), Krt::ka, &["kira"]);

    // vAsarUpa-vidhi
    assert_has_krdanta(&[], &kzip, Krt::Rvul, &["kzepaka"]);
    assert_has_krdanta(&[], &kzip, Krt::tfc, &["kzeptf"]);
}

#[ignore]
#[test]
fn sutra_3_1_136() {
    assert_has_krdanta(&["pra"], &d("zWA\\", Bhvadi), Krt::ka, &["prasTa"]);
    assert_has_krdanta(&["su"], &d("glE\\", Bhvadi), Krt::ka, &["sugla"]);
    assert_has_krdanta(&["su"], &d("mlE\\", Bhvadi), Krt::ka, &["sumla"]);
}

#[test]
fn sutra_3_1_137() {
    let paa = &d("pA\\", Bhvadi);
    assert_has_krdanta(&["ud"], &paa, Krt::Sa, &["utpiba"]);
    assert_has_krdanta(&["vi"], &paa, Krt::Sa, &["vipiba"]);

    let ghra = &d("GrA\\", Bhvadi);
    assert_has_krdanta(&["ud"], &ghra, Krt::Sa, &["ujjiGra"]);
    assert_has_krdanta(&["vi"], &ghra, Krt::Sa, &["vijiGra"]);

    let dhma = &d("DmA\\", Bhvadi);
    assert_has_krdanta(&["ud"], &dhma, Krt::Sa, &["udDama"]);
    assert_has_krdanta(&["vi"], &dhma, Krt::Sa, &["viDama"]);

    let dhe = &d("De\\w", Bhvadi);
    assert_has_krdanta(&["ud"], &dhe, Krt::Sa, &["udDaya"]);
    assert_has_krdanta(&["vi"], &dhe, Krt::Sa, &["viDaya"]);

    let dfs = &d("df\\Si~r", Bhvadi);
    assert_has_krdanta(&["ud"], &dfs, Krt::Sa, &["utpaSya"]);
    assert_has_krdanta(&["vi"], &dfs, Krt::Sa, &["vipaSya"]);
}

#[ignore]
#[test]
fn sutra_3_1_138() {
    assert_has_krdanta(&[], &d("li\\pa~^", Tudadi), Krt::Sa, &["limpa"]);
    assert_has_krdanta(&[], &d("vi\\dx~^", Tudadi), Krt::Sa, &["vinda"]);
    assert_has_krdanta(&[], &d("", Bhvadi), Krt::Sa, &["DAraya"]);
    assert_has_krdanta(&[], &d("pF", Curadi), Krt::Sa, &["pAraya"]);
    assert_has_krdanta(&[], &d("vida~", Bhvadi), Krt::Sa, &["vedaya"]);
    assert_has_krdanta(&["ud"], &d("ejf~\\", Bhvadi), Krt::Sa, &["udejaya"]);
    assert_has_krdanta(&[], &d("cita~", Curadi), Krt::Sa, &["cetaya"]);
    assert_has_krdanta(&[], &d("sAti", Bhvadi), Krt::Sa, &["sAtaya"]);
    assert_has_krdanta(&[], &d("zaha~\\", Bhvadi), Krt::Sa, &["sAhaya"]);
}

#[test]
fn sutra_3_1_142() {
    assert_has_krdanta(&[], &d("wudu\\", Svadi), Krt::Ra, &["dAva"]);
    assert_has_krdanta(&[], &d("RI\\Y", Bhvadi), Krt::Ra, &["nAya"]);
    // anupasarge
    assert_has_krdanta(&["pra"], &d("wudu\\", Svadi), Krt::Ra, &[]);
    assert_has_krdanta(&["pra"], &d("RI\\Y", Bhvadi), Krt::Ra, &[]);
    // TODO: others
}

#[test]
fn sutra_3_1_143() {
    let grah = d("graha~^", Kryadi);
    assert_has_krdanta(&[], &grah, Krt::aR, &["grAha"]);
    assert_has_krdanta(&[], &grah, Krt::ac, &["graha"]);
}

#[test]
fn sutra_3_1_144() {
    assert_has_krdanta(&[], &d("graha~^", Kryadi), Krt::ka, &["gfha"]);
}

#[test]
fn sutra_3_1_145() {
    // TODO: change examples to nartakI, etc.
    assert_has_krdanta(&[], &d("nftI~", Divadi), Krt::zvun, &["nartaka"]);
    assert_has_krdanta(&[], &d("Kanu~^", Bhvadi), Krt::zvun, &["Kanaka"]);
    assert_has_krdanta(&[], &d("ra\\nja~^", Bhvadi), Krt::zvun, &["rajaka"]);
}

#[test]
fn sutra_3_1_146() {
    assert_has_krdanta(&[], &d("gE\\", Bhvadi), Krt::Takan, &["gATaka"]);
    // TODO: gATikA
}

#[test]
fn sutra_3_1_147() {
    assert_has_krdanta(&[], &d("gE\\", Bhvadi), Krt::Ryuw, &["gAyana"]);
    // TODO: gAyanI
}

#[test]
fn sutra_3_1_148() {
    assert_has_krdanta(&[], &d("o~hA\\N", Juhotyadi), Krt::Ryuw, &["hAyana"]);
    assert_has_krdanta(&[], &d("o~hA\\N", Juhotyadi), Krt::Ryuw, &["hAyana"]);
}

#[test]
fn sutra_3_1_149() {
    assert_has_krdanta(&[], &d("pru\\N", Bhvadi), Krt::vun, &["pravaka"]);
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), Krt::vun, &["saraka"]);
    assert_has_krdanta(&[], &d("lUY", Kryadi), Krt::vun, &["lavaka"]);
}

#[test]
fn sutra_3_1_150() {
    assert_has_krdanta(&[], &d("jIva~", Bhvadi), Krt::vun, &["jIvaka"]);
    assert_has_krdanta(&[], &d("wunadi~", Bhvadi), Krt::vun, &["nandaka"]);
}
