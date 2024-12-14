extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as K;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::KrtArtha::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::*;

fn assert_has_bhave_krdanta(upapadas: &[&str], dhatu: &Dhatu, krt: BaseKrt, expected: &[&str]) {
    assert_has_artha_krdanta(upapadas, dhatu, KrtArtha::Bhava, krt, expected);
}

fn assert_krt_blocked(upapadas: &[&str], dhatu: &Dhatu, krt: BaseKrt) {
    assert_has_krdanta(upapadas, dhatu, krt, &[]);
}

#[test]
fn sutra_3_3_1() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Unadi::uR, &["kAru"]);
    assert_has_krdanta(&[], &d("pA\\", Bhvadi), Unadi::uR, &["pAyu"]);
    assert_has_krdanta(&[], &d("vA\\", Adadi), Unadi::uR, &["vAyu"]);
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Unadi::uR, &["jAyu"]);
    assert_has_krdanta(&[], &d("qumi\\Y", Svadi), Unadi::uR, &["mAyu"]);
    assert_has_krdanta(&[], &d("zvada~\\", Bhvadi), Unadi::uR, &["svAdu"]);
    assert_has_krdanta(&[], &d("sA\\Da~", Svadi), Unadi::uR, &["sADu"]);
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), Unadi::uR, &["ASu"]);

    // For more specific uNAdi tests, see `kaumudi_67.rs`
}

#[test]
fn sutra_3_3_2() {
    assert_has_krdanta(&[], &d("vftu~\\", Bhvadi), Unadi::manin, &["vartman"]);
    assert_has_krdanta(&[], &d("cara~", Bhvadi), Unadi::manin, &["carman"]);
}

#[test]
fn sutra_3_3_3() {
    use Unadi::ini;
    use K::GinuR;
    use K::Rini;
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
    assert_has_krdanta(&[], &d("Bu\\ja~", Rudhadi), K::tumun, &["Boktum"]);
    assert_has_krdanta(&[], &d("Bu\\ja~", Rudhadi), K::Rvul, &["Bojaka"]);
}

#[test]
fn sutra_3_3_13() {
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lrt, &["karizyati"]);
    assert_has_tip(&[], &d("hf\\Y", Bhvadi), Lrt, &["harizyati"]);
}

#[test]
fn sutra_3_3_14() {
    let kr = d("qukf\\Y", Tanadi);

    let create_lrt_sat = |text, dhatu: &Dhatu, krt: BaseKrt| {
        Krdanta::builder()
            .lakara(Lrt)
            .dhatu(dhatu.clone())
            .krt(krt)
            .require(text)
            .build()
            .unwrap()
    };

    let karishyat = create_lrt_sat("karizyat", &kr, K::Satf);
    let karishyamana = create_lrt_sat("karizyamARa", &kr, K::SAnac);
    assert_has_sup_2s(&karishyat, Pum, &["karizyantam"]);
    assert_has_sup_2s(&karishyamana, Pum, &["karizyamARam"]);
    assert_has_sup_ss(&karishyat, Pum, &["karizyan"]);
    assert_has_sup_ss(&karishyamana, Pum, &["karizyamARa"]);
    assert_has_sup_1s(&karishyat, Pum, &["karizyan"]);
    assert_has_sup_1s(&karishyamana, Pum, &["karizyamARaH"]);

    assert_has_tip(&[], &kr, Lrt, &["karizyati"]);
    assert_has_ta(&[], &kr, Lrt, &["karizyate"]);
}

#[test]
fn sutra_3_3_15() {
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lut, &["kartA"]);
    assert_has_tip(&[], &d("Bu\\ja~", Rudhadi), Lut, &["BoktA"]);
}

#[test]
fn sutra_3_3_16() {
    assert_has_krdanta(&[], &d("pa\\da~\\", Divadi), K::GaY, &["pAda"]);
    assert_has_krdanta(&[], &d("ru\\jo~", Tudadi), K::GaY, &["roga"]);
    assert_has_krdanta(&[], &d("vi\\Sa~", Tudadi), K::GaY, &["veSa"]);
    assert_has_krdanta(&[], &d("spf\\Sa~", Tudadi), K::GaY, &["sparSa"]);
}

#[test]
fn sutra_3_3_17() {
    let sr = d("sf\\", Bhvadi);
    assert_has_krdanta(&[], &sr, K::GaY, &["sAra"]);
    assert_has_krdanta(&[], &sr, K::tfc, &["sartf"]);
    assert_has_krdanta(&[], &sr, K::Rvul, &["sAraka"]);
}

#[test]
fn sutra_3_3_18() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), K::GaY, &["pAka"]);
    assert_has_krdanta(&[], &d("tya\\ja~", Bhvadi), K::GaY, &["tyAga"]);
    assert_has_krdanta(&[], &d("ra\\nja~^", Divadi), K::GaY, &["rAga", "raNga"]);
}

#[test]
fn sutra_3_3_21() {
    assert_has_bhave_krdanta(&["aDi"], &d("i\\N", Adadi), K::GaY, &["aDyAya"]);
    assert_has_bhave_krdanta(&["upa", "aDi"], &d("i\\N", Adadi), K::GaY, &["upADyAya"]);
}

#[test]
fn sutra_3_3_22() {
    let ru = d("ru", Adadi);
    assert_has_bhave_krdanta(&["upa"], &ru, K::GaY, &["uparAva"]);
    assert_krt_blocked(&["upa"], &ru, K::ap);
    assert_has_bhave_krdanta(&[], &ru, K::ap, &["rava"]);
}

