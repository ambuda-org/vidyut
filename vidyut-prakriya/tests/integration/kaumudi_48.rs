extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn sk_2534() {
    let tud = d("tu\\da~^", Tudadi);
    assert_has_tip(&[], &tud, Lat, &["tudati"]);
    assert_has_ta(&[], &tud, Lat, &["tudate"]);
    assert_has_tip(&[], &tud, Lit, &["tutoda"]);
    assert_has_sip(&[], &tud, Lit, &["tutodiTa"]);
    assert_has_ta(&[], &tud, Lit, &["tutude"]);
    assert_has_tip(&[], &tud, Lut, &["tottA"]);
    assert_has_tip(&[], &tud, Lun, &["atOtsIt"]);
    assert_has_ta(&[], &tud, Lun, &["atutta"]);

    let nud = d("Ru\\da~^", Tudadi);
    assert_has_tip(&[], &nud, Lat, &["nudati"]);
    assert_has_ta(&[], &nud, Lat, &["nudate"]);
    assert_has_tip(&[], &nud, Lit, &["nunoda"]);
    assert_has_ta(&[], &nud, Lit, &["nunude"]);
    assert_has_tip(&[], &nud, Lut, &["nottA"]);

    let dish = d("di\\Sa~^", Tudadi);
    assert_has_tip(&[], &dish, Lut, &["dezwA"]);
    assert_has_ta(&[], &dish, AshirLin, &["dikzIzwa"]);
    assert_has_tip(&[], &dish, Lun, &["adikzat"]);
    assert_has_ta(&[], &dish, Lun, &["adikzata"]);

    let bhrasj = d("Bra\\sja~^", Tudadi);
    assert_has_tip(&[], &bhrasj, Lat, &["Bfjjati"]);
    assert_has_ta(&[], &bhrasj, Lat, &["Bfjjate"]);
}

#[test]
fn sk_2535() {
    let bhrasj = d("Bra\\sja~^", Tudadi);
    assert_has_tip(&[], &bhrasj, Lit, &["baBarja", "baBrajja"]);
    assert_has_tas(&[], &bhrasj, Lit, &["baBarjatuH", "baBrajjatuH"]);
    assert_has_sip(
        &[],
        &bhrasj,
        Lit,
        &["baBarjiTa", "baBarzWa", "baBrajjiTa", "baBrazWa"],
    );
    assert_has_ta(&[], &bhrasj, Lit, &["baBarje", "baBrajje"]);

    assert_has_tip(&[], &bhrasj, Lut, &["BrazwA", "BarzwA"]);
    assert_has_tip(&[], &bhrasj, Lrt, &["Brakzyati", "Barkzyati"]);
    assert_has_tip(&[], &bhrasj, AshirLin, &["BfjjyAt"]);
    assert_has_tas(&[], &bhrasj, AshirLin, &["BfjjyAstAm"]);
    assert_has_ta(&[], &bhrasj, AshirLin, &["BarkzIzwa", "BrakzIzwa"]);
    assert_has_tip(&[], &bhrasj, Lun, &["aBArkzIt", "aBrAkzIt"]);
    assert_has_ta(&[], &bhrasj, Lun, &["aBarzwa", "aBrazwa"]);

    let kship = d("kzi\\pa~^", Tudadi);
    assert_has_tip(&[], &kship, Lat, &["kzipati"]);
    assert_has_ta(&[], &kship, Lat, &["kzipate"]);
    assert_has_tip(&[], &kship, Lut, &["kzeptA"]);
    assert_has_tip(&[], &kship, Lun, &["akzEpsIt"]);
    assert_has_ta(&[], &kship, Lun, &["akzipta"]);

    let krsh = d("kf\\za~^", Tudadi);
    assert_has_tip(&[], &krsh, Lat, &["kfzati"]);
    assert_has_ta(&[], &krsh, Lat, &["kfzate"]);
    assert_has_tip(&[], &krsh, Lut, &["krazwA", "karzwA"]);
    assert_has_tip(&[], &krsh, AshirLin, &["kfzyAt"]);
    assert_has_ta(&[], &krsh, AshirLin, &["kfkzIzwa"]);
    assert_has_tip(&[], &krsh, Lun, &["akrAkzIt", "akArkzIt", "akfkzat"]);
    assert_has_ta(&[], &krsh, Lun, &["akfzwa", "akfkzata"]);
    assert_has_aataam(&[], &krsh, Lun, &["akfkzAtAm"]);
    assert_has_jha(&[], &krsh, Lun, &["akfkzata", "akfkzanta"]);

    let rsh = d("fzI~", Tudadi);
    assert_has_tip(&[], &rsh, Lat, &["fzati"]);
    assert_has_tip(&[], &rsh, Lit, &["Anarza"]);

    let jush = d("juzI~\\", Tudadi);
    assert_has_ta(&[], &jush, Lat, &["juzate"]);

    let vij = d("o~vijI~\\", Tudadi);
    assert_has_ta(&["ud"], &vij, Lat, &["udvijate"]);
}

