extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::*;

fn assert_has_san_tip(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_tip(prefixes, &san(&dhatu), Lat, expected);
}

fn assert_has_san_ta(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_ta(prefixes, &san(&dhatu), Lat, expected);
}

fn assert_has_kvasu_su(dhatu: &Dhatu, expected: &[&str]) {
    let vas = krdanta(&[], dhatu, Krt::kvasu);
    assert_has_sup_1s(&vas, Pum, expected);
}

#[test]
fn sutra_7_2_1() {
    assert_has_tip(&[], &d("ci\\Y", Svadi), Lun, &["acEzIt"]);
    assert_has_tip(&[], &d("RI\\Y", Bhvadi), Lun, &["anEzIt"]);
    assert_has_tip(&[], &d("lUY", Kryadi), Lun, &["alAvIt"]);
    assert_has_tip(&[], &d("pUY", Kryadi), Lun, &["apAvIt"]);
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lun, &["akArzIt"]);
    assert_has_tip(&[], &d("hf\\Y", Bhvadi), Lun, &["ahArzIt"]);

    let kutadi = |u| d_kutadi(u, Tudadi);
    assert_has_tip(&["ni"], &kutadi("RU"), Lun, &["nyanuvIt"]);
    assert_has_tip(&["ni"], &kutadi("DU"), Lun, &["nyaDuvIt"]);

    assert_has_ta(&[], &d("cyu\\N", Bhvadi), Lun, &["acyozwa"]);
    assert_has_ta(&[], &d("plu\\N", Bhvadi), Lun, &["aplozwa"]);
}

#[test]
fn sutra_7_2_2() {
    assert_has_tip(&[], &d("kzara~", Bhvadi), Lun, &["akzArIt"]);
    assert_has_tip(&[], &d("tsara~", Bhvadi), Lun, &["atsArIt"]);
    assert_has_tip(&[], &d("jvala~", Bhvadi), Lun, &["ajvAlIt"]);
    assert_has_tip(&[], &d("hmala~", Bhvadi), Lun, &["ahmAlIt"]);

    assert_has_tip(&["ni"], &d("Kura~", Tudadi), Lun, &["nyaKorIt"]);
    assert_has_tip(&["ni"], &d("mIla~", Svadi), Lun, &["nyamIlIt"]);

    // TODO: awIt, aSIt

    assert_has_tip(&[], &d("vaBra~", Bhvadi), Lun, &["avaBrIt"]);
    assert_has_tip(&[], &d("Svalla~", Bhvadi), Lun, &["aSvallIt"]);
}

#[test]
fn sutra_7_2_3() {
    assert_has_tip(&[], &d("vada~", Bhvadi), Lun, &["avAdIt"]);
    assert_has_tip(&[], &d("vraja~", Bhvadi), Lun, &["avrAjIt"]);
    assert_has_tip(&[], &d("Bi\\di~^r", Rudhadi), Lun, &["aBEtsIt", "aBidat"]);
    assert_has_tip(&[], &d("Ci\\di~^r", Rudhadi), Lun, &["acCEtsIt", "acCidat"]);
    assert_has_tip(&[], &d("ru\\Di~^r", Rudhadi), Lun, &["arOtsIt", "aruDat"]);
    assert_has_tip(&[], &d("ra\\nja~^", Bhvadi), Lun, &["arANkzIt"]);
    assert_has_tip(&[], &d("za\\nja~", Bhvadi), Lun, &["asANkzIt"]);
    assert_has_tas(&["ud"], &d("va\\ha~^", Bhvadi), Lakara::Lun, &["udavoQAm"]);
}

#[test]
fn sutra_7_2_4() {
    assert_has_tip(&[], &d("divu~", Divadi), Lun, &["adevIt"]);
    assert_has_tip(&[], &d("zivu~", Divadi), Lun, &["asevIt"]);
    assert_has_tip(&[], &d("kuza~", Kryadi), Lun, &["akozIt"]);
    assert_has_tip(&[], &d("muza~", Kryadi), Lun, &["amozIt"]);
    assert_has_tip(&[], &d("lUY", Kryadi), Lun, &["alAvIt"]);
}

#[test]
fn sutra_7_2_5() {
    assert_has_tip(&[], &d("graha~^", Kryadi), Lun, &["agrahIt"]);
    assert_has_tip(&[], &d("syamu~", Divadi), Lun, &["asyamIt"]);
    assert_has_tip(&[], &d("vyaya~^", Bhvadi), Lun, &["avyayIt"]);
    assert_has_tip(&[], &d("wuvama~", Bhvadi), Lun, &["avamIt"]);
    assert_has_tip(&[], &d("kzaRu~^", Kryadi), Lun, &["akzaRIt"]);
    assert_has_tip(&[], &d("Svasa~", Adadi), Lun, &["aSvasIt"]);
    assert_has_tip(&[], &d("jAgf", Adadi), Lun, &["ajAgarIt"]);
    assert_has_tip(&[], &d("rage~", Bhvadi), Lun, &["aragIt"]);
    assert_has_tip(&[], &d("kaKe~", Kryadi), Lun, &["akaKIt"]);
    // Ignore OnayIt and aSvayIt, which are chAndasa forms.
}

#[test]
fn sutra_7_2_6() {
    let urnu = d("UrRuY", Adadi);
    assert_has_tip(
        &["pra"],
        &urnu,
        Lun,
        &["prOrRavIt", "prOrRAvIt", "prOrRuvIt"],
    );
}

#[test]
fn sutra_7_2_7() {
    assert_has_tip(&[], &d("kaRa~", Bhvadi), Lun, &["akaRIt", "akARIt"]);
    assert_has_tip(&[], &d("raRa~", Bhvadi), Lun, &["araRIt", "arARIt"]);

    let kuw = d_kutadi("kuwa~", Tudadi);
    let puw = d_kutadi("puwa~", Tudadi);

    // Various counterexamples
    assert_has_tip(&[], &d("divu~", Divadi), Lun, &["adevIt"]);
    assert_has_tip(&[], &d("zivu~", Divadi), Lun, &["asevIt"]);
    assert_has_tip(&["ni"], &kuw, Lun, &["nyakuwIt"]);
    assert_has_tip(&["ni"], &puw, Lun, &["nyapuwIt"]);
    assert_has_tip(&[], &d("takza~", Bhvadi), Lun, &["atakzIt"]);
    assert_has_tip(&[], &d("rakza~", Divadi), Lun, &["arakzIt"]);
    assert_has_tip(&[], &d("cakAsf~", Adadi), Lun, &["acakAsIt"]);
    assert_has_tip(&[], &d("qupa\\ca~^z", Bhvadi), Lun, &["apAkzIt"]);

    // TODO: aSIt, awIt
}

#[test]
fn sutra_7_2_8() {
    let iz = d("ISa~\\", Adadi);
    assert_has_krdanta(&[], &iz, Krt::tfc, &["ISitf"]);
    assert_has_krdanta(&[], &iz, Krt::tumun, &["ISitum"]);
    assert_has_krdanta(&[], &iz, Krt::varac, &["ISvara"]);
    let dip = d("dIpI~\\", Divadi);
    assert_has_krdanta(&[], &dip, Krt::tfc, &["dIpitf"]);
    assert_has_krdanta(&[], &dip, Krt::tumun, &["dIpitum"]);
    assert_has_krdanta(&[], &dip, Krt::ra, &["dIpra"]);
    let bhas = d("Basa~", Juhotyadi);
    assert_has_krdanta(&[], &bhas, Krt::tfc, &["Basitf"]);
    assert_has_krdanta(&[], &bhas, Krt::tumun, &["Basitum"]);
    assert_has_krdanta(&[], &bhas, Krt::manin, &["Basman"]);
    let yac = d("quyAcf~^", Bhvadi);
    assert_has_krdanta(&[], &yac, Krt::tfc, &["yAcitf"]);
    assert_has_krdanta(&[], &yac, Krt::tumun, &["yAcitum"]);
    assert_has_krdanta(&[], &yac, Krt::naN, &["yAcYA"]);
}

#[test]
fn sutra_7_2_9() {
    // ktic
    let tan = d("tanu~^", Tanadi);
    assert_has_krdanta(&[], &tan, Krt::tfc, &["tanitf"]);
    assert_has_krdanta(&[], &tan, Krt::tumun, &["tanitum"]);
    assert_has_krdanta(&[], &tan, Krt::ktic, &["tanti"]);
    let dip = d("dIpI~\\", Divadi);
    assert_has_krdanta(&[], &dip, Krt::tfc, &["dIpitf"]);
    assert_has_krdanta(&[], &dip, Krt::tumun, &["dIpitum"]);
    assert_has_krdanta(&[], &dip, Krt::ktin, &["dIpti"]);

    // tun
    let sac = d("zaca~\\", Bhvadi);
    assert_has_krdanta(&[], &sac, Krt::tfc, &["sacitf"]);
    assert_has_krdanta(&[], &sac, Krt::tumun, &["sacitum"]);
    assert_has_krdanta(&[], &sac, Unadi::tun, &["saktu"]);

    // zwran
    let pat = d("patx~", Bhvadi);
    assert_has_krdanta(&[], &pat, Krt::tfc, &["patitf"]);
    assert_has_krdanta(&[], &pat, Krt::tumun, &["patitum"]);
    assert_has_krdanta(&[], &pat, Unadi::zwran, &["pattra"]);
    assert_has_krdanta(&[], &tan, Unadi::zwran, &["tantra"]);

    // tan
    let has = d("hase~", Bhvadi);
    assert_has_krdanta(&[], &has, Krt::tfc, &["hasitf"]);
    assert_has_krdanta(&[], &has, Krt::tumun, &["hasitum"]);
    assert_has_krdanta(&[], &has, Unadi::tan, &["hasta"]);

    let lu = d("lUY", Kryadi);
    assert_has_krdanta(&[], &lu, Krt::tfc, &["lavitf"]);
    assert_has_krdanta(&[], &lu, Krt::tumun, &["lavitum"]);
    assert_has_krdanta(&[], &lu, Unadi::tan, &["lota"]);

    let pu = d("pUY", Kryadi);
    assert_has_krdanta(&[], &pu, Krt::tfc, &["pavitf"]);
    assert_has_krdanta(&[], &pu, Krt::tumun, &["pavitum"]);
    assert_has_krdanta(&[], &pu, Unadi::tan, &["pota"]);

    let dhurv = d("DurvI~", Kryadi);
    assert_has_krdanta(&[], &dhurv, Krt::tfc, &["DUrvitf"]);
    assert_has_krdanta(&[], &dhurv, Krt::tumun, &["DUrvitum"]);
    // TODO: revisit.
    // assert_has_krdanta(&[], &dhurv, Krt::tan, &["DUrta"]);

    // But, not kta
    assert_has_krdanta(&[], &has, Krt::kta, &["hasita"]);

    // kTan
    let kuz = d("kuza~", Bhvadi);
    assert_has_krdanta(&[], &kuz, Krt::tfc, &["kozitf"]);
    assert_has_krdanta(&[], &kuz, Krt::tumun, &["kozitum"]);
    assert_has_krdanta(&[], &kuz, Unadi::kTan, &["kuzWa"]);

    let kash = d("kASf~", Bhvadi);
    assert_has_krdanta(&[], &kash, Krt::tfc, &["kASitf"]);
    assert_has_krdanta(&[], &kash, Krt::tumun, &["kASitum"]);
    assert_has_krdanta(&[], &kash, Unadi::kTan, &["kAzWa"]);

    // ksi
    assert_has_krdanta(&[], &kuz, Unadi::ksi, &["kukzi"]);

    // suk
    let iz = d("izu~", Tudadi);
    assert_has_krdanta(&[], &iz, Krt::tfc, &["ezitf", "ezwf"]);
    assert_has_krdanta(&[], &iz, Krt::tumun, &["ezitum", "ezwum"]);
    assert_has_krdanta(&[], &iz, Unadi::ksu, &["ikzu"]);

    // kzaran
    let ash = d("aSU~\\", Svadi);
    assert_has_krdanta(&[], &ash, Krt::tfc, &["aSitf", "azwf"]);
    assert_has_krdanta(&[], &ash, Krt::tumun, &["aSitum", "azwum"]);
    assert_has_krdanta(&[], &ash, Unadi::sara, &["akzara"]);

    // kan
    let shal = d("Sala~", Bhvadi);
    assert_has_krdanta(&[], &shal, Krt::tfc, &["Salitf"]);
    assert_has_krdanta(&[], &shal, Krt::tumun, &["Salitum"]);
    assert_has_krdanta(&[], &shal, Unadi::kan, &["Salka"]);

    // sa
    let vad = d("vada~", Svadi);
    assert_has_krdanta(&[], &vad, Krt::tfc, &["vaditf"]);
    assert_has_krdanta(&[], &vad, Krt::tumun, &["vaditum"]);
    assert_has_krdanta(&[], &vad, Unadi::sa, &["vatsa"]);

    // kfti?
    assert_has_lat(&[], &d("rudi~r", Adadi), &["roditi"]);
    assert_has_lat(&[], &d("Yizva\\pa~", Adadi), &["svapiti"]);
}

