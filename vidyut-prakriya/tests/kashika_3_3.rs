extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::KrtArtha::*;
use vidyut_prakriya::args::*;

fn assert_has_bhave_krdanta(upapadas: &[&str], dhatu: &Dhatu, krt: Krt, expected: &[&str]) {
    assert_has_artha_krdanta(upapadas, dhatu, KrtArtha::Bhava, krt, expected);
}

fn assert_krt_blocked(upapadas: &[&str], dhatu: &Dhatu, krt: Krt) {
    assert_has_krdanta(upapadas, dhatu, krt, &[]);
}

#[test]
fn sutra_3_3_1() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::uR, &["kAru"]);
    assert_has_krdanta(&[], &d("pA\\", Bhvadi), Krt::uR, &["pAyu"]);
    assert_has_krdanta(&[], &d("vA\\", Adadi), Krt::uR, &["vAyu"]);
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Krt::uR, &["jAyu"]);
    assert_has_krdanta(&[], &d("qumi\\Y", Svadi), Krt::uR, &["mAyu"]);
    assert_has_krdanta(&[], &d("zvada~\\", Bhvadi), Krt::uR, &["svAdu"]);
    assert_has_krdanta(&[], &d("sA\\Da~", Svadi), Krt::uR, &["sADu"]);
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), Krt::uR, &["ASu"]);

    // For more specific uNAdi tests, see `kaumudi_67.rs`
}

#[test]
fn sutra_3_3_2() {
    assert_has_krdanta(&[], &d("vftu~\\", Bhvadi), Krt::manin, &["vartman"]);
    assert_has_krdanta(&[], &d("cara~", Bhvadi), Krt::manin, &["carman"]);
}

#[test]
fn sutra_3_3_3() {
    use Krt::ini;
    use Krt::GinuR;
    use Krt::Rini;
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_krdanta(&[], &gam, ini, &["gamin"]);
    assert_has_krdanta(&["AN"], &gam, ini, &["AgAmin"]);
    assert_has_krdanta(&["pra"], &d("zWA\\", Bhvadi), ini, &["prasTAyin"]);
    // per nyAsa, these are from Rini. nyAsa also indicates that prasTAyin comes from Rini, but
    // this seems like an error given Unadi 4.9.
    assert_has_krdanta(&["prati"], &d("ru\\Di~^r", Rudhadi), Rini, &["pratiroDin"]);
    assert_has_krdanta(&["prati"], &d("buDa~", Bhvadi), Rini, &["pratiboDin"]);
    assert_has_krdanta(&["prati"], &d("yu\\Da~\\", Divadi), Rini, &["pratiyoDin"]);
    assert_has_krdanta(&["prati"], &d("yu\\ji~^r", Rudhadi), GinuR, &["pratiyogin"]);
    assert_has_krdanta(&["AN"], &d("yA\\", Adadi), Rini, &["AyAyin"]);
}

#[test]
fn sutra_3_3_10() {
    assert_has_krdanta(&[], &d("Bu\\ja~", Rudhadi), Krt::tumun, &["Boktum"]);
}

#[test]
fn sutra_3_3_13() {
    assert_has_lrt_p(&[], &d("qukf\\Y", Tanadi), &["karizyati"]);
    assert_has_lrt_p(&[], &d("hf\\Y", Bhvadi), &["harizyati"]);
}

#[test]
fn sutra_3_3_15() {
    assert_has_lut_p(&[], &d("qukf\\Y", Tanadi), &["kartA"]);
    assert_has_lut_p(&[], &d("Bu\\ja~", Rudhadi), &["BoktA"]);
}

#[test]
fn sutra_3_3_16() {
    assert_has_krdanta(&[], &d("pa\\da~\\", Divadi), Krt::GaY, &["pAda"]);
    assert_has_krdanta(&[], &d("ru\\jo~", Tudadi), Krt::GaY, &["roga"]);
    assert_has_krdanta(&[], &d("vi\\Sa~", Tudadi), Krt::GaY, &["veSa"]);
    assert_has_krdanta(&[], &d("spf\\Sa~", Tudadi), Krt::GaY, &["sparSa"]);
}