#[test]
fn sutra_3_3_23() {
    let yu = d("yu", Adadi);
    let dru = d("dru\\", Tudadi);
    let du = d("du\\", Bhvadi);
    assert_has_bhave_krdanta(&["sam"], &yu, K::GaY, &["saMyAva"]);
    assert_has_bhave_krdanta(&["sam"], &dru, K::GaY, &["sandrAva"]);
    assert_has_bhave_krdanta(&["sam"], &du, K::GaY, &["sandAva"]);
    // sami?
    assert_krt_blocked(&["pra"], &yu, K::GaY);
    assert_has_bhave_krdanta(&["pra"], &yu, K::ap, &["prayava"]);
}

#[test]
fn sutra_3_3_24() {
    let shri = d("SriY", Bhvadi);
    let ni = d("RI\\Y", Bhvadi);
    let bhu = d("BU", Bhvadi);
    assert_has_bhave_krdanta(&[], &shri, K::GaY, &["SrAya"]);
    assert_has_bhave_krdanta(&[], &ni, K::GaY, &["nAya"]);
    assert_has_bhave_krdanta(&[], &bhu, K::GaY, &["BAva"]);
    // anupasarge?
    assert_has_bhave_krdanta(&["pra"], &shri, K::ac, &["praSraya"]);
    assert_has_bhave_krdanta(&["pra"], &ni, K::ac, &["praRaya"]);
    assert_has_bhave_krdanta(&["pra"], &bhu, K::ac, &["praBava"]);
}

#[test]
fn sutra_3_3_25() {
    let kshu = d("wukzu", Adadi);
    let shru = d("Sru\\", Bhvadi);
    assert_has_bhave_krdanta(&["vi"], &kshu, K::GaY, &["vikzAva"]);
    assert_has_bhave_krdanta(&["vi"], &shru, K::GaY, &["viSrAva"]);
    // vau?
    assert_has_bhave_krdanta(&[], &kshu, K::ac, &["kzava"]);
    assert_has_bhave_krdanta(&[], &shru, K::ac, &["Srava"]);
}

#[test]
fn sutra_3_3_26() {
    let ni = d("RI\\Y", Bhvadi);
    assert_has_bhave_krdanta(&["ava"], &ni, K::GaY, &["avanAya"]);
    assert_has_bhave_krdanta(&["ud"], &ni, K::GaY, &["unnAya"]);
}

#[test]
fn sutra_3_3_27() {
    let dru = d("dru\\", Tudadi);
    let stu = d("zwu\\Y", Adadi);
    let sru = d("sru\\", Bhvadi);
    assert_has_bhave_krdanta(&["pra"], &dru, K::GaY, &["pradrAva"]);
    assert_has_bhave_krdanta(&["pra"], &stu, K::GaY, &["prastAva"]);
    assert_has_bhave_krdanta(&["pra"], &sru, K::GaY, &["prasrAva"]);
    // pre?
    assert_has_bhave_krdanta(&[], &dru, K::ap, &["drava"]);
    assert_has_bhave_krdanta(&[], &stu, K::ap, &["stava"]);
    assert_has_bhave_krdanta(&[], &sru, K::ap, &["srava"]);
}

#[test]
fn sutra_3_3_28() {
    let pavate = d("pUN", Kryadi);
    let punati = d("pUY", Kryadi);
    let lu = d("lUY", Kryadi);
    assert_has_bhave_krdanta(&["nir"], &pavate, K::GaY, &["nizpAva"]);
    assert_has_bhave_krdanta(&["nir"], &punati, K::GaY, &["nizpAva"]);
    assert_has_bhave_krdanta(&["aBi"], &lu, K::GaY, &["aBilAva"]);
    // nirabhyoH?
    assert_has_bhave_krdanta(&[], &pavate, K::ap, &["pava"]);
    assert_has_bhave_krdanta(&[], &punati, K::ap, &["pava"]);
    assert_has_bhave_krdanta(&[], &lu, K::ap, &["lava"]);
}

#[test]
fn sutra_3_3_29() {
    let girati = d("gF", Tudadi);
    let grnati = d("gF", Kryadi);
    assert_has_bhave_krdanta(&["ni"], &girati, K::GaY, &["nigAra", "nigAla"]);
    assert_has_bhave_krdanta(&["ud"], &girati, K::GaY, &["udgAra", "udgAla"]);
    assert_has_bhave_krdanta(&["ud"], &grnati, K::GaY, &["udgAra"]);
    assert_has_bhave_krdanta(&["ni"], &girati, K::GaY, &["nigAra", "nigAla"]);
    assert_has_bhave_krdanta(&["ni"], &grnati, K::GaY, &["nigAra"]);
    // unnyoH?
    assert_has_bhave_krdanta(&[], &girati, K::ap, &["gara", "gala"]);
    assert_has_bhave_krdanta(&[], &grnati, K::ap, &["gara"]);
}

#[test]
fn sutra_3_3_30() {
    let kirati = d("kF", Tudadi);
    assert_has_bhave_krdanta(&["ud"], &kirati, K::GaY, &["utkAra"]);
    assert_has_bhave_krdanta(&["ni"], &kirati, K::GaY, &["nikAra"]);
    // dhAnye?
    assert_has_bhave_krdanta(&["ud"], &kirati, K::ap, &["utkara"]);
    assert_has_bhave_krdanta(&["ni"], &kirati, K::ap, &["nikara"]);
}

#[test]
fn sutra_3_3_31() {
    let stu = d("zwu\\Y", Adadi);
    assert_has_bhave_krdanta(&["sam"], &stu, K::GaY, &["saMstAva"]);
    // yajne?
    assert_has_bhave_krdanta(&["sam"], &stu, K::ap, &["saMstava"]);
}

