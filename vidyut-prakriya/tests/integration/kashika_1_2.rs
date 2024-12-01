extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::*;

#[test]
fn sutra_1_2_1() {
    assert_has_ta(&["aDi"], &d("i\\N", Adadi), Lun, &["aDyagIzwa", "aDyEzwa"]);

    let kuw = d_kutadi("kuwa~", Tudadi);
    assert_has_krdanta(&[], &kuw, Krt::tfc, &["kuwitf"]);
    assert_has_krdanta(&[], &kuw, Krt::tumun, &["kuwitum"]);
    assert_has_krdanta(&[], &kuw, Krt::tavya, &["kuwitavya"]);

    let puw = d_kutadi("puwa~", Tudadi);
    assert_has_krdanta(&["ud"], &puw, Krt::tfc, &["utpuwitf"]);
    assert_has_krdanta(&["ud"], &puw, Krt::tumun, &["utpuwitum"]);
    assert_has_krdanta(&["ud"], &puw, Krt::tavya, &["utpuwitavya"]);

    let kuw_nic = nic(&kuw);
    assert_has_tip(&["ud"], &kuw_nic, Lat, &["utkowayati"]);
    assert_has_tip(&["ud"], &kuw, Lit, &["uccukowa"]);
    assert_has_krdanta(&["ud"], &kuw, Krt::Rvul, &["utkowaka"]);
    assert_has_krdanta(&["ud"], &kuw, Krt::GaY, &["utkowa"]);
    // TODO: varttika
}

#[test]
fn sutra_1_2_2() {
    let vij = d("o~vijI~\\", Tudadi);
    assert_has_krdanta(&["ud"], &vij, Krt::tfc, &["udvijitf"]);
    assert_has_krdanta(&["ud"], &vij, Krt::tumun, &["udvijitum"]);
    assert_has_krdanta(&["ud"], &vij, Krt::tavya, &["udvijitavya"]);

    assert_has_krdanta(&["ud"], &vij, Krt::lyuw, &["udvejana"]);
    assert_has_krdanta(&["ud"], &vij, Krt::anIyar, &["udvejanIya"]);
}

#[test]
fn sutra_1_2_3() {
    let urnu = d("UrRuY", Adadi);
    assert_has_krdanta(&["pra"], &urnu, Krt::tfc, &["prorRuvitf", "prorRavitf"]);
    // iwi
    assert_has_krdanta(&["pra"], &urnu, Krt::lyuw, &["prorRavana"]);
    assert_has_krdanta(&["pra"], &urnu, Krt::anIyar, &["prorRavanIya"]);
}

#[test]
fn sutra_1_2_4() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_tas(&[], &kf, Lat, &["kurutaH"]);
    assert_has_jhi(&[], &kf, Lat, &["kurvanti"]);

    let ci = d("ciY", Svadi);
    assert_has_tas(&[], &ci, Lat, &["cinutaH"]);
    assert_has_jhi(&[], &ci, Lat, &["cinvanti"]);

    // sArvadhAtukam
    assert_has_krdanta(&[], &kf, Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &kf, Krt::tumun, &["kartum"]);
    assert_has_krdanta(&[], &kf, Krt::tavya, &["kartavya"]);
    // apit
    assert_has_tip(&[], &kf, Lat, &["karoti"]);
    assert_has_sip(&[], &kf, Lat, &["karozi"]);
    assert_has_mip(&[], &kf, Lat, &["karomi"]);
}

#[test]
fn sutra_1_2_5() {
    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_tas(&[], &bhid, Lit, &["biBidatuH"]);
    assert_has_jhi(&[], &bhid, Lit, &["biBiduH"]);

    let chid = d("Ci\\di~^r", Rudhadi);
    assert_has_tas(&[], &chid, Lit, &["cicCidatuH"]);
    assert_has_jhi(&[], &chid, Lit, &["cicCiduH"]);

    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_tas(&[], &yaj, Lit, &["IjatuH"]);
    assert_has_jhi(&[], &yaj, Lit, &["IjuH"]);

    // asaMyogAt
    assert_has_lit(&[], &d("sransu~\\", Bhvadi), &["sasraMse"]);
    assert_has_lit(&[], &d("Dvansu~\\", Bhvadi), &["daDvaMse"]);
    // apit
    assert_has_sip(&[], &bhid, Lit, &["biBediTa"]);
}