#[test]
fn sutra_3_3_17() {
    let sr = d("sf\\", Bhvadi);
    assert_has_krdanta(&[], &sr, Krt::GaY, &["sAra"]);
    assert_has_krdanta(&[], &sr, Krt::tfc, &["sartf"]);
    assert_has_krdanta(&[], &sr, Krt::Rvul, &["sAraka"]);
}

#[test]
fn sutra_3_3_18() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::GaY, &["pAka"]);
    assert_has_krdanta(&[], &d("tya\\ja~", Bhvadi), Krt::GaY, &["tyAga"]);
    assert_has_krdanta(&[], &d("ra\\nja~^", Divadi), Krt::GaY, &["rAga", "raNga"]);
}

#[test]
fn sutra_3_3_21() {
    assert_has_bhave_krdanta(&["aDi"], &d("i\\N", Adadi), Krt::GaY, &["aDyAya"]);
    assert_has_bhave_krdanta(&["upa", "aDi"], &d("i\\N", Adadi), Krt::GaY, &["upADyAya"]);
}

#[test]
fn sutra_3_3_22() {
    let ru = d("ru", Adadi);
    assert_has_bhave_krdanta(&["upa"], &ru, Krt::GaY, &["uparAva"]);
    assert_krt_blocked(&["upa"], &ru, Krt::ap);
    assert_has_bhave_krdanta(&[], &ru, Krt::ap, &["rava"]);
}

#[test]
fn sutra_3_3_23() {
    let yu = d("yu", Adadi);
    let dru = d("dru\\", Tudadi);
    let du = d("du\\", Bhvadi);
    assert_has_bhave_krdanta(&["sam"], &yu, Krt::GaY, &["saMyAva"]);
    assert_has_bhave_krdanta(&["sam"], &dru, Krt::GaY, &["sandrAva"]);
    assert_has_bhave_krdanta(&["sam"], &du, Krt::GaY, &["sandAva"]);
    // sami?
    assert_krt_blocked(&["pra"], &yu, Krt::GaY);
    assert_has_bhave_krdanta(&["pra"], &yu, Krt::ap, &["prayava"]);
}

#[test]
fn sutra_3_3_24() {
    let shri = d("SriY", Bhvadi);
    let ni = d("RI\\Y", Bhvadi);
    let bhu = d("BU", Bhvadi);
    assert_has_bhave_krdanta(&[], &shri, Krt::GaY, &["SrAya"]);
    assert_has_bhave_krdanta(&[], &ni, Krt::GaY, &["nAya"]);
    assert_has_bhave_krdanta(&[], &bhu, Krt::GaY, &["BAva"]);
    // anupasarge?
    assert_has_bhave_krdanta(&["pra"], &shri, Krt::ac, &["praSraya"]);
    assert_has_bhave_krdanta(&["pra"], &ni, Krt::ac, &["praRaya"]);
    assert_has_bhave_krdanta(&["pra"], &bhu, Krt::ac, &["praBava"]);
}

#[test]
fn sutra_3_3_25() {
    let kshu = d("wukzu", Adadi);
    let shru = d("Sru\\", Bhvadi);
    assert_has_bhave_krdanta(&["vi"], &kshu, Krt::GaY, &["vikzAva"]);
    assert_has_bhave_krdanta(&["vi"], &shru, Krt::GaY, &["viSrAva"]);
    // vau?
    assert_has_bhave_krdanta(&[], &kshu, Krt::ac, &["kzava"]);
    assert_has_bhave_krdanta(&[], &shru, Krt::ac, &["Srava"]);
}

#[test]
fn sutra_3_3_26() {
    let ni = d("RI\\Y", Bhvadi);
    assert_has_bhave_krdanta(&["ava"], &ni, Krt::GaY, &["avanAya"]);
    assert_has_bhave_krdanta(&["ud"], &ni, Krt::GaY, &["unnAya"]);
}

#[test]
fn sutra_3_3_27() {
    let dru = d("dru\\", Tudadi);
    let stu = d("zwu\\Y", Adadi);
    let sru = d("sru\\", Bhvadi);
    assert_has_bhave_krdanta(&["pra"], &dru, Krt::GaY, &["pradrAva"]);
    assert_has_bhave_krdanta(&["pra"], &stu, Krt::GaY, &["prastAva"]);
    assert_has_bhave_krdanta(&["pra"], &sru, Krt::GaY, &["prasrAva"]);
    // pre?
    assert_has_bhave_krdanta(&[], &dru, Krt::ap, &["drava"]);
    assert_has_bhave_krdanta(&[], &stu, Krt::ap, &["stava"]);
    assert_has_bhave_krdanta(&[], &sru, Krt::ap, &["srava"]);
}

