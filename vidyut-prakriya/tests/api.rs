/*!
Basic tests against our high-level API. These tests verify two important things:

1. That the API contract hasn't changed unexpectedly.
2. That the function can handle pathological input without panicking.
*/
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;

#[test]
fn derive_tinantas() {
    let a = Ashtadhyayi::new();
    let kr = Dhatu::builder()
        .upadesha("qukf\\Y")
        .gana(Gana::Tanadi)
        .build()
        .unwrap();

    // Should get 2 results.
    let args_ubhaya = Tinanta::builder()
        .dhatu(kr.clone())
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();
    let prakriyas = a.derive_tinantas(&args_ubhaya);
    let results: Vec<_> = prakriyas.iter().map(|x| x.text()).collect();
    assert!(results.iter().any(|x| x == "karoti"));
    assert!(results.iter().any(|x| x == "kurute"));

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
    let prakriyas = a.derive_tinantas(&args_parasmai);
    assert_eq!(prakriyas.len(), 1);
    assert!(prakriyas[0].text() == "karoti");

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
    let prakriyas = a.derive_tinantas(&args_atmane);
    assert_eq!(prakriyas.len(), 1);
    assert!(prakriyas[0].text() == "kurute");
}

#[test]
fn derive_tinantas_with_invalid_dhatu() {
    let a = Ashtadhyayi::new();

    let empty_dhatu = Dhatu::builder()
        .upadesha("")
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
    let prakriyas = a.derive_tinantas(&args);
    assert_eq!(prakriyas.len(), 0);

    let invalid_dhatu = Dhatu::builder()
        .upadesha("k")
        .gana(Gana::Tanadi)
        .build()
        .unwrap();

    let args = args.with_dhatu(invalid_dhatu);
    let prakriyas = a.derive_tinantas(&args);
    assert_eq!(prakriyas.len(), 0);
}

#[test]
fn derive_krdantas() {
    let a = Ashtadhyayi::new();

    let kr = Dhatu::builder()
        .upadesha("qukf\\Y")
        .gana(Gana::Tanadi)
        .build()
        .unwrap();

    let args = Krdanta::builder().dhatu(kr).krt(Krt::ktvA).build().unwrap();
    let prakriyas = a.derive_krdantas(&args);
    let results: Vec<_> = prakriyas.iter().map(|x| x.text()).collect();
    assert!(results.iter().any(|x| x == "kftvA"));

    let kr_san = Dhatu::builder()
        .upadesha("qukf\\Y")
        .gana(Gana::Tanadi)
        .sanadi(&[Sanadi::Ric])
        .build()
        .unwrap();
    let args = Krdanta::builder()
        .dhatu(kr_san)
        .krt(Krt::ktvA)
        .build()
        .unwrap();
    let prakriyas = a.derive_krdantas(&args);
    let results: Vec<_> = prakriyas.iter().map(|x| x.text()).collect();
    assert!(results.iter().any(|x| x == "kArayitvA"));
}
