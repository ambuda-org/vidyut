extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;

fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San])
}

fn yan(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Yan])
}

fn prati_udit(text: &str) -> Pratipadika {
    Pratipadika::builder()
        .text(text)
        .is_udit(true)
        .build()
        .unwrap()
}

pub fn assert_has_krta(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_krdanta(&prefixes, &dhatu, Krt::kta, &expected);
}

#[test]
fn sutra_8_2_7() {
    assert_has_subantas("rAjan", Pum, Prathama, Eka, &["rAjA"]);
    assert_has_subantas("rAjan", Pum, Trtiya, Dvi, &["rAjaByAm"]);
    assert_has_subantas("rAjan", Pum, Trtiya, Bahu, &["rAjaBiH"]);
    assert_has_taddhitanta(&prati("rAjan"), T::tal, &["rAjatA"]);
    assert_has_taddhitanta(&prati("rAjan"), T::tarap, &["rAjatara"]);
    assert_has_taddhitanta(&prati("rAjan"), T::tamap, &["rAjatama"]);
}

#[test]
fn sutra_8_2_8() {
    assert_has_subantas("rAjan", Pum, Sambodhana, Eka, &["rAjan"]);
    assert_has_subantas("takzan", Pum, Sambodhana, Eka, &["takzan"]);
}

#[test]
fn sutra_8_2_9() {
    // makAra-anta
    assert_has_taddhitanta(&prati("kim"), T::matup, &["kiMvat"]);
    assert_has_taddhitanta(&prati("Sam"), T::matup, &["SaMvat"]);
    // makAra-upadha
    assert_has_taddhitanta(&prati("SamI"), T::matup, &["SamIvat"]);
    assert_has_taddhitanta(&prati("dAqimI"), T::matup, &["dAqimIvat"]);
    // avarRa-anta
    assert_has_taddhitanta(&prati("vfkza"), T::matup, &["vfkzavat"]);
    assert_has_taddhitanta(&prati("plakza"), T::matup, &["plakzavat"]);
    assert_has_taddhitanta(&prati("KawvA"), T::matup, &["KawvAvat"]);
    assert_has_taddhitanta(&prati("mAlA"), T::matup, &["mAlAvat"]);
    // avarRa-upaDa
    assert_has_taddhitanta(&prati("payas"), T::matup, &["payasvat"]);
    assert_has_taddhitanta(&prati("yaSas"), T::matup, &["yaSasvat"]);
    assert_has_taddhitanta(&prati("BAs"), T::matup, &["BAsvat"]);
    // mAd?
    assert_has_taddhitanta(&prati("agni"), T::matup, &["agnimat"]);
    assert_has_taddhitanta(&prati("vAyu"), T::matup, &["vAyumat"]);
    // avAdibhyaH?
    assert_has_taddhitanta(&prati("yava"), T::matup, &["yavamat"]);
    assert_has_taddhitanta(&prati("dalmi"), T::matup, &["dalmimat"]);
    assert_has_taddhitanta(&prati("Urmi"), T::matup, &["Urmimat"]);
}

#[test]
fn sutra_8_2_10() {
    assert_has_taddhitanta(&prati("agnicit"), T::matup, &["agnicitvat"]);
    assert_has_taddhitanta(&prati("vidyut"), T::matup, &["vidyutvat"]);
    assert_has_taddhitanta(&prati("marut"), T::matup, &["marutvat"]);
    assert_has_taddhitanta(&prati("dfzad"), T::matup, &["dfzadvat"]);
}

#[test]
fn sutra_8_2_18() {
    use Purusha::*;
    let kfp = d("kfpU~\\", Bhvadi);
    assert_has_atmane_tinanta(&[], &kfp, Lut, Prathama, Eka, &["kalptA", "kalpitA"]);
    assert_has_atmane_tinanta(&[], &kfp, Lut, Prathama, Dvi, &["kalptArO", "kalpitArO"]);
    assert_has_atmane_tinanta(&[], &kfp, Lut, Prathama, Bahu, &["kalptAraH", "kalpitAraH"]);
    assert_has_lat_p(&[], &san(&kfp), &["cikxpsati"]);
    assert_has_krdanta(&[], &kfp, Krt::kta, &["kxpta"]);
    assert_has_krdanta(&[], &kfp, Krt::ktavatu, &["kxptavat"]);
}

