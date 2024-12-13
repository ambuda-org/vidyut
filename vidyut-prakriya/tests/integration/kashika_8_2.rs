extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Pada;
use vidyut_prakriya::args::Subanta;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha;
use vidyut_prakriya::args::Vacana;
use vidyut_prakriya::args::Vibhakti;

#[test]
fn sutra_8_2_7() {
    assert_has_sup_1s("rAjan", Pum, &["rAjA"]);
    assert_has_sup_3d("rAjan", Pum, &["rAjaByAm"]);
    assert_has_sup_3p("rAjan", Pum, &["rAjaBiH"]);
    assert_has_taddhita("rAjan", T::tal, &["rAjatA"]);
    assert_has_taddhita("rAjan", T::tarap, &["rAjatara"]);
    assert_has_taddhita("rAjan", T::tamap, &["rAjatama"]);
}

#[test]
fn sutra_8_2_8() {
    assert_has_sup_ss("rAjan", Pum, &["rAjan"]);
    assert_has_sup_ss("takzan", Pum, &["takzan"]);
}

#[test]
fn sutra_8_2_9() {
    // makAra-anta
    assert_has_taddhita("kim", T::matup, &["kiMvat"]);
    assert_has_taddhita("Sam", T::matup, &["SaMvat"]);
    // makAra-upadha
    assert_has_taddhita("SamI", T::matup, &["SamIvat"]);
    assert_has_taddhita("dAqimI", T::matup, &["dAqimIvat"]);
    // avarRa-anta
    assert_has_taddhita("vfkza", T::matup, &["vfkzavat"]);
    assert_has_taddhita("plakza", T::matup, &["plakzavat"]);
    assert_has_taddhita("KawvA", T::matup, &["KawvAvat"]);
    assert_has_taddhita("mAlA", T::matup, &["mAlAvat"]);
    // avarRa-upaDa
    assert_has_taddhita("payas", T::matup, &["payasvat"]);
    assert_has_taddhita("yaSas", T::matup, &["yaSasvat"]);
    assert_has_taddhita("BAs", T::matup, &["BAsvat"]);
    // mAd?
    assert_has_taddhita("agni", T::matup, &["agnimat"]);
    assert_has_taddhita("vAyu", T::matup, &["vAyumat"]);
    // avAdibhyaH?
    assert_has_taddhita("yava", T::matup, &["yavamat"]);
    assert_has_taddhita("dalmi", T::matup, &["dalmimat"]);
    assert_has_taddhita("Urmi", T::matup, &["Urmimat"]);
}

#[test]
fn sutra_8_2_10() {
    assert_has_taddhita("agnicit", T::matup, &["agnicitvat"]);
    assert_has_taddhita("vidyut", T::matup, &["vidyutvat"]);
    assert_has_taddhita("marut", T::matup, &["marutvat"]);
    assert_has_taddhita("dfzad", T::matup, &["dfzadvat"]);
}

#[test]
fn sutra_8_2_18() {
    let kfp = d("kfpU~\\", Bhvadi);
    assert_has_ta(&[], &kfp, Lut, &["kalptA", "kalpitA"]);
    assert_has_aataam(&[], &kfp, Lut, &["kalptArO", "kalpitArO"]);
    assert_has_jha(&[], &kfp, Lut, &["kalptAraH", "kalpitAraH"]);
    assert_has_tip(&[], &san(&kfp), Lat, &["cikxpsati"]);
    assert_has_krdanta(&[], &kfp, Krt::kta, &["kxpta"]);
    assert_has_krdanta(&[], &kfp, Krt::ktavatu, &["kxptavat"]);
}

#[test]
fn sutra_8_2_19() {
    let ay = d("aya~\\", Bhvadi);
    assert_has_ta(&["pra"], &ay, Lat, &["plAyate"]);
    assert_has_ta(&["parA"], &ay, Lat, &["palAyate"]);
    assert_has_ta(&["pari"], &ay, Lat, &["palyayate"]);

    assert_has_ta(&["prati"], &ay, Lat, &["pratyayate"]);
    assert_has_krdanta(&["nis"], &ay, Krt::lyuw, &["nirayaRa"]);
    assert_has_krdanta(&["dus"], &ay, Krt::lyuw, &["durayaRa"]);

    // extra examples from the Siddhantakaumudi
    assert_has_ta(&["nis"], &ay, Lat, &["nirayate"]);
    assert_has_ta(&["dus"], &ay, Lat, &["durayate"]);
    assert_has_ta(&["nir"], &ay, Lat, &["nilayate"]);
    assert_has_ta(&["dur"], &ay, Lat, &["dulayate"]);
}

#[test]
fn sutra_8_2_20() {
    let gf = d("gF", Tudadi);
    assert_has_ta(&["ni"], &yan(&gf), Lat, &["nijegilyate"]);
    assert_has_aataam(&["ni"], &yan(&gf), Lat, &["nijegilyete"]);
    assert_has_jha(&["ni"], &yan(&gf), Lat, &["nijegilyante"]);
    // yaNi
    assert_has_ta_k(&["ni"], &gf, Lat, &["nigIryate"]);
}

#[test]
fn sutra_8_2_21() {
    let gf = d("gF", Tudadi);
    assert_has_lat(&["ni"], &gf, &["nigirati", "nigilati"]);
    assert_has_krdanta(&["ni"], &gf, Krt::lyuw, &["nigaraRa", "nigalana"]);
    assert_has_krdanta(&["ni"], &gf, Krt::Rvul, &["nigAraka", "nigAlaka"]);
    // TODO: others.
}

#[test]
fn sutra_8_2_23() {
    assert_has_sup_1s(taddhitanta("go", T::matup), Pum, &["gomAn"]);
    assert_has_sup_1s(taddhitanta("yava", T::matup), Pum, &["yavamAn"]);
    assert_has_sup_1s(taddhitanta("kfta", T::matup), Pum, &["kftavAn"]);
    assert_has_sup_1s(taddhitanta("hata", T::matup), Pum, &["hatavAn"]);
    assert_has_sup_1s(
        taddhitanta("praSasya", T::Iyasun).with_require("Sreyas"),
        Pum,
        &["SreyAn"],
    );
    assert_has_sup_1s(taddhitanta("bahu", T::Iyasun), Pum, &["BUyAn"]);
    // TODO: others
}

