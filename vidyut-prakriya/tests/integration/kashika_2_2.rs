extern crate test_utils;
use test_utils::*;

#[test]
fn sutra_2_2_6() {
    assert_has_avyaya_tatpurusha("naY", "brAhmaRa", &["abrAhmaRa"]);
    assert_has_avyaya_tatpurusha("naY", "vfzala", &["avfzala"]);
}

#[test]
fn sutra_2_2_8() {
    assert_has_sasthi_tatpurusha("rAjan", "puruza", &["rAjapuruza"]);
    assert_has_sasthi_tatpurusha("brAhmaRa", "kambala", &["brAhmaRakambala"]);
}

#[ignore]
#[test]
fn sutra_2_2_24() {
    assert_has_bahuvrihi("UQa", "raTa", &["UQaraTa"]);
    assert_has_bahuvrihi("upagfta", "paSu", &["upagftapaSu"]);
    assert_has_bahuvrihi("udDfta", "odana", &["udDftOdana"]);
    assert_has_bahuvrihi("citra", "go", &["citragu"]);
}

#[test]
fn sutra_2_2_29() {
    assert_has_dvandva(&["plakza", "nyagroDa"], &["plakzanyagroDa"]);
    assert_has_dvandva(&["Dava", "Kadira", "palASa"], &["DavaKadirapalASa"]);
    assert_has_samahara_dvandva(&["vAc", "tvac"], &["vAktvaca"]);
    assert_has_samahara_dvandva(&["vAc", "dfzad"], &["vAgdfzada"]);
}