#[test]
fn sk_2536() {
    let vij = d("o~vijI~\\", Tudadi);
    assert_has_ta(&["ud"], &vij, Lut, &["udvijitA"]);
    assert_has_ta(&["ud"], &vij, Lrt, &["udvijizyate"]);

    let laj = d("o~lajI~\\", Tudadi);
    assert_has_ta(&[], &laj, Lat, &["lajate"]);
    assert_has_ta(&[], &laj, Lit, &["leje"]);

    let lasj = d("o~lasjI~\\", Tudadi);
    assert_has_ta(&[], &lasj, Lat, &["lajjate"]);
    assert_has_ta(&[], &lasj, Lit, &["lalajje"]);

    let vrasc = d("o~vrascU~", Tudadi);
    assert_has_tip(&[], &vrasc, Lat, &["vfScati"]);
    assert_has_tip(&[], &vrasc, Lit, &["vavraSca"]);
    assert_has_tas(&[], &vrasc, Lit, &["vavraScatuH"]);
    assert_has_sip(&[], &vrasc, Lit, &["vavraSciTa", "vavrazWa"]);
    assert_has_tip(&[], &vrasc, Lut, &["vraScitA", "vrazwA"]);
    assert_has_tip(&[], &vrasc, Lrt, &["vraScizyati", "vrakzyati"]);
    assert_has_tip(&[], &vrasc, AshirLin, &["vfScyAt"]);
    assert_has_tip(&[], &vrasc, Lun, &["avraScIt", "avrAkzIt"]);

    let vyac = d("vyaca~", Tudadi);
    assert_has_tip(&[], &vyac, Lat, &["vicati"]);
    assert_has_tip(&[], &vyac, Lit, &["vivyAca"]);
    assert_has_tas(&[], &vyac, Lit, &["vivicatuH"]);
    assert_has_tip(&[], &vyac, Lut, &["vyacitA"]);
    assert_has_tip(&[], &vyac, Lrt, &["vyacizyati"]);
    assert_has_tip(&[], &vyac, AshirLin, &["vicyAt"]);
    assert_has_tip(&[], &vyac, Lun, &["avyAcIt", "avyacIt"]);

    let unch = d("uCi~", Tudadi);
    assert_has_tip(&[], &unch, Lat, &["uYCati"]);

    let uch = d("uCI~", Tudadi);
    assert_has_tip(&[], &uch, Lat, &["ucCati"]);

    let rch = d("fCa~", Tudadi);
    assert_has_tip(&[], &rch, Lit, &["AnarCa", "AnarcCa"]);
    assert_has_tas(&[], &rch, Lit, &["AnarCatuH", "AnarcCatuH"]);
    assert_has_tip(&[], &rch, Lut, &["fcCitA"]);

    let mich = d("miCa~", Tudadi);
    assert_has_tip(&[], &mich, Lun, &["amicCIt"]);

    let tvac = d("tvaca~", Tudadi);
    assert_has_tip(&[], &tvac, Lit, &["tatvAca"]);

    let rc = d("fca~", Tudadi);
    assert_has_tip(&[], &rc, Lit, &["Anarca"]);

    let lubh = d("luBa~", Tudadi);
    assert_has_tip(&[], &lubh, Lat, &["luBati"]);
    assert_has_tip(&[], &lubh, Lut, &["loBitA", "lobDA"]);
    assert_has_tip(&[], &lubh, Lrt, &["loBizyati"]);

    let riph = d("riPa~", Tudadi);
    assert_has_tip(&[], &riph, Lat, &["riPati"]);
    assert_has_tip(&[], &riph, Lit, &["rirePa"]);

    let trp = d("tfpa~", Tudadi);
    assert_has_tip(&[], &trp, Lat, &["tfpati"]);
    assert_has_tip(&[], &trp, Lit, &["tatarpa"]);
    assert_has_tip(&[], &trp, Lut, &["tarpitA"]);
    assert_has_tip(&[], &trp, Lun, &["atarpIt"]);

    let trmph = d("tfnPa~", Tudadi);
    assert_has_tip(&[], &trmph, Lat, &["tfmPati"]);
    assert_has_tip(&[], &trmph, Lit, &["tatfmPa"]);
    assert_has_tip(&[], &trmph, AshirLin, &["tfPyAt"]);

    let tup = d("tupa~", Tudadi);
    assert_has_tip(&[], &tup, Lat, &["tupati"]);

    let tump = d("tunpa~", Tudadi);
    assert_has_tip(&[], &tump, Lat, &["tumpati"]);

    let tuph = d("tuPa~", Tudadi);
    assert_has_tip(&[], &tuph, Lat, &["tuPati"]);

    let tumph = d("tunPa~", Tudadi);
    assert_has_tip(&[], &tumph, Lat, &["tumPati"]);

    let drp = d("dfpa~", Tudadi);
    assert_has_tip(&[], &drp, Lat, &["dfpati"]);

    let drmph = d("dfnPa~", Tudadi);
    assert_has_tip(&[], &drmph, Lat, &["dfmPati"]);

    let rph = d("fPa~", Tudadi);
    assert_has_tip(&[], &rph, Lat, &["fPati"]);
    assert_has_tip(&[], &rph, Lit, &["AnarPa"]);

    let rmph = d("fnPa~", Tudadi);
    assert_has_tip(&[], &rmph, Lat, &["fmPati"]);
    assert_has_tip(&[], &rmph, Lit, &["fmPAYcakAra", "fmPAmbaBUva", "fmPAmAsa"]);

    let guph = d("guPa~", Tudadi);
    assert_has_tip(&[], &guph, Lat, &["guPati"]);
    assert_has_tip(&[], &guph, Lit, &["jugoPa"]);

    let gumph = d("gunPa~", Tudadi);
    assert_has_tip(&[], &gumph, Lat, &["gumPati"]);
    assert_has_tip(&[], &gumph, Lit, &["jugumPa"]);

    let ubh = d("uBa~", Tudadi);
    assert_has_tip(&[], &ubh, Lat, &["uBati"]);
    assert_has_tip(&[], &ubh, Lit, &["uvoBa"]);

    let umbh = d("unBa~", Tudadi);
    assert_has_tip(&[], &umbh, Lat, &["umBati"]);
    assert_has_tip(&[], &umbh, Lit, &["umBAYcakAra", "umBAmbaBUva", "umBAmAsa"]);

    let shubh = d("SuBa~", Tudadi);
    assert_has_tip(&[], &shubh, Lat, &["SuBati"]);

    let shumbh = d("SunBa~", Tudadi);
    assert_has_tip(&[], &shumbh, Lat, &["SumBati"]);

    let drbh = d("dfBI~", Tudadi);
    assert_has_tip(&[], &drbh, Lat, &["dfBati"]);

    // cartsyati is from KV 7.2.57.
    let crt = d("cftI~", Tudadi);
    assert_has_tip(&[], &crt, Lut, &["cartitA"]);
    assert_has_tip(&[], &crt, Lrt, &["cartizyati", "cartsyati"]);
    assert_has_tip(&[], &crt, Lun, &["acartIt"]);

    let vidh = d("viDa~", Tudadi);
    assert_has_tip(&[], &vidh, Lat, &["viDati"]);
    assert_has_tip(&[], &vidh, Lut, &["veDitA"]);

    let jud = d("juqa~", Tudadi);
    assert_has_tip(&[], &jud, Lat, &["juqati"]);

    let mrd = d("mfqa~", Tudadi);
    assert_has_tip(&[], &mrd, Lat, &["mfqati"]);
    assert_has_tip(&[], &mrd, Lut, &["marqitA"]);

    let prd = d("pfqa~", Tudadi);
    assert_has_tip(&[], &prd, Lat, &["pfqati"]);

    let prn = d("pfRa~", Tudadi);
    assert_has_tip(&[], &prn, Lat, &["pfRati"]);
    assert_has_tip(&[], &prn, Lit, &["paparRa"]);

    let vrn = d("vfRa~", Tudadi);
    assert_has_tip(&[], &vrn, Lat, &["vfRati"]);

    let tun = d("tuRa~", Tudadi);
    assert_has_tip(&[], &tun, Lit, &["tutoRa"]);

    let pun = d("puRa~", Tudadi);
    assert_has_tip(&[], &pun, Lat, &["puRati"]);

    let sur = d("zura~", Tudadi);
    assert_has_tip(&[], &sur, Lat, &["surati"]);
    assert_has_tip(&[], &sur, Lit, &["suzora"]);

    let kur = d("kura~", Tudadi);
    assert_has_tip(&[], &kur, Lat, &["kurati"]);
    assert_has_tip(&[], &kur, AshirLin, &["kUryAt"]);

    let trh = d("tfhU~", Tudadi);
    assert_has_tip(&[], &trh, Lat, &["tfhati"]);
    assert_has_tip(&[], &trh, Lit, &["tatarha"]);

    let strh = d("stfhU~", Tudadi);
    assert_has_tip(&[], &strh, Lat, &["stfhati"]);
    assert_has_tip(&[], &strh, Lit, &["tastarha"]);
    assert_has_tip(&[], &strh, Lut, &["starhitA", "starQA"]);

    // atfMhizwAm is justified.
    let trnh = d("tfnhU~", Tudadi);
    assert_has_tip(&[], &trnh, Lun, &["atfMhIt", "atArNkzIt"]);
    assert_has_tas(&[], &trnh, Lun, &["atfMhizwAm", "atArRQAm"]);

    let ish = d("izu~", Tudadi);
    assert_has_tip(&[], &ish, Lat, &["icCati"]);
    assert_has_tip(&[], &ish, Lut, &["ezitA", "ezwA"]);
    assert_has_tip(&[], &ish, Lrt, &["ezizyati"]);
    assert_has_tip(&[], &ish, AshirLin, &["izyAt"]);
    assert_has_tip(&[], &ish, Lun, &["EzIt"]);

    let mish = d("miza~", Tudadi);
    assert_has_tip(&[], &mish, Lat, &["mizati"]);
    assert_has_tip(&[], &mish, Lut, &["mezitA"]);

    let likh = d("liKa~", Tudadi);
    assert_has_tip(&[], &likh, Lit, &["lileKa"]);

    let kut = d_kutadi("kuwa~", Tudadi);
    assert_has_sip(&[], &kut, Lit, &["cukuwiTa"]);
    assert_has_mip(&[], &kut, Lit, &["cukowa", "cukuwa"]);
    assert_has_tip(&[], &kut, Lut, &["kuwitA"]);

    let chur = d_kutadi("Cura~", Tudadi);
    assert_has_tip(&[], &chur, AshirLin, &["CuryAt"]);

    let sphut = d_kutadi("sPuwa~", Tudadi);
    assert_has_tip(&[], &sphut, Lat, &["sPuwati"]);
    assert_has_tip(&[], &sphut, Lit, &["pusPowa"]);

    let trut = d_kutadi("truwa~", Tudadi);
    assert_has_tip(&[], &trut, Lat, &["truwyati", "truwati"]);
    assert_has_tip(&[], &trut, Lit, &["tutrowa"]);
    assert_has_tip(&[], &trut, Lut, &["truwitA"]);

    let tut = d_kutadi("tuwa~", Tudadi);
    assert_has_tip(&[], &tut, Lat, &["tuwati"]);
    assert_has_tip(&[], &tut, Lit, &["tutowa"]);
    assert_has_tip(&[], &tut, Lut, &["tuwitA"]);

    let krd = d_kutadi("kfqa~", Tudadi);
    assert_has_tip(&[], &krd, Lit, &["cakarqa"]);
    assert_has_tip(&[], &krd, Lut, &["kfqitA"]);

    let thud = d_kutadi("Tuqa~", Tudadi);
    assert_has_tip(&[], &thud, Lat, &["Tuqati"]);
    assert_has_tip(&[], &thud, Lit, &["tuToqa"]);

    let sthud = d_kutadi("sTuqa~", Tudadi);
    assert_has_tip(&[], &sthud, Lit, &["tusToqa"]);

    let sphur = d_kutadi("sPura~", Tudadi);
    assert_has_tip(&[], &sphur, Lat, &["sPurati"]);
}

