extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Dhatu;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Krdanta;
use vidyut_prakriya::args::Lakara::*;

pub fn create_kta(text: &str, prefixes: &[&str], d: &Dhatu) -> Krdanta {
    create_krdanta(text, prefixes, d, Krt::kta)
}

pub fn kta(prefixes: &[&str], d: &Dhatu) -> Krdanta {
    krdanta(prefixes, d, Krt::kta)
}

#[test]
fn sutra_2_1_1() {
    assert_has_dvitiya_tatpurusha("kazwa", "Srita", &["kazwaSrita"]);
    assert_has_trtiya_tatpurusha("SaNkulA", "KaRqa", &["SaNkulAKaRqa"]);
    assert_has_caturthi_tatpurusha("yUpa", "dAru", &["yUpadAru"]);
    assert_has_panchami_tatpurusha("vfka", "Baya", &["vfkaBaya"]);
    assert_has_sasthi_tatpurusha("rAjan", "puruza", &["rAjapuruza"]);
    assert_has_saptami_tatpurusha("akza", "SORqa", &["akzaSORqa"]);
}

#[test]
fn skip_sutra_2_1_3() {}

#[test]
fn sutra_2_1_4() {
    assert_has_tip(&["anu", "vi"], &d("cala~", Bhvadi), Lan, &["anuvyacalat"]);
    assert_has_tip(&["anu", "pra"], &d("vfzu~", Bhvadi), Lan, &["anuprAvarzat"]);
}

#[test]
fn sutra_2_1_5() {
    assert_has_avyayibhava("yaTA", "vfdDa", &["yaTAvfdDam"]);
}

#[ignore]
#[test]
fn sutra_2_1_6() {
    // TODO: Kashika seems to have typos on ashtadhyayi.com -- double check these.
    assert_has_avyayibhava("aDi", "strI", &["aDistri"]);
    assert_has_avyayibhava("aDi", "kumArI", &["aDikumAri"]);
    assert_has_avyayibhava("upa", "kumBa", &["upakumBam"]);
    assert_has_avyayibhava("upa", "maRika", &["upamaRikam"]);
    assert_has_avyayibhava("su", "madra", &["sumadram"]);
    assert_has_avyayibhava("su", "magaDa", &["sumagaDam"]);
    assert_has_avyayibhava("dur", "gavadika", &["durgavadikam"]);
    assert_has_avyayibhava("dur", "yavana", &["duryavanam"]);
    assert_has_avyayibhava("nir", "makzika", &["nirmakzikam"]);
    assert_has_avyayibhava("nir", "maSaka", &["nirmaSakam"]);
    assert_has_avyayibhava("ati", "hima", &["atihimam"]);
    assert_has_avyayibhava("nir", "hima", &["nirhimam"]);
    assert_has_avyayibhava("nir", "SIta", &["niHSItam", "niSSItam"]);
    assert_has_avyayibhava("ati", "tEsfka", &["atitEsfkam"]);
    assert_has_avyayibhava("tad", "pARini", &["tatpARini"]);
    assert_has_avyayibhava("anu", "raTa", &["anuraTam"]);
    assert_has_avyayibhava("prati", "arTa", &["pratyarTam"]);
    assert_has_avyayibhava("yaTA", "Sakti", &["yaTASakti"]);
    assert_has_avyayibhava("anu", "jyezWa", &["anujyezWam"]);
    assert_has_avyayibhava("sa", "cakra", &["sacakram"]);
    assert_has_avyayibhava("sa", "brahman", &["sabrahma"]);
    assert_has_avyayibhava("sa", "kzatra", &["sakzatram"]);
    assert_has_avyayibhava("sa", "busa", &["sabusam"]);
    assert_has_avyayibhava("sa", "agni", &["sAgni"]);
    assert_has_avyayibhava("sa", "izwi", &["sezwi"]);
}

#[test]
fn sutra_2_1_7() {
    assert_has_avyayibhava("yaTA", "vfdDa", &["yaTAvfdDam"]);
    assert_has_avyayibhava("yaTA", "aDyApaka", &["yaTADyApakam"]);
}

#[test]
fn sutra_2_1_8() {
    assert_has_avyayibhava("yAvat", "amatra", &["yAvadamatram"]);
}

#[test]
fn sutra_2_1_9() {
    assert_has_avyayibhava("SAka", "prati", &["SAkaprati"]);
}

#[test]
fn sutra_2_1_10() {
    assert_has_avyayibhava("akza", "pari", &["akzapari"]);
    assert_has_avyayibhava("SalAkA", "pari", &["SalAkApari"]);
    assert_has_avyayibhava("eka", "pari", &["ekapari"]);
    assert_has_avyayibhava("dvi", "pari", &["dvipari"]);
    assert_has_avyayibhava("tri", "pari", &["tripari"]);
    assert_has_avyayibhava("catus", "pari", &["catuzpari"]);
}

