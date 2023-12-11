extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Dhatu;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

fn assert_has_san_tip(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_tip(prefixes, &san(&dhatu), Lat, expected);
}

fn assert_has_san_ta(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_ta(prefixes, &san(&dhatu), Lat, expected);
}

#[test]
fn sk_2608() {
    let path = d("paWa~", Bhvadi);
    assert_has_tip(&[], &san(&path), Lat, &["pipaWizati"]);

    let irshy = d("Irzya~", Bhvadi);
    assert_has_tip(&[], &san(&irshy), Lat, &["Irzyiyizati", "Irzyizizati"]);
}

#[test]
fn sk_2609() {
    let rud = d("rudi~r", Adadi);
    assert_has_tip(&[], &san(&rud), Lat, &["rurudizati"]);

    let vid = d("vida~", Adadi);
    assert_has_tip(&[], &san(&vid), Lat, &["vividizati"]);

    let mush = d("muza~", Kryadi);
    assert_has_tip(&[], &san(&mush), Lat, &["mumuzizati"]);
}

#[test]
fn sk_2610() {
    let grah = d("graha~^", Kryadi);
    assert_has_tip(&[], &san(&grah), Lat, &["jiGfkzati"]);

    let svap = d("Yizva\\pa~", Adadi);
    assert_has_tip(&[], &san(&svap), Lat, &["suzupsati"]);
}

#[test]
fn sk_2611() {
    let prach = d("pra\\Ca~", Tudadi);
    assert_has_tip(&[], &san(&prach), Lat, &["pipfcCizati"]);

    let kr = d("kF", Tudadi);
    assert_has_tip(&[], &san(&kr), Lat, &["cikarizati"]);

    let gr = d("gF", Tudadi);
    assert_has_tip(&[], &san(&gr), Lat, &["jigarizati", "jigalizati"]);

    let dr = d("df\\N", Tudadi);
    assert_has_ta(&[], &san(&dr), Lat, &["didarizate"]);

    let dhr = d("Df\\N", Tudadi);
    assert_has_ta(&[], &san(&dhr), Lat, &["diDarizate"]);
}

#[test]
fn sk_2612() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &san(&bhu), Lat, &["buBUzati"]);

    let di = d("dI\\N", Divadi);
    assert_has_ta(&[], &san(&di), Lat, &["didIzate"]);
}

#[test]
fn sk_2613() {
    let guh = d("guhU~^", Bhvadi);
    assert_has_tip(&[], &san(&guh), Lat, &["juGukzati"]);

    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_tip(&[], &san(&bhid), Lat, &["biBitsati"]);

    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_ta(&[], &san(&yaj), Lat, &["yiyakzate"]);

    let vrdh = d("vfDu~\\", Bhvadi);
    assert_has_ta(&[], &san(&vrdh), Lat, &["vivarDizate"]);

    let trnh = d("tfnhU~", Tudadi);
    assert_has_tip(&[], &san(&trnh), Lat, &["titfkzati", "titfMhizati"]);
}

#[test]
fn sk_2614() {
    let ji = d("ji\\", Bhvadi);
    assert_has_tip(&[], &san(&ji), Lat, &["jigIzati"]);

    let ci = d("ci\\Y", Svadi);
    assert_has_tip(&[], &san(&ci), Lat, &["cikIzati", "cicIzati"]);

    let han = d("ha\\na~", Adadi);
    assert_has_tip(&[], &san(&han), Lat, &["jiGAMsati"]);
}

// All pass except pratIzizati.
#[test]
fn sk_2615() {
    let i = d("i\\R", Adadi);
    assert_has_tip(&[], &san(&i), Lat, &["jigamizati"]);
    assert_has_tip(&["prati"], &san(&i), Lat, &["pratIzizati"]);

    let ik = d("i\\k", Adadi);
    assert_has_tip(&["aDi"], &san(&ik), Lat, &["aDijigamizati"]);
    assert_has_ta_k(&[], &san(&ik), Lat, &["jigAMsyate"]);
    assert_has_ta_k(&["aDi"], &san(&ik), Lat, &["aDijigAMsyate"]);

    let gam = d("ga\\mx~", Adadi);
    assert_has_ta_k(&[], &san(&gam), Lat, &["jigaMsyate"]);
    assert_has_ta_k(&["sam"], &san(&gam), Lat, &["saYjigaMsyate"]);
}