#[test]
fn sk_2537() {
    // nissPurati and nizzPurati are justified.
    let sphur = d("sPura~", Tudadi);
    assert_has_tip(
        &["nis"],
        &sphur,
        Lat,
        &["niHzPurati", "niHsPurati", "nissPurati", "nizzPurati"],
    );

    let sphar = d("sPara~", Tudadi);
    assert_has_tip(&[], &sphar, Lit, &["pasPAra"]);

    let gur = d_kutadi("gurI~\\", Tudadi);
    assert_has_ta(&[], &gur, Lat, &["gurate"]);
    assert_has_ta(&[], &gur, Lit, &["jugure"]);
    assert_has_ta(&[], &gur, Lut, &["guritA"]);

    let nu = d_kutadi("RU", Tudadi);
    assert_has_tip(&[], &nu, Lat, &["nuvati"]);
    assert_has_tip(&[], &nu, Lun, &["anuvIt"]);

    let dhu = d("DU", Tudadi);
    assert_has_tip(&[], &dhu, Lat, &["Duvati"]);

    let gu = d_kutadi("gu\\", Tudadi);
    assert_has_sip(&[], &gu, Lit, &["juguviTa", "juguTa"]);
    assert_has_tip(&[], &gu, Lut, &["gutA"]);
    assert_has_tip(&[], &gu, Lrt, &["guzyati"]);
    assert_has_tip(&[], &gu, Lun, &["aguzIt"]);
    assert_has_tas(&[], &gu, Lun, &["agutAm"]);
    assert_has_jhi(&[], &gu, Lun, &["aguzuH"]);

    // "Adyasya DruvatItyAdi guvativat"
    let dhru = d_kutadi("Dru\\", Tudadi);
    assert_has_sip(&[], &dhru, Lit, &["duDruviTa", "duDruTa"]);
    assert_has_tip(&[], &dhru, Lut, &["DrutA"]);
    assert_has_tip(&[], &dhru, Lrt, &["Druzyati"]);
    assert_has_tip(&[], &dhru, Lun, &["aDruzIt"]);
    assert_has_tas(&[], &dhru, Lun, &["aDrutAm"]);
    assert_has_jhi(&[], &dhru, Lun, &["aDruzuH"]);

    // "dvitIyastu sew"
    let dhruv = d_kutadi("Druva~", Tudadi);
    assert_has_tip(&[], &dhruv, Lut, &["DruvitA"]);
    assert_has_sip(&[], &dhruv, Lit, &["duDruviTa"]);
    assert_has_tip(&[], &dhruv, Lut, &["DruvitA"]);
    assert_has_tip(&[], &dhruv, Lrt, &["Druvizyati"]);
    assert_has_tip(&[], &dhruv, AshirLin, &["DrUvyAt"]);
    assert_has_tip(&[], &dhruv, Lun, &["aDruvIt"]);
    assert_has_tas(&[], &dhruv, Lun, &["aDruvizwAm"]);

    let kuu = d_kutadi("kUN", Tudadi);
    assert_has_ta(&[], &kuu, Lut, &["kuvitA"]);
    assert_has_ta(&[], &kuu, Lun, &["akuvizwa"]);

    let ku = d_kutadi("ku\\N", Tudadi);
    assert_has_ta(&[], &ku, Lut, &["kutA"]);
    assert_has_ta(&[], &ku, Lun, &["akuta"]);

    let pr = d("pf\\N", Tudadi);
    assert_has_ta(&["vi", "AN"], &pr, Lat, &["vyApriyate"]);
    assert_has_ta(&["vi", "AN"], &pr, Lit, &["vyApapre"]);
    assert_has_aataam(&["vi", "AN"], &pr, Lit, &["vyApaprAte"]);
    assert_has_ta(&["vi", "AN"], &pr, Lrt, &["vyAparizyate"]);
    assert_has_ta(&["vi", "AN"], &pr, Lun, &["vyApfta"]);
    assert_has_aataam(&["vi", "AN"], &pr, Lun, &["vyApfzAtAm"]);
}

