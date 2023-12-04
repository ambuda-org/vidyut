extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn sk_2576() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &nic(&bhu), Lat, &["BAvayati"]);
    assert_has_ta(&[], &nic(&bhu), Lat, &["BAvayate"]);
    assert_has_tip(
        &[],
        &nic(&bhu),
        Lit,
        &["BAvayAYcakAra", "BAvayAmbaBUva", "BAvayAmAsa"],
    );
}

#[test]
fn sk_2577() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &nic(&bhu), Lun, &["abIBavat"]);

    let pu = d("pUY", Kryadi);
    assert_has_tip(&[], &nic(&pu), Lun, &["apIpavat"]);

    let mu = d("mUN", Bhvadi);
    assert_has_tip(&[], &nic(&mu), Lun, &["amImavat"]);

    let yu = d("yu", Adadi);
    assert_has_tip(&[], &nic(&yu), Lun, &["ayIyavat"]);

    let ru = d("ru", Adadi);
    assert_has_tip(&[], &nic(&ru), Lun, &["arIravat"]);

    let lu = d("lUY", Kryadi);
    assert_has_tip(&[], &nic(&lu), Lun, &["alIlavat"]);

    let ju = d("ju", Adadi);
    assert_has_tip(&[], &nic(&ju), Lun, &["ajIjavat"]);
}

#[ignore]
#[test]
fn sk_2578() {
    let sru = d("sru\\", Bhvadi);
    assert_has_tip(&[], &nic(&sru), Lun, &["asisravat", "asusravat"]);

    let shas = d("SAsu~", Adadi);
    assert_has_tip(&[], &nic(&shas), Lun, &["aSaSAsat"]);

    let dhauk = d("QOkf~\\", Bhvadi);
    assert_has_tip(&[], &nic(&dhauk), Lun, &["aquQOkat"]);

    // TODO: how to derive acIcakAsat?
    // sanvallaGuni -> sanyataH? But if so, why is cakAs laGu?
    let cakas = d("cakAsf~", Adadi);
    assert_has_tip(&[], &nic(&cakas), Lun, &["acacakAsat"]);

    let cur = d("cura~", Curadi);
    assert_has_tip(&[], &cur, Lun, &["acUcurat"]);
}

#[ignore]
#[test]
fn sk_2579() {
    let i = d("i\\N", Adadi);
    assert_has_tip(&["aDi"], &nic(&i), Lun, &["aDyajIgapat", "aDyApipat"]);
}

#[test]
fn skip_sk_2581() {}

#[test]
fn sk_2585() {
    assert_has_tip(&[], &nic(&d("So\\", Divadi)), Lat, &["SAyayati"]);
    assert_has_tip(&[], &nic(&d("hve\\Y", Bhvadi)), Lat, &["hvAyayati"]);
}

#[ignore]
#[test]
fn sk_2586() {
    let hve = &d("hve\\Y", Bhvadi);
    assert_has_tip(&[], &nic(&hve), Lun, &["ajUhavat", "ajuhAvat"]);
}

#[test]
fn sk_2597() {
    let sphay = &d("sPAyI~\\", Bhvadi);
    assert_has_tip(&[], &nic(&sphay), Lat, &["sPAvayati"]);
}

#[test]
fn sk_2598() {
    let shad = d("Sa\\dx~", Bhvadi);
    assert_has_tip(&[], &nic(&shad), Lat, &["SAtayati", "SAdayati"]);
}

#[test]
fn sk_2599() {
    let ruh = d("ru\\ha~", Bhvadi);
    assert_has_tip(&[], &nic(&ruh), Lat, &["ropayati", "rohayati"]);
}

#[test]
fn sk_2600() {
    assert_has_tip(&[], &nic(&d("qukrI\\Y", Kryadi)), Lat, &["krApayati"]);
    assert_has_tip(&["aDi"], &nic(&d("i\\N", Adadi)), Lat, &["aDyApayati"]);
    assert_has_tip(&[], &nic(&d("ji\\", Bhvadi)), Lat, &["jApayati"]);
}

#[test]
fn sk_2602() {
    let sidh = d("zi\\Du~", Divadi);
    assert_has_tip(&[], &nic(&sidh), Lat, &["sADayati", "seDayati"]);
}

#[test]
fn sk_2603() {
    let guh = d("guhU~^", Bhvadi);
    assert_has_tip(&[], &nic(&guh), Lat, &["gUhayati"]);
}

#[test]
fn sk_2604() {
    let dus = d("du\\za~", Divadi);
    // dozayati by SK 2605.
    assert_has_tip(&[], &nic(&dus), Lat, &["dUzayati", "dozayati"]);
}

// All pass except apusParat
#[ignore]
#[test]
fn sk_2605() {
    let dus = d("du\\za~", Divadi);
    assert_has_tip(&[], &nic(&dus), Lat, &["dUzayati", "dozayati"]);

    assert_has_tip(&[], &nic(&d("Gawa~\\", Bhvadi)), Lat, &["Gawayati"]);
    assert_has_tip(&[], &nic(&d("janI~\\", Divadi)), Lat, &["janayati"]);
    assert_has_tip(&[], &nic(&d("jFz", Divadi)), Lat, &["jarayati"]);
    assert_has_tip(&[], &nic(&d("jF", Kryadi)), Lat, &["jArayati"]);
    assert_has_tip(
        &[],
        &nic(&d("ra\\nja~^", Divadi)),
        Lat,
        &["raYjayati", "rajayati"],
    );

    let ci_cur = d("ciY", Curadi);
    let cinoti = d("ci\\Y", Svadi);
    assert_has_tip(&[], &ci_cur, Lat, &["capayati", "cayayati"]);
    assert_has_tip(&[], &nic(&cinoti), Lat, &["cApayati", "cAyayati"]);

    let sphur = d("sPura~", Bhvadi);
    assert_has_tip(&[], &nic(&sphur), Lat, &["sPArayati", "sPorayati"]);
    assert_has_tip(&[], &nic(&sphur), Lun, &["apusPurat", "apusParat"]);
}

#[test]
fn sk_2606() {
    let an = d("ana~", Adadi);
    assert_has_tip(&["pra"], &nic(&an), Lun, &["prARiRat"]);
}
