extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha;
use vidyut_prakriya::args::TaddhitaArtha::*;

fn assert_blocked(text: &str, artha: TaddhitaArtha, t: T) {
    assert_has_artha_taddhita(text, artha, t, &[]);
}

#[ignore]
#[test]
fn sutra_5_2_1() {
    let artha = DhanyanamBhavaneKshetre;
    assert_has_artha_taddhita("mudra", artha, T::KaY, &["mOdrIna"]);
    assert_has_artha_taddhita("kudrava", artha, T::KaY, &["kOdravIna"]);
    assert_has_artha_taddhita("kulatTa", artha, T::KaY, &["kOlatTIna"]);
}

#[test]
fn sutra_5_2_2() {
    let artha = DhanyanamBhavaneKshetre;
    assert_has_artha_taddhita("vrIhi", artha, T::Qak, &["vrEheya"]);
    assert_has_artha_taddhita("SAli", artha, T::Qak, &["SAleya"]);
}

#[test]
fn sutra_5_2_3() {
    let artha = DhanyanamBhavaneKshetre;
    assert_has_artha_taddhita("yava", artha, T::yat, &["yavya"]);
    assert_has_artha_taddhita("yavaka", artha, T::yat, &["yavakya"]);
    assert_has_artha_taddhita("zazwika", artha, T::yat, &["zazwikya"]);
}

#[test]
fn sutra_5_2_4() {
    fn assert_yat_or_khan(prati: &str, yat: &str, khan: &str) {
        let artha = DhanyanamBhavaneKshetre;
        assert_has_artha_taddhita(prati, artha, T::yat, &[yat]);
        assert_has_artha_taddhita(prati, artha, T::KaY, &[khan]);
    }
    assert_yat_or_khan("tila", "tilya", "tElIna");
    assert_yat_or_khan("mAza", "mAzya", "mAzIRa");
    assert_yat_or_khan("umA", "umya", "OmIna");
    assert_yat_or_khan("BaNgA", "BaNgya", "BANgIna");
    assert_yat_or_khan("aRu", "aRavya", "ARavIna");
}

#[test]
fn sutra_5_2_5() {
    assert_has_artha_taddhita("sarvacarman", Krta, T::Ka, &["sarvacarmIRa"]);
    assert_has_artha_taddhita("sarvacarman", Krta, T::KaY, &["sArvacarmIRa"]);
}

#[test]
fn sutra_5_2_6() {
    assert_has_artha_taddhita("yaTAmuKa", Darshana, T::Ka, &["yaTAmuKIna"]);
    assert_has_artha_taddhita("sammuKa", Darshana, T::Ka, &["sammuKIna"]);
}

#[test]
fn sutra_5_2_7() {
    assert_has_artha_taddhita("sarvapaTi", Vyapnoti, T::Ka, &["sarvapaTIna"]);
    assert_has_artha_taddhita("sarvANga", Vyapnoti, T::Ka, &["sarvANgIRa"]);
    assert_has_artha_taddhita("sarvakarman", Vyapnoti, T::Ka, &["sarvakarmIRa"]);
    assert_has_artha_taddhita("sarvapatra", Vyapnoti, T::Ka, &["sarvapatrIRa"]);
    assert_has_artha_taddhita("sarvapAtra", Vyapnoti, T::Ka, &["sarvapAtrIRa"]);
}

#[test]
fn sutra_5_2_8() {
    assert_has_artha_taddhita("Aprapada", Prapnoti, T::Ka, &["AprapadIna"]);
}

#[ignore]
#[test]
fn sutra_5_2_9() {
    assert_has_taddhita("anupada", T::Ka, &["anupadIna"]);
    assert_has_taddhita("sarvAnna", T::Ka, &["sarvAnnIna"]);
    assert_has_taddhita("AyAnaya", T::Ka, &["AyAnayIna"]);
}