#[test]
fn sutra_2_1_13() {
    assert_has_avyayibhava("A", "pAwaliputra", &["ApAwaliputram"]);
    assert_has_avyayibhava("A", "kumAra", &["AkumAram"]);
}

#[test]
fn sutra_2_1_14() {
    assert_has_avyayibhava("aBi", "agni", &["aByagni"]);
    assert_has_avyayibhava("prati", "agni", &["pratyagni"]);
}

#[test]
fn sutra_2_1_15() {
    assert_has_avyayibhava("anu", "vana", &["anuvanam"]);
}

#[test]
fn sutra_2_1_19() {
    assert_has_avyayibhava("dvi", "muni", &["dvimuni"]);
    assert_has_avyayibhava("tri", "muni", &["trimuni"]);
}

#[test]
fn sutra_2_1_24() {
    assert_has_dvitiya_tatpurusha("kazwa", "Srita", &["kazwaSrita"]);
    assert_has_dvitiya_tatpurusha("naraka", "Srita", &["narakaSrita"]);
    assert_has_dvitiya_tatpurusha("kAntAra", "atIta", &["kAntArAtIta"]);
    assert_has_dvitiya_tatpurusha("naraka", "patita", &["narakapatita"]);
    assert_has_dvitiya_tatpurusha("grAma", "gata", &["grAmagata"]);
    assert_has_dvitiya_tatpurusha("taraNga", "atyasta", &["taraNgAtyasta"]);
    assert_has_dvitiya_tatpurusha("tuhina", "atyasta", &["tuhinAtyasta"]);
    assert_has_dvitiya_tatpurusha("suKa", "prApta", &["suKaprApta"]);
    assert_has_dvitiya_tatpurusha("suKa", "Apanna", &["suKApanna"]);
    assert_has_dvitiya_tatpurusha("duHKa", "Apanna", &["duHKApanna"]);
}

#[test]
fn sutra_2_1_25() {
    let dhauta = kta(&[], &d("DAvu~^", Bhvadi));
    let vilina = kta(&["vi"], &d("lI\\N", Divadi));
    assert_has_avyaya_tatpurusha("svayam", &dhauta, &["svayanDOta"]);
    assert_has_avyaya_tatpurusha("svayam", &vilina, &["svayaMvilIna"]);
}

#[test]
fn sutra_2_1_26() {
    let arudha = kta(&["AN"], &d("ru\\ha~", Bhvadi));
    let pluta = kta(&[], &d("plu\\N", Bhvadi));
    assert_has_dvitiya_tatpurusha("KawvA", &arudha, &["KawvArUQa"]);
    assert_has_dvitiya_tatpurusha("KawvA", &pluta, &["KawvApluta"]);
}

#[test]
fn sutra_2_1_27() {
    let krta = create_kta("kfta", &[], &d("qukf\\Y", Tanadi));
    let pita = create_kta("pIta", &[], &d("pA\\", Bhvadi));
    let bhukta = create_kta("Bukta", &[], &d("Bu\\ja~", Rudhadi));
    assert_has_avyaya_tatpurusha("sAmi", &krta, &["sAmikfta"]);
    assert_has_avyaya_tatpurusha("sAmi", &pita, &["sAmipIta"]);
    assert_has_avyaya_tatpurusha("sAmi", &bhukta, &["sAmiBukta"]);
}

#[test]
fn sutra_2_1_30() {
    assert_has_trtiya_tatpurusha("SaNkulA", "KaRqa", &["SaNkulAKaRqa"]);
    assert_has_trtiya_tatpurusha("kiri", "kARa", &["kirikARa"]);
    assert_has_trtiya_tatpurusha("DAnya", "arTa", &["DAnyArTa"]);
}

#[test]
fn sutra_2_1_31() {
    assert_has_trtiya_tatpurusha("mAsa", "pUrva", &["mAsapUrva"]);
    assert_has_trtiya_tatpurusha("saMvatsara", "pUrva", &["saMvatsarapUrva"]);
    assert_has_trtiya_tatpurusha("mAtf", "sadfSa", &["mAtfsadfSa"]);
    assert_has_trtiya_tatpurusha("pitf", "sadfSa", &["pitfsadfSa"]);
    assert_has_trtiya_tatpurusha("mAtf", "sama", &["mAtfsama"]);
    assert_has_trtiya_tatpurusha("mASa", "Una", &["mASona"]);
    assert_has_trtiya_tatpurusha("kArzApaRa", "Una", &["kArzApaRona"]);
    // TODO: mAzavikala, kArzApanavikala
    assert_has_trtiya_tatpurusha("asi", "kalaha", &["asikalaha"]);
    assert_has_trtiya_tatpurusha("vAc", "kalaha", &["vAkkalaha"]);
    assert_has_trtiya_tatpurusha("vAc", "nipuRa", &["vANnipuRa"]);
    assert_has_trtiya_tatpurusha("AcAra", "nipuRa", &["AcAranipuRa"]);
    assert_has_trtiya_tatpurusha("guqa", "miSra", &["guqamiSra"]);
    assert_has_trtiya_tatpurusha("tila", "miSra", &["tilamiSra"]);
    assert_has_trtiya_tatpurusha("AcAra", "SlakzRa", &["AcAraSlakzRa"]);
}

