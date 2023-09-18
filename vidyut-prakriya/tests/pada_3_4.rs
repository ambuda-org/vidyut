extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti as V;
use vidyut_prakriya::args::*;

#[test]
fn sutra_3_4_12() {
    assert_has_krdanta(&["vi"], &d("Ba\\ja~^", Tudadi), Krt::Ramul, &["viBAjam"]);
    assert_has_krdanta(&["apa"], &d("lu\\px~^", Tudadi), Krt::kamul, &["apalupam"]);
}

#[test]
fn sutra_3_4_17() {
    assert_has_krdanta(&["vi"], &d("sf\\px~", Tudadi), Krt::kasun, &["visfpas"]);
    assert_has_krdanta(&["AN"], &d("u~tfdi~^r", Rudhadi), Krt::kasun, &["Atfdas"]);
}

#[test]
fn sutra_3_4_21() {
    assert_has_krdanta(&[], &d("Bu\\ja~", Rudhadi), Krt::ktvA, &["BuktvA"]);
    assert_has_krdanta(&[], &d("pA\\", Bhvadi), Krt::ktvA, &["pItvA"]);
    assert_has_krdanta(&[], &d("zRA\\", Adadi), Krt::ktvA, &["snAtvA"]);
}

#[test]
fn sutra_3_4_70() {
    let kf = &d("qukf\\Y", Tanadi);
    let bhuj = &d("Bu\\ja~", Rudhadi);
    let aas = &d("Asa~\\", Tanadi);
    let shi = &d("SIN", Adadi);
    assert_has_krdanta(&[], &kf, Krt::tavya, &["kartavya"]);
    assert_has_krdanta(&[], &bhuj, Krt::tavya, &["Boktavya"]);
    assert_has_krdanta(&[], &aas, Krt::tavya, &["Asitavya"]);
    assert_has_krdanta(&[], &shi, Krt::tavya, &["Sayitavya"]);
    // kta
    assert_has_krdanta(&[], &kf, Krt::kta, &["kfta"]);
    assert_has_krdanta(&[], &bhuj, Krt::kta, &["Bukta"]);
    assert_has_krdanta(&[], &aas, Krt::kta, &["Asita"]);
    assert_has_krdanta(&[], &shi, Krt::kta, &["Sayita"]);

    assert_has_krdanta(&[], &kf, Krt::Kal, &["kara"]);
    assert_has_krdanta(&[], &d("BU", Bhvadi), Krt::Kal, &["Bava"]);
}

#[test]
fn sutra_3_4_78() {
    use Pada::*;
    use Purusha::*;
    use Vacana::*;

    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tip(&[], &pac, Lat, &["pacati"]);
    assert_has_tas(&[], &pac, Lat, &["pacataH"]);
    assert_has_jhi(&[], &pac, Lat, &["pacanti"]);
    assert_has_sip(&[], &pac, Lat, &["pacasi"]);
    assert_has_thas(&[], &pac, Lat, &["pacaTaH"]);
    assert_has_tha(&[], &pac, Lat, &["pacaTa"]);
    assert_has_mip(&[], &pac, Lat, &["pacAmi"]);
    assert_has_vas(&[], &pac, Lat, &["pacAvaH"]);
    assert_has_mas(&[], &pac, Lat, &["pacAmaH"]);
    let items = &[
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
    assert_has_thaas(&[], &pac, Lat, &["pacase"]);
    assert_has_thaas(&[], &pac, Lit, &["pecize"]);
    assert_has_thaas(&[], &pac, Lut, &["paktAse"]);
    assert_has_thaas(&[], &pac, Lrt, &["pakzyase"]);
}

#[test]
fn sutra_3_4_81() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_ta(&[], &pac, Lit, &["pece"]);
    assert_has_aataam(&[], &pac, Lit, &["pecAte"]);
    assert_has_jha(&[], &pac, Lit, &["pecire"]);
    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_ta(&[], &labh, Lit, &["leBe"]);
    assert_has_aataam(&[], &labh, Lit, &["leBAte"]);
    assert_has_jha(&[], &labh, Lit, &["leBire"]);
}