#[test]
fn sutra_7_2_10() {
    fn assert_has_tfc(p: &[&str], dhatu: &Dhatu, expected: &[&str]) {
        assert_has_krdanta(p, dhatu, Krt::tfc, expected);
    }

    use Krt::*;

    // adanta are sew
    assert_has_tha(&[], &d("ha\\na~", Adadi), Lun, &["avaDizwa"]);
    // Fdanta are sew
    assert_has_krdanta(&[], &d("tF", Bhvadi), Krt::tfc, &["taritf", "tarItf"]);
    // fdanta are aniw, except for vfN/vfY
    assert_has_krdanta(
        &["nis"],
        &d("vfN", Kryadi),
        tfc,
        &["nirvaritf", "nirvarItf"],
    );
    assert_has_tfc(&["pra"], &d("vfY", Svadi), &["pravaritf", "pravarItf"]);
    // idanta are aniw, except for Svi and Sri
    // Idanta are aniw, except for SIN and DIN
    assert_has_tfc(&[], &d("wuo~Svi", Bhvadi), &["Svayitf"]);
    assert_has_tfc(&["ud"], &d("qIN", Divadi), &["uqqayitf"]);
    assert_has_tfc(&[], &d("SIN", Adadi), &["Sayitf"]);
    assert_has_tfc(&[], &d("SriY", Bhvadi), &["Srayitf"]);
    // Udanta (sew)
    assert_has_tfc(&[], &d("lUY", Kryadi), &["lavitf"]);
    assert_has_tfc(&[], &d("pUY", Svadi), &["pavitf"]);
    // ekAkzara udanta are aniw, except for ru, snu, kzu, yu, nu, kRu, UrRu
    assert_has_tfc(&[], &d("ru", Adadi), &["ravitf"]);
    assert_has_tfc(&["pra"], &d("zRU", Adadi), &["prasnavitf"]);
    assert_has_tfc(&[], &d("wukzu", Adadi), &["kzavitf"]);
    assert_has_tfc(&["pra"], &d("UrRuY", Adadi), &["prorRavitf", "prorRuvitf"]);
    assert_has_tfc(&[], &d("yu", Adadi), &["yavitf"]);
    assert_has_tfc(&[], &d("Ru", Adadi), &["navitf"]);
    assert_has_tfc(&[], &d("kzRu", Adadi), &["kzRavitf"]);
    // TODO: why anit? Should this be Ga\\sx~?
    // assert_has_tfc(&[], &d("Gasx~", Bhvadi), &["Gastf"]);
    assert_has_tfc(&[], &d("va\\sa~", Bhvadi), &["vastf"]);
    assert_has_tfc(&["AN"], &d("ra\\Ba~\\", Bhvadi), &["ArabDf"]);
    assert_has_tfc(&[], &d("ya\\Ba~", Svadi), &["yabDf"]);
    assert_has_tfc(&[], &d("qula\\Ba~\\z", Bhvadi), &["labDf"]);
    assert_has_tfc(&[], &d("ya\\ma~", Bhvadi), &["yantf"]);
    assert_has_tfc(&[], &d("ra\\ma~\\", Bhvadi), &["rantf"]);
    assert_has_tfc(&[], &d("ma\\na~\\", Divadi), &["mantf"]);
    assert_has_tfc(&[], &d("manu~\\", Tanadi), &["manitf"]);
    assert_has_tfc(&[], &d("Ra\\ma~\\", Bhvadi), &["nantf"]);
    assert_has_tfc(&[], &d("ha\\na~\\", Adadi), &["hantf"]);
    assert_has_tfc(&[], &d("ga\\mx~", Bhvadi), &["gantf"]);
    assert_has_tfc(&[], &d("di\\ha~^", Adadi), &["degDf"]);
    assert_has_tfc(&[], &d("du\\ha~^", Adadi), &["dogDf"]);
    assert_has_tfc(&[], &d("mi\\ha~", Bhvadi), &["meQf"]);
    assert_has_tfc(&[], &d("ru\\ha~", Bhvadi), &["roQf"]);
    assert_has_tfc(&[], &d("va\\ha~^", Bhvadi), &["voQf"]);
    assert_has_tfc(&[], &d("Ra\\ha~^", Divadi), &["nadDf"]);
    assert_has_tfc(&[], &d("da\\ha~", Bhvadi), &["dagDf"]);
    assert_has_tfc(&[], &d("li\\ha~^", Adadi), &["leQf"]);
    assert_has_tfc(&[], &d("di\\Sa~^", Tudadi), &["dezwf"]);
    assert_has_tfc(&[], &d("df\\Si~r", Bhvadi), &["drazwf"]);
    assert_has_tfc(&[], &d("da\\nSa~", Bhvadi), &["daMzwf"]);
    assert_has_tfc(&["AN"], &d("mf\\Sa~", Tudadi), &["Amrazwf", "Amarzwf"]);
    assert_has_tfc(&[], &d("spf\\Sa~", Tudadi), &["sprazwf", "sparzwf"]);
    assert_has_tfc(&[], &d("ri\\Sa~", Tudadi), &["rezwf"]);
    assert_has_tfc(&[], &d("ru\\Sa~", Tudadi), &["rozwf"]);
    assert_has_tfc(&[], &d("kru\\Sa~", Bhvadi), &["krozwf"]);
    assert_has_tfc(&["pra"], &d("vi\\zx~^", Juhotyadi), &["pravezwf"]);
    assert_has_tfc(&[], &d("li\\Sa~\\", Divadi), &["lezwf"]);
    assert_has_tfc(&[], &d("ru\\Di~^r", Rudhadi), &["rodDf"]);
    assert_has_tfc(&[], &d("rA\\Da~", Divadi), &["rAdDf"]);
    assert_has_tfc(&[], &d("yu\\Da~\\", Divadi), &["yodDf"]);
    assert_has_tfc(&[], &d("ba\\nDa~", Kryadi), &["banDf", "bandDf"]);
    assert_has_tfc(&[], &d("sA\\Da~", Svadi), &["sAdDf"]);
    assert_has_tfc(&[], &d("kru\\Da~", Divadi), &["krodDf"]);
    assert_has_tfc(&[], &d("kzu\\Da~", Divadi), &["kzodDf"]);
    assert_has_tfc(&[], &d("Su\\Da~", Divadi), &["SodDf"]);
    assert_has_tfc(&[], &d("bu\\Da~\\", Divadi), &["bodDf"]);
    assert_has_tfc(&[], &d("vya\\Da~", Divadi), &["vyadDf"]);
    assert_has_tfc(&[], &d("zi\\Du~", Divadi), &["sedDf"]);
    // budh, etc. are sew
    assert_has_tfc(&[], &d("buDa~", Bhvadi), &["boDitf"]);
    assert_has_tfc(&[], &d("ziDa~", Bhvadi), &["seDitf"]);
    assert_has_krdanta(&[], &d("buDa~", Bhvadi), Krt::kta, &["buDita"]);
    assert_has_krdanta(&[], &d("ziDa~", Bhvadi), Krt::kta, &["siDita"]);
    // zAnta (aniw) + kfz
    assert_has_tfc(&[], &d("Si\\zx~", Rudhadi), &["Sezwf"]);
    assert_has_tfc(&[], &d("pi\\zx~", Rudhadi), &["pezwf"]);
    assert_has_tfc(&[], &d("Su\\za~", Divadi), &["Sozwf"]);
    assert_has_tfc(&[], &d("pu\\za~", Divadi), &["pozwf"]);
    assert_has_tfc(&[], &d("tvi\\za~^", Bhvadi), &["tvezwf"]);
    assert_has_tfc(&[], &d("vi\\zx~^", Juhotyadi), &["vezwf"]);
    assert_has_tfc(&[], &d("Sli\\za~", Divadi), &["Slezwf"]);
    assert_has_tfc(&[], &d("tu\\za~", Divadi), &["tozwf"]);
    assert_has_tfc(&[], &d("du\\za~", Divadi), &["dozwf"]);
    assert_has_tfc(&[], &d("dvi\\za~^", Adadi), &["dvezwf"]);
    assert_has_tfc(&[], &d("kf\\za~^", Tudadi), &["krazwf", "karzwf"]);
    // pAnta (aniw)
    assert_has_tfc(&[], &d("ta\\pa~", Bhvadi), &["taptf"]);
    assert_has_tfc(&[], &d("ti\\pf~\\", Bhvadi), &["teptf"]);
    assert_has_tfc(&[], &d("A\\px~", Svadi), &["Aptf"]);
    assert_has_tfc(&[], &d("quva\\pa~^", Bhvadi), &["vaptf"]);
    assert_has_tfc(&[], &d("Yizva\\pa~", Adadi), &["svaptf"]);
    assert_has_tfc(&[], &d("li\\pa~^", Tudadi), &["leptf"]);
    assert_has_tfc(&[], &d("lu\\px~^", Tudadi), &["loptf"]);
    // pAnta (vew)
    assert_has_tfc(&[], &d("tf\\pa~", Divadi), &["traptf", "tarptf", "tarpitf"]);
    assert_has_tfc(&[], &d("df\\pa~", Divadi), &["draptf", "darptf", "darpitf"]);
    assert_has_tfc(&[], &d("sf\\px~", Bhvadi), &["sraptf", "sarptf"]);
    assert_has_tfc(&[], &d("Sa\\pa~^", Divadi), &["Saptf"]);
    assert_has_tfc(&[], &d("Cu\\pa~", Tudadi), &["Coptf"]);
    assert_has_tfc(&[], &d("kzi\\pa~", Divadi), &["kzeptf"]);
    // dAnta (aniw)
    assert_has_tfc(&[], &d("a\\da~", Adadi), &["attf"]);
    assert_has_tfc(&[], &d("ha\\da~\\", Bhvadi), &["hattf"]);
    assert_has_tfc(&[], &d("ska\\ndi~r", Bhvadi), &["skanttf", "skantf"]);
    assert_has_tfc(&[], &d("Bi\\di~^r", Rudhadi), &["Bettf"]);
    assert_has_tfc(&[], &d("Ci\\di~^r", Rudhadi), &["Cettf"]);
    assert_has_tfc(&[], &d("kzu\\di~^r", Rudhadi), &["kzottf"]);
    assert_has_tfc(&[], &d("Sa\\dx~", Bhvadi), &["Sattf"]);
    assert_has_tfc(&[], &d("za\\dx~", Bhvadi), &["sattf"]);
    assert_has_tfc(&[], &d("zvi\\dA~", Divadi), &["svettf"]);
    // But, YizvidA is sew.
    assert_has_tfc(&[], &d("YizvidA~", Bhvadi), &["sveditf"]);
    assert_has_tfc(&[], &d("pa\\da~\\", Divadi), &["pattf"]);
    assert_has_tfc(&[], &d("Ki\\da~\\", Divadi), &["Kettf"]);
    assert_has_tfc(&[], &d("tu\\da~^", Tudadi), &["tottf"]);
    assert_has_tfc(&[], &d("Ru\\da~^", Tudadi), &["nottf"]);
    assert_has_tfc(&[], &d("vi\\da~\\", Rudhadi), &["vettf"]);
    // cAnta, CAnta, jAnta (aniw)
    assert_has_tfc(&[], &d("qupa\\ca~^z", Bhvadi), &["paktf"]);
    assert_has_tfc(&[], &d("va\\ca~", Adadi), &["vaktf"]);
    assert_has_tfc(&[], &d("ri\\ci~^r", Rudhadi), &["rektf"]);
    assert_has_tfc(&[], &d("ra\\nja~^", Bhvadi), &["raNktf"]);
    assert_has_tfc(&[], &d("pra\\Ca~", Tudadi), &["prazwf"]);

    assert_has_tfc(&["nis"], &d("Ri\\ji~^r", Juhotyadi), &["nirRektf"]);
    assert_has_tfc(&[], &d("zi\\ca~^", Tudadi), &["sektf"]);
    assert_has_tfc(&[], &d("mu\\cx~^", Tudadi), &["moktf"]);
    assert_has_tfc(&[], &d("Ba\\ja~^", Bhvadi), &["Baktf"]);
    assert_has_tfc(&[], &d("Ba\\njo~", Rudhadi), &["BaNktf"]);
    assert_has_tfc(&[], &d("Bra\\sja~^", Tudadi), &["Brazwf", "Barzwf"]);
    assert_has_tfc(&[], &d("tya\\ja~", Bhvadi), &["tyaktf"]);
    assert_has_tfc(&[], &d("ya\\ja~^", Bhvadi), &["yazwf"]);
    assert_has_tfc(&[], &d("yu\\ji~^r", Rudhadi), &["yoktf"]);
    assert_has_tfc(&[], &d("ru\\jo~", Tudadi), &["roktf"]);
    assert_has_tfc(&[], &d("za\\nja~", Bhvadi), &["saNktf"]);
    assert_has_tfc(&[], &d("wuma\\sjo~", Tudadi), &["maNktf"]);
    assert_has_tfc(&[], &d("Bu\\jo~", Tudadi), &["Boktf"]);
    // TODO: why parizvaktA and not parizvaNktA?
    // assert_has_tfc(&["pari"], &d("zva\\nja~\\", Bhvadi), &["parizvaktf"]);
    assert_has_tfc(&[], &d("sf\\ja~", Tudadi), &["srazwf"]);
    // vew
    assert_has_tfc(&[], &d("mfjU~", Adadi), &["mArzwf", "mArjitf"]);
}

