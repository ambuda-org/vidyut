extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;

pub fn assert_has_lit_p_1d(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(
        prefixes,
        dhatu,
        Lakara::Lit,
        Purusha::Uttama,
        Vacana::Dvi,
        expected,
    );
}

pub fn assert_has_lit_p_2s(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(
        prefixes,
        dhatu,
        Lakara::Lit,
        Purusha::Madhyama,
        Vacana::Eka,
        expected,
    );
}

pub fn assert_has_lit_a_1d(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Uttama)
        .vacana(Vacana::Dvi)
        .lakara(Lakara::Lit)
        .pada(Pada::Atmane)
        .build()
        .unwrap();

    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

#[test]
fn sutra_7_2_1() {
    let d = Dhatu::new;
    assert_has_lun_p(&[], &d("ci\\Y", Svadi), &["acEzIt"]);
    assert_has_lun_p(&[], &d("RI\\Y", Bhvadi), &["anEzIt"]);
    assert_has_lun_p(&[], &d("lUY", Kryadi), &["alAvIt"]);
    assert_has_lun_p(&[], &d("pUY", Kryadi), &["apAvIt"]);
    assert_has_lun_p(&[], &d("qukf\\Y", Tanadi), &["akArzIt"]);
    assert_has_lun_p(&[], &d("hf\\Y", Bhvadi), &["ahArzIt"]);

    let kutadi = |u| Dhatu::new(u, Tudadi).with_antargana(Some(Antargana::Kutadi));
    assert_has_lun_p(&["ni"], &kutadi("RU"), &["nyanuvIt"]);
    assert_has_lun_p(&["ni"], &kutadi("DU"), &["nyaDuvIt"]);

    assert_has_lun_a(&[], &d("cyu\\N", Bhvadi), &["acyozwa"]);
    assert_has_lun_a(&[], &d("plu\\N", Bhvadi), &["aplozwa"]);
}

#[test]
fn sutra_7_2_2() {
    let d = Dhatu::new;
    assert_has_lun_p(&[], &d("kzara~", Bhvadi), &["akzArIt"]);
    assert_has_lun_p(&[], &d("tsara~", Bhvadi), &["atsArIt"]);
    assert_has_lun_p(&[], &d("jvala~", Bhvadi), &["ajvAlIt"]);
    assert_has_lun_p(&[], &d("hmala~", Bhvadi), &["ahmAlIt"]);

    assert_has_lun_p(&["ni"], &d("Kura~", Tudadi), &["nyaKorIt"]);
    assert_has_lun_p(&["ni"], &d("mIla~", Svadi), &["nyamIlIt"]);

    // TODO: awIt, aSIt

    assert_has_lun_p(&[], &d("vaBra~", Bhvadi), &["avaBrIt"]);
    assert_has_lun_p(&[], &d("Svalla~", Bhvadi), &["aSvallIt"]);
}

#[test]
fn sutra_7_2_3() {
    let d = Dhatu::new;
    assert_has_lun_p(&[], &d("vada~", Bhvadi), &["avAdIt"]);
    assert_has_lun_p(&[], &d("vraja~", Bhvadi), &["avrAjIt"]);
    assert_has_lun_p(&[], &d("Bi\\di~^r", Rudhadi), &["aBEtsIt", "aBidat"]);
    assert_has_lun_p(&[], &d("Ci\\di~^r", Rudhadi), &["acCEtsIt", "acCidat"]);
    assert_has_lun_p(&[], &d("ru\\Di~^r", Rudhadi), &["arOtsIt", "aruDat"]);
    assert_has_lun_p(&[], &d("ra\\nja~^", Bhvadi), &["arANkzIt"]);
    assert_has_lun_p(&[], &d("za\\nja~", Bhvadi), &["asANkzIt"]);
    assert_has_parasmai_tinanta(
        &["ud"],
        &d("va\\ha~^", Bhvadi),
        Lakara::Lun,
        Purusha::Prathama,
        Vacana::Dvi,
        &["udavoQAm"],
    );
}