#[test]
fn sutra_3_4_82() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tip(&[], &pac, Lit, &["papAca"]);
    assert_has_tas(&[], &pac, Lit, &["pecatuH"]);
    assert_has_jhi(&[], &pac, Lit, &["pecuH"]);
    assert_has_sip(&[], &pac, Lit, &["peciTa", "papakTa"]);
    assert_has_thas(&[], &pac, Lit, &["pecaTuH"]);
    assert_has_tha(&[], &pac, Lit, &["peca"]);
    assert_has_mip(&[], &pac, Lit, &["papAca", "papaca"]);
    assert_has_vas(&[], &pac, Lit, &["peciva"]);
    assert_has_mas(&[], &pac, Lit, &["pecima"]);
}

#[test]
fn sutra_3_4_83() {
    let vid = d("vida~", Adadi);
    assert_has_tip(&[], &vid, Lat, &["veda", "vetti"]);
    assert_has_tas(&[], &vid, Lat, &["vidatuH", "vittaH"]);
    assert_has_jhi(&[], &vid, Lat, &["viduH", "vidanti"]);
    assert_has_sip(&[], &vid, Lat, &["vetTa", "vetsi"]);
    assert_has_thas(&[], &vid, Lat, &["vidaTuH", "vitTaH"]);
    assert_has_tha(&[], &vid, Lat, &["vida", "vitTa"]);
    assert_has_mip(&[], &vid, Lat, &["veda", "vedmi"]);
    assert_has_vas(&[], &vid, Lat, &["vidva", "vidvaH"]);
    assert_has_mas(&[], &vid, Lat, &["vidma", "vidmaH"]);
}

#[test]
fn sutra_3_4_84() {
    let bru = d("brUY", Adadi);
    assert_has_tip(&[], &bru, Lat, &["Aha", "bravIti"]);
    assert_has_tas(&[], &bru, Lat, &["AhatuH", "brUtaH"]);
    assert_has_jhi(&[], &bru, Lat, &["AhuH", "bruvanti"]);
    assert_has_sip(&[], &bru, Lat, &["AtTa", "bravIzi"]);
    assert_has_thas(&[], &bru, Lat, &["AhaTuH", "brUTaH"]);
    assert_has_tha(&[], &bru, Lat, &["brUTa"]);
    assert_has_mip(&[], &bru, Lat, &["bravImi"]);
    assert_has_vas(&[], &bru, Lat, &["brUvaH"]);
    assert_has_mas(&[], &bru, Lat, &["brUmaH"]);
}

#[test]
fn sutra_3_4_85() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tas(&[], &pac, Lot, &["pacatAm"]);
    assert_has_thas(&[], &pac, Lot, &["pacatam"]);
    assert_has_tha(&[], &pac, Lot, &["pacata"]);
    assert_has_vas(&[], &pac, Lot, &["pacAva"]);
    assert_has_mas(&[], &pac, Lot, &["pacAma"]);
    //
    assert_has_jhi(&[], &d("yA\\", Adadi), Lot, &["yAntu"]);
    assert_has_jhi(&[], &d("vA\\", Adadi), Lot, &["vAntu"]);
}

#[test]
fn sutra_3_4_86() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tip(&[], &pac, Lot, &["pacatu", "pacatAt"]);
    assert_has_jhi(&[], &pac, Lot, &["pacantu"]);
}

#[test]
fn sutra_3_4_87() {
    assert_has_sip(&[], &d("lUY", Kryadi), Lot, &["lunIhi", "lunItAt"]);
    assert_has_sip(&[], &d("pUY", Kryadi), Lot, &["punIhi", "punItAt"]);
    assert_has_sip(&[], &d("rA\\Da~", Svadi), Lot, &["rADnuhi", "rADnutAt"]);
    assert_has_sip(
        &[],
        &d("takzU~", Bhvadi),
        Lot,
        &["takzRuhi", "takzRutAt", "takza", "takzatAt"],
    );
}

// 3.4.88 is chAndasa.

#[test]
fn sutra_3_4_89() {
    assert_has_mip(&[], &d("qupa\\ca~^z", Bhvadi), Lot, &["pacAni"]);
    assert_has_mip(&[], &d("paWa~", Bhvadi), Lot, &["paWAni"]);
}

