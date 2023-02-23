extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_6_4_23() {
    let d = Dhatu::new;
    assert_has_lat(&[], &d("anjU~", Gana::Rudhadi), &["anakti"]);
    assert_has_lat(&[], &d("Ba\\njo~", Gana::Rudhadi), &["Banakti"]);
    assert_has_lat(&[], &d("hisi~", Gana::Rudhadi), &["hinasti"]);

    // TODO: make these proper tests and model the dhatu.
    assert_has_subantas("yajYa", Pum, Sasthi, Bahu, &["yajYAnAm"]);
    assert_has_subantas("yatna", Pum, Sasthi, Bahu, &["yatnAnAm"]);
    assert_has_subantas("viSna", Pum, Sasthi, Bahu, &["viSnAnAm"]);
    assert_has_subantas("praSna", Pum, Sasthi, Bahu, &["praSnAnAm"]);
}

#[test]
fn sutra_6_4_24() {
    let yan = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::Yan]);
    let sras = Dhatu::new("sransu~\\", Gana::Bhvadi);
    let dhvas = Dhatu::new("Dvansu~", Gana::Bhvadi);

    assert_has_krdanta(&[], &sras, Krt::kta, &["srasta"]);
    assert_has_krdanta(&[], &dhvas, Krt::kta, &["Dvasta"]);
    assert_has_lat_karmani(&[], &sras, &["srasyate"]);
    assert_has_lat_karmani(&[], &dhvas, &["Dvasyate"]);
    assert_has_lat(&[], &yan(&sras), &["sanIsrasyate"]);
    assert_has_lat(&[], &yan(&dhvas), &["danIDvasyate"]);

    let nand = Dhatu::new("wunadi~", Gana::Bhvadi);
    assert_has_lat_karmani(&[], &nand, &["nandyate"]);
    assert_has_lat(&[], &yan(&nand), &["nAnandyate"]);

    let ni = Dhatu::new("RI\\Y", Gana::Bhvadi);
    assert_has_lat_karmani(&[], &ni, &["nIyate"]);
    assert_has_lat(&[], &yan(&ni), &["nenIyate"]);

    let nah = Dhatu::new("Ra\\ha~^", Gana::Divadi);
    assert_has_lat_karmani(&[], &nah, &["nahyate"]);
    assert_has_lat(&[], &yan(&nah), &["nAnahyate"]);

    assert_has_krdanta(&[], &sras, Krt::tfc, &["sraMsitf"]);
    assert_has_krdanta(&[], &dhvas, Krt::tfc, &["DvaMsitf"]);

    // TODO: varttikas
}

#[test]
fn sutra_6_4_25() {
    let d = Dhatu::new;
    assert_has_lat(&[], &d("da\\nSa~", Gana::Bhvadi), &["daSati"]);
    assert_has_lat(&[], &d("za\\nja~", Gana::Bhvadi), &["sajati"]);
    assert_has_lat(&["pari"], &d("zva\\nja~\\", Gana::Bhvadi), &["parizvajate"]);
}

#[test]
fn sutra_6_4_26() {
    let d = Dhatu::new;
    assert_has_lat_p(&[], &d("ra\\nja~^", Gana::Bhvadi), &["rajati"]);
}

#[test]
fn sutra_6_4_27() {
    let ranj = Dhatu::new("ra\\nja~^", Gana::Bhvadi);
    assert_has_krdanta(&[], &ranj, Krt::GaY, &["rAga", "raNga"]);
}

#[ignore]
#[test]
fn sutra_6_4_28() {
    let syand = Dhatu::new("syandU~\\", Gana::Bhvadi);
    assert_has_krdanta(&[], &syand, Krt::GaY, &["syada", "syanda"]);
}

#[test]
fn sutra_6_4_31() {
    let d = Dhatu::new;
    assert_has_krdanta(
        &[],
        &d("ska\\ndi~r", Gana::Bhvadi),
        Krt::ktvA,
        &["skantvA", "skanttvA"],
    );
    assert_has_krdanta(
        &[],
        &d("syandU~\\", Gana::Bhvadi),
        Krt::ktvA,
        &["syantvA", "syanttvA", "syanditvA"],
    );
}

#[test]
fn sutra_6_4_32() {
    let d = Dhatu::new;
    assert_has_krdanta(
        &[],
        &d("ra\\nja~^", Gana::Bhvadi),
        Krt::ktvA,
        &["raNktvA", "raktvA"],
    );
    assert_has_krdanta(
        &[],
        &d("Ba\\njo~", Gana::Bhvadi),
        Krt::ktvA,
        &["BaNktvA", "BaktvA"],
    );
    assert_has_krdanta(
        &[],
        &d("Ra\\Sa~", Gana::Divadi),
        Krt::ktvA,
        &["naMzwvA", "nazwvA", "naSitvA"],
    );
}