#[test]
fn sutra_7_2_11() {
    let shri = d("SriY", Bhvadi);
    assert_has_krdanta(&[], &shri, Krt::ktvA, &["SritvA"]);
    assert_has_krdanta(&[], &shri, Krt::kta, &["Srita"]);
    assert_has_krdanta(&[], &shri, Krt::ktavatu, &["Sritavat"]);

    let yu = d("yu", Adadi);
    assert_has_krdanta(&[], &yu, Krt::ktvA, &["yutvA"]);
    assert_has_krdanta(&[], &yu, Krt::kta, &["yuta"]);
    assert_has_krdanta(&[], &yu, Krt::ktavatu, &["yutavat"]);

    let lu = d("lUY", Kryadi);
    assert_has_krdanta(&[], &lu, Krt::ktvA, &["lUtvA"]);
    assert_has_krdanta(&[], &lu, Krt::kta, &["lUna"]);
    assert_has_krdanta(&[], &lu, Krt::ktavatu, &["lUnavat"]);

    let vf = d("vfN", Kryadi);
    assert_has_krdanta(&[], &vf, Krt::ktvA, &["vftvA"]);
    assert_has_krdanta(&[], &vf, Krt::kta, &["vfta"]);
    assert_has_krdanta(&[], &vf, Krt::ktavatu, &["vftavat"]);

    let tf = d("tF", Bhvadi);
    assert_has_krdanta(&[], &tf, Krt::ktvA, &["tIrtvA"]);
    assert_has_krdanta(&[], &tf, Krt::kta, &["tIrRa"]);
    assert_has_krdanta(&[], &tf, Krt::ktavatu, &["tIrRavat"]);

    let vid = d("vida~", Adadi);
    assert_has_krdanta(&[], &vid, Krt::kta, &["vidita"]);

    assert_has_krdanta(&[], &shri, Krt::tfc, &["Srayitf"]);
    assert_has_krdanta(&[], &shri, Krt::tumun, &["Srayitum"]);
    assert_has_krdanta(&[], &shri, Krt::tavya, &["Srayitavya"]);
}

#[test]
fn sutra_7_2_12() {
    assert_has_tip(&[], &san(&d("graha~^", Kryadi)), Lat, &["jiGfkzati"]);
    assert_has_tip(&[], &san(&d("guhU~^", Bhvadi)), Lat, &["juGukzati"]);
    assert_has_tip(&[], &san(&d("ru", Adadi)), Lat, &["rurUzati"]);
    assert_has_tip(&[], &san(&d("lUY", Kryadi)), Lat, &["lulUzati"]);
}

#[test]
fn sutra_7_2_13() {
    assert_has_vas(&[], &d("qukf\\Y", Tanadi), Lit, &["cakfva"]);
    assert_has_vas(&[], &d("sf\\", Bhvadi), Lit, &["sasfva"]);
    assert_has_vas(&[], &d("Bf\\Y", Bhvadi), Lit, &["baBfva"]);
    assert_has_vas(
        &[],
        &d("quBf\\Y", Juhotyadi),
        Lit,
        &["baBfva", "biBarAYcakfva", "biBarAmAsiva", "biBarAmbaBUviva"],
    );
    assert_has_vas(&[], &d("vfY", Svadi), Lit, &["vavfva"]);
    assert_has_vahi(&[], &d("vfN", Kryadi), Lit, &["vavfvahe"]);
    assert_has_vas(&[], &d("zwu\\Y", Adadi), Lit, &["tuzwuva"]);
    assert_has_vas(&[], &d("dru\\", Bhvadi), Lit, &["dudruva"]);
    assert_has_vas(&[], &d("sru\\", Bhvadi), Lit, &["susruva"]);
    assert_has_vas(&[], &d("Sru\\", Bhvadi), Lit, &["SuSruva"]);
}

#[test]
fn sutra_7_2_13_v1() {
    let kf = &d("qukf\\Y", Tanadi);
    assert_has_vas(&["sam"], kf, Lit, &["saYcaskariva", "saYcakfva"]);
    assert_has_mas(&["sam"], kf, Lit, &["saYcaskarima", "saYcakfma"]);
    assert_has_sip(&["sam"], kf, Lit, &["saYcaskariTa", "saYcakarTa"]);
}

#[test]
fn sutra_7_2_14() {
    let shvi = d("wuo~Svi", Bhvadi);
    assert_has_krdanta(&[], &shvi, Krt::kta, &["SUna"]);
    assert_has_krdanta(&[], &shvi, Krt::ktavatu, &["SUnavat"]);

    let lasj = d("o~lasjI~\\", Tudadi);
    assert_has_krdanta(&[], &lasj, Krt::kta, &["lagna"]);
    assert_has_krdanta(&[], &lasj, Krt::ktavatu, &["lagnavat"]);

    let vij = d("o~vijI~", Rudhadi);
    assert_has_krdanta(&["ud"], &vij, Krt::kta, &["udvigna"]);
    assert_has_krdanta(&["ud"], &vij, Krt::ktavatu, &["udvignavat"]);

    let dip = d("dIpI~\\", Rudhadi);
    assert_has_krdanta(&[], &dip, Krt::kta, &["dIpta"]);
    assert_has_krdanta(&[], &dip, Krt::ktavatu, &["dIptavat"]);

    let vij = d("o~vijI~", Rudhadi);
    assert_has_krdanta(&["ud"], &vij, Krt::kta, &["udvigna"]);
    assert_has_krdanta(&["ud"], &vij, Krt::ktavatu, &["udvignavat"]);

    // todo: uqqIna
}

#[test]
fn sutra_7_2_15() {
    let dhu = d("DUY", Svadi);
    assert_has_krdanta(&["vi"], &dhu, Krt::kta, &["viDUta"]);
    assert_has_krdanta(&["vi"], &dhu, Krt::ktavatu, &["viDUtavat"]);

    let guh = d("guhU~^", Bhvadi);
    assert_has_krdanta(&[], &guh, Krt::kta, &["gUQa"]);
    assert_has_krdanta(&[], &guh, Krt::ktavatu, &["gUQavat"]);

    let vfdh = d("vfDu~\\", Bhvadi);
    assert_has_krdanta(&[], &vfdh, Krt::kta, &["vfdDa"]);
    assert_has_krdanta(&[], &vfdh, Krt::ktavatu, &["vfdDavat"]);
}

#[test]
fn sutra_7_2_16() {
    let mid = d("YimidA~", Divadi);
    assert_has_krdanta(&[], &mid, Krt::kta, &["minna"]);
    assert_has_krdanta(&[], &mid, Krt::ktavatu, &["minnavat"]);

    let kzvid = d("YikzvidA~", Divadi);
    assert_has_krdanta(&[], &kzvid, Krt::kta, &["kzviRRa"]);
    assert_has_krdanta(&[], &kzvid, Krt::ktavatu, &["kzviRRavat"]);

    let svid = d("YizvidA~", Bhvadi);
    assert_has_krdanta(&[], &svid, Krt::kta, &["svinna"]);
    assert_has_krdanta(&[], &svid, Krt::ktavatu, &["svinnavat"]);
}

#[test]
fn sutra_7_2_23() {
    assert_has_krdanta(&[], &d("Guzi~r", Bhvadi), Krt::kta, &["Guzwa", "Guzita"]);
    assert_has_krdanta(
        &[],
        &d("Guzi~r", Curadi),
        Krt::kta,
        &["Gozita", "Guzwa", "Guzita"],
    );
}

#[test]
fn sutra_7_2_24() {
    let ard = d("arda~", Bhvadi);
    assert_has_krdanta(&["sam"], &ard, Krt::kta, &["samarRa", "samarRRa"]);
    assert_has_krdanta(&["ni"], &ard, Krt::kta, &["nyarRa", "nyarRRa"]);
    assert_has_krdanta(&["vi"], &ard, Krt::kta, &["vyarRa", "vyarRRa"]);

    let edh = d("eDa~\\", Bhvadi);
    assert_has_krdanta(&["sam"], &edh, Krt::kta, &["sameDita"]);

    assert_has_krdanta(&[], &ard, Krt::kta, &["ardita"]);
}

#[test]
fn sutra_7_2_25() {
    let ard = d("arda~", Bhvadi);
    assert_has_krdanta(
        &["aBi"],
        &ard,
        Krt::kta,
        &["aByarRa", "aByarRRa", "aByardita"],
    );
}