#[test]
fn sutra_3_4_90() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_ta(&[], &pac, Lot, &["pacatAm"]);
    assert_has_aataam(&[], &pac, Lot, &["pacetAm"]);
    assert_has_jha(&[], &pac, Lot, &["pacantAm"]);
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
    assert_has_mip(&[], &kf, Lot, &["karavARi"]);
    assert_has_vas(&[], &kf, Lot, &["karavAva"]);
    assert_has_mas(&[], &kf, Lot, &["karavAma"]);
    assert_has_atmane_tinanta(&[], &kf, Lot, Uttama, Eka, &["karavE"]);
    assert_has_atmane_tinanta(&[], &kf, Lot, Uttama, Dvi, &["karavAvahE"]);
    assert_has_atmane_tinanta(&[], &kf, Lot, Uttama, Bahu, &["karavAmahE"]);
}

// 3.4.94 - 3.4.98 are for lew.

#[test]
fn sutra_3_4_99() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_vas(&[], &pac, Lan, &["apacAva"]);
    assert_has_mas(&[], &pac, Lan, &["apacAma"]);
}

#[test]
fn sutra_3_4_100() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tip(&[], &pac, Lan, &["apacat"]);
    assert_has_tip(&[], &pac, Lun, &["apAkzIt"]);
    assert_has_atmane_tinanta(&[], &pac, Lan, Uttama, Dvi, &["apacAvahi"]);
    assert_has_atmane_tinanta(&[], &pac, Lan, Uttama, Bahu, &["apacAmahi"]);
}

#[test]
fn sutra_3_4_101() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tas(&[], &pac, Lan, &["apacatAm"]);
    assert_has_thas(&[], &pac, Lan, &["apacatam"]);
    assert_has_tha(&[], &pac, Lan, &["apacata"]);
    assert_has_mip(&[], &pac, Lan, &["apacam"]);
    assert_has_tas(&[], &pac, Lun, &["apAktAm"]);
    assert_has_thas(&[], &pac, Lun, &["apAktam"]);
    assert_has_tha(&[], &pac, Lun, &["apAkta"]);
    assert_has_mip(&[], &pac, Lun, &["apAkzam"]);
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
    assert_has_tip(&[], &kf, VidhiLin, &["kuryAt"]);
    assert_has_tas(&[], &kf, VidhiLin, &["kuryAtAm"]);
    assert_has_jhi(&[], &kf, VidhiLin, &["kuryuH"]);
    // Counterexamples
    assert_has_mip(&[], &d("ci\\Y", Svadi), Lan, &["acinavam"]);
    assert_has_mip(&[], &kf, Lan, &["akaravam"]);
}

#[test]
fn sutra_3_4_104() {
    let iz = d("izu~", Tudadi);
    assert_has_tip(&[], &iz, AshirLin, &["izyAt"]);
    assert_has_tas(&[], &iz, AshirLin, &["izyAstAm"]);
    assert_has_jhi(&[], &iz, AshirLin, &["izyAsuH"]);
    let jagf = d("jAgf", Adadi);
    assert_has_tip(&[], &jagf, AshirLin, &["jAgaryAt"]);
    assert_has_tas(&[], &jagf, AshirLin, &["jAgaryAstAm"]);
    assert_has_jhi(&[], &jagf, AshirLin, &["jAgaryAsuH"]);
    // Counterexamples
    assert_has_vidhilin_p(&[], &d("va\\ca~", Adadi), &["vacyAt"]);
    assert_has_vidhilin_p(&[], &jagf, &["jAgfyAt"]);
}

#[test]
fn sutra_3_4_105() {
    assert_has_jha(&[], &d("qupa\\ca~^z", Bhvadi), VidhiLin, &["paceran"]);
    assert_has_jha(&[], &d("ya\\ja~^", Bhvadi), VidhiLin, &["yajeran"]);
    assert_has_jha(&[], &d("qukf\\Y", Tanadi), AshirLin, &["kfzIran"]);
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
    assert_has_jhi(&[], &d("qupa\\ca~^z", Bhvadi), VidhiLin, &["paceyuH"]);
    assert_has_jhi(&[], &d("ya\\ja~^", Bhvadi), VidhiLin, &["yajeyuH"]);
}

