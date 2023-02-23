extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti::*;
use vidyut_prakriya::args::*;

pub fn assert_has_lit_p_1d(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(
        prefixes,
        dhatu,
        Lakara::Lit,
        Purusha::Uttama,
        Vacana::Dvi,
        expected,
    );
}

pub fn assert_has_lit_p_2s(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(
        prefixes,
        dhatu,
        Lakara::Lit,
        Purusha::Madhyama,
        Vacana::Eka,
        expected,
    );
}

pub fn assert_has_lit_a_1d(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    let args = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Uttama)
        .vacana(Vacana::Dvi)
        .lakara(Lakara::Lit)
        .pada(Pada::Atmane)
        .build()
        .unwrap();

    let actual = derive_tinantas(&dhatu.clone().with_prefixes(prefixes), &args);
    assert_padas(actual, expected);
}

#[test]
fn sutra_7_2_1() {
    let d = Dhatu::new;
    assert_has_lun_p(&[], &d("ci\\Y", Gana::Svadi), &["acEzIt"]);
    assert_has_lun_p(&[], &d("RI\\Y", Gana::Bhvadi), &["anEzIt"]);
    assert_has_lun_p(&[], &d("lUY", Gana::Kryadi), &["alAvIt"]);
    assert_has_lun_p(&[], &d("pUY", Gana::Kryadi), &["apAvIt"]);
    assert_has_lun_p(&[], &d("qukf\\Y", Gana::Tanadi), &["akArzIt"]);
    assert_has_lun_p(&[], &d("hf\\Y", Gana::Bhvadi), &["ahArzIt"]);

    let kutadi = |u| Dhatu::new(u, Gana::Tudadi).with_antargana(Some(Antargana::Kutadi));
    assert_has_lun_p(&["ni"], &kutadi("RU"), &["nyanuvIt"]);
    assert_has_lun_p(&["ni"], &kutadi("DU"), &["nyaDuvIt"]);

    assert_has_lun_a(&[], &d("cyu\\N", Gana::Bhvadi), &["acyozwa"]);
    assert_has_lun_a(&[], &d("plu\\N", Gana::Bhvadi), &["aplozwa"]);
}

#[test]
fn sutra_7_2_2() {
    let d = Dhatu::new;
    assert_has_lun_p(&[], &d("kzara~", Gana::Bhvadi), &["akzArIt"]);
    assert_has_lun_p(&[], &d("tsara~", Gana::Bhvadi), &["atsArIt"]);
    assert_has_lun_p(&[], &d("jvala~", Gana::Bhvadi), &["ajvAlIt"]);
    assert_has_lun_p(&[], &d("hmala~", Gana::Bhvadi), &["ahmAlIt"]);

    assert_has_lun_p(&["ni"], &d("Kura~", Gana::Tudadi), &["nyaKorIt"]);
    assert_has_lun_p(&["ni"], &d("mIla~", Gana::Svadi), &["nyamIlIt"]);

    // TODO: awIt, aSIt

    assert_has_lun_p(&[], &d("vaBra~", Gana::Bhvadi), &["avaBrIt"]);
    assert_has_lun_p(&[], &d("Svalla~", Gana::Bhvadi), &["aSvallIt"]);
}

#[test]
fn sutra_7_2_3() {
    let d = Dhatu::new;
    assert_has_lun_p(&[], &d("vada~", Gana::Bhvadi), &["avAdIt"]);
    assert_has_lun_p(&[], &d("vraja~", Gana::Bhvadi), &["avrAjIt"]);
    assert_has_lun_p(&[], &d("Bi\\di~^r", Gana::Rudhadi), &["aBEtsIt", "aBidat"]);
    assert_has_lun_p(
        &[],
        &d("Ci\\di~^r", Gana::Rudhadi),
        &["acCEtsIt", "acCidat"],
    );
    assert_has_lun_p(&[], &d("ru\\Di~^r", Gana::Rudhadi), &["arOtsIt", "aruDat"]);
    assert_has_lun_p(&[], &d("ra\\nja~^", Gana::Bhvadi), &["arANkzIt"]);
    assert_has_lun_p(&[], &d("za\\nja~", Gana::Bhvadi), &["asANkzIt"]);
    assert_has_parasmai_tinanta(
        &["ud"],
        &d("va\\ha~^", Gana::Bhvadi),
        Lakara::Lun,
        Purusha::Prathama,
        Vacana::Dvi,
        &["udavoQAm"],
    );
}