#[test]
fn sutra_5_2_10() {
    assert_has_artha_taddhita("parovara", TadAnubhavati, T::Ka, &["parovarIRa"]);
    assert_has_artha_taddhita("parampara", TadAnubhavati, T::Ka, &["paramparIRa"]);
    assert_has_artha_taddhita("putrapOtra", TadAnubhavati, T::Ka, &["putrapOtrIRa"]);
}

#[test]
fn sutra_5_2_11() {
    assert_has_artha_taddhita("avArapAra", Gami, T::Ka, &["avArapArIRa"]);
    assert_has_artha_taddhita("atyanta", Gami, T::Ka, &["atyantIna"]);
    assert_has_artha_taddhita("anukAma", Gami, T::Ka, &["anukAmIna"]);
}

#[test]
fn sutra_5_2_15() {
    assert_has_artha_taddhita("anugu", AlamGami, T::Ka, &["anugavIna"]);
}

#[test]
fn sutra_5_2_16() {
    assert_has_artha_taddhita("aDvan", AlamGami, T::yat, &["aDvanya"]);
    assert_has_artha_taddhita("aDvan", AlamGami, T::Ka, &["aDvanIna"]);
}

#[test]
fn sutra_5_2_17() {
    assert_has_artha_taddhita("anugu", AlamGami, T::Ka, &["anugavIna"]);
}

#[test]
fn sutra_5_2_18() {
    assert_has_artha_taddhita("gozWa", BhutaPurva, T::KaY, &["gOzWIna"]);
}

#[test]
fn sutra_5_2_19() {
    assert_has_artha_taddhita("aSva", EkahaGama, T::KaY, &["ASvIna"]);
}

#[test]
fn sutra_5_2_21() {
    assert_has_artha_taddhita("vrAta", TenaJivati, T::KaY, &["vrAtIna"]);
}

#[test]
fn sutra_5_2_24() {
    let artha = TasyaPakamula;
    assert_has_artha_taddhita("pIlu", artha, T::kuRap, &["pIlukuRa"]);
    assert_has_artha_taddhita("karkanDu", artha, T::kuRap, &["karkanDukuRa"]);
    assert_has_artha_taddhita("karRa", artha, T::jAhac, &["karRajAha"]);
}

#[test]
fn sutra_5_2_25() {
    assert_has_artha_taddhita("pakza", TasyaMula, T::ti, &["pakzati"]);
}

#[test]
fn sutra_5_2_26() {
    let artha = TenaVitta;
    assert_has_artha_taddhita("vidyA", artha, T::cuYcup, &["vidyAcuYcu"]);
    assert_has_artha_taddhita("vidyA", artha, T::caRap, &["vidyAcaRa"]);
}

#[test]
fn sutra_5_2_28() {
    assert_has_taddhita("vi", T::SAlac, &["viSAla"]);
    assert_has_taddhita("vi", T::SaNkawac, &["viSaNkawa"]);
}

#[test]
fn sutra_5_2_29() {
    assert_has_taddhita("sam", T::kawac, &["saNkawa"]);
    assert_has_taddhita("pra", T::kawac, &["prakawa"]);
    assert_has_taddhita("ud", T::kawac, &["utkawa"]);
    assert_has_taddhita("vi", T::kawac, &["vikawa"]);
}

#[test]
fn sutra_5_2_29_v1() {
    assert_has_taddhita("alAbU", T::kawac, &["alAbUkawa"]);
    assert_has_taddhita("tila", T::kawac, &["tilakawa"]);
}

#[test]
fn sutra_5_2_30() {
    assert_has_taddhita("ava", T::kawac, &["avakawa"]);
    assert_has_taddhita("ava", T::kuwArac, &["avakuwAra"]);
}

#[test]
fn sutra_5_2_31() {
    assert_has_taddhita("ava", T::wIwac, &["avawIwa"]);
    assert_has_taddhita("ava", T::nAwac, &["avanAwa"]);
    assert_has_taddhita("ava", T::Brawac, &["avaBrawa"]);
}