#[test]
fn sutra_8_2_24() {
    assert_has_sup_6s("mAtf", Stri, &["mAtuH"]);
    assert_has_sup_6s("pitf", Pum, &["pituH"]);
    assert_has_sup_1s("Urj", Pum, &["Urk"]);
    assert_has_lan(&[], &d("mfjU~", Adadi), &["amArw"]);
}

#[test]
fn sutra_8_2_25() {
    assert_has_dhvam(&[], &d("lUY", Kryadi), Lun, &["alaviDvam", "alaviQvam"]);
    assert_has_dhvam(
        &[],
        &d("pUY", Kryadi),
        AshirLin,
        &["pavizIDvam", "pavizIQvam"],
    );
    // izwi, taking Patanjali's interpretation:
    assert_has_sip(&[], &d("cakAsf~", Adadi), Lot, &["cakADi", "cakAstAt"]);
    assert_has_sandhi("payas", "DAvati", &["payo DAvati"]);
}

#[test]
fn sutra_8_2_26() {
    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_ta(&[], &bhid, Lun, &["aBitta"]);
    assert_has_thaas(&[], &bhid, Lun, &["aBitTAH"]);

    let chid = d("Ci\\di~^r", Rudhadi);
    assert_has_ta(&[], &chid, Lun, &["acCitta"]);
    assert_has_thaas(&[], &chid, Lun, &["acCitTAH"]);

    let vas = d("va\\sa~", Bhvadi);
    assert_has_tas(&[], &vas, Lun, &["avAttAm"]);
    assert_has_tha(&[], &vas, Lun, &["avAtta"]);

    // JalaH
    let man = d("ma\\na~\\", Divadi);
    assert_has_ta(&[], &man, Lun, &["amaMsta"]);
    assert_has_thaas(&[], &man, Lun, &["amaMsTAH"]);

    // Jali
    assert_has_aataam(&[], &bhid, Lun, &["aBitsAtAm"]);
    assert_has_jha(&[], &bhid, Lun, &["aBitsata"]);

    // TODO: others
}

#[test]
fn sutra_8_2_27() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_ta(&[], &kf, Lun, &["akfta"]);
    assert_has_thaas(&[], &kf, Lun, &["akfTAH"]);

    let hf = d("hf\\Y", Bhvadi);
    assert_has_ta(&[], &hf, Lun, &["ahfta"]);
    assert_has_thaas(&[], &hf, Lun, &["ahfTAH"]);

    // hrasvAt?
    assert_has_ta(&[], &d("cyu\\N", Bhvadi), Lun, &["acyozwa"]);
    assert_has_ta(&[], &d("plu\\N", Bhvadi), Lun, &["aplozwa"]);

    // aNgAt?
    let lu = d("lUY", Kryadi);
    assert_has_tas(&[], &lu, Lun, &["alAvizwAm"]);
    assert_has_jhi(&[], &lu, Lun, &["alAvizuH"]);

    // Jali?
    assert_has_aataam(&[], &kf, Lun, &["akfzAtAm"]);
    assert_has_jha(&[], &kf, Lun, &["akfzata"]);

    // TODO: others
}

#[test]
fn sutra_8_2_28() {
    // TODO: adAvIt?
    // assert_has_tip(&[], &d("du\\", Bhvadi), Lun, &["adAvIt"]);
    assert_has_tip(&[], &d("lUY", Kryadi), Lun, &["alAvIt"]);
    assert_has_tip(&[], &d("zivu~", Divadi), Lun, &["asevIt"]);
    assert_has_tip(&[], &d("kuza~", Kryadi), Lun, &["akozIt"]);
    assert_has_tip(&[], &d("muza~", Kryadi), Lun, &["amozIt"]);

    // iwaH?
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lun, &["akArzIt"]);
    assert_has_tip(&[], &d("hf\\Y", Bhvadi), Lun, &["ahArzIt"]);

    // Iwi?
    let lu = d("lUY", Kryadi);
    assert_has_tas(&[], &lu, Lun, &["alAvizwAm"]);
    assert_has_jhi(&[], &lu, Lun, &["alAvizuH"]);
}

#[ignore]
#[test]
fn sutra_8_2_29() {
    let lasj = d("o~lasjI~\\", Tudadi);
    assert_has_krdanta(&[], &lasj, Krt::kta, &["lagna"]);
    assert_has_krdanta(&[], &lasj, Krt::ktavatu, &["lagnavat"]);
    assert_has_krdanta(&[], &d("wuma\\sjo~", Tudadi), Krt::kta, &["magna"]);

    assert_has_krdanta(&[], &d("takza~", Bhvadi), Krt::kta, &["tazwa"]);
    assert_has_krdanta(&[], &d("takza~", Bhvadi), Krt::ktavatu, &["tazwavat"]);

    // TODO: others
}

#[test]
fn sutra_8_2_30() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::tfc, &["paktf"]);
    assert_has_krdanta(&[], &pac, Krt::tumun, &["paktum"]);
    assert_has_krdanta(&[], &pac, Krt::tavya, &["paktavya"]);

    let vac = d("va\\ca~", Adadi);
    assert_has_krdanta(&[], &vac, Krt::tfc, &["vaktf"]);
    assert_has_krdanta(&[], &vac, Krt::tumun, &["vaktum"]);
    assert_has_krdanta(&[], &vac, Krt::tavya, &["vaktavya"]);
    // TODO: add the rest of the examples
}

#[test]
fn sutra_8_2_31() {
    let sah = d("zaha~\\", Bhvadi);
    assert_has_krdanta(&[], &sah, Krt::tfc, &["soQf", "sahitf"]);
    assert_has_krdanta(&[], &sah, Krt::tumun, &["soQum", "sahitum"]);
    assert_has_krdanta(&[], &sah, Krt::tavya, &["soQavya", "sahitavya"]);
    assert_has_sup_1s("jalAzAh", Pum, &["jalAzAw"]);

    let vah = d("va\\ha~^", Bhvadi);
    assert_has_krdanta(&[], &vah, Krt::tfc, &["voQf"]);
    assert_has_krdanta(&[], &vah, Krt::tumun, &["voQum"]);
    assert_has_krdanta(&[], &vah, Krt::tavya, &["voQavya"]);
    assert_has_sup_1s("prazWavAh", Pum, &["prazWavAw"]);
    assert_has_sup_1s("dityavAh", Pum, &["dityavAw"]);
}