#[test]
fn sutra_7_2_4() {
    let d = Dhatu::new;
    assert_has_lun_p(&[], &d("divu~", Gana::Divadi), &["adevIt"]);
    assert_has_lun_p(&[], &d("zivu~", Gana::Divadi), &["asevIt"]);
    assert_has_lun_p(&[], &d("kuza~", Gana::Kryadi), &["akozIt"]);
    assert_has_lun_p(&[], &d("muza~", Gana::Kryadi), &["amozIt"]);
    assert_has_lun_p(&[], &d("lUY", Gana::Kryadi), &["alAvIt"]);
}

#[test]
fn sutra_7_2_5() {
    let d = Dhatu::new;
    assert_has_lun_p(&[], &d("graha~^", Gana::Kryadi), &["agrahIt"]);
    assert_has_lun_p(&[], &d("syamu~", Gana::Divadi), &["asyamIt"]);
    assert_has_lun_p(&[], &d("vyaya~^", Gana::Bhvadi), &["avyayIt"]);
    assert_has_lun_p(&[], &d("wuvama~", Gana::Bhvadi), &["avamIt"]);
    assert_has_lun_p(&[], &d("kzaRu~^", Gana::Kryadi), &["akzaRIt"]);
    assert_has_lun_p(&[], &d("Svasa~", Gana::Adadi), &["aSvasIt"]);
    assert_has_lun_p(&[], &d("jAgf", Gana::Adadi), &["ajAgarIt"]);
    assert_has_lun_p(&[], &d("rage~", Gana::Bhvadi), &["aragIt"]);
    assert_has_lun_p(&[], &d("kaKe~", Gana::Kryadi), &["akaKIt"]);
    // Ignore OnayIt and aSvayIt, which are chAndasa forms.
}

#[test]
fn sutra_7_2_6() {
    let urnu = Dhatu::new("UrRuY", Gana::Adadi);
    assert_has_lun_p(&["pra"], &urnu, &["prOrRavIt", "prOrRAvIt", "prOrRuvIt"]);
}

#[test]
fn sutra_7_2_7() {
    let d = Dhatu::new;

    assert_has_lun_p(&[], &d("kaRa~", Gana::Bhvadi), &["akaRIt", "akARIt"]);
    assert_has_lun_p(&[], &d("raRa~", Gana::Bhvadi), &["araRIt", "arARIt"]);

    let kuw = Dhatu::new("kuwa~", Gana::Tudadi).with_antargana(Some(Antargana::Kutadi));
    let puw = Dhatu::new("puwa~", Gana::Tudadi).with_antargana(Some(Antargana::Kutadi));

    // Various counterexamples
    assert_has_lun_p(&[], &d("divu~", Gana::Divadi), &["adevIt"]);
    assert_has_lun_p(&[], &d("zivu~", Gana::Divadi), &["asevIt"]);
    assert_has_lun_p(&["ni"], &kuw, &["nyakuwIt"]);
    assert_has_lun_p(&["ni"], &puw, &["nyapuwIt"]);
    assert_has_lun_p(&[], &d("takza~", Gana::Bhvadi), &["atakzIt"]);
    assert_has_lun_p(&[], &d("rakza~", Gana::Divadi), &["arakzIt"]);
    assert_has_lun_p(&[], &d("cakAsf~", Gana::Adadi), &["acakAsIt"]);
    assert_has_lun_p(&[], &d("qupa\\ca~^z", Gana::Bhvadi), &["apAkzIt"]);

    // TODO: aSIt, awIt
}

