extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Dhatu;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Sanadi;

#[test]
fn sk_2563() {
    let cur = d("cura~", Curadi);
    assert_has_tip(&[], &cur, Lat, &["corayati"]);
}

#[test]
fn sk_2564() {
    let cur = d("cura~", Curadi);
    assert_has_ta(&[], &cur, Lat, &["corayate"]);
    assert_has_tip(
        &[],
        &cur,
        Lit,
        &["corayAYcakAra", "corayAmbaBUva", "corayAmAsa"],
    );
    assert_has_tip(&[], &cur, Lut, &["corayitA"]);
    assert_has_tip(&[], &cur, AshirLin, &["coryAt"]);
    assert_has_ta(&[], &cur, AshirLin, &["corayizIzwa"]);
    assert_has_tip(&[], &cur, Lun, &["acUcurat"]);
    assert_has_ta(&[], &cur, Lun, &["acUcurata"]);

    let cint = d("citi~", Curadi);
    assert_has_tip(&[], &cint, Lat, &["cintayati", "cintati"]);
    assert_has_tip(&[], &cint, Lun, &["acicintat", "acintIt"]);
    assert_has_tip(&[], &cint, AshirLin, &["cintyAt"]);
    assert_has_ta_k(&[], &cint, Lat, &["cintyate"]);
    assert_has_tip(&[], &cint, VidhiLin, &["cintayet", "cintet"]);

    // TODO: jagARa not capturable right now.

    let yantr = d("yatri~", Curadi);
    assert_has_tip(&[], &yantr, Lat, &["yantrayati", "yantrati"]);

    let sphund = d("sPuqi~", Curadi);
    assert_has_tip(&[], &sphund, Lat, &["sPuRqayati", "sPuRqati"]);

    let sphunt = d("sPuwi~", Curadi);
    assert_has_tip(&[], &sphunt, Lat, &["sPuRwayati", "sPuRwati"]);

    // "evaM kudritatrimatrizu" for "kundrati"
    let kundr = d("kudri~", Curadi);
    assert_has_tip(&[], &kundr, Lat, &["kundrayati", "kundrati"]);

    // "evaM kudritatrimatrizu" for "kundrati"
    let lad = d("laqa~", Curadi);
    assert_has_tip(&[], &lad, Lat, &["lAqayati"]);

    let mind = d("midi~", Curadi);
    assert_has_tip(&[], &mind, Lat, &["mindayati", "mindati"]);

    let oland = d("olaqi~", Curadi);
    assert_has_tip(&[], &oland, Lat, &["olaRqayati", "olaRqati"]);

    let land = d("o~laqi~", Curadi);
    assert_has_tip(&[], &land, Lat, &["laRqayati", "laRqati"]);

    let uland = d("ulaqi~", Curadi);
    assert_has_tip(&[], &uland, Lat, &["ulaRqayati", "ulaRqati"]);

    let pid = d("pIqa~", Curadi);
    assert_has_tip(&[], &pid, Lat, &["pIqayati"]);
}

#[test]
fn sk_2565() {
    let pid = d("pIqa~", Curadi);
    assert_has_tip(&[], &pid, Lun, &["apipIqat", "apIpiqat"]);

    let badh = d("baDa~", Curadi);
    assert_has_tip(&[], &badh, Lat, &["bADayati"]);

    let pr = d("pF", Curadi);
    assert_has_tip(&[], &pr, Lat, &["pArayati", "parati"]);

    let prath = d("praTa~", Curadi);
    assert_has_tip(&[], &prath, Lat, &["prATayati"]);
}

#[test]
fn sk_2566() {
    let prath = d("praTa~", Curadi);
    assert_has_tip(&[], &prath, Lun, &["apapraTat"]);

    let prth = d("pfTa~", Curadi);
    assert_has_tip(&[], &prth, Lat, &["parTayati"]);
}

