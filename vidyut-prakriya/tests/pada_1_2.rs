extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

fn nic(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Nic])
}

fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San])
}

#[test]
fn sutra_1_2_1() {
    assert_has_lun_a(&["aDi"], &d("i\\N", Adadi), &["aDyagIzwa", "aDyEzwa"]);

    let kuw = d("kuwa~", Tudadi).with_antargana(Some(Antargana::Kutadi));
    assert_has_krdanta(&[], &kuw, Krt::tfc, &["kuwitf"]);
    assert_has_krdanta(&[], &kuw, Krt::tumun, &["kuwitum"]);
    assert_has_krdanta(&[], &kuw, Krt::tavya, &["kuwitavya"]);

    let puw = d("puwa~", Tudadi).with_antargana(Some(Antargana::Kutadi));
    assert_has_krdanta(&["ud"], &puw, Krt::tfc, &["utpuwitf"]);
    assert_has_krdanta(&["ud"], &puw, Krt::tumun, &["utpuwitum"]);
    assert_has_krdanta(&["ud"], &puw, Krt::tavya, &["utpuwitavya"]);

    let kuw_nic = nic(&kuw);
    assert_has_lat_p(&["ud"], &kuw_nic, &["utkowayati"]);
    assert_has_lit_p(&["ud"], &kuw, &["uccukowa"]);
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
    assert_has_parasmai_tinanta(&[], &kf, Lat, Prathama, Dvi, &["kurutaH"]);
    assert_has_parasmai_tinanta(&[], &kf, Lat, Prathama, Bahu, &["kurvanti"]);

    let ci = d("ciY", Svadi);
    assert_has_parasmai_tinanta(&[], &ci, Lat, Prathama, Dvi, &["cinutaH"]);
    assert_has_parasmai_tinanta(&[], &ci, Lat, Prathama, Bahu, &["cinvanti"]);

    // sArvadhAtukam
    assert_has_krdanta(&[], &kf, Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &kf, Krt::tumun, &["kartum"]);
    assert_has_krdanta(&[], &kf, Krt::tavya, &["kartavya"]);
    // apit
    assert_has_parasmai_tinanta(&[], &kf, Lat, Prathama, Eka, &["karoti"]);
    assert_has_parasmai_tinanta(&[], &kf, Lat, Madhyama, Eka, &["karozi"]);
    assert_has_parasmai_tinanta(&[], &kf, Lat, Uttama, Eka, &["karomi"]);
}

#[test]
fn sutra_1_2_5() {
    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_parasmai_tinanta(&[], &bhid, Lit, Prathama, Dvi, &["biBidatuH"]);
    assert_has_parasmai_tinanta(&[], &bhid, Lit, Prathama, Bahu, &["biBiduH"]);

    let chid = d("Ci\\di~^r", Rudhadi);
    assert_has_parasmai_tinanta(&[], &chid, Lit, Prathama, Dvi, &["cicCidatuH"]);
    assert_has_parasmai_tinanta(&[], &chid, Lit, Prathama, Bahu, &["cicCiduH"]);

    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_parasmai_tinanta(&[], &yaj, Lit, Prathama, Dvi, &["IjatuH"]);
    assert_has_parasmai_tinanta(&[], &yaj, Lit, Prathama, Bahu, &["IjuH"]);

    // asaMyogAt
    assert_has_lit(&[], &d("sransu~\\", Bhvadi), &["sasraMse"]);
    assert_has_lit(&[], &d("Dvansu~\\", Bhvadi), &["daDvaMse"]);
    // apit
    assert_has_parasmai_tinanta(&[], &bhid, Lit, Madhyama, Eka, &["biBediTa"]);
}

#[test]
fn sutra_1_2_6() {
    let bhu = d("BU", Bhvadi);
    assert_has_parasmai_tinanta(&[], &bhu, Lit, Prathama, Eka, &["baBUva"]);
    assert_has_parasmai_tinanta(&[], &bhu, Lit, Madhyama, Eka, &["baBUviTa"]);
}