#[test]
fn sutra_3_3_32() {
    let stf = d("stFY", Kryadi);
    assert_has_bhave_krdanta(&["pra"], &stf, K::GaY, &["prastAra"]);
    // a-yajne?
    assert_has_bhave_krdanta(&["pra"], &stf, K::ap, &["prastara"]);
}

#[test]
fn sutra_3_3_33() {
    let stf = d("stFY", Kryadi);
    assert_has_bhave_krdanta(&["vi"], &stf, K::GaY, &["vistAra"]);
    // a-Sabde?
    assert_has_bhave_krdanta(&["vi"], &stf, K::ap, &["vistara"]);
}

#[test]
fn sutra_3_3_35() {
    let grah = d("graha~^", Kryadi);
    assert_has_bhave_krdanta(&["ud"], &grah, K::GaY, &["udgrAha"]);
    assert_krt_blocked(&["ud"], &grah, K::ap);
}

#[test]
fn sutra_3_3_36() {
    let grah = d("graha~^", Kryadi);
    assert_has_bhave_krdanta(&["sam"], &grah, K::GaY, &["saNgrAha"]);
    // muzwO?
    assert_has_bhave_krdanta(&["sam"], &grah, K::ap, &["saNgraha"]);
}

#[test]
fn sutra_3_3_38() {
    let i = d("i\\R", Adadi);
    assert_has_bhave_krdanta(&["pari"], &i, K::GaY, &["paryAya"]);
    // an-upAtyaye
    assert_has_bhave_krdanta(&["pari"], &i, K::ac, &["paryaya"]);
}

#[test]
fn sutra_3_3_39() {
    let shi = d("SIN", Adadi);
    assert_has_bhave_krdanta(&["vi"], &shi, K::GaY, &["viSAya"]);
    assert_has_bhave_krdanta(&["upa"], &shi, K::GaY, &["upaSAya"]);
    // paryAye?
    assert_has_bhave_krdanta(&["vi"], &shi, K::ac, &["viSaya"]);
    assert_has_bhave_krdanta(&["upa"], &shi, K::ac, &["upaSaya"]);
}

#[test]
fn sutra_3_3_40() {
    let ci = d("ci\\Y", Svadi);
    // TODO: separate out prakAya.
    assert_has_bhave_krdanta(&["pra"], &ci, K::GaY, &["pracAya", "prakAya"]);
    // handAdAne?
    assert_has_bhave_krdanta(&["pra"], &ci, K::ac, &["pracaya"]);
}

#[test]
fn sutra_3_3_41() {
    // TODO: others
    let ci = d("ci\\Y", Svadi);
    assert_has_bhave_krdanta(&[], &ci, K::GaY, &["cAya", "kAya"]);
    assert_has_bhave_krdanta(&[], &ci, K::ac, &["caya"]);
}

#[test]
fn sutra_3_3_45() {
    let grah = d("graha~^", Kryadi);
    assert_has_bhave_krdanta(&["ava"], &grah, K::GaY, &["avagrAha"]);
    assert_has_bhave_krdanta(&["ni"], &grah, K::GaY, &["nigrAha"]);
    assert_has_bhave_krdanta(&["ava"], &grah, K::ap, &["avagraha"]);
    assert_has_bhave_krdanta(&["ni"], &grah, K::ap, &["nigraha"]);
}

#[test]
fn sutra_3_3_46() {
    let grah = d("graha~^", Kryadi);
    assert_has_bhave_krdanta(&["pra"], &grah, K::GaY, &["pragrAha"]);
    // lipsAyAm?
    assert_has_bhave_krdanta(&["pra"], &grah, K::ap, &["pragraha"]);
}

#[test]
fn sutra_3_3_47() {
    let grah = d("graha~^", Kryadi);
    assert_has_bhave_krdanta(&["pari"], &grah, K::GaY, &["parigrAha"]);
    // yajne?
    assert_has_bhave_krdanta(&["pari"], &grah, K::ap, &["parigraha"]);
}

#[test]
fn sutra_3_3_48() {
    let vrnite = d("vfN", Kryadi);
    let vrnoti = d("vfY", Svadi);
    assert_has_bhave_krdanta(&["ni"], &vrnite, K::GaY, &["nIvAra"]);
    assert_has_bhave_krdanta(&["ni"], &vrnoti, K::GaY, &["nIvAra"]);
    // dhAnye?
    // TODO: assert_has_bhave_krdanta(&["ni"], &vrnite, K::ap, &["nivara"]);
    assert_has_bhave_krdanta(&["ni"], &vrnoti, K::ap, &["nivara"]);
}

#[test]
fn sutra_3_3_49() {
    let shri = d("SriY", Bhvadi);
    let yu = d("yu", Adadi);
    let pu = d("pUY", Kryadi);
    let dru = d("dru\\", Tudadi);
    assert_has_bhave_krdanta(&["ud"], &shri, K::GaY, &["ucCrAya"]);
    assert_has_bhave_krdanta(&["ud"], &yu, K::GaY, &["udyAva"]);
    assert_has_bhave_krdanta(&["ud"], &pu, K::GaY, &["utpAva"]);
    assert_has_bhave_krdanta(&["ud"], &dru, K::GaY, &["uddrAva"]);
}

#[test]
fn sutra_3_3_50() {
    let ru = d("ru", Adadi);
    let plu = d("plu\\N", Bhvadi);
    assert_has_bhave_krdanta(&["AN"], &ru, K::GaY, &["ArAva"]);
    assert_has_bhave_krdanta(&["AN"], &ru, K::ap, &["Arava"]);
    assert_has_bhave_krdanta(&["AN"], &plu, K::GaY, &["AplAva"]);
    assert_has_bhave_krdanta(&["AN"], &plu, K::ap, &["Aplava"]);
}