#[test]
fn sutra_8_2_32() {
    let dah = d("da\\ha~", Bhvadi);
    assert_has_krdanta(&[], &dah, Krt::tfc, &["dagDf"]);
    assert_has_krdanta(&[], &dah, Krt::tumun, &["dagDum"]);
    assert_has_krdanta(&[], &dah, Krt::tavya, &["dagDavya"]);

    let duh = d("du\\ha~^", Bhvadi);
    assert_has_krdanta(&[], &duh, Krt::tfc, &["dogDf"]);
    assert_has_krdanta(&[], &duh, Krt::tumun, &["dogDum"]);
    assert_has_krdanta(&[], &duh, Krt::tavya, &["dogDavya"]);

    let lih = d("li\\ha~^", Adadi);
    assert_has_krdanta(&[], &lih, Krt::tfc, &["leQf"]);
    assert_has_krdanta(&[], &lih, Krt::tumun, &["leQum"]);
    assert_has_krdanta(&[], &lih, Krt::tavya, &["leQavya"]);
    // TODO: kAzWaDak, etc.
}

#[ignore]
#[test]
fn sutra_8_2_33() {
    let druh = d("dru\\ha~", Divadi);
    assert_has_krdanta(&[], &druh, Krt::tfc, &["drogDf", "droQf", "drohitf"]);
    assert_has_krdanta(&[], &druh, Krt::kvip, &["Druk", "Druw"]);
    let muh = d("mu\\ha~", Divadi);
    assert_has_krdanta(&["ud"], &muh, Krt::tfc, &["unmogDf", "unmoQf", "unmohitf"]);
    // TODO: why muN?
    // assert_has_krdanta(&[], &muh, Krt::kvip, &["muk", "muN"]);
    let snuh = d("zRu\\ha~", Divadi);
    assert_has_krdanta(
        &["ud"],
        &snuh,
        Krt::tfc,
        &["utsnogDf", "utsnoQf", "utsnohitf"],
    );
    assert_has_krdanta(&["ud"], &snuh, Krt::kvip, &["utsnuk", "utsnuw"]);
    let snih = d("zRi\\ha~", Divadi);
    assert_has_krdanta(&[], &snih, Krt::tfc, &["snegDf", "sneQf", "snehitf"]);
    assert_has_krdanta(&[], &snih, Krt::kvip, &["snik", "sniw"]);
}

#[test]
fn sutra_8_2_34() {
    let nah = d("Ra\\ha~^", Divadi);
    assert_has_krdanta(&[], &nah, Krt::tfc, &["nadDf"]);
    assert_has_krdanta(&[], &nah, Krt::tumun, &["nadDum"]);
    assert_has_krdanta(&[], &nah, Krt::tavya, &["nadDavya"]);
    // TODO: upAnat, etc.
}

#[test]
fn sutra_8_2_35() {
    let bru = d("brUY", Adadi);
    assert_has_sip(&[], &bru, Lat, &["AtTa", "bravIzi"]);
    // otherwise --
    assert_has_tip(&[], &bru, Lat, &["Aha", "bravIti"]);
    assert_has_tas(&[], &bru, Lat, &["AhatuH", "brUtaH"]);
    assert_has_jhi(&[], &bru, Lat, &["AhuH", "bruvanti"]);
}

#[ignore]
#[test]
fn sutra_8_2_36() {
    let vrasc = d("o~vrascU~", Tudadi);
    assert_has_krdanta(&[], &vrasc, Krt::tfc, &["vrazwf", "vraScitf"]);
    assert_has_krdanta(&[], &vrasc, Krt::tumun, &["vrazwum", "vraScitum"]);
    assert_has_krdanta(&[], &vrasc, Krt::tavya, &["vrazwavya", "vraScitavya"]);
    assert_has_krdanta(&[], &vrasc, Krt::kvip, &["vfw"]);

    let bhrasj = d("Bra\\sja~^", Tudadi);
    assert_has_krdanta(&[], &bhrasj, Krt::tfc, &["Brazwf", "Barzwf"]);
    assert_has_krdanta(&[], &bhrasj, Krt::tumun, &["Brazwum", "Barzwum"]);
    assert_has_krdanta(&[], &bhrasj, Krt::tavya, &["Brazwavya", "Barzwavya"]);
    assert_has_krdanta(&[], &bhrasj, Krt::kvip, &["Bfw"]);

    let sfj = d("sf\\ja~\\", Tudadi);
    assert_has_krdanta(&[], &sfj, Krt::tfc, &["srazwf"]);
    assert_has_krdanta(&[], &sfj, Krt::tumun, &["srazwum"]);
    assert_has_krdanta(&[], &sfj, Krt::tavya, &["srazwavya"]);
    assert_has_krdanta(&[], &sfj, Krt::kvip, &["sfw"]);

    let mfj = d("mfjU~", Adadi);
    assert_has_krdanta(&[], &mfj, Krt::tfc, &["mArzwf", "mArjitf"]);
    assert_has_krdanta(&[], &mfj, Krt::tumun, &["mArzwum", "mArjitum"]);
    assert_has_krdanta(&[], &mfj, Krt::tavya, &["mArzwavya", "mArjitavya"]);
    assert_has_krdanta(&[], &mfj, Krt::kvip, &["mfw"]);

    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_krdanta(&[], &yaj, Krt::tfc, &["yazwf"]);
    assert_has_krdanta(&[], &yaj, Krt::tumun, &["yazwum"]);
    assert_has_krdanta(&[], &yaj, Krt::tavya, &["yazwavya"]);

    let raj = d("rAjf~^", Bhvadi);
    assert_has_krdanta(&[], &raj, Krt::kvip, &["rAw"]);

    let bhraj = d("BrAjf~\\", Bhvadi);
    assert_has_krdanta(&[], &bhraj, Krt::kvip, &["BrAw"]);

    let prach = d("pra\\Ca~", Tudadi);
    assert_has_krdanta(&[], &prach, Krt::tfc, &["prazwf"]);
    assert_has_krdanta(&[], &prach, Krt::tumun, &["prazwum"]);
    assert_has_krdanta(&[], &prach, Krt::tavya, &["prazwavya"]);

    let lish = d("li\\Sa~", Tudadi);
    assert_has_krdanta(&[], &lish, Krt::tfc, &["lezwf"]);
    assert_has_krdanta(&[], &lish, Krt::tumun, &["lezwum"]);
    assert_has_krdanta(&[], &lish, Krt::tavya, &["lezwavya"]);
    assert_has_krdanta(&[], &lish, Krt::kvip, &["liw"]);

    let vish = d("vi\\Sa~", Tudadi);
    assert_has_krdanta(&[], &vish, Krt::tfc, &["vezwf"]);
    assert_has_krdanta(&[], &vish, Krt::tumun, &["vezwum"]);
    assert_has_krdanta(&[], &vish, Krt::tavya, &["vezwavya"]);
    assert_has_krdanta(&[], &vish, Krt::kvip, &["viw"]);

    // TODO: others
}