#[test]
fn sutra_7_2_11() {
    let shri = Dhatu::new("SriY", Gana::Bhvadi);
    assert_has_krdanta(&[], &shri, Krt::ktvA, &["SritvA"]);
    assert_has_krdanta(&[], &shri, Krt::kta, &["Srita"]);
    assert_has_krdanta(&[], &shri, Krt::ktavatu, &["Sritavat"]);

    let yu = Dhatu::new("yu", Gana::Adadi);
    assert_has_krdanta(&[], &yu, Krt::ktvA, &["yutvA"]);
    assert_has_krdanta(&[], &yu, Krt::kta, &["yuta"]);
    assert_has_krdanta(&[], &yu, Krt::ktavatu, &["yutavat"]);

    let lu = Dhatu::new("lUY", Gana::Kryadi);
    assert_has_krdanta(&[], &lu, Krt::ktvA, &["lUtvA"]);
    assert_has_krdanta(&[], &lu, Krt::kta, &["lUna"]);
    assert_has_krdanta(&[], &lu, Krt::ktavatu, &["lUnavat"]);

    let vf = Dhatu::new("vfN", Gana::Kryadi);
    assert_has_krdanta(&[], &vf, Krt::ktvA, &["vftvA"]);
    assert_has_krdanta(&[], &vf, Krt::kta, &["vfta"]);
    assert_has_krdanta(&[], &vf, Krt::ktavatu, &["vftavat"]);

    let tf = Dhatu::new("tF", Gana::Bhvadi);
    assert_has_krdanta(&[], &tf, Krt::ktvA, &["tIrtvA"]);
    assert_has_krdanta(&[], &tf, Krt::kta, &["tIrRa"]);
    assert_has_krdanta(&[], &tf, Krt::ktavatu, &["tIrRavat"]);

    let vid = Dhatu::new("vida~", Gana::Adadi);
    assert_has_krdanta(&[], &vid, Krt::kta, &["vidita"]);

    assert_has_krdanta(&[], &shri, Krt::tfc, &["Srayitf"]);
    assert_has_krdanta(&[], &shri, Krt::tumun, &["Srayitum"]);
    assert_has_krdanta(&[], &shri, Krt::tavya, &["Srayitavya"]);
}

#[test]
fn sutra_7_2_12() {
    let d = |u, gana| Dhatu::new(u, gana).with_sanadi(&[Sanadi::San]);
    assert_has_lat_p(&[], &d("graha~^", Gana::Kryadi), &["jiGfkzati"]);
    assert_has_lat_p(&[], &d("guhU~^", Gana::Bhvadi), &["juGukzati"]);
    assert_has_lat_p(&[], &d("ru", Gana::Adadi), &["rurUzati"]);
    assert_has_lat_p(&[], &d("lUY", Gana::Kryadi), &["lulUzati"]);
}

#[test]
fn sutra_7_2_13() {
    let d = Dhatu::new;
    assert_has_lit_p_1d(&[], &d("qukf\\Y", Gana::Tanadi), &["cakfva"]);
    assert_has_lit_p_1d(&[], &d("sf\\", Gana::Bhvadi), &["sasfva"]);
    assert_has_lit_p_1d(&[], &d("Bf\\Y", Gana::Bhvadi), &["baBfva"]);
    assert_has_lit_p_1d(
        &[],
        &d("quBf\\Y", Gana::Juhotyadi),
        &["baBfva", "biBarAYcakfva", "biBarAmAsiva", "biBarAmbaBUviva"],
    );
    assert_has_lit_p_1d(&[], &d("vfY", Gana::Svadi), &["vavfva"]);
    assert_has_lit_a_1d(&[], &d("vfN", Gana::Kryadi), &["vavfvahe"]);
    assert_has_lit_p_1d(&[], &d("zwu\\Y", Gana::Adadi), &["tuzwuva"]);
    assert_has_lit_p_1d(&[], &d("dru\\", Gana::Bhvadi), &["dudruva"]);
    assert_has_lit_p_1d(&[], &d("sru\\", Gana::Bhvadi), &["susruva"]);
    assert_has_lit_p_1d(&[], &d("Sru\\", Gana::Bhvadi), &["SuSruva"]);
}

