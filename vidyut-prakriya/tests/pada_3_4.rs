extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_3_4_78() {
    use Pada::*;
    use Purusha::*;
    use Vacana::*;

    let pac = Dhatu::new("qupa\\ca~^z", Gana::Bhvadi);
    let items = &[
        (Prathama, Eka, Parasmai, "pacati"),
        (Prathama, Dvi, Parasmai, "pacataH"),
        (Prathama, Bahu, Parasmai, "pacanti"),
        (Madhyama, Eka, Parasmai, "pacasi"),
        (Madhyama, Dvi, Parasmai, "pacaTaH"),
        (Madhyama, Bahu, Parasmai, "pacaTa"),
        (Uttama, Eka, Parasmai, "pacAmi"),
        (Uttama, Dvi, Parasmai, "pacAvaH"),
        (Uttama, Bahu, Parasmai, "pacAmaH"),
        (Prathama, Eka, Atmane, "pacate"),
        (Prathama, Dvi, Atmane, "pacete"),
        (Prathama, Bahu, Atmane, "pacante"),
        (Madhyama, Eka, Atmane, "pacase"),
        (Madhyama, Dvi, Atmane, "paceTe"),
        (Madhyama, Bahu, Atmane, "pacaDve"),
        (Uttama, Eka, Atmane, "pace"),
        (Uttama, Dvi, Atmane, "pacAvahe"),
        (Uttama, Bahu, Atmane, "pacAmahe"),
    ];
    for (purusha, vacana, pada, expected) in items {
        let args = TinantaArgs::builder()
            .prayoga(Prayoga::Kartari)
            .purusha(*purusha)
            .vacana(*vacana)
            .lakara(Lakara::Lat)
            .pada(*pada)
            .build()
            .unwrap();
        let actual = derive_tinantas(&pac, &args);
        assert_padas(actual, &[expected])
    }
}