#[test]
fn sk_2567() {
    let prth = d("pfTa~", Curadi);
    assert_has_tip(&[], &prth, Lun, &["apIpfTat", "apaparTat"]);

    let path = d("paTa~", Curadi);
    assert_has_tip(&[], &path, Lat, &["pATayati"]);

    let sanb = d("zanba~", Curadi);
    assert_has_tip(&[], &sanb, Lat, &["sambayati"]);
    assert_has_tip(&[], &sanb, Lun, &["asasambat"]);

    let shanb = d("Sanba~", Curadi);
    assert_has_tip(&[], &shanb, Lun, &["aSaSambat"]);

    let kutt = d("kuwwa~", Curadi);
    assert_has_tip(&[], &kutt, Lat, &["kuwwayati"]);

    // "ayaM dopaDaH"
    let att = d("adwa~", Curadi);
    assert_has_tip(&[], &att, Lat, &["awwayati"]);
    assert_has_tip(&[], &att, Lun, &["Awwiwat"]);

    let lunth = d("lunWa~", Curadi);
    assert_has_tip(&[], &lunth, Lat, &["luRWayati"]);
    assert_has_tip(&[], &d("luWi~", Bhvadi), Lat, &["luRWati"]);

    let tunj = d("tuji~", Curadi);
    assert_has_tip(&[], &tunj, Lat, &["tuYjayati", "tuYjati"]);

    let pinj = d("piji~", Curadi);
    assert_has_tip(&[], &pinj, Lat, &["piYjayati", "piYjati"]);

    let pis = d("pisa~", Curadi);
    assert_has_tip(&[], &pis, Lat, &["pesayati"]);
    assert_has_tip(&[], &d("pisf~", Bhvadi), Lat, &["pesati"]);

    let smit = d("smiwa~", Curadi);
    assert_has_tip(&[], &smit, Lun, &["asismiwat"]);

    let smi = d("zmiN", Curadi);
    assert_has_tip(&[], &smi, Lat, &[]);
    assert_has_ta(&[], &smi, Lat, &["smAyayate"]);

    let panth = d("paTi~", Curadi);
    assert_has_tip(&[], &panth, Lat, &["panTayati", "panTati"]);

    let chand = d("Cadi~", Curadi);
    assert_has_tip(&[], &chand, Lat, &["Candayati", "Candati"]);

    let shran = d("SraRa~", Curadi);
    assert_has_tip(&["vi"], &shran, Lat, &["viSrARayati"]);

    let tad = d("taqa~", Curadi);
    assert_has_tip(&[], &tad, Lat, &["tAqayati"]);

    let khad = d("Kaqa~", Curadi);
    assert_has_tip(&[], &khad, Lat, &["KAqayati"]);

    let khand = d("Kaqi~", Curadi);
    assert_has_tip(&[], &khand, Lat, &["KaRqayati", "KaRqati"]);

    let kand = d("kaqi~", Curadi);
    assert_has_tip(&[], &kand, Lat, &["kaRqayati", "kaRqati"]);

    let kunth = d("kuWi~", Curadi);
    assert_has_tip(&["ava"], &kunth, Lat, &["avakuRWayati", "avakuRWati"]);

    let nakk = d("nakka~", Curadi);
    assert_has_tip(&["pra"], &nakk, Lat, &["pranakkayati"]);

    let tul = d("tula~", Curadi);
    assert_has_tip(&[], &tul, Lat, &["tolayati"]);
    assert_has_tip(
        &[],
        &tul,
        Lit,
        &["tolayAYcakAra", "tolayAmAsa", "tolayAmbaBUva"],
    );
    assert_has_tip(&[], &tul, Lun, &["atUtulat"]);

    // TODO: tulayati, tulanA

    let dul = d("dula~", Curadi);
    assert_has_tip(&[], &dul, Lat, &["dolayati"]);
    assert_has_tip(
        &[],
        &dul,
        Lit,
        &["dolayAYcakAra", "dolayAmAsa", "dolayAmbaBUva"],
    );
    assert_has_tip(&[], &dul, Lun, &["adUdulat"]);

    let kal = d("kala~", Curadi);
    assert_has_tip(&[], &kal, Lat, &["kAlayati"]);

    let pand = d("paqi~", Curadi);
    assert_has_tip(&[], &pand, Lat, &["paRqayati", "paRqati"]);

    let pans = d("pasi~", Curadi);
    assert_has_tip(&[], &pans, Lat, &["paMsayati", "paMsati"]);

    let camp = d("capi~", Curadi);
    assert_has_tip(&[], &camp, Lat, &["campayati", "campati"]);

    let kshamp = d("kzapi~", Curadi);
    assert_has_tip(&[], &kshamp, Lat, &["kzampayati", "kzampati"]);
}

#[test]
fn skip_sk_2568() {
    let jnap = d("jYapa~", Curadi);
    assert_has_tip(&[], &jnap, Lat, &["jYapayati"]);

    let yam = d("yama~", Curadi);
    assert_has_tip(&[], &yam, Lat, &["yamayati"]);

    let cah = d("caha~", Curadi);
    assert_has_tip(&[], &cah, Lat, &["cahayati"]);
    assert_has_tip(&[], &cah, Lun, &["acIcahat"]);
    // TODO: acacahat ?

    let cap = d("capa~", Curadi);
    assert_has_tip(&[], &cap, Lat, &["capayati"]);

    let rah = d("raha~", Curadi);
    assert_has_tip(&[], &rah, Lun, &["arIrahat"]);
    // TODO: ararahat?

    let bal = d("bala~", Curadi);
    assert_has_tip(&[], &bal, Lat, &["balayati"]);
}

#[test]
fn skip_sk_2569() {}

