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