#[test]
fn sk_2538() {
    let mr = d("mf\\N", Tudadi);
    assert_has_ta(&[], &mr, Lat, &["mriyate"]);
    assert_has_tip(&[], &mr, Lit, &["mamAra"]);
    assert_has_sip(&[], &mr, Lit, &["mamarTa"]);
    assert_has_vas(&[], &mr, Lit, &["mamriva"]);
    assert_has_sip(&[], &mr, Lut, &["martAsi"]);
    assert_has_tip(&[], &mr, Lrt, &["marizyati"]);
    assert_has_ta(&[], &mr, AshirLin, &["mfzIzwa"]);
    assert_has_ta(&[], &mr, Lun, &["amfta"]);

    let ri = d("ri\\", Tudadi);
    assert_has_tip(&[], &ri, Lat, &["riyati"]);

    let pi = d("pi\\", Tudadi);
    assert_has_tip(&[], &pi, Lat, &["piyati"]);

    assert_has_tip(&[], &ri, Lut, &["retA"]);
    assert_has_tip(&[], &pi, Lut, &["petA"]);

    let su = d("zU", Tudadi);
    assert_has_tip(&[], &su, Lat, &["suvati"]);
    assert_has_tip(&[], &su, Lut, &["savitA"]);

    let kr = d("kF", Tudadi);
    assert_has_tip(&[], &kr, Lat, &["kirati"]);
    assert_has_tas(&[], &kr, Lat, &["kirataH"]);
    assert_has_tip(&[], &kr, Lit, &["cakAra"]);
    assert_has_tas(&[], &kr, Lit, &["cakaratuH"]);
    assert_has_tip(&[], &kr, Lut, &["karitA", "karItA"]);
    assert_has_tip(&[], &kr, AshirLin, &["kIryAt"]);
    assert_has_tip(&[], &kr, Lun, &["akArIt"]);
}