#[test]
fn skip_sk_2570() {
    let ci = d("ciY", Curadi);
    assert_has_tip(&[], &ci, Lat, &["capayati", "cayayati", "cayati"]);
    assert_has_ta(&[], &ci, Lat, &["capayate", "cayayate", "cayate"]);
    assert_has_tip(
        &["pra", "ni"],
        &ci,
        Lat,
        &[
            "pranicapayati",
            "praRicapayati",
            "pranicayayati",
            "praRicayayati",
            "pranicayati",
            "praRicayati",
        ],
    );

    let tank = d("waki~", Curadi);
    assert_has_tip(&[], &tank, Lat, &["waNkayati", "waNkati"]);

    let dhus = d("DUsa~", Curadi);
    assert_has_tip(&[], &dhus, Lat, &["DUsayati"]);

    let shunt = d("SuWi~", Curadi);
    assert_has_tip(&[], &shunt, Lat, &["SuRWayati", "SuRWati"]);

    let gaj = d("gaja~", Curadi);
    assert_has_tip(&[], &gaj, Lat, &["gAjayati"]);

    let marj = d("mArja~", Curadi);
    assert_has_tip(&[], &marj, Lat, &["mArjayati"]);

    let marc = d("marca~", Curadi);
    assert_has_tip(&[], &marc, Lat, &["marcayati"]);

    let panc = d("paci~", Curadi);
    assert_has_tip(&[], &panc, Lat, &["paYcayati", "paYcati"]);
    // no paYcate for paci~ (curAdi)
    assert_has_ta(&[], &panc, Lat, &["paYcayate"]);
    assert_has_ta(&[], &d("paci~\\", Bhvadi), Lat, &["paYcate"]);

    let tij = d("tija~", Curadi);
    assert_has_tip(&[], &tij, Lat, &["tejayati"]);
}

