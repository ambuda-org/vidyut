extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Taddhita as T;

#[test]
fn sutra_4_3_92() {
    assert_has_taddhitanta(&prati("SaRqika"), T::Yya, &["SARqikya"]);
    assert_has_taddhitanta(&prati("sarvasena"), T::Yya, &["sArvasenya"]);
}

#[test]
fn sutra_4_3_93() {
    assert_has_taddhitanta(&prati("sinDu"), T::aR, &["sEnDava"]);
    assert_has_taddhitanta(&prati("sinDu"), T::aY, &["sEnDava"]);
    assert_has_taddhitanta(&prati("varRu"), T::aR, &["vArRava"]);
    assert_has_taddhitanta(&prati("varRu"), T::aY, &["vArRava"]);
}

#[test]
fn sutra_4_3_94() {
    assert_has_taddhitanta(&prati("tUdI"), T::Qak, &["tOdeya"]);
    assert_has_taddhitanta(&prati("SAlAtura"), T::CaR, &["SAlAturIya"]);
    assert_has_taddhitanta(&prati("varmatI"), T::QaY, &["vArmateya"]);
    assert_has_taddhitanta(&prati("kUcavAra"), T::yak, &["kOcavArya"]);
}

#[ignore]
#[test]
fn sutra_4_3_98() {
    assert_has_taddhitanta(&prati("vAsudeva"), T::vun, &["vAsudevaka"]);
    assert_has_taddhitanta(&prati("arjuna"), T::vun, &["arjunaka"]);
}

#[test]
fn sutra_4_3_102() {
    assert_has_taddhitanta(&prati("tittiri"), T::CaR, &["tEttirIya"]);
    assert_has_taddhitanta(&prati("varatantu"), T::CaR, &["vAratantavIya"]);
    assert_has_taddhitanta(&prati("KaRqika"), T::CaR, &["KARqikIya"]);
    assert_has_taddhitanta(&prati("uKa"), T::CaR, &["OKIya"]);
}

#[test]
fn sutra_4_3_103() {
    assert_has_taddhitanta(&prati("kASyapa"), T::Rini, &["kASyapin"]);
    assert_has_taddhitanta(&prati("kOSika"), T::Rini, &["kOSikin"]);
}
