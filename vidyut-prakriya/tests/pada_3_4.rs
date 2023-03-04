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

#[test]
fn sutra_3_4_78() {
    use Pada::*;
    use Purusha::*;
    use Vacana::*;

    let pac = d("qupa\\ca~^z", Bhvadi);
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
            .lakara(Lat)
            .pada(*pada)
            .build()
            .unwrap();
        let actual = derive_tinantas(&pac, &args);
        assert_padas(actual, &[expected])
    }
}

#[test]
fn sutra_3_4_79() {
    // Counterexamples
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::SAnac, &["pacamAna"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::SAnac, &["yajamAna"]);
}

#[test]
fn sutra_3_4_80() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_atmane_tinanta(&[], &pac, Lat, Madhyama, Eka, &["pacase"]);
    assert_has_atmane_tinanta(&[], &pac, Lit, Madhyama, Eka, &["pecize"]);
    assert_has_atmane_tinanta(&[], &pac, Lut, Madhyama, Eka, &["paktAse"]);
    assert_has_atmane_tinanta(&[], &pac, Lrt, Madhyama, Eka, &["pakzyase"]);
}

#[test]
fn sutra_3_4_81() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_atmane_tinanta(&[], &pac, Lit, Prathama, Eka, &["pece"]);
    assert_has_atmane_tinanta(&[], &pac, Lit, Prathama, Dvi, &["pecAte"]);
    assert_has_atmane_tinanta(&[], &pac, Lit, Prathama, Bahu, &["pecire"]);
    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_atmane_tinanta(&[], &labh, Lit, Prathama, Eka, &["leBe"]);
    assert_has_atmane_tinanta(&[], &labh, Lit, Prathama, Dvi, &["leBAte"]);
    assert_has_atmane_tinanta(&[], &labh, Lit, Prathama, Bahu, &["leBire"]);
}

#[test]
fn sutra_3_4_82() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_parasmai_tinanta(&[], &pac, Lit, Prathama, Eka, &["papAca"]);
    assert_has_parasmai_tinanta(&[], &pac, Lit, Prathama, Dvi, &["pecatuH"]);
    assert_has_parasmai_tinanta(&[], &pac, Lit, Prathama, Bahu, &["pecuH"]);
    assert_has_parasmai_tinanta(&[], &pac, Lit, Madhyama, Eka, &["peciTa", "papakTa"]);
    assert_has_parasmai_tinanta(&[], &pac, Lit, Madhyama, Dvi, &["pecaTuH"]);
    assert_has_parasmai_tinanta(&[], &pac, Lit, Madhyama, Bahu, &["peca"]);
    assert_has_parasmai_tinanta(&[], &pac, Lit, Uttama, Eka, &["papAca", "papaca"]);
    assert_has_parasmai_tinanta(&[], &pac, Lit, Uttama, Dvi, &["peciva"]);
    assert_has_parasmai_tinanta(&[], &pac, Lit, Uttama, Bahu, &["pecima"]);
}

#[test]
fn sutra_3_4_85() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_parasmai_tinanta(&[], &pac, Lot, Prathama, Dvi, &["pacatAm"]);
    assert_has_parasmai_tinanta(&[], &pac, Lot, Madhyama, Dvi, &["pacatam"]);
    assert_has_parasmai_tinanta(&[], &pac, Lot, Madhyama, Bahu, &["pacata"]);
    assert_has_parasmai_tinanta(&[], &pac, Lot, Uttama, Dvi, &["pacAva"]);
    assert_has_parasmai_tinanta(&[], &pac, Lot, Uttama, Bahu, &["pacAma"]);
    //
    assert_has_parasmai_tinanta(&[], &d("yA\\", Adadi), Lot, Prathama, Bahu, &["yAntu"]);
    assert_has_parasmai_tinanta(&[], &d("vA\\", Adadi), Lot, Prathama, Bahu, &["vAntu"]);
}

#[test]
fn sutra_3_4_86() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_parasmai_tinanta(&[], &pac, Lot, Prathama, Eka, &["pacatu", "pacatAt"]);
    assert_has_parasmai_tinanta(&[], &pac, Lot, Prathama, Bahu, &["pacantu"]);
}

#[test]
fn sutra_3_4_87() {
    let assert_has_lot_madhyama_eka = |dhatu, exp| {
        assert_has_parasmai_tinanta(&[], &dhatu, Lot, Madhyama, Eka, exp);
    };
    assert_has_lot_madhyama_eka(d("lUY", Kryadi), &["lunIhi", "lunItAt"]);
    assert_has_lot_madhyama_eka(d("pUY", Kryadi), &["punIhi", "punItAt"]);
    assert_has_lot_madhyama_eka(d("rA\\Da~", Svadi), &["rADnuhi", "rADnutAt"]);
    assert_has_lot_madhyama_eka(
        d("takzU~", Bhvadi),
        &["takzRuhi", "takzRutAt", "takza", "takzatAt"],
    );
}