#[test]
fn sutra_7_2_4() {
    let d = Dhatu::new;
    assert_has_lun_p(&[], &d("divu~", Divadi), &["adevIt"]);
    assert_has_lun_p(&[], &d("zivu~", Divadi), &["asevIt"]);
    assert_has_lun_p(&[], &d("kuza~", Kryadi), &["akozIt"]);
    assert_has_lun_p(&[], &d("muza~", Kryadi), &["amozIt"]);
    assert_has_lun_p(&[], &d("lUY", Kryadi), &["alAvIt"]);
}

#[test]
fn sutra_7_2_5() {
    let d = Dhatu::new;
    assert_has_lun_p(&[], &d("graha~^", Kryadi), &["agrahIt"]);
    assert_has_lun_p(&[], &d("syamu~", Divadi), &["asyamIt"]);
    assert_has_lun_p(&[], &d("vyaya~^", Bhvadi), &["avyayIt"]);
    assert_has_lun_p(&[], &d("wuvama~", Bhvadi), &["avamIt"]);
    assert_has_lun_p(&[], &d("kzaRu~^", Kryadi), &["akzaRIt"]);
    assert_has_lun_p(&[], &d("Svasa~", Adadi), &["aSvasIt"]);
    assert_has_lun_p(&[], &d("jAgf", Adadi), &["ajAgarIt"]);
    assert_has_lun_p(&[], &d("rage~", Bhvadi), &["aragIt"]);
    assert_has_lun_p(&[], &d("kaKe~", Kryadi), &["akaKIt"]);
    // Ignore OnayIt and aSvayIt, which are chAndasa forms.
}

#[test]
fn sutra_7_2_6() {
    let urnu = Dhatu::new("UrRuY", Adadi);
    assert_has_lun_p(&["pra"], &urnu, &["prOrRavIt", "prOrRAvIt", "prOrRuvIt"]);
}

#[test]
fn sutra_7_2_7() {
    let d = Dhatu::new;

    assert_has_lun_p(&[], &d("kaRa~", Bhvadi), &["akaRIt", "akARIt"]);
    assert_has_lun_p(&[], &d("raRa~", Bhvadi), &["araRIt", "arARIt"]);

    let kuw = Dhatu::new("kuwa~", Tudadi).with_antargana(Some(Antargana::Kutadi));
    let puw = Dhatu::new("puwa~", Tudadi).with_antargana(Some(Antargana::Kutadi));

    // Various counterexamples
    assert_has_lun_p(&[], &d("divu~", Divadi), &["adevIt"]);
    assert_has_lun_p(&[], &d("zivu~", Divadi), &["asevIt"]);
    assert_has_lun_p(&["ni"], &kuw, &["nyakuwIt"]);
    assert_has_lun_p(&["ni"], &puw, &["nyapuwIt"]);
    assert_has_lun_p(&[], &d("takza~", Bhvadi), &["atakzIt"]);
    assert_has_lun_p(&[], &d("rakza~", Divadi), &["arakzIt"]);
    assert_has_lun_p(&[], &d("cakAsf~", Adadi), &["acakAsIt"]);
    assert_has_lun_p(&[], &d("qupa\\ca~^z", Bhvadi), &["apAkzIt"]);

    // TODO: aSIt, awIt
}