#[test]
fn sutra_7_2_28() {
    let assert_has_kta = |w, x, y| assert_has_krdanta(w, &x, Krt::kta, y);
    assert_has_kta(&[], d("ruza~", Bhvadi), &["ruzwa", "ruzita"]);
    assert_has_kta(&["aBi"], d("ama~", Bhvadi), &["aByAnta", "aByamita"]);
    assert_has_kta(&[], d("YitvarA~\\", Bhvadi), &["tUrRa", "tvarita"]);
    assert_has_kta(&["sam"], d("Guzi~r", Bhvadi), &["saNGuzwa", "saNGuzita"]);
    assert_has_kta(&["AN"], d("svana~", Bhvadi), &["AsvAnta", "Asvanita"]);
}

#[test]
fn sutra_7_2_35() {
    let lu = d("lUY", Kryadi);
    assert_has_krdanta(&[], &lu, Krt::tfc, &["lavitf"]);
    assert_has_krdanta(&[], &lu, Krt::tumun, &["lavitum"]);
    assert_has_krdanta(&[], &lu, Krt::tavya, &["lavitavya"]);

    let pu = d("pUY", Kryadi);
    assert_has_krdanta(&[], &pu, Krt::tfc, &["pavitf"]);
    assert_has_krdanta(&[], &pu, Krt::tumun, &["pavitum"]);
    assert_has_krdanta(&[], &pu, Krt::tavya, &["pavitavya"]);

    // TODO: Aste, vaste

    assert_has_krdanta(&[], &lu, Krt::yat, &["lavya"]);
    assert_has_krdanta(&[], &pu, Krt::yat, &["pavya"]);
    assert_has_krdanta(&[], &lu, Krt::anIyar, &["lavanIya"]);
    assert_has_krdanta(&[], &pu, Krt::anIyar, &["pavanIya"]);
}

#[test]
fn sutra_7_2_36() {
    let snu = d("zRu", Adadi);
    assert_has_krdanta(&["pra"], &snu, Krt::tfc, &["prasnavitf"]);
    assert_has_krdanta(&["pra"], &snu, Krt::tumun, &["prasnavitum"]);
    assert_has_krdanta(&["pra"], &snu, Krt::tavya, &["prasnavitavya"]);

    let kram = d("kramu~", Bhvadi);
    assert_has_krdanta(&["pra"], &kram, Krt::tfc, &["prakramitf"]);
    assert_has_krdanta(&["pra"], &kram, Krt::tumun, &["prakramitum"]);
    assert_has_krdanta(&["pra"], &kram, Krt::tavya, &["prakramitavya"]);

    // anAtmanepadanimitte
    assert_has_ta_k(&["pra"], &snu, AshirLin, &["prasnozIzwa", "prasnAvizIzwa"]);
    assert_has_ta_k(&["pra"], &kram, AshirLin, &["prakraMsIzwa"]);
    assert_has_ta_k(&["pra"], &snu, Lrt, &["prasnozyate", "prasnAvizyate"]);
    assert_has_ta_k(&["pra"], &kram, Lrt, &["prakraMsyate"]);
    assert_has_ta_k(&["pra"], &san(&snu), Lrt, &["prasusnUzizyate"]);
    // TODO: we produce pracikramizizyate instead
    // assert_has_lrt_karmani(&["pra"], &san(&kram), &["pracikraMsizyate"]);
}

#[test]
fn sutra_7_2_37() {
    let grah = d("graha~^", Kryadi);
    assert_has_krdanta(&[], &grah, Krt::tfc, &["grahItf"]);
    assert_has_krdanta(&[], &grah, Krt::tumun, &["grahItum"]);
    assert_has_krdanta(&[], &grah, Krt::tavya, &["grahItavya"]);
    assert_has_vas(&[], &grah, Lit, &["jagfhiva"]);
    assert_has_ta_k(&[], &grah, Lut, &["grAhitA", "grahItA"]);
    assert_has_ta_k(&[], &grah, Lrt, &["grAhizyate", "grahIzyate"]);
}

#[test]
fn sutra_7_2_38() {
    let vf1 = d("vfN", Kryadi);
    let vf2 = d("vfY", Kryadi);
    assert_has_krdanta(&[], &vf1, Krt::tfc, &["varitf", "varItf"]);
    assert_has_krdanta(&["pra", "AN"], &vf2, Krt::tfc, &["prAvaritf", "prAvarItf"]);
    // Ft
    let tf = &d("tF", Bhvadi);
    assert_has_krdanta(&[], &tf, Krt::tfc, &["taritf", "tarItf"]);
    assert_has_krdanta(
        &["AN"],
        &d("stFY", Kryadi),
        Krt::tfc,
        &["Astaritf", "AstarItf"],
    );
    // vFtaH?
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lrt, &["karizyati"]);
    assert_has_tip(&[], &d("hf\\Y", Bhvadi), Lrt, &["harizyati"]);
    // aliwi
    assert_has_sip(&[], &d("vF", Kryadi), Lit, &["vavariTa"]);
    assert_has_sip(&[], &tf, Lit, &["teriTa"]);
}

#[test]
fn sutra_7_2_39() {
    let vf1 = d("vfN", Kryadi);
    let vf2 = d("vfY", Kryadi);
    let stf = d("stFY", Kryadi);

    // For aniw, see 7.2.42.
    assert_has_ta(&["vi"], &vf1, AshirLin, &["vivarizIzwa", "vivfzIzwa"]);
    assert_has_ta(
        &["pra", "AN"],
        &vf2,
        AshirLin,
        &["prAvarizIzwa", "prAvfzIzwa"],
    );
    assert_has_ta(&["AN"], &stf, AshirLin, &["AstarizIzwa", "AstIrzIzwa"]);
    assert_has_ta(&["vi"], &stf, AshirLin, &["vistarizIzwa", "vistIrzIzwa"]);
}

#[test]
fn sutra_7_2_40() {
    let vf = d("vfY", Kryadi);
    let tf = d("tF", Bhvadi);
    let stf = d("stFY", Kryadi);

    // for aniw, see 7.2.42.
    assert_has_tas(&["pra", "AN"], &vf, Lun, &["prAvArizwAm"]);
    assert_has_jhi(&["pra", "AN"], &vf, Lun, &["prAvArizuH"]);
    assert_has_tas(&[], &tf, Lun, &["atArizwAm"]);
    assert_has_jhi(&[], &tf, Lun, &["atArizuH"]);
    assert_has_tas(&[], &stf, Lun, &["astArizwAm"]);
    assert_has_jhi(&[], &stf, Lun, &["astArizuH"]);

    // parasmEpadezu?
    // For aniw, see 7.2.42.
    assert_has_ta(
        &["pra", "AN"],
        &vf,
        Lun,
        &["prAvarizwa", "prAvarIzwa", "prAvfta"],
    );
}

#[test]
fn sutra_7_2_41() {
    assert_has_san_ta(
        &[],
        &d("vfN", Kryadi),
        &["vuvUrzate", "vivarizate", "vivarIzate"],
    );
    assert_has_san_tip(
        &["pra", "AN"],
        &d("vfY", Svadi),
        &["prAvuvUrzati", "prAvivarizati", "prAvivarIzati"],
    );
    assert_has_san_tip(
        &[],
        &d("tF", Bhvadi),
        &["titIrzati", "titarizati", "titarIzati"],
    );
    assert_has_san_ta(
        &["AN"],
        &d("stFY", Kryadi),
        &["AtistIrzate", "Atistarizate", "AtistarIzate"],
    );

    assert_has_san_tip(&[], &d("qukf\\Y", Tanadi), &["cikIrzati"]);
    assert_has_san_tip(&[], &d("hf\\Y", Bhvadi), &["jihIrzati"]);
}

#[test]
fn sutra_7_2_42() {
    let vf1 = d("vfN", Kryadi);
    let vf2 = d("vfY", Kryadi);
    let stf = d("stFY", Kryadi);

    assert_has_ta(&[], &vf1, AshirLin, &["vfzIzwa", "varizIzwa"]);
    assert_has_ta(
        &["pra", "AN"],
        &vf2,
        AshirLin,
        &["prAvfzIzwa", "prAvarizIzwa"],
    );
    assert_has_ta(&["AN"], &stf, AshirLin, &["AstarizIzwa", "AstIrzIzwa"]);

    // sici?
    assert_has_ta(&[], &vf1, Lun, &["avfta", "avarizwa", "avarIzwa"]);
    assert_has_ta(
        &["pra", "AN"],
        &vf2,
        Lun,
        &["prAvfta", "prAvarizwa", "prAvarIzwa"],
    );
    assert_has_ta(&["AN"], &stf, Lun, &["AstIrzwa", "Astarizwa", "AstarIzwa"]);

    // Atmanepadezu?
    assert_has_tas(&["pra", "AN"], &vf2, Lun, &["prAvArizwAm"]);
    assert_has_jhi(&["pra", "AN"], &vf2, Lun, &["prAvArizuH"]);
}

#[test]
fn sutra_7_2_43() {
    let dhvf = d("Dvf\\", Bhvadi);
    let smf = d("smf\\", Bhvadi);

    assert_has_ta_k(
        &[],
        &dhvf,
        AshirLin,
        &["DvfzIzwa", "DvarizIzwa", "DvArizIzwa"],
    );
    assert_has_ta_k(
        &[],
        &smf,
        AshirLin,
        &["smfzIzwa", "smarizIzwa", "smArizIzwa"],
    );
    assert_has_aataam_k(
        &[],
        &dhvf,
        Lun,
        &["aDvfzAtAm", "aDvarizAtAm", "aDvArizAtAm"],
    );
    assert_has_aataam_k(&[], &smf, Lun, &["asmfzAtAm", "asmarizAtAm", "asmArizAtAm"]);

    // fta?
    let cyu = d("cyu\\N", Bhvadi);
    let plu = d("plu\\N", Bhvadi);
    assert_has_ta(&[], &cyu, AshirLin, &["cyozIzwa"]);
    assert_has_ta(&[], &plu, AshirLin, &["plozIzwa"]);
    assert_has_ta(&[], &cyu, Lun, &["acyozwa"]);
    assert_has_ta(&[], &plu, Lun, &["aplozwa"]);

    // saMyogAdeH?
    let kr = d("qukf\\Y", Tanadi);
    let hr = d("hf\\Y", Bhvadi);
    assert_has_ta(&[], &kr, AshirLin, &["kfzIzwa"]);
    assert_has_ta(&[], &hr, AshirLin, &["hfzIzwa"]);
    assert_has_ta(&[], &kr, Lun, &["akfta"]);
    assert_has_ta(&[], &hr, Lun, &["ahfta"]);

    // Atmanepadezu?
    assert_has_tip(&[], &dhvf, Lun, &["aDvArzIt"]);
    assert_has_tip(&[], &smf, Lun, &["asmArzIt"]);

    // suw-Agama
    assert_has_ta(&["sam"], &kr, AshirLin, &["saNkfzIzwa", "saMskfzIzwa"]);
    assert_has_ta(&["sam"], &kr, Lun, &["samakfta", "samaskfta"]);
}

#[test]
fn sutra_7_2_44() {
    let svf = d("svf", Bhvadi);
    assert_has_krdanta(&[], &svf, Krt::tfc, &["svartf", "svaritf"]);

    let su = d("zUN", Adadi);
    assert_has_krdanta(&["pra"], &su, Krt::tfc, &["prasotf", "prasavitf"]);

    let su2 = d("zUN", Divadi);
    assert_has_krdanta(&[], &su2, Krt::tfc, &["sotf", "savitf"]);

    let dhu = d("DUY", Svadi);
    assert_has_krdanta(&[], &dhu, Krt::tfc, &["Dotf", "Davitf"]);

    let gah = d("gAhU~\\", Bhvadi);
    assert_has_krdanta(&["vi"], &gah, Krt::tfc, &["vigAQf", "vigAhitf"]);

    let gup = d("gupU~", Bhvadi);
    assert_has_krdanta(&[], &gup, Krt::tfc, &["goptf", "gopitf", "gopAyitf"]);

    // Exclude su and dhu in tudAdi
    assert_has_krdanta(&[], &d("zU", Tudadi), Krt::tfc, &["savitf"]);
    // TODO: Davitf or Duvitf?
    assert_has_krdanta(&[], &d("DU", Tudadi), Krt::tfc, &["Davitf"]);
}

