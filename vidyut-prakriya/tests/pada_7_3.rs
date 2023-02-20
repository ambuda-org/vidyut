extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

/// Checks parasmaipada + the given lakara/purusha/vacana
pub fn assert_has_p_lpv(
    prefixes: &[&str],
    dhatu: &Dhatu,
    lakara: Lakara,
    purusha: Purusha,
    vacana: Vacana,
    expected: &[&str],
) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(purusha)
        .vacana(vacana)
        .lakara(lakara)
        .pada(Pada::Parasmai)
        .build()
        .unwrap();
    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

pub fn assert_has_lat_p_3d(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_p_lpv(
        prefixes,
        dhatu,
        Lakara::Lat,
        Purusha::Prathama,
        Vacana::Dvi,
        expected,
    )
}

pub fn assert_has_lan_p_1s(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_p_lpv(
        prefixes,
        dhatu,
        Lakara::Lan,
        Purusha::Uttama,
        Vacana::Eka,
        expected,
    )
}

pub fn assert_has_lot_p_1s(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_p_lpv(
        prefixes,
        dhatu,
        Lakara::Lot,
        Purusha::Uttama,
        Vacana::Eka,
        expected,
    )
}

pub fn assert_has_lan_p_3p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_p_lpv(
        prefixes,
        dhatu,
        Lakara::Lan,
        Purusha::Prathama,
        Vacana::Bahu,
        expected,
    )
}

pub fn assert_has_vidhilin_p_3p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_p_lpv(
        prefixes,
        dhatu,
        Lakara::VidhiLin,
        Purusha::Prathama,
        Vacana::Bahu,
        expected,
    )
}

#[test]
fn sutra_7_3_52() {
    let d = Dhatu::new;

    let pac = d("qupa\\ca~^z", Gana::Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::GaY, &["pAka"]);
    assert_has_krdanta(&[], &d("tya\\ja~", Gana::Bhvadi), Krt::GaY, &["tyAga"]);
    assert_has_krdanta(
        &[],
        &d("ra\\nja~^", Gana::Bhvadi),
        Krt::GaY,
        &["rAga", "raNga"],
    );

    assert_has_krdanta(&[], &pac, Krt::Ryat, &["pAkya"]);
    assert_has_krdanta(&[], &d("va\\ca~", Gana::Bhvadi), Krt::Ryat, &["vAkya"]);
    assert_has_krdanta(&[], &d("ri\\ci~^r", Gana::Bhvadi), Krt::Ryat, &["rekya"]);
}

#[test]
fn sutra_7_3_82() {
    let mid = Dhatu::new("YimidA~", Gana::Divadi);
    assert_has_lat(&[], &mid, &["medyati"]);
    assert_has_lat_karmani(&[], &mid, &["midyate"]);
}

#[test]
fn sutra_7_3_83() {
    let d = Dhatu::new;
    assert_has_lan_p_3p(&[], &d("hu\\", Gana::Juhotyadi), &["ajuhavuH"]);
    assert_has_lan_p_3p(&[], &d("YiBI\\", Gana::Juhotyadi), &["abiBayuH"]);
    assert_has_lan_p_3p(&[], &d("quBf\\Y", Gana::Juhotyadi), &["abiBaruH"]);

    assert_has_vidhilin_p_3p(&[], &d("ci\\Y", Gana::Svadi), &["cinuyuH"]);
    assert_has_vidhilin_p_3p(&[], &d("zu\\Y", Gana::Svadi), &["sunuyuH"]);
}

#[test]
fn sutra_7_3_84() {
    let d = Dhatu::new;
    assert_has_lat_p(&[], &d("tF", Gana::Bhvadi), &["tarati"]);
    assert_has_lat_p(&[], &d("RI\\Y", Gana::Bhvadi), &["nayati"]);
    assert_has_lat_p(&[], &d("BU", Gana::Bhvadi), &["Bavati"]);

    assert_has_krdanta(&[], &d("qukf\\Y", Gana::Tanadi), Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &d("ci\\Y", Gana::Svadi), Krt::tfc, &["cetf"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Gana::Adadi), Krt::tfc, &["stotf"]);

    // TODO: agnitva, agnikAmyati
}