#[test]
fn sutra_3_3_28() {
    let pavate = d("pUN", Kryadi);
    let punati = d("pUY", Kryadi);
    let lu = d("lUY", Kryadi);
    assert_has_bhave_krdanta(&["nir"], &pavate, Krt::GaY, &["nizpAva"]);
    assert_has_bhave_krdanta(&["nir"], &punati, Krt::GaY, &["nizpAva"]);
    assert_has_bhave_krdanta(&["aBi"], &lu, Krt::GaY, &["aBilAva"]);
    // nirabhyoH?
    assert_has_bhave_krdanta(&[], &pavate, Krt::ap, &["pava"]);
    assert_has_bhave_krdanta(&[], &punati, Krt::ap, &["pava"]);
    assert_has_bhave_krdanta(&[], &lu, Krt::ap, &["lava"]);
}

#[test]
fn sutra_3_3_29() {
    let girati = d("gF", Tudadi);
    let grnati = d("gF", Kryadi);
    assert_has_bhave_krdanta(&["ni"], &girati, Krt::GaY, &["nigAra", "nigAla"]);
    assert_has_bhave_krdanta(&["ud"], &girati, Krt::GaY, &["udgAra", "udgAla"]);
    assert_has_bhave_krdanta(&["ud"], &grnati, Krt::GaY, &["udgAra"]);
    assert_has_bhave_krdanta(&["ni"], &girati, Krt::GaY, &["nigAra", "nigAla"]);
    assert_has_bhave_krdanta(&["ni"], &grnati, Krt::GaY, &["nigAra"]);
    // unnyoH?
    assert_has_bhave_krdanta(&[], &girati, Krt::ap, &["gara", "gala"]);
    assert_has_bhave_krdanta(&[], &grnati, Krt::ap, &["gara"]);
}

#[test]
fn sutra_3_3_30() {
    let kirati = d("kF", Tudadi);
    assert_has_bhave_krdanta(&["ud"], &kirati, Krt::GaY, &["utkAra"]);
    assert_has_bhave_krdanta(&["ni"], &kirati, Krt::GaY, &["nikAra"]);
    // dhAnye?
    assert_has_bhave_krdanta(&["ud"], &kirati, Krt::ap, &["utkara"]);
    assert_has_bhave_krdanta(&["ni"], &kirati, Krt::ap, &["nikara"]);
}

#[test]
fn sutra_3_3_31() {
    let stu = d("zwu\\Y", Adadi);
    assert_has_bhave_krdanta(&["sam"], &stu, Krt::GaY, &["saMstAva"]);
    // yajne?
    assert_has_bhave_krdanta(&["sam"], &stu, Krt::ap, &["saMstava"]);
}

#[test]
fn sutra_3_3_32() {
    let stf = d("stFY", Kryadi);
    assert_has_bhave_krdanta(&["pra"], &stf, Krt::GaY, &["prastAra"]);
    // a-yajne?
    assert_has_bhave_krdanta(&["pra"], &stf, Krt::ap, &["prastara"]);
}

#[test]
fn sutra_3_3_33() {
    let stf = d("stFY", Kryadi);
    assert_has_bhave_krdanta(&["vi"], &stf, Krt::GaY, &["vistAra"]);
    // a-Sabde?
    assert_has_bhave_krdanta(&["vi"], &stf, Krt::ap, &["vistara"]);
}

#[test]
fn sutra_3_3_35() {
    let grah = d("graha~^", Kryadi);
    assert_has_bhave_krdanta(&["ud"], &grah, Krt::GaY, &["udgrAha"]);
    assert_krt_blocked(&["ud"], &grah, Krt::ap);
}

#[test]
fn sutra_3_3_36() {
    let grah = d("graha~^", Kryadi);
    assert_has_bhave_krdanta(&["sam"], &grah, Krt::GaY, &["saNgrAha"]);
    // muzwO?
    assert_has_bhave_krdanta(&["sam"], &grah, Krt::ap, &["saNgraha"]);
}