#[test]
fn sutra_3_3_51() {
    let grah = d("graha~^", Kryadi);
    assert_has_krdanta(&["ava"], &grah, K::GaY, &["avagrAha"]);
    assert_has_krdanta(&["ava"], &grah, K::ap, &["avagraha"]);
}

#[test]
fn sutra_3_3_52_and_sutra_3_3_53() {
    let grah = d("graha~^", Kryadi);
    assert_has_krdanta(&["pra"], &grah, K::GaY, &["pragrAha"]);
    assert_has_krdanta(&["pra"], &grah, K::ap, &["pragraha"]);
}

#[test]
fn sutra_3_3_55() {
    let bhu = d("BU", Bhvadi);
    assert_has_krdanta(&["pari"], &bhu, K::GaY, &["pariBAva"]);
    assert_has_krdanta(&["pari"], &bhu, K::ap, &["pariBava"]);
}

#[test]
fn sutra_3_3_56() {
    assert_has_bhave_krdanta(&[], &d("ci\\Y", Svadi), K::ac, &["caya"]);
    assert_has_bhave_krdanta(&[], &d("i\\R", Adadi), K::ac, &["aya"]);
    assert_has_bhave_krdanta(&[], &d("ji\\", Bhvadi), K::ac, &["jaya"]);
    assert_has_bhave_krdanta(&[], &d("kzi\\", Svadi), K::ac, &["kzaya"]);
}

#[test]
fn sutra_3_3_57() {
    assert_has_bhave_krdanta(&[], &d("kF", Tudadi), K::ap, &["kara"]);
    assert_has_bhave_krdanta(&[], &d("gF", Tudadi), K::ap, &["gara", "gala"]);
    assert_has_bhave_krdanta(&[], &d("SF", Kryadi), K::ap, &["Sara"]);
    assert_has_bhave_krdanta(&[], &d("yu", Adadi), K::ap, &["yava"]);
    assert_has_bhave_krdanta(&[], &d("lUY", Kryadi), K::ap, &["lava"]);
    assert_has_bhave_krdanta(&[], &d("zwu\\Y", Adadi), K::ap, &["stava"]);
    assert_has_bhave_krdanta(&[], &d("pUY", Kryadi), K::ap, &["pava"]);
}

#[test]
fn sutra_3_3_58() {
    assert_has_bhave_krdanta(&[], &d("df", Svadi), K::ap, &["dara"]);
    assert_has_bhave_krdanta(&[], &d("graha~^", Kryadi), K::ap, &["graha"]);
    assert_has_bhave_krdanta(&[], &d("vfY", Svadi), K::ap, &["vara"]);
    assert_has_bhave_krdanta(&["nir"], &d("ci\\Y", Svadi), K::ap, &["niScaya"]);
    assert_has_bhave_krdanta(&[], &d("ga\\mx~", Bhvadi), K::ap, &["gama"]);
}

#[test]
fn sutra_3_3_59() {
    let ad = d("a\\da~", Adadi);
    assert_has_bhave_krdanta(&["pra"], &ad, K::ap, &["praGasa"]);
    assert_has_bhave_krdanta(&["vi"], &ad, K::ap, &["viGasa"]);
    // upasarge?
    assert_has_bhave_krdanta(&[], &ad, K::GaY, &["GAsa"]);
}

#[test]
fn sutra_3_3_60() {
    let ad = d("a\\da~", Adadi);
    assert_has_bhave_krdanta(&["ni"], &ad, K::ap, &["niGasa"]);
    assert_has_bhave_krdanta(&["ni"], &ad, K::Ra, &["nyAda"]);
}

#[test]
fn sutra_3_3_61() {
    let vyadh = d("vya\\Da~", Divadi);
    let jap = d("japa~", Bhvadi);
    assert_has_bhave_krdanta(&[], &vyadh, K::ap, &["vyaDa"]);
    assert_has_bhave_krdanta(&[], &jap, K::ap, &["japa"]);
    assert_has_bhave_krdanta(&[], &vyadh, K::GaY, &[]);
    assert_has_bhave_krdanta(&[], &jap, K::GaY, &[]);
    // an-upasarge?
    assert_has_bhave_krdanta(&["AN"], &vyadh, K::GaY, &["AvyADa"]);
    assert_has_bhave_krdanta(&["upa"], &jap, K::GaY, &["upajApa"]);
}

#[test]
fn sutra_3_3_62() {
    let svan = d("svana~", Bhvadi);
    let has = d("hase~", Bhvadi);
    assert_has_bhave_krdanta(&[], &svan, K::ap, &["svana"]);
    assert_has_bhave_krdanta(&[], &svan, K::GaY, &["svAna"]);
    assert_has_bhave_krdanta(&[], &has, K::ap, &["hasa"]);
    assert_has_bhave_krdanta(&[], &has, K::GaY, &["hAsa"]);
    // an-upasarge?
    assert_has_bhave_krdanta(&["pra"], &svan, K::GaY, &["prasvAna"]);
    assert_has_bhave_krdanta(&["pra"], &has, K::GaY, &["prahAsa"]);
    assert_has_bhave_krdanta(&["pra"], &svan, K::ap, &[]);
    assert_has_bhave_krdanta(&["pra"], &has, K::ap, &[]);
}

#[test]
fn sutra_3_3_63() {
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_bhave_krdanta(&["sam"], &yam, K::ap, &["saMyama"]);
    assert_has_bhave_krdanta(&["sam"], &yam, K::GaY, &["saMyAma"]);
    assert_has_bhave_krdanta(&["upa"], &yam, K::ap, &["upayama"]);
    assert_has_bhave_krdanta(&["upa"], &yam, K::GaY, &["upayAma"]);
    assert_has_bhave_krdanta(&["ni"], &yam, K::ap, &["niyama"]);
    assert_has_bhave_krdanta(&["ni"], &yam, K::GaY, &["niyAma"]);
    assert_has_bhave_krdanta(&["vi"], &yam, K::ap, &["viyama"]);
    assert_has_bhave_krdanta(&["vi"], &yam, K::GaY, &["viyAma"]);
}