#[test]
fn sk_2616() {
    let i = d("i\\N", Adadi);
    assert_has_ta(&["aDi"], &san(&i), Lat, &["aDijigAMsate"]);
}

#[test]
fn sk_2617() {
    let dyut = d("dyuta~\\", Bhvadi);
    assert_has_ta(&[], &san(&dyut), Lat, &["didyutizate", "didyotizate"]);

    let ruc = d("ruca~\\", Bhvadi);
    assert_has_ta(&[], &san(&ruc), Lat, &["rurucizate", "rurocizate"]);

    // lileKizati is from KV 1.2.26.
    let likh = d("liKa~", Tudadi);
    assert_has_tip(&[], &san(&likh), Lat, &["liliKizati", "lileKizati"]);

    // dudyUzati is from SK 2618.
    let div = d("divu~", Divadi);
    assert_has_tip(&[], &san(&div), Lat, &["didevizati", "dudyUzati"]);

    let vrt = d("vftu~\\", Bhvadi);
    assert_has_ta(&[], &san(&vrt), Lat, &["vivartizate"]);

    let ish = d("iza~", Divadi);
    assert_has_tip(&[], &san(&ish), Lat, &["ezizizati"]);
}

#[test]
fn sk_2618() {
    let vrt = d("vftu~\\", Bhvadi);
    assert_has_ta(&[], &san(&vrt), Lat, &["vivartizate"]);

    let siv = d("zivu~", Divadi);
    assert_has_tip(&[], &san(&siv), Lat, &["susyUzati", "sisevizati"]);
}

#[test]
fn skip_sk_2619() {}

#[test]
fn sk_2620() {
    let aap = d("A\\px~", Svadi);
    assert_has_tip(&[], &san(&aap), Lat, &["Ipsati"]);

    let rdh = d("fDu~", Svadi);
    assert_has_tip(&[], &san(&rdh), Lat, &["Irtsati", "ardiDizati"]);

    let bhrasj = d("Bra\\sja~^", Tudadi);
    assert_has_tip(
        &[],
        &san(&bhrasj),
        Lat,
        &["biBrajjizati", "biBarjizati", "biBrakzati", "biBarkzati"],
    );
}

#[test]
fn sk_2621() {
    let dambh = &d("danBu~", Svadi);
    assert_has_tip(
        &[],
        &san(dambh),
        Lat,
        &["didamBizati", "Dipsati", "DIpsati"],
    );

    let shri = &d("SriY", Bhvadi);
    assert_has_tip(&[], &san(shri), Lat, &["SiSrIzati", "SiSrayizati"]);

    let svr = &d("svf", Bhvadi);
    assert_has_tip(&[], &san(svr), Lat, &["susvUrzati", "sisvarizati"]);

    let yu = &d("yu", Adadi);
    assert_has_tip(&[], &san(yu), Lat, &["yuyUzati", "yiyavizati"]);

    let urnu = &d("UrRuY", Adadi);
    assert_has_tip(
        &[],
        &san(urnu),
        Lat,
        &["UrRunUzati", "UrRunuvizati", "UrRunavizati"],
    );

    let bhr = &d("Bf\\Y", Bhvadi);
    assert_has_tip(&[], &san(bhr), Lat, &["buBUrzati", "biBarizati"]);

    let jnap = &d("jYapa~", Curadi);
    assert_has_san_tip(&[], &jnap, &["jYIpsati", "jijYapayizati"]);

    let jna = &d("jYA", Curadi);
    assert_has_san_tip(&[], &jna, &["jijYApayizati"]);

    let sana = &d("zaRa~", Bhvadi);
    assert_has_tip(&[], &san(sana), Lat, &["sizAsati", "sisanizati"]);
}

#[test]
fn sk_2622() {
    assert_has_san_tip(
        &[],
        &d("tanu~^", Tanadi),
        &["titAMsati", "titaMsati", "titanizati"],
    );

    let mr = d("mf\\N", Tudadi);
    assert_has_tip(&[], &san(&mr), Lat, &["mumUrzati"]);

    // For "pitsati" see SK 2623.
    let pat = d("patx~", Bhvadi);
    assert_has_tip(&[], &san(&pat), Lat, &["pipatizati", "pitsati"]);
}