#[test]
fn sutra_3_3_38() {
    let i = d("i\\R", Adadi);
    assert_has_bhave_krdanta(&["pari"], &i, Krt::GaY, &["paryAya"]);
    // an-upAtyaye
    assert_has_bhave_krdanta(&["pari"], &i, Krt::ac, &["paryaya"]);
}

#[test]
fn sutra_3_3_39() {
    let shi = d("SIN", Adadi);
    assert_has_bhave_krdanta(&["vi"], &shi, Krt::GaY, &["viSAya"]);
    assert_has_bhave_krdanta(&["upa"], &shi, Krt::GaY, &["upaSAya"]);
    // paryAye?
    assert_has_bhave_krdanta(&["vi"], &shi, Krt::ac, &["viSaya"]);
    assert_has_bhave_krdanta(&["upa"], &shi, Krt::ac, &["upaSaya"]);
}

#[test]
fn sutra_3_3_40() {
    let ci = d("ci\\Y", Svadi);
    // TODO: separate out prakAya.
    assert_has_bhave_krdanta(&["pra"], &ci, Krt::GaY, &["pracAya", "prakAya"]);
    // handAdAne?
    assert_has_bhave_krdanta(&["pra"], &ci, Krt::ac, &["pracaya"]);
}

#[test]
fn sutra_3_3_41() {
    // TODO: others
    let ci = d("ci\\Y", Svadi);
    assert_has_bhave_krdanta(&[], &ci, Krt::GaY, &["cAya", "kAya"]);
    assert_has_bhave_krdanta(&[], &ci, Krt::ac, &["caya"]);
}

#[test]
fn sutra_3_3_45() {
    let grah = d("graha~^", Kryadi);
    assert_has_bhave_krdanta(&["ava"], &grah, Krt::GaY, &["avagrAha"]);
    assert_has_bhave_krdanta(&["ni"], &grah, Krt::GaY, &["nigrAha"]);
    assert_has_bhave_krdanta(&["ava"], &grah, Krt::ap, &["avagraha"]);
    assert_has_bhave_krdanta(&["ni"], &grah, Krt::ap, &["nigraha"]);
}

#[test]
fn sutra_3_3_46() {
    let grah = d("graha~^", Kryadi);
    assert_has_bhave_krdanta(&["pra"], &grah, Krt::GaY, &["pragrAha"]);
    // lipsAyAm?
    assert_has_bhave_krdanta(&["pra"], &grah, Krt::ap, &["pragraha"]);
}

#[test]
fn sutra_3_3_47() {
    let grah = d("graha~^", Kryadi);
    assert_has_bhave_krdanta(&["pari"], &grah, Krt::GaY, &["parigrAha"]);
    // yajne?
    assert_has_bhave_krdanta(&["pari"], &grah, Krt::ap, &["parigraha"]);
}

#[test]
fn sutra_3_3_48() {
    let vrnite = d("vfN", Kryadi);
    let vrnoti = d("vfY", Svadi);
    assert_has_bhave_krdanta(&["ni"], &vrnite, Krt::GaY, &["nIvAra"]);
    assert_has_bhave_krdanta(&["ni"], &vrnoti, Krt::GaY, &["nIvAra"]);
    // dhAnye?
    // TODO: assert_has_bhave_krdanta(&["ni"], &vrnite, Krt::ap, &["nivara"]);
    assert_has_bhave_krdanta(&["ni"], &vrnoti, Krt::ap, &["nivara"]);
}

#[test]
fn sutra_3_3_49() {
    let shri = d("SriY", Bhvadi);
    let yu = d("yu", Adadi);
    let pu = d("pUY", Kryadi);
    let dru = d("dru\\", Tudadi);
    assert_has_bhave_krdanta(&["ud"], &shri, Krt::GaY, &["ucCrAya"]);
    assert_has_bhave_krdanta(&["ud"], &yu, Krt::GaY, &["udyAva"]);
    assert_has_bhave_krdanta(&["ud"], &pu, Krt::GaY, &["utpAva"]);
    assert_has_bhave_krdanta(&["ud"], &dru, Krt::GaY, &["uddrAva"]);
}

