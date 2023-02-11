use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;

#[test]
fn sutra_8_3_79() {
    let a = Ashtadhyayi::new();

    let derive = |dhatu, args| {
        let prakriyas = a.derive_tinantas(dhatu, args);
        let results: Vec<_> = prakriyas.iter().map(|p| p.text()).collect();
        results
    };

    let shidhvam = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::AshirLin)
        .build()
        .unwrap();

    let lu = Dhatu::new("lUY", Gana::Kryadi);
    let results = derive(&lu, &shidhvam);
    assert!(results.iter().any(|x| *x == "lavizIQvam"));
    assert!(results.iter().any(|x| *x == "lavizIDvam"));

    let pu = Dhatu::new("pUY", Gana::Kryadi);
    let results = derive(&pu, &shidhvam);
    assert!(results.iter().any(|x| *x == "pavizIQvam"));
    assert!(results.iter().any(|x| *x == "pavizIDvam"));

    let lun = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::Lun)
        .build()
        .unwrap();

    let lit = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::Lit)
        .build()
        .unwrap();

    let results = derive(&lu, &lun);
    assert!(results.iter().any(|x| *x == "alaviQvam"));
    assert!(results.iter().any(|x| *x == "alaviDvam"));

    let results = derive(&lu, &lit);
    assert!(results.iter().any(|x| *x == "luluviQve"));
    assert!(results.iter().any(|x| *x == "luluviDve"));

    let aas = Dhatu::new("Asa~\\", Gana::Adadi);
    let results = derive(&aas, &shidhvam);
    assert!(!results.iter().any(|x| *x == "AsizIQvam"));
    assert!(results.iter().any(|x| *x == "AsizIDvam"));
}