#[ignore]
#[test]
fn sutra_5_2_32() {
    assert_has_taddhita("ni", T::biqac, &["nibiqa"]);
    assert_has_taddhita("ni", T::birIsac, &["nibirIsa"]);
}

#[test]
fn sutra_5_2_33() {
    assert_has_taddhita("ni", T::inac, &["cikina"]);
    assert_has_taddhita("ni", T::piwac, &["cipiwa"]);
}

#[test]
fn sutra_5_2_34() {
    assert_has_taddhita("upa", T::tyakan, &["upatyaka"]);
    assert_has_taddhita("aDi", T::tyakan, &["aDityaka"]);
}

#[test]
fn sutra_5_2_35() {
    assert_has_taddhita("karman", T::aWac, &["karmaWa"]);
}

#[test]
fn sutra_5_2_36() {
    let artha = TadAsyaSamjatam;
    assert_has_artha_taddhita("tArakA", artha, T::itac, &["tArakita"]);
    assert_has_artha_taddhita("puzpa", artha, T::itac, &["puzpita"]);
}

#[test]
fn sutra_5_2_37() {
    let artha = TadAsyaPramanam;
    assert_has_artha_taddhita("Uru", artha, T::dvayasac, &["Urudvayasa"]);
    assert_has_artha_taddhita("Uru", artha, T::daGnac, &["UrudaGna"]);
    assert_has_artha_taddhita("Uru", artha, T::mAtrac, &["UrumAtra"]);

    assert_has_artha_taddhita("jAnu", artha, T::dvayasac, &["jAnudvayasa"]);
    assert_has_artha_taddhita("jAnu", artha, T::daGnac, &["jAnudaGna"]);
    assert_has_artha_taddhita("jAnu", artha, T::mAtrac, &["jAnumAtra"]);

    assert_has_artha_taddhita("prasTa", artha, T::mAtrac, &["prasTamAtra"]);
}

#[test]
fn sutra_5_2_38() {
    let artha = TadAsyaPramanam;
    assert_has_artha_taddhita("puruza", artha, T::aR, &["pOruza"]);
    assert_has_artha_taddhita("puruza", artha, T::dvayasac, &["puruzadvayasa"]);
    assert_has_artha_taddhita("puruza", artha, T::daGnac, &["puruzadaGna"]);
    assert_has_artha_taddhita("puruza", artha, T::mAtrac, &["puruzamAtra"]);

    assert_has_artha_taddhita("hastin", artha, T::aR, &["hAstina"]);
    assert_has_artha_taddhita("hastin", artha, T::dvayasac, &["hastidvayasa"]);
    assert_has_artha_taddhita("hastin", artha, T::daGnac, &["hastidaGna"]);
    assert_has_artha_taddhita("hastin", artha, T::mAtrac, &["hastimAtra"]);
}

#[test]
fn sutra_5_2_39() {
    assert_has_artha_taddhita("yad", Parimana, T::vatup, &["yAvat"]);
    assert_has_artha_taddhita("tad", Parimana, T::vatup, &["tAvat"]);
    assert_has_artha_taddhita("etad", Parimana, T::vatup, &["etAvat"]);
}

#[test]
fn sutra_5_2_40() {
    assert_has_artha_taddhita("kim", Parimana, T::vatup, &["kiyat"]);
    assert_has_artha_taddhita("idam", Parimana, T::vatup, &["iyat"]);
}

#[test]
fn sutra_5_2_41() {
    assert_has_artha_taddhita("kim", Parimana, T::qati, &["kati"]);
}

#[test]
fn sutra_5_2_42() {
    assert_has_artha_taddhita("paYcan", Avasana, T::tayap, &["paYcataya"]);
    assert_has_artha_taddhita("daSan", Avasana, T::tayap, &["daSataya"]);
    assert_has_artha_taddhita("catur", Avasana, T::tayap, &["catuzwaya"]);

    assert_has_sup_1s(&taddhitanta("catur", T::tayap), Stri, &["catuzwayI"]);
}

