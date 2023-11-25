extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn sk_2543() {
    let rudh = d("ru\\Di~^r", Rudhadi);
    assert_has_tip(&[], &rudh, Lat, &["ruRadDi"]);
    assert_has_tas(&[], &rudh, Lat, &["rundDaH", "runDaH"]);
    assert_has_jhi(&[], &rudh, Lat, &["runDanti"]);
    assert_has_iw(&[], &rudh, Lat, &["runDe"]);
    assert_has_tip(&[], &rudh, Lut, &["rodDA"]);
    assert_has_tip(&[], &rudh, Lrt, &["rotsyati"]);
    assert_has_tip(&[], &rudh, Lot, &["ruRadDu", "rundDAt", "runDAt"]);
    assert_has_sip(&[], &rudh, Lot, &["rundDi", "runDi", "rundDAt", "runDAt"]);
    assert_has_mip(&[], &rudh, Lot, &["ruRaDAni"]);
    assert_has_iw(&[], &rudh, Lot, &["ruRaDE"]);

    assert_has_tip(&[], &rudh, Lan, &["aruRat"]);
    assert_has_tas(&[], &rudh, Lan, &["arunDAm", "arundDAm"]);
    assert_has_sip(&[], &rudh, Lan, &["aruRat", "aruRaH"]);
    assert_has_mip(&[], &rudh, Lan, &["aruRaDam"]);

    assert_has_tip(&[], &rudh, Lun, &["aruDat", "arOtsIt"]);
    assert_has_ta(&[], &rudh, Lun, &["arudDa"]);

    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_tip(&[], &bhid, Lat, &["Binatti"]);
    assert_has_ta(&[], &bhid, Lat, &["Bintte", "Binte"]);
    assert_has_tip(&[], &bhid, Lut, &["BettA"]);
    assert_has_tip(&[], &bhid, Lrt, &["Betsyati"]);
    assert_has_ta(&[], &bhid, Lrt, &["Betsyate"]);

    assert_has_tip(&[], &bhid, Lan, &["aBinat"]);
    assert_has_tas(&[], &bhid, Lan, &["aBinttAm", "aBintAm"]);
    assert_has_mip(&[], &bhid, Lan, &["aBinadam"]);
    assert_has_ta(&[], &bhid, Lan, &["aBinta", "aBintta"]);
    assert_has_tip(&[], &bhid, Lun, &["aBidat", "aBEtsIt"]);
    assert_has_ta(&[], &bhid, Lun, &["aBitta"]);

    let chid = d("Ci\\di~^r", Rudhadi);
    assert_has_tip(&[], &chid, Lun, &["acCidat", "acCEtsIt"]);
    assert_has_ta(&[], &chid, Lun, &["acCitta"]);

    let ric = d("ri\\ci~^r", Rudhadi);
    assert_has_tip(&[], &ric, Lat, &["riRakti"]);
    assert_has_ta(&[], &ric, Lat, &["riNkte"]);
    assert_has_tip(&[], &ric, Lit, &["rireca"]);
    assert_has_ta(&[], &ric, Lit, &["ririce"]);
    assert_has_ta(&[], &ric, Lut, &["rektA"]);
    assert_has_tip(&[], &ric, Lan, &["ariRak"]);
    assert_has_tip(&[], &ric, Lun, &["aricat", "arEkzIt"]);
    assert_has_ta(&[], &ric, Lun, &["arikta"]);

    let vic = d("vi\\ci~^r", Rudhadi);
    assert_has_tip(&[], &vic, Lat, &["vinakti"]);
    assert_has_ta(&[], &vic, Lat, &["viNkte"]);

    let kzud = d("kzu\\di~^r", Rudhadi);
    assert_has_tip(&[], &kzud, Lat, &["kzuRatti"]);
    assert_has_ta(&[], &kzud, Lat, &["kzunte", "kzuntte"]);
    assert_has_tip(&[], &kzud, Lut, &["kzottA"]);
    assert_has_tip(&[], &kzud, Lun, &["akzudat", "akzOtsIt"]);
    assert_has_ta(&[], &kzud, Lun, &["akzutta"]);

    let yuj = d("yu\\ji~^r", Rudhadi);
    assert_has_tip(&[], &yuj, Lut, &["yoktA"]);

    let chrd = d("u~Cfdi~^r", Rudhadi);
    assert_has_tip(&[], &chrd, Lat, &["CfRatti"]);
    assert_has_ta(&[], &chrd, Lat, &["Cfntte", "Cfnte"]);
    assert_has_tip(&[], &chrd, Lit, &["cacCarda"]);
    assert_has_thaas(&[], &chrd, Lit, &["cacCfdize", "cacCftse"]);
    assert_has_tip(&[], &chrd, Lut, &["CarditA"]);
    assert_has_tip(&[], &chrd, Lrt, &["Cardizyati", "Cartsyati"]);
    assert_has_tip(&[], &chrd, Lun, &["acCfdat", "acCardIt"]);
    assert_has_ta(&[], &chrd, Lun, &["acCardizwa"]);

    let trd = d("u~tfdi~^r", Rudhadi);
    assert_has_tip(&[], &trd, Lat, &["tfRatti"]);

    let krt = d("kftI~", Rudhadi);
    assert_has_tip(&[], &krt, Lat, &["kfRatti"]);
}