#[test]
fn sutra_7_3_85() {
    let jagf = Dhatu::new("jAgf", Gana::Adadi);

    assert_has_lat_p(
        &[],
        &jagf.clone().with_sanadi(&[Sanadi::Nic]),
        &["jAgarayati"],
    );
    assert_has_krdanta(&[], &jagf, Krt::Rvul, &["jAgaraka"]);
    assert_has_krdanta(&[], &jagf, Krt::GaY, &["jAgara"]);
    assert_has_krdanta(&[], &jagf, Krt::kta, &["jAgarita"]);
    assert_has_krdanta(&[], &jagf, Krt::ktavatu, &["jAgaritavat"]);

    // TODO: add support for unadi sutras
    // assert_has_krdanta(&[], &jagf, Krt::Unadi::vin, &["jAgfvi"]);
    assert_has_lun_karmani(&[], &jagf, &["ajAgAri"]);
    assert_has_lit(
        &[],
        &jagf,
        &["jajAgAra", "jAgarAYcakAra", "jAgarAmAsa", "jAgarAmbaBUva"],
    );
    assert_has_lat_p_3d(&[], &jagf, &["jAgftaH"]);
}

#[test]
fn sutra_7_3_86() {
    let d = |u, g| Dhatu::new(u, g).with_sanadi(&[Sanadi::Nic]);

    assert_has_lat_p(&[], &d("vlI\\", Gana::Kryadi), &["vlepayati"]);
    assert_has_lat_p(&[], &d("hrI\\", Gana::Juhotyadi), &["hrepayati"]);
    assert_has_lat_p(&[], &d("knUyI~\\", Gana::Bhvadi), &["knopayati"]);

    let bhid = Dhatu::new("Bi\\di~^r", Gana::Rudhadi);
    let chid = Dhatu::new("Ci\\di~^r", Gana::Rudhadi);
    assert_has_krdanta(&[], &bhid, Krt::lyuw, &["Bedana"]);
    assert_has_krdanta(&[], &chid, Krt::lyuw, &["Cedana"]);
    assert_has_krdanta(&[], &bhid, Krt::tfc, &["Bettf"]);
    assert_has_krdanta(&[], &chid, Krt::tfc, &["Cettf"]);
}

#[test]
fn sutra_7_3_87() {
    let d = Dhatu::new;

    let nij = d("Ri\\ji~^r", Gana::Juhotyadi);
    let vij = d("vi\\ji~^r", Gana::Juhotyadi);
    let viz = d("vi\\zx~^", Gana::Juhotyadi);

    assert_has_lot_p_1s(&[], &nij, &["nenijAni"]);
    assert_has_lot_p_1s(&[], &vij, &["vevijAni"]);
    assert_has_lot_p_1s(&["pari"], &viz, &["parivevizARi"]);

    assert_has_lan_p_1s(&[], &nij, &["anenijam"]);
    assert_has_lan_p_1s(&[], &vij, &["avevijam"]);
    assert_has_lan_p_1s(&["pari"], &viz, &["paryavevizam"]);

    assert_has_lot_p_1s(&[], &d("vida~", Gana::Adadi), &["vedAni", "vidANkaravARi"]);
    assert_has_lat_p(&[], &nij, &["nenekti"]);
    assert_has_lit_p(&[], &nij, &["nineja"]);

    let hu = d("hu\\", Gana::Juhotyadi);
    assert_has_lot_p_1s(&[], &hu, &["juhavAni"]);
    assert_has_lan_p_1s(&[], &hu, &["ajuhavam"]);

    // TODO: other examples
}