#[test]
fn sk_2571() {
    let kirt = d("kFta~", Curadi);
    assert_has_tip(&[], &kirt, Lat, &["kIrtayati"]);
    assert_has_tip(&[], &kirt, Lun, &["acIkftat", "acikIrtat"]);

    let kumb = d("kubi~", Curadi);
    assert_has_tip(&[], &kumb, Lat, &["kumbayati", "kumbati"]);

    let il = d("ila~", Curadi);
    assert_has_tip(&[], &il, Lat, &["elayati"]);
    assert_has_tip(&[], &il, Lun, &["Elilat"]);

    let jans = d("jasi~", Curadi);
    assert_has_tip(&[], &jans, Lat, &["jaMsayati", "jaMsati"]);

    // start of AkusmIya

    let cit = d_akusmiya("cita~", Curadi);
    assert_has_lat(&[], &cit, &["cetayate"]);
    assert_has_lun(&[], &cit, &["acIcitata"]);

    let dansh = d_akusmiya("daSi~", Curadi);
    assert_has_tip(&[], &dansh, Lat, &["daMSati"]);

    let dans = d_akusmiya("dasi~", Curadi);
    assert_has_ta(&[], &dans, Lat, &["daMsayate"]);
    assert_has_tip(&[], &dans, Lat, &["daMsati"]);

    let tantr = d_akusmiya("tatri~", Curadi);
    assert_has_ta(&[], &tantr, Lat, &["tantrayate"]);

    let bast = d_akusmiya("basta~", Curadi);
    assert_has_ta(&[], &bast, Lat, &["bastayate"]);

    let gandh = d_akusmiya("ganDa~", Curadi);
    assert_has_ta(&[], &gandh, Lat, &["ganDayate"]);

    let sham = d_akusmiya("Sama~", Curadi);
    assert_has_ta(&[], &sham, Lat, &["SAmayate"]);

    let vrsh = d_akusmiya("vfza~", Curadi);
    assert_has_ta(&[], &vrsh, Lat, &["varzayate"]);

    let mad = d_akusmiya("mada~", Curadi);
    assert_has_ta(&[], &mad, Lat, &["mAdayate"]);

    let gr = d_akusmiya("gf", Curadi);
    assert_has_ta(&[], &gr, Lat, &["gArayate"]);

    let vid = d_akusmiya("vida~", Curadi);
    assert_has_ta(&[], &vid, Lat, &["vedayate"]);
    // TODO: vindati?
    assert_has_tip(&[], &d("vi\\dx~^", Tudadi), Lat, &["vindati"]);

    let man = d_akusmiya("mAna~", Curadi);
    assert_has_ta(&[], &man, Lat, &["mAnayate"]);

    let yu = d_akusmiya("yu", Curadi);
    assert_has_ta(&[], &yu, Lat, &["yAvayate"]);

    let kusm = d_akusmiya("kusma~", Curadi);
    assert_has_ta(&[], &kusm, Lat, &["kusmayate"]);
    assert_has_ta(&[], &kusm, Lun, &["acukusmata"]);

    // end of AkusmIya

    let shabd = d("Sabda~", Curadi);
    assert_has_tip(&["prati"], &shabd, Lat, &["pratiSabdayati"]);
    assert_has_tip(&[], &shabd, Lat, &["Sabdayati"]);

    let kan = d("kaRa~", Curadi);
    assert_has_tip(&[], &kan, Lat, &["kARayati"]);
    assert_has_tip(&[], &kan, Lun, &["acIkaRat", "acakARat"]);

    let jambh = d("jaBi~", Curadi);
    assert_has_tip(&[], &jambh, Lat, &["jamBayati", "jamBati"]);

    let sud = d("zUda~", Curadi);
    assert_has_tip(&[], &sud, Lat, &["sUdayati"]);
    assert_has_tip(&[], &sud, Lun, &["asUzudat"]);

    let jas = d("jasu~", Curadi);
    assert_has_tip(&[], &jas, Lat, &["jAsayati", "jasati"]);

    let pas = d("paSa~", Curadi);
    assert_has_tip(&[], &pas, Lat, &["pASayati"]);

    let am = d("ama~", Curadi);
    assert_has_tip(&[], &am, Lat, &["Amayati"]);
    assert_has_tip(&["AN"], &nic(&d("ama~", Bhvadi)), Lat, &["Amayati"]);

    assert_has_ta(&[], &d("sPuwa~\\", Bhvadi), Lat, &["sPowate"]);
    assert_has_tip(&[], &d("sPuwa~", Tudadi), Lat, &["sPuwati"]);

    let ghat = d("Gawa~", Curadi);
    assert_has_tip(&[], &ghat, Lat, &["GAwayati"]);

    let div = d("divu~", Curadi);
    assert_has_tip(&[], &div, Lat, &["devayati", "devati"]);

    let arj = d("arja~", Curadi);
    assert_has_tip(&[], &arj, Lat, &["arjayati"]);

    let ghush = d("Guzi~r", Curadi);
    assert_has_tip(&[], &ghush, Lat, &["Gozayati", "Gozati"]);
    assert_has_tip(&[], &ghush, Lun, &["aGuzat", "aGozIt", "ajUGuzat"]);

    // TODO: curAdi or bhvAdi?
    let krand = d("kranda~", Curadi);
    assert_has_tip(&["AN"], &krand, Lat, &["Akrandayati"]);

    let tans = d("tasi~", Curadi);
    assert_has_tip(&["ava"], &tans, Lat, &["avataMsayati", "avataMsati"]);

    let bhush = d("BUza~", Curadi);
    assert_has_tip(&[], &bhush, Lat, &["BUzayati"]);

    let jna = d("jYA", Curadi);
    assert_has_tip(&["AN"], &jna, Lat, &["AjYApayati"]);

    // aSarDIt by udit --> Ric-vikalpa
    let shrd = d("SfDu~", Curadi);
    assert_has_tip(&[], &shrd, Lun, &["aSaSarDat", "aSISfDat", "aSarDIt"]);

    let anc = d("ancu~", Curadi);
    assert_has_tip(&[], &anc, Lat, &["aYcayati", "aYcati"]);

    let ling = d("ligi~", Curadi);
    assert_has_tip(&[], &ling, Lat, &["liNgayati", "liNgati"]);

    let mud = d("muda~", Curadi);
    assert_has_tip(&[], &mud, Lat, &["modayati"]);

    let dhras = d("u~Drasa~", Curadi);
    assert_has_tip(&[], &dhras, Lat, &["DrAsayati", "Drasati"]);

    let udhras = d("uDrasa~", Curadi);
    assert_has_tip(&[], &udhras, Lat, &["uDrAsayati"]);

    let cyu = d("cyu", Curadi);
    assert_has_tip(&[], &cyu, Lat, &["cyAvayati"]);

    let cyus = d("cyusa~", Curadi);
    assert_has_tip(&[], &cyus, Lat, &["cyosayati"]);

    let bhu = d("BU", Curadi);
    assert_has_tip(&[], &bhu, Lat, &["BAvayati"]);

    let krp = d("kfpa~", Curadi);
    assert_has_tip(&[], &krp, Lat, &["kalpayati"]);

    let gras = d("grasa~", Curadi);
    assert_has_tip(&[], &gras, Lat, &["grAsayati"]);

    let push = d("puza~", Curadi);
    assert_has_tip(&[], &push, Lat, &["pozayati"]);

    let dal = d("dala~", Curadi);
    assert_has_tip(&[], &dal, Lat, &["dAlayati"]);

    let pat = d("pawa~", Curadi);
    assert_has_tip(&[], &pat, Lat, &["pAwayati"]);

    let put = d("puwa~", Curadi);
    assert_has_tip(&[], &put, Lat, &["powayati"]);

    let lut = d("luwa~", Curadi);
    assert_has_tip(&[], &lut, Lat, &["lowayati"]);

    let tunj = d("tuji~", Curadi);
    assert_has_tip(&[], &tunj, Lat, &["tuYjayati", "tuYjati"]);

    let pinj = d("piji~", Curadi);
    assert_has_tip(&[], &pinj, Lat, &["piYjayati", "piYjati"]);

    let ghat = d("Gawa~", Curadi);
    assert_has_tip(&[], &ghat, Lat, &["GAwayati"]);

    let ghant = d("Gawi~", Curadi);
    assert_has_tip(&[], &ghant, Lat, &["GaRwayati", "GaRwati"]);
}