#[test]
fn sk_2539() {
    let kr = d("kF", Tudadi);
    assert_has_tip(&["upa"], &kr, Lat, &["upaskirati", "upakirati"]);
    assert_has_tip(&["upa"], &kr, Lan, &["upAskirat", "upAkirat"]);
    assert_has_tip(&["upa"], &kr, Lit, &["upacaskAra", "upacakAra"]);
}

#[test]
fn sk_2540() {
    let kr = d("kF", Tudadi);
    assert_has_tip(&["upa"], &kr, Lat, &["upaskirati", "upakirati"]);
    assert_has_tip(&["prati"], &kr, Lat, &["pratiskirati", "pratikirati"]);
}

#[test]
fn sk_2541() {
    let gr = d("gF", Tudadi);
    assert_has_tip(&[], &gr, Lat, &["girati", "gilati"]);
    assert_has_tip(&[], &gr, Lit, &["jagAra", "jagAla"]);
    assert_has_sip(&[], &gr, Lit, &["jagariTa", "jagaliTa"]);
    assert_has_tip(&[], &gr, Lut, &["garitA", "garItA", "galitA", "galItA"]);

    let dr = d("df\\N", Tudadi);
    assert_has_ta(&["AN"], &dr, Lat, &["Adriyate"]);
    assert_has_aataam(&["AN"], &dr, Lat, &["Adriyete"]);
    assert_has_ta(&["AN"], &dr, Lit, &["Adadre"]);
    assert_has_thaas(&["AN"], &dr, Lit, &["Adadrize"]);
    assert_has_ta(&["AN"], &dr, Lut, &["AdartA"]);
    assert_has_ta(&["AN"], &dr, Lrt, &["Adarizyate"]);
    assert_has_ta(&["AN"], &dr, AshirLin, &["AdfzIzwa"]);
    assert_has_ta(&["AN"], &dr, Lun, &["Adfta"]);
    assert_has_aataam(&["AN"], &dr, Lun, &["AdfzAtAm"]);

    let dhr = d("Df\\N", Tudadi);
    assert_has_ta(&[], &dhr, Lat, &["Driyate"]);

    let prach = d("pra\\Ca~", Tudadi);
    assert_has_tip(&[], &prach, Lat, &["pfcCati"]);
    assert_has_tip(&[], &prach, Lit, &["papracCa"]);
    assert_has_tas(&[], &prach, Lit, &["papracCatuH"]);
    assert_has_sip(&[], &prach, Lit, &["papracCiTa", "paprazWa"]);
    assert_has_tip(&[], &prach, Lut, &["prazwA"]);
    assert_has_tip(&[], &prach, Lrt, &["prakzyati"]);
    assert_has_tip(&[], &prach, Lun, &["aprAkzIt"]);

    let srj = d("sf\\ja~", Tudadi);
    assert_has_sip(&[], &srj, Lit, &["sasarjiTa", "sasrazWa"]);
    assert_has_tip(&[], &srj, Lut, &["srazwA"]);
    assert_has_tip(&[], &srj, Lrt, &["srakzyati"]);
    assert_has_tip(&[], &srj, VidhiLin, &["sfjet"]);
    assert_has_tip(&[], &srj, AshirLin, &["sfjyAt"]);
    assert_has_tip(&[], &srj, Lun, &["asrAkzIt"]);

    let masj = d("wuma\\sjo~", Tudadi);
    assert_has_tip(&[], &masj, Lat, &["majjati"]);
    assert_has_tip(&[], &masj, Lit, &["mamajja"]);
    assert_has_sip(&[], &masj, Lit, &["mamaNkTa", "mamajjiTa"]);
    assert_has_tip(&[], &masj, Lut, &["maNktA"]);
    assert_has_tip(&[], &masj, Lrt, &["maNkzyati"]);
    assert_has_tip(&[], &masj, Lun, &["amANkzIt"]);
    assert_has_tas(&[], &masj, Lun, &["amANktAm"]);
    assert_has_jhi(&[], &masj, Lun, &["amANkzuH"]);

    let ruj = d("ru\\jo~", Tudadi);
    assert_has_tip(&[], &ruj, Lut, &["roktA"]);
    assert_has_tip(&[], &ruj, Lrt, &["rokzyati"]);
    assert_has_tip(&[], &ruj, Lun, &["arOkzIt"]);
    assert_has_tas(&[], &ruj, Lun, &["arOktAm"]);

    let chup = d("Cu\\pa~", Tudadi);
    assert_has_tip(&[], &chup, Lun, &["acCOpsIt"]);

    let rush = d("ru\\Sa~", Tudadi);
    assert_has_tip(&[], &rush, Lut, &["rozwA"]);
    assert_has_tip(&[], &rush, Lrt, &["rokzyati"]);

    let rish = d("ri\\Sa~", Tudadi);
    assert_has_tip(&[], &rish, Lut, &["rezwA"]);
    assert_has_tip(&[], &rish, Lrt, &["rekzyati"]);

    let lish = d("li\\Sa~", Tudadi);
    assert_has_tip(&[], &lish, Lun, &["alikzat"]);

    let sprsh = d("spf\\Sa~", Tudadi);
    assert_has_tip(&[], &sprsh, Lut, &["sprazwA", "sparzwA"]);
    assert_has_tip(&[], &sprsh, Lrt, &["sprakzyati", "sparkzyati"]);
    assert_has_tip(&[], &sprsh, Lun, &["asprAkzIt", "aspArkzIt", "aspfkzat"]);

    let vicch = d("viCa~", Tudadi);
    assert_has_tip(&[], &vicch, Lat, &["vicCAyati"]);
    assert_has_tip(
        &[],
        &vicch,
        Lit,
        &["vivicCa", "vicCAyAYcakAra", "vicCAyAmbaBUva", "vicCAyAmAsa"],
    );

    let vish = d("vi\\Sa~", Tudadi);
    assert_has_tip(&[], &vish, Lat, &["viSati"]);
    assert_has_tip(&[], &vish, Lut, &["vezwA"]);

    let mrsh = d("mf\\Sa~", Tudadi);
    assert_has_tip(&[], &mrsh, Lun, &["amrAkzIt", "amArkzIt", "amfkzat"]);

    let sad = d("za\\dx~", Tudadi);
    assert_has_tip(&[], &sad, Lat, &["sIdati"]);
    assert_has_krdanta(&[], &sad, Krt::Satf, &["sIdat"]);
    assert_has_krdanta(&[], &sad, Krt::GaY, &["sAda"]);

    let mil = d("mila~^", Tudadi);
    assert_has_tip(&[], &mil, Lat, &["milati"]);
    assert_has_ta(&[], &mil, Lat, &["milate"]);
    assert_has_tip(&[], &mil, Lit, &["mimela"]);
    assert_has_ta(&[], &mil, Lit, &["mimile"]);
}