#[test]
fn sutra_7_2_10() {
    fn assert_has_tfc(p: &[&str], dhatu: &Dhatu, expected: &[&str]) {
        assert_has_krdanta(p, dhatu, Krt::tfc, expected);
    }

    use Krt::*;
    let d = Dhatu::new;

    // adanta are sew
    assert_has_parasmai_tinanta(
        &[],
        &d("ha\\na~", Adadi),
        Lun,
        Purusha::Madhyama,
        Bahu,
        &["avaDizwa"],
    );
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
    let shri = Dhatu::new("SriY", Bhvadi);
    assert_has_krdanta(&[], &shri, Krt::ktvA, &["SritvA"]);
    assert_has_krdanta(&[], &shri, Krt::kta, &["Srita"]);
    assert_has_krdanta(&[], &shri, Krt::ktavatu, &["Sritavat"]);

    let yu = Dhatu::new("yu", Adadi);
    assert_has_krdanta(&[], &yu, Krt::ktvA, &["yutvA"]);
    assert_has_krdanta(&[], &yu, Krt::kta, &["yuta"]);
    assert_has_krdanta(&[], &yu, Krt::ktavatu, &["yutavat"]);

    let lu = Dhatu::new("lUY", Kryadi);
    assert_has_krdanta(&[], &lu, Krt::ktvA, &["lUtvA"]);
    assert_has_krdanta(&[], &lu, Krt::kta, &["lUna"]);
    assert_has_krdanta(&[], &lu, Krt::ktavatu, &["lUnavat"]);

    let vf = Dhatu::new("vfN", Kryadi);
    assert_has_krdanta(&[], &vf, Krt::ktvA, &["vftvA"]);
    assert_has_krdanta(&[], &vf, Krt::kta, &["vfta"]);
    assert_has_krdanta(&[], &vf, Krt::ktavatu, &["vftavat"]);

    let tf = Dhatu::new("tF", Bhvadi);
    assert_has_krdanta(&[], &tf, Krt::ktvA, &["tIrtvA"]);
    assert_has_krdanta(&[], &tf, Krt::kta, &["tIrRa"]);
    assert_has_krdanta(&[], &tf, Krt::ktavatu, &["tIrRavat"]);

    let vid = Dhatu::new("vida~", Adadi);
    assert_has_krdanta(&[], &vid, Krt::kta, &["vidita"]);

    assert_has_krdanta(&[], &shri, Krt::tfc, &["Srayitf"]);
    assert_has_krdanta(&[], &shri, Krt::tumun, &["Srayitum"]);
    assert_has_krdanta(&[], &shri, Krt::tavya, &["Srayitavya"]);
}

#[test]
fn sutra_7_2_12() {
    let d = |u, gana| Dhatu::new(u, gana).with_sanadi(&[Sanadi::San]);
    assert_has_lat_p(&[], &d("graha~^", Kryadi), &["jiGfkzati"]);
    assert_has_lat_p(&[], &d("guhU~^", Bhvadi), &["juGukzati"]);
    assert_has_lat_p(&[], &d("ru", Adadi), &["rurUzati"]);
    assert_has_lat_p(&[], &d("lUY", Kryadi), &["lulUzati"]);
}

#[test]
fn sutra_7_2_13() {
    let d = Dhatu::new;
    assert_has_lit_p_1d(&[], &d("qukf\\Y", Tanadi), &["cakfva"]);
    assert_has_lit_p_1d(&[], &d("sf\\", Bhvadi), &["sasfva"]);
    assert_has_lit_p_1d(&[], &d("Bf\\Y", Bhvadi), &["baBfva"]);
    assert_has_lit_p_1d(
        &[],
        &d("quBf\\Y", Juhotyadi),
        &["baBfva", "biBarAYcakfva", "biBarAmAsiva", "biBarAmbaBUviva"],
    );
    assert_has_lit_p_1d(&[], &d("vfY", Svadi), &["vavfva"]);
    assert_has_lit_a_1d(&[], &d("vfN", Kryadi), &["vavfvahe"]);
    assert_has_lit_p_1d(&[], &d("zwu\\Y", Adadi), &["tuzwuva"]);
    assert_has_lit_p_1d(&[], &d("dru\\", Bhvadi), &["dudruva"]);
    assert_has_lit_p_1d(&[], &d("sru\\", Bhvadi), &["susruva"]);
    assert_has_lit_p_1d(&[], &d("Sru\\", Bhvadi), &["SuSruva"]);
}

#[test]
fn sutra_7_2_14() {
    let d = Dhatu::new;

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
    let dhu = Dhatu::new("DUY", Svadi);
    assert_has_krdanta(&["vi"], &dhu, Krt::kta, &["viDUta"]);
    assert_has_krdanta(&["vi"], &dhu, Krt::ktavatu, &["viDUtavat"]);

    let guh = Dhatu::new("guhU~^", Bhvadi);
    assert_has_krdanta(&[], &guh, Krt::kta, &["gUQa"]);
    assert_has_krdanta(&[], &guh, Krt::ktavatu, &["gUQavat"]);

    let vfdh = Dhatu::new("vfDu~\\", Bhvadi);
    assert_has_krdanta(&[], &vfdh, Krt::kta, &["vfdDa"]);
    assert_has_krdanta(&[], &vfdh, Krt::ktavatu, &["vfdDavat"]);
}