#[test]
fn sutra_2_1_32() {
    assert_has_trtiya_tatpurusha("ahi", "hata", &["ahihata"]);
    let nirbhinna = krdanta(&["nir"], &d("Bi\\di~^r", Rudhadi), Krt::kta);
    assert_has_trtiya_tatpurusha("naKa", nirbhinna, &["naKanirBinna"]);
    assert_has_trtiya_tatpurusha("paraSu", "Cinna", &["paraSucCinna"]);
}

#[test]
fn sutra_2_1_33() {
    let peya = create_krdanta("peya", &[], &d("pA\\", Bhvadi), Krt::yat);
    assert_has_trtiya_tatpurusha("kAka", &peya, &["kAkapeya"]);

    let lehya = create_krdanta("lehya", &[], &d("li\\ha~^", Adadi), Krt::Ryat);
    assert_has_trtiya_tatpurusha("Svan", &lehya, &["Svalehya"]);

    let chedya = create_krdanta("Cedya", &[], &d("Ci\\di~^r", Rudhadi), Krt::Ryat);
    assert_has_trtiya_tatpurusha("bAzpa", &chedya, &["bAzpacCedya"]);

    let sanceya = krdanta(&["sam"], &d("ci\\Y", Svadi), Krt::yat);
    assert_has_trtiya_tatpurusha("kaRwaka", &sanceya, &["kaRwakasaYceya"]);
}

#[test]
fn sutra_2_1_34() {
    assert_has_trtiya_tatpurusha("daDi", "odana", &["daDyodana"]);
    assert_has_trtiya_tatpurusha("kzIra", "odana", &["kzIrOdana"]);
}

#[test]
fn sutra_2_1_36() {
    assert_has_caturthi_tatpurusha("yUpa", "dAru", &["yUpadAru"]);
    assert_has_caturthi_tatpurusha("kuRqala", "hiraRya", &["kuRqalahiraRya"]);
    assert_has_caturthi_tatpurusha("brAhmaRa", "arTa", &["brAhmaRArTa"]);
    assert_has_caturthi_tatpurusha("kubera", "bali", &["kuberabali"]);
    assert_has_caturthi_tatpurusha("mahArAja", "bali", &["mahArAjabali"]);
    assert_has_caturthi_tatpurusha("go", "hita", &["gohita"]);
    assert_has_caturthi_tatpurusha("aSva", "hita", &["aSvahita"]);
    assert_has_caturthi_tatpurusha("go", "suKa", &["gosuKa"]);
    assert_has_caturthi_tatpurusha("aSva", "suKa", &["aSvasuKa"]);
    assert_has_caturthi_tatpurusha("go", "rakzita", &["gorakzita"]);
    assert_has_caturthi_tatpurusha("aSva", "rakzita", &["aSvarakzita"]);
}

#[test]
fn sutra_2_1_37() {
    assert_has_panchami_tatpurusha("vfka", "Baya", &["vfkaBaya"]);
    assert_has_panchami_tatpurusha("cOra", "Baya", &["cOraBaya"]);
    assert_has_panchami_tatpurusha("dasyu", "Baya", &["dasyuBaya"]);
}

#[test]
fn sutra_2_1_37_v1() {
    assert_has_panchami_tatpurusha("vfka", "BIta", &["vfkaBIta"]);
    assert_has_panchami_tatpurusha("vfka", "BIti", &["vfkaBIti"]);
    assert_has_panchami_tatpurusha("vfka", "BI", &["vfkaBI"]);
}

#[test]
fn sutra_2_1_38() {
    assert_has_panchami_tatpurusha("suKa", "apeta", &["suKApeta"]);
    assert_has_panchami_tatpurusha("kalpanA", "apoQa", &["kalpanApoQa"]);
    assert_has_panchami_tatpurusha("cakra", "mukta", &["cakramukta"]);
    assert_has_panchami_tatpurusha("svarga", "patita", &["svargapatita"]);
    assert_has_panchami_tatpurusha("taraNga", "apatrasta", &["taraNgApatrasta"]);
}