// TODO: this test needs more work, but it covers enough important cases that it's not disabled.
#[test]
fn sk_2572() {
    let lok = d("lokf~", Curadi);
    assert_has_tip(&[], &lok, Lun, &["alulokat"]);

    let loc = d("locf~", Curadi);
    assert_has_tip(&[], &loc, Lun, &["alulocat"]);

    let vrt = d("vftu~", Curadi);
    assert_has_tip(&[], &vrt, Lat, &["vartayati", "vartati"]);

    let vrdh = d("vfDu~", Curadi);
    assert_has_tip(&[], &vrdh, Lat, &["varDayati", "varDati"]);

    let pur = d("pUrI~", Curadi);
    assert_has_tip(&[], &pur, Lat, &["pUrayati", "pUrati"]);

    let svad = d("zvada~", Curadi);
    assert_has_tip(&[], &svad, Lun, &["asizvadat"]);

    let svaad = d("svAda~", Curadi);
    assert_has_tip(&[], &svaad, Lun, &["asisvadat"]);

    // start of ADfzIya

    let yuj = d_adhrshiya("yu\\ja~", Curadi);
    assert_has_tip(&[], &yuj, Lat, &["yojayati", "yojati"]);
    assert_has_tip(&[], &yuj, Lun, &["ayUyujat", "ayOkzIt"]);

    let prc = d_adhrshiya("pfca~", Curadi);
    assert_has_tip(&[], &prc, Lat, &["parcayati", "parcati"]);
    assert_has_tip(&[], &prc, Lut, &["parcayitA", "parcitA"]);
    // first two are not in SK.
    assert_has_tip(&[], &prc, Lun, &["apIpfcat", "apaparcat", "aparcIt"]);

    let sah = d_adhrshiya("zaha~", Curadi);
    assert_has_tip(&[], &sah, Lat, &["sAhayati", "sahati"]);

    let li = d_adhrshiya("lI\\", Curadi);
    assert_has_tip(&[], &li, Lat, &["lAyayati", "layati"]);
    assert_has_tip(&[], &li, Lut, &["lAyayitA", "letA"]);

    let varj = d_adhrshiya("vfjI~", Curadi);
    assert_has_tip(&[], &varj, Lat, &["varjayati", "varjati"]);

    let vr = d_adhrshiya("vfY", Curadi);
    assert_has_tip(&[], &vr, Lat, &["vArayati", "varati"]);
    assert_has_ta(&[], &vr, Lat, &["vArayate", "varate"]);
    assert_has_tip(&[], &vr, Lut, &["vArayitA", "varitA", "varItA"]);

    let jr = d_adhrshiya("jF", Curadi);
    assert_has_tip(&[], &jr, Lat, &["jArayati", "jarati"]);
    assert_has_tip(&[], &jr, Lut, &["jArayitA", "jaritA", "jarItA"]);

    let jri = d_adhrshiya("jri\\", Curadi);
    assert_has_tip(&[], &jri, Lat, &["jrAyayati", "jrayati"]);
    assert_has_tip(&[], &jri, Lut, &["jrAyayitA", "jretA"]);

    let ric = d_adhrshiya("ri\\ca~", Curadi);
    assert_has_tip(&[], &ric, Lat, &["recayati", "recati"]);
    assert_has_tip(&[], &ric, Lut, &["recayitA", "rektA"]);

    let shish = d_adhrshiya("Si\\za~", Curadi);
    assert_has_tip(&[], &shish, Lat, &["Sezayati", "Sezati"]);
    assert_has_tip(&[], &shish, Lut, &["SezayitA", "SezwA"]);
    assert_has_tip(&[], &shish, Lun, &["aSISizat", "aSikzat"]);

    let tap = d_adhrshiya("ta\\pa~", Curadi);
    assert_has_tip(&[], &tap, Lat, &["tApayati", "tapati"]);
    assert_has_tip(&[], &tap, Lut, &["tApayitA", "taptA"]);

    let trp = d_adhrshiya("tfpa~", Curadi);
    assert_has_tip(&[], &trp, Lat, &["tarpayati", "tarpati"]);
    assert_has_tip(&[], &trp, Lut, &["tarpayitA", "tarpitA"]);

    let chrd = d_adhrshiya("CfdI~", Curadi);
    assert_has_tip(&[], &chrd, Lat, &["Cardayati", "Cardati"]);
    assert_has_tip(&[], &chrd, Lut, &["CardayitA", "CarditA"]);
    assert_has_tip(&[], &chrd, Lrt, &["Cardayizyati", "Cardizyati"]);

    let carp = d_adhrshiya("cfpa~", Curadi);
    assert_has_tip(&[], &carp, Lat, &["carpayati", "carpati"]);

    let charp = d_adhrshiya("Cfpa~", Curadi);
    assert_has_tip(&[], &charp, Lat, &["Carpayati", "Carpati"]);

    let drbh = d_adhrshiya("dfBI~", Curadi);
    assert_has_tip(&[], &drbh, Lat, &["darBayati", "darBati"]);
    assert_has_tip(&[], &drbh, Lut, &["darBayitA", "darBitA"]);

    let mi = d_adhrshiya("mI\\", Curadi);
    assert_has_tip(&[], &mi, Lat, &["mAyayati", "mayati"]);
    assert_has_tip(&[], &mi, Lut, &["mAyayitA", "metA"]);

    let granth = d_adhrshiya("granTa~", Curadi);
    assert_has_tip(&[], &granth, Lat, &["granTayati", "granTati"]);

    let ard = d_adhrshiya("arda~^", Curadi);
    assert_has_tip(&[], &ard, Lat, &["ardayati", "ardati"]);
    assert_has_ta(&[], &ard, Lat, &["ardayate", "ardate"]);

    let hins = d_adhrshiya("hisi~", Curadi);
    assert_has_tip(&[], &hins, Lat, &["hiMsayati", "hiMsati"]);
    // TODO: comment on Snam ?

    let x = d_adhrshiya("za\\da~", Curadi);
    assert_has_tip(&["AN"], &x, Lat, &["AsAdayati", "AsIdati"]);
    assert_has_tip(&["AN"], &x, Lut, &["AsAdayitA", "AsattA"]);
    assert_has_tip(&["AN"], &x, Lun, &["AsIzadat", "AsAtsIt"]);

    let shund = d_adhrshiya("SunDa~", Curadi);
    assert_has_tip(&[], &shund, Lut, &["SunDayitA", "SunDitA"]);
    assert_has_tip(&[], &shund, Lun, &["aSuSunDat", "aSunDIt"]);
    assert_has_tas(&[], &shund, Lun, &["aSuSunDatAm", "aSunDizwAm"]);

    let jush = d_adhrshiya("juza~", Curadi);
    assert_has_tip(&[], &jush, Lat, &["jozayati", "jozati"]);

    let dhu = d_adhrshiya("DUY", Curadi);
    assert_has_tip(&[], &dhu, Lat, &["DAvayati", "DUnayati", "Davati"]);
    assert_has_ta(&[], &dhu, Lat, &["DAvayate", "DUnayate", "Davate"]);

    let pri = d_adhrshiya("prIY", Curadi);
    assert_has_tip(&[], &pri, Lat, &["prIRayati", "prAyayati", "prayati"]);
    assert_has_ta(&[], &pri, Lat, &["prIRayate", "prAyayate", "prayate"]);

    let aap = d_adhrshiya("A\\px~", Curadi);
    assert_has_tip(&[], &aap, Lat, &["Apayati", "Apati"]);
    assert_has_tip(&[], &aap, Lun, &["Apipat", "Apat"]);
    assert_has_tip(&[], &aap, Lut, &["ApayitA", "AptA"]);

    let aap = d_adhrshiya("A\\px~^", Curadi);
    assert_has_ta(&[], &aap, Lat, &["Apayate", "Apate"]);

    let tan = d_adhrshiya("tanu~", Curadi);
    assert_has_tip(&[], &tan, Lat, &["tAnayati", "tanati"]);
    assert_has_tip(&["vi"], &tan, Lat, &["vitAnayati", "vitanati"]);

    let can = d_adhrshiya("cana~", Curadi);
    assert_has_tip(&[], &can, Lat, &["cAnayati", "canati"]);

    let vad = d_adhrshiya("vada~^", Curadi);
    assert_has_tip(&[], &vad, Lat, &["vAdayati", "vadati"]);
    assert_has_ta(&[], &vad, Lat, &["vAdayate", "vadate"]);
    assert_has_tas(
        &[],
        &vad,
        Lit,
        &[
            "vAdayAYcakratuH",
            "vAdayAmAsatuH",
            "vAdayAmbaBUvatuH",
            "vavadatuH",
        ],
    );
    assert_has_sip(
        &[],
        &vad,
        Lit,
        &[
            "vAdayAYcakarTa",
            "vAdayAmAsiTa",
            "vAdayAmbaBUviTa",
            "vavadiTa",
        ],
    );
    assert_has_iw(
        &[],
        &vad,
        Lit,
        &["vAdayAYcakre", "vAdayAmAsa", "vAdayAmbaBUva", "vavade"],
    );
    assert_has_tip(&[], &vad, AshirLin, &["vadyAt", "vAdyAt"]);

    let vac = d_adhrshiya("va\\ca~", Curadi);
    assert_has_tip(&[], &vac, Lat, &["vAcayati", "vacati"]);
    assert_has_tip(&[], &vac, Lut, &["vAcayitA", "vaktA"]);
    assert_has_tip(&[], &vac, Lun, &["avIvacat", "avAkzIt"]);

    let man = d_adhrshiya("mAna~", Curadi);
    assert_has_tip(&[], &man, Lat, &["mAnayati", "mAnati"]);
    assert_has_tip(&[], &man, Lut, &["mAnayitA", "mAnitA"]);

    // End of AkusmIya

    // TODO: encode atmanepada here.
    // let bhu = d_adhrshiya("BU", Curadi);
    // assert_has_ta(&[], &bhu, Lat, &["BAvayate", "Bavate"]);
    // assert_has_tip(&[], &bhu, Lat, &["BAvayati", "Bavati"]);

    let mrj = d_adhrshiya("mfjU~", Curadi);
    assert_has_tip(&[], &mrj, Lat, &["mArjayati", "mArjati"]);
    assert_has_tip(&[], &mrj, Lut, &["mArjayitA", "mArjitA", "mArzwA"]);

    let mrsh = d_adhrshiya("mfza~^", Curadi);
    assert_has_tip(&[], &mrsh, Lat, &["marzayati", "marzati"]);
    assert_has_ta(&[], &mrsh, Lat, &["marzayate", "marzate"]);

    let dhrsh = d_adhrshiya("Dfza~", Curadi);
    assert_has_tip(&[], &dhrsh, Lat, &["Darzayati", "Darzati"]);

    // End of ADfzIya

    let katha = d("kaTa", Curadi);
    assert_has_tip(&[], &katha, Lat, &["kaTayati"]);
    assert_has_tip(&[], &katha, Lun, &["acakaTat"]);

    let vara = d("vara", Curadi);
    assert_has_tip(&[], &vara, Lat, &["varayati"]);

    let gana = d("gaRa", Curadi);
    assert_has_tip(&[], &gana, Lat, &["gaRayati"]);
}