#[test]
fn sutra_5_2_43() {
    assert_has_artha_taddhita("dvi", Avasana, T::tayap, &["dvaya", "dvitaya"]);
    assert_has_artha_taddhita("tri", Avasana, T::tayap, &["traya", "tritaya"]);

    let traya = taddhitanta("tri", T::tayap).with_require("traya");
    assert_has_sup_1s(&traya, Stri, &["trayI"]);
}

#[ignore]
#[test]
fn sutra_5_2_44() {
    let ubhaya = create_taddhitanta("uBaya", "uBa", T::tayap);
    assert_has_sup_1s(&ubhaya, Pum, &["uBayaH"]);
    assert_has_sup_1p(&ubhaya, Pum, &["uBaye"]);
}

#[test]
fn sutra_5_2_47() {
    assert_has_artha_taddhita("dvi", Nimana, T::mayaw, &["dvimaya"]);
    assert_has_artha_taddhita("tri", Nimana, T::mayaw, &["trimaya"]);
    assert_has_artha_taddhita("catur", Nimana, T::mayaw, &["caturmaya"]);
}

#[ignore]
#[test]
fn sutra_5_2_48() {
    assert_has_artha_taddhita("ekadaSan", Purana, T::qaw, &["ekadaSa"]);
    assert_has_artha_taddhita("ekadaSan", Purana, T::qaw, &["ekadaSa"]);
}

#[ignore]
#[test]
fn sutra_5_2_49() {
    assert_has_artha_taddhita("paYcan", Purana, T::qaw, &["paYcama"]);
}

#[test]
fn sutra_5_2_54() {
    assert_has_artha_taddhita("dvi", Purana, T::tIya, &["dvitIya"]);
}

#[test]
fn sutra_5_2_55() {
    assert_has_artha_taddhita("tri", Purana, T::tIya, &["tftIya"]);
}

#[test]
fn sutra_5_2_61() {
    let artha = TadAsyaAstiAsmin;
    assert_has_artha_taddhita("vimukta", artha, T::aR, &["vEmukta"]);
    assert_has_artha_taddhita("devAsura", artha, T::aR, &["dEvAsura"]);
}

#[test]
fn sutra_5_2_62() {
    let artha = TadAsyaAstiAsmin;
    assert_has_artha_taddhita("gozada", artha, T::vun, &["gozadaka"]);
    assert_has_artha_taddhita("izetvA", artha, T::vun, &["izetvaka"]);
    assert_has_artha_taddhita("mAtariSvan", artha, T::vun, &["mAtariSvaka"]);
}

#[test]
fn sutra_5_2_63() {
    assert_has_artha_taddhita("paTin", TatraKushala, T::vun, &["paTaka"]);
}

#[test]
fn sutra_5_2_64() {
    assert_has_artha_taddhita("Akarza", TatraKushala, T::kan, &["Akarzaka"]);
    assert_has_artha_taddhita("tsaru", TatraKushala, T::kan, &["tsaruka"]);
}

#[test]
fn sutra_5_2_65() {
    assert_has_artha_taddhita("Dana", TatraKama, T::kan, &["Danaka"]);
    assert_has_artha_taddhita("hiraRya", TatraKama, T::kan, &["hiraRyaka"]);
}

#[test]
fn sutra_5_2_67() {
    assert_has_artha_taddhita("udara", TatraAdyuna, T::Wak, &["Odarika"]);
    assert_blocked("udara", TatraAdyuna, T::kan);
    // HACK -- what is the real artha for udaraka?
    assert_has_artha_taddhita("udara", Alpe, T::ka, &["udaraka"]);
}

#[test]
fn sutra_5_2_68() {
    assert_has_artha_taddhita("sasya", TatraParijata, T::kan, &["sasyaka"]);
}

#[test]
fn sutra_5_2_69() {
    assert_has_artha_taddhita("aMSa", Hari, T::kan, &["aMSaka"]);
}

#[test]
fn sutra_5_2_70() {
    assert_has_artha_taddhita("tantra", AciraApahrta, T::kan, &["tantraka"]);
}