#[test]
fn sutra_1_2_6_v1() {
    let sranth = d("SranTa~", Kryadi);
    assert_has_tinanta(
        &[],
        &sranth,
        Lit,
        Prathama,
        Dvi,
        &["SreTatuH", "SaSranTatuH"],
    );
    assert_has_tinanta(&[], &sranth, Lit, Prathama, Bahu, &["SreTuH", "SaSranTuH"]);

    let granth = d("granTa~", Kryadi);
    assert_has_tinanta(
        &[],
        &granth,
        Lit,
        Prathama,
        Dvi,
        &["greTatuH", "jagranTatuH"],
    );
    assert_has_tinanta(&[], &granth, Lit, Prathama, Bahu, &["greTuH", "jagranTuH"]);

    let danbh = d("danBu~", Svadi);
    assert_has_tinanta(&[], &danbh, Lit, Prathama, Bahu, &["deBuH", "dadamBuH"]);

    let svaj = d("zva\\nja~\\", Bhvadi);
    assert_has_tinanta(
        &["pari"],
        &svaj,
        Lit,
        Prathama,
        Eka,
        &["parizasvaje", "parizasvaYje", "parisasvaje", "parisasvaYje"],
    );
    assert_has_tinanta(
        &["pari"],
        &svaj,
        Lit,
        Prathama,
        Dvi,
        &[
            "parizasvajAte",
            "parizasvaYjAte",
            "parisasvajAte",
            "parisasvaYjAte",
        ],
    );
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
    assert_has_lat_p(&[], &san(&rud), &["rurudizati"]);

    let vid = d("vida~", Adadi);
    assert_has_krdanta(&[], &vid, Krt::ktvA, &["viditvA"]);
    assert_has_lat_p(&[], &san(&vid), &["vividizati"]);

    let mus = d("muza~", Kryadi);
    assert_has_krdanta(&[], &mus, Krt::ktvA, &["muzitvA"]);
    assert_has_lat_p(&[], &san(&mus), &["mumuzizati"]);

    let grah = d("graha~^", Kryadi);
    assert_has_krdanta(&[], &grah, Krt::ktvA, &["gfhItvA"]);
    assert_has_lat_p(&[], &san(&grah), &["jiGfkzati"]);

    let svap = d("Yizva\\pa~", Adadi);
    assert_has_krdanta(&[], &svap, Krt::ktvA, &["suptvA"]);
    assert_has_lat_p(&[], &san(&svap), &["suzupsati"]);

    let prach = d("pra\\Ca~", Tudadi);
    assert_has_krdanta(&[], &prach, Krt::ktvA, &["pfzwvA"]);
    assert_has_lat_p(&[], &san(&prach), &["pipfcCizati"]);
}

#[ignore]
#[test]
fn sutra_1_2_9() {
    assert_has_lat_p(&[], &san(&d("ci\\Y", Svadi)), &["cicIzati", "cikIzati"]);
    assert_has_lat_p(&[], &san(&d("zwu\\Y", Adadi)), &["tuzwUzati"]);
    assert_has_lat_p(&[], &san(&d("qukf\\Y", Tanadi)), &["cikIrzati"]);
    assert_has_lat_p(&[], &san(&d("pA\\", Bhvadi)), &["pipAsati"]);
    assert_has_lat_p(&[], &san(&d("zWA\\", Bhvadi)), &["tizWAsati"]);
    assert_has_lat_a(&[], &san(&d("SIN", Adadi)), &["SiSayizate"]);
    assert_has_lat_p(&[], &san(&d("jYapa~", Curadi)), &["jYIpsati"]);
}

#[ignore]
#[test]
fn sutra_1_2_10() {
    assert_has_lat_p(&[], &san(&d("Bi\\di~^r", Rudhadi)), &["biBitsati"]);
    assert_has_lat_a(&[], &san(&d("bu\\Da~\\", Divadi)), &["buButsate"]);
    // ikaH
    assert_has_lat_a(&[], &san(&d("ya\\ja~^", Bhvadi)), &["yiyakzate"]);
    // Jal
    assert_has_lat_a(&[], &san(&d("vftu~\\", Bhvadi)), &["vivartizate"]);
    // danB also in scope
    assert_has_lat_p(&[], &san(&d("danBu~", Svadi)), &["DIpsati", "Dipsati"]);
}

#[test]
fn sutra_1_2_11() {
    let bhid = d("Bi\\di~^r", Rudhadi);
    let budh = d("bu\\Da~\\", Divadi);
    assert_has_ashirlin_a(&[], &bhid, &["BitsIzwa"]);
    assert_has_ashirlin_a(&[], &budh, &["ButsIzwa"]);
    // sic
    assert_has_lun_a(&[], &bhid, &["aBitta"]);
    assert_has_lun_a(&[], &budh, &["abudDa"]);
    // ikaH
    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_ashirlin_a(&[], &yaj, &["yakzIzwa"]);
    assert_has_lun_a(&[], &yaj, &["ayazwa"]);
    // Atmanepadezu
    assert_has_lun_p(&[], &d("sf\\ja~", Tudadi), &["asrAkzIt"]);
    assert_has_lun_p(&[], &d("df\\Si~r", Bhvadi), &["adrAkzIt", "adarSat"]);
    // halantAt
    let ci = d("ci\\Y", Svadi);
    assert_has_ashirlin_a(&[], &ci, &["cezIzwa"]);
    assert_has_lun_a(&[], &ci, &["acezwa"]);
    // Jali
    let vft = d("vftu~\\", Bhvadi);
    assert_has_ashirlin_a(&[], &vft, &["vartizIzwa"]);
    assert_has_lun_a(&[], &vft, &["avartizwa"]);
    // liNsicO
    let dviz = d("dvi\\za~^", Adadi);
    assert_has_krdanta(&[], &dviz, Krt::tfc, &["dvezwf"]);
    assert_has_lrt_a(&[], &dviz, &["dvekzyate"]);
}