#[ignore]
#[test]
fn sutra_8_2_37() {
    let budh = d("bu\\Da~\\", Divadi);
    assert_has_lrt(&[], &budh, &["Botsyate"]);
    assert_has_dhvam(&[], &budh, Lun, &["aBudDvam"]);
    assert_has_krdanta(&[], &budh, Krt::kvip, &["But"]);

    let guh = d("guhU~^", Bhvadi);
    assert_has_ta(&["ni"], &guh, Lrt, &["niGokzyate", "nigUhizyate"]);
    assert_has_dhvam(
        &["ni"],
        &guh,
        Lun,
        &["nyaGUQvam", "nyaGukzaDvam", "nyagUhiDvam", "nyagUhiQvam"],
    );
    assert_has_krdanta(&[], &guh, Krt::kvip, &["Guw"]);

    let duh = d("du\\ha~^", Adadi);
    assert_has_ta(&[], &duh, Lrt, &["Dokzyate"]);
    assert_has_dhvam(&[], &duh, Lun, &["aDugDvam", "aDukzaDvam"]);
    assert_has_krdanta(&[], &duh, Krt::kvip, &["Duk"]);

    // baS?
    assert_has_lrt(&[], &d("kru\\Da~", Divadi), &["krotsyati"]);

    // Jazantasya?
    assert_has_tip(&[], &d("qudA\\Y", Juhotyadi), Lrt, &["dAsyati"]);

    // sDvoH?
    assert_has_krdanta(&[], &budh, Krt::tfc, &["bodDf"]);
    assert_has_krdanta(&[], &budh, Krt::tumun, &["bodDum"]);
    assert_has_krdanta(&[], &budh, Krt::tavya, &["bodDavya"]);

    // TODO: others
}

#[test]
fn sutra_8_2_38() {
    let dha = d("quDA\\Y", Juhotyadi);
    assert_has_tas(&[], &dha, Lat, &["DattaH"]);
    assert_has_thas(&[], &dha, Lat, &["DatTaH"]);
    assert_has_thaas(&[], &dha, Lat, &["Datse"]);
    assert_has_dhvam(&[], &dha, Lot, &["DadDvam"]);

    // Jazantasya?
    assert_has_tip(&[], &dha, Lat, &["daDAti"]);
}

#[test]
fn sutra_8_2_39() {
    assert_has_sandhi("vAk", "atra", &["vAg atra"]);
    assert_has_sandhi("Svaliw", "atra", &["Svaliq atra"]);
    assert_has_sandhi("agnicit", "atra", &["agnicid atra"]);
    assert_has_sandhi("trizWup", "atra", &["trizWub atra"]);

    // ante?
    let vas = d("va\\sa~", Bhvadi);
    assert_has_krdanta(&[], &vas, Krt::tfc, &["vastf"]);
    assert_has_krdanta(&[], &vas, Krt::tavyat, &["vastavya", "vAstavya"]);
}

#[test]
fn sutra_8_2_40() {
    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_krdanta(&[], &labh, Krt::tfc, &["labDf"]);
    assert_has_krdanta(&[], &labh, Krt::tumun, &["labDum"]);
    assert_has_krdanta(&[], &labh, Krt::tavya, &["labDavya"]);
    assert_has_ta(&[], &labh, Lun, &["alabDa"]);
    assert_has_thaas(&[], &labh, Lun, &["alabDAH"]);

    let duh = d("du\\ha~^", Bhvadi);
    assert_has_krdanta(&[], &duh, Krt::tfc, &["dogDf"]);
    assert_has_krdanta(&[], &duh, Krt::tumun, &["dogDum"]);
    assert_has_krdanta(&[], &duh, Krt::tavya, &["dogDavya"]);
    assert_has_ta(&[], &duh, Lun, &["adugDa", "aDukzata"]);
    assert_has_thaas(&[], &duh, Lun, &["adugDAH", "aDukzaTAH"]);

    let lih = d("li\\ha~^", Adadi);
    assert_has_krdanta(&[], &lih, Krt::tfc, &["leQf"]);
    assert_has_krdanta(&[], &lih, Krt::tumun, &["leQum"]);
    assert_has_krdanta(&[], &lih, Krt::tavya, &["leQavya"]);
    assert_has_ta(&[], &lih, Lun, &["alIQa", "alikzata"]);
    assert_has_thaas(&[], &lih, Lun, &["alIQAH", "alikzaTAH"]);

    let budh = d("bu\\Da~\\", Divadi);
    assert_has_krdanta(&[], &budh, Krt::tfc, &["bodDf"]);
    assert_has_krdanta(&[], &budh, Krt::tumun, &["bodDum"]);
    assert_has_krdanta(&[], &budh, Krt::tavya, &["bodDavya"]);
    assert_has_ta(&[], &budh, Lun, &["abudDa", "aboDi"]);
    assert_has_thaas(&[], &budh, Lun, &["abudDAH"]);

    // aDaH?
    let dha = d("quDA\\Y", Juhotyadi);
    assert_has_tas(&[], &dha, Lat, &["DattaH"]);
    assert_has_thas(&[], &dha, Lat, &["DatTaH"]);
}