#[test]
fn sutra_1_2_6() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &bhu, Lit, &["baBUva"]);
    assert_has_sip(&[], &bhu, Lit, &["baBUviTa"]);
}

#[test]
fn sutra_1_2_6_v1() {
    let sranth = d("SranTa~", Kryadi);
    assert_has_tas(&[], &sranth, Lit, &["SreTatuH", "SaSranTatuH"]);
    assert_has_jhi(&[], &sranth, Lit, &["SreTuH", "SaSranTuH"]);

    let granth = d("granTa~", Kryadi);
    assert_has_tas(&[], &granth, Lit, &["greTatuH", "jagranTatuH"]);
    assert_has_jhi(&[], &granth, Lit, &["greTuH", "jagranTuH"]);

    let danbh = d("danBu~", Svadi);
    assert_has_jhi(&[], &danbh, Lit, &["deBuH", "dadamBuH"]);

    let svaj = d("zva\\nja~\\", Bhvadi);
    assert_has_ta(&["pari"], &svaj, Lit, &["parizasvaje", "parizasvaYje"]);
    assert_has_aataam(&["pari"], &svaj, Lit, &["parizasvajAte", "parizasvaYjAte"]);
}

#[test]
fn sutra_1_2_7() {
    assert_has_krdanta(&[], &d("mfqa~", Tudadi), Krt::ktvA, &["mfqitvA"]);
    assert_has_krdanta(&[], &d("mfda~", Kryadi), Krt::ktvA, &["mfditvA"]);
    assert_has_krdanta(&[], &d("guDa~", Divadi), Krt::ktvA, &["guDitvA"]);
    assert_has_krdanta(&[], &d("kuza~", Kryadi), Krt::ktvA, &["kuzitvA"]);
    // Applies to both kliS-dhAtus -- see 7.2.50
    assert_has_krdanta(
        &[],
        &d("kliSa~\\", Divadi),
        Krt::ktvA,
        &["kliSitvA", "klizwvA"],
    );
    assert_has_krdanta(
        &[],
        &d("kliSU~\\", Kryadi),
        Krt::ktvA,
        &["kliSitvA", "klizwvA"],
    );
    assert_has_krdanta(&[], &d("vada~", Bhvadi), Krt::ktvA, &["uditvA"]);
    assert_has_krdanta(&[], &d("va\\sa~", Bhvadi), Krt::ktvA, &["uzitvA"]);
}

#[test]
fn sutra_1_2_8() {
    let rud = d("rudi~r", Adadi);
    assert_has_krdanta(&[], &rud, Krt::ktvA, &["ruditvA"]);
    assert_has_tip(&[], &san(&rud), Lat, &["rurudizati"]);

    let vid = d("vida~", Adadi);
    assert_has_krdanta(&[], &vid, Krt::ktvA, &["viditvA"]);
    assert_has_tip(&[], &san(&vid), Lat, &["vividizati"]);

    let mus = d("muza~", Kryadi);
    assert_has_krdanta(&[], &mus, Krt::ktvA, &["muzitvA"]);
    assert_has_tip(&[], &san(&mus), Lat, &["mumuzizati"]);

    let grah = d("graha~^", Kryadi);
    assert_has_krdanta(&[], &grah, Krt::ktvA, &["gfhItvA"]);
    assert_has_tip(&[], &san(&grah), Lat, &["jiGfkzati"]);

    let svap = d("Yizva\\pa~", Adadi);
    assert_has_krdanta(&[], &svap, Krt::ktvA, &["suptvA"]);
    assert_has_tip(&[], &san(&svap), Lat, &["suzupsati"]);

    let prach = d("pra\\Ca~", Tudadi);
    assert_has_krdanta(&[], &prach, Krt::ktvA, &["pfzwvA"]);
    assert_has_tip(&[], &san(&prach), Lat, &["pipfcCizati"]);
}

