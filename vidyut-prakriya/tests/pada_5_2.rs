extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Taddhita as T;

#[ignore]
#[test]
fn sutra_5_2_1() {
    assert_has_taddhitanta(&prati("mudra"), T::KaY, &["mOdrIna"]);
    assert_has_taddhitanta(&prati("kudrava"), T::KaY, &["kOdravIna"]);
    assert_has_taddhitanta(&prati("kulatTa"), T::KaY, &["kOlatTIna"]);
}

#[test]
fn sutra_5_2_2() {
    assert_has_taddhitanta(&prati("vrIhi"), T::Qak, &["vrEheya"]);
    assert_has_taddhitanta(&prati("SAli"), T::Qak, &["SAleya"]);
}

#[test]
fn sutra_5_2_3() {
    assert_has_taddhitanta(&prati("yava"), T::yat, &["yavya"]);
    assert_has_taddhitanta(&prati("yavaka"), T::yat, &["yavakya"]);
    assert_has_taddhitanta(&prati("zazwika"), T::yat, &["zazwikya"]);
}

#[test]
fn sutra_5_2_94() {
    assert_has_taddhitanta(&prati("go"), T::matup, &["gomat"]);
    assert_has_taddhitanta(&prati("vfkza"), T::matup, &["vfkzavat"]);
    assert_has_taddhitanta(&prati("yava"), T::matup, &["yavamat"]);
    assert_has_taddhitanta(&prati("plakza"), T::matup, &["plakzavat"]);
}

#[test]
fn sutra_5_2_96() {
    assert_has_taddhitanta(&prati("cUqA"), T::lac, &["cUqAla"]);
    assert_has_taddhitanta(&prati("cUqA"), T::matup, &["cUqAvat"]);
    // AtaH
    assert_has_taddhitanta(&prati("hasta"), T::matup, &["hastavat"]);
    assert_has_taddhitanta(&prati("pAda"), T::matup, &["pAdavat"]);
}

#[test]
fn sutra_5_2_100() {
    assert_has_taddhitanta(&prati("loman"), T::Sa, &["lomaSa"]);
    assert_has_taddhitanta(&prati("loman"), T::matup, &["lomavat"]);
    assert_has_taddhitanta(&prati("pAman"), T::na, &["pAmana"]);
    assert_has_taddhitanta(&prati("pAman"), T::matup, &["pAmavat"]);
    assert_has_taddhitanta(&prati("picCa"), T::ilac, &["picCila"]);
    assert_has_taddhitanta(&prati("picCa"), T::matup, &["picCavat"]);
    assert_has_taddhitanta(&prati("uras"), T::ilac, &["urasila"]);
    assert_has_taddhitanta(&prati("uras"), T::matup, &["urasvat"]);
}

#[test]
fn sutra_5_2_121() {
    assert_has_taddhitanta(&prati("yaSas"), T::vini, &["yaSasvin"]);
    assert_has_taddhitanta(&prati("payas"), T::vini, &["payasvin"]);
    assert_has_taddhitanta(&prati("mAyA"), T::vini, &["mAyAvin"]);
    assert_has_taddhitanta(&prati("meDA"), T::vini, &["meDAvin"]);
    assert_has_taddhitanta(&prati("sraj"), T::vini, &["sragvin"]);
}