#[test]
fn sutra_3_4_89() {
    assert_has_parasmai_tinanta(
        &[],
        &d("qupa\\ca~^z", Bhvadi),
        Lot,
        Uttama,
        Eka,
        &["pacAni"],
    );
    assert_has_parasmai_tinanta(&[], &d("paWa~", Bhvadi), Lot, Uttama, Eka, &["paWAni"]);
}

#[test]
fn sutra_3_4_90() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_atmane_tinanta(&[], &pac, Lot, Prathama, Eka, &["pacatAm"]);
    assert_has_atmane_tinanta(&[], &pac, Lot, Prathama, Dvi, &["pacetAm"]);
    assert_has_atmane_tinanta(&[], &pac, Lot, Prathama, Bahu, &["pacantAm"]);
}

#[test]
fn sutra_3_4_91() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_atmane_tinanta(&[], &pac, Lot, Madhyama, Eka, &["pacasva"]);
    assert_has_atmane_tinanta(&[], &pac, Lot, Madhyama, Bahu, &["pacaDvam"]);
}

#[test]
fn sutra_3_4_92_and_sutra_3_4_93() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_parasmai_tinanta(&[], &kf, Lot, Uttama, Eka, &["karavARi"]);
    assert_has_parasmai_tinanta(&[], &kf, Lot, Uttama, Dvi, &["karavAva"]);
    assert_has_parasmai_tinanta(&[], &kf, Lot, Uttama, Bahu, &["karavAma"]);
    assert_has_atmane_tinanta(&[], &kf, Lot, Uttama, Eka, &["karavE"]);
    assert_has_atmane_tinanta(&[], &kf, Lot, Uttama, Dvi, &["karavAvahE"]);
    assert_has_atmane_tinanta(&[], &kf, Lot, Uttama, Bahu, &["karavAmahE"]);
}

#[test]
fn sutra_3_4_99() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_parasmai_tinanta(&[], &pac, Lan, Uttama, Dvi, &["apacAva"]);
    assert_has_parasmai_tinanta(&[], &pac, Lan, Uttama, Bahu, &["apacAma"]);
}

#[test]
fn sutra_3_4_100() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_parasmai_tinanta(&[], &pac, Lan, Prathama, Eka, &["apacat"]);
    assert_has_parasmai_tinanta(&[], &pac, Lun, Prathama, Eka, &["apAkzIt"]);
    assert_has_atmane_tinanta(&[], &pac, Lan, Uttama, Dvi, &["apacAvahi"]);
    assert_has_atmane_tinanta(&[], &pac, Lan, Uttama, Bahu, &["apacAmahi"]);
}

#[test]
fn sutra_3_4_101() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_parasmai_tinanta(&[], &pac, Lan, Prathama, Dvi, &["apacatAm"]);
    assert_has_parasmai_tinanta(&[], &pac, Lan, Madhyama, Dvi, &["apacatam"]);
    assert_has_parasmai_tinanta(&[], &pac, Lan, Madhyama, Bahu, &["apacata"]);
    assert_has_parasmai_tinanta(&[], &pac, Lan, Uttama, Eka, &["apacam"]);
    assert_has_parasmai_tinanta(&[], &pac, Lun, Prathama, Dvi, &["apAktAm"]);
    assert_has_parasmai_tinanta(&[], &pac, Lun, Madhyama, Dvi, &["apAktam"]);
    assert_has_parasmai_tinanta(&[], &pac, Lun, Madhyama, Bahu, &["apAkta"]);
    assert_has_parasmai_tinanta(&[], &pac, Lun, Uttama, Eka, &["apAkzam"]);
}

#[test]
fn sutra_3_4_102() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_atmane_tinanta(&[], &pac, VidhiLin, Prathama, Eka, &["paceta"]);
    assert_has_atmane_tinanta(&[], &pac, VidhiLin, Prathama, Dvi, &["paceyAtAm"]);
    assert_has_atmane_tinanta(&[], &pac, VidhiLin, Prathama, Bahu, &["paceran"]);
    assert_has_atmane_tinanta(&[], &pac, AshirLin, Prathama, Eka, &["pakzIzwa"]);
    assert_has_atmane_tinanta(&[], &pac, AshirLin, Prathama, Dvi, &["pakzIyAstAm"]);
    assert_has_atmane_tinanta(&[], &pac, AshirLin, Prathama, Bahu, &["pakzIran"]);
}

#[test]
fn sutra_3_4_103() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_parasmai_tinanta(&[], &kf, VidhiLin, Prathama, Eka, &["kuryAt"]);
    assert_has_parasmai_tinanta(&[], &kf, VidhiLin, Prathama, Dvi, &["kuryAtAm"]);
    assert_has_parasmai_tinanta(&[], &kf, VidhiLin, Prathama, Bahu, &["kuryuH"]);
    // Counterexamples
    assert_has_parasmai_tinanta(&[], &d("ci\\Y", Svadi), Lan, Uttama, Eka, &["acinavam"]);
    assert_has_parasmai_tinanta(&[], &kf, Lan, Uttama, Eka, &["akaravam"]);
}