#[test]
fn sutra_7_2_14() {
    let d = Dhatu::new;

    let shvi = d("wuo~Svi", Gana::Bhvadi);
    assert_has_krdanta(&[], &shvi, Krt::kta, &["SUna"]);
    assert_has_krdanta(&[], &shvi, Krt::ktavatu, &["SUnavat"]);

    let lasj = d("o~lasjI~\\", Gana::Tudadi);
    assert_has_krdanta(&[], &lasj, Krt::kta, &["lagna"]);
    assert_has_krdanta(&[], &lasj, Krt::ktavatu, &["lagnavat"]);

    let vij = d("o~vijI~", Gana::Rudhadi);
    assert_has_krdanta(&["ud"], &vij, Krt::kta, &["udvigna"]);
    assert_has_krdanta(&["ud"], &vij, Krt::ktavatu, &["udvignavat"]);

    let dip = d("dIpI~\\", Gana::Rudhadi);
    assert_has_krdanta(&[], &dip, Krt::kta, &["dIpta"]);
    assert_has_krdanta(&[], &dip, Krt::ktavatu, &["dIptavat"]);

    let vij = d("o~vijI~", Gana::Rudhadi);
    assert_has_krdanta(&["ud"], &vij, Krt::kta, &["udvigna"]);
    assert_has_krdanta(&["ud"], &vij, Krt::ktavatu, &["udvignavat"]);

    // todo: uqqIna
}

#[test]
fn sutra_7_2_15() {
    let dhu = Dhatu::new("DUY", Gana::Svadi);
    assert_has_krdanta(&["vi"], &dhu, Krt::kta, &["viDUta"]);
    assert_has_krdanta(&["vi"], &dhu, Krt::ktavatu, &["viDUtavat"]);

    let guh = Dhatu::new("guhU~^", Gana::Bhvadi);
    assert_has_krdanta(&[], &guh, Krt::kta, &["gUQa"]);
    assert_has_krdanta(&[], &guh, Krt::ktavatu, &["gUQavat"]);

    let vfdh = Dhatu::new("vfDu~\\", Gana::Bhvadi);
    assert_has_krdanta(&[], &vfdh, Krt::kta, &["vfdDa"]);
    assert_has_krdanta(&[], &vfdh, Krt::ktavatu, &["vfdDavat"]);
}

#[test]
fn sutra_7_2_16() {
    let mid = Dhatu::new("YimidA~", Gana::Divadi);
    assert_has_krdanta(&[], &mid, Krt::kta, &["minna"]);
    assert_has_krdanta(&[], &mid, Krt::ktavatu, &["minnavat"]);

    let kzvid = Dhatu::new("YikzvidA~", Gana::Divadi);
    assert_has_krdanta(&[], &kzvid, Krt::kta, &["kzviRRa"]);
    assert_has_krdanta(&[], &kzvid, Krt::ktavatu, &["kzviRRavat"]);

    let svid = Dhatu::new("YizvidA~", Gana::Bhvadi);
    assert_has_krdanta(&[], &svid, Krt::kta, &["svinna"]);
    assert_has_krdanta(&[], &svid, Krt::ktavatu, &["svinnavat"]);
}

#[test]
fn sutra_7_2_24() {
    let ard = Dhatu::new("arda~", Gana::Bhvadi);
    assert_has_krdanta(&["sam"], &ard, Krt::kta, &["samarRa", "samarRRa"]);
    assert_has_krdanta(&["ni"], &ard, Krt::kta, &["nyarRa", "nyarRRa"]);
    assert_has_krdanta(&["vi"], &ard, Krt::kta, &["vyarRa", "vyarRRa"]);

    let edh = Dhatu::new("eDa~\\", Gana::Bhvadi);
    assert_has_krdanta(&["sam"], &edh, Krt::kta, &["sameDita"]);

    assert_has_krdanta(&[], &ard, Krt::kta, &["ardita"]);
}

#[test]
fn sutra_7_2_25() {
    let ard = Dhatu::new("arda~", Gana::Bhvadi);
    assert_has_krdanta(
        &["aBi"],
        &ard,
        Krt::kta,
        &["aByarRa", "aByarRRa", "aByardita"],
    );
}

/*
#[test]
fn sutra_7_2_28() {
    let d = Dhatu::new;
    let assert_has_kta = |x, y, z| assert_has_krdanta(x, &y, Krt::kta, z);
    assert_has_kta(&[], d("ruza~", Gana::Bhvadi), &["ruzwa", "ruzita"]);
    assert_has_kta(&["aBi"], d("ama~", Gana::Bhvadi), &["aByAnta", "aByamita"]);
}
*/