#[test]
fn sutra_3_3_64() {
    let gad = d("gada~", Bhvadi);
    let nad = d("Rada~", Bhvadi);
    let path = d("paWa~", Bhvadi);
    let svan = d("svana~", Bhvadi);
    assert_has_bhave_krdanta(&["ni"], &gad, K::ap, &["nigada"]);
    assert_has_bhave_krdanta(&["ni"], &gad, K::GaY, &["nigAda"]);
    assert_has_bhave_krdanta(&["ni"], &nad, K::ap, &["ninada"]);
    assert_has_bhave_krdanta(&["ni"], &nad, K::GaY, &["ninAda"]);
    assert_has_bhave_krdanta(&["ni"], &path, K::ap, &["nipaWa"]);
    assert_has_bhave_krdanta(&["ni"], &path, K::GaY, &["nipAWa"]);
    assert_has_bhave_krdanta(&["ni"], &svan, K::ap, &["nisvana"]);
    assert_has_bhave_krdanta(&["ni"], &svan, K::GaY, &["nisvAna"]);
}

#[test]
fn sutra_3_3_65() {
    let kvan = d("kvaRa~", Bhvadi);
    assert_has_bhave_krdanta(&["ni"], &kvan, K::ap, &["nikvaRa"]);
    assert_has_bhave_krdanta(&["ni"], &kvan, K::GaY, &["nikvARa"]);
    assert_has_bhave_krdanta(&[], &kvan, K::ap, &["kvaRa"]);
    assert_has_bhave_krdanta(&[], &kvan, K::GaY, &["kvARa"]);
    // etezu?
    assert_has_bhave_krdanta(&["ati"], &kvan, K::GaY, &["atikvARa"]);
}

#[test]
fn sutra_3_3_66() {
    let pan = d("paRa~\\", Bhvadi);
    assert_has_bhave_krdanta(&[], &pan, K::ap, &["paRa"]);
    // parimARe?
    assert_has_bhave_krdanta(&[], &pan, K::GaY, &["pARa", "paRAya"]);
}

#[test]
fn sutra_3_3_67() {
    let mad = d("madI~", Divadi);
    assert_has_bhave_krdanta(&[], &mad, K::ap, &["mada"]);
    assert_has_bhave_krdanta(&[], &mad, K::GaY, &[]);
    // anupasarge?
    assert_has_bhave_krdanta(&["ud"], &mad, K::GaY, &["unmAda"]);
    assert_has_bhave_krdanta(&["pra"], &mad, K::GaY, &["pramAda"]);
}

#[test]
fn sutra_3_3_69() {
    let aj = d("aja~", Divadi);
    assert_has_krdanta(&["sam"], &aj, K::ap, &["samaja"]);
    assert_has_krdanta(&["ud"], &aj, K::ap, &["udaja"]);
    // paSuzu?
    assert_has_bhave_krdanta(&["sam"], &aj, K::GaY, &["samAja"]);
    assert_has_bhave_krdanta(&["ud"], &aj, K::GaY, &["udAja"]);
}

#[test]
fn sutra_3_3_71() {
    let sf = d("sf\\", Bhvadi);
    assert_has_krdanta(&["upa"], &sf, K::ap, &["upasara"]);
}

#[test]
fn sutra_3_3_72() {
    let hve = d("hve\\Y", Bhvadi);
    assert_has_bhave_krdanta(&["ni"], &hve, K::ap, &["nihava"]);
    assert_has_bhave_krdanta(&["aBi"], &hve, K::ap, &["aBihava"]);
    assert_has_bhave_krdanta(&["upa"], &hve, K::ap, &["upahava"]);
    assert_has_bhave_krdanta(&["vi"], &hve, K::ap, &["vihava"]);
    // etezu?
    assert_has_bhave_krdanta(&["pra"], &hve, K::GaY, &["prahvAya"]);
    assert_has_bhave_krdanta(&["pra"], &hve, K::ap, &[]);
}

#[test]
fn sutra_3_3_73() {
    let hve = d("hve\\Y", Bhvadi);
    assert_has_krdanta(&["AN"], &hve, K::ap, &["Ahava"]);
    assert_has_krdanta(&["AN"], &hve, K::GaY, &["AhvAya"]);
}

#[test]
fn sutra_3_3_75() {
    let hve = d("hve\\Y", Bhvadi);
    assert_has_bhave_krdanta(&[], &hve, K::ap, &["hava"]);
    assert_has_bhave_krdanta(&["AN"], &hve, K::GaY, &["AhvAya"]);
}

#[test]
fn sutra_3_3_76() {
    let han = d("ha\\na~", Adadi);
    assert_has_bhave_krdanta(&[], &han, K::ap, &["vaDa"]);
    assert_has_krdanta(&[], &han, K::GaY, &["GAta"]);
    // anupasargasya?
    assert_has_bhave_krdanta(&["pra"], &han, K::GaY, &["praGAta"]);
    assert_has_bhave_krdanta(&["vi"], &han, K::GaY, &["viGAta"]);
}

#[test]
fn sutra_3_3_77() {
    let han = d("ha\\na~", Adadi);
    assert_has_artha_krdanta(&[], &han, Murti, K::ap, &["Gana"]);
}

#[ignore]
#[test]
fn sutra_3_3_78() {
    let han = d("ha\\na~", Adadi);
    assert_has_artha_krdanta(&["antar"], &han, Desha, K::ap, &["antarGana"]);
    // deSe?
    assert_has_krdanta(&["antar"], &han, K::GaY, &["antarGAta"]);
}