#[test]
fn sutra_3_3_50() {
    let ru = d("ru", Adadi);
    let plu = d("plu\\N", Bhvadi);
    assert_has_bhave_krdanta(&["AN"], &ru, Krt::GaY, &["ArAva"]);
    assert_has_bhave_krdanta(&["AN"], &ru, Krt::ap, &["Arava"]);
    assert_has_bhave_krdanta(&["AN"], &plu, Krt::GaY, &["AplAva"]);
    assert_has_bhave_krdanta(&["AN"], &plu, Krt::ap, &["Aplava"]);
}

#[test]
fn sutra_3_3_51() {
    let grah = d("graha~^", Kryadi);
    assert_has_krdanta(&["ava"], &grah, Krt::GaY, &["avagrAha"]);
    assert_has_krdanta(&["ava"], &grah, Krt::ap, &["avagraha"]);
}

#[test]
fn sutra_3_3_52_and_sutra_3_3_53() {
    let grah = d("graha~^", Kryadi);
    assert_has_krdanta(&["pra"], &grah, Krt::GaY, &["pragrAha"]);
    assert_has_krdanta(&["pra"], &grah, Krt::ap, &["pragraha"]);
}

#[test]
fn sutra_3_3_55() {
    let bhu = d("BU", Bhvadi);
    assert_has_krdanta(&["pari"], &bhu, Krt::GaY, &["pariBAva"]);
    assert_has_krdanta(&["pari"], &bhu, Krt::ap, &["pariBava"]);
}

#[test]
fn sutra_3_3_56() {
    assert_has_bhave_krdanta(&[], &d("ci\\Y", Svadi), Krt::ac, &["caya"]);
    assert_has_bhave_krdanta(&[], &d("i\\R", Adadi), Krt::ac, &["aya"]);
    assert_has_bhave_krdanta(&[], &d("ji\\", Bhvadi), Krt::ac, &["jaya"]);
    assert_has_bhave_krdanta(&[], &d("kzi\\", Svadi), Krt::ac, &["kzaya"]);
}

#[test]
fn sutra_3_3_57() {
    assert_has_bhave_krdanta(&[], &d("kF", Tudadi), Krt::ap, &["kara"]);
    assert_has_bhave_krdanta(&[], &d("gF", Tudadi), Krt::ap, &["gara", "gala"]);
    assert_has_bhave_krdanta(&[], &d("SF", Kryadi), Krt::ap, &["Sara"]);
    assert_has_bhave_krdanta(&[], &d("yu", Adadi), Krt::ap, &["yava"]);
    assert_has_bhave_krdanta(&[], &d("lUY", Kryadi), Krt::ap, &["lava"]);
    assert_has_bhave_krdanta(&[], &d("zwu\\Y", Adadi), Krt::ap, &["stava"]);
    assert_has_bhave_krdanta(&[], &d("pUY", Kryadi), Krt::ap, &["pava"]);
}

#[test]
fn sutra_3_3_58() {
    assert_has_bhave_krdanta(&[], &d("df", Svadi), Krt::ap, &["dara"]);
    assert_has_bhave_krdanta(&[], &d("graha~^", Kryadi), Krt::ap, &["graha"]);
    assert_has_bhave_krdanta(&[], &d("vfY", Svadi), Krt::ap, &["vara"]);
    assert_has_bhave_krdanta(&["nir"], &d("ci\\Y", Svadi), Krt::ap, &["niScaya"]);
    assert_has_bhave_krdanta(&[], &d("ga\\mx~", Bhvadi), Krt::ap, &["gama"]);
}

#[test]
fn sutra_3_3_59() {
    let ad = d("a\\da~", Adadi);
    assert_has_bhave_krdanta(&["pra"], &ad, Krt::ap, &["praGasa"]);
    assert_has_bhave_krdanta(&["vi"], &ad, Krt::ap, &["viGasa"]);
    // upasarge?
    assert_has_bhave_krdanta(&[], &ad, Krt::GaY, &["GAsa"]);
}

#[test]
fn sutra_3_3_60() {
    let ad = d("a\\da~", Adadi);
    assert_has_bhave_krdanta(&["ni"], &ad, Krt::ap, &["niGasa"]);
    assert_has_bhave_krdanta(&["ni"], &ad, Krt::aR, &["nyAda"]);
}

