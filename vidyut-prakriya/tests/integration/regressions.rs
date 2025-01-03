//! Bug fixes that aren't captured in other tests.
//!
//! The tests here are responses to bug fixes that users have filed. Each should show that the bug
//! has been fixed and help ensure that the bug does not reappear.
extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Krdanta;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::{BaseKrt as Krt, Dhatu, Lakara, Prayoga, Taddhita};
use vidyut_prakriya::Vyakarana;

#[test]
fn ambibat() {
    assert_has_tip(&[], &nic(&d("abi~", Bhvadi)), Lun, &["Ambibat"]);
}

#[test]
fn aucicchat() {
    assert_has_tip(&[], &nic(&d("uCI~", Tudadi)), Lun, &["OcicCat"]);
}

#[test]
fn urjijiayizati() {
    // Given as Urj in sk.
    assert_has_tip(&[], &san(&d("Urja~", Curadi)), Lat, &["Urjijayizati"]);
}

#[test]
fn titikzizati() {
    assert_has_ta(&[], &san(&d("tija~\\", Bhvadi)), Lat, &["titikzizate"]);
}

#[test]
fn praipsat() {
    // Not a real regression, just want to make sure this works consistently.
    let aap = d("A\\px~", Svadi);
    assert_has_tip(&["pra"], &san(&aap), Lan, &["prEpsat"]);
}

#[test]
fn irshy_san_lan() {
    assert_has_tip(
        &[],
        &san(&d("Irzya~", Bhvadi)),
        Lan,
        &["Erzyiyizat", "Erzyizizat"],
    );
}

// Fixes https://github.com/ambuda-org/vidyut/issues/120
#[test]
fn ayiyat() {
    assert_has_tip(&[], &nic(&d("I\\N", Divadi)), Lun, &["Ayiyat"]);
}

// Fixes https://github.com/ambuda-org/vidyut/issues/121
#[test]
fn adhyajigapat_adhyapipat() {
    assert_has_tip(
        &["aDi"],
        &nic(&d("i\\N", Adadi)),
        Lun,
        &["aDyajIgapat", "aDyApipat"],
    );
}

// Verifies that Sap-pratyaya is added when Satf follows.
#[test]
fn shap_shatr() {
    let bhavat = krdanta(&[], &d("BU", Bhvadi), Krt::Satf);
    assert_has_sup_1s(&bhavat, Pum, &["Bavan"]);
    assert_has_sup_1d(&bhavat, Pum, &["BavantO"]);
    assert_has_sup_1p(&bhavat, Pum, &["BavantaH"]);
}

// Broken during refactor.
#[test]
fn tfmhyat() {
    assert_has_tip(&[], &d("tfha~", Rudhadi), VidhiLin, &["tfMhyAt"])
}

// Broken during refactor.
#[test]
fn acikat() {
    assert_has_tip(&[], &nic(&d_ghatadi("aka~", Bhvadi)), Lun, &["Acikat"])
}

// Broken during refactor.
#[test]
fn ahidi() {
    assert_has_ta_k(
        &[],
        &nic(&d_ghatadi("heqa~", Bhvadi)),
        Lun,
        &["ahIqi", "ahiqi"],
    )
}

#[test]
fn arpipat() {
    assert_has_tip(&[], &nic(&d("f\\", Bhvadi)), Lun, &["Arpipat"]);
}

#[test]
fn adhayishata() {
    assert_has_jha_k(
        &[],
        &d("De\\w", Bhvadi),
        Lun,
        &["aDAyizata", "aDizata", "adaDanta"],
    );
}

#[test]
fn aninivat() {
    assert_has_tip(&[], &nic(&d("RIva~", Bhvadi)), Lun, &["anInivat"]);
}

#[test]
fn vavati() {
    assert_has_tip(
        &[],
        &yan_luk(&d("vaya~\\", Bhvadi)),
        Lat,
        &["vAvati", "vAvayIti"],
    );
}

#[test]
fn rnjitva() {
    assert_has_krdanta(&[], &d("fji~\\", Bhvadi), Krt::ktvA, &["fYjitvA"]);
}

#[test]
fn bibhavaizitavya() {
    let krt = Krdanta::builder()
        .dhatu(nic_san(&d("BU", Bhvadi)))
        .krt(Krt::tavya)
        .build()
        .unwrap();
    assert_has_sup_1s(&krt, Stri, &["biBAvayizitavyA"]);
}

// Breaks if we process the wrong terms when doing ac sandhi for 6.4.51.
#[test]
fn bhavaniya_nic_sup_stri() {
    let krt = Krdanta::builder()
        .dhatu(nic(&d("BU", Bhvadi)))
        .krt(Krt::anIyar)
        .build()
        .unwrap();
    assert_has_sup_1p(&krt, Pum, &["BAvanIyAH"]);

    assert_has_sup_1d(&krt, Stri, &["BAvanIye"]);
    assert_has_sup_2d(&krt, Stri, &["BAvanIye"]);
    assert_has_sup_3p(&krt, Stri, &["BAvanIyABiH"]);
    assert_has_sup_4p(&krt, Stri, &["BAvanIyAByaH"]);
    assert_has_sup_5s(&krt, Stri, &["BAvanIyAyAH"]);
    assert_has_sup_6s(&krt, Stri, &["BAvanIyAyAH"]);
    assert_has_sup_6d(&krt, Stri, &["BAvanIyayoH"]);
    assert_has_sup_7p(&krt, Stri, &["BAvanIyAsu"]);
}

#[test]
fn ajighasam() {
    assert_has_mip(&[], &nic(&d("Gasx~", Bhvadi)), Lun, &["ajIGasam"]);
}