#[test]
fn sk_2544() {
    let indh = d("YiinDI~\\", Rudhadi);
    assert_has_ta(&[], &indh, Lat, &["inDe", "indDe"]);
    assert_has_thaas(&[], &indh, Lat, &["intse"]);
    assert_has_ta(&[], &indh, Lut, &["inDitA"]);
    assert_has_iw(&[], &indh, Lot, &["inaDE"]);
    assert_has_ta(&[], &indh, Lan, &["EnDa", "EndDa"]);
    assert_has_thaas(&[], &indh, Lan, &["EnDAH", "EndDAH"]);

    let khid = d("Ki\\da~\\", Rudhadi);
    assert_has_ta(&[], &khid, Lut, &["KettA"]);

    let vid = d("vi\\da~\\", Rudhadi);
    assert_has_ta(&[], &vid, Lat, &["vinte", "vintte"]);
    assert_has_ta(&[], &vid, Lut, &["vettA"]);

    let shish = d("Si\\zx~", Rudhadi);
    assert_has_tip(&[], &shish, Lat, &["Sinazwi"]);
    assert_has_tas(&[], &shish, Lat, &["SiMzwaH"]);
    assert_has_jhi(&[], &shish, Lat, &["SiMzanti"]);
    assert_has_sip(&[], &shish, Lit, &["SiSeziTa"]);
    assert_has_tip(&[], &shish, Lut, &["SezwA"]);
    assert_has_tip(&[], &shish, Lrt, &["Sekzyati"]);
    assert_has_sip(&[], &shish, Lot, &["SiRQi", "SiRqQi", "SiMzwAt"]);
    assert_has_mip(&[], &shish, Lot, &["SinazARi"]);
    assert_has_tip(&[], &shish, Lan, &["aSinaw"]);
    assert_has_tip(&[], &shish, Lun, &["aSizat"]);

    let pish = d("pi\\zx~", Rudhadi);
    assert_has_tip(&[], &pish, Lat, &["pinazwi"]);

    let bhanj = d("Ba\\njo~", Rudhadi);
    assert_has_tip(&[], &bhanj, Lat, &["Banakti"]);
    assert_has_sip(&[], &bhanj, Lit, &["baBaYjiTa", "baBaNkTa"]);
    assert_has_tip(&[], &bhanj, Lut, &["BaNktA"]);

    let bhuj = d("Bu\\ja~", Rudhadi);
    assert_has_tip(&[], &bhuj, Lat, &["Bunakti"]);
    assert_has_tip(&[], &bhuj, Lut, &["BoktA"]);
    assert_has_tip(&[], &bhuj, Lrt, &["Bokzyati"]);
    assert_has_tip(&[], &bhuj, Lan, &["aBunak"]);
}