#[test]
fn sutra_3_3_88() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), K::ktri, &["paktrima"]);
    assert_has_krdanta(&[], &d("quva\\pa~^", Bhvadi), K::ktri, &["uptrima"]);
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), K::ktri, &["kftrima"]);
}

#[test]
fn sutra_3_3_89() {
    assert_has_krdanta(&[], &d("wuvepf~\\", Bhvadi), K::aTuc, &["vepaTu"]);
    assert_has_krdanta(&[], &d("wuo~Svi", Bhvadi), K::aTuc, &["SvayaTu"]);
    assert_has_krdanta(&[], &d("wukzu", Adadi), K::aTuc, &["kzavaTu"]);
}

#[test]
fn sutra_3_3_90() {
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), K::naN, &["yajYa"]);
    assert_has_krdanta(&[], &d("quyAcf~^", Bhvadi), K::naN, &["yAcYA"]);
    assert_has_krdanta(&[], &d("yatI~\\", Bhvadi), K::naN, &["yatna"]);
    assert_has_krdanta(&[], &d("viCa~", Tudadi), K::naN, &["viSna"]);
    assert_has_krdanta(&[], &d("pra\\Ca~", Tudadi), K::naN, &["praSna"]);
    assert_has_krdanta(&[], &d("rakza~", Bhvadi), K::naN, &["rakzRa"]);
}

#[test]
fn sutra_3_3_91() {
    assert_has_krdanta(&[], &d("Yizva\\pa~", Adadi), K::nan, &["svapna"]);
}

#[test]
fn sutra_3_3_92() {
    assert_has_krdanta(&["pra"], &d("qudA\\Y", Juhotyadi), K::ki, &["pradi"]);
    assert_has_krdanta(&["pra"], &d("quDA\\Y", Juhotyadi), K::ki, &["praDi"]);
    assert_has_krdanta(&["antar"], &d("quDA\\Y", Juhotyadi), K::ki, &["antarDi"]);
}

#[test]
fn sutra_3_3_94() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), K::ktin, &["kfti"]);
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), K::ktin, &["citi"]);
    assert_has_krdanta(&[], &d("ma\\na~\\", Divadi), K::ktin, &["mati"]);
    // TODO: others
}

#[test]
fn sutra_3_3_94_v1() {
    assert_has_krdanta(&[], &d("A\\px~", Svadi), K::ktin, &["Apti"]);
    assert_has_krdanta(&[], &d("rA\\Da~", Svadi), K::ktin, &["rAdDi"]);
    assert_has_krdanta(&[], &d("dIpI~\\", Divadi), K::ktin, &["dIpti"]);
    assert_has_krdanta(&[], &d("sransu~\\", Divadi), K::ktin, &["srasti"]);
    assert_has_krdanta(&[], &d("Dvansu~\\", Divadi), K::ktin, &["Dvasti"]);
    assert_has_krdanta(&[], &d("qula\\Ba~\\z", Bhvadi), K::ktin, &["labDi"]);
}

#[test]
fn sutra_3_3_94_v2() {
    assert_has_krdanta(&[], &d("Sru\\", Bhvadi), K::ktin, &["Sruti"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), K::ktin, &["izwi"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), K::ktin, &["stuti"]);
}

#[test]
fn sutra_3_3_94_v3() {
    assert_has_krdanta(&[], &d("glE\\", Bhvadi), K::ni, &["glAni"]);
    assert_has_krdanta(&[], &d("mlE\\", Bhvadi), K::ni, &["mlAni"]);
    assert_has_krdanta(&[], &d("jyA\\", Kryadi), K::ni, &["jyAni"]);
    assert_has_krdanta(&[], &d("o~hA\\k", Juhotyadi), K::ni, &["hAni"]);
}

#[test]
fn sutra_3_3_94_v4() {
    assert_has_krdanta(&[], &d("kF", Tudadi), K::ktin, &["kIrRi"]);
    assert_has_krdanta(&[], &d("gF", Tudadi), K::ktin, &["gIrRi"]);
    assert_has_krdanta(&[], &d("jF", Kryadi), K::ktin, &["jIrRi"]);
    assert_has_krdanta(&[], &d("SF", Kryadi), K::ktin, &["SIrRi"]);
    assert_has_krdanta(&[], &d("lUY", Kryadi), K::ktin, &["lUni"]);
    assert_has_krdanta(&[], &d("DUY", Kryadi), K::ktin, &["DUni"]);
}

#[test]
fn sutra_3_3_94_v5() {
    let pad = d("pa\\da~\\", Divadi);
    assert_has_krdanta(&["sam"], &pad, K::kvip, &["sampad"]);
    assert_has_krdanta(&["vi"], &pad, K::kvip, &["vipad"]);
    assert_has_krdanta(&["prati"], &pad, K::kvip, &["pratipad"]);

    assert_has_krdanta(&["sam"], &pad, K::ktin, &["sampatti"]);
    assert_has_krdanta(&["vi"], &pad, K::ktin, &["vipatti"]);
}

#[test]
fn sutra_3_3_96() {
    assert_has_krdanta(&["pra"], &d("zWA\\", Bhvadi), K::ktin, &["prasTiti"]);
    assert_has_krdanta(&["ud"], &d("gE\\", Bhvadi), K::ktin, &["udgIti"]);
    assert_has_krdanta(&["sam"], &d("gE\\", Bhvadi), K::ktin, &["saNgIti"]);
    assert_has_krdanta(&["pra"], &d("pA\\", Bhvadi), K::ktin, &["prapIti"]);
    assert_has_krdanta(&["sam"], &d("pA\\", Bhvadi), K::ktin, &["sampIti"]);
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), K::ktin, &["pakti"]);
}