#[test]
fn sutra_8_2_41() {
    let pish = d("pi\\zx~", Rudhadi);
    assert_has_lrt(&[], &pish, &["pekzyati"]);
    assert_has_lrn(&[], &pish, &["apekzyat"]);
    assert_has_lat(&[], &san(&pish), &["pipikzati"]);

    let lih = d("li\\ha~^", Adadi);
    assert_has_tip(&[], &lih, Lrt, &["lekzyati"]);
    assert_has_tip(&[], &lih, Lrn, &["alekzyat"]);
    assert_has_tip(&[], &san(&lih), Lat, &["lilikzati"]);

    // si?
    assert_has_tip(&[], &pish, Lat, &["pinazwi"]);
    assert_has_tip(&[], &lih, Lat, &["leQi"]);
}

#[test]
fn sutra_8_2_42() {
    let stf = d("stFY", Kryadi);
    assert_has_krdanta(&["AN"], &stf, Krt::kta, &["AstIrRa"]);
    assert_has_krdanta(&["vi"], &stf, Krt::kta, &["vistIrRa"]);
    assert_has_krdanta(&["vi"], &d("SFY", Kryadi), Krt::kta, &["viSIrRa"]);
    assert_has_krdanta(&["ni"], &d("gF", Tudadi), Krt::kta, &["nigIrRa"]);
    assert_has_krdanta(&["ava"], &d("gurvI~\\", Divadi), Krt::kta, &["avagUrRa"]);

    // dakArAt
    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_krdanta(&[], &bhid, Krt::kta, &["Binna"]);
    assert_has_krdanta(&[], &bhid, Krt::ktavatu, &["Binnavat"]);

    let chid = d("Ci\\di~^r", Rudhadi);
    assert_has_krdanta(&[], &chid, Krt::kta, &["Cinna"]);
    assert_has_krdanta(&[], &chid, Krt::ktavatu, &["Cinnavat"]);

    // radAByAm
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::kta, &["kfta"]);
    assert_has_krdanta(&[], &kf, Krt::ktavatu, &["kftavat"]);

    // nizWA?
    assert_has_krdanta(&[], &kf, Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), Krt::tfc, &["hartf"]);

    // taH
    assert_has_krdanta(&[], &d("cara~", Bhvadi), Krt::kta, &["carita"]);
    assert_has_krdanta(&[], &d("muda~\\", Bhvadi), Krt::kta, &["mudita"]);
    // TODO: pUrvasya
}

#[test]
fn sutra_8_2_43() {
    let dra = d("drA\\", Adadi);
    assert_has_krdanta(&["pra"], &dra, Krt::kta, &["pradrARa"]);
    assert_has_krdanta(&["pra"], &dra, Krt::ktavatu, &["pradrARavat"]);

    let mlai = d("mlE\\", Bhvadi);
    assert_has_krdanta(&[], &mlai, Krt::kta, &["mlAna"]);
    assert_has_krdanta(&[], &mlai, Krt::ktavatu, &["mlAnavat"]);

    // saMyogAdeH
    let yaa = d("yA\\", Adadi);
    assert_has_krdanta(&[], &yaa, Krt::kta, &["yAta"]);
    assert_has_krdanta(&[], &yaa, Krt::ktavatu, &["yAtavat"]);

    // AtaH
    let cyu = d("cyu\\N", Bhvadi);
    assert_has_krdanta(&[], &cyu, Krt::kta, &["cyuta"]);
    assert_has_krdanta(&[], &cyu, Krt::ktavatu, &["cyutavat"]);

    let plu = d("plu\\N", Bhvadi);
    assert_has_krdanta(&[], &plu, Krt::kta, &["pluta"]);
    assert_has_krdanta(&[], &plu, Krt::ktavatu, &["plutavat"]);

    // DAtoH
    assert_has_krdanta(&["nis"], &yaa, Krt::kta, &["niryAta"]);
    assert_has_krdanta(
        &["nis"],
        &d("vA\\", Adadi),
        Krt::kta,
        &["nirvAta", "nirvARa"],
    );
    // yaNvat
    let sna = d("zRA\\", Adadi);
    assert_has_krdanta(&[], &sna, Krt::kta, &["snAta"]);
    assert_has_krdanta(&[], &sna, Krt::ktavatu, &["snAtavat"]);
}

#[test]
fn sutra_8_2_44() {
    let lu = d("lUY", Kryadi);
    assert_has_krdanta(&[], &lu, Krt::kta, &["lUna"]);
    assert_has_krdanta(&[], &lu, Krt::ktavatu, &["lUnavat"]);
    let dhu = d("DUY", Kryadi);
    assert_has_krdanta(&[], &dhu, Krt::kta, &["DUna"]);
    assert_has_krdanta(&[], &dhu, Krt::ktavatu, &["DUnavat"]);
    let jya = d("jyA\\", Kryadi);
    assert_has_krdanta(&[], &jya, Krt::kta, &["jIna"]);
    assert_has_krdanta(&[], &jya, Krt::ktavatu, &["jInavat"]);
}