#[test]
fn sutra_7_2_35() {
    let lu = Dhatu::new("lUY", Gana::Kryadi);
    assert_has_krdanta(&[], &lu, Krt::tfc, &["lavitf"]);
    assert_has_krdanta(&[], &lu, Krt::tumun, &["lavitum"]);
    assert_has_krdanta(&[], &lu, Krt::tavya, &["lavitavya"]);

    let pu = Dhatu::new("pUY", Gana::Kryadi);
    assert_has_krdanta(&[], &pu, Krt::tfc, &["pavitf"]);
    assert_has_krdanta(&[], &pu, Krt::tumun, &["pavitum"]);
    assert_has_krdanta(&[], &pu, Krt::tavya, &["pavitavya"]);

    // TODO: Aste, vaste

    assert_has_krdanta(&[], &lu, Krt::yat, &["lavya"]);
    assert_has_krdanta(&[], &pu, Krt::yat, &["pavya"]);
    assert_has_krdanta(&[], &lu, Krt::anIyar, &["lavanIya"]);
    assert_has_krdanta(&[], &pu, Krt::anIyar, &["pavanIya"]);
}

#[test]
fn sutra_7_2_37() {
    let grah = Dhatu::new("graha~^", Gana::Kryadi);
    assert_has_krdanta(&[], &grah, Krt::tfc, &["grahItf"]);
    assert_has_krdanta(&[], &grah, Krt::tumun, &["grahItum"]);
    assert_has_krdanta(&[], &grah, Krt::tavya, &["grahItavya"]);

    assert_has_lit_p_1d(&[], &grah, &["jagfhiva"]);

    // TODO: ignore cinvat it
}

#[test]
fn sutra_7_2_62() {
    let yaj = Dhatu::new("ya\\ja~^", Gana::Bhvadi);
    assert_has_lit_p_2s(&[], &yaj, &["iyazWa", "iyajiTa"]);
}

#[test]
fn sutra_7_2_66() {
    let d = Dhatu::new;
    // TODO: check jaGasTa
    assert_has_lit_p_2s(
        &[],
        &d("a\\da~", Gana::Adadi),
        &["AdiTa", "jaGasiTa", "jaGasTa"],
    );
    assert_has_lit_p_2s(&[], &d("f\\", Gana::Bhvadi), &["AriTa"]);
    assert_has_lit_p_2s(&[], &d("vye\\Y", Gana::Bhvadi), &["vivyayiTa"]);
}

#[test]
fn sutra_7_2_68() {
    let assert_has_kvasu = |d, exp| {
        assert_has_krdanta(&[], &d, Krt::kvasu, exp);
    };

    assert_has_kvasu(
        Dhatu::new("ga\\mx~", Gana::Bhvadi),
        &["jagmivas", "jaganvas"],
    );
    assert_has_kvasu(
        Dhatu::new("ha\\na~", Gana::Adadi),
        &["jaGnivas", "jaGanvas"],
    );
    assert_has_kvasu(
        Dhatu::new("vida~", Gana::Adadi),
        &[
            "vividivas",
            "vividvas",
            "vidAmAsivas",
            "vidAmbaBUvas",
            "vidAYcakfvas",
        ],
    );
    assert_has_kvasu(
        Dhatu::new("vi\\Sa~", Gana::Tudadi),
        &["viviSivas", "viviSvas"],
    );
    assert_has_kvasu(
        Dhatu::new("df\\Si~r", Gana::Bhvadi),
        &["dadfSivas", "dadfSvas"],
    );
}

#[test]
fn sutra_7_2_70() {
    let d = Dhatu::new;
    assert_has_lrt_p(&[], &d("qukf\\Y", Gana::Tanadi), &["karizyati"]);
    assert_has_lrt_p(&[], &d("hf\\Y", Gana::Bhvadi), &["harizyati"]);
    assert_has_lrt_p(&[], &d("ha\\na~", Gana::Adadi), &["hanizyati"]);
    assert_has_lrt_p(&[], &d("svf", Gana::Bhvadi), &["svarizyati"]);
}

#[test]
fn sutra_7_2_71() {
    let anj = Dhatu::new("anjU~", Gana::Rudhadi);
    assert_has_lun(&[], &anj, &["AYjIt"]);
    assert_has_krdanta(&[], &anj, Krt::tfc, &["aNktf", "aYjitf"]);
}