// Breaks if we process the wrong terms when doing ac sandhi for 6.4.51.
#[test]
fn adhyapidhvam() {
    assert_has_dhvam_k(
        &["aDi"],
        &nic(&d("i\\N", Adadi)),
        Lun,
        &[
            "aDyApayiDvam",
            "aDyApayiQvam",
            "aDyApiQvam",
            "aDyagApayiDvam",
            "aDyagApayiQvam",
            "aDyagApiDvam",
        ],
    );
}

#[test]
fn bhavakanam() {
    let krt = Krdanta::builder()
        .dhatu(nic(&d("BU", Bhvadi)))
        .krt(Krt::Rvul)
        .build()
        .unwrap();
    assert_has_sup_6p(&krt, Pum, &["BAvakAnAm"]);
}

// Cache-dependent test.
#[test]
fn pampanyamanan() {
    let krt = Krdanta::builder()
        .dhatu(yan(&d("paRa~\\", Bhvadi)))
        .krt(Krt::Satf)
        .build()
        .unwrap();
    assert_has_sup_1s(&krt, Pum, &[]);

    let krt = Krdanta::builder()
        .dhatu(yan(&d("paRa~\\", Bhvadi)))
        .krt(Krt::SAnac)
        .build()
        .unwrap();
    assert_has_sup_1s(&krt, Pum, &["pampaRyamAnaH"]);
    assert_has_sup_1d(&krt, Pum, &["pampaRyamAnO"]);
}

#[test]
fn kr_sat() {
    use Prayoga::*;

    fn assert_has_sat(
        dhatu: &Dhatu,
        krt: Krt,
        lakara: Lakara,
        prayoga: Prayoga,
        expected: &[&str],
    ) {
        let v = Vyakarana::new();
        let args = Krdanta::builder()
            .lakara(Lrt)
            .dhatu(dhatu.clone())
            .lakara(lakara)
            .prayoga(prayoga)
            .krt(krt)
            .build()
            .unwrap();
        let prakriyas = v.derive_krdantas(&args);
        assert_has_results(prakriyas, expected)
    }

    let kr = &d("qukf\\Y", Tanadi);

    // Check the subanta form since 6.4.100 was previously buggy.
    let kurvat = krdanta(&[], &kr, Krt::Satf);
    assert_has_sup_1s(kurvat, Pum, &["kurvan"]);

    assert_has_sat(&kr, Krt::Satf, Lat, Kartari, &["kurvat"]);
    assert_has_sat(&kr, Krt::SAnac, Lat, Kartari, &["kurvARa"]);
    assert_has_sat(&kr, Krt::Satf, Lat, Karmani, &[]);
    assert_has_sat(&kr, Krt::SAnac, Lat, Karmani, &["kriyamARa"]);
    assert_has_sat(&kr, Krt::Satf, Lrt, Kartari, &["karizyat"]);
    assert_has_sat(&kr, Krt::SAnac, Lrt, Kartari, &["karizyamARa"]);
    assert_has_sat(&kr, Krt::Satf, Lrt, Karmani, &[]);
    assert_has_sat(
        &kr,
        Krt::SAnac,
        Lrt,
        Karmani,
        &["karizyamARa", "kArizyamARa"],
    );
}

#[test]
fn ay_kvip() {
    let u = krdanta(&[], &d("aya~", Bhvadi), Krt::kvip);
    assert_has_sup_1s(&u, Pum, &[]);
}

// Tests that nipatana of izu + Sa is not overgenerating.
#[test]
fn iccha_nipatana() {
    assert_has_krdanta(&[], &d("izu~", Tudadi), Krt::Sa, &["icCA"]);
    assert_has_krdanta(&[], &d("izu~", Tudadi), Krt::vuY, &[]);
}

// Tests (Bas -> Bs) + (hi -> Di) for asiddhavat rules.
#[test]
fn babdhi() {
    assert_has_sip(&[], &d("Basa~", Juhotyadi), Lot, &["babDi", "babDAt"]);
}

// Tests that sup-luk occurs after avyayas, regardless of which sup we try adding.
#[test]
fn gantum_sup() {
    let gantum = krdanta(&[], &d("ga\\mx~", Bhvadi), Krt::tumun);
    assert_has_sup_1s(&gantum, Pum, &["gantum"]);
    assert_has_sup_1d(&gantum, Pum, &["gantum"]);
    assert_has_sup_1p(&gantum, Pum, &["gantum"]);
    assert_has_sup_1s(&gantum, Stri, &["gantum"]);

    let gatva = krdanta(&[], &d("ga\\mx~", Bhvadi), Krt::ktvA);
    assert_has_sup_1s(&gatva, Pum, &["gatvA"]);
    assert_has_sup_4s(&gatva, Napumsaka, &["gatvA"]);
}

// Simultaneous derivation of taddhita + sup
#[test]
fn hanumataa() {
    let hanumat = taddhitanta("hanu", Taddhita::matup);
    assert_has_sup_3s(&hanumat, Pum, &["hanumatA"]);
}

// Match againt vAh + Rvi, not general vAh.
#[test]
fn vahakena() {
    let vahaka = krdanta(&[], &d("va\\ha~^", Bhvadi), Krt::Rvul);
    assert_has_sup_3s(&vahaka, Pum, &["vAhakena"]);
}

// Fixes bug around nuw-Agama for anc.
#[test]
fn anancanas() {
    // TODO: is AYcAnaH allowed too?
    let anancana = krdanta(&[], &d("ancu~", Bhvadi), Krt::kAnac);
    assert_has_sup_1s(&anancana, Pum, &["AnaYcAnaH", "AYcAnaH"]);
}

// Fixes bug around n --> R for basic pratipadikas.
#[test]
fn prabhinnavish() {
    assert_has_sup_1p("praBinnaviz", Pum, &["praBinnavizaH"]);
}