#[test]
fn sutra_8_2_45() {
    let lasj = d("o~lasjI~\\", Tudadi);
    assert_has_krdanta(&[], &lasj, Krt::kta, &["lagna"]);
    assert_has_krdanta(&[], &lasj, Krt::ktavatu, &["lagnavat"]);
    let vij = d("o~vijI~", Rudhadi);
    assert_has_krdanta(&["ud"], &vij, Krt::kta, &["udvigna"]);
    assert_has_krdanta(&["ud"], &vij, Krt::ktavatu, &["udvignavat"]);
    let pyay = d("o~pyAyI~\\", Bhvadi);
    assert_has_krdanta(&["AN"], &pyay, Krt::kta, &["ApIna", "ApyAna"]);
    assert_has_krdanta(&["AN"], &pyay, Krt::ktavatu, &["ApInavat", "ApyAnavat"]);
    // svAdayaH
    let su = d("zUN", Divadi);
    assert_has_krdanta(&[], &su, Krt::kta, &["sUna"]);
    assert_has_krdanta(&[], &su, Krt::ktavatu, &["sUnavat"]);
    let du = d("dUN", Divadi);
    assert_has_krdanta(&[], &du, Krt::kta, &["dUna"]);
    assert_has_krdanta(&[], &du, Krt::ktavatu, &["dUnavat"]);
    let di = d("dI\\N", Divadi);
    assert_has_krdanta(&[], &di, Krt::kta, &["dIna"]);
    assert_has_krdanta(&[], &di, Krt::ktavatu, &["dInavat"]);
    // TODO: why qIna?
    let _qi = d("qIN", Divadi);
    // assert_has_krdanta(&[], &qi, Krt::kta, &["qIna"]);
    // assert_has_krdanta(&[], &qi, Krt::ktavatu, &["qInavat"]);
    let dhi = d("DI\\N", Divadi);
    assert_has_krdanta(&[], &dhi, Krt::kta, &["DIna"]);
    assert_has_krdanta(&[], &dhi, Krt::ktavatu, &["DInavat"]);
    let mi = d("mI\\N", Divadi);
    assert_has_krdanta(&[], &mi, Krt::kta, &["mIna"]);
    assert_has_krdanta(&[], &mi, Krt::ktavatu, &["mInavat"]);
    let ri = d("rI\\N", Divadi);
    assert_has_krdanta(&[], &ri, Krt::kta, &["rIRa"]);
    assert_has_krdanta(&[], &ri, Krt::ktavatu, &["rIRavat"]);
    let li = d("lI\\N", Divadi);
    assert_has_krdanta(&[], &li, Krt::kta, &["lIna"]);
    assert_has_krdanta(&[], &li, Krt::ktavatu, &["lInavat"]);
    let vri = d("vrI\\N", Divadi);
    assert_has_krdanta(&[], &vri, Krt::kta, &["vrIRa"]);
    assert_has_krdanta(&[], &vri, Krt::ktavatu, &["vrIRavat"]);
}

#[ignore]
#[test]
fn sutra_8_2_46() {
    let kshi = d("kzi\\", Bhvadi);
    assert_has_krdanta(&[], &kshi, Krt::kta, &["kzIRa"]);
    // TODO: akzita
}

#[ignore]
#[test]
fn sutra_8_2_47() {
    let shyai = d("SyE\\N", Bhvadi);
    assert_has_krdanta(&[], &shyai, Krt::kta, &["SIna", "SIta"]);
}

#[test]
fn sutra_8_2_51() {
    let sus = d("Su\\za~", Divadi);
    assert_has_krdanta(&[], &sus, Krt::kta, &["Suzka"]);
    assert_has_krdanta(&[], &sus, Krt::ktavatu, &["Suzkavat"]);
}

#[test]
fn sutra_8_2_52() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::kta, &["pakva"]);
    assert_has_krdanta(&[], &pac, Krt::ktavatu, &["pakvavat"]);
}

#[test]
fn sutra_8_2_53() {
    let kzai = d("kzE\\", Bhvadi);
    assert_has_krdanta(&[], &kzai, Krt::kta, &["kzAma"]);
    assert_has_krdanta(&[], &kzai, Krt::ktavatu, &["kzAmavat"]);
}

#[test]
fn sutra_8_2_54() {
    let styai = d("styE\\", Bhvadi);
    assert_has_krdanta(&["pra"], &styai, Krt::kta, &["prastIma", "prastIta"]);
    assert_has_krdanta(
        &["pra"],
        &styai,
        Krt::ktavatu,
        &["prastImavat", "prastItavat"],
    );
}

#[test]
fn sutra_8_2_55() {
    let phal = d("YiPalA~", Bhvadi);
    assert_has_krdanta(&[], &phal, Krt::kta, &["Pulla"]);
    assert_has_krdanta(&[], &phal, Krt::ktavatu, &["Pullavat"]);

    let kzib = d("kzIbf~\\", Bhvadi);
    assert_has_krdanta(&[], &kzib, Krt::kta, &["kzIba"]);

    let kfs = d("kfSa~", Divadi);
    assert_has_krdanta(&[], &kfs, Krt::kta, &["kfSa"]);

    let lagh = d("lAGf~\\", Bhvadi);
    assert_has_krdanta(&["ud"], &lagh, Krt::kta, &["ullAGa"]);

    // TODO: allow praPulta
    // assert_has_krdanta(&["pra"], &phal, Krt::kta, &["praPulta"]);
    assert_has_krdanta(&["pra"], &kzib, Krt::kta, &["prakzIbita"]);
    assert_has_krdanta(&["pra"], &kfs, Krt::kta, &["prakfSita"]);
    assert_has_krdanta(&["pra", "ud"], &lagh, Krt::kta, &["prollAGita"]);
}

#[test]
fn sutra_8_2_56() {
    use Krt::kta;
    assert_has_krdanta(&[], &d("Ru\\da~^", Tudadi), kta, &["nunna", "nutta"]);
    assert_has_krdanta(&[], &d("vi\\da~\\", Rudhadi), kta, &["vinna", "vitta"]);
    assert_has_krdanta(&["sam"], &d("undI~", Rudhadi), kta, &["samunna", "samutta"]);
    assert_has_krdanta(&[], &d("trE\\N", Bhvadi), kta, &["trARa", "trAta"]);
    assert_has_krdanta(&[], &d("GrA\\", Bhvadi), kta, &["GrARa", "GrAta"]);
    assert_has_krdanta(&[], &d("hrI\\", Juhotyadi), kta, &["hrIRa", "hrIta"]);
    // only for vinatti
    assert_has_krdanta(&[], &d("vida~", Adadi), kta, &["vidita"]);
    assert_has_krdanta(&[], &d("vi\\da~\\", Divadi), kta, &["vinna"]);
    assert_has_krdanta(&[], &d("vi\\dx~^", Tudadi), kta, &["vinna", "vidita"]);
}