#[test]
fn sutra_8_2_19() {
    let ay = d("aya~\\", Bhvadi);
    assert_has_lat_a(&["pra"], &ay, &["plAyate"]);
    assert_has_lat_a(&["parA"], &ay, &["palAyate"]);
    assert_has_lat_a(&["pari"], &ay, &["palyayate"]);

    assert_has_lat_a(&["prati"], &ay, &["pratyayate"]);
    assert_has_krdanta(&["nis"], &ay, Krt::lyuw, &["nirayaRa"]);
    assert_has_krdanta(&["dus"], &ay, Krt::lyuw, &["durayaRa"]);

    // extra examples from the Siddhantakaumudi
    assert_has_lat_a(&["nis"], &ay, &["nirayate"]);
    assert_has_lat_a(&["dus"], &ay, &["durayate"]);
    assert_has_lat_a(&["nir"], &ay, &["nilayate"]);
    assert_has_lat_a(&["dur"], &ay, &["dulayate"]);
}

#[test]
fn sutra_8_2_20() {
    use Purusha::*;
    let gf = d("gF", Tudadi);
    assert_has_atmane_tinanta(&["ni"], &yan(&gf), Lat, Prathama, Eka, &["nijegilyate"]);
    assert_has_atmane_tinanta(&["ni"], &yan(&gf), Lat, Prathama, Dvi, &["nijegilyete"]);
    assert_has_atmane_tinanta(&["ni"], &yan(&gf), Lat, Prathama, Bahu, &["nijegilyante"]);
    // yaNi
    assert_has_lat_karmani(&["ni"], &gf, &["nigIryate"]);
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
    assert_has_subantas_p(&prati_udit("gomat"), Pum, Prathama, Eka, &["gomAn"]);
    assert_has_subantas_p(&prati_udit("yavamat"), Pum, Prathama, Eka, &["yavamAn"]);
    assert_has_subantas_p(&prati_udit("kftavat"), Pum, Prathama, Eka, &["kftavAn"]);
    assert_has_subantas_p(&prati_udit("hatavat"), Pum, Prathama, Eka, &["hatavAn"]);
    assert_has_subantas_p(&prati_udit("Sreyas"), Pum, Prathama, Eka, &["SreyAn"]);
    assert_has_subantas_p(&prati_udit("BUyas"), Pum, Prathama, Eka, &["BUyAn"]);
    // TODO: others
}

#[ignore]
#[test]
fn sutra_8_2_24() {
    assert_has_subantas("mAtf", Stri, Sasthi, Eka, &["mAtuH"]);
    assert_has_subantas("pitf", Pum, Sasthi, Eka, &["pituH"]);
    assert_has_subantas("Urj", Pum, Prathama, Eka, &["Urk"]);
    assert_has_lan(&[], &d("mfjU~", Adadi), &["amArw"]);
}

#[test]
fn sutra_8_2_26() {
    use Purusha::*;
    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_atmane_tinanta(&[], &bhid, Lun, Prathama, Eka, &["aBitta"]);
    assert_has_atmane_tinanta(&[], &bhid, Lun, Madhyama, Eka, &["aBitTAH"]);

    let chid = d("Ci\\di~^r", Rudhadi);
    assert_has_atmane_tinanta(&[], &chid, Lun, Prathama, Eka, &["acCitta"]);
    assert_has_atmane_tinanta(&[], &chid, Lun, Madhyama, Eka, &["acCitTAH"]);

    let vas = d("va\\sa~", Bhvadi);
    assert_has_parasmai_tinanta(&[], &vas, Lun, Prathama, Dvi, &["avAttAm"]);
    assert_has_parasmai_tinanta(&[], &vas, Lun, Madhyama, Bahu, &["avAtta"]);

    // JalaH
    let man = d("ma\\na~\\", Divadi);
    assert_has_atmane_tinanta(&[], &man, Lun, Prathama, Eka, &["amaMsta"]);
    assert_has_atmane_tinanta(&[], &man, Lun, Madhyama, Eka, &["amaMsTAH"]);

    // Jali
    assert_has_atmane_tinanta(&[], &bhid, Lun, Prathama, Dvi, &["aBitsAtAm"]);
    assert_has_atmane_tinanta(&[], &bhid, Lun, Prathama, Bahu, &["aBitsata"]);

    // TODO: others
}