#[test]
fn sutra_2_1_40() {
    assert_has_saptami_tatpurusha("akza", "SORqa", &["akzaSORqa"]);
    assert_has_saptami_tatpurusha("akza", "DUrta", &["akzaDUrta"]);
    assert_has_saptami_tatpurusha("akza", "kitava", &["akzakitava"]);
}

#[test]
fn sutra_2_1_41() {
    assert_has_saptami_tatpurusha("sANkASya", "sidDa", &["sANkASyasidDa"]);
    assert_has_saptami_tatpurusha("kAmpilya", "sidDa", &["kAmpilyasidDa"]);
    assert_has_saptami_tatpurusha("Atapa", "Suzka", &["AtapaSuzka"]);
    assert_has_saptami_tatpurusha("CAyA", "Suzka", &["CAyASuzka"]);
    assert_has_saptami_tatpurusha("sTAlI", "pakva", &["sTAlIpakva"]);
    assert_has_saptami_tatpurusha("kumBI", "pakva", &["kumBIpakva"]);
    assert_has_saptami_tatpurusha("cakra", "banDa", &["cakrabanDa"]);
}

#[test]
fn sutra_2_1_42() {
    assert_has_saptami_tatpurusha("tIrTa", "DvANkza", &["tIrTaDvANkza"]);
    // TODO: others
}

#[ignore]
#[test]
fn sutra_2_1_47() {
    let bhukta = create_kta("Bukta", &[], &d("Bu\\ja~", Rudhadi));
    let krta = create_kta("kfta", &[], &d("qukf\\Y", Tanadi));
    let pita = create_kta("pIta", &[], &d("pA\\", Bhvadi));
    assert_has_saptami_tatpurusha("tatra", &bhukta, &["tatraBukta"]);
    assert_has_saptami_tatpurusha("tatra", &krta, &["tatrakfta"]);
    assert_has_saptami_tatpurusha("tatra", &pita, &["tatrapIta"]);
}

#[test]
fn sutra_2_1_58() {
    assert_has_karmadharaya("pUrva", "puruza", &["pUrvapuruza"]);
    assert_has_karmadharaya("apara", "puruza", &["aparapuruza"]);
    assert_has_karmadharaya("praTama", "puruza", &["praTamapuruza"]);
    assert_has_karmadharaya("carama", "puruza", &["caramapuruza"]);
    assert_has_karmadharaya("jaGanya", "puruza", &["jaGanyapuruza"]);
    assert_has_karmadharaya("samAna", "puruza", &["samAnapuruza"]);
    assert_has_karmadharaya("maDya", "puruza", &["maDyapuruza"]);
    assert_has_karmadharaya("maDyama", "puruza", &["maDyamapuruza"]);
    assert_has_karmadharaya("vIra", "puruza", &["vIrapuruza"]);
}

#[test]
fn sutra_2_1_61() {
    assert_has_karmadharaya("sat", "puruza", &["satpuruza"]);
    assert_has_karmadharaya("mahat", "puruza", &["mahApuruza"]);
    assert_has_karmadharaya("parama", "puruza", &["paramapuruza"]);
    assert_has_karmadharaya("uttama", "puruza", &["uttamapuruza"]);
    assert_has_karmadharaya("utkfzwa", "puruza", &["utkfzwapuruza"]);
}

#[test]
fn sutra_2_1_62() {
    assert_has_karmadharaya("go", "vfndAraka", &["govfndAraka"]);
    assert_has_karmadharaya("aSva", "vfndAraka", &["aSvavfndAraka"]);
    assert_has_karmadharaya("go", "nAga", &["gonAga"]);
    assert_has_karmadharaya("aSva", "nAga", &["aSvanAga"]);
    assert_has_karmadharaya("go", "kuYjara", &["gokuYjara"]);
    assert_has_karmadharaya("aSva", "kuYjara", &["aSvakuYjara"]);
    // TODO: pujyamAna?
}

#[ignore]
#[test]
fn sutra_2_1_64() {
    assert_has_karmadharaya("kim", "rAjan", &["kiMrAjan"]);
    assert_has_karmadharaya("kim", "saKi", &["kiMsaKi"]);
    assert_has_karmadharaya("kim", "go", &["kiNgo"]);
}

#[test]
fn sutra_2_1_67() {
    assert_has_karmadharaya("yuvan", "Kalati", &["yuvaKalati"]);
    assert_has_karmadharaya("yuvan", "palita", &["yuvapalita"]);
    assert_has_karmadharaya("yuvan", "valina", &["yuvavalina"]);
    assert_has_karmadharaya("yuvan", "jarati", &["yuvajarati"]);
    // TODO: stri
}