#[test]
fn sk_2542() {
    let muc = d("mu\\cx~^", Tudadi);
    assert_has_tip(&[], &muc, Lat, &["muYcati"]);
    assert_has_ta(&[], &muc, Lat, &["muYcate"]);
    assert_has_tip(&[], &muc, Lut, &["moktA"]);
    assert_has_tip(&[], &muc, AshirLin, &["mucyAt"]);
    assert_has_ta(&[], &muc, AshirLin, &["mukzIzwa"]);
    assert_has_tip(&[], &muc, Lun, &["amucat"]);
    assert_has_ta(&[], &muc, Lun, &["amukta"]);
    assert_has_aataam(&[], &muc, Lun, &["amukzAtAm"]);

    let lup = d("lu\\px~^", Tudadi);
    assert_has_tip(&[], &lup, Lat, &["lumpati"]);
    assert_has_ta(&[], &lup, Lat, &["lumpate"]);
    assert_has_tip(&[], &lup, Lun, &["alupat"]);
    assert_has_ta(&[], &lup, Lun, &["alupta"]);

    let vid = d("vi\\dx~^", Tudadi);
    assert_has_tip(&[], &vid, Lat, &["vindati"]);
    assert_has_ta(&[], &vid, Lat, &["vindate"]);
    assert_has_tip(&[], &vid, Lit, &["viveda"]);
    assert_has_ta(&[], &vid, Lit, &["vivide"]);
    assert_has_tip(&[], &vid, Lut, &["veditA", "vettA"]);
    assert_has_tip(&["pari"], &vid, Lut, &["parivettA", "pariveditA"]);

    let lip = d("li\\pa~^", Tudadi);
    assert_has_tip(&[], &lip, Lat, &["limpati"]);
    assert_has_ta(&[], &lip, Lat, &["limpate"]);
    assert_has_tip(&[], &lip, Lut, &["leptA"]);
    assert_has_tip(&[], &lip, Lun, &["alipat"]);
    assert_has_ta(&[], &lip, Lun, &["alipata", "alipta"]);

    let sic = d("zi\\ca~^", Tudadi);
    assert_has_tip(&[], &sic, Lat, &["siYcati"]);
    assert_has_ta(&[], &sic, Lat, &["siYcate"]);
    assert_has_tip(&[], &sic, Lun, &["asicat"]);
    assert_has_ta(&[], &sic, Lun, &["asicata", "asikta"]);
    assert_has_tip(&["aBi"], &sic, Lat, &["aBiziYcati"]);
    assert_has_tip(&["aBi"], &sic, Lan, &["aByaziYcat"]);
    assert_has_tip(&["aBi"], &sic, Lit, &["aBizizeca"]);

    let krt = d("kftI~", Tudadi);
    assert_has_tip(&[], &krt, Lat, &["kfntati"]);
    assert_has_tip(&[], &krt, Lit, &["cakarta"]);
    assert_has_tip(&[], &krt, Lut, &["kartitA"]);
    assert_has_tip(&[], &krt, Lrt, &["kartizyati", "kartsyati"]);
    assert_has_tip(&[], &krt, Lun, &["akartIt"]);

    let khid = d("Ki\\da~", Tudadi);
    assert_has_tip(&[], &khid, Lat, &["Kindati"]);
    assert_has_tip(&[], &khid, Lit, &["ciKeda"]);
    assert_has_tip(&[], &khid, Lut, &["KettA"]);

    let pish = d("piSa~", Tudadi);
    assert_has_tip(&[], &pish, Lat, &["piMSati"]);
    assert_has_tip(&[], &pish, Lut, &["peSitA"]);
    assert_has_tip(&[], &pish, Lot, &["piMSatu", "piMSatAt"]);
}