#[test]
fn sutra_7_2_45() {
    assert_has_krdanta(&[], &d("ra\\Da~", Divadi), Krt::tfc, &["radDf", "raDitf"]);
    assert_has_krdanta(&[], &d("Ra\\Sa~", Divadi), Krt::tfc, &["naMzwf", "naSitf"]);
    assert_has_krdanta(
        &[],
        &d("tf\\pa~", Divadi),
        Krt::tfc,
        &["traptf", "tarptf", "tarpitf"],
    );
    assert_has_krdanta(
        &[],
        &d("df\\pa~", Divadi),
        Krt::tfc,
        &["draptf", "darptf", "darpitf"],
    );
    assert_has_krdanta(
        &[],
        &d("dru\\ha~", Divadi),
        Krt::tfc,
        &["drogDf", "droQf", "drohitf"],
    );
    assert_has_krdanta(
        &[],
        &d("mu\\ha~", Divadi),
        Krt::tfc,
        &["mogDf", "moQf", "mohitf"],
    );
    assert_has_krdanta(
        &[],
        &d("zRu\\ha~", Divadi),
        Krt::tfc,
        &["snogDf", "snoQf", "snohitf"],
    );
    assert_has_krdanta(
        &[],
        &d("zRi\\ha~", Divadi),
        Krt::tfc,
        &["snegDf", "sneQf", "snehitf"],
    );
}

#[test]
fn sutra_7_2_46() {
    let kuz = d("kuza~", Kryadi);
    assert_has_krdanta(&["nir"], &kuz, Krt::tfc, &["nizkozwf", "nizkozitf"]);
    assert_has_krdanta(&["nir"], &kuz, Krt::tumun, &["nizkozwum", "nizkozitum"]);
    assert_has_krdanta(&["nir"], &kuz, Krt::tavya, &["nizkozwavya", "nizkozitavya"]);
    // niraH
    assert_has_krdanta(&[], &kuz, Krt::tfc, &["kozitf"]);
    assert_has_krdanta(&[], &kuz, Krt::tumun, &["kozitum"]);
    assert_has_krdanta(&[], &kuz, Krt::tavya, &["kozitavya"]);
}

#[test]
fn sutra_7_2_47() {
    let kuz = d("kuza~", Kryadi);
    assert_has_krdanta(&["nir"], &kuz, Krt::kta, &["nizkuzita"]);
    assert_has_krdanta(&["nir"], &kuz, Krt::ktavatu, &["nizkuzitavat"]);
}

#[test]
fn sutra_7_2_48() {
    let iz_divadi = d("iza~", Divadi);
    let iz_tudadi = d("izu~", Tudadi);
    let iz_kryadi = d("iza~", Kryadi);
    assert_has_krdanta(&[], &iz_tudadi, Krt::tfc, &["ezwf", "ezitf"]);
    // Exclude other "iz" entries:
    assert_has_krdanta(&["pra"], &iz_divadi, Krt::tfc, &["prezitf"]);
    assert_has_krdanta(&["pra"], &iz_divadi, Krt::tumun, &["prezitum"]);
    assert_has_krdanta(&["pra"], &iz_divadi, Krt::tavya, &["prezitavya"]);
    assert_has_krdanta(&["pra"], &iz_kryadi, Krt::tfc, &["prezitf"]);
    assert_has_krdanta(&["pra"], &iz_kryadi, Krt::tumun, &["prezitum"]);
    assert_has_krdanta(&["pra"], &iz_kryadi, Krt::tavya, &["prezitavya"]);
}

#[test]
fn sutra_7_2_49() {
    assert_has_san_tip(&[], &d("divu~", Divadi), &["didevizati", "dudyUzati"]);
    assert_has_san_tip(&[], &d("zivu~", Divadi), &["sisevizati", "susyUzati"]);
    assert_has_san_tip(&[], &d("fDu~", Divadi), &["ardiDizati", "Irtsati"]);
    assert_has_san_tip(
        &[],
        &d("Bra\\sja~^", Tudadi),
        &["biBrajjizati", "biBrakzati", "biBarjizati", "biBarkzati"],
    );
    assert_has_san_tip(
        &[],
        &d("danBu~", Svadi),
        &["didamBizati", "Dipsati", "DIpsati"],
    );
    assert_has_san_tip(
        &["ud"],
        &d("SriY", Bhvadi),
        &["ucCiSrayizati", "ucCiSrIzati"],
    );
    assert_has_san_tip(&[], &d("svf", Bhvadi), &["sisvarizati", "susvUrzati"]);
    assert_has_san_tip(&[], &d("yu", Adadi), &["yiyavizati", "yuyUzati"]);
    assert_has_san_tip(
        &["pra"],
        &d("UrRuY", Adadi),
        &["prorRunavizati", "prorRunuvizati", "prorRunUzati"],
    );
    assert_has_san_tip(&[], &d("Bf\\Y", Bhvadi), &["biBarizati", "buBUrzati"]);
    assert_has_san_tip(&[], &d("jYapa~", Curadi), &["jijYapayizati", "jYIpsati"]);
    assert_has_san_tip(&[], &d("zaRa~", Bhvadi), &["sisanizati", "sizAsati"]);
}

#[test]
fn sutra_7_2_49_v1() {
    assert_has_tip(
        &[],
        &san(&d("tanu~^", Tanadi)),
        Lat,
        &["titanizati", "titaMsati", "titAMsati"],
    );
    assert_has_lat(&[], &san(&d("patx~", Bhvadi)), &["pipatizati", "pitsati"]);
    assert_has_lat(
        &[],
        &san(&d("daridrA", Adadi)),
        &["didaridrizati", "didaridrAsati"],
    );
}

#[test]
fn sutra_7_2_50() {
    let klish_div = d("kliSa~\\", Divadi);
    assert_has_krdanta(&[], &klish_div, Krt::ktvA, &["klizwvA", "kliSitvA"]);
    assert_has_krdanta(&[], &klish_div, Krt::kta, &["klizwa", "kliSita"]);
    assert_has_krdanta(&[], &klish_div, Krt::ktavatu, &["klizwavat", "kliSitavat"]);

    let klish_kri = d("kliSU~\\", Kryadi);
    assert_has_krdanta(&[], &klish_kri, Krt::ktvA, &["klizwvA", "kliSitvA"]);
    assert_has_krdanta(&[], &klish_kri, Krt::kta, &["klizwa", "kliSita"]);
    assert_has_krdanta(&[], &klish_kri, Krt::ktavatu, &["klizwavat", "kliSitavat"]);
}

#[test]
fn sutra_7_2_51() {
    let pu = d("pUN", Bhvadi);
    assert_has_krdanta(&[], &pu, Krt::ktvA, &["pUtvA", "pavitvA"]);
    assert_has_krdanta(&["ati"], &pu, Krt::kta, &["atipUta", "atipavita"]);
    assert_has_krdanta(&[], &pu, Krt::ktavatu, &["pUtavat", "pavitavat"]);
}

#[test]
fn sutra_7_2_52() {
    let vas = d("va\\sa~", Bhvadi);
    assert_has_krdanta(&[], &vas, Krt::ktvA, &["uzitvA"]);
    assert_has_krdanta(&[], &vas, Krt::kta, &["uzita"]);
    assert_has_krdanta(&[], &vas, Krt::ktavatu, &["uzitavat"]);

    let kzudh = d("kzu\\Da~", Divadi);
    assert_has_krdanta(&[], &kzudh, Krt::ktvA, &["kzuDitvA", "kzoDitvA"]);
    assert_has_krdanta(&[], &kzudh, Krt::kta, &["kzuDita"]);
    assert_has_krdanta(&[], &kzudh, Krt::ktavatu, &["kzuDitavat"]);
}

#[ignore]
#[test]
fn sutra_7_2_53() {
    let anc = d("ancu~", Bhvadi);
    assert_has_krdanta(&[], &anc, Krt::ktvA, &["aYcitvA", "aktvA"]);
    assert_has_krdanta(&[], &anc, Krt::kta, &["aYcita", "akta"]);
}

#[test]
fn sutra_7_2_55() {
    assert_has_krdanta(&[], &d("jF", Kryadi), Krt::ktvA, &["jaritvA", "jarItvA"]);
    assert_has_krdanta(&[], &d("jFz", Divadi), Krt::ktvA, &["jaritvA", "jarItvA"]);
    assert_has_krdanta(&[], &d("o~vrascU~", Tudadi), Krt::ktvA, &["vraScitvA"]);
}

#[test]
fn sutra_7_2_56() {
    assert_has_krdanta(&[], &d("Samu~", Divadi), Krt::ktvA, &["SamitvA", "SAntvA"]);
    assert_has_krdanta(&[], &d("tamu~", Divadi), Krt::ktvA, &["tamitvA", "tAntvA"]);
    assert_has_krdanta(&[], &d("damu~", Divadi), Krt::ktvA, &["damitvA", "dAntvA"]);
}

#[test]
fn sutra_7_2_57() {
    let kft = d("kftI~", Tudadi);
    assert_has_tip(&[], &kft, Lrt, &["kartsyati", "kartizyati"]);
    assert_has_tip(&[], &san(&kft), Lat, &["cikftsati", "cikartizati"]);
    assert_has_tip(&[], &kft, Lrn, &["akartsyat", "akartizyat"]);
    let kft = d("kftI~", Rudhadi);
    assert_has_tip(&[], &kft, Lrt, &["kartsyati", "kartizyati"]);
    assert_has_tip(&[], &san(&kft), Lat, &["cikftsati", "cikartizati"]);
    assert_has_tip(&[], &kft, Lrn, &["akartsyat", "akartizyat"]);

    let cft = d("cftI~", Tudadi);
    assert_has_tip(&[], &cft, Lrt, &["cartsyati", "cartizyati"]);
    assert_has_tip(&[], &san(&cft), Lat, &["cicftsati", "cicartizati"]);
    assert_has_tip(&[], &cft, Lrn, &["acartsyat", "acartizyat"]);

    let chfd = d("u~Cfdi~^r", Rudhadi);
    assert_has_tip(&[], &chfd, Lrt, &["Cartsyati", "Cardizyati"]);
    assert_has_tip(&[], &san(&chfd), Lat, &["cicCftsati", "cicCardizati"]);
    assert_has_tip(&[], &chfd, Lrn, &["acCartsyat", "acCardizyat"]);

    let tfd = d("u~tfdi~^r", Rudhadi);
    assert_has_tip(&[], &tfd, Lrt, &["tartsyati", "tardizyati"]);
    assert_has_tip(&[], &san(&tfd), Lat, &["titftsati", "titardizati"]);
    assert_has_tip(&[], &tfd, Lrn, &["atartsyat", "atardizyat"]);

    let nft = d("nftI~", Divadi);
    assert_has_tip(&[], &nft, Lrt, &["nartsyati", "nartizyati"]);
    assert_has_tip(&[], &san(&nft), Lat, &["ninftsati", "ninartizati"]);
    assert_has_tip(&[], &nft, Lrn, &["anartsyat", "anartizyat"]);
}