#[test]
fn sutra_3_3_97() {
    assert_has_krdanta(&[], &d("ava~", Bhvadi), K::ktin, &["Uti"]);
    assert_has_krdanta(&[], &d("yu", Adadi), K::ktin, &["yUti"]);
    assert_has_krdanta(&[], &d("ju", Bhvadi), K::ktin, &["jUti"]);
    assert_has_krdanta(&[], &d("zo\\", Divadi), K::ktin, &["sAti"]);
    assert_has_krdanta(&[], &d("zaRa~", Bhvadi), K::ktin, &["sAti"]);
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), K::ktin, &["heti"]);
    assert_has_krdanta(&[], &d("hi\\", Svadi), K::ktin, &["heti"]);
    assert_has_krdanta(&[], &d("kFta~", Curadi), K::ktin, &["kIrti"]);
}

#[test]
fn sutra_3_3_98() {
    assert_has_krdanta(&[], &d("vraja~", Bhvadi), K::kyap, &["vrajyA"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), K::kyap, &["ijyA"]);
}

#[test]
fn sutra_3_3_99() {
    assert_has_krdanta(&["sam"], &d("aja~", Bhvadi), K::kyap, &["samajyA"]);
    assert_has_krdanta(&["ni"], &d("za\\dx~", Bhvadi), K::kyap, &["nizadyA"]);
    assert_has_krdanta(&["ni"], &d("patx~", Bhvadi), K::kyap, &["nipatyA"]);
    assert_has_krdanta(&[], &d("ma\\na~\\", Divadi), K::kyap, &["manyA"]);
    assert_has_krdanta(&[], &d("vida~", Adadi), K::kyap, &["vidyA"]);
    assert_has_krdanta(&[], &d("zu\\Y", Svadi), K::kyap, &["sutyA"]);
    assert_has_krdanta(&[], &d("SIN", Adadi), K::kyap, &["SayyA"]);
    // Bftya is from 3.1.112.
    assert_has_krdanta(&[], &d("Bf\\Y", Bhvadi), K::kyap, &["Bftya", "BftyA"]);
    // itya is from 3.1.109.
    assert_has_krdanta(&[], &d("i\\R", Adadi), K::kyap, &["itya", "ityA"]);
}

#[test]
fn sutra_3_3_100() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kr, K::Sa, &["kriyA"]);
    // kftya (pum) by 3.1.120.
    assert_has_krdanta(&[], &kr, K::kyap, &["kftya", "kftyA"]);
    assert_has_krdanta(&[], &kr, K::ktin, &["kfti"]);
}

#[test]
fn sutra_3_3_101() {
    assert_has_krdanta(&[], &d("izu~", Tudadi), K::Sa, &["icCA"]);
}

#[test]
fn sutra_3_3_102() {
    assert_has_krdanta(&[], &san(&d("qukf\\Y", Tanadi)), K::a, &["cikIrzA"]);
    assert_has_krdanta(&[], &san(&d("hf\\Y", Bhvadi)), K::a, &["jihIrzA"]);
    // TODO: others
}

#[test]
fn sutra_3_3_103() {
    assert_has_krdanta(&[], &d("kuqi~\\", Bhvadi), K::a, &["kuRqA"]);
    assert_has_krdanta(&[], &d("huqi~\\", Bhvadi), K::a, &["huRqA"]);
    assert_has_krdanta(&[], &d("Iha~\\", Bhvadi), K::a, &["IhA"]);
    assert_has_krdanta(&[], &d("Uha~\\", Bhvadi), K::a, &["UhA"]);
    // guroH
    assert_has_krdanta(&[], &d("Ba\\ja~^", Bhvadi), K::a, &[]);
    // halaH
    assert_has_krdanta(&[], &d("RI\\Y", Bhvadi), K::a, &[]);
}

#[test]
fn sutra_3_3_106() {
    let da = &d("qudA\\Y", Juhotyadi);
    let dha = &d("quDA\\Y", Juhotyadi);
    assert_has_krdanta(&["pra"], &da, K::aN, &["pradA"]);
    assert_has_krdanta(&["upa"], &da, K::aN, &["upadA"]);
    assert_has_krdanta(&["pra"], &dha, K::aN, &["praDA"]);
    assert_has_krdanta(&["upa"], &dha, K::aN, &["upaDA"]);
}

#[test]
fn sutra_3_3_106_v1() {
    let dha = &d("quDA\\Y", Juhotyadi);
    assert_has_upapada_krdanta("Srad", &[], &dha, K::aN, &["SradDA"]);
    assert_has_upapada_krdanta("antar", &[], &dha, K::aN, &["antarDA"]);
}

#[test]
fn sutra_3_3_107() {
    assert_has_krdanta(&[], &nic(&d("qukf\\Y", Tanadi)), K::yuc, &["kAraRA"]);
    assert_has_krdanta(&[], &nic(&d("hf\\Y", Bhvadi)), K::yuc, &["hAraRA"]);
    assert_has_krdanta(&[], &nic(&d("Asa~\\", Adadi)), K::yuc, &["AsanA"]);
    assert_has_krdanta(&[], &nic(&d("SranTa~", Kryadi)), K::yuc, &["SranTanA"]);
}

#[test]
fn sutra_3_3_115() {
    assert_has_krdanta(&[], &d("hase~", Bhvadi), K::lyuw, &["hasana"]);
    assert_has_krdanta(&[], &d("SuBa~\\", Bhvadi), K::lyuw, &["SoBana"]);
    assert_has_krdanta(&[], &d("jalpa~", Bhvadi), K::lyuw, &["jalpana"]);
    assert_has_krdanta(&[], &d("SIN", Adadi), K::lyuw, &["Sayana"]);
    assert_has_krdanta(&[], &d("Asa~\\", Adadi), K::lyuw, &["Asana"]);
}