#[test]
fn sutra_8_2_27() {
    use Purusha::*;
    let kf = d("qukf\\Y", Tanadi);
    assert_has_atmane_tinanta(&[], &kf, Lun, Prathama, Eka, &["akfta"]);
    assert_has_atmane_tinanta(&[], &kf, Lun, Madhyama, Eka, &["akfTAH"]);

    let hf = d("hf\\Y", Bhvadi);
    assert_has_atmane_tinanta(&[], &hf, Lun, Prathama, Eka, &["ahfta"]);
    assert_has_atmane_tinanta(&[], &hf, Lun, Madhyama, Eka, &["ahfTAH"]);

    // hrasvAt?
    assert_has_lun_a(&[], &d("cyu\\N", Bhvadi), &["acyozwa"]);
    assert_has_lun_a(&[], &d("plu\\N", Bhvadi), &["aplozwa"]);

    // aNgAt?
    let lu = d("lUY", Kryadi);
    assert_has_parasmai_tinanta(&[], &lu, Lun, Prathama, Dvi, &["alAvizwAm"]);
    assert_has_parasmai_tinanta(&[], &lu, Lun, Prathama, Bahu, &["alAvizuH"]);

    // Jali?
    assert_has_atmane_tinanta(&[], &kf, Lun, Prathama, Dvi, &["akfzAtAm"]);
    assert_has_atmane_tinanta(&[], &kf, Lun, Prathama, Bahu, &["akfzata"]);

    // TODO: others
}

#[test]
fn sutra_8_2_28() {
    use Purusha::*;
    // TODO: adAvIt?
    // assert_has_lun_p(&[], &d("du\\", Bhvadi), &["adAvIt"]);
    assert_has_lun_p(&[], &d("lUY", Kryadi), &["alAvIt"]);
    assert_has_lun_p(&[], &d("zivu~", Divadi), &["asevIt"]);
    assert_has_lun_p(&[], &d("kuza~", Kryadi), &["akozIt"]);
    assert_has_lun_p(&[], &d("muza~", Kryadi), &["amozIt"]);

    // iwaH?
    assert_has_lun_p(&[], &d("qukf\\Y", Tanadi), &["akArzIt"]);
    assert_has_lun_p(&[], &d("hf\\Y", Bhvadi), &["ahArzIt"]);

    // Iwi?
    let lu = d("lUY", Kryadi);
    assert_has_parasmai_tinanta(&[], &lu, Lun, Prathama, Dvi, &["alAvizwAm"]);
    assert_has_parasmai_tinanta(&[], &lu, Lun, Prathama, Bahu, &["alAvizuH"]);
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
    let sah = Dhatu::new("zaha~\\", Bhvadi);
    assert_has_krdanta(&[], &sah, Krt::tfc, &["soQf", "sahitf"]);
    assert_has_krdanta(&[], &sah, Krt::tumun, &["soQum", "sahitum"]);
    assert_has_krdanta(&[], &sah, Krt::tavya, &["soQavya", "sahitavya"]);
    assert_has_subantas("jalAzAh", Pum, Prathama, Eka, &["jalAzAw"]);

    let vah = Dhatu::new("va\\ha~^", Bhvadi);
    assert_has_krdanta(&[], &vah, Krt::tfc, &["voQf"]);
    assert_has_krdanta(&[], &vah, Krt::tumun, &["voQum"]);
    assert_has_krdanta(&[], &vah, Krt::tavya, &["voQavya"]);
    assert_has_subantas("prazWavAh", Pum, Prathama, Eka, &["prazWavAw"]);
    assert_has_subantas("dityavAh", Pum, Prathama, Eka, &["dityavAw"]);
}