#[test]
fn sutra_7_2_58() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_tip(&[], &gam, Lrt, &["gamizyati"]);
    assert_has_tip(&[], &gam, Lrn, &["agamizyat"]);
    assert_has_tip(&[], &san(&gam), Lat, &["jigamizati"]);
    // gameH
    assert_has_tip(&[], &d("ci\\Y", Svadi), Lrt, &["cezyati"]);

    // parasmEpadezu?
    // saNgasIzwa is justified by KV 1.2.13, etc.
    assert_has_ta(&["sam"], &gam, AshirLin, &["saNgaMsIzwa", "saNgasIzwa"]);
    assert_has_ta(&["sam"], &gam, Lrt, &["saNgaMsyate"]);
    assert_has_ta(&["sam"], &san(&gam), Lat, &["saYjigaMsate"]);
    assert_has_ta(&["sam"], &san(&gam), Lrt, &["saYjigaMsizyate"]);
    assert_has_ta(&["aDi"], &san(&d("i\\N", Adadi)), Lat, &["aDijigAMsate"]);
}

#[test]
fn sutra_7_2_59() {
    let vft = d("vftu~\\", Bhvadi);
    assert_has_lrt(&[], &vft, &["vartsyati", "vartizyate"]);
    assert_has_lrn(&[], &vft, &["avartsyat", "avartizyata"]);
    assert_has_lat(&[], &san(&vft), &["vivftsati", "vivartizate"]);

    let vfdh = d("vfDu~\\", Bhvadi);
    assert_has_lrt(&[], &vfdh, &["vartsyati", "varDizyate"]);
    assert_has_lrn(&[], &vfdh, &["avartsyat", "avarDizyata"]);
    assert_has_lat(&[], &san(&vfdh), &["vivftsati", "vivarDizate"]);

    let shfdh = d("SfDu~\\", Bhvadi);
    assert_has_lrt(&[], &shfdh, &["Sartsyati", "SarDizyate"]);
    assert_has_lrn(&[], &shfdh, &["aSartsyat", "aSarDizyata"]);
    assert_has_lat(&[], &san(&shfdh), &["SiSftsati", "SiSarDizate"]);

    let syand = d("syandU~\\", Bhvadi);
    assert_has_lrt(&[], &syand, &["syantsyati", "syandizyate", "syantsyate"]);
    assert_has_lrn(&[], &syand, &["asyantsyat", "asyandizyata", "asyantsyata"]);
    assert_has_lat(
        &[],
        &san(&syand),
        &["sisyantsati", "sisyandizate", "sisyantsate"],
    );
}

#[test]
fn sutra_7_2_60() {
    let kfp = d("kfpU~\\", Bhvadi);
    assert_has_tip(&[], &kfp, Lut, &["kalptA"]);
    assert_has_tip(&[], &kfp, Lrt, &["kalpsyati"]);
    assert_has_tip(&[], &kfp, Lrn, &["akalpsyat"]);
    assert_has_tip(&[], &san(&kfp), Lat, &["cikxpsati"]);
    // parasmaipadezu
    assert_has_tinantas(
        &[],
        &kfp,
        Lut,
        Madhyama,
        Eka,
        &["kalptAsi", "kalptAse", "kalpitAse"],
    );
    assert_has_ta(&[], &kfp, Lrt, &["kalpizyate", "kalpsyate"]);
    assert_has_ta(&[], &kfp, AshirLin, &["kalpizIzwa", "kxpsIzwa"]);
    assert_has_ta(&[], &kfp, Lrn, &["akalpizyata", "akalpsyata"]);
    assert_has_ta(&[], &san(&kfp), Lat, &["cikalpizate", "cikxpsate"]);
}

#[test]
fn sutra_7_2_61() {
    // The four dhatus below (yA, ci, ni, hu) are vew per 7.2.63.
    let yaa = d("yA\\", Adadi);
    assert_has_krdanta(&[], &yaa, Krt::tfc, &["yAtf"]);
    assert_has_sip(&[], &yaa, Lit, &["yayATa", "yayiTa"]);

    let ci = d("ci\\Y", Svadi);
    assert_has_krdanta(&[], &ci, Krt::tfc, &["cetf"]);
    assert_has_sip(&[], &ci, Lit, &["ciceTa", "cicayiTa", "cikeTa", "cikayiTa"]);

    let ni = d("RI\\Y", Bhvadi);
    assert_has_krdanta(&[], &ni, Krt::tfc, &["netf"]);
    assert_has_sip(&[], &ni, Lit, &["nineTa", "ninayiTa"]);

    let hu = d("hu\\", Juhotyadi);
    assert_has_krdanta(&[], &hu, Krt::tfc, &["hotf"]);
    assert_has_sip(
        &[],
        &hu,
        Lit,
        &[
            "juhoTa",
            "juhaviTa",
            "juhavAYcakarTa",
            "juhavAmAsiTa",
            "juhavAmbaBUviTa",
        ],
    );

    // acaH
    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_krdanta(&[], &bhid, Krt::tfc, &["Bettf"]);
    assert_has_sip(&[], &bhid, Lit, &["biBediTa"]);
    // tAsvat
    let lu = d("lUY", Kryadi);
    assert_has_krdanta(&[], &lu, Krt::ktvA, &["lUtvA"]);
    assert_has_sip(&[], &lu, Lit, &["lulaviTa"]);
    // Tali
    assert_has_vas(&[], &yaa, Lit, &["yayiva"]);
    assert_has_mas(&[], &yaa, Lit, &["yayima"]);

    // nityagrahaRam
    let dhu = d("DUY", Kryadi);
    assert_has_krdanta(&["vi"], &dhu, Krt::tfc, &["viDotf", "viDavitf"]);
    // SK justifies viduDoTa.
    assert_has_sip(&["vi"], &dhu, Lit, &["viduDaviTa", "viduDoTa"]);
}

#[test]
fn sutra_7_2_62() {
    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_sip(&[], &yaj, Lit, &["iyazWa", "iyajiTa"]);
}

#[test]
fn sutra_7_2_63() {
    let smf = d("smf\\", Bhvadi);
    assert_has_krdanta(&[], &smf, Krt::tfc, &["smartf"]);
    assert_has_sip(&[], &smf, Lit, &["sasmarTa"]);
    let dhvf = d("Dvf\\", Bhvadi);
    assert_has_krdanta(&[], &dhvf, Krt::tfc, &["Dvartf"]);
    assert_has_sip(&[], &dhvf, Lit, &["daDvarTa"]);
    // vikalpa for others
    assert_has_sip(&[], &d("yA\\", Adadi), Lit, &["yayiTa", "yayATa"]);
    assert_has_sip(&[], &d("vA\\", Adadi), Lit, &["vaviTa", "vavATa"]);
    assert_has_sip(&[], &d("qupa\\ca~^z", Adadi), Lit, &["peciTa", "papakTa"]);
    assert_has_sip(&[], &d("Sa\\kx~", Adadi), Lit, &["SekiTa", "SaSakTa"]);
}

#[test]
fn sutra_7_2_65() {
    assert_has_sip(&[], &d("sf\\ja~", Tudadi), Lit, &["sasrazWa", "sasarjiTa"]);
    assert_has_sip(&[], &d("df\\Si~r", Bhvadi), Lit, &["dadrazWa", "dadarSiTa"]);
}

#[test]
fn sutra_7_2_66() {
    assert_has_sip(&[], &d("a\\da~", Adadi), Lit, &["AdiTa", "jaGasiTa"]);
    assert_has_sip(&[], &d("f\\", Bhvadi), Lit, &["AriTa"]);
    assert_has_sip(&[], &d("vye\\Y", Bhvadi), Lit, &["vivyayiTa"]);
}

#[test]
fn sutra_7_2_67() {
    // ekAc
    assert_has_kvasu_su(&d("a\\da~", Adadi), &["AdivAn", "jakzivAn"]);
    assert_has_kvasu_su(&d("aSa~", Kryadi), &["ASivAn"]);
    assert_has_kvasu_su(&d("qupa\\ca~^z", Bhvadi), &["pecivAn"]);
    assert_has_kvasu_su(&d("Sa\\kx~", Svadi), &["SekivAn"]);

    // At
    assert_has_kvasu_su(&d("yA\\", Adadi), &["yayivAn"]);
    assert_has_kvasu_su(&d("zWA\\", Bhvadi), &["tasTivAn"]);

    // Gas
    assert_has_kvasu_su(&d("Gasx~", Bhvadi), &["jakzivAn"]);

    // ekAc?
    assert_has_kvasu_su(&d("Bi\\di~^r", Rudhadi), &["biBidvAn"]);
    assert_has_kvasu_su(&d("Ci\\di~^r", Rudhadi), &["cicCidvAn"]);
    assert_has_kvasu_su(&d("BU", Bhvadi), &["baBUvAn"]);
    assert_has_kvasu_su(&d("SriY", Bhvadi), &["SiSrivAn"]);

    // TODO: double check these.
    assert_has_kvasu_su(
        &d("daridrA", Adadi),
        &[
            "dadaridrvAn",
            "daridrAYcakfvAn",
            "daridrAmbaBUvAn",
            "daridrAmAsivAn",
        ],
    );
}

#[test]
fn sutra_7_2_68() {
    assert_has_kvasu_su(&d("ga\\mx~", Bhvadi), &["jagmivAn", "jaganvAn"]);
    assert_has_kvasu_su(&d("ha\\na~", Adadi), &["jaGnivAn", "jaGanvAn"]);
    assert_has_kvasu_su(
        &d("vida~", Adadi),
        &[
            "vividivAn",
            "vividvAn",
            "vidAmAsivAn",
            "vidAmbaBUvAn",
            "vidAYcakfvAn",
        ],
    );
    assert_has_kvasu_su(&d("vi\\Sa~", Tudadi), &["viviSivAn", "viviSvAn"]);
}

#[test]
fn sutra_7_2_68_v1() {
    assert_has_kvasu_su(&d("df\\Si~r", Bhvadi), &["dadfSivAn", "dadfSvAn"]);
}

#[test]
fn sutra_7_2_70() {
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lrt, &["karizyati"]);
    assert_has_tip(&[], &d("hf\\Y", Bhvadi), Lrt, &["harizyati"]);
    assert_has_tip(&[], &d("ha\\na~", Adadi), Lrt, &["hanizyati"]);
    assert_has_tip(&[], &d("svf", Bhvadi), Lrt, &["svarizyati"]);
}

#[test]
fn sutra_7_2_71() {
    let anj = d("anjU~", Rudhadi);
    assert_has_lun(&[], &anj, &["AYjIt"]);
    assert_has_krdanta(&[], &anj, Krt::tfc, &["aNktf", "aYjitf"]);
}

#[test]
fn sutra_7_2_72() {
    let stu = d("zwu\\Y", Adadi);
    let su = d("zu\\Y", Svadi);
    let dhu = d("DUY", Svadi);

    assert_has_tip(&[], &stu, Lun, &["astAvIt"]);
    assert_has_tip(&[], &su, Lun, &["asAvIt"]);
    assert_has_tip(&[], &dhu, Lun, &["aDAvIt"]);

    assert_has_ta(&[], &stu, Lun, &["astozwa"]);
    assert_has_ta(&[], &su, Lun, &["asozwa"]);
    assert_has_ta(&[], &dhu, Lun, &["aDozwa", "aDavizwa"]);
}