#[test]
fn sk_2573() {
    let gana = d("gaRa", Curadi);
    assert_has_tip(&[], &gana, Lun, &["ajIgaRat", "ajagaRat"]);

    let raha = d("raha", Curadi);
    assert_has_tip(&[], &raha, Lun, &["ararahat"]);

    let stana = d("stana", Curadi);
    assert_has_tip(&[], &stana, Lat, &["stanayati"]);

    let gada = d("gada", Curadi);
    assert_has_tip(&[], &gada, Lat, &["gadayati"]);
    assert_has_tip(&[], &gada, Lun, &["ajagadat"]);

    let pata = d("pata", Curadi);
    assert_has_tip(&[], &pata, Lat, &["patayati", "patati", "pAtayati"]);
    assert_has_tip(
        &[],
        &pata,
        Lit,
        &[
            "patAYcakAra",
            "patAmAsa",
            "patAmbaBUva",
            "patayAYcakAra",
            "patayAmAsa",
            "patayAmbaBUva",
            "pAtayAYcakAra",
            "pAtayAmAsa",
            "pAtayAmbaBUva",
        ],
    );
    // Neelesh ji confirms that apapatat is also justified.
    assert_has_tip(&[], &pata, Lun, &["apatIt", "apIpatat", "apapatat"]);

    let pasha = d("paza", Curadi);
    assert_has_tip(&[], &pasha, Lat, &["pazayati"]);

    let svara = d("svara", Curadi);
    assert_has_tip(&[], &svara, Lat, &["svarayati"]);

    let raca = d("raca", Curadi);
    assert_has_tip(&[], &raca, Lat, &["racayati"]);

    let maha = d("maha", Curadi);
    assert_has_tip(&[], &maha, Lat, &["mahayati"]);
    assert_has_tip(&[], &d("maha~", Bhvadi), Lat, &["mahati"]);

    let sara = d("sAra", Curadi);
    assert_has_tip(&[], &sara, Lat, &["sArayati"]);

    let kfpa = d("kfpa", Curadi);
    assert_has_tip(&[], &kfpa, Lat, &["kfpayati"]);

    let shratha = d("SraTa", Curadi);
    assert_has_tip(&[], &shratha, Lat, &["SraTayati"]);

    let bham = d("BAma", Curadi);
    assert_has_tip(&[], &bham, Lun, &["abaBAmat"]);

    let suc = d("sUca", Curadi);
    assert_has_tip(&[], &suc, Lat, &["sUcayati"]);

    let goma = d("goma", Curadi);
    assert_has_tip(&[], &goma, Lun, &["ajugomat"]);

    let kumara = d("kumAra", Curadi);
    assert_has_tip(&[], &kumara, Lun, &["acukumArat"]);

    let sama = d("sAma", Curadi);
    assert_has_tip(&[], &sama, Lun, &["asasAmat"]);
    // TODO: sAma sAntvane?

    let vela = d("vela", Curadi);
    assert_has_tip(&[], &vela, Lat, &["velayati"]);

    let kala = d("kAla", Curadi);
    assert_has_tip(&[], &kala, Lat, &["kAlayati"]);

    let vata = d("vAta", Curadi);
    assert_has_tip(&[], &vata, Lat, &["vAtayati"]);
    assert_has_tip(&[], &vata, Lun, &["avavAtat"]);

    let gavesha = d("gaveza", Curadi);
    assert_has_tip(&[], &gavesha, Lun, &["ajagavezat"]);

    let nivasa = d("nivAsa", Curadi);
    assert_has_tip(&[], &nivasa, Lun, &["aninivAsat"]);

    let sabhaja = d("saBAja", Curadi);
    assert_has_tip(&[], &sabhaja, Lat, &["saBAjayati"]);

    let una = d("Una", Curadi);
    assert_has_tip(&[], &una, Lat, &["Unayati"]);
    assert_has_tip(&[], &una, Lun, &["Onanat"]);

    let dhvan = d("Dvana", Curadi);
    assert_has_tip(&[], &dhvan, Lun, &["adaDvanat"]);

    let kuta = d("kUwa", Curadi);
    assert_has_tip(&[], &kuta, Lat, &["kUwayati"]);

    let sanketa = d("sanketa", Curadi);
    assert_has_tip(&[], &sanketa, Lat, &["saNketayati"]);

    let grama = d("grAma", Curadi);
    assert_has_tip(&[], &grama, Lat, &["grAmayati"]);

    let kuna = d("kuRa", Curadi);
    assert_has_tip(&[], &kuna, Lat, &["kuRayati"]);

    let guna = d("guRa", Curadi);
    assert_has_tip(&[], &guna, Lat, &["guRayati"]);

    let keta = d("keta", Curadi);
    assert_has_tip(&[], &keta, Lat, &["ketayati"]);
    assert_has_tip(&["aBi"], &keta, Lat, &["aBiketayati"]);

    let stena = d("stena", Curadi);
    assert_has_tip(&[], &stena, Lun, &["atistenat"]);

    // start of AgarvIya

    let pada = d("pada", Curadi);
    assert_has_lat(&[], &pada, &["padayate"]);
    assert_has_lun(&[], &pada, &["apapadata"]);

    let grha = d("gfha", Curadi);
    assert_has_lat(&[], &grha, &["gfhayate"]);

    let mfga = d("mfga", Curadi);
    assert_has_lat(&[], &mfga, &["mfgayate"]);
    // TODO: how is this kaRqvAdi?
    assert_has_lat(&[], &d("mfga", Kandvadi), &["mfgyati"]);

    let sthula = d("sTUla", Curadi);
    assert_has_lat(&[], &sthula, &["sTUlayate"]);
    assert_has_lun(&[], &sthula, &["atusTUlata"]);

    let artha = d("arTa", Curadi);
    assert_has_lat(&[], &artha, &["arTayate"]);
    assert_has_lun(&[], &artha, &["ArtaTata"]);

    let satra = d("satra", Curadi);
    assert_has_lun(&[], &satra, &["asasatrata"]);
    // TODO: assert_has_lat(&[], &san(&satra), &["sisatrayizate"]);

    let garva = d("garva", Curadi);
    assert_has_lat(&[], &garva, &["garvayate", "garvati"]);

    // end of AgarvIya

    let sutra = d("sUtra", Curadi);
    assert_has_tip(&[], &sutra, Lat, &["sUtrayati"]);
    assert_has_tip(&[], &sutra, Lun, &["asusUtrat"]);

    let mutra = d("mUtra", Curadi);
    assert_has_tip(&[], &mutra, Lat, &["mUtrayati", "mUtrati"]);

    let para = d("pAra", Curadi);
    assert_has_tip(&[], &para, Lun, &["apapArat"]);

    let tira = d("tIra", Curadi);
    assert_has_tip(&[], &tira, Lun, &["atitIrat"]);

    let put = d("puwa", Curadi);
    assert_has_tip(&[], &put, Lat, &["puwayati"]);

    let dhek = d("Deka", Curadi);
    assert_has_tip(&[], &dhek, Lun, &["adiDekat"]);

    let katra = d("katra", Curadi);
    assert_has_tip(&[], &katra, Lat, &["katrayati", "katrati"]);

    let karta = d("karta", Curadi);
    assert_has_tip(&[], &karta, Lat, &["kartayati", "kartati"]);

    let pati = Dhatu::nama(phit("pawu"), Some(Sanadi::Ric));
    assert_has_tip(&[], &pati, Lat, &["pawayati"]);
    // TODO: apIpawat, apapawat, ...
}