#[test]
fn sutra_7_2_72() {
    let stu = Dhatu::new("zwu\\Y", Gana::Adadi);
    let su = Dhatu::new("zu\\Y", Gana::Svadi);
    let dhu = Dhatu::new("DUY", Gana::Svadi);

    assert_has_lun_p(&[], &stu, &["astAvIt"]);
    assert_has_lun_p(&[], &su, &["asAvIt"]);
    assert_has_lun_p(&[], &dhu, &["aDAvIt"]);

    assert_has_lun_a(&[], &stu, &["astozwa"]);
    assert_has_lun_a(&[], &su, &["asozwa"]);
    assert_has_lun_a(&[], &dhu, &["aDozwa", "aDavizwa"]);
}

#[test]
fn sutra_7_2_73() {
    let yam = Dhatu::new("ya\\ma~", Gana::Bhvadi);
    let ram = Dhatu::new("ra\\mu~\\", Gana::Bhvadi);
    let nam = Dhatu::new("Ra\\ma~", Gana::Bhvadi);
    let yaa = Dhatu::new("yA\\", Gana::Adadi);

    assert_has_lun_p(&[], &yam, &["ayaMsIt"]);
    assert_has_lun_p(&["vi"], &ram, &["vyaraMsIt"]);
    assert_has_lun(&[], &nam, &["anaMsIt"]);
    assert_has_lun_p(&[], &yaa, &["ayAsIt"]);

    // TODO: not sure how to derive ayaMsta and anaMsta
    // assert_has_lun_karmani(&[], &yam, &["ayaMsta"]);
    assert_has_lun_a(&[], &ram, &["araMsta"]);
    // assert_has_lun_karmani(&[], &nam, &["anaMsta"]);
}

#[ignore]
#[test]
fn sutra_7_2_74() {
    let assert_has_san_lat = |u, gana, exp| {
        let dhatu = Dhatu::new(u, gana).with_sanadi(&[Sanadi::San]);
        assert_has_lat(&[], &dhatu, exp);
    };

    assert_has_san_lat("zmi\\N", Gana::Bhvadi, &["sismayizate"]);
    assert_has_san_lat("pUN", Gana::Bhvadi, &["pipavizate"]);
    assert_has_san_lat("f\\", Gana::Bhvadi, &["aririzati"]);
    assert_has_san_lat("anjU~", Gana::Rudhadi, &["aYjijizati"]);
    assert_has_san_lat("aSU~\\", Gana::Svadi, &["aSiSizate"]);
    assert_has_san_lat("pUY", Gana::Kryadi, &["pupUzati"]);
}

#[ignore]
#[test]
fn sutra_7_2_75() {
    let assert_has_san_lat = |u, gana, exp| {
        let dhatu = Dhatu::new(u, gana).with_sanadi(&[Sanadi::San]);
        assert_has_lat(&[], &dhatu, exp);
    };

    assert_has_san_lat("kF", Gana::Tudadi, &["cikarizati"]);
    assert_has_san_lat("gF", Gana::Tudadi, &["jigarizati", "jigalizati"]);
    assert_has_san_lat("df\\N", Gana::Tudadi, &["didarizate"]);
    assert_has_san_lat("Df\\N", Gana::Tudadi, &["diDarizate"]);
    assert_has_san_lat("pra\\Ca~", Gana::Tudadi, &["papracCizati"]);
    assert_has_san_lat("sf\\ja~", Gana::Tudadi, &["sisfkzati"]);
}

#[test]
fn sutra_7_2_76() {
    let d = Dhatu::new;
    let rud = d("rudi~r", Gana::Adadi);
    let svap = d("Yizva\\pa~", Gana::Adadi);

    assert_has_lat_p(&[], &rud, &["roditi"]);
    assert_has_lat_p(&[], &svap, &["svapiti"]);
    assert_has_lat_p(&[], &d("Svasa~", Gana::Adadi), &["Svasiti"]);
    assert_has_lat_p(&["pra"], &d("ana~", Gana::Adadi), &["prARiti"]);
    assert_has_lat_p(&[], &d("jakza~", Gana::Adadi), &["jakziti"]);

    assert_has_lat_p(&[], &d("jAgf", Gana::Adadi), &["jAgarti"]);
    assert_has_krdanta(&[], &svap, Krt::tfc, &["svaptf"]);
    assert_has_parasmai_tinanta(
        &[],
        &rud,
        Lakara::Lat,
        Purusha::Prathama,
        Vacana::Bahu,
        &["rudanti"],
    );
}