#[test]
fn sutra_1_2_9() {
    assert_has_tip(
        &[],
        &san(&d("ci\\Y", Svadi)),
        Lat,
        &["cicIzati", "cikIzati"],
    );
    assert_has_tip(&[], &san(&d("zwu\\Y", Adadi)), Lat, &["tuzwUzati"]);
    assert_has_tip(&[], &san(&d("qukf\\Y", Tanadi)), Lat, &["cikIrzati"]);
    assert_has_tip(&[], &san(&d("pA\\", Bhvadi)), Lat, &["pipAsati"]);
    assert_has_tip(&[], &san(&d("zWA\\", Bhvadi)), Lat, &["tizWAsati"]);
    assert_has_ta(&[], &san(&d("SIN", Adadi)), Lat, &["SiSayizate"]);
    assert_has_tip(
        &[],
        &san(&d("jYapa~", Curadi)),
        Lat,
        &["jYIpsati", "jijYapayizati"],
    );
}

#[test]
fn sutra_1_2_10() {
    assert_has_tip(&[], &san(&d("Bi\\di~^r", Rudhadi)), Lat, &["biBitsati"]);
    assert_has_ta(&[], &san(&d("bu\\Da~\\", Divadi)), Lat, &["buButsate"]);
    // ikaH
    assert_has_ta(&[], &san(&d("ya\\ja~^", Bhvadi)), Lat, &["yiyakzate"]);
    // Jal
    assert_has_ta(&[], &san(&d("vftu~\\", Bhvadi)), Lat, &["vivartizate"]);
    // danB also in scope.
    assert_has_tip(
        &[],
        &san(&d("danBu~", Svadi)),
        Lat,
        &["DIpsati", "Dipsati", "didamBizati"],
    );
}

#[test]
fn sutra_1_2_11() {
    let bhid = d("Bi\\di~^r", Rudhadi);
    let budh = d("bu\\Da~\\", Divadi);
    assert_has_ta(&[], &bhid, AshirLin, &["BitsIzwa"]);
    assert_has_ta(&[], &budh, AshirLin, &["ButsIzwa"]);
    // sic
    assert_has_ta(&[], &bhid, Lun, &["aBitta"]);
    assert_has_ta(&[], &budh, Lun, &["abudDa", "aboDi"]);
    // ikaH
    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_ta(&[], &yaj, AshirLin, &["yakzIzwa"]);
    assert_has_ta(&[], &yaj, Lun, &["ayazwa"]);
    // Atmanepadezu
    assert_has_tip(&[], &d("sf\\ja~", Tudadi), Lun, &["asrAkzIt"]);
    assert_has_tip(&[], &d("df\\Si~r", Bhvadi), Lun, &["adrAkzIt", "adarSat"]);
    // halantAt
    let ci = d("ci\\Y", Svadi);
    assert_has_ta(&[], &ci, AshirLin, &["cezIzwa"]);
    assert_has_ta(&[], &ci, Lun, &["acezwa"]);
    // Jali
    let vft = d("vftu~\\", Bhvadi);
    assert_has_ta(&[], &vft, AshirLin, &["vartizIzwa"]);
    assert_has_ta(&[], &vft, Lun, &["avartizwa"]);
    // liNsicO
    let dviz = d("dvi\\za~^", Adadi);
    assert_has_krdanta(&[], &dviz, Krt::tfc, &["dvezwf"]);
    assert_has_ta(&[], &dviz, Lrt, &["dvekzyate"]);
}

#[test]
fn sutra_1_2_12() {
    let kf = d("qukf\\Y", Tanadi);
    let hf = d("hf\\Y", Bhvadi);
    assert_has_ta(&[], &kf, AshirLin, &["kfzIzwa"]);
    assert_has_ta(&[], &hf, AshirLin, &["hfzIzwa"]);
    // sicaH
    assert_has_ta(&[], &kf, Lun, &["akfta"]);
    assert_has_ta(&[], &hf, Lun, &["ahfta"]);
    // Jali
    let vf = d("vfY", Svadi);
    assert_has_ta(&[], &vf, AshirLin, &["varizIzwa", "vfzIzwa"]);
    assert_has_ta(&[], &vf, Lun, &["avarizwa", "avarIzwa", "avfta"]);
}

