extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

fn prati(text: &str) -> Pratipadika {
    Pratipadika::new(text)
}

fn stri(text: &str) -> Pratipadika {
    Pratipadika::builder()
        .text(text)
        .is_nyap(true)
        .build()
        .unwrap()
}

#[test]
fn sutra_1_1_20() {
    assert_has_lat_p(&["pra", "ni"], &d("qudA\\Y", Juhotyadi), &["praRidadAti"]);
    assert_has_krdanta(
        &["pra", "ni"],
        &d("dA\\R", Bhvadi),
        Krt::tfc,
        &["praRidAtf"],
    );
    assert_has_lat(&["pra", "ni"], &d("do\\", Divadi), &["praRidyati"]);
    assert_has_lat(&["pra", "ni"], &d("de\\N", Bhvadi), &["praRidayate"]);
    assert_has_lat_p(&["pra", "ni"], &d("quDA\\Y", Juhotyadi), &["praRidaDAti"]);
    assert_has_lat_p(&["pra", "ni"], &d("De\\w", Bhvadi), &["praRiDayati"]);
    // adAp
    assert_has_krdanta(&[], &d("dA\\p", Adadi), Krt::kta, &["dAta"]);
    assert_has_krdanta(&["ava"], &d("dE\\p", Bhvadi), Krt::kta, &["avadAta"]);
}

#[test]
fn sutra_1_1_26() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::kta, &["kfta"]);
    assert_has_krdanta(&[], &kf, Krt::ktavatu, &["kftavat"]);

    let bhuj = d("Bu\\ja~", Rudhadi);
    assert_has_krdanta(&[], &bhuj, Krt::kta, &["Bukta"]);
    assert_has_krdanta(&[], &bhuj, Krt::ktavatu, &["Buktavat"]);
}

#[test]
fn sutra_1_1_27() {
    // use Taddhita as T;
    assert_has_subantas("sarva", Pum, Prathama, Eka, &["sarvaH"]);
    assert_has_subantas("sarva", Pum, Prathama, Dvi, &["sarvO"]);
    assert_has_subantas("sarva", Pum, Prathama, Bahu, &["sarve"]);
    assert_has_subantas("sarva", Pum, Caturthi, Eka, &["sarvasmE"]);
    assert_has_subantas("sarva", Pum, Panchami, Eka, &["sarvasmAt"]);
    assert_has_subantas("sarva", Pum, Sasthi, Bahu, &["sarvezAm"]);
    assert_has_subantas("sarva", Pum, Saptami, Eka, &["sarvasmin"]);
    // assert_has_taddhitanta(&prati("sarva"), T::akac, &["sarvaka"]);

    assert_has_subantas("viSva", Pum, Prathama, Eka, &["viSvaH"]);
    assert_has_subantas("viSva", Pum, Prathama, Dvi, &["viSvO"]);
    assert_has_subantas("viSva", Pum, Prathama, Bahu, &["viSve"]);
    assert_has_subantas("viSva", Pum, Caturthi, Eka, &["viSvasmE"]);
    assert_has_subantas("viSva", Pum, Panchami, Eka, &["viSvasmAt"]);
    assert_has_subantas("viSva", Pum, Sasthi, Bahu, &["viSvezAm"]);
    assert_has_subantas("viSva", Pum, Saptami, Eka, &["viSvasmin"]);
    // assert_has_taddhitanta(&prati("viSva"), T::akac, &["viSvaka"]);

    assert_has_subantas("uBaya", Pum, Prathama, Bahu, &["uBaye"]);
    assert_has_subantas("uBaya", Pum, Caturthi, Eka, &["uBayasmE"]);
    assert_has_subantas("uBaya", Pum, Panchami, Eka, &["uBayasmAt"]);
    assert_has_subantas("uBaya", Pum, Sasthi, Bahu, &["uBayezAm"]);
    assert_has_subantas("uBaya", Pum, Saptami, Eka, &["uBayasmin"]);
    // qatara
    assert_has_subantas("katara", Pum, Caturthi, Eka, &["katarasmE"]);
    assert_has_subantas("katama", Pum, Caturthi, Eka, &["katamasmE"]);
    // itara, etc.
    assert_has_subantas("itara", Pum, Caturthi, Eka, &["itarasmE"]);
    assert_has_subantas("anya", Pum, Caturthi, Eka, &["anyasmE"]);
    assert_has_subantas("anyatara", Pum, Caturthi, Eka, &["anyatarasmE"]);

    // TODO: others
}