#[test]
fn sutra_3_3_61() {
    let vyadh = d("vya\\Da~", Divadi);
    let jap = d("japa~", Bhvadi);
    assert_has_bhave_krdanta(&[], &vyadh, Krt::ap, &["vyaDa"]);
    assert_has_bhave_krdanta(&[], &jap, Krt::ap, &["japa"]);
    assert_has_bhave_krdanta(&[], &vyadh, Krt::GaY, &[]);
    assert_has_bhave_krdanta(&[], &jap, Krt::GaY, &[]);
    // an-upasarge?
    assert_has_bhave_krdanta(&["AN"], &vyadh, Krt::GaY, &["AvyADa"]);
    assert_has_bhave_krdanta(&["upa"], &jap, Krt::GaY, &["upajApa"]);
}

#[test]
fn sutra_3_3_62() {
    let svan = d("svana~", Bhvadi);
    let has = d("hase~", Bhvadi);
    assert_has_bhave_krdanta(&[], &svan, Krt::ap, &["svana"]);
    assert_has_bhave_krdanta(&[], &svan, Krt::GaY, &["svAna"]);
    assert_has_bhave_krdanta(&[], &has, Krt::ap, &["hasa"]);
    assert_has_bhave_krdanta(&[], &has, Krt::GaY, &["hAsa"]);
    // an-upasarge?
    assert_has_bhave_krdanta(&["pra"], &svan, Krt::GaY, &["prasvAna"]);
    assert_has_bhave_krdanta(&["pra"], &has, Krt::GaY, &["prahAsa"]);
    assert_has_bhave_krdanta(&["pra"], &svan, Krt::ap, &[]);
    assert_has_bhave_krdanta(&["pra"], &has, Krt::ap, &[]);
}

#[test]
fn sutra_3_3_63() {
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_bhave_krdanta(&["sam"], &yam, Krt::ap, &["saMyama"]);
    assert_has_bhave_krdanta(&["sam"], &yam, Krt::GaY, &["saMyAma"]);
    assert_has_bhave_krdanta(&["upa"], &yam, Krt::ap, &["upayama"]);
    assert_has_bhave_krdanta(&["upa"], &yam, Krt::GaY, &["upayAma"]);
    assert_has_bhave_krdanta(&["ni"], &yam, Krt::ap, &["niyama"]);
    assert_has_bhave_krdanta(&["ni"], &yam, Krt::GaY, &["niyAma"]);
    assert_has_bhave_krdanta(&["vi"], &yam, Krt::ap, &["viyama"]);
    assert_has_bhave_krdanta(&["vi"], &yam, Krt::GaY, &["viyAma"]);
}

#[test]
fn sutra_3_3_64() {
    let gad = d("gada~", Bhvadi);
    let nad = d("Rada~", Bhvadi);
    let path = d("paWa~", Bhvadi);
    let svan = d("svana~", Bhvadi);
    assert_has_bhave_krdanta(&["ni"], &gad, Krt::ap, &["nigada"]);
    assert_has_bhave_krdanta(&["ni"], &gad, Krt::GaY, &["nigAda"]);
    assert_has_bhave_krdanta(&["ni"], &nad, Krt::ap, &["ninada"]);
    assert_has_bhave_krdanta(&["ni"], &nad, Krt::GaY, &["ninAda"]);
    assert_has_bhave_krdanta(&["ni"], &path, Krt::ap, &["nipaWa"]);
    assert_has_bhave_krdanta(&["ni"], &path, Krt::GaY, &["nipAWa"]);
    assert_has_bhave_krdanta(&["ni"], &svan, Krt::ap, &["nisvana"]);
    assert_has_bhave_krdanta(&["ni"], &svan, Krt::GaY, &["nisvAna"]);
}

#[test]
fn sutra_3_3_65() {
    let kvan = d("kvaRa~", Bhvadi);
    assert_has_bhave_krdanta(&["ni"], &kvan, Krt::ap, &["nikvaRa"]);
    assert_has_bhave_krdanta(&["ni"], &kvan, Krt::GaY, &["nikvARa"]);
    assert_has_bhave_krdanta(&[], &kvan, Krt::ap, &["kvaRa"]);
    assert_has_bhave_krdanta(&[], &kvan, Krt::GaY, &["kvARa"]);
    // etezu?
    assert_has_bhave_krdanta(&["ati"], &kvan, Krt::GaY, &["atikvARa"]);
}