#[test]
fn sutra_7_2_77() {
    let is = Dhatu::new("ISa~\\", Gana::Adadi);
    assert_has_tinanta(
        &[],
        &is,
        Lakara::Lat,
        Purusha::Madhyama,
        Vacana::Eka,
        &["ISize"],
    );
    assert_has_tinanta(
        &[],
        &is,
        Lakara::Lot,
        Purusha::Madhyama,
        Vacana::Eka,
        &["ISizva"],
    );
}

#[test]
fn sutra_7_2_80() {
    let d = Dhatu::new;
    assert_has_vidhilin_p(&[], &d("qupa\\ca~^z", Gana::Bhvadi), &["pacet"]);
    assert_has_vidhilin_p(&[], &d("ci\\Y", Gana::Svadi), &["cinuyAt"]);
    assert_has_vidhilin_p(&[], &d("zu\\Y", Gana::Svadi), &["sunuyAt"]);
    assert_has_vidhilin_p(&[], &d("yA\\", Gana::Adadi), &["yAyAt"]);

    let _cikirs = d("qukf\\Y", Gana::Tanadi).with_sanadi(&[Sanadi::San]);
    // TODO: cikIrzyAt
}

#[test]
fn sutra_7_2_83() {
    let aas = Dhatu::new("Asa~\\", Gana::Adadi);
    assert_has_krdanta(&[], &aas, Krt::SAnac, &["AsIna"]);
}

#[test]
fn sutra_7_2_85() {
    assert_has_subantas("rE", Pum, Trtiya, Dvi, &["rAByAm"]);
    assert_has_subantas("rE", Pum, Trtiya, Bahu, &["rABiH"]);

    assert_has_subantas("rE", Pum, Prathama, Dvi, &["rAyO"]);
    // TOO: rEtva, rEtA
}

#[test]
fn sutra_7_2_87() {
    assert_has_subantas("yuzmad", Pum, Dvitiya, Eka, &["tvAm"]);
    assert_has_subantas("asmad", Pum, Dvitiya, Eka, &["mAm"]);
    assert_has_subantas("yuzmad", Pum, Dvitiya, Dvi, &["yuvAm"]);
    assert_has_subantas("asmad", Pum, Dvitiya, Dvi, &["AvAm"]);
}

#[test]
fn sutra_7_2_88() {
    assert_has_subantas("yuzmad", Pum, Prathama, Dvi, &["yuvAm"]);
    assert_has_subantas("asmad", Pum, Prathama, Dvi, &["AvAm"]);

    assert_has_subantas("yuzmad", Pum, Sasthi, Dvi, &["yuvayoH"]);
    assert_has_subantas("asmad", Pum, Sasthi, Dvi, &["AvayoH"]);
    assert_has_subantas("yuzmad", Pum, Prathama, Eka, &["tvam"]);
    assert_has_subantas("asmad", Pum, Prathama, Eka, &["aham"]);
    assert_has_subantas("yuzmad", Pum, Prathama, Bahu, &["yUyam"]);
    assert_has_subantas("asmad", Pum, Prathama, Bahu, &["vayam"]);
}

#[test]
fn sutra_7_2_89() {
    assert_has_subantas("yuzmad", Pum, Trtiya, Eka, &["tvayA"]);
    assert_has_subantas("asmad", Pum, Trtiya, Eka, &["mayA"]);
    assert_has_subantas("yuzmad", Pum, Saptami, Eka, &["tvayi"]);
    assert_has_subantas("asmad", Pum, Saptami, Eka, &["mayi"]);
    assert_has_subantas("yuzmad", Pum, Saptami, Dvi, &["yuvayoH"]);
    assert_has_subantas("asmad", Pum, Saptami, Dvi, &["AvayoH"]);

    assert_has_subantas("yuzmad", Pum, Trtiya, Dvi, &["yuvAByAm"]);
    assert_has_subantas("asmad", Pum, Trtiya, Dvi, &["AvAByAm"]);

    assert_has_subantas("yuzmad", Pum, Panchami, Eka, &["tvat"]);
    assert_has_subantas("asmad", Pum, Panchami, Eka, &["mat"]);
}
