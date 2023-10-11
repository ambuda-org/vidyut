extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

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

#[ignore]
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

#[ignore]
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

#[ignore]
#[test]
fn sk_2615() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_tip(&[], &san(&gam), Lat, &["jigamizati"]);

    let i = d("i\\R", Adadi);
    assert_has_tip(&["aDi"], &san(&i), Lat, &["aDijigamizati"]);
    assert_has_ta_k(&[], &san(&gam), Lat, &["jigAMsyate"]);
    assert_has_ta_k(&["aDi"], &san(&i), Lat, &["aDijigAMsyate"]);
    assert_has_ta_k(&[], &san(&gam), Lat, &["jigaMsyate"]);
    assert_has_ta_k(&["sam"], &san(&gam), Lat, &["saMjigaMsyate"]);
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

#[ignore]
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
fn sk_2622() {
    let tan = d("tanu~^", Tanadi);
    assert_has_tip(
        &[],
        &san(&tan),
        Lat,
        &["titAMsati", "titaMsati", "titanizati"],
    );
}