#[test]
fn sutra_3_3_66() {
    let pan = d("paRa~\\", Bhvadi);
    assert_has_bhave_krdanta(&[], &pan, Krt::ap, &["paRa", "paRAya"]);
    // parimARe?
    assert_has_bhave_krdanta(&[], &pan, Krt::GaY, &["pARa", "paRAya"]);
}

#[test]
fn sutra_3_3_67() {
    let mad = d("madI~", Divadi);
    assert_has_bhave_krdanta(&[], &mad, Krt::ap, &["mada"]);
    assert_has_bhave_krdanta(&[], &mad, Krt::GaY, &[]);
    // anupasarge?
    assert_has_bhave_krdanta(&["ud"], &mad, Krt::GaY, &["unmAda"]);
    assert_has_bhave_krdanta(&["pra"], &mad, Krt::GaY, &["pramAda"]);
}

#[test]
fn sutra_3_3_69() {
    let aj = d("aja~", Divadi);
    assert_has_krdanta(&["sam"], &aj, Krt::ap, &["samaja"]);
    assert_has_krdanta(&["ud"], &aj, Krt::ap, &["udaja"]);
    // paSuzu?
    assert_has_bhave_krdanta(&["sam"], &aj, Krt::GaY, &["samAja"]);
    assert_has_bhave_krdanta(&["ud"], &aj, Krt::GaY, &["udAja"]);
}

#[test]
fn sutra_3_3_71() {
    let sf = d("sf\\", Bhvadi);
    assert_has_krdanta(&["upa"], &sf, Krt::ap, &["upasara"]);
}

#[test]
fn sutra_3_3_72() {
    let hve = d("hve\\Y", Bhvadi);
    assert_has_bhave_krdanta(&["ni"], &hve, Krt::ap, &["nihava"]);
    assert_has_bhave_krdanta(&["aBi"], &hve, Krt::ap, &["aBihava"]);
    assert_has_bhave_krdanta(&["upa"], &hve, Krt::ap, &["upahava"]);
    assert_has_bhave_krdanta(&["vi"], &hve, Krt::ap, &["vihava"]);
    // etezu?
    assert_has_bhave_krdanta(&["pra"], &hve, Krt::GaY, &["prahvAya"]);
    assert_has_bhave_krdanta(&["pra"], &hve, Krt::ap, &[]);
}

#[test]
fn sutra_3_3_73() {
    let hve = d("hve\\Y", Bhvadi);
    assert_has_krdanta(&["AN"], &hve, Krt::ap, &["Ahava"]);
    assert_has_krdanta(&["AN"], &hve, Krt::GaY, &["AhvAya"]);
}

#[test]
fn sutra_3_3_75() {
    let hve = d("hve\\Y", Bhvadi);
    assert_has_bhave_krdanta(&[], &hve, Krt::ap, &["hava"]);
    assert_has_bhave_krdanta(&["AN"], &hve, Krt::GaY, &["AhvAya"]);
}

#[test]
fn sutra_3_3_76() {
    let han = d("ha\\na~", Adadi);
    assert_has_bhave_krdanta(&[], &han, Krt::ap, &["vaDa"]);
    assert_has_krdanta(&[], &han, Krt::GaY, &["GAta"]);
    // anupasargasya?
    assert_has_bhave_krdanta(&["pra"], &han, Krt::GaY, &["praGAta"]);
    assert_has_bhave_krdanta(&["vi"], &han, Krt::GaY, &["viGAta"]);
}

#[test]
fn sutra_3_3_77() {
    let han = d("ha\\na~", Adadi);
    assert_has_artha_krdanta(&[], &han, Murti, Krt::ap, &["Gana"]);
}

#[ignore]
#[test]
fn sutra_3_3_78() {
    let han = d("ha\\na~", Adadi);
    assert_has_artha_krdanta(&["antar"], &han, Desha, Krt::ap, &["antarGana"]);
    // deSe?
    assert_has_krdanta(&["antar"], &han, Krt::GaY, &["antarGAta"]);
}

#[test]
fn sutra_3_3_88() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::ktri, &["paktrima"]);
    assert_has_krdanta(&[], &d("quva\\pa~^", Bhvadi), Krt::ktri, &["uptrima"]);
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::ktri, &["kftrima"]);
}