#[test]
fn sutra_7_2_16() {
    let mid = Dhatu::new("YimidA~", Divadi);
    assert_has_krdanta(&[], &mid, Krt::kta, &["minna"]);
    assert_has_krdanta(&[], &mid, Krt::ktavatu, &["minnavat"]);

    let kzvid = Dhatu::new("YikzvidA~", Divadi);
    assert_has_krdanta(&[], &kzvid, Krt::kta, &["kzviRRa"]);
    assert_has_krdanta(&[], &kzvid, Krt::ktavatu, &["kzviRRavat"]);

    let svid = Dhatu::new("YizvidA~", Bhvadi);
    assert_has_krdanta(&[], &svid, Krt::kta, &["svinna"]);
    assert_has_krdanta(&[], &svid, Krt::ktavatu, &["svinnavat"]);
}

#[test]
fn sutra_7_2_24() {
    let ard = Dhatu::new("arda~", Bhvadi);
    assert_has_krdanta(&["sam"], &ard, Krt::kta, &["samarRa", "samarRRa"]);
    assert_has_krdanta(&["ni"], &ard, Krt::kta, &["nyarRa", "nyarRRa"]);
    assert_has_krdanta(&["vi"], &ard, Krt::kta, &["vyarRa", "vyarRRa"]);

    let edh = Dhatu::new("eDa~\\", Bhvadi);
    assert_has_krdanta(&["sam"], &edh, Krt::kta, &["sameDita"]);

    assert_has_krdanta(&[], &ard, Krt::kta, &["ardita"]);
}

#[test]
fn sutra_7_2_25() {
    let ard = Dhatu::new("arda~", Bhvadi);
    assert_has_krdanta(
        &["aBi"],
        &ard,
        Krt::kta,
        &["aByarRa", "aByarRRa", "aByardita"],
    );
}

/*
#[test]
fn sutra_7_2_28() {
    let d = Dhatu::new;
    let assert_has_kta = |x, y, z| assert_has_krdanta(x, &y, Krt::kta, z);
    assert_has_kta(&[], d("ruza~", Bhvadi), &["ruzwa", "ruzita"]);
    assert_has_kta(&["aBi"], d("ama~", Bhvadi), &["aByAnta", "aByamita"]);
}
*/