#[test]
fn sk_2623() {
    assert_has_san_tip(&[], &d("patx~", Bhvadi), &["pipatizati", "pitsati"]);

    let daridra = &d("daridrA", Adadi);
    assert_has_san_tip(&[], &daridra, &["didaridrizati", "didaridrAsati"]);

    assert_has_tip(&[], &san(&d("mI\\Y", Kryadi)), Lat, &["mitsati"]);
    assert_has_tip(&[], &san(&d("qumi\\Y", Svadi)), Lat, &["mitsati"]);
    assert_has_tip(&[], &san(&d("do\\", Divadi)), Lat, &["ditsati"]);
    assert_has_tip(&[], &san(&d("dA\\R", Bhvadi)), Lat, &["ditsati"]);
    assert_has_ta(&[], &san(&d("de\\N", Bhvadi)), Lat, &["ditsate"]);

    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_tip(&[], &san(&daa), Lat, &["ditsati"]);
    assert_has_ta(&[], &san(&daa), Lat, &["ditsate"]);

    assert_has_tip(&[], &san(&d("De\\w", Bhvadi)), Lat, &["Ditsati"]);

    let dhaa = d("quDA\\Y", Juhotyadi);
    assert_has_tip(&[], &san(&dhaa), Lat, &["Ditsati"]);
    assert_has_ta(&[], &san(&&dhaa), Lat, &["Ditsate"]);

    let rabh = &d("ra\\Ba~\\", Bhvadi);
    assert_has_ta(&[], &san(&rabh), Lat, &["ripsate"]);

    let labh = &d("qula\\Ba~\\z", Bhvadi);
    assert_has_ta(&[], &san(&labh), Lat, &["lipsate"]);

    assert_has_tip(&[], &san(&d("Sa\\kx~", Svadi)), Lat, &["Sikzati"]);

    // SiSakizati seems justified because Sa\\kx~ is optionally seT.
    let shak = &d("Sa\\ka~^", Divadi);
    assert_has_tip(&[], &san(&shak), Lat, &["Sikzati", "SiSakizati"]);
    assert_has_ta(&[], &san(&shak), Lat, &["Sikzate", "SiSakizate"]);

    assert_has_ta(&[], &san(&d("pa\\da~\\", Divadi)), Lat, &["pitsate"]);

    // TODO: this produces the right forms slightly overgenerates since we don't check for
    // "hiMsAyAm".
    let radh = d("rA\\Da~", Divadi);
    assert_has_tip(&[], &san(&radh), Lat, &["ritsati", "rirAtsati"]);
    assert_has_tip(&["AN"], &san(&radh), Lat, &["Aritsati", "ArirAtsati"]);
}

#[ignore]
#[test]
fn sk_2624() {
    let muc = d("mu\\cx~^", Tudadi);
    assert_has_ta_k(&[], &san(&muc), Lat, &["mokzate", "mumukzate"]);
    assert_has_san_tip(&[], &muc, &["mumukzati"]);
}

#[test]
fn sk_2625() {
    assert_has_san_tip(
        &[],
        &d("tF", Bhvadi),
        &["titarizati", "titarIzati", "titIrzati"],
    );

    assert_has_san_tip(
        &[],
        &d("vfY", Svadi),
        &["vivarizati", "vivarIzati", "vuvUrzati"],
    );

    // vivarIzate is justified by KV 7.2.41.
    assert_has_san_ta(
        &[],
        &d("vfN", Kryadi),
        &["vuvUrzate", "vivarizate", "vivarIzate"],
    );
    assert_has_san_tip(&[], &d("Dvf\\", Bhvadi), &["duDvUrzati"]);
}