#[test]
fn sutra_3_3_89() {
    assert_has_krdanta(&[], &d("wuvepf~\\", Bhvadi), Krt::aTuc, &["vepaTu"]);
    assert_has_krdanta(&[], &d("wuo~Svi", Bhvadi), Krt::aTuc, &["SvayaTu"]);
    assert_has_krdanta(&[], &d("wukzu", Adadi), Krt::aTuc, &["kzavaTu"]);
}

#[test]
fn sutra_3_3_90() {
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::naN, &["yajYa"]);
    assert_has_krdanta(&[], &d("quyAcf~^", Bhvadi), Krt::naN, &["yAcYA"]);
    assert_has_krdanta(&[], &d("yatI~\\", Bhvadi), Krt::naN, &["yatna"]);
    assert_has_krdanta(&[], &d("viCa~", Tudadi), Krt::naN, &["viSna"]);
    assert_has_krdanta(&[], &d("pra\\Ca~", Tudadi), Krt::naN, &["praSna"]);
    assert_has_krdanta(&[], &d("rakza~", Bhvadi), Krt::naN, &["rakzRa"]);
}

#[test]
fn sutra_3_3_91() {
    assert_has_krdanta(&[], &d("Yizva\\pa~", Adadi), Krt::nan, &["svapna"]);
}

#[test]
fn sutra_3_3_94() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::ktin, &["kfti"]);
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::ktin, &["citi"]);
    assert_has_krdanta(&[], &d("ma\\na~\\", Divadi), Krt::ktin, &["mati"]);
    // TODO: others
}

#[test]
fn sutra_3_3_102() {
    assert_has_krdanta(&[], &san(&d("qukf\\Y", Tanadi)), Krt::a, &["cikIrzA"]);
    assert_has_krdanta(&[], &san(&d("hf\\Y", Bhvadi)), Krt::a, &["jihIrzA"]);
    // TODO: others
}

#[test]
fn sutra_3_3_103() {
    assert_has_krdanta(&[], &d("kuqi~\\", Bhvadi), Krt::a, &["kuRqA"]);
    assert_has_krdanta(&[], &d("huqi~\\", Bhvadi), Krt::a, &["huRqA"]);
    assert_has_krdanta(&[], &d("Iha~\\", Bhvadi), Krt::a, &["IhA"]);
    assert_has_krdanta(&[], &d("Uha~\\", Bhvadi), Krt::a, &["UhA"]);
    // guroH
    assert_has_krdanta(&[], &d("Ba\\ja~^", Bhvadi), Krt::a, &[]);
    // halaH
    assert_has_krdanta(&[], &d("RI\\Y", Bhvadi), Krt::a, &[]);
}

#[ignore]
#[test]
fn sutra_3_3_107() {
    assert_has_krdanta(&[], &nic(&d("qukf\\Y", Tanadi)), Krt::yuc, &["kAraRA"]);
    assert_has_krdanta(&[], &nic(&d("hf\\Y", Bhvadi)), Krt::yuc, &["hArRA"]);
    assert_has_krdanta(&[], &nic(&d("Asa~\\", Adadi)), Krt::yuc, &["AsanA"]);
    assert_has_krdanta(&[], &nic(&d("SranTa~", Kryadi)), Krt::yuc, &["SranTanA"]);
}

#[test]
fn sutra_3_3_115() {
    assert_has_krdanta(&[], &d("hase~", Bhvadi), Krt::lyuw, &["hasana"]);
    assert_has_krdanta(&[], &d("SuBa~\\", Bhvadi), Krt::lyuw, &["SoBana"]);
    assert_has_krdanta(&[], &d("jalpa~", Bhvadi), Krt::lyuw, &["jalpana"]);
    assert_has_krdanta(&[], &d("SIN", Adadi), Krt::lyuw, &["Sayana"]);
    assert_has_krdanta(&[], &d("Asa~\\", Adadi), Krt::lyuw, &["Asana"]);
}

#[test]
fn sutra_3_3_120() {
    assert_has_krdanta(&["ava"], &d("tF", Bhvadi), Krt::GaY, &["avatAra"]);
    assert_has_krdanta(&["ava"], &d("stFY", Kryadi), Krt::GaY, &["avastAra"]);
}
