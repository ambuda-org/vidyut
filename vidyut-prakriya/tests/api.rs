/// Basic tests against our high-level API.
use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;

#[test]
fn derive_tinantas() {
    let a = Ashtadhyayi::new();
    let kr = Dhatu::builder()
        .upadesha("qukf\\Y")
        .gana(8)
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
    assert!(results.iter().any(|x| x == &"karoti"));
    assert!(results.iter().any(|x| x == &"kurute"));

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
