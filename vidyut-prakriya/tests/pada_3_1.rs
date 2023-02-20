extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_3_1_67() {
    let d = Dhatu::new;
    assert_has_lat_karmani(&[], &d("Asa~\\", Gana::Adadi), &["Asyate"]);
    assert_has_lat_karmani(&[], &d("SIN", Gana::Adadi), &["Sayyate"]);
    assert_has_lat_karmani(&[], &d("qukf\\Y", Gana::Tanadi), &["kriyate"]);
    assert_has_lat_karmani(&[], &d("ga\\mx~", Gana::Bhvadi), &["gamyate"]);
    assert_has_lat_karmani(&[], &d("qupa\\ca~^z", Gana::Bhvadi), &["pacyate"]);
}

#[test]
fn sutra_3_1_68() {
    let d = Dhatu::new;
    assert_has_lat(&[], &d("BU", Gana::Bhvadi), &["Bavati"]);
    assert_has_lat(&[], &d("qupa\\ca~^z", Gana::Bhvadi), &["pacati", "pacate"]);
}

#[test]
fn sutra_3_1_69() {
    let d = Dhatu::new;
    assert_has_lat(&[], &d("divu~", Gana::Divadi), &["dIvyati"]);
    assert_has_lat(&[], &d("zivu~", Gana::Divadi), &["sIvyati"]);
}

#[test]
fn sutra_3_1_70() {
    let d = Dhatu::new;
    assert_has_lat(
        &[],
        &d("wuBrASf~\\", Gana::Bhvadi),
        &["BrASyate", "BrASate"],
    );
    assert_has_lat(
        &[],
        &d("wuBlASf~\\", Gana::Bhvadi),
        &["BlASyate", "BlASate"],
    );
    assert_has_lat(&[], &d("Bramu~", Gana::Divadi), &["BrAmyati", "Bramati"]);
    assert_has_lat_p(&[], &d("kramu~", Gana::Bhvadi), &["krAmyati", "krAmati"]);
    assert_has_lat(&[], &d("klamu~", Gana::Divadi), &["klAmyati", "klAmati"]);
    assert_has_lat(&[], &d("trasI~", Gana::Divadi), &["trasyati", "trasati"]);
    assert_has_lat(&[], &d("truwa~", Gana::Tudadi), &["truwyati", "truwati"]);
    assert_has_lat_p(&[], &d("laza~^", Gana::Bhvadi), &["lazyati", "lazati"]);
}

#[test]
fn sutra_3_1_73() {
    let d = Dhatu::new;
    assert_has_lat_p(&[], &d("zu\\Y", Gana::Svadi), &["sunoti"]);
    assert_has_lat_p(&[], &d("zi\\Y", Gana::Svadi), &["sinoti"]);
}

#[test]
fn sutra_3_1_74() {
    let d = Dhatu::new;
    assert_has_lat_p(&[], &d("Sru\\", Gana::Bhvadi), &["SfRoti"]);
}

#[test]
fn sutra_3_1_75() {
    let d = Dhatu::new;
    assert_has_lat_p(&[], &d("akzU~", Gana::Bhvadi), &["akzRoti", "akzati"]);
}

#[test]
fn sutra_3_1_76() {
    let d = Dhatu::new;
    assert_has_lat_p(&[], &d("takzU~", Gana::Bhvadi), &["takzRoti", "takzati"]);
}

#[test]
fn sutra_3_1_77() {
    let d = Dhatu::new;
    assert_has_lat_p(&[], &d("tu\\da~^", Gana::Tudadi), &["tudati"]);
    assert_has_lat_p(&[], &d("Ru\\da~^", Gana::Tudadi), &["nudati"]);
}

#[test]
fn sutra_3_1_78() {
    let d = Dhatu::new;
    assert_has_lat_p(&[], &d("ru\\Di~^r", Gana::Rudhadi), &["ruRadDi"]);
    assert_has_lat_p(&[], &d("Bi\\di~^r", Gana::Rudhadi), &["Binatti"]);
}

#[test]
fn sutra_3_1_79() {
    let d = Dhatu::new;
    assert_has_lat_p(&[], &d("tanu~^", Gana::Tanadi), &["tanoti"]);
    assert_has_lat_p(&[], &d("zaRu~^", Gana::Tanadi), &["sanoti"]);
    assert_has_lat_p(&[], &d("kzaRu~^", Gana::Tanadi), &["kzaRoti"]);
    assert_has_lat_p(&[], &d("qukf\\Y", Gana::Tanadi), &["karoti"]);

    assert_has_lun_a(&[], &d("qukf\\Y", Gana::Tanadi), &["akfta"]);
}

#[test]
fn sutra_3_1_80() {
    let d = Dhatu::new;
    assert_has_lat(&[], &d("Divi~", Gana::Bhvadi), &["Dinoti"]);
    assert_has_lat(&[], &d("kfvi~", Gana::Bhvadi), &["kfRoti"]);
}

#[test]
fn sutra_3_1_81() {
    let d = Dhatu::new;
    assert_has_lat_p(&[], &d("qukfI\\Y", Gana::Kryadi), &["krIRAti"]);
    assert_has_lat_p(&[], &d("prI\\Y", Gana::Kryadi), &["prIRAti"]);
}

#[test]
fn sutra_3_1_82() {
    let d = Dhatu::new;
    assert_has_lat(
        &[],
        &d("sta\\nBu~", Gana::Kryadi),
        &["staBnAti", "staBnoti"],
    );
    assert_has_lat(
        &[],
        &d("stu\\nBu~", Gana::Kryadi),
        &["stuBnAti", "stuBnoti"],
    );
    assert_has_lat(
        &[],
        &d("ska\\nBu~", Gana::Kryadi),
        &["skaBnAti", "skaBnoti"],
    );
    assert_has_lat(
        &[],
        &d("sku\\nBu~", Gana::Kryadi),
        &["skuBnAti", "skuBnoti"],
    );
    assert_has_lat_p(&[], &d("sku\\Y", Gana::Kryadi), &["skunAti", "skunoti"]);
}