#[test]
fn sutra_3_3_120() {
    assert_has_krdanta(&["ava"], &d("tF", Bhvadi), K::GaY, &["avatAra"]);
    assert_has_krdanta(&["ava"], &d("stFY", Kryadi), K::GaY, &["avastAra"]);
}

#[test]
fn sutra_3_3_121() {
    use K::GaY;
    assert_has_krdanta(&[], &d("liKa~", Tudadi), GaY, &["leKa"]);
    assert_has_krdanta(&[], &d("vida~", Adadi), GaY, &["veda"]);
    assert_has_krdanta(&[], &d("vezwa~\\", Bhvadi), GaY, &["vezwa"]);
    assert_has_krdanta(&[], &d("ba\\nDa~", Kryadi), GaY, &["banDa"]);
    assert_has_krdanta(&[], &d("mfjU~", Adadi), GaY, &["mArga"]);
    assert_has_krdanta(&["apa", "AN"], &d("mfjU~", Adadi), GaY, &["apAmArga"]);
    assert_has_krdanta(&[], &d("ra\\ma~\\", Bhvadi), GaY, &["rAma"]);
}

#[test]
fn sutra_3_3_125() {
    let khan = d("Kanu~^", Bhvadi);
    assert_has_krdanta(&["AN"], &khan, K::Ga, &["AKana"]);
    assert_has_krdanta(&["AN"], &khan, K::GaY, &["AKAna"]);
}

#[test]
fn sutra_3_3_125_v1() {
    let khan = d("Kanu~^", Bhvadi);
    assert_has_krdanta(&["AN"], &khan, K::qa, &["AKa"]);
}

#[test]
fn sutra_3_3_125_v2() {
    let khan = d("Kanu~^", Bhvadi);
    assert_has_krdanta(&["AN"], &khan, K::qara, &["AKara"]);
}

#[test]
fn sutra_3_3_125_v3() {
    let khan = d("Kanu~^", Bhvadi);
    assert_has_krdanta(&["AN"], &khan, K::ika, &["AKanika"]);
}

#[test]
fn sutra_3_3_125_v4() {
    let khan = d("Kanu~^", Bhvadi);
    assert_has_krdanta(&["AN"], &khan, K::ikavaka, &["AKanikavaka"]);
}

#[ignore]
#[test]
fn sutra_3_3_126() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_upapada_krdanta("Izat", &[], &kr, K::Kal, &["Izatkara"]);
    assert_has_upapada_krdanta("dur", &[], &kr, K::Kal, &["duzkara"]);
    assert_has_upapada_krdanta("su", &[], &kr, K::Kal, &["sukara"]);
}

#[test]
fn sutra_3_3_137() {
    let jiv = d("jIva~", Bhvadi);
    assert_has_tip(&[], &jiv, AshirLin, &["jIvyAt"]);
    assert_has_tip(&[], &jiv, Lot, &["jIvatu", "jIvatAt"]);
    assert_has_tip(&[], &jiv, Lat, &["jIvati"]);
}

#[test]
fn sutra_3_3_139() {
    assert_has_tip(&["AN"], &d("yA\\", Bhvadi), Lrn, &["AyAsyat"]);
    assert_has_tip(&["pari", "AN"], &d("BU", Bhvadi), Lrn, &["paryABavizyat"]);
    assert_has_tip(&["AN"], &d("hve\\Y", Bhvadi), Lrn, &["AhvAsyat"]);
    assert_has_ta(&[], &d("Bu\\ja~", Rudhadi), Lrn, &["aBokzyata"]);
    assert_has_tip(&["AN"], &d("ga\\mx~", Bhvadi), Lrn, &["Agamizyat"]);
}

#[test]
fn sutra_3_3_161() {
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), VidhiLin, &["kuryAt"]);
    assert_has_tip(&["AN"], &d("ga\\mx~", Bhvadi), VidhiLin, &["AgacCet"]);
    assert_has_ta(&[], &d("Bu\\ja~", Rudhadi), VidhiLin, &["BuYjIta"]);
    assert_has_ta(&[], &d("Asa~\\", Adadi), VidhiLin, &["AsIta"]);
    assert_has_tip(&["upa"], &d("RI\\Y", Bhvadi), VidhiLin, &["upanayet"]);
    assert_has_iw(&["aDi"], &d("i\\N", Adadi), VidhiLin, &["aDIyIya"]);
}

#[test]
fn sutra_3_3_162() {
    assert_has_tip(
        &["AN"],
        &d("ga\\mx~", Bhvadi),
        Lot,
        &["AgacCatu", "AgacCatAt"],
    );
    assert_has_ta(&[], &d("Asa~\\", Adadi), Lot, &["AstAm"]);
    assert_has_ta(&[], &d("Bu\\ja~", Rudhadi), Lot, &["BuNktAm"]);

    let i = d("i\\N", Adadi);
    assert_has_tip(&["aDi"], &nic(&i), Lot, &["aDyApayatu", "aDyApayatAt"]);
    assert_has_ta(&["upa"], &d("RI\\Y", Bhvadi), Lot, &["upanayatAm"]);
    assert_has_iw(&["aDi"], &i, Lot, &["aDyayE"]);
}

#[test]
fn sutra_3_3_173() {
    let jiv = d("jIva~", Bhvadi);
    assert_has_ashirlin(&[], &jiv, &["jIvyAt"]);
    assert_has_lot(&[], &jiv, &["jIvatu", "jIvatAt"]);

    // ASizi?
    assert_has_lat(&[], &jiv, &["jIvati"]);
}