#[test]
fn sutra_7_2_73() {
    let yam = d("ya\\ma~", Bhvadi);
    let ram = d("ra\\mu~\\", Bhvadi);
    let nam = d("Ra\\ma~", Bhvadi);
    let yaa = d("yA\\", Adadi);

    assert_has_tip(&[], &yam, Lun, &["ayaMsIt"]);
    assert_has_tip(&["vi"], &ram, Lun, &["vyaraMsIt"]);
    assert_has_lun(&[], &nam, &["anaMsIt"]);
    assert_has_tip(&[], &yaa, Lun, &["ayAsIt"]);

    // TODO: not sure how to derive ayaMsta and anaMsta
    // assert_has_lun_karmani(&[], &yam, &["ayaMsta"]);
    assert_has_ta(&[], &ram, Lun, &["araMsta"]);
    // assert_has_lun_karmani(&[], &nam, &["anaMsta"]);
}

#[test]
fn sutra_7_2_74() {
    let assert_has_san_lat = |u, gana, exp| {
        let dhatu = d(u, gana).with_sanadi(&[Sanadi::san]);
        assert_has_lat(&[], &dhatu, exp);
    };

    assert_has_san_lat("zmi\\N", Bhvadi, &["sismayizate"]);
    assert_has_san_lat("pUN", Bhvadi, &["pipavizate"]);
    assert_has_san_lat("f\\", Bhvadi, &["aririzati"]);
    assert_has_san_lat("anjU~", Rudhadi, &["aYjijizati"]);
    assert_has_san_lat("aSU~\\", Svadi, &["aSiSizate"]);
    assert_has_san_lat("pUY", Kryadi, &["pupUzati", "pupUzate"]);
}

#[test]
fn sutra_7_2_75() {
    assert_has_tip(&[], &san(&d("kF", Tudadi)), Lat, &["cikarizati"]);
    assert_has_tip(
        &[],
        &san(&d("gF", Tudadi)),
        Lat,
        &["jigarizati", "jigalizati"],
    );
    assert_has_ta(&[], &san(&d("df\\N", Tudadi)), Lat, &["didarizate"]);
    assert_has_ta(&[], &san(&d("Df\\N", Tudadi)), Lat, &["diDarizate"]);
    assert_has_tip(&[], &san(&d("pra\\Ca~", Tudadi)), Lat, &["pipfcCizati"]);
    assert_has_tip(&[], &san(&d("sf\\ja~", Tudadi)), Lat, &["sisfkzati"]);
}

#[test]
fn sutra_7_2_76() {
    let rud = d("rudi~r", Adadi);
    let svap = d("Yizva\\pa~", Adadi);

    assert_has_tip(&[], &rud, Lat, &["roditi"]);
    assert_has_tip(&[], &svap, Lat, &["svapiti"]);
    assert_has_tip(&[], &d("Svasa~", Adadi), Lat, &["Svasiti"]);
    assert_has_tip(&["pra"], &d("ana~", Adadi), Lat, &["prARiti"]);
    assert_has_tip(&[], &d("jakza~", Adadi), Lat, &["jakziti"]);

    assert_has_tip(&[], &d("jAgf", Adadi), Lat, &["jAgarti"]);
    assert_has_krdanta(&[], &svap, Krt::tfc, &["svaptf"]);
    assert_has_jhi(&[], &rud, Lakara::Lat, &["rudanti"]);
}

#[test]
fn sutra_7_2_77() {
    let ish = d("ISa~\\", Adadi);
    assert_has_thaas(&[], &ish, Lat, &["ISize"]);
    assert_has_thaas(&[], &ish, Lot, &["ISizva"]);
}

#[ignore]
#[test]
fn sutra_7_2_78() {
    let id = d("Iqa~\\", Adadi);
    assert_has_dhvam(&[], &id, Lat, &["IqiDve"]);
    assert_has_dhvam(&[], &id, Lot, &["IqiDvam"]);
    assert_has_thaas(&[], &id, Lat, &["Iqize"]);
    assert_has_thaas(&[], &id, Lot, &["Iqizva"]);

    // TODO: let jan = d("janI~\\", Divadi);

    // HACK: force atmanepada by adding anudAttet.
    let jan = d("jana~", Juhotyadi);
    assert_has_thaas(&["vi", "ati"], &jan, Lat, &["vyatijajYize"]);
    assert_has_thaas(&["vi", "ati"], &jan, Lot, &["vyatijajYizva"]);
    assert_has_dhvam(&["vi", "ati"], &jan, Lat, &["vyatijajYiDve"]);
    assert_has_dhvam(&["vi", "ati"], &jan, Lot, &["vyatijajYiDvam"]);

    let ish = d("ISa~\\", Adadi);
    assert_has_dhvam(&[], &ish, Lat, &["ISiDve"]);
    assert_has_dhvam(&[], &ish, Lot, &["ISiDvam"]);
}

#[test]
fn sutra_7_2_79() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_tinantas(&[], &kf, VidhiLin, Prathama, Eka, &["kuryAt", "kurvIta"]);
    assert_has_tinantas(
        &[],
        &kf,
        VidhiLin,
        Prathama,
        Dvi,
        &["kuryAtAm", "kurvIyAtAm"],
    );
    assert_has_tinantas(&[], &kf, VidhiLin, Prathama, Bahu, &["kuryuH", "kurvIran"]);
    // anantasya
    assert_has_sip(&[], &kf, VidhiLin, &["kuryAH"]);
    // sArvadhAtuke
    assert_has_tinantas(
        &[],
        &kf,
        AshirLin,
        Prathama,
        Dvi,
        &["kriyAstAm", "kfzIyAstAm"],
    );
    assert_has_tinantas(&[], &kf, AshirLin, Prathama, Bahu, &["kriyAsuH", "kfzIran"]);
    assert_has_ta(&[], &kf, AshirLin, &["kfzIzwa"]);
}

#[test]
fn sutra_7_2_80() {
    assert_has_tip(&[], &d("qupa\\ca~^z", Bhvadi), VidhiLin, &["pacet"]);
    assert_has_tip(&[], &d("ci\\Y", Svadi), VidhiLin, &["cinuyAt"]);
    assert_has_tip(&[], &d("zu\\Y", Svadi), VidhiLin, &["sunuyAt"]);
    assert_has_tip(&[], &d("yA\\", Adadi), VidhiLin, &["yAyAt"]);

    let _cikirs = d("qukf\\Y", Tanadi).with_sanadi(&[Sanadi::san]);
    // TODO: cikIrzyAt
}

#[test]
fn sutra_7_2_81() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_aataam(&[], &pac, Lat, &["pacete"]);
    assert_has_aathaam(&[], &pac, Lat, &["paceTe"]);
    assert_has_aataam(&[], &pac, Lot, &["pacetAm"]);
    assert_has_aathaam(&[], &pac, Lot, &["paceTAm"]);
    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_aataam(&[], &yaj, Lat, &["yajete"]);
    assert_has_aathaam(&[], &yaj, Lat, &["yajeTe"]);
    assert_has_aataam(&[], &yaj, Lot, &["yajetAm"]);
    assert_has_aathaam(&[], &yaj, Lot, &["yajeTAm"]);
    // AtaH
    assert_has_jhi(&[], &pac, Lat, &["pacanti"]);
    assert_has_jhi(&[], &yaj, Lat, &["yajanti"]);
    assert_has_jha(&[], &pac, Lat, &["pacante"]);
    assert_has_jha(&[], &yaj, Lat, &["yajante"]);
    // NitaH
    assert_has_vahi(&[], &pac, Lot, &["pacAvahE"]);
    assert_has_mahin(&[], &pac, Lot, &["pacAmahE"]);
    // ataH
    assert_has_aataam(&[], &d("ci\\Y", Svadi), Lat, &["cinvAte"]);
    assert_has_aataam(&[], &d("zu\\Y", Svadi), Lat, &["sunvAte"]);
    // tapara
    assert_has_aataam(&[], &d("mA\\N", Juhotyadi), Lat, &["mimAte"]);
    assert_has_aathaam(&[], &d("mA\\N", Juhotyadi), Lat, &["mimATe"]);
}

#[test]
fn sutra_7_2_82() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::SAnac, &["pacamAna"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::SAnac, &["yajamAna"]);
}

#[test]
fn sutra_7_2_83() {
    let aas = d("Asa~\\", Adadi);
    assert_has_krdanta(&[], &aas, Krt::SAnac, &["AsIna"]);
}

#[test]
fn sutra_7_2_84() {
    assert_has_sup_3p("azwan", Pum, &["azwABiH", "azwaBiH"]);
    assert_has_sup_4p("azwan", Pum, &["azwAByaH", "azwaByaH"]);
    assert_has_sup_6p("azwan", Pum, &["azwAnAm"]);
    assert_has_sup_7p("azwan", Pum, &["azwAsu", "azwasu"]);
    // viBaktO
    assert_has_taddhita("azwan", T::tva, &["azwatva"]);
    assert_has_taddhita("azwan", T::tal, &["azwatA"]);
}

#[test]
fn sutra_7_2_85() {
    assert_has_sup_3d("rE", Pum, &["rAByAm"]);
    assert_has_sup_3p("rE", Pum, &["rABiH"]);
    // hali
    assert_has_sup_1d("rE", Pum, &["rAyO"]);
    // viBaktO
    assert_has_taddhita("rE", T::tva, &["rEtva"]);
    assert_has_taddhita("rE", T::tal, &["rEtA"]);
}

#[test]
fn sutra_7_2_86() {
    assert_has_sup_3p("yuzmad", Pum, &["yuzmABiH"]);
    assert_has_sup_3p("asmad", Pum, &["asmABiH"]);
    assert_has_sup_7p("yuzmad", Pum, &["yuzmAsu"]);
    assert_has_sup_7p("asmad", Pum, &["asmAsu"]);
    // anAdeSe
    assert_has_sup_5p("yuzmad", Pum, &["yuzmat"]);
    assert_has_sup_5p("asmad", Pum, &["asmat"]);
}

#[test]
fn sutra_7_2_87() {
    assert_has_sup_2s("yuzmad", Pum, &["tvAm"]);
    assert_has_sup_2s("asmad", Pum, &["mAm"]);
    assert_has_sup_2d("yuzmad", Pum, &["yuvAm"]);
    assert_has_sup_2d("asmad", Pum, &["AvAm"]);
}

#[test]
fn sutra_7_2_88() {
    assert_has_sup_1d("yuzmad", Pum, &["yuvAm"]);
    assert_has_sup_1d("asmad", Pum, &["AvAm"]);

    assert_has_sup_6d("yuzmad", Pum, &["yuvayoH"]);
    assert_has_sup_6d("asmad", Pum, &["AvayoH"]);
    assert_has_sup_1s("yuzmad", Pum, &["tvam"]);
    assert_has_sup_1s("asmad", Pum, &["aham"]);
    assert_has_sup_1p("yuzmad", Pum, &["yUyam"]);
    assert_has_sup_1p("asmad", Pum, &["vayam"]);
}

#[test]
fn sutra_7_2_89() {
    assert_has_sup_3s("yuzmad", Pum, &["tvayA"]);
    assert_has_sup_3s("asmad", Pum, &["mayA"]);
    assert_has_sup_7s("yuzmad", Pum, &["tvayi"]);
    assert_has_sup_7s("asmad", Pum, &["mayi"]);
    assert_has_sup_7d("yuzmad", Pum, &["yuvayoH"]);
    assert_has_sup_7d("asmad", Pum, &["AvayoH"]);

    assert_has_sup_3d("yuzmad", Pum, &["yuvAByAm"]);
    assert_has_sup_3d("asmad", Pum, &["AvAByAm"]);

    assert_has_sup_5s("yuzmad", Pum, &["tvat"]);
    assert_has_sup_5s("asmad", Pum, &["mat"]);
}