#[test]
fn sutra_3_4_109() {
    assert_has_jhi(&[], &d("qukf\\Y", Tanadi), Lun, &["akArzuH"]);
    assert_has_jhi(&[], &d("hf\\Y", Bhvadi), Lun, &["ahArzuH"]);
    assert_has_jhi(&[], &d("YiBI\\", Juhotyadi), Lan, &["abiBayuH"]);
    assert_has_jhi(&[], &d("hrI\\", Juhotyadi), Lan, &["ajihrayuH"]);
    assert_has_jhi(&[], &d("jAgf", Adadi), Lan, &["ajAgaruH"]);
    assert_has_jhi(&[], &d("vida~", Adadi), Lan, &["aviduH"]);
}

#[test]
fn sutra_3_4_110() {
    assert_has_jhi(&[], &d("qudA\\Y", Juhotyadi), Lun, &["aduH"]);
    assert_has_jhi(&[], &d("quDA\\Y", Juhotyadi), Lun, &["aDuH"]);
    assert_has_jhi(&[], &d("zWA\\", Bhvadi), Lun, &["asTuH"]);

    // AtaH, sic-luk
    assert_has_jhi(&[], &d("BU", Bhvadi), Lun, &["aBUvan"]);
    assert_has_jhi(&[], &d("qukf\\Y", Tanadi), Lun, &["akArzuH"]);
    assert_has_jhi(&[], &d("hf\\Y", Bhvadi), Lun, &["ahArzuH"]);
}

#[test]
fn sutra_3_4_111() {
    let yaa = d("yA\\", Adadi);
    assert_has_jhi(&[], &yaa, Lan, &["ayuH", "ayAn"]);
    assert_has_jhi(&[], &d("dA\\p", Adadi), Lan, &["aduH", "adAn"]);

    assert_has_jhi(&[], &yaa, Lot, &["yAntu"]);
    assert_has_jhi(&[], &d("vA\\", Adadi), Lot, &["vAntu"]);

    // not for Lot
    assert_has_jhi(&[], &d("YiBI\\", Juhotyadi), Lot, &["biByatu"]);
    assert_has_jhi(&[], &d("jAgf", Adadi), Lot, &["jAgratu"]);
    assert_has_jhi(&[], &d("vida~", Adadi), Lot, &["vidantu", "vidANkurvantu"]);
}

#[test]
fn sutra_3_4_112() {
    assert_has_jhi(&[], &d("dvi\\za~^", Adadi), Lan, &["advizuH", "advizan"]);
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

#[test]
fn sutra_3_4_114() {
    let lu = d("lUY", Kryadi);
    assert_has_krdanta(&[], &lu, Krt::tfc, &["lavitf"]);
    assert_has_krdanta(&[], &lu, Krt::tumun, &["lavitum"]);
    assert_has_krdanta(&[], &lu, Krt::tavya, &["lavitavya"]);

    // DAtoH
    assert_has_taddhitanta(&prati("vfkza"), T::tva, &["vfkzatva"]);
    assert_has_taddhitanta(&prati("vfkza"), T::tal, &["vfkzatA"]);
    assert_has_subantas("lU", Pum, V::Trtiya, Dvi, &["lUByAm"]);
    assert_has_subantas("lU", Pum, V::Trtiya, Bahu, &["lUBiH"]);
    assert_has_lat(&[], &d("gupa~\\", Bhvadi), &["jugupsate"]);
}

#[test]
fn sutra_3_4_115() {
    assert_has_sip(&[], &d("qupa\\ca~^z", Adadi), Lit, &["peciTa", "papakTa"]);
    assert_has_sip(&[], &d("Sa\\kx~", Adadi), Lit, &["SekiTa", "SaSakTa"]);
    assert_has_lit_karmani(&[], &d("glE\\", Bhvadi), &["jagle"]);
    assert_has_lit_karmani(&[], &d("mlE\\", Bhvadi), &["mamle"]);
}

#[test]
fn sutra_3_4_116() {
    let lu = d("lUY", Gana::Kryadi);
    let pu = d("pUY", Gana::Kryadi);

    assert_has_ashirlin_a(&[], &lu, &["lavizIzwa"]);
    assert_has_ashirlin_a(&[], &pu, &["pavizIzwa"]);

    assert_has_vidhilin_p(&[], &lu, &["lunIyAt"]);
    assert_has_vidhilin_p(&[], &pu, &["punIyAt"]);
}

// 3.4.117 is chAndasa.
