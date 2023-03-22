extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San])
}

#[test]
fn sutra_2_4_36() {
    let ad = d("a\\da~", Adadi);
    assert_has_krdanta(&["pra"], &ad, Krt::ktvA, &["prajagDya"]);
    assert_has_krdanta(&["vi"], &ad, Krt::ktvA, &["vijagDya"]);
    assert_has_krdanta(&[], &ad, Krt::kta, &["jagDa", "jagdDa"]);
    assert_has_krdanta(&[], &ad, Krt::ktavatu, &["jagDavat", "jagdDavat"]);

    assert_has_lat_karmani(&[], &ad, &["adyate"]);
    assert_has_krdanta(&[], &ad, Krt::tavya, &["attavya"]);

    // TODO: annam
}

#[ignore]
#[test]
fn sutra_2_4_37() {
    let ad = d("a\\da~", Adadi);
    assert_has_parasmai_tinanta(&[], &ad, Lun, Prathama, Eka, &["aGasat"]);
    assert_has_parasmai_tinanta(&[], &ad, Lun, Prathama, Dvi, &["aGasatAm"]);
    assert_has_parasmai_tinanta(&[], &ad, Lun, Prathama, Bahu, &["aGasan"]);
    assert_has_parasmai_tinanta(&[], &san(&ad), Lat, Prathama, Eka, &["jiGatsati"]);
    assert_has_parasmai_tinanta(&[], &san(&ad), Lat, Prathama, Dvi, &["jiGatsataH"]);
    assert_has_parasmai_tinanta(&[], &san(&ad), Lat, Prathama, Bahu, &["jiGatsanti"]);
}

#[ignore]
#[test]
fn sutra_2_4_37_v1() {
    let ad = d("a\\da~", Adadi);
    assert_has_krdanta(&["pra"], &ad, Krt::ac, &["praGasa"]);
}

#[ignore]
#[test]
fn sutra_2_4_38() {
    let ad = d("a\\da~", Adadi);
    assert_has_krdanta(&[], &ad, Krt::GaY, &["GAsa"]);
    assert_has_krdanta(&["pra"], &ad, Krt::ap, &["praGasa"]);
}

#[test]
fn sutra_2_4_40() {
    let ad = d("a\\da~", Adadi);
    assert_has_parasmai_tinanta(&[], &ad, Lit, Prathama, Eka, &["jaGAsa", "Ada"]);
    assert_has_parasmai_tinanta(&[], &ad, Lit, Prathama, Dvi, &["jakzatuH", "AdatuH"]);
    assert_has_parasmai_tinanta(&[], &ad, Lit, Prathama, Bahu, &["jakzuH", "AduH"]);
}

#[test]
fn sutra_2_4_42() {
    let han = d("ha\\na~", Adadi);
    assert_has_parasmai_tinanta(&[], &han, AshirLin, Prathama, Eka, &["vaDyAt"]);
    assert_has_parasmai_tinanta(&[], &han, AshirLin, Prathama, Dvi, &["vaDyAstAm"]);
    assert_has_parasmai_tinanta(&[], &han, AshirLin, Prathama, Bahu, &["vaDyAsuH"]);
}

#[test]
fn sutra_2_4_43() {
    let han = d("ha\\na~", Adadi);
    assert_has_parasmai_tinanta(&[], &han, Lun, Prathama, Eka, &["avaDIt"]);
    assert_has_parasmai_tinanta(&[], &han, Lun, Prathama, Dvi, &["avaDizwAm"]);
    assert_has_parasmai_tinanta(&[], &han, Lun, Prathama, Bahu, &["avaDizuH"]);
}

#[test]
fn sutra_2_4_44() {
    let han = d("ha\\na~", Adadi);
    assert_has_atmane_tinanta(&["AN"], &han, Lun, Prathama, Eka, &["AvaDizwa", "Ahata"]);
    assert_has_atmane_tinanta(
        &["AN"],
        &han,
        Lun,
        Prathama,
        Dvi,
        &["AvaDizAtAm", "AhasAtAm"],
    );
    assert_has_atmane_tinanta(
        &["AN"],
        &han,
        Lun,
        Prathama,
        Bahu,
        &["AvaDizata", "Ahasata"],
    );
}

#[test]
fn sutra_2_4_45() {
    let i = d("i\\R", Adadi);
    assert_has_parasmai_tinanta(&[], &i, Lun, Prathama, Eka, &["agAt"]);
    assert_has_parasmai_tinanta(&[], &i, Lun, Prathama, Dvi, &["agAtAm"]);
    assert_has_parasmai_tinanta(&[], &i, Lun, Prathama, Bahu, &["aguH"]);
    // karmani
    assert_has_lun_karmani(&[], &i, &["agAyi"]);
}

#[test]
fn sutra_2_4_45_v1() {
    let ik = d("i\\k", Adadi);
    assert_has_parasmai_tinanta(&["aDi"], &ik, Lun, Prathama, Eka, &["aDyagAt"]);
    assert_has_parasmai_tinanta(&["aDi"], &ik, Lun, Prathama, Dvi, &["aDyagAtAm"]);
    assert_has_parasmai_tinanta(&["aDi"], &ik, Lun, Prathama, Bahu, &["aDyaguH"]);
}