#[test]
fn sutra_8_2_32() {
    let dah = Dhatu::new("da\\ha~", Bhvadi);
    assert_has_krdanta(&[], &dah, Krt::tfc, &["dagDf"]);
    assert_has_krdanta(&[], &dah, Krt::tumun, &["dagDum"]);
    assert_has_krdanta(&[], &dah, Krt::tavya, &["dagDavya"]);

    let duh = Dhatu::new("du\\ha~^", Bhvadi);
    assert_has_krdanta(&[], &duh, Krt::tfc, &["dogDf"]);
    assert_has_krdanta(&[], &duh, Krt::tumun, &["dogDum"]);
    assert_has_krdanta(&[], &duh, Krt::tavya, &["dogDavya"]);

    let lih = Dhatu::new("li\\ha~^", Adadi);
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
    assert_has_lrt_a(&["ni"], &guh, &["niGokzyate", "nigUhizyate"]);
    assert_has_dhvam(
        &["ni"],
        &guh,
        Lun,
        &["nyaGUQvam", "nyaGukzaDvam", "nyagUhiDvam", "nyagUhiQvam"],
    );
    assert_has_krdanta(&[], &guh, Krt::kvip, &["Guw"]);

    let duh = d("du\\ha~^", Adadi);
    assert_has_lrt_a(&[], &duh, &["Dokzyate"]);
    assert_has_dhvam(&[], &duh, Lun, &["aDugDvam", "aDukzaDvam"]);
    assert_has_krdanta(&[], &duh, Krt::kvip, &["Duk"]);

    // baS?
    assert_has_lrt(&[], &d("kru\\Da~", Divadi), &["krotsyati"]);

    // Jazantasya?
    assert_has_lrt_p(&[], &d("qudA\\Y", Juhotyadi), &["dAsyati"]);

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
fn sutra_8_2_40() {
    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_krdanta(&[], &labh, Krt::tfc, &["labDf"]);
    assert_has_krdanta(&[], &labh, Krt::tumun, &["labDum"]);
    assert_has_krdanta(&[], &labh, Krt::tavya, &["labDavya"]);
    assert_has_ta(&[], &labh, Lun, &["alabDa"]);
    assert_has_thaas(&[], &labh, Lun, &["alabDAH"]);

    let duh = Dhatu::new("du\\ha~^", Bhvadi);
    assert_has_krdanta(&[], &duh, Krt::tfc, &["dogDf"]);
    assert_has_krdanta(&[], &duh, Krt::tumun, &["dogDum"]);
    assert_has_krdanta(&[], &duh, Krt::tavya, &["dogDavya"]);
    assert_has_ta(&[], &duh, Lun, &["adugDa", "aDukzata"]);
    assert_has_thaas(&[], &duh, Lun, &["adugDAH", "aDukzaTAH"]);

    let lih = Dhatu::new("li\\ha~^", Adadi);
    assert_has_krdanta(&[], &lih, Krt::tfc, &["leQf"]);
    assert_has_krdanta(&[], &lih, Krt::tumun, &["leQum"]);
    assert_has_krdanta(&[], &lih, Krt::tavya, &["leQavya"]);
    assert_has_ta(&[], &lih, Lun, &["alIQa", "alikzata"]);
    assert_has_thaas(&[], &lih, Lun, &["alIQAH", "alikzaTAH"]);

    let budh = d("bu\\Da~\\", Divadi);
    assert_has_krdanta(&[], &budh, Krt::tfc, &["bodDf"]);
    assert_has_krdanta(&[], &budh, Krt::tumun, &["bodDum"]);
    assert_has_krdanta(&[], &budh, Krt::tavya, &["bodDavya"]);
    assert_has_ta(&[], &budh, Lun, &["abudDa"]);
    assert_has_thaas(&[], &budh, Lun, &["abudDAH"]);

    // aDaH?
    let dha = d("quDA\\Y", Juhotyadi);
    assert_has_tas(&[], &dha, Lat, &["DattaH"]);
    assert_has_thas(&[], &dha, Lat, &["DatTaH"]);
}

#[test]
fn sutra_8_2_41() {
    let pish = Dhatu::new("pi\\zx~", Rudhadi);
    assert_has_lrt(&[], &pish, &["pekzyati"]);
    assert_has_lrn(&[], &pish, &["apekzyat"]);
    assert_has_lat(&[], &san(&pish), &["pipikzati"]);

    let lih = Dhatu::new("li\\ha~^", Adadi);
    assert_has_lrt_p(&[], &lih, &["lekzyati"]);
    assert_has_lrn_p(&[], &lih, &["alekzyat"]);
    assert_has_lat_p(&[], &san(&lih), &["lilikzati"]);

    // si?
    assert_has_lat_p(&[], &pish, &["pinazwi"]);
    assert_has_lat_p(&[], &lih, &["leQi"]);
}

#[test]
fn sutra_8_2_42() {
    let stf = d("stFY", Kryadi);
    assert_has_krdanta(&["AN"], &stf, Krt::kta, &["AstIrRa"]);
    assert_has_krdanta(&["vi"], &stf, Krt::kta, &["vistIrRa"]);
    assert_has_krdanta(&["vi"], &d("SFY", Kryadi), Krt::kta, &["viSIrRa"]);
    assert_has_krdanta(&["ni"], &d("gF", Tudadi), Krt::kta, &["nigIrRa"]);
    assert_has_krdanta(&["ava"], &d("gUrI~\\", Divadi), Krt::kta, &["avagUrRa"]);
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
    let kshi = Dhatu::new("kzi\\", Bhvadi);
    assert_has_krdanta(&[], &kshi, Krt::kta, &["kzIRa"]);
    // TODO: akzita
}

#[test]
fn sutra_8_2_51() {
    let sus = Dhatu::new("Su\\za~", Divadi);
    assert_has_krdanta(&[], &sus, Krt::kta, &["Suzka"]);
    assert_has_krdanta(&[], &sus, Krt::ktavatu, &["Suzkavat"]);
}

#[test]
fn sutra_8_2_52() {
    let pac = Dhatu::new("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::kta, &["pakva"]);
    assert_has_krdanta(&[], &pac, Krt::ktavatu, &["pakvavat"]);
}

#[test]
fn sutra_8_2_53() {
    let kzai = Dhatu::new("kzE\\", Bhvadi);
    assert_has_krdanta(&[], &kzai, Krt::kta, &["kzAma"]);
    assert_has_krdanta(&[], &kzai, Krt::ktavatu, &["kzAmavat"]);
}

#[test]
fn sutra_8_2_54() {
    let styai = Dhatu::new("styE\\", Bhvadi);
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
    let phal = Dhatu::new("YiPalA~", Bhvadi);
    assert_has_krdanta(&[], &phal, Krt::kta, &["Pulla"]);
    assert_has_krdanta(&[], &phal, Krt::ktavatu, &["Pullavat"]);

    let kzib = Dhatu::new("kzIbf~\\", Bhvadi);
    assert_has_krdanta(&[], &kzib, Krt::kta, &["kzIba"]);

    let kfs = Dhatu::new("kfSa~", Divadi);
    assert_has_krdanta(&[], &kfs, Krt::kta, &["kfSa"]);

    let lagh = Dhatu::new("lAGf~\\", Bhvadi);
    assert_has_krdanta(&["ud"], &lagh, Krt::kta, &["ullAGa"]);

    // TODO: allow praPulta
    // assert_has_krdanta(&["pra"], &phal, Krt::kta, &["praPulta"]);
    assert_has_krdanta(&["pra"], &kzib, Krt::kta, &["prakzIbita"]);
    assert_has_krdanta(&["pra"], &kfs, Krt::kta, &["prakfSita"]);
    assert_has_krdanta(&["pra", "ud"], &lagh, Krt::kta, &["prollAGita"]);
}

#[test]
fn sutra_8_2_56() {
    assert_has_krdanta(&[], &d("Ru\\da~^", Tudadi), Krt::kta, &["nunna", "nutta"]);
    assert_has_krdanta(&[], &d("vi\\da~\\", Rudhadi), Krt::kta, &["vinna", "vitta"]);
    assert_has_krdanta(
        &["sam"],
        &d("undI~", Rudhadi),
        Krt::kta,
        &["samunna", "samutta"],
    );
    assert_has_krdanta(&[], &d("trE\\N", Bhvadi), Krt::kta, &["trARa", "trAta"]);
    assert_has_krdanta(&[], &d("GrA\\", Bhvadi), Krt::kta, &["GrARa", "GrAta"]);
    assert_has_krdanta(&[], &d("hrI\\", Juhotyadi), Krt::kta, &["hrIRa", "hrIta"]);
    // only for vinatti
    assert_has_krdanta(&[], &d("vida~", Adadi), Krt::kta, &["vidita"]);
    assert_has_krdanta(&[], &d("vi\\da~\\", Divadi), Krt::kta, &["vinna"]);
    assert_has_krdanta(&[], &d("vi\\dx~^", Tudadi), Krt::kta, &["vinna"]);
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

#[ignore]
#[test]
fn sutra_8_2_80() {
    assert_has_subantas("adas", Pum, Dvitiya, Eka, &["amum"]);
    assert_has_subantas("adas", Pum, Dvitiya, Dvi, &["amU"]);
    assert_has_subantas("adas", Pum, Dvitiya, Bahu, &["amUn"]);
    // TODO:
    // assert_has_subantas("adas", Pum, Trtiya, Eka, &["amunA"]);
    assert_has_subantas("adas", Pum, Trtiya, Dvi, &["amUByAm"]);
    // aseH
    assert_has_subantas("adas", Napumsaka, Prathama, Eka, &["adaH"]);
}

#[test]
fn sutra_8_2_81() {
    assert_has_subantas("adas", Pum, Prathama, Bahu, &["amI"]);
    assert_has_subantas("adas", Pum, Trtiya, Bahu, &["amIBiH"]);
    assert_has_subantas("adas", Pum, Caturthi, Bahu, &["amIByaH"]);
    assert_has_subantas("adas", Pum, Sasthi, Bahu, &["amIzAm"]);
    assert_has_subantas("adas", Pum, Saptami, Bahu, &["amIzu"]);
}