#[test]
fn sutra_1_2_12() {
    let kf = d("qukf\\Y", Tanadi);
    let hf = d("hf\\Y", Bhvadi);
    assert_has_ashirlin_a(&[], &kf, &["kfzIzwa"]);
    assert_has_ashirlin_a(&[], &hf, &["hfzIzwa"]);
    // sicaH
    assert_has_lun_a(&[], &kf, &["akfta"]);
    assert_has_lun_a(&[], &hf, &["ahfta"]);
    // Jali
    let vf = d("vfY", Svadi);
    assert_has_ashirlin_a(&[], &vf, &["varizIzwa", "vfzIzwa"]);
    assert_has_lun_a(&[], &vf, &["avarizwa", "avarIzwa", "avfta"]);
}

#[test]
fn sutra_1_2_13() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_ashirlin_a(&["sam"], &gam, &["saNgaMsIzwa", "saNgasIzwa"]);
    // sicaH
    assert_has_lun_a(&["sam"], &gam, &["samagaMsta", "samagata"]);
}

#[test]
fn sutra_1_2_14() {
    let han = d("ha\\na~", Adadi);
    assert_has_atmane_tinanta(&["AN"], &han, Lun, Prathama, Eka, &["Ahata", "AvaDizwa"]);
    assert_has_atmane_tinanta(
        &["AN"],
        &han,
        Lun,
        Prathama,
        Dvi,
        &["AhasAtAm", "AvaDizAtAm"],
    );
    assert_has_atmane_tinanta(
        &["AN"],
        &han,
        Lun,
        Prathama,
        Bahu,
        &["Ahasata", "AvaDizata"],
    );
}

#[test]
fn sutra_1_2_15() {
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_atmane_tinanta(
        &["ud", "AN"],
        &yam,
        Lun,
        Prathama,
        Eka,
        &["udAyata", "udAyaMsta"],
    );
    assert_has_atmane_tinanta(
        &["ud", "AN"],
        &yam,
        Lun,
        Prathama,
        Dvi,
        &["udAyasAtAm", "udAyaMsAtAm"],
    );
    assert_has_atmane_tinanta(
        &["ud", "AN"],
        &yam,
        Lun,
        Prathama,
        Bahu,
        &["udAyasata", "udAyaMsata"],
    );
}

#[test]
fn sutra_1_2_17() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_atmane_tinanta(&["upa"], &stha, Lun, Prathama, Eka, &["upAsTita"]);
    assert_has_atmane_tinanta(&["upa"], &stha, Lun, Prathama, Dvi, &["upAsTizAtAm"]);
    assert_has_atmane_tinanta(&["upa"], &stha, Lun, Prathama, Bahu, &["upAsTizata"]);

    assert_has_lun_a(&[], &d("qudA\\Y", Juhotyadi), &["adita"]);
    assert_has_lun_a(&[], &d("quDA\\Y", Juhotyadi), &["aDita"]);
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
    assert_has_lat(&[], &san(&vft), &["vivartizate"]);
    // halAdeH
    let iz = d("izu~", Tudadi);
    assert_has_krdanta(&[], &iz, Krt::ktvA, &["ezitvA", "izwvA"]);
    assert_has_lat(&[], &san(&iz), &["ezizizati"]);
    // sew
    let bhuj = d("Bu\\ja~", Rudhadi);
    assert_has_krdanta(&[], &bhuj, Krt::ktvA, &["BuktvA"]);
    assert_has_lat_a(&[], &san(&bhuj), &["buBukzate"]);
}

#[test]
fn sutra_1_2_47() {
    use Vibhakti::*;
    assert_has_subantas("atirE", Napumsaka, Prathama, Eka, &["atiri"]);
    assert_has_subantas("atinO", Napumsaka, Prathama, Eka, &["atinu"]);
    // napuMsake
    assert_has_subantas("grAmaRI", Pum, Prathama, Eka, &["grAmaRIH"]);
    // prAtipadikasya
    assert_has_subantas("kuRqa", Napumsaka, Prathama, Dvi, &["kuRqe"]);
}