#[test]
fn sutra_7_2_90() {
    assert_has_sup_1s("yuzmad", Pum, &["tvam"]);
    assert_has_sup_1s("asmad", Pum, &["aham"]);
    assert_has_sup_1p("yuzmad", Pum, &["yUyam"]);
    assert_has_sup_1p("asmad", Pum, &["vayam"]);
    assert_has_sup_4s("yuzmad", Pum, &["tuByam"]);
    assert_has_sup_4s("asmad", Pum, &["mahyam"]);
    assert_has_sup_4p("yuzmad", Pum, &["yuzmaByam"]);
    assert_has_sup_4p("asmad", Pum, &["asmaByam"]);
    assert_has_sup_5s("yuzmad", Pum, &["tvat"]);
    assert_has_sup_5s("asmad", Pum, &["mat"]);
    assert_has_sup_5p("yuzmad", Pum, &["yuzmat"]);
    assert_has_sup_5p("asmad", Pum, &["asmat"]);
    assert_has_sup_6s("yuzmad", Pum, &["tava"]);
    assert_has_sup_6s("asmad", Pum, &["mama"]);
    assert_has_sup_6p("yuzmad", Pum, &["yuzmAkam"]);
    assert_has_sup_6p("asmad", Pum, &["asmAkam"]);
}

#[test]
fn sutra_7_2_92() {
    assert_has_sup_1d("yuzmad", Pum, &["yuvAm"]);
    assert_has_sup_1d("asmad", Pum, &["AvAm"]);
    assert_has_sup_3d("yuzmad", Pum, &["yuvAByAm"]);
    assert_has_sup_3d("asmad", Pum, &["AvAByAm"]);
    assert_has_sup_6d("yuzmad", Pum, &["yuvayoH"]);
    assert_has_sup_6d("asmad", Pum, &["AvayoH"]);
    // TODO: others
}

#[test]
fn sutra_7_2_93() {
    assert_has_sup_1p("yuzmad", Pum, &["yUyam"]);
    assert_has_sup_1p("asmad", Pum, &["vayam"]);
    // TODO: others
}

#[test]
fn sutra_7_2_94() {
    assert_has_sup_1s("yuzmad", Pum, &["tvam"]);
    assert_has_sup_1s("asmad", Pum, &["aham"]);
    // TODO: others
}

#[test]
fn sutra_7_2_95() {
    assert_has_sup_4s("yuzmad", Pum, &["tuByam"]);
    assert_has_sup_4s("asmad", Pum, &["mahyam"]);
    // TODO: others
}

#[test]
fn sutra_7_2_96() {
    assert_has_sup_6s("yuzmad", Pum, &["tava"]);
    assert_has_sup_6s("asmad", Pum, &["mama"]);
    // TODO: others
}

#[test]
fn sutra_7_2_97() {
    assert_has_sup_2s("yuzmad", Pum, &["tvAm"]);
    assert_has_sup_2s("asmad", Pum, &["mAm"]);
    assert_has_sup_3s("yuzmad", Pum, &["tvayA"]);
    assert_has_sup_3s("asmad", Pum, &["mayA"]);
    assert_has_sup_5s("yuzmad", Pum, &["tvat"]);
    assert_has_sup_5s("asmad", Pum, &["mat"]);
    assert_has_sup_7s("yuzmad", Pum, &["tvayi"]);
    assert_has_sup_7s("asmad", Pum, &["mayi"]);
    // TODO: others
}

#[test]
fn sutra_7_2_99() {
    assert_has_sup_1p("tri", Stri, &["tisraH"]);
    assert_has_sup_1p("catur", Stri, &["catasraH"]);
    assert_has_sup_3p("tri", Stri, &["tisfBiH"]);
    assert_has_sup_3p("catur", Stri, &["catasfBiH"]);
    // striyAm
    assert_has_sup_1p("tri", Pum, &["trayaH"]);
    assert_has_sup_1p("catur", Pum, &["catvAraH"]);
    assert_has_sup_1p("tri", Napumsaka, &["trIRi"]);
    assert_has_sup_1p("catur", Napumsaka, &["catvAri"]);
}

#[test]
fn sutra_7_2_100() {
    assert_has_sup_1p("tri", Stri, &["tisraH"]);
    assert_has_sup_1p("catur", Stri, &["catasraH"]);
    assert_has_sup_2p("tri", Stri, &["tisraH"]);
    assert_has_sup_2p("catur", Stri, &["catasraH"]);
    // TODO: others
}

#[test]
fn sutra_7_2_101() {
    let jara = nyap("jarA");
    assert_has_sup_3s(&jara, Stri, &["jarasA", "jarayA"]);
    assert_has_sup_4s(&jara, Stri, &["jarase", "jarAyE"]);
    // aci
    assert_has_sup_3d(&jara, Stri, &["jarAByAm"]);
    assert_has_sup_3p(&jara, Stri, &["jarABiH"]);
    // TODO: others
}

#[test]
fn sutra_7_2_102() {
    assert_has_sup_1s("tyad", Pum, &["syaH"]);
    assert_has_sup_1d("tyad", Pum, &["tyO"]);
    assert_has_sup_1p("tyad", Pum, &["tye"]);
    assert_has_sup_1s("tad", Pum, &["saH"]);
    assert_has_sup_1d("tad", Pum, &["tO"]);
    assert_has_sup_1p("tad", Pum, &["te"]);
    assert_has_sup_1s("yad", Pum, &["yaH"]);
    assert_has_sup_1d("yad", Pum, &["yO"]);
    assert_has_sup_1p("yad", Pum, &["ye"]);
    assert_has_sup_1s("etad", Pum, &["ezaH"]);
    assert_has_sup_1d("etad", Pum, &["etO"]);
    assert_has_sup_1p("etad", Pum, &["ete"]);
    assert_has_sup_1s("idam", Pum, &["ayam"]);
    assert_has_sup_1d("idam", Pum, &["imO"]);
    assert_has_sup_1p("idam", Pum, &["ime"]);
    assert_has_sup_1s("adas", Pum, &["asO"]);
    assert_has_sup_1d("adas", Pum, &["amU"]);
    assert_has_sup_1p("adas", Pum, &["amI"]);
    // exception
    assert_has_sup_1d("dvi", Pum, &["dvO"]);
    assert_has_sup_3d("dvi", Pum, &["dvAByAm"]);
}

#[test]
fn sutra_7_2_103() {
    assert_has_sup_1s("kim", Pum, &["kaH"]);
    assert_has_sup_1d("kim", Pum, &["kO"]);
    assert_has_sup_1p("kim", Pum, &["ke"]);
}

#[test]
fn sutra_7_2_104() {
    assert_has_taddhita("kim", T::tasil, &["kutas"]);
    assert_has_taddhita("kim", T::tral, &["kutra"]);
    assert_has_taddhita("kim", T::ha, &["kuha"]);
}

#[test]
fn sutra_7_2_105() {
    assert_has_taddhita("kim", T::at, &["kva"]);
}

#[test]
fn sutra_7_2_106() {
    assert_has_sup_1s("tyad", Pum, &["syaH"]);
    assert_has_sup_1s("tad", Pum, &["saH"]);
    assert_has_sup_1s("etad", Pum, &["ezaH"]);
    assert_has_sup_1s("adas", Pum, &["asO"]);
}

#[test]
fn sutra_7_2_107() {
    assert_has_sup_1s("adas", Pum, &["asO"]);
    // TODO: others
}

#[test]
fn sutra_7_2_108() {
    assert_has_sup_1s("idam", Stri, &["iyam"]);
    assert_has_sup_1s("idam", Pum, &["ayam"]);
}

#[test]
fn sutra_7_2_109() {
    assert_has_sup_1d("idam", Pum, &["imO"]);
    assert_has_sup_1p("idam", Pum, &["ime"]);
    assert_has_sup_2s("idam", Pum, &["imam"]);
    assert_has_sup_2d("idam", Pum, &["imO"]);
    assert_has_sup_2p("idam", Pum, &["imAn"]);
}

#[test]
fn sutra_7_2_110() {
    assert_has_sup_1s("idam", Stri, &["iyam"]);
}

#[test]
fn sutra_7_2_111() {
    assert_has_sup_1s("idam", Pum, &["ayam"]);
    // puMsi
    assert_has_sup_1s("idam", Stri, &["iyam"]);
}

#[test]
fn sutra_7_2_112() {
    assert_has_sup_3s("idam", Pum, &["anena"]);
    assert_has_sup_6d("idam", Pum, &["anayoH"]);
    // TODO: others
}

#[test]
fn sutra_7_2_113() {
    assert_has_sup_3d("idam", Pum, &["AByAm"]);
    assert_has_sup_3p("idam", Pum, &["eBiH"]);
    assert_has_sup_5p("idam", Pum, &["eByaH"]);
    assert_has_sup_6p("idam", Pum, &["ezAm"]);
    assert_has_sup_7p("idam", Pum, &["ezu"]);
    // TODO: others
}

#[test]
fn sutra_7_2_114() {
    let mfj = d("mfjU~", Adadi);
    assert_has_krdanta(&[], &mfj, Krt::tfc, &["mArzwf", "mArjitf"]);
    assert_has_krdanta(&[], &mfj, Krt::tumun, &["mArzwum", "mArjitum"]);
    assert_has_krdanta(&[], &mfj, Krt::tavya, &["mArzwavya", "mArjitavya"]);
    // TODO: kaMsaparimfqByAm, etc.
}

#[test]
fn sutra_7_2_115() {
    assert_has_sup_1d("saKi", Pum, &["saKAyO"]);
    assert_has_sup_1p("saKi", Pum, &["saKAyaH"]);
    assert_has_sup_2s("saKi", Pum, &["saKAyam"]);
    assert_has_sup_2d("saKi", Pum, &["saKAyO"]);

    assert_has_sup_3s("saKi", Pum, &["saKyA"]);
    assert_has_sup_4s("saKi", Pum, &["saKye"]);
}

#[test]
fn sutra_7_2_116() {
    let nic = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::Ric]);

    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::GaY, &["pAka"]);
    assert_has_krdanta(&[], &d("tya\\ja~", Bhvadi), Krt::GaY, &["tyAga"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::GaY, &["yAga"]);
    assert_has_tip(&[], &nic(&pac), Lat, &["pAcayati"]);
    assert_has_krdanta(&[], &pac, Krt::Rvul, &["pAcaka"]);

    let path = d("paWa~", Bhvadi);
    assert_has_tip(&[], &nic(&path), Lat, &["pAWayati"]);
    assert_has_krdanta(&[], &path, Krt::Rvul, &["pAWaka"]);
    // ataH
    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_tip(&[], &nic(&bhid), Lat, &["Bedayati"]);
    assert_has_krdanta(&[], &bhid, Krt::Rvul, &["Bedaka"]);
    // upadhAyAH
    assert_has_tip(&[], &nic(&d("cakAsf~", Adadi)), Lat, &["cakAsayati"]);
    assert_has_krdanta(&[], &d("takza~", Bhvadi), Krt::Rvul, &["takzaka"]);
}

#[test]
fn sutra_7_2_117() {
    assert_has_taddhita("garga", T::yaY, &["gArgya"]);
    assert_has_taddhita("vatsa", T::yaY, &["vAtsya"]);
    assert_has_taddhita("dakza", T::iY, &["dAkzi"]);
    assert_has_taddhita("plakza", T::iY, &["plAkzi"]);
    assert_has_taddhita("upagu", T::aR, &["Opagava"]);
    assert_has_taddhita("kapawu", T::aR, &["kApawava"]);
    // TODO: others
}

#[test]
fn sutra_7_2_118() {
    assert_has_taddhita("naqa", T::Pak, &["nAqAyana"]);
    assert_has_taddhita("cara", T::Pak, &["cArAyaRa"]);
    assert_has_taddhita("akza", T::Wak, &["Akzika"]);
    assert_has_taddhita("SalAkA", T::Wak, &["SAlAkika"]);
    // TODO: others
}