#[test]
fn sutra_1_2_13() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_ta(&["sam"], &gam, AshirLin, &["saNgaMsIzwa", "saNgasIzwa"]);
    // sicaH
    assert_has_ta(&["sam"], &gam, Lun, &["samagaMsta", "samagata"]);
}

#[test]
fn sutra_1_2_14() {
    let han = d("ha\\na~", Adadi);
    assert_has_ta(&["AN"], &han, Lun, &["Ahata", "AvaDizwa"]);
    assert_has_aataam(&["AN"], &han, Lun, &["AhasAtAm", "AvaDizAtAm"]);
    assert_has_jha(&["AN"], &han, Lun, &["Ahasata", "AvaDizata"]);
}

#[test]
fn sutra_1_2_15() {
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_ta(&["ud", "AN"], &yam, Lun, &["udAyata", "udAyaMsta"]);
    assert_has_aataam(&["ud", "AN"], &yam, Lun, &["udAyasAtAm", "udAyaMsAtAm"]);
    assert_has_jha(&["ud", "AN"], &yam, Lun, &["udAyasata", "udAyaMsata"]);
}

#[test]
fn sutra_1_2_16() {
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_ta(&["upa", "AN"], &yam, Lun, &["upAyata", "upAyaMsta"]);
}

#[test]
fn sutra_1_2_17() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_ta(&["upa"], &stha, Lun, &["upAsTita"]);
    assert_has_aataam(&["upa"], &stha, Lun, &["upAsTizAtAm"]);
    assert_has_jha(&["upa"], &stha, Lun, &["upAsTizata"]);

    assert_has_ta(&[], &d("qudA\\Y", Juhotyadi), Lun, &["adita"]);
    assert_has_ta(&[], &d("quDA\\Y", Juhotyadi), Lun, &["aDita"]);
}

#[test]
fn sutra_1_2_18() {
    assert_has_krdanta(&[], &d("devf~\\", Bhvadi), Krt::ktvA, &["devitvA"]);
    assert_has_krdanta(
        &[],
        &d("vftu~\\", Tudadi),
        Krt::ktvA,
        &["vartitvA", "vfttvA"],
    );
    // zew
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::ktvA, &["kftvA"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), Krt::ktvA, &["hftvA"]);
    // ktvA
    assert_has_krdanta(&["ni"], &d("graha~^", Kryadi), Krt::kta, &["nigfhIta"]);
    assert_has_krdanta(
        &["upa"],
        &d("zRi\\ha~", Divadi),
        Krt::kta,
        &["upasnigDa", "upasnIQa"],
    );
    assert_has_krdanta(&["ni"], &d("kuca~", Tudadi), Krt::kta, &["nikucita"]);
}

#[ignore]
#[test]
fn sutra_1_2_19() {
    use Krt::{kta, ktavatu};

    let shi = d("SIN", Adadi);
    assert_has_krdanta(&[], &shi, kta, &["Sayita"]);
    assert_has_krdanta(&[], &shi, ktavatu, &["Sayitavat"]);

    let svid = d("zvi\\dA~", Divadi);
    assert_has_krdanta(&["pra"], &svid, kta, &["prasvedita"]);
    assert_has_krdanta(&["pra"], &svid, ktavatu, &["prasveditavat"]);

    let mid = d("YimidA~\\", Bhvadi);
    assert_has_krdanta(&["pra"], &mid, kta, &["pramedita"]);
    assert_has_krdanta(&["pra"], &mid, ktavatu, &["prameditavat"]);

    let kshved = d("YikzvidA~", Bhvadi);
    assert_has_krdanta(&["pra"], &kshved, kta, &["prakzvedita"]);
    assert_has_krdanta(&["pra"], &kshved, ktavatu, &["prakzveditavat"]);

    let dharsh = d("YiDfzA~", Svadi);
    assert_has_krdanta(&["pra"], &dharsh, kta, &["praDarzita"]);
    assert_has_krdanta(&["pra"], &dharsh, ktavatu, &["praDarzitavat"]);

    // set?
    assert_has_krdanta(&[], &svid, kta, &["svinna"]);
    assert_has_krdanta(&[], &svid, ktavatu, &["svinnavat"]);
}

