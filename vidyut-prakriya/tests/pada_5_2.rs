extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Artha::*;
use vidyut_prakriya::args::Taddhita as T;

#[ignore]
#[test]
fn sutra_5_2_1() {
    let artha = DhanyanamBhavaneKshetre;
    assert_has_artha_taddhita("mudra", artha, T::KaY, &["mOdrIna"]);
    assert_has_artha_taddhita("kudrava", artha, T::KaY, &["kOdravIna"]);
    assert_has_artha_taddhita("kulatTa", artha, T::KaY, &["kOlatTIna"]);
}

#[test]
fn sutra_5_2_2() {
    let artha = DhanyanamBhavaneKshetre;
    assert_has_artha_taddhita("vrIhi", artha, T::Qak, &["vrEheya"]);
    assert_has_artha_taddhita("SAli", artha, T::Qak, &["SAleya"]);
}

#[test]
fn sutra_5_2_3() {
    let artha = DhanyanamBhavaneKshetre;
    assert_has_artha_taddhita("yava", artha, T::yat, &["yavya"]);
    assert_has_artha_taddhita("yavaka", artha, T::yat, &["yavakya"]);
    assert_has_artha_taddhita("zazwika", artha, T::yat, &["zazwikya"]);
}

#[test]
fn sutra_5_2_4() {
    fn assert_yat_or_khan(prati: &str, yat: &str, khan: &str) {
        let artha = DhanyanamBhavaneKshetre;
        assert_has_artha_taddhita(prati, artha, T::yat, &[yat]);
        assert_has_artha_taddhita(prati, artha, T::KaY, &[khan]);
    }
    assert_yat_or_khan("tila", "tilya", "tElIna");
    assert_yat_or_khan("mAza", "mAzya", "mAzIRa");
    assert_yat_or_khan("umA", "umya", "OmIna");
    assert_yat_or_khan("BaNgA", "BaNgya", "BANgIna");
    assert_yat_or_khan("aRu", "aRavya", "ARavIna");
}

#[test]
fn sutra_5_2_5() {
    assert_has_artha_taddhita("sarvacarman", Krta, T::Ka, &["sarvacarmIRa"]);
    assert_has_artha_taddhita("sarvacarman", Krta, T::KaY, &["sArvacarmIRa"]);
}

#[test]
fn sutra_5_2_6() {
    assert_has_artha_taddhita("yaTAmuKa", Darshana, T::Ka, &["yaTAmuKIna"]);
    assert_has_artha_taddhita("sammuKa", Darshana, T::Ka, &["sammuKIna"]);
}

#[test]
fn sutra_5_2_7() {
    assert_has_artha_taddhita("sarvapaTi", Vyapnoti, T::Ka, &["sarvapaTIna"]);
    assert_has_artha_taddhita("sarvANga", Vyapnoti, T::Ka, &["sarvANgIRa"]);
    assert_has_artha_taddhita("sarvakarman", Vyapnoti, T::Ka, &["sarvakarmIRa"]);
    assert_has_artha_taddhita("sarvapatra", Vyapnoti, T::Ka, &["sarvapatrIRa"]);
    assert_has_artha_taddhita("sarvapAtra", Vyapnoti, T::Ka, &["sarvapAtrIRa"]);
}

#[test]
fn sutra_5_2_8() {
    assert_has_artha_taddhita("Aprapada", Prapnoti, T::Ka, &["AprapadIna"]);
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