#[test]
fn sutra_3_4_104() {
    let iz = d("izu~", Tudadi);
    assert_has_parasmai_tinanta(&[], &iz, AshirLin, Prathama, Eka, &["izyAt"]);
    assert_has_parasmai_tinanta(&[], &iz, AshirLin, Prathama, Dvi, &["izyAstAm"]);
    assert_has_parasmai_tinanta(&[], &iz, AshirLin, Prathama, Bahu, &["izyAsuH"]);
    let jagf = d("jAgf", Adadi);
    assert_has_parasmai_tinanta(&[], &jagf, AshirLin, Prathama, Eka, &["jAgaryAt"]);
    assert_has_parasmai_tinanta(&[], &jagf, AshirLin, Prathama, Dvi, &["jAgaryAstAm"]);
    assert_has_parasmai_tinanta(&[], &jagf, AshirLin, Prathama, Bahu, &["jAgaryAsuH"]);
    // Counterexamples
    assert_has_vidhilin_p(&[], &d("va\\ca~", Adadi), &["vacyAt"]);
    assert_has_vidhilin_p(&[], &jagf, &["jAgfyAt"]);
}

#[test]
fn sutra_3_4_105() {
    let assert_has_jha = |dhatu, lakara, exp| {
        assert_has_atmane_tinanta(&[], &dhatu, lakara, Prathama, Bahu, exp);
    };
    assert_has_jha(d("qupa\\ca~^z", Bhvadi), VidhiLin, &["paceran"]);
    assert_has_jha(d("ya\\ja~^", Bhvadi), VidhiLin, &["yajeran"]);
    assert_has_jha(d("qukf\\Y", Tanadi), AshirLin, &["kfzIran"]);
}

#[test]
fn sutra_3_4_106() {
    let assert_has_iw = |dhatu, lakara, exp| {
        assert_has_atmane_tinanta(&[], &dhatu, lakara, Uttama, Eka, exp);
    };
    assert_has_iw(d("qupa\\ca~^z", Bhvadi), VidhiLin, &["paceya"]);
    assert_has_iw(d("ya\\ja~^", Bhvadi), VidhiLin, &["yajeya"]);
    assert_has_iw(d("qukf\\Y", Tanadi), AshirLin, &["kfzIya"]);
    assert_has_iw(d("hf\\Y", Bhvadi), AshirLin, &["hfzIya"]);
}

#[test]
fn sutra_3_4_107() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_atmane_tinanta(&[], &kf, AshirLin, Prathama, Eka, &["kfzIzwa"]);
    assert_has_atmane_tinanta(&[], &kf, AshirLin, Prathama, Dvi, &["kfzIyAstAm"]);
    assert_has_atmane_tinanta(&[], &kf, AshirLin, Madhyama, Eka, &["kfzIzWAH"]);
    assert_has_atmane_tinanta(&[], &kf, AshirLin, Madhyama, Dvi, &["kfzIyAsTAm"]);
}

#[test]
fn sutra_3_4_108() {
    let assert_has_jhi = |dhatu, lakara, exp| {
        assert_has_parasmai_tinanta(&[], &dhatu, lakara, Prathama, Bahu, exp);
    };
    assert_has_jhi(d("qupa\\ca~^z", Bhvadi), VidhiLin, &["paceyuH"]);
    assert_has_jhi(d("ya\\ja~^", Bhvadi), VidhiLin, &["yajeyuH"]);
}

#[test]
fn sutra_3_4_109() {
    let assert_has_jhi = |dhatu, lakara, exp| {
        assert_has_parasmai_tinanta(&[], &dhatu, lakara, Prathama, Bahu, exp);
    };
    assert_has_jhi(d("qukf\\Y", Tanadi), Lun, &["akArzuH"]);
    assert_has_jhi(d("hf\\Y", Bhvadi), Lun, &["ahArzuH"]);
    assert_has_jhi(d("YiBI\\", Juhotyadi), Lan, &["abiBayuH"]);
    assert_has_jhi(d("hrI\\", Juhotyadi), Lan, &["ajihrayuH"]);
    assert_has_jhi(d("jAgf", Adadi), Lan, &["ajAgaruH"]);
    assert_has_jhi(d("vida~", Adadi), Lan, &["aviduH"]);
}

#[test]
fn sutra_3_4_113() {
    assert_has_lat_p(&[], &d("BU", Bhvadi), &["Bavati"]);
    assert_has_lat_p(&[], &d("RI\\Y", Bhvadi), &["nayati"]);
    assert_has_lat_p(&[], &d("Yizva\\pa~", Adadi), &["svapiti"]);
    assert_has_lat_p(&[], &d("rudi~r", Adadi), &["roditi"]);
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), Krt::SAnac, &["pacamAna"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::SAnac, &["yajamAna"]);
}