#[test]
fn sutra_1_2_20() {
    let mfz = d("mfza~^", Divadi);
    assert_has_krdanta(&[], &mfz, Krt::kta, &["mfzita", "marzita"]);
    assert_has_krdanta(&[], &mfz, Krt::ktavatu, &["mfzitavat", "marzitavat"]);
}

#[test]
fn sutra_1_2_22() {
    let pu = d("pUN", Bhvadi);
    assert_has_krdanta(&[], &pu, Krt::kta, &["pavita", "pUta"]);
    assert_has_krdanta(&[], &pu, Krt::ktavatu, &["pavitavat", "pUtavat"]);
    assert_has_krdanta(&[], &pu, Krt::ktvA, &["pavitvA", "pUtvA"]);
}

#[test]
fn sutra_1_2_23() {
    assert_has_krdanta(
        &[],
        &d("granTa~", Kryadi),
        Krt::ktvA,
        &["graTitvA", "granTitvA"],
    );
    assert_has_krdanta(
        &[],
        &d("SranTa~", Kryadi),
        Krt::ktvA,
        &["SraTitvA", "SranTitvA"],
    );
    assert_has_krdanta(
        &[],
        &d("gunPa~", Tudadi),
        Krt::ktvA,
        &["guPitvA", "gumPitvA"],
    );
    // nopaDa (but, see 1.2.26)
    assert_has_krdanta(&[], &d("riPa~", Tudadi), Krt::ktvA, &["rePitvA", "riPitvA"]);
    assert_has_krdanta(&[], &d("guPa~", Tudadi), Krt::ktvA, &["goPitvA", "guPitvA"]);
    // Ta-Pa-anta
    assert_has_krdanta(
        &[],
        &d("sransu~\\", Bhvadi),
        Krt::ktvA,
        &["sraMsitvA", "srastvA"],
    );
    assert_has_krdanta(
        &[],
        &d("Dvansu~\\", Bhvadi),
        Krt::ktvA,
        &["DvaMsitvA", "DvastvA"],
    );
}

#[test]
fn sutra_1_2_24() {
    assert_has_krdanta(
        &[],
        &d("vancu~", Bhvadi),
        Krt::ktvA,
        &["vacitvA", "vaYcitvA", "vaktvA"],
    );
    assert_has_krdanta(
        &[],
        &d("lunca~", Bhvadi),
        Krt::ktvA,
        &["lucitvA", "luYcitvA"],
    );
    assert_has_krdanta(
        &[],
        &d("fti", Bhvadi),
        Krt::ktvA,
        &["ftitvA", "artitvA", "ftIyitvA"],
    );
}

#[test]
fn sutra_1_2_25() {
    assert_has_krdanta(
        &[],
        &d("YitfzA~", Divadi),
        Krt::ktvA,
        &["tfzitvA", "tarzitvA"],
    );
    assert_has_krdanta(
        &[],
        &d("mfza~^", Divadi),
        Krt::ktvA,
        &["mfzitvA", "marzitvA"],
    );
    assert_has_krdanta(
        &[],
        &d("kfSa~", Divadi),
        Krt::ktvA,
        &["kfSitvA", "karSitvA"],
    );
}

#[test]
fn sutra_1_2_26() {
    let dyut = d("dyuta~\\", Bhvadi);
    assert_has_krdanta(&[], &dyut, Krt::ktvA, &["dyutitvA", "dyotitvA"]);
    assert_has_lat(&[], &san(&dyut), &["didyutizate", "didyotizate"]);
    let likh = d("liKa~", Tudadi);
    assert_has_krdanta(&[], &likh, Krt::ktvA, &["liKitvA", "leKitvA"]);
    assert_has_lat(&[], &san(&likh), &["liliKizati", "lileKizati"]);
    // ralaH
    let div = d("divu~", Divadi);
    assert_has_krdanta(&[], &div, Krt::ktvA, &["devitvA", "dyUtvA"]);
    assert_has_lat(&[], &san(&div), &["didevizati", "dudyUzati"]);
    // vyupaDAt
    let vft = d("vftu~\\", Bhvadi);
    assert_has_krdanta(&[], &vft, Krt::ktvA, &["vartitvA", "vfttvA"]);
    assert_has_lat(&[], &san(&vft), &["vivftsati", "vivartizate"]);
    // halAdeH
    let iz = d("izu~", Tudadi);
    assert_has_krdanta(&[], &iz, Krt::ktvA, &["ezitvA", "izwvA"]);
    assert_has_lat(&[], &san(&iz), &["ezizizati"]);
    // sew
    let bhuj = d("Bu\\ja~", Rudhadi);
    assert_has_krdanta(&[], &bhuj, Krt::ktvA, &["BuktvA"]);
    assert_has_ta(&[], &san(&bhuj), Lat, &["buBukzate"]);
}