#[ignore]
#[test]
fn sk_2626() {
    assert_has_san_ta(&[], &d("zmi\\N", Bhvadi), &["sismayizate"]);
    assert_has_san_ta(&[], &d("pUN", Bhvadi), &["pipavizate"]);
    assert_has_san_tip(&[], &d("f\\", Bhvadi), &["aririzati"]);
    assert_has_san_tip(&[], &d("anjU~", Rudhadi), &["aYjijizati"]);
    assert_has_san_ta(&[], &d("aSU~\\", Svadi), &["aSiSizate"]);
    assert_has_san_tip(&["pra"], &d("ana~", Adadi), &["prARiRizati"]);
    assert_has_san_tip(&[], &d("uCI~", Tudadi), &["ucicCizati"]);

    let i = d("i\\N", Adadi);
    assert_has_tip(
        &["aDi"],
        &nic_san(&i),
        Lat,
        &["aDijigApayizati", "aDyApipayizati"],
    );

    let svi = d("wuo~Svi", Bhvadi);
    assert_has_tip(&[], &nic_san(&svi), Lat, &["SiSvAyayizati", "SuSAvayizati"]);

    assert_has_tip(&[], &nic_san(&d("hu\\", Juhotyadi)), Lat, &["juhAvayizati"]);
    assert_has_tip(
        &[],
        &nic_san(&d("hu\\", Juhotyadi)),
        Lat,
        &["pusPArayizati"],
    );
    assert_has_tip(
        &[],
        &nic_san(&d("hu\\", Juhotyadi)),
        Lat,
        &["cukzAyayizati"],
    );
    assert_has_tip(&[], &nic_san(&d("hu\\", Juhotyadi)), Lat, &["pipAvayizati"]);
    assert_has_tip(&[], &nic_san(&d("BU", Bhvadi)), Lat, &["biBAvayizati"]);
    assert_has_tip(&[], &nic_san(&d("ju\\", Bhvadi)), Lat, &["jijAyayizati"]);
    assert_has_tip(&[], &nic_san(&d("Ru", Adadi)), Lat, &["nunAvayizati"]);
    assert_has_san_tip(&[], &d("BU", Bhvadi), &["buBUzati"]);
    assert_has_tip(
        &[],
        &nic_san(&d("sru\\", Bhvadi)),
        Lat,
        &["sisrAvayizati", "susrAvayizati"],
    );
}

#[ignore]
#[test]
fn sk_2627() {
    assert_has_san_tip(&[], &d("zwu\\Y", Adadi), &["tuzwUzati"]);
    assert_has_tip(
        &[],
        &nic_san(&d("Yizva\\pa~", Adadi)),
        Lat,
        &["suzvApayizati"],
    );
    // i -> A is optional by 6.1.49
    assert_has_tip(
        &[],
        &nic_san(&d("zi\\Du~", Divadi)),
        Lat,
        &["sizADayizati", "sizeDayizati"],
    );

    let sic = &d("zi\\ca~^", Tudadi);
    assert_has_san_tip(&[], &sic, &["sisikzati"]);
    assert_has_san_tip(&["pari"], &sic, &["parizizikzati"]);
    assert_has_san_tip(&[], &d("zWA\\", Bhvadi), &["tizWAsati"]);
    assert_has_san_tip(&[], &d("Yizva\\pa~", Adadi), &["suzupsati"]);
    assert_has_san_tip(&["prati"], &d("i\\R", Adadi), &["pratIzizati"]);
    assert_has_san_tip(&["aDi"], &d("i\\k", Adadi), &["aDIzizati"]);
}

#[test]
fn sk_2628() {
    let svid = d("YizvidA~", Bhvadi);
    assert_has_tip(&[], &nic_san(&svid), Lat, &["sisvedayizati"]);

    let svad = d("zvada~\\", Bhvadi);
    assert_has_tip(&[], &nic_san(&svad), Lat, &["sisvAdayizati"]);

    let sah = d("zaha~\\", Bhvadi);
    assert_has_tip(&[], &nic_san(&sah), Lat, &["sisAhayizati"]);

    let sush = d("zu\\Y", Svadi);
    assert_has_tip(&["aBi"], &san(&sush), Lat, &["aBisusUzati"]);

    assert_has_san_ta(&[], &d("gupa~\\", Bhvadi), &["jugupsizate"]);
    assert_has_san_ta(&[], &d("mAna~\\", Bhvadi), &["mImAMsizate"]);
}