#[test]
fn sk_2574() {
    // "citrati" seems justified. TODO: double-check.
    let citra = d("citra", Curadi);
    assert_has_tip(&[], &citra, Lat, &["citrayati"]);

    let vant = d("vawi~", Curadi);
    assert_has_tip(&[], &vant, Lat, &["vaRwayati", "vaRwati"]);

    let lanj = d("laji~", Curadi);
    assert_has_tip(&[], &lanj, Lat, &["laYjayati", "laYjati"]);

    // TODO: vaRwApayati, laYjApayati

    let stoma = d("stoma", Curadi);
    assert_has_tip(&[], &stoma, Lun, &["atustomat"]);

    let andha = &d("anDa", Curadi);
    assert_has_tip(&[], &andha, Lun, &["AndaDat"]);

    let anka = &d("anka", Curadi);
    assert_has_tip(&[], &anka, Lun, &["AYcakat"]);

    let anga = &d("anga", Curadi);
    assert_has_tip(&[], &anga, Lun, &["AYjagat"]);

    let vyaya = &d("vyaya", Curadi);
    assert_has_tip(&[], &vyaya, Lun, &["avavyayat"]);

    let cheda = d("Ceda", Curadi);
    assert_has_tip(&[], &cheda, Lun, &["acicCedat"]);

    let chada = d("Cada", Curadi);
    assert_has_tip(&[], &chada, Lat, &["Cadayati"]);

    let varna = d("varRa", Curadi);
    assert_has_tip(&[], &varna, Lat, &["varRayati"]);

    let parna = d("parRa", Curadi);
    assert_has_tip(&[], &parna, Lun, &["apaparRat"]);

    // TODO: hastayate, pAdayate, ...
}