#[test]
fn sutra_8_2_57() {
    let dhyai = d("DyE\\", Bhvadi);
    assert_has_krdanta(&[], &dhyai, Krt::kta, &["DyAta"]);
    assert_has_krdanta(&[], &dhyai, Krt::ktavatu, &["DyAtavat"]);

    let khya = d("KyA\\", Adadi);
    assert_has_krdanta(&[], &khya, Krt::kta, &["KyAta"]);
    assert_has_krdanta(&[], &khya, Krt::ktavatu, &["KyAtavat"]);

    let pf = d("pF", Kryadi);
    assert_has_krdanta(&[], &pf, Krt::kta, &["pUrta"]);
    assert_has_krdanta(&[], &pf, Krt::ktavatu, &["pUrtavat"]);

    let murch = d("murCA~", Bhvadi);
    assert_has_krdanta(&[], &murch, Krt::kta, &["mUrta"]);
    assert_has_krdanta(&[], &murch, Krt::ktavatu, &["mUrtavat"]);

    let mad = d("madI~", Divadi);
    assert_has_krdanta(&[], &mad, Krt::kta, &["matta"]);
    assert_has_krdanta(&[], &mad, Krt::ktavatu, &["mattavat"]);
}

#[ignore]
#[test]
fn sutra_8_2_58() {
    assert_has_krdanta(&[], &d("vi\\dx~^", Tudadi), Krt::kta, &["vinna", "vitta"]);
}

#[ignore]
#[test]
fn sutra_8_2_59() {
    assert_has_krdanta(&[], &d("Bi\\di~^r", Rudhadi), Krt::kta, &["Binna", "Bitta"]);
}

#[ignore]
#[test]
fn sutra_8_2_60() {
    assert_has_krdanta(&[], &d("f\\", Juhotyadi), Krt::kta, &["fta", "fRa"]);
}

#[test]
fn sutra_8_2_62() {
    let ghrtasprk = upapada_krdanta("Gfta", &[], &d("spf\\Sa~", Tudadi), Krt::kvin);
    assert_has_sup_1s(&ghrtasprk, Pum, &["Gftaspfk"]);

    let halasprk = upapada_krdanta("hala", &[], &d("spf\\Sa~", Tudadi), Krt::kvin);
    assert_has_sup_1s(&halasprk, Pum, &["halaspfk"]);

    let mantrasprk = upapada_krdanta("mantra", &[], &d("spf\\Sa~", Tudadi), Krt::kvin);
    assert_has_sup_1s(&mantrasprk, Pum, &["mantraspfk"]);

    // TODO: asrAk, etc.
}

#[test]
fn sutra_8_2_64() {
    use Krt::kvip;
    let k = krdanta;
    assert_has_sup_1s(k(&["pra"], &d("Samu~", Tudadi), kvip), Pum, &["praSAn"]);
    assert_has_sup_1s(k(&["pra"], &d("tamu~", Tudadi), kvip), Pum, &["pratAn"]);
    assert_has_sup_1s(k(&["pra"], &d("damu~", Tudadi), kvip), Pum, &["pradAn"]);

    // ma?
    assert_has_sup_1s(k(&[], &d("Bi\\di~^r", Rudhadi), kvip), Pum, &["Bit"]);
    assert_has_sup_1s(k(&[], &d("Ci\\di~^r", Rudhadi), kvip), Pum, &["Cit"]);

    // dhAtoH?
    assert_has_sup_1s("idam", Napumsaka, &["idam"]);
    assert_has_sup_1s("kim", Napumsaka, &["kim"]);

    // padasya?
    let pratam = k(&["pra"], &d("tamu~", Tudadi), kvip);
    assert_has_sup_1d(&pratam, Pum, &["pratAmO"]);
    assert_has_sup_1p(&pratam, Pum, &["pratAmaH"]);
}

#[ignore]
#[test]
fn sutra_8_2_66() {
    let pada = Pada::from_text;
    assert_has_vakya(&pada("agnis"), &pada("atra"), &["agnir atra"]);
    assert_has_vakya(&pada("vAyus"), &pada("atra"), &["vAyur atra"]);

    // TODO: sandhi applied at pada before moving to vakya, which prevents match.
    let jush = krdanta(&[], &d("juzI~\\", Tudadi), Krt::kvip);
    // HACK: technically "saha", but use "sa" for convenience.
    let sajush = bahuvrihi("sa", jush);
    let sajuh = Subanta::new(&sajush, Stri, Vibhakti::Prathama, Vacana::Eka).into();
    assert_has_vakya(&sajuh, &pada("ftuBis"), &["sajUr ftuBiH"]);
    assert_has_vakya(&sajuh, &pada("deveBiH"), &["sajUr deeBiH"]);
}

#[test]
fn sutra_8_2_68() {
    assert_has_sup_3d("ahan", Pum, &["ahoByAm"]);
    assert_has_sup_3p("ahan", Pum, &["ahoBiH"]);
    // TODO: others
}

#[ignore]
#[test]
fn sutra_8_2_69() {
    let ahan = sup_1s("ahan", "ahan", Napumsaka);

    let dadati = Pada::from_text("dadAti");
    let bhunkte = Pada::from_text("BuNkte");
    assert_has_vakya(&ahan, &dadati, &["ahar dadAti"]);
    assert_has_vakya(&ahan, &bhunkte, &["ahar BuNkte"]);

    // a-supi?
    assert_has_sup_3d("ahan", Pum, &["ahoByAm"]);
    assert_has_sup_3p("ahan", Pum, &["ahoBiH"]);
}

