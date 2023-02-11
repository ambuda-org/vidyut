use vidyut_prakriya::args::*;
use vidyut_prakriya::Ashtadhyayi;

fn derive(a: &Ashtadhyayi, dhatu: &Dhatu, args: &TinantaArgs) -> Vec<String> {
    a.derive_tinantas(dhatu, args)
        .iter()
        .map(|p| p.text().to_string())
        .collect()
}

fn contains(results: &[String], item: &'static str) -> bool {
    results.iter().any(|x| *x == item)
}

#[test]
fn sutra_7_2_62() {
    let a = Ashtadhyayi::new();
    let thal = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lit)
        .build()
        .unwrap();

    let yaj = Dhatu::new("ya\\ja~^", Gana::Bhvadi);
    let results = derive(&a, &yaj, &thal);
    assert!(contains(&results, "iyazWa"));
    assert!(contains(&results, "iyajiTa"));
}

#[test]
fn sutra_7_2_66() {
    let a = Ashtadhyayi::new();
    let thal = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Eka)
        .lakara(Lakara::Lit)
        .build()
        .unwrap();

    let ad = Dhatu::new("a\\da~", Gana::Adadi);
    let results = derive(&a, &ad, &thal);
    assert!(contains(&results, "AdiTa"));
    assert!(!contains(&results, "AtTa"));

    let f = Dhatu::new("f\\", Gana::Bhvadi);
    let results = derive(&a, &f, &thal);
    assert!(contains(&results, "AriTa"));

    let vye = Dhatu::new("vye\\Y", Gana::Bhvadi);
    let results = derive(&a, &vye, &thal);
    assert!(contains(&results, "vivyayiTa"));
    assert!(!contains(&results, "vivyeTa"));
}

#[test]
fn sutra_8_3_79() {
    let a = Ashtadhyayi::new();

    let shidhvam = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::AshirLin)
        .build()
        .unwrap();

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

    // Examples from Kashika Vrtti
    let lu = Dhatu::new("lUY", Gana::Kryadi);
    let results = derive(&a, &lu, &shidhvam);
    assert!(contains(&results, "lavizIQvam"));
    assert!(contains(&results, "lavizIDvam"));

    let pu = Dhatu::new("pUY", Gana::Kryadi);
    let results = derive(&a, &pu, &shidhvam);
    assert!(contains(&results, "pavizIQvam"));
    assert!(contains(&results, "pavizIDvam"));

    let results = derive(&a, &lu, &lun);
    assert!(contains(&results, "alaviQvam"));
    assert!(contains(&results, "alaviDvam"));

    let results = derive(&a, &lu, &lit);
    assert!(contains(&results, "luluviQve"));
    assert!(contains(&results, "luluviDve"));

    let aas = Dhatu::new("Asa~\\", Gana::Adadi);
    let results = derive(&a, &aas, &shidhvam);
    assert!(!contains(&results, "AsizIQvam"));
    assert!(contains(&results, "AsizIDvam"));

    // Other cases
    let kr = Dhatu::new("qukf\\Y", Gana::Tanadi);
    let results = derive(&a, &kr, &lit);
    assert!(contains(&results, "cakfQve"));
    assert!(!contains(&results, "cakfDve"));
}