#[test]
fn sutra_5_2_72() {
    assert_has_artha_taddhita("SIta", Karin, T::kan, &["SItaka"]);
    assert_has_artha_taddhita("uzRa", Karin, T::kan, &["uzRaka"]);
}

#[test]
fn sutra_5_2_75() {
    assert_has_artha_taddhita("pArSva", Anvicchati, T::kan, &["pArSvaka"]);
}

#[test]
fn sutra_5_2_94() {
    assert_has_taddhita("go", T::matup, &["gomat"]);
    assert_has_taddhita("vfkza", T::matup, &["vfkzavat"]);
    assert_has_taddhita("yava", T::matup, &["yavamat"]);
    assert_has_taddhita("plakza", T::matup, &["plakzavat"]);
}

#[test]
fn sutra_5_2_96() {
    assert_has_taddhita("cUqA", T::lac, &["cUqAla"]);
    assert_has_taddhita("cUqA", T::matup, &["cUqAvat"]);
    // AtaH
    assert_has_taddhita("hasta", T::matup, &["hastavat"]);
    assert_has_taddhita("pAda", T::matup, &["pAdavat"]);
}

#[test]
fn sutra_5_2_97() {
    let artha = TadAsyaAstiAsmin;
    assert_has_artha_taddhita("siDma", artha, T::lac, &["siDmala"]);
    assert_has_artha_taddhita("siDma", artha, T::matup, &["siDmavat"]);
    assert_has_artha_taddhita("gaqu", artha, T::lac, &["gaqula"]);
    assert_has_artha_taddhita("gaqu", artha, T::matup, &["gaqumat"]);
}

#[test]
fn sutra_5_2_99() {
    let artha = TadAsyaAstiAsmin;
    assert_has_artha_taddhita("Pena", artha, T::ilac, &["Penila"]);
    assert_has_artha_taddhita("Pena", artha, T::lac, &["Penala"]);
    assert_has_artha_taddhita("Pena", artha, T::matup, &["Penavat"]);
}

#[test]
fn sutra_5_2_100() {
    assert_has_taddhita("loman", T::Sa, &["lomaSa"]);
    assert_has_taddhita("loman", T::matup, &["lomavat"]);
    assert_has_taddhita("pAman", T::na, &["pAmana"]);
    assert_has_taddhita("pAman", T::matup, &["pAmavat"]);
    assert_has_taddhita("picCa", T::ilac, &["picCila"]);
    assert_has_taddhita("picCa", T::matup, &["picCavat"]);
    assert_has_taddhita("uras", T::ilac, &["urasila"]);
    assert_has_taddhita("uras", T::matup, &["urasvat"]);
}

#[test]
fn sutra_5_2_112() {
    let artha = TadAsyaAstiAsmin;
    assert_has_artha_taddhita("rajas", artha, T::valac, &["rajasvala"]);
    assert_has_artha_taddhita("kfzI", artha, T::valac, &["kfzIvala"]);
    assert_has_artha_taddhita("AsutI", artha, T::valac, &["AsutIvala"]);
    assert_has_artha_taddhita("parizad", artha, T::valac, &["parizadvala"]);
}

#[test]
fn sutra_5_2_112_v1() {
    let artha = TadAsyaAstiAsmin;
    assert_has_artha_taddhita("BrAtf", artha, T::valac, &["BrAtfvala"]);
    assert_has_artha_taddhita("putra", artha, T::valac, &["putravala"]);
    assert_has_artha_taddhita("utsAha", artha, T::valac, &["utsAhavala"]);
}

#[test]
fn sutra_5_2_121() {
    assert_has_taddhita("yaSas", T::vini, &["yaSasvin"]);
    assert_has_taddhita("payas", T::vini, &["payasvin"]);
    assert_has_taddhita("mAyA", T::vini, &["mAyAvin"]);
    assert_has_taddhita("meDA", T::vini, &["meDAvin"]);
    assert_has_taddhita("sraj", T::vini, &["sragvin"]);
}