#[test]
fn sutra_8_2_72() {
    let vid = d("vida~", Adadi);
    let vidvas = create_krdanta("vidvas", &[], &vid, Krt::Satf);
    assert_has_sup_3d(&vidvas, Pum, &["vidvadByAm"]);
    assert_has_sup_3p(&vidvas, Pum, &["vidvadBiH"]);

    let papivas = create_krdanta("papivas", &[], &d("pA\\", Bhvadi), Krt::kvasu);
    assert_has_sup_3d(&papivas, Pum, &["papivadByAm"]);
    assert_has_sup_3p(&papivas, Pum, &["papivadBiH"]);

    let ukhasras =
        create_upapada_krdanta("uKAsras", "uKA", &[], &d("sransu~\\", Bhvadi), Krt::kvip);
    assert_has_sup_3d(&ukhasras, Pum, &["uKAsradByAm"]);
    assert_has_sup_3p(&ukhasras, Pum, &["uKAsradBiH"]);

    let parnadhvas = create_upapada_krdanta(
        "parRaDvas",
        "parRa",
        &[],
        &d("Dvansu~\\", Bhvadi),
        Krt::kvip,
    );
    assert_has_sup_3d(&parnadhvas, Pum, &["parRaDvadByAm"]);
    assert_has_sup_3p(&parnadhvas, Pum, &["parRaDvadBiH"]);

    assert_has_sup_3d("anaquh", Pum, &["anaqudByAm"]);
    assert_has_sup_3p("anaquh", Pum, &["anaqudBiH"]);

    // saH?
    assert_has_sup_1s(&vidvas, Pum, &["vidvAn"]);
    assert_has_sup_1s(&papivas, Pum, &["papivAn"]);
    assert_has_sup_1s("anaquh", Pum, &["anaqvAn"]);
    assert_has_sup_ss("anaquh", Pum, &["anaqvan"]);

    // padasya?
    assert_has_sup_1d(&vidvas, Pum, &["vidvAMsO"]);
    assert_has_sup_1p(&vidvas, Pum, &["vidvAMsaH"]);
}

#[test]
fn sutra_8_2_73() {
    let cakas = &d("cakAsf~", Adadi);
    assert_has_lan(&[], &cakas, &["acakAt"]);
    assert_has_lan(&["anu"], &d("SAsu~", Adadi), &["anvaSAt"]);
    assert_has_krdanta(&[], &cakas, Krt::kvip, &["cakAs"]);
    // TODO: others
}

#[test]
fn sutra_8_2_74() {
    assert_has_sip(&[], &d("cakAsf~", Adadi), Lan, &["acakAH", "acakAt"]);
    assert_has_sip(&["anu"], &d("SAsu~", Adadi), Lan, &["anvaSAH", "anvaSAt"]);
}

#[test]
fn sutra_8_2_75() {
    assert_has_sip(&[], &d("Bi\\di~^r", Rudhadi), Lan, &["aBinaH", "aBinat"]);
    assert_has_sip(&[], &d("Ci\\di~^r", Rudhadi), Lan, &["acCinaH", "acCinat"]);
}

#[test]
fn sutra_8_2_76() {
    let gir = krdanta(&[], &d("gF", Kryadi), Krt::kvip);
    assert_has_sup_1s(&gir, Pum, &["gIH"]);

    let dhur = krdanta(&[], &d("DurvI~", Bhvadi), Krt::kvip);
    assert_has_sup_1s(&dhur, Pum, &["DUH"]);

    let pur = krdanta(&[], &d("pF", Kryadi), Krt::kvip);
    assert_has_sup_1s(&pur, Pum, &["pUH"]);

    let ashis = create_krdanta("ASis", &["AN"], &d("SAsu~\\", Adadi), Krt::kvip);
    assert_has_sup_1s(&ashis, Pum, &["ASIH"]);
}

#[test]
fn sutra_8_2_77() {
    // ra
    let stf = &d("stFY", Kryadi);
    assert_has_krdanta(&["AN"], &stf, Krt::kta, &["AstIrRa"]);
    assert_has_krdanta(&["vi"], &stf, Krt::kta, &["vistIrRa"]);
    assert_has_krdanta(&["vi"], &d("SF", Kryadi), Krt::kta, &["viSIrRa"]);
    assert_has_krdanta(&["ava"], &d("gurvI~", Bhvadi), Krt::kta, &["avagUrRa"]);
    // va
    assert_has_lat(&[], &d("divu~", Divadi), &["dIvyati"]);
    assert_has_lat(&[], &d("zivu~", Divadi), &["sIvyati"]);

    // TODO: others
}

#[test]
fn sutra_8_2_78() {
    assert_has_krdanta(&[], &d("hurCA~", Bhvadi), Krt::tfc, &["hUrCitf"]);
    assert_has_krdanta(&[], &d("murCA~", Bhvadi), Krt::tfc, &["mUrCitf"]);
    assert_has_krdanta(&[], &d("urvI~", Bhvadi), Krt::tfc, &["Urvitf"]);
    assert_has_krdanta(&[], &d("DurvI~", Bhvadi), Krt::tfc, &["DUrvitf"]);
    // hali
    assert_has_lat(&[], &d("ciri", Svadi), &["ciriRoti"]);
    assert_has_lat(&[], &d("jiri", Svadi), &["jiriRoti"]);
}

#[test]
fn sutra_8_2_79() {
    // bha
    use TaddhitaArtha::*;
    assert_has_artha_taddhita("Dur", TadVahati, T::yat, &["Durya"]);
    assert_has_artha_taddhita("div", TatraJata, T::yat, &["divya"]);
    // kur
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), VidhiLin, &["kuryAt"]);
    // chur
    assert_has_tip(&[], &d("Cura~", Tanadi), AshirLin, &["CuryAt"]);
}

#[test]
fn sutra_8_2_80() {
    assert_has_sup_2s("adas", Pum, &["amum"]);
    assert_has_sup_2d("adas", Pum, &["amU"]);
    assert_has_sup_2p("adas", Pum, &["amUn"]);
    // TODO:
    assert_has_sup_3s("adas", Pum, &["amunA"]);
    assert_has_sup_3d("adas", Pum, &["amUByAm"]);
    // aseH
    assert_has_sup_1s("adas", Napumsaka, &["adaH"]);
}

#[test]
fn sutra_8_2_81() {
    assert_has_sup_1p("adas", Pum, &["amI"]);
    assert_has_sup_3p("adas", Pum, &["amIBiH"]);
    assert_has_sup_4p("adas", Pum, &["amIByaH"]);
    assert_has_sup_6p("adas", Pum, &["amIzAm"]);
    assert_has_sup_7p("adas", Pum, &["amIzu"]);
}
