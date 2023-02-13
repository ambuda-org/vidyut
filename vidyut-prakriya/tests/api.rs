/// Basic tests against our high-level API. These tests verify two important things:
///
/// 1. That the API contract hasn't changed unexpectedly.
/// 2. That the function can handle pathological input without panicking.
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
    let args_ubhaya = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();
    let prakriyas = a.derive_tinantas(&kr, &args_ubhaya);
    let results: Vec<_> = prakriyas.iter().map(|x| x.text()).collect();
    assert!(results.iter().any(|x| x == "karoti"));
    assert!(results.iter().any(|x| x == "kurute"));

    // Should get 1 parasmaipada result.
    let args_parasmai = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .pada(Pada::Parasmai)
        .build()
        .unwrap();
    let prakriyas = a.derive_tinantas(&kr, &args_parasmai);
    assert_eq!(prakriyas.len(), 1);
    assert!(prakriyas[0].text() == "karoti");

    // Should get 1 Atmanepada result.
    let args_atmane = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .pada(Pada::Atmane)
        .build()
        .unwrap();
    let prakriyas = a.derive_tinantas(&kr, &args_atmane);
    assert_eq!(prakriyas.len(), 1);
    assert!(prakriyas[0].text() == "kurute");
}

#[test]
fn derive_tinantas_with_invalid_dhatu() {
    let a = Ashtadhyayi::new();
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Prathama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lat)
        .build()
        .unwrap();

    let empty_dhatu = Dhatu::builder()
        .upadesha("")
        .gana(Gana::Tanadi)
        .build()
        .unwrap();

    // The current API contract returns an empty vec if there's a severe error.
    let prakriyas = a.derive_tinantas(&empty_dhatu, &args);
    assert_eq!(prakriyas.len(), 0);

    let invalid_dhatu = Dhatu::builder()
        .upadesha("k")
        .gana(Gana::Tanadi)
        .build()
        .unwrap();

    let prakriyas = a.derive_tinantas(&invalid_dhatu, &args);
    assert_eq!(prakriyas.len(), 0);
}

#[test]
fn derive_krdantas() {
    let a = Ashtadhyayi::new();
    let args = KrdantaArgs::builder().krt(Krt::ktvA).build().unwrap();

    let kr = Dhatu::builder()
        .upadesha("qukf\\Y")
        .gana(Gana::Tanadi)
        .build()
        .unwrap();
    let prakriyas = a.derive_krdantas(&kr, &args);
    let results: Vec<_> = prakriyas.iter().map(|x| x.text()).collect();
    assert!(results.iter().any(|x| x == "kftvA"));

    let kr = Dhatu::builder()
        .upadesha("qukf\\Y")
        .gana(Gana::Tanadi)
        .sanadi(&[Sanadi::Nic])
        .build()
        .unwrap();
    let prakriyas = a.derive_krdantas(&kr, &args);
    let results: Vec<_> = prakriyas.iter().map(|x| x.text()).collect();
    assert!(results.iter().any(|x| x == "kArayitvA"));
}
