/*!
Basic tests against our high-level API. These tests verify two important things:

1. That the API contract hasn't changed unexpectedly.
2. That the function can handle pathological input without panicking.
*/
extern crate test_utils;

use std::convert::TryInto;
use test_utils::assert_has_results;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Vyakarana;

/// Checks that Enum string representations exist and are 1:1.
#[test]
fn enum_as_str_and_from_string() {
    for val in Gana::iter() {
        assert_eq!(val, val.as_str().parse().expect("ok"));
    }

    for val in Krt::iter() {
        assert_eq!(val, val.as_str().parse().expect("ok"));
    }

    for val in Lakara::iter() {
        assert_eq!(val, val.as_str().parse().expect("ok"));
    }

    for val in Linga::iter() {
        assert_eq!(val, val.as_str().parse().expect("ok"));
    }

    for val in Prayoga::iter() {
        assert_eq!(val, val.as_str().parse().expect("ok"));
    }

    for val in Purusha::iter() {
        assert_eq!(val, val.as_str().parse().expect("ok"));
    }

    for val in Sanadi::iter() {
        assert_eq!(val, val.as_str().parse().expect("ok"));
    }

    for val in Taddhita::iter() {
        assert_eq!(val, val.as_str().parse().expect("ok"));
    }

    for val in Unadi::iter() {
        assert_eq!(val, val.as_str().parse().expect("ok"));
    }

    for val in Vacana::iter() {
        assert_eq!(val, val.as_str().parse().expect("ok"));
    }

    for val in Vibhakti::iter() {
        assert_eq!(val, val.as_str().parse().expect("ok"));
    }
}

fn assert_derive_dhatu(dhatu: Dhatu, expected: &[&str]) {
    let v = Vyakarana::new();
    let prakriyas = v.derive_dhatus(&dhatu);
    assert_has_results(prakriyas, expected);
}

fn mula(u: &str, gana: Gana) -> Dhatu {
    Dhatu::mula(u.try_into().expect("ok"), gana)
}

#[test]
fn derive_dhatus_for_mula_dhatus() {
    // Basic dhatus
    assert_derive_dhatu(mula("BU", Bhvadi), &["BU"]);
    assert_derive_dhatu(mula("eDa~\\", Bhvadi), &["eD"]);
    assert_derive_dhatu(mula("qukf\\Y", Tanadi), &["kf"]);
    assert_derive_dhatu(mula("gADf~\\", Bhvadi), &["gAD"]);
}

#[test]
fn derive_dhatus_for_mula_dhatus_and_upasargas() {
    let pranici = mula("ci\\Y", Svadi).with_prefixes(&["pra", "ni"]);
    assert_derive_dhatu(pranici, &["praRici"]);

    let sanskr = mula("qukf\\Y", Tanadi).with_prefixes(&["sam"]);
    assert_derive_dhatu(sanskr, &["saNkf", "saMskf", "saMkf"]);
}

#[test]
fn derive_dhatus_for_nama_dhatus() {
    // with explicit sanAdi
    assert_derive_dhatu(
        Dhatu::nama("putra".try_into().expect("ok"), Some(Sanadi::kyac)),
        &["putrIya"],
    );
    assert_derive_dhatu(
        Dhatu::nama("putra".try_into().expect("ok"), Some(Sanadi::kAmyac)),
        &["putrakAmya"],
    );

    // with inferred sanAdi
    assert_derive_dhatu(
        Dhatu::nama("lohita".try_into().expect("ok"), None),
        &["lohitAya"],
    );
}

#[test]
fn derive_tinantas() {
    let v = Vyakarana::new();
    let kr = mula("qukf\\Y", Gana::Tanadi);

    // Should get 2 results.
    let args_ubhaya = Tinanta::builder()
        .dhatu(kr.clone())
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();
    let prakriyas = v.derive_tinantas(&args_ubhaya);
    assert_has_results(prakriyas, &["karoti", "kurute"]);

    // Should get 1 parasmaipada result.
    let args_parasmai = Tinanta::builder()
        .dhatu(kr.clone())
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .pada(DhatuPada::Parasmai)
        .build()
        .unwrap();
    let prakriyas = v.derive_tinantas(&args_parasmai);
    assert_has_results(prakriyas, &["karoti"]);

    // Should get 1 Atmanepada result.
    let args_atmane = Tinanta::builder()
        .dhatu(kr.clone())
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .pada(DhatuPada::Atmane)
        .build()
        .unwrap();
    let prakriyas = v.derive_tinantas(&args_atmane);
    assert_has_results(prakriyas, &["kurute"]);
}

#[test]
fn derive_tinantas_with_invalid_dhatu() {
    let v = Vyakarana::new();

    let empty_dhatu = Dhatu::builder()
        .aupadeshika("")
        .gana(Gana::Tanadi)
        .build()
        .unwrap();

    let args = Tinanta::builder()
        .dhatu(empty_dhatu)
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();

    // The current API contract returns an empty vec if there's a severe error.
    let prakriyas = v.derive_tinantas(&args);
    assert_has_results(prakriyas, &[]);

    let invalid_dhatu = Dhatu::builder()
        .aupadeshika("k")
        .gana(Gana::Tanadi)
        .build()
        .unwrap();

    let args = args.with_dhatu(invalid_dhatu);
    let prakriyas = v.derive_tinantas(&args);
    assert_has_results(prakriyas, &[]);
}

#[test]
fn derive_krdantas() {
    let v = Vyakarana::new();

    let kr = Dhatu::builder()
        .aupadeshika("qukf\\Y")
        .gana(Gana::Tanadi)
        .build()
        .unwrap();

    let args = Krdanta::builder().dhatu(kr).krt(Krt::ktvA).build().unwrap();
    let prakriyas = v.derive_krdantas(&args);
    assert_has_results(prakriyas, &["kftvA"]);

    let kr_san = Dhatu::builder()
        .aupadeshika("qukf\\Y")
        .gana(Gana::Tanadi)
        .sanadi(&[Sanadi::Ric])
        .build()
        .unwrap();
    let krdanta = Krdanta::new(kr_san, Krt::ktvA);
    let prakriyas = v.derive_krdantas(&krdanta);
    assert_has_results(prakriyas, &["kArayitvA"]);
}