#[test]
fn sutra_7_2_35() {
    let lu = Dhatu::new("lUY", Kryadi);
    assert_has_krdanta(&[], &lu, Krt::tfc, &["lavitf"]);
    assert_has_krdanta(&[], &lu, Krt::tumun, &["lavitum"]);
    assert_has_krdanta(&[], &lu, Krt::tavya, &["lavitavya"]);

    let pu = Dhatu::new("pUY", Kryadi);
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
fn sutra_7_2_37() {
    let grah = Dhatu::new("graha~^", Kryadi);
    assert_has_krdanta(&[], &grah, Krt::tfc, &["grahItf"]);
    assert_has_krdanta(&[], &grah, Krt::tumun, &["grahItum"]);
    assert_has_krdanta(&[], &grah, Krt::tavya, &["grahItavya"]);

    assert_has_lit_p_1d(&[], &grah, &["jagfhiva"]);

    // TODO: ignore cinvat it
}

#[test]
fn sutra_7_2_62() {
    let yaj = Dhatu::new("ya\\ja~^", Bhvadi);
    assert_has_lit_p_2s(&[], &yaj, &["iyazWa", "iyajiTa"]);
}

#[test]
fn sutra_7_2_66() {
    let d = Dhatu::new;
    // TODO: check jaGasTa
    assert_has_lit_p_2s(&[], &d("a\\da~", Adadi), &["AdiTa", "jaGasiTa", "jaGasTa"]);
    assert_has_lit_p_2s(&[], &d("f\\", Bhvadi), &["AriTa"]);
    assert_has_lit_p_2s(&[], &d("vye\\Y", Bhvadi), &["vivyayiTa"]);
}

#[test]
fn sutra_7_2_68() {
    let assert_has_kvasu = |d, exp| {
        assert_has_krdanta(&[], &d, Krt::kvasu, exp);
    };

    assert_has_kvasu(Dhatu::new("ga\\mx~", Bhvadi), &["jagmivas", "jaganvas"]);
    assert_has_kvasu(Dhatu::new("ha\\na~", Adadi), &["jaGnivas", "jaGanvas"]);
    assert_has_kvasu(
        Dhatu::new("vida~", Adadi),
        &[
            "vividivas",
            "vividvas",
            "vidAmAsivas",
            "vidAmbaBUvas",
            "vidAYcakfvas",
        ],
    );
    assert_has_kvasu(Dhatu::new("vi\\Sa~", Tudadi), &["viviSivas", "viviSvas"]);
    assert_has_kvasu(Dhatu::new("df\\Si~r", Bhvadi), &["dadfSivas", "dadfSvas"]);
}

#[test]
fn sutra_7_2_70() {
    let d = Dhatu::new;
    assert_has_lrt_p(&[], &d("qukf\\Y", Tanadi), &["karizyati"]);
    assert_has_lrt_p(&[], &d("hf\\Y", Bhvadi), &["harizyati"]);
    assert_has_lrt_p(&[], &d("ha\\na~", Adadi), &["hanizyati"]);
    assert_has_lrt_p(&[], &d("svf", Bhvadi), &["svarizyati"]);
}

#[test]
fn sutra_7_2_71() {
    let anj = Dhatu::new("anjU~", Rudhadi);
    assert_has_lun(&[], &anj, &["AYjIt"]);
    assert_has_krdanta(&[], &anj, Krt::tfc, &["aNktf", "aYjitf"]);
}

#[test]
fn sutra_7_2_72() {
    let stu = Dhatu::new("zwu\\Y", Adadi);
    let su = Dhatu::new("zu\\Y", Svadi);
    let dhu = Dhatu::new("DUY", Svadi);

    assert_has_lun_p(&[], &stu, &["astAvIt"]);
    assert_has_lun_p(&[], &su, &["asAvIt"]);
    assert_has_lun_p(&[], &dhu, &["aDAvIt"]);

    assert_has_lun_a(&[], &stu, &["astozwa"]);
    assert_has_lun_a(&[], &su, &["asozwa"]);
    assert_has_lun_a(&[], &dhu, &["aDozwa", "aDavizwa"]);
}

#[test]
fn sutra_7_2_73() {
    let yam = Dhatu::new("ya\\ma~", Bhvadi);
    let ram = Dhatu::new("ra\\mu~\\", Bhvadi);
    let nam = Dhatu::new("Ra\\ma~", Bhvadi);
    let yaa = Dhatu::new("yA\\", Adadi);

    assert_has_lun_p(&[], &yam, &["ayaMsIt"]);
    assert_has_lun_p(&["vi"], &ram, &["vyaraMsIt"]);
    assert_has_lun(&[], &nam, &["anaMsIt"]);
    assert_has_lun_p(&[], &yaa, &["ayAsIt"]);

    // TODO: not sure how to derive ayaMsta and anaMsta
    // assert_has_lun_karmani(&[], &yam, &["ayaMsta"]);
    assert_has_lun_a(&[], &ram, &["araMsta"]);
    // assert_has_lun_karmani(&[], &nam, &["anaMsta"]);
}

#[ignore]
#[test]
fn sutra_7_2_74() {
    let assert_has_san_lat = |u, gana, exp| {
        let dhatu = Dhatu::new(u, gana).with_sanadi(&[Sanadi::San]);
        assert_has_lat(&[], &dhatu, exp);
    };

    assert_has_san_lat("zmi\\N", Bhvadi, &["sismayizate"]);
    assert_has_san_lat("pUN", Bhvadi, &["pipavizate"]);
    assert_has_san_lat("f\\", Bhvadi, &["aririzati"]);
    assert_has_san_lat("anjU~", Rudhadi, &["aYjijizati"]);
    assert_has_san_lat("aSU~\\", Svadi, &["aSiSizate"]);
    assert_has_san_lat("pUY", Kryadi, &["pupUzati"]);
}

#[ignore]
#[test]
fn sutra_7_2_75() {
    let assert_has_san_lat = |u, gana, exp| {
        let dhatu = Dhatu::new(u, gana).with_sanadi(&[Sanadi::San]);
        assert_has_lat(&[], &dhatu, exp);
    };

    assert_has_san_lat("kF", Tudadi, &["cikarizati"]);
    assert_has_san_lat("gF", Tudadi, &["jigarizati", "jigalizati"]);
    assert_has_san_lat("df\\N", Tudadi, &["didarizate"]);
    assert_has_san_lat("Df\\N", Tudadi, &["diDarizate"]);
    assert_has_san_lat("pra\\Ca~", Tudadi, &["papracCizati"]);
    assert_has_san_lat("sf\\ja~", Tudadi, &["sisfkzati"]);
}

#[test]
fn sutra_7_2_76() {
    let d = Dhatu::new;
    let rud = d("rudi~r", Adadi);
    let svap = d("Yizva\\pa~", Adadi);

    assert_has_lat_p(&[], &rud, &["roditi"]);
    assert_has_lat_p(&[], &svap, &["svapiti"]);
    assert_has_lat_p(&[], &d("Svasa~", Adadi), &["Svasiti"]);
    assert_has_lat_p(&["pra"], &d("ana~", Adadi), &["prARiti"]);
    assert_has_lat_p(&[], &d("jakza~", Adadi), &["jakziti"]);

    assert_has_lat_p(&[], &d("jAgf", Adadi), &["jAgarti"]);
    assert_has_krdanta(&[], &svap, Krt::tfc, &["svaptf"]);
    assert_has_parasmai_tinanta(
        &[],
        &rud,
        Lakara::Lat,
        Purusha::Prathama,
        Vacana::Bahu,
        &["rudanti"],
    );
}

#[test]
fn sutra_7_2_77() {
    let is = Dhatu::new("ISa~\\", Adadi);
    assert_has_tinanta(
        &[],
        &is,
        Lakara::Lat,
        Purusha::Madhyama,
        Vacana::Eka,
        &["ISize"],
    );
    assert_has_tinanta(
        &[],
        &is,
        Lakara::Lot,
        Purusha::Madhyama,
        Vacana::Eka,
        &["ISizva"],
    );
}

#[test]
fn sutra_7_2_80() {
    let d = Dhatu::new;
    assert_has_vidhilin_p(&[], &d("qupa\\ca~^z", Bhvadi), &["pacet"]);
    assert_has_vidhilin_p(&[], &d("ci\\Y", Svadi), &["cinuyAt"]);
    assert_has_vidhilin_p(&[], &d("zu\\Y", Svadi), &["sunuyAt"]);
    assert_has_vidhilin_p(&[], &d("yA\\", Adadi), &["yAyAt"]);

    let _cikirs = d("qukf\\Y", Tanadi).with_sanadi(&[Sanadi::San]);
    // TODO: cikIrzyAt
}

#[test]
fn sutra_7_2_83() {
    let aas = Dhatu::new("Asa~\\", Adadi);
    assert_has_krdanta(&[], &aas, Krt::SAnac, &["AsIna"]);
}

#[test]
fn sutra_7_2_85() {
    assert_has_subantas("rE", Pum, Trtiya, Dvi, &["rAByAm"]);
    assert_has_subantas("rE", Pum, Trtiya, Bahu, &["rABiH"]);

    assert_has_subantas("rE", Pum, Prathama, Dvi, &["rAyO"]);
    // TOO: rEtva, rEtA
}

#[test]
fn sutra_7_2_87() {
    assert_has_subantas("yuzmad", Pum, Dvitiya, Eka, &["tvAm"]);
    assert_has_subantas("asmad", Pum, Dvitiya, Eka, &["mAm"]);
    assert_has_subantas("yuzmad", Pum, Dvitiya, Dvi, &["yuvAm"]);
    assert_has_subantas("asmad", Pum, Dvitiya, Dvi, &["AvAm"]);
}

#[test]
fn sutra_7_2_88() {
    assert_has_subantas("yuzmad", Pum, Prathama, Dvi, &["yuvAm"]);
    assert_has_subantas("asmad", Pum, Prathama, Dvi, &["AvAm"]);

    assert_has_subantas("yuzmad", Pum, Sasthi, Dvi, &["yuvayoH"]);
    assert_has_subantas("asmad", Pum, Sasthi, Dvi, &["AvayoH"]);
    assert_has_subantas("yuzmad", Pum, Prathama, Eka, &["tvam"]);
    assert_has_subantas("asmad", Pum, Prathama, Eka, &["aham"]);
    assert_has_subantas("yuzmad", Pum, Prathama, Bahu, &["yUyam"]);
    assert_has_subantas("asmad", Pum, Prathama, Bahu, &["vayam"]);
}

#[test]
fn sutra_7_2_89() {
    assert_has_subantas("yuzmad", Pum, Trtiya, Eka, &["tvayA"]);
    assert_has_subantas("asmad", Pum, Trtiya, Eka, &["mayA"]);
    assert_has_subantas("yuzmad", Pum, Saptami, Eka, &["tvayi"]);
    assert_has_subantas("asmad", Pum, Saptami, Eka, &["mayi"]);
    assert_has_subantas("yuzmad", Pum, Saptami, Dvi, &["yuvayoH"]);
    assert_has_subantas("asmad", Pum, Saptami, Dvi, &["AvayoH"]);

    assert_has_subantas("yuzmad", Pum, Trtiya, Dvi, &["yuvAByAm"]);
    assert_has_subantas("asmad", Pum, Trtiya, Dvi, &["AvAByAm"]);

    assert_has_subantas("yuzmad", Pum, Panchami, Eka, &["tvat"]);
    assert_has_subantas("asmad", Pum, Panchami, Eka, &["mat"]);
}

#[test]
fn sutra_7_2_114() {
    let mfj = Dhatu::new("mfjU~", Adadi);
    assert_has_krdanta(&[], &mfj, Krt::tfc, &["mArzwf", "mArjitf"]);
    assert_has_krdanta(&[], &mfj, Krt::tumun, &["mArzwum", "mArjitum"]);
    assert_has_krdanta(&[], &mfj, Krt::tavya, &["mArzwavya", "mArjitavya"]);
    // TODO: kaMsaparimfqByAm, etc.
}

#[test]
fn sutra_7_2_115() {
    assert_has_subantas("saKi", Pum, Prathama, Dvi, &["saKAyO"]);
    assert_has_subantas("saKi", Pum, Prathama, Bahu, &["saKAyaH"]);
    assert_has_subantas("saKi", Pum, Dvitiya, Eka, &["saKAyam"]);
    assert_has_subantas("saKi", Pum, Dvitiya, Dvi, &["saKAyO"]);

    assert_has_subantas("saKi", Pum, Trtiya, Eka, &["saKyA"]);
    assert_has_subantas("saKi", Pum, Caturthi, Eka, &["saKye"]);
}

#[test]
fn sutra_7_2_116() {
    let d = Dhatu::new;
    let nic = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::Nic]);

    let pac = Dhatu::new("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::GaY, &["pAka"]);
    assert_has_krdanta(&[], &d("tya\\ja~", Bhvadi), Krt::GaY, &["tyAga"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::GaY, &["yAga"]);
    assert_has_lat_p(&[], &nic(&pac), &["pAcayati"]);
    assert_has_krdanta(&[], &pac, Krt::Rvul, &["pAcaka"]);

    let path = d("paWa~", Bhvadi);
    assert_has_lat_p(&[], &nic(&path), &["pAWayati"]);
    assert_has_krdanta(&[], &path, Krt::Rvul, &["pAWaka"]);
    // ataH
    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_lat_p(&[], &nic(&bhid), &["Bedayati"]);
    assert_has_krdanta(&[], &bhid, Krt::Rvul, &["Bedaka"]);
    // upadhAyAH
    assert_has_lat_p(&[], &nic(&d("cakAsf~", Adadi)), &["cakAsayati"]);
    assert_has_krdanta(&[], &d("takza~", Bhvadi), Krt::Rvul, &["takzaka"]);
}