#[test]
fn sk_2545() {
    let trh = d("tfha~", Rudhadi);
    assert_has_tip(&[], &trh, Lat, &["tfReQi"]);
    assert_has_tas(&[], &trh, Lat, &["tfRQaH"]);
    assert_has_tip(&[], &trh, Lit, &["tatarha"]);
    assert_has_tip(&[], &trh, Lut, &["tarhitA"]);
    assert_has_tip(&[], &trh, Lan, &["atfRew"]);

    let hins = d("hisi~", Rudhadi);
    assert_has_tip(&[], &hins, Lat, &["hinasti"]);
    assert_has_tip(&[], &hins, Lit, &["jihiMsa"]);
    assert_has_tip(&[], &hins, Lut, &["hiMsitA"]);

    let und = d("undI~", Rudhadi);
    assert_has_tip(&[], &und, Lat, &["unatti"]);
    assert_has_tas(&[], &und, Lat, &["untaH", "unttaH"]);
    assert_has_jhi(&[], &und, Lat, &["undanti"]);
    assert_has_tip(&[], &und, Lit, &["undAYcakAra", "undAmbaBUva", "undAmAsa"]);
    assert_has_tip(&[], &und, Lan, &["Onat"]);
    assert_has_tas(&[], &und, Lan, &["OntAm", "OnttAm"]);
    assert_has_jhi(&[], &und, Lan, &["Ondan"]);
    assert_has_sip(&[], &und, Lan, &["OnaH", "Onat"]);
    assert_has_mip(&[], &und, Lan, &["Onadam"]);

    let anj = d("anjU~", Rudhadi);
    assert_has_tip(&[], &anj, Lat, &["anakti"]);
    assert_has_tas(&[], &anj, Lat, &["aNktaH"]);
    assert_has_jhi(&[], &anj, Lat, &["aYjanti"]);
    assert_has_tip(&[], &anj, Lit, &["AnaYja"]);
    assert_has_sip(&[], &anj, Lit, &["AnaYjiTa", "AnaNkTa"]);
    assert_has_tip(&[], &anj, Lut, &["aNktA", "aYjitA"]);
    assert_has_sip(&[], &anj, Lot, &["aNgDi", "aNktAt"]);
    assert_has_mip(&[], &anj, Lot, &["anajAni"]);
    assert_has_tip(&[], &anj, Lan, &["Anak"]);
}

#[test]
fn sk_2546() {
    let anj = d("anjU~", Rudhadi);
    assert_has_tip(&[], &anj, Lun, &["AYjIt"]);

    let tanj = d("tancU~", Rudhadi);
    assert_has_tip(&[], &tanj, Lat, &["tanakti"]);
    assert_has_tip(&[], &tanj, Lut, &["taNktA", "taYcitA"]);

    let vij = d("o~vijI~", Rudhadi);
    assert_has_tip(&[], &vij, Lat, &["vinakti"]);
    assert_has_tas(&[], &vij, Lat, &["viNktaH"]);
    assert_has_sip(&[], &vij, Lit, &["vivijiTa"]);
    assert_has_tip(&[], &vij, Lut, &["vijitA"]);
    assert_has_tip(&[], &vij, Lan, &["avinak"]);
    assert_has_tip(&[], &vij, Lun, &["avijIt"]);

    let vrj = d("vfjI~", Rudhadi);
    assert_has_tip(&[], &vrj, Lat, &["vfRakti"]);
    assert_has_tip(&[], &vrj, Lut, &["varjitA"]);

    let prc = d("pfcI~", Rudhadi);
    assert_has_tip(&[], &prc, Lat, &["pfRakti"]);
    assert_has_tip(&[], &prc, Lit, &["paparca"]);
}
