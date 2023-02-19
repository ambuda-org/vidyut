///! Tests for specific rules.
///!
///! Although these test cases generally appear in our exhaustive test suite, these explicit
///! tests protect against accidental changes and clarify the underlying logic.
use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;

fn derive(a: &Ashtadhyayi, dhatu: &Dhatu, args: &TinantaArgs) -> Vec<String> {
    let mut results: Vec<_> = a
        .derive_tinantas(dhatu, args)
        .iter()
        .map(|p| p.text())
        .collect();
    results.sort();
    results
}

#[test]
fn sutra_8_3_79() {
    let a = Ashtadhyayi::new();

    let shidhvam = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::AshirLin)
        .pada(Pada::Atmane)
        .build()
        .unwrap();

    let lun = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::Lun)
        .pada(Pada::Atmane)
        .build()
        .unwrap();

    let lit = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::Lit)
        .pada(Pada::Atmane)
        .build()
        .unwrap();

    // Examples from Kashika Vrtti
    let lu = Dhatu::new("lUY", Gana::Kryadi);
    let results = derive(&a, &lu, &shidhvam);
    assert_eq!(results, vec!["lavizIDvam", "lavizIQvam"]);

    let pu = Dhatu::new("pUY", Gana::Kryadi);
    let results = derive(&a, &pu, &shidhvam);
    assert_eq!(results, vec!["pavizIDvam", "pavizIQvam"]);

    let results = derive(&a, &lu, &lun);
    assert_eq!(results, vec!["alaviDvam", "alaviQvam"]);

    let results = derive(&a, &lu, &lit);
    assert_eq!(results, vec!["luluviDve", "luluviQve"]);

    let aas = Dhatu::new("Asa~\\", Gana::Adadi);
    let results = derive(&a, &aas, &shidhvam);
    assert_eq!(results, vec!["AsizIDvam"]);

    // Other cases
    let kr = Dhatu::new("qukf\\Y", Gana::Tanadi);
    let results = derive(&a, &kr, &lit);
    assert_eq!(results, vec!["cakfQve"]);
}
