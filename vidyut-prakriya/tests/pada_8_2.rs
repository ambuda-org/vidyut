extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San])
}

fn yan(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Yan])
}

fn prati(text: &str) -> Pratipadika {
    Pratipadika::builder().text(text).build().unwrap()
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

    let vah = Dhatu::new("va\\ha~^", Bhvadi);
    assert_has_krdanta(&[], &vah, Krt::tfc, &["voQf"]);
    assert_has_krdanta(&[], &vah, Krt::tumun, &["voQum"]);
    assert_has_krdanta(&[], &vah, Krt::tavya, &["voQavya"]);
    // TODO: jalAzAt, dityavAw
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

    let lih = Dhatu::new("li\\ha~^", Bhvadi);
    assert_has_krdanta(&[], &lih, Krt::tfc, &["leQf"]);
    assert_has_krdanta(&[], &lih, Krt::tumun, &["leQum"]);
    assert_has_krdanta(&[], &lih, Krt::tavya, &["leQavya"]);
    // TODO: kAzWaDak, etc.
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

#[ignore]
#[test]
fn sutra_8_2_54() {
    let styai = Dhatu::new("styE\\", Bhvadi);
    assert_has_krdanta(&["pra"], &styai, Krt::kta, &["prastIma", "prastIta"]);
    assert_has_krdanta(&[], &styai, Krt::ktavatu, &["prastImavat", "prastItavat"]);
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