#[test]
fn sutra_1_2_41() {
    let bhaj = d("Ba\\ja~^", Bhvadi);
    assert_has_upapada_krdanta(
        "Gfta",
        &[],
        &d("spf\\Sa~", Tudadi),
        Krt::kvin,
        &["GftaspfS"],
    );
    assert_has_upapada_krdanta("arDa", &[], &bhaj, Krt::Rvi, &["arDaBAj"]);
    assert_has_upapada_krdanta("pAda", &[], &bhaj, Krt::Rvi, &["pAdaBAj"]);
}

#[test]
fn sutra_1_2_42() {
    assert_has_karmadharaya("parama", "rAjya", &["paramarAjya"]);
    assert_has_karmadharaya("uttama", "rAjya", &["uttamarAjya"]);
    // TODO: others
}

#[test]
fn sutra_1_2_45() {
    assert_has_sup_1s("qitTa", Pum, &["qitTaH"]);
    assert_has_sup_1s("kapitTa", Pum, &["kapitTaH"]);
    assert_has_sup_1s("kuRqa", Napumsaka, &["kuRqam"]);
    assert_has_sup_1s("pIWa", Napumsaka, &["pIWam"]);
    // a-dhAtu?
    assert_has_lan(&[], &d("ha\\na~", Adadi), &["ahan"]);
    // TODO: others
}

#[test]
fn sutra_1_2_46() {
    let kr = d("qukf\\Y", Tanadi);
    let hr = d("hf\\Y", Bhvadi);

    // krt
    let karaka = create_krdanta("kAraka", &[], &kr, Krt::Rvul);
    assert_has_sup_1s(&karaka, Pum, &["kArakaH"]);
    let haraka = create_krdanta("hAraka", &[], &hr, Krt::Rvul);
    assert_has_sup_1s(&haraka, Pum, &["hArakaH"]);
    let kartr = create_krdanta("kartf", &[], &kr, Krt::tfc);
    assert_has_sup_1s(&kartr, Pum, &["kartA"]);
    let hartr = create_krdanta("hartf", &[], &hr, Krt::tfc);
    assert_has_sup_1s(&hartr, Pum, &["hartA"]);

    // taddhita
    let aupagava = create_taddhitanta("Opagava", "upagu", T::aR);
    assert_has_sup_1s(&aupagava, Pum, &["OpagavaH"]);
    let kapatava = create_taddhitanta("kApawava", "kapawu", T::aR);
    assert_has_sup_1s(&kapatava, Pum, &["kApawavaH"]);

    // samasa
    let rajapurusha = tatpurusha("rAjan", "puruza", Vibhakti::Sasthi);
    assert_has_sup_1s(&rajapurusha, Pum, &["rAjapuruzaH"]);
    let brahmanakambala = tatpurusha("brAhmaRa", "kambala", Vibhakti::Sasthi);
    assert_has_sup_1s(&brahmanakambala, Pum, &["brAhmaRakambalaH"]);
}

#[test]
fn sutra_1_2_47() {
    assert_has_sup_1s("atirE", Napumsaka, &["atiri"]);
    assert_has_sup_1s("atinO", Napumsaka, &["atinu"]);
    // napuMsake
    assert_has_sup_1s("grAmaRI", Pum, &["grAmaRIH"]);
    // prAtipadikasya
    assert_has_sup_1d("kuRqa", Napumsaka, &["kuRqe"]);
}