#[test]
fn sutra_1_1_33() {
    assert_has_subantas("praTama", Pum, Prathama, Bahu, &["praTame", "praTamAH"]);
    assert_has_subantas("carama", Pum, Prathama, Bahu, &["carame", "caramAH"]);
    assert_has_subantas("alpa", Pum, Prathama, Bahu, &["alpe", "alpAH"]);
    assert_has_subantas("arDa", Pum, Prathama, Bahu, &["arDe", "arDAH"]);
    assert_has_subantas("katipaya", Pum, Prathama, Bahu, &["katipaye", "katipayAH"]);
    assert_has_subantas("nema", Pum, Prathama, Bahu, &["neme", "nemAH"]);

    // TODO: taya
}

#[test]
fn sutra_1_1_34() {
    assert_has_subantas("pUrva", Pum, Prathama, Bahu, &["pUrve", "pUrvAH"]);
    assert_has_subantas("para", Pum, Prathama, Bahu, &["pare", "parAH"]);
    assert_has_subantas("avara", Pum, Prathama, Bahu, &["avare", "avarAH"]);
    assert_has_subantas("dakziRa", Pum, Prathama, Bahu, &["dakziRe", "dakziRAH"]);
    assert_has_subantas("uttara", Pum, Prathama, Bahu, &["uttare", "uttarAH"]);
    assert_has_subantas("apara", Pum, Prathama, Bahu, &["apare", "aparAH"]);
    assert_has_subantas("aDara", Pum, Prathama, Bahu, &["aDare", "aDarAH"]);
}

#[test]
fn sutra_1_1_42() {
    assert_has_subantas("kuRqa", Napumsaka, Prathama, Bahu, &["kuRqAni"]);
    assert_has_subantas("kuRqa", Napumsaka, Dvitiya, Bahu, &["kuRqAni"]);
    assert_has_subantas("daDi", Napumsaka, Prathama, Bahu, &["daDIni"]);
    assert_has_subantas("maDu", Napumsaka, Prathama, Bahu, &["maDUni"]);
    assert_has_subantas("trapu", Napumsaka, Prathama, Bahu, &["trapURi"]);
    assert_has_subantas("jatu", Napumsaka, Prathama, Bahu, &["jatUni"]);
}

#[test]
fn sutra_1_1_43() {
    assert_has_subantas("rAjan", Pum, Prathama, Eka, &["rAjA"]);
    assert_has_subantas("rAjan", Pum, Prathama, Dvi, &["rAjAnO"]);
    assert_has_subantas("rAjan", Pum, Prathama, Bahu, &["rAjAnaH"]);
    assert_has_subantas("rAjan", Pum, Dvitiya, Eka, &["rAjAnam"]);
    assert_has_subantas("rAjan", Pum, Dvitiya, Dvi, &["rAjAnO"]);
    // suw
    assert_has_subantas("rAjan", Pum, Dvitiya, Bahu, &["rAjYaH"]);
    // anapuMsakasya
    assert_has_subantas("sAman", Napumsaka, Prathama, Dvi, &["sAmanI", "sAmnI"]);
    assert_has_subantas("veman", Napumsaka, Prathama, Dvi, &["vemanI", "vemnI"]);
}

#[test]
fn sutra_1_1_73() {
    use Taddhita as T;
    assert_has_taddhitanta(&stri("SAlA"), T::Ca, &["SAlIya"]);
    assert_has_taddhitanta(&stri("mAlA"), T::Ca, &["mAlIya"]);
    assert_has_taddhitanta(&prati("Opagava"), T::Ca, &["OpagavIya"]);
    assert_has_taddhitanta(&prati("kApawava"), T::Ca, &["kApawavIya"]);
    // TODO: do others
}
