extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::*;
use vidyut_prakriya::Vyakarana;

fn assert_has_lrt_sat(dhatu: &Dhatu, krt: BaseKrt, expected: &[&str]) {
    let v = Vyakarana::new();
    let args = Krdanta::builder()
        .lakara(Lrt)
        .dhatu(dhatu.clone())
        .krt(krt)
        .build()
        .unwrap();
    let prakriyas = v.derive_krdantas(&args);
    assert_has_results(prakriyas, expected)
}

#[test]
fn sutra_3_2_1() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_upapada_krdanta("kumBa", &[], &kf, Krt::aR, &["kumBakAra"]);
    assert_has_upapada_krdanta("nagara", &[], &kf, Krt::aR, &["nagarakAra"]);

    let i = d("i\\N", Adadi);
    assert_has_upapada_krdanta("veda", &["aDi"], &i, Krt::aR, &["vedADyAya"]);
    let lu = d("lUY", Kryadi);
    assert_has_upapada_krdanta("kaRqa", &[], &lu, Krt::aR, &["kaRqalAva"]);
    assert_has_upapada_krdanta("Sara", &[], &lu, Krt::aR, &["SaralAva"]);

    let pf = d("pF", Juhotyadi);
    assert_has_upapada_krdanta("carcA", &[], &pf, Krt::aR, &["carcApAra"]);
}

#[test]
fn sutra_3_2_2() {
    let hve = &d("hve\\Y", Bhvadi);
    let ve = d("ve\\Y", Bhvadi);
    let maa = d("mA\\N", Juhotyadi);
    assert_has_upapada_krdanta("svarga", &[], &hve, Krt::aR, &["svargahvAya"]);
    assert_has_upapada_krdanta("tantu", &[], &ve, Krt::aR, &["tantuvAya"]);
    assert_has_upapada_krdanta("DAnya", &[], &maa, Krt::aR, &["DAnyamAya"]);

    // ka-pratyaya apavAda --
    assert_has_upapada_krdanta("svarga", &[], &hve, Krt::ka, &[]);
    assert_has_upapada_krdanta("tantu", &[], &ve, Krt::ka, &[]);
    assert_has_upapada_krdanta("DAnya", &[], &maa, Krt::ka, &[]);
}

#[ignore]
#[test]
fn sutra_3_2_3() {
    let daa = d("qudA\\Y", Juhotyadi);
    let trai = d("trE\\N", Bhvadi);
    assert_has_upapada_krdanta("kambala", &[], &daa, Krt::ka, &["kambalada"]);
    assert_has_upapada_krdanta("pArzRi", &[], &trai, Krt::ka, &["pArzRitra"]);
    assert_has_upapada_krdanta("aNguli", &[], &trai, Krt::ka, &["aNgulitra"]);
    // an-upasarge?
    assert_has_upapada_krdanta("go", &["sam"], &daa, Krt::aR, &["gosandAya"]);
    assert_has_upapada_krdanta("vAqava", &["sam"], &daa, Krt::aR, &["vAqavasandAya"]);

    // aN-pratyaya apavAda --
    assert_has_upapada_krdanta("kambala", &[], &daa, Krt::aR, &[]);
    assert_has_upapada_krdanta("pArzRi", &[], &trai, Krt::aR, &[]);
    assert_has_upapada_krdanta("aNguli", &[], &trai, Krt::aR, &[]);
}

#[test]
fn sutra_3_2_4() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_upapada_krdanta("sama", &[], &stha, Krt::ka, &["samasTa"]);
    assert_has_upapada_krdanta("vizama", &[], &stha, Krt::ka, &["vizamasTa"]);
    // TODO: others
}

#[test]
fn sutra_3_2_5() {
    let mfj = d("mfjU~", Adadi);
    let nud = d("Ru\\da~^", Tudadi);
    assert_has_upapada_krdanta("tunda", &["pari"], &mfj, Krt::ka, &["tundaparimfja"]);
    assert_has_upapada_krdanta("Soka", &["apa"], &nud, Krt::ka, &["SokApanuda"]);
}

#[test]
fn sutra_3_2_6() {
    let daa = d("qudA\\Y", Juhotyadi);
    let jna = d("jYA\\", Kryadi);
    assert_has_upapada_krdanta("sarva", &["pra"], &daa, Krt::ka, &["sarvaprada"]);
    assert_has_upapada_krdanta("paTin", &["pra"], &jna, Krt::ka, &["paTiprajYa"]);
    // pre?
    assert_has_upapada_krdanta("go", &["sam"], &daa, Krt::aR, &["gosandAya"]);
}

#[ignore]
#[test]
fn sutra_3_2_7() {
    let caksh = d("ca\\kzi~\\N", Adadi);
    assert_has_upapada_krdanta("go", &["sam"], &caksh, Krt::ka, &["gosaNKya"]);
    assert_has_upapada_krdanta("go", &["sam"], &caksh, Krt::aR, &[]);
}

#[test]
fn sutra_3_2_8() {
    let gai = d("gE\\", Bhvadi);
    assert_has_upapada_krdanta("Sakra", &[], &gai, Krt::wak, &["Sakraga"]);
    assert_has_upapada_krdanta("sAman", &[], &gai, Krt::wak, &["sAmaga"]);

    // an-upasarge?
    assert_has_upapada_krdanta("Sakra", &["sam"], &gai, Krt::wak, &[]);
    assert_has_upapada_krdanta("Sakra", &["sam"], &gai, Krt::aR, &["SakrasaNgAya"]);
    assert_has_upapada_krdanta("sAman", &["sam"], &gai, Krt::wak, &[]);
    assert_has_upapada_krdanta("sAman", &["sam"], &gai, Krt::aR, &["sAmasaNgAya"]);

    // TODO: stri
}

#[test]
fn sutra_3_2_8_v1() {
    let paa = d("pA\\", Bhvadi);
    assert_has_upapada_krdanta("surA", &[], &paa, Krt::wak, &["surApa"]);
    assert_has_upapada_krdanta("SIDu", &[], &paa, Krt::wak, &["SIDupa"]);

    // other upapadas:
    assert_has_upapada_krdanta("kzIra", &[], &paa, Krt::wak, &[]);
    assert_has_upapada_krdanta("kzIra", &[], &paa, Krt::ka, &["kzIrapa"]);

    // other pA dhAtus
    let paa2 = d("pA\\", Adadi);
    assert_has_upapada_krdanta("surA", &[], &paa2, Krt::wak, &[]);
    // TODO: stri
}

#[test]
fn sutra_3_2_9() {
    let hf = d("hf\\Y", Bhvadi);
    assert_has_upapada_krdanta("aMSa", &[], &hf, Krt::ac, &["aMSahara"]);
    assert_has_upapada_krdanta("rikTa", &[], &hf, Krt::ac, &["rikTahara"]);

    // anudyamana?
    assert_has_upapada_krdanta("BAra", &[], &hf, Krt::aR, &["BArahAra"]);
}

#[test]
fn sutra_3_2_9_v1() {
    let grah = d("graha~^", Kryadi);
    assert_has_upapada_krdanta("Sakti", &[], &grah, Krt::ac, &["Saktigraha"]);
    assert_has_upapada_krdanta("lANgala", &[], &grah, Krt::ac, &["lANgalagraha"]);
    assert_has_upapada_krdanta("aNkuSa", &[], &grah, Krt::ac, &["aNkuSagraha"]);
    assert_has_upapada_krdanta("yazwi", &[], &grah, Krt::ac, &["yazwigraha"]);
    assert_has_upapada_krdanta("tomara", &[], &grah, Krt::ac, &["tomaragraha"]);
    assert_has_upapada_krdanta("Gawa", &[], &grah, Krt::ac, &["Gawagraha"]);
    assert_has_upapada_krdanta("GawI", &[], &grah, Krt::ac, &["GawIgraha"]);
    assert_has_upapada_krdanta("Danus", &[], &grah, Krt::ac, &["Danurgraha"]);

    // apavAda?
    assert_has_upapada_krdanta("Sakti", &[], &grah, Krt::aR, &["SaktigrAha"]);
}

#[test]
fn sutra_3_2_9_v2() {
    let grah = d("graha~^", Kryadi);
    assert_has_upapada_krdanta("sUtra", &[], &grah, Krt::ac, &["sUtragraha"]);
    assert_has_upapada_krdanta("sUtra", &[], &grah, Krt::aR, &["sUtragrAha"]);
}

#[test]
fn sutra_3_2_10() {
    let hf = d("hf\\Y", Bhvadi);
    assert_has_upapada_krdanta("asTin", &[], &hf, Krt::ac, &["asTihara"]);
    assert_has_upapada_krdanta("kavaca", &[], &hf, Krt::ac, &["kavacahara"]);
}

#[test]
fn sutra_3_2_11() {
    let hf = d("hf\\Y", Bhvadi);
    assert_has_upapada_krdanta("puzpa", &["AN"], &hf, Krt::ac, &["puzpAhara"]);
    assert_has_upapada_krdanta("Pala", &["AN"], &hf, Krt::ac, &["PalAhara"]);
}

#[test]
fn sutra_3_2_12() {
    let arh = d("arha~", Bhvadi);
    assert_has_upapada_krdanta("pUjA", &[], &arh, Krt::ac, &["pUjArha"]);
    assert_has_upapada_krdanta("ganDa", &[], &arh, Krt::ac, &["ganDArha"]);
    assert_has_upapada_krdanta("mAlA", &[], &arh, Krt::ac, &["mAlArha"]);
    // TODO: stri
}

#[ignore]
#[test]
fn sutra_3_2_13() {
    assert_has_upapada_krdanta(
        "stamba",
        &[],
        &d("ra\\ma~\\", Bhvadi),
        Krt::ac,
        &["stamberama"],
    );
    assert_has_upapada_krdanta("karRa", &[], &d("japa~", Bhvadi), Krt::ac, &["karRejapa"]);
}

#[test]
fn sutra_3_2_14() {
    assert_has_upapada_krdanta("Sam", &[], &d("qukf\\Y", Tanadi), Krt::ac, &["SaNkara"]);
    assert_has_upapada_krdanta("Sam", &[], &d("BU", Bhvadi), Krt::ac, &["SamBava"]);
    assert_has_upapada_krdanta("Sam", &[], &d("vada~", Bhvadi), Krt::ac, &["SaMvada"]);
}

#[test]
fn sutra_3_2_15() {
    let shi = d("SIN", Adadi);
    assert_has_upapada_krdanta("Ka", &[], &shi, Krt::ac, &["KaSaya"]);
    assert_has_upapada_krdanta("garta", &[], &shi, Krt::ac, &["gartaSaya"]);
}

#[test]
fn sutra_3_2_16() {
    let car = d("cara~", Bhvadi);
    assert_has_upapada_krdanta("kuru", &[], &car, Krt::wa, &["kurucara"]);
    assert_has_upapada_krdanta("madra", &[], &car, Krt::wa, &["madracara"]);
    // TODO: stri
}

#[test]
fn sutra_3_2_17() {
    let car = d("cara~", Bhvadi);
    assert_has_upapada_krdanta("BikzA", &[], &car, Krt::wa, &["BikzAcara"]);
    assert_has_upapada_krdanta("senA", &[], &car, Krt::wa, &["senAcara"]);
    assert_has_upapada_krdanta("AdAya", &[], &car, Krt::wa, &["AdAyacara"]);
}

#[test]
fn sutra_3_2_18() {
    let sf = d("sf\\", Bhvadi);
    assert_has_upapada_krdanta("puraH", &[], &sf, Krt::wa, &["puraHsara", "purassara"]);
    assert_has_upapada_krdanta(
        "agrataH",
        &[],
        &sf,
        Krt::wa,
        &["agrataHsara", "agratassara"],
    );
    assert_has_upapada_krdanta("agre", &[], &sf, Krt::wa, &["agresara"]);
}

#[test]
fn sutra_3_2_19() {
    let sf = d("sf\\", Bhvadi);
    assert_has_upapada_krdanta("pUrva", &[], &sf, Krt::wa, &["pUrvasara"]);
}

#[test]
fn sutra_3_2_20() {
    use Krt::wa;
    let kf = d("qukf\\Y", Tanadi);
    assert_has_upapada_krdanta("Soka", &[], &kf, wa, &["Sokakara"]);
    assert_has_upapada_krdanta("yaSas", &[], &kf, wa, &["yaSaskara"]);
    assert_has_upapada_krdanta("kula", &[], &kf, wa, &["kulakara"]);
    assert_has_upapada_krdanta("SrAdDa", &[], &kf, wa, &["SrAdDakara"]);
    assert_has_upapada_krdanta("arTa", &[], &kf, wa, &["arTakara"]);
    assert_has_upapada_krdanta("prEza", &[], &kf, wa, &["prEzakara"]);
    assert_has_upapada_krdanta("vacana", &[], &kf, wa, &["vacanakara"]);
    // TODO: stri
}

#[test]
fn sutra_3_2_21() {
    use Krt::wa;
    let kf = d("qukf\\Y", Tanadi);
    assert_has_upapada_krdanta("divA", &[], &kf, wa, &["divAkara"]);
    assert_has_upapada_krdanta("viBA", &[], &kf, wa, &["viBAkara"]);
    assert_has_upapada_krdanta("niSA", &[], &kf, wa, &["niSAkara"]);
    assert_has_upapada_krdanta("praBA", &[], &kf, wa, &["praBAkara"]);
    assert_has_upapada_krdanta(phit("BAs"), &[], &kf, wa, &["BAskara"]);
    assert_has_upapada_krdanta("kAra", &[], &kf, wa, &["kArakara"]);
    assert_has_upapada_krdanta("anta", &[], &kf, wa, &["antakara"]);
    assert_has_upapada_krdanta("ananta", &[], &kf, wa, &["anantakara"]);
    assert_has_upapada_krdanta("Adi", &[], &kf, wa, &["Adikara"]);
    assert_has_upapada_krdanta("bahu", &[], &kf, wa, &["bahukara"]);
    assert_has_upapada_krdanta("nAndI", &[], &kf, wa, &["nAndIkara"]);
    assert_has_upapada_krdanta("kim", &[], &kf, wa, &["kiNkara"]);
    assert_has_upapada_krdanta("lipi", &[], &kf, wa, &["lipikara"]);
    assert_has_upapada_krdanta("libi", &[], &kf, wa, &["libikara"]);
    assert_has_upapada_krdanta("bali", &[], &kf, wa, &["balikara"]);
    assert_has_upapada_krdanta("Bakti", &[], &kf, wa, &["Baktikara"]);
    assert_has_upapada_krdanta("kartf", &[], &kf, wa, &["kartfkara"]);
    assert_has_upapada_krdanta("citra", &[], &kf, wa, &["citrakara"]);
    assert_has_upapada_krdanta("kzetra", &[], &kf, wa, &["kzetrakara"]);
    assert_has_upapada_krdanta("eka", &[], &kf, wa, &["ekakara"]);
    assert_has_upapada_krdanta("dvi", &[], &kf, wa, &["dvikara"]);
    assert_has_upapada_krdanta("tri", &[], &kf, wa, &["trikara"]);
    assert_has_upapada_krdanta("jaNGA", &[], &kf, wa, &["jaNGAkara"]);
    assert_has_upapada_krdanta("bAhu", &[], &kf, wa, &["bAhukara"]);
    assert_has_upapada_krdanta("ahan", &[], &kf, wa, &["ahaskara"]);
    assert_has_upapada_krdanta("yad", &[], &kf, wa, &["yatkara"]);
    assert_has_upapada_krdanta("tad", &[], &kf, wa, &["tatkara"]);
    assert_has_upapada_krdanta("Danus", &[], &kf, wa, &["Danuzkara"]);
    assert_has_upapada_krdanta("arus", &[], &kf, wa, &["aruzkara"]);
}

#[test]
fn sutra_3_2_22() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_upapada_krdanta("karman", &[], &kf, Krt::wa, &["karmakara"]);
    assert_has_upapada_krdanta("karman", &[], &kf, Krt::aR, &["karmakAra"]);
}

#[test]
fn sutra_3_2_23() {
    let kf = d("qukf\\Y", Tanadi);
    let assert_apavada = |upapada, result| {
        assert_has_upapada_krdanta(upapada, &[], &kf, Krt::aR, &[result]);
        assert_has_upapada_krdanta(upapada, &[], &kf, Krt::wa, &[]);
    };
    assert_apavada("Sabda", "SabdakAra");
    assert_apavada("Sloka", "SlokakAra");
    assert_apavada("kalaha", "kalahakAra");
    assert_apavada("gATA", "gATAkAra");
    assert_apavada("vEra", "vErakAra");
    assert_apavada("cAwu", "cAwukAra");
    assert_apavada("sUtra", "sUtrakAra");
    assert_apavada("mantra", "mantrakAra");
    assert_apavada("pada", "padakAra");
}

#[test]
fn sutra_3_2_24() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_upapada_krdanta("stamba", &[], &kf, Krt::in_, &["stambakari"]);
    assert_has_upapada_krdanta("Sakft", &[], &kf, Krt::in_, &["Sakftkari"]);
}

#[test]
fn sutra_3_2_24_v1() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_upapada_krdanta("stamba", &[], &kf, Krt::aR, &["stambakAra"]);
    assert_has_upapada_krdanta("Sakft", &[], &kf, Krt::aR, &["SakftkAra"]);
}

#[test]
fn sutra_3_2_25() {
    let hf = d("hf\\Y", Bhvadi);
    assert_has_upapada_krdanta("dfti", &[], &hf, Krt::in_, &["dftihari"]);
    assert_has_upapada_krdanta("nATa", &[], &hf, Krt::in_, &["nATahari"]);
    assert_has_upapada_krdanta("dfti", &[], &hf, Krt::aR, &["dftihAra"]);
    assert_has_upapada_krdanta("nATa", &[], &hf, Krt::aR, &["nATahAra"]);
}

#[test]
fn sutra_3_2_26() {
    let grah = d("graha~^", Kryadi);
    let bhf = d("quBf\\Y", Juhotyadi);
    assert_has_upapada_krdanta("Pala", &[], &grah, Krt::in_, &["Palegrahi"]);
    assert_has_upapada_krdanta("Atman", &[], &bhf, Krt::in_, &["AtmamBari"]);
}

#[test]
fn sutra_3_2_27() {
    let t = Tester::with_chaandasa();
    t.assert_has_upapada_krdanta(
        "brahman",
        &[],
        &d("vanu~\\", Tanadi),
        Krt::in_,
        &["brahmavani"],
    );
    t.assert_has_upapada_krdanta("go", &[], &d("zaRa~", Bhvadi), Krt::in_, &["gozaRi"]);
    t.assert_has_upapada_krdanta("paTin", &[], &d("rakza~", Bhvadi), Krt::in_, &["paTirakzi"]);
    t.assert_has_upapada_krdanta("havis", &[], &d("maTe~", Bhvadi), Krt::in_, &["havirmaTi"]);
}

#[ignore]
#[test]
fn sutra_3_2_28() {
    let ej_nic = nic(&d("ejf~\\", Bhvadi));
    assert_has_upapada_krdanta("aNga", &[], &ej_nic, Krt::KaS, &["aNgamejaya"]);
    assert_has_upapada_krdanta("jana", &[], &ej_nic, Krt::KaS, &["janamejaya"]);
}

#[test]
fn sutra_3_2_29() {
    let dhma = d("DmA\\", Bhvadi);
    let dhe = d("De\\w", Bhvadi);
    assert_has_upapada_krdanta("nAsikA", &[], &dhma, Krt::KaS, &["nAsikanDama"]);
    assert_has_upapada_krdanta("nAsikA", &[], &dhe, Krt::KaS, &["nAsikanDaya"]);
    assert_has_upapada_krdanta("stana", &[], &dhma, Krt::KaS, &[]);
    assert_has_upapada_krdanta("stana", &[], &dhe, Krt::KaS, &["stananDaya"]);
}

#[test]
fn sutra_3_2_30() {
    let dhma = d("DmA\\", Bhvadi);
    let dhe = d("De\\w", Bhvadi);
    assert_has_upapada_krdanta("nAqI", &[], &dhma, Krt::KaS, &["nAqinDama"]);
    assert_has_upapada_krdanta("muzwi", &[], &dhma, Krt::KaS, &["muzwinDama"]);
    assert_has_upapada_krdanta("nAqI", &[], &dhe, Krt::KaS, &["nAqinDaya"]);
    assert_has_upapada_krdanta("muzwi", &[], &dhe, Krt::KaS, &["muzwinDaya"]);
}

#[test]
fn sutra_3_2_31() {
    let ruj = d("ru\\jo~", Tudadi);
    let vah = d("va\\ha~^", Bhvadi);
    assert_has_upapada_krdanta("kUla", &["ud"], &ruj, Krt::KaS, &["kUlamudruja"]);
    assert_has_upapada_krdanta("kUla", &["ud"], &vah, Krt::KaS, &["kUlamudvaha"]);
}

#[test]
fn sutra_3_2_32() {
    let lih = d("li\\ha~^", Adadi);
    assert_has_upapada_krdanta("vaha", &[], &lih, Krt::KaS, &["vahaMliha"]);
    assert_has_upapada_krdanta("aBra", &[], &lih, Krt::KaS, &["aBraMliha"]);
}

#[test]
fn sutra_3_2_33() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_upapada_krdanta("prasTa", &[], &pac, Krt::KaS, &["prasTampaca"]);
    assert_has_upapada_krdanta("droRa", &[], &pac, Krt::KaS, &["droRampaca"]);
    assert_has_upapada_krdanta("KAri", &[], &pac, Krt::KaS, &["KArimpaca"]);
}

#[test]
fn sutra_3_2_34() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_upapada_krdanta("mita", &[], &pac, Krt::KaS, &["mitampaca"]);
    assert_has_upapada_krdanta("naKa", &[], &pac, Krt::KaS, &["naKampaca"]);
}

#[test]
fn sutra_3_2_35() {
    let tud = d("tu\\da~^", Tudadi);
    assert_has_upapada_krdanta("viDu", &[], &tud, Krt::KaS, &["viDuntuda"]);
    assert_has_upapada_krdanta("arus", &[], &tud, Krt::KaS, &["aruntuda"]);
}

#[test]
fn sutra_3_2_36() {
    let dfs = d("df\\Si~r", Bhvadi);
    let tap = &d("ta\\pa~", Bhvadi);
    assert_has_upapada_krdanta("asUrya", &[], &dfs, Krt::KaS, &["asUryampaSya"]);
    assert_has_upapada_krdanta("lalAwa", &[], &tap, Krt::KaS, &["lalAwantapa"]);
}

#[test]
fn sutra_3_2_37() {
    assert_has_upapada_krdanta(
        "ugra",
        &[],
        &d("df\\Si~r", Bhvadi),
        Krt::KaS,
        &["ugrampaSya"],
    );
    assert_has_upapada_krdanta("irA", &[], &d("madI~", Divadi), Krt::KaS, &["irammada"]);
    assert_has_upapada_krdanta("pARi", &[], &d("DmA\\", Bhvadi), Krt::KaS, &["pARinDama"]);
}

#[test]
fn sutra_3_2_38() {
    let vad = d("vada~", Bhvadi);
    assert_has_upapada_krdanta("priya", &[], &vad, Krt::Kac, &["priyaMvada"]);
    assert_has_upapada_krdanta("vaSa", &[], &vad, Krt::Kac, &["vaSaMvada"]);
}

#[test]
fn sutra_3_2_39() {
    let taapi = nic(&d("ta\\pa~", Bhvadi));
    assert_has_upapada_krdanta("dvizat", &[], &taapi, Krt::Kac, &["dvizantapa"]);
    assert_has_upapada_krdanta("para", &[], &taapi, Krt::Kac, &["parantapa"]);

    let tap_cur = &d("tapa~", Curadi);
    assert_has_upapada_krdanta("dvizat", &[], &tap_cur, Krt::Kac, &["dvizantapa"]);
    assert_has_upapada_krdanta("para", &[], &tap_cur, Krt::Kac, &["parantapa"]);
}

#[test]
fn sutra_3_2_40() {
    let yam = &d("ya\\ma~", Bhvadi);
    assert_has_upapada_krdanta("vAc", &[], &yam, Krt::Kac, &["vAcaMyama"]);
}

#[test]
fn sutra_3_2_41() {
    let daari = nic(&d("dF", Bhvadi));
    let sah = &d("zaha~\\", Bhvadi);
    assert_has_upapada_krdanta("pur", &[], &daari, Krt::Kac, &["purandara"]);
    assert_has_upapada_krdanta("sarva", &[], &sah, Krt::Kac, &["sarvaMsaha"]);
}

#[test]
fn sutra_3_2_42() {
    let kaz = &d("kaza~", Bhvadi);
    assert_has_upapada_krdanta("sarva", &[], &kaz, Krt::Kac, &["sarvaNkaza"]);
    assert_has_upapada_krdanta("kUla", &[], &kaz, Krt::Kac, &["kUlaNkaza"]);
    assert_has_upapada_krdanta("aBra", &[], &kaz, Krt::Kac, &["aBraNkaza"]);
    assert_has_upapada_krdanta("karIza", &[], &kaz, Krt::Kac, &["karIzaNkaza"]);
}

#[test]
fn sutra_3_2_43() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_upapada_krdanta("meGa", &[], &kf, Krt::Kac, &["meGaNkara"]);
    assert_has_upapada_krdanta("fti", &[], &kf, Krt::Kac, &["ftiNkara"]);
    assert_has_upapada_krdanta("Baya", &[], &kf, Krt::Kac, &["BayaNkara"]);
}

#[test]
fn sutra_3_2_44() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_upapada_krdanta("kzema", &[], &kf, Krt::Kac, &["kzemaNkara"]);
    assert_has_upapada_krdanta("kzema", &[], &kf, Krt::aR, &["kzemakAra"]);
    assert_has_upapada_krdanta("priya", &[], &kf, Krt::Kac, &["priyaNkara"]);
    assert_has_upapada_krdanta("priya", &[], &kf, Krt::aR, &["priyakAra"]);
    assert_has_upapada_krdanta("madra", &[], &kf, Krt::Kac, &["madraNkara"]);
    assert_has_upapada_krdanta("madra", &[], &kf, Krt::aR, &["madrakAra"]);
    // TODO: strI
}

#[test]
fn sutra_3_2_45() {
    let bhu = d("BU", Bhvadi);
    assert_has_upapada_krdanta("ASita", &[], &bhu, Krt::Kac, &["ASitamBava"]);
}

#[test]
fn sutra_3_2_47() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_upapada_krdanta("suta", &[], &gam, Krt::Kac, &["sutaNgama"]);
}

#[test]
fn sutra_3_2_48() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_upapada_krdanta("anta", &[], &gam, Krt::qa, &["antaga"]);
    assert_has_upapada_krdanta("atyanta", &[], &gam, Krt::qa, &["atyantaga"]);
    assert_has_upapada_krdanta("aDvan", &[], &gam, Krt::qa, &["aDvaga"]);
    assert_has_upapada_krdanta("dUra", &[], &gam, Krt::qa, &["dUraga"]);
    assert_has_upapada_krdanta("pAra", &[], &gam, Krt::qa, &["pAraga"]);
    assert_has_upapada_krdanta("sarva", &[], &gam, Krt::qa, &["sarvaga"]);
    assert_has_upapada_krdanta("ananta", &[], &gam, Krt::qa, &["anantaga"]);
}

#[test]
fn sutra_3_2_49() {
    let han = d("ha\\na~", Adadi);
    assert_has_upapada_krdanta("timi", &[], &han, Krt::qa, &["timiha"]);
    assert_has_upapada_krdanta("Satru", &[], &han, Krt::qa, &["Satruha"]);
    // AziSi?
    assert_has_upapada_krdanta("Satru", &[], &han, Krt::aR, &["SatruGAta"]);
}

#[test]
fn sutra_3_2_50() {
    let han = d("ha\\na~", Adadi);
    assert_has_upapada_krdanta("kleSa", &["apa"], &han, Krt::qa, &["kleSApaha"]);
    assert_has_upapada_krdanta("tamas", &["apa"], &han, Krt::qa, &["tamopaha"]);
}

#[test]
fn sutra_3_2_51() {
    let han = d("ha\\na~", Adadi);
    assert_has_upapada_krdanta("kumAra", &[], &han, Krt::Rini, &["kumAraGAtin"]);
    assert_has_upapada_krdanta("SIrzan", &[], &han, Krt::Rini, &["SIrzaGAtin"]);
}

#[test]
fn sutra_3_2_52() {
    let han = d("ha\\na~", Adadi);
    assert_has_upapada_krdanta("jAyA", &[], &han, Krt::wak, &["jAyAGna"]);
    assert_has_upapada_krdanta("pati", &[], &han, Krt::wak, &["patiGna"]);
    // TODO: stri
}

#[test]
fn sutra_3_2_53() {
    let han = d("ha\\na~", Adadi);
    assert_has_upapada_krdanta("jAyA", &[], &han, Krt::wak, &["jAyAGna"]);
    assert_has_upapada_krdanta("pati", &[], &han, Krt::wak, &["patiGna"]);
    assert_has_upapada_krdanta("Slezma", &[], &han, Krt::wak, &["SlezmaGna"]);
    assert_has_upapada_krdanta("pitta", &[], &han, Krt::wak, &["pittaGna"]);
    // TODO: a-manuzya-kartfke
}

#[test]
fn sutra_3_2_54() {
    let han = d("ha\\na~", Adadi);
    assert_has_upapada_krdanta("hastin", &[], &han, Krt::wak, &["hastiGna"]);
    assert_has_upapada_krdanta("kapAwa", &[], &han, Krt::wak, &["kapAwaGna"]);
    // SaktO?
    assert_has_upapada_krdanta("hastin", &[], &han, Krt::aR, &["hastiGAta"]);
}

#[test]
fn sutra_3_2_55() {
    let han = d("ha\\na~", Adadi);
    assert_has_upapada_krdanta("pARi", &[], &han, Krt::ka, &["pARiGa"]);
    assert_has_upapada_krdanta("tAqa", &[], &han, Krt::ka, &["tAqaGa"]);
    // Silpini?
    assert_has_upapada_krdanta("pARi", &[], &han, Krt::aR, &["pARiGAta"]);
    assert_has_upapada_krdanta("tAqa", &[], &han, Krt::aR, &["tAqaGAta"]);
}

#[test]
fn sutra_3_2_56() {
    let kf = d("qukf\\Y", Bhvadi);
    assert_has_upapada_krdanta("AQya", &[], &kf, Krt::Kyun, &["AQyaNkaraRa"]);
    assert_has_upapada_krdanta("suBaga", &[], &kf, Krt::Kyun, &["suBagaNkaraRa"]);
    assert_has_upapada_krdanta("sTUla", &[], &kf, Krt::Kyun, &["sTUlaNkaraRa"]);
    assert_has_upapada_krdanta("palita", &[], &kf, Krt::Kyun, &["palitaNkaraRa"]);
    assert_has_upapada_krdanta("nagna", &[], &kf, Krt::Kyun, &["nagnaNkaraRa"]);
    assert_has_upapada_krdanta("anDa", &[], &kf, Krt::Kyun, &["anDaNkaraRa"]);
    assert_has_upapada_krdanta("priya", &[], &kf, Krt::Kyun, &["priyaNkaraRa"]);
    // kartari?
}

#[test]
fn sutra_3_2_57() {
    let bhu = d("BU", Bhvadi);
    assert_has_upapada_krdanta("AQya", &[], &bhu, Krt::KizRuc, &["AQyamBavizRu"]);
    assert_has_upapada_krdanta("AQya", &[], &bhu, Krt::KukaY, &["AQyamBAvuka"]);
    assert_has_upapada_krdanta("suBaga", &[], &bhu, Krt::KizRuc, &["suBagamBavizRu"]);
    assert_has_upapada_krdanta("suBaga", &[], &bhu, Krt::KukaY, &["suBagamBAvuka"]);
    assert_has_upapada_krdanta("sTUla", &[], &bhu, Krt::KizRuc, &["sTUlamBavizRu"]);
    assert_has_upapada_krdanta("sTUla", &[], &bhu, Krt::KukaY, &["sTUlamBAvuka"]);
    assert_has_upapada_krdanta("palita", &[], &bhu, Krt::KizRuc, &["palitamBavizRu"]);
    assert_has_upapada_krdanta("palita", &[], &bhu, Krt::KukaY, &["palitamBAvuka"]);
    assert_has_upapada_krdanta("nagna", &[], &bhu, Krt::KizRuc, &["nagnamBavizRu"]);
    assert_has_upapada_krdanta("nagna", &[], &bhu, Krt::KukaY, &["nagnamBAvuka"]);
    assert_has_upapada_krdanta("anDa", &[], &bhu, Krt::KizRuc, &["anDamBavizRu"]);
    assert_has_upapada_krdanta("anDa", &[], &bhu, Krt::KukaY, &["anDamBAvuka"]);
    assert_has_upapada_krdanta("priya", &[], &bhu, Krt::KizRuc, &["priyamBavizRu"]);
    assert_has_upapada_krdanta("priya", &[], &bhu, Krt::KukaY, &["priyamBAvuka"]);
    // kartari?
}

#[test]
fn sutra_3_2_58() {
    let spfs = d("spf\\Sa~", Tudadi);
    assert_has_upapada_krdanta("mantra", &[], &spfs, Krt::kvin, &["mantraspfS"]);
    assert_has_upapada_krdanta("jala", &[], &spfs, Krt::kvin, &["jalaspfS"]);
    assert_has_upapada_krdanta("udaka", &[], &spfs, Krt::kvin, &[]);
    assert_has_upapada_krdanta("udaka", &[], &spfs, Krt::aR, &["udakasparSa"]);
}

#[test]
fn sutra_3_2_59() {
    use Krt::kvin;
    assert_has_upapada_krdanta("ftu", &[], &d("ya\\ja~^", Bhvadi), kvin, &["ftvij"]);
    assert_has_krdanta(&[], &d("YiDfzA~", Svadi), kvin, &["daDfz"]);
    assert_has_krdanta(&[], &d("sf\\ja~", Tudadi), kvin, &["sraj"]);
    assert_has_krdanta(&[], &d("di\\Sa~^", Tudadi), kvin, &["diS"]);
    assert_has_krdanta(&["ud"], &d("zRi\\ha~", Divadi), kvin, &["uzRih"]);

    let anc = d("ancu~", Bhvadi);
    assert_has_upapada_krdanta("pra", &[], &anc, Krt::kvin, &["prAc"]);
    assert_has_upapada_krdanta("prati", &[], &anc, Krt::kvin, &["pratyac"]);
    assert_has_upapada_krdanta("ud", &[], &anc, Krt::kvin, &["udac"]);
    assert_has_krdanta(&[], &d("yu\\ji~^r", Rudhadi), Krt::kvin, &["yuj"]);
    assert_has_krdanta(&[], &d("krunca~", Bhvadi), Krt::kvin, &["kruYc"]);
}

#[ignore]
#[test]
fn sutra_3_2_60() {
    use upapada_krdanta as upa_krt;

    let dfs = d("df\\Si~r", Bhvadi);
    assert_has_sup_1s(&upa_krt("tyad", &[], &dfs, Krt::kaY), Pum, &["tyAdfSaH"]);
    assert_has_sup_1s(&upa_krt("tyad", &[], &dfs, Krt::kvin), Pum, &["tyAdfk"]);
    assert_has_sup_1s(&upa_krt("tad", &[], &dfs, Krt::kaY), Pum, &["tAdfSaH"]);
    assert_has_sup_1s(&upa_krt("tad", &[], &dfs, Krt::kvin), Pum, &["tAdfk"]);
    assert_has_sup_1s(&upa_krt("yad", &[], &dfs, Krt::kaY), Pum, &["yAdfSaH"]);
    assert_has_sup_1s(&upa_krt("yad", &[], &dfs, Krt::kvin), Pum, &["yAdfk"]);

    // an-Alocana?
    assert_has_upapada_krdanta("tad", &[], &dfs, Krt::aR, &["taddarSaH"]);
}

#[test]
fn sutra_3_2_61() {
    let sad = d("za\\dx~", Bhvadi);
    assert_has_upapada_krdanta("Suci", &[], &sad, Krt::kvip, &["Sucizad"]);
    assert_has_upapada_krdanta("antarikza", &[], &sad, Krt::kvip, &["antarikzasad"]);
    assert_has_krdanta(&["upa"], &sad, Krt::kvip, &["upasad"]);

    let su = &d("zUN", Adadi);
    assert_has_krdanta(&["aRqa"], &su, Krt::kvip, &["aRqasU"]);
    assert_has_krdanta(&["Sata"], &su, Krt::kvip, &["SatasU"]);
    assert_has_krdanta(&["pra"], &su, Krt::kvip, &["prasU"]);

    let dvis = &d("dvi\\za~^", Adadi);
    assert_has_krdanta(&["mitra"], &dvis, Krt::kvip, &["mitradviz"]);
    assert_has_krdanta(&["pra"], &dvis, Krt::kvip, &["pradviz"]);

    let druh = &d("dru\\ha~", Divadi);
    assert_has_krdanta(&["mitra"], &druh, Krt::kvip, &["mitradruh"]);
    assert_has_krdanta(&["pra"], &druh, Krt::kvip, &["pradruh"]);

    let duh = &d("du\\ha~^", Divadi);
    assert_has_krdanta(&["go"], &duh, Krt::kvip, &["goduh"]);
    assert_has_krdanta(&["pra"], &duh, Krt::kvip, &["praduh"]);

    let yuj = &d("yu\\ji~^r", Rudhadi);
    assert_has_krdanta(&["aSva"], &yuj, Krt::kvip, &["aSvayuj"]);
    assert_has_krdanta(&["pra"], &yuj, Krt::kvip, &["prayuj"]);

    let vid = d("vida~", Adadi);
    assert_has_krdanta(&["veda"], &vid, Krt::kvip, &["vedavid"]);
    assert_has_krdanta(&["pra"], &vid, Krt::kvip, &["pravid"]);
    assert_has_krdanta(&["brahma"], &vid, Krt::kvip, &["brahmavid"]);

    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_krdanta(&["kAzWa"], &bhid, Krt::kvip, &["kAzWaBid"]);
    assert_has_krdanta(&["pra"], &bhid, Krt::kvip, &["praBid"]);

    let chid = d("Ci\\di~^r", Rudhadi);
    assert_has_krdanta(&["rajju"], &chid, Krt::kvip, &["rajjucCid"]);
    assert_has_krdanta(&["pra"], &chid, Krt::kvip, &["pracCid"]);

    let ji = d("ji\\", Bhvadi);
    assert_has_krdanta(&["Satru"], &ji, Krt::kvip, &["Satrujit"]);
    assert_has_krdanta(&["pra"], &ji, Krt::kvip, &["prajit"]);

    let ni = d("RI\\Y", Bhvadi);
    assert_has_krdanta(&["senA"], &ni, Krt::kvip, &["senAnI"]);
    assert_has_krdanta(&["pra"], &ni, Krt::kvip, &["praRI"]);
    assert_has_krdanta(&["grAma"], &ni, Krt::kvip, &["grAmaRI"]);
    assert_has_krdanta(&["agra"], &ni, Krt::kvip, &["agraRI"]);
}

#[test]
fn sutra_3_2_62() {
    assert_has_krdanta(&["pra"], &d("Ba\\ja~^", Bhvadi), Krt::Rvi, &["praBAj"]);
}

// 3.2.63 is chAndasa.

#[test]
fn sutra_3_2_64() {
    let vah = d("va\\ha~^", Bhvadi);
    assert_has_upapada_krdanta("prazWa", &[], &vah, Krt::Rvi, &["prazWavAh"]);
    assert_has_upapada_krdanta("ditya", &[], &vah, Krt::Rvi, &["dityavAh"]);
}

#[ignore]
#[test]
fn sutra_3_2_65() {
    let t = Tester::with_chaandasa();
    let vah = d("va\\ha~^", Bhvadi);
    t.assert_has_upapada_krdanta("kavya", &[], &vah, Krt::Yyuw, &["kavyavAhana"]);
    t.assert_has_upapada_krdanta("purIza", &[], &vah, Krt::Yyuw, &["purIzavAhaRa"]);
    t.assert_has_upapada_krdanta("purIzya", &[], &vah, Krt::Yyuw, &["purIzyavAhana"]);
}

#[test]
fn sutra_3_2_66() {
    let t = Tester::with_chaandasa();
    let vah = d("va\\ha~^", Bhvadi);
    t.assert_has_upapada_krdanta("havya", &[], &vah, Krt::Yyuw, &["havyavAhana"]);
    t.assert_has_upapada_krdanta("havya", &[], &vah, Krt::Rvi, &["havyavAh"]);
}

#[test]
fn sutra_3_2_68() {
    let ad = d("a\\da~", Adadi);
    assert_has_upapada_krdanta("Ama", &[], &ad, Krt::viw, &["AmAd"]);
    assert_has_upapada_krdanta("sasya", &[], &ad, Krt::viw, &["sasyAd"]);
    assert_has_upapada_krdanta("anna", &[], &ad, Krt::aR, &["annAda"]);
}

#[test]
fn sutra_3_2_69() {
    let ad = d("a\\da~", Adadi);
    assert_has_upapada_krdanta("kravya", &[], &ad, Krt::viw, &["kravyAd"]);
}

#[test]
fn sutra_3_2_70() {
    let duh = d("du\\ha~^", Adadi);
    assert_has_upapada_krdanta("kAma", &[], &duh, Krt::kap, &["kAmaduGa"]);
    assert_has_upapada_krdanta("arDa", &[], &duh, Krt::kap, &["arDaduGa"]);
    assert_has_upapada_krdanta("Darma", &[], &duh, Krt::kap, &["DarmaduGa"]);
    // TODO: stri
}

// 3.2.71 - 3.2.72 are mAntra.
// 3.2.73 - 3.2.75 are chAndasa.

#[ignore]
#[test]
fn sutra_3_2_76() {
    let srans = d("sransu~\\", Bhvadi);
    let dhvans = d("Dvansu~\\", Bhvadi);
    let bhrans = d("BranSu~", Divadi);
    assert_has_upapada_krdanta("uKyA", &[], &srans, Krt::kvip, &["uKyAsras"]);
    assert_has_upapada_krdanta("parRa", &[], &dhvans, Krt::kvip, &["parRaDvas"]);
    assert_has_upapada_krdanta("vAha", &[], &bhrans, Krt::kvip, &["vAhABraS"]);
}

#[ignore]
#[test]
fn sutra_3_2_77() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_upapada_krdanta("Sam", &[], &stha, Krt::ka, &["SaMsTa"]);
    assert_has_upapada_krdanta("Sam", &[], &stha, Krt::kvip, &["SaMsTA"]);
}

#[test]
fn sutra_3_2_78() {
    let bhuj = d("Bu\\ja~", Rudhadi);
    assert_has_upapada_krdanta("uzRa", &[], &bhuj, Krt::Rini, &["uzRaBojin"]);
    assert_has_upapada_krdanta("SIta", &[], &bhuj, Krt::Rini, &["SItaBojin"]);
    // TODO: ajAtau
}

#[test]
fn sutra_3_2_79() {
    assert_has_upapada_krdanta(
        "uzwra",
        &[],
        &d("kru\\Sa~", Bhvadi),
        Krt::Rini,
        &["uzwrakroSin"],
    );
    assert_has_upapada_krdanta(
        "DvANkza",
        &[],
        &d("ru", Adadi),
        Krt::Rini,
        &["DvANkzarAvin"],
    );
}

#[test]
fn sutra_3_2_80() {
    assert_has_upapada_krdanta(
        "sTaRqila",
        &[],
        &d("SIN", Adadi),
        Krt::Rini,
        &["sTaRqilaSAyin"],
    );
    assert_has_upapada_krdanta(
        "aSrAdDa",
        &[],
        &d("Bu\\ja~", Rudhadi),
        Krt::Rini,
        &["aSrAdDaBojin"],
    );
    // TODO: others
}

#[test]
fn sutra_3_2_81() {
    let paa = d("pA\\", Bhvadi);
    assert_has_upapada_krdanta("kazAya", &[], &paa, Krt::Rini, &["kazAyapAyin"]);
    assert_has_upapada_krdanta("kzIra", &[], &paa, Krt::Rini, &["kzIrapAyin"]);
    // TODO: others
}

#[test]
fn sutra_3_2_82() {
    let man = d("ma\\na~\\", Divadi);
    assert_has_upapada_krdanta("darSanIya", &[], &man, Krt::Rini, &["darSanIyamAnin"]);
    assert_has_upapada_krdanta("SoBana", &[], &man, Krt::Rini, &["SoBanamAnin"]);
}

#[test]
fn sutra_3_2_83() {
    let man = d("ma\\na~\\", Divadi);
    assert_has_upapada_krdanta("darSanIya", &[], &man, Krt::KaS, &["darSanIyammanya"]);
    assert_has_upapada_krdanta("paRqita", &[], &man, Krt::KaS, &["paRqitammanya"]);
}

#[test]
fn sutra_3_2_84_and_sutra_3_2_85() {
    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_upapada_krdanta("agnizwoma", &[], &yaj, Krt::Rini, &["agnizwomayAjin"]);
}

#[test]
fn sutra_3_2_86() {
    let han = d("ha\\na~", Adadi);
    assert_has_upapada_krdanta("pitfvya", &[], &han, Krt::Rini, &["pitfvyaGAtin"]);
    assert_has_upapada_krdanta("mAtula", &[], &han, Krt::Rini, &["mAtulaGAtin"]);
    // TODO: kutsita-grahanam
}

#[ignore]
#[test]
fn sutra_3_2_87() {
    let han = d("ha\\na~", Adadi);
    assert_has_upapada_krdanta("brahman", &[], &han, Krt::kvip, &["brahmahan"]);
    assert_has_upapada_krdanta("BrUna", &[], &han, Krt::kvip, &["BrUnahan"]);
    assert_has_upapada_krdanta("vftra", &[], &han, Krt::kvip, &["vftrahan"]);
}

// 3.2.88 is chAndasa.

#[test]
fn sutra_3_2_89() {
    use Krt::kvip;
    let kf = d("qukf\\Y", Tanadi);
    assert_has_upapada_krdanta("su", &[], &kf, kvip, &["sukft"]);
    assert_has_upapada_krdanta("karman", &[], &kf, kvip, &["karmakft"]);
    assert_has_upapada_krdanta("pApa", &[], &kf, kvip, &["pApakft"]);
    assert_has_upapada_krdanta("mantra", &[], &kf, kvip, &["mantrakft"]);
    assert_has_upapada_krdanta("puRya", &[], &kf, kvip, &["puRyakft"]);
}

#[test]
fn sutra_3_2_90() {
    let su = d("zu\\Y", Svadi);
    assert_has_upapada_krdanta("soma", &[], &su, Krt::kvip, &["somasut"]);
}

#[test]
fn sutra_3_2_91() {
    let ci = d("ci\\Y", Svadi);
    assert_has_upapada_krdanta("agni", &[], &ci, Krt::kvip, &["agnicit"]);
}

#[test]
fn sutra_3_2_92() {
    let ci = d("ci\\Y", Svadi);
    assert_has_upapada_krdanta("Syena", &[], &ci, Krt::kvip, &["Syenacit"]);
    assert_has_upapada_krdanta("kanka", &[], &ci, Krt::kvip, &["kaNkacit"]);
}

#[test]
fn sutra_3_2_93() {
    let kri = d("qukrI\\Y", Kryadi);
    assert_has_upapada_krdanta("soma", &["vi"], &kri, Krt::ini, &["somavikrayin"]);
    assert_has_upapada_krdanta("rasa", &["vi"], &kri, Krt::ini, &["rasavikrayin"]);
}

#[test]
fn sutra_3_2_93_k() {
    let kri = d("qukrI\\Y", Kryadi);
    assert_has_upapada_krdanta("DAnya", &["vi"], &kri, Krt::ini, &[]);
}

#[test]
fn sutra_3_2_94() {
    let dfs = d("df\\Si~r", Bhvadi);
    assert_has_upapada_krdanta("pAra", &[], &dfs, Krt::kvanip, &["pAradfSvan"]);
}

#[test]
fn sutra_3_2_95() {
    let kf = d("qukf\\Y", Tanadi);
    let yudh = d("yu\\Da~\\", Divadi);
    assert_has_upapada_krdanta("rAjan", &[], &kf, Krt::kvanip, &["rAjakftvan"]);
    assert_has_upapada_krdanta("rAjan", &[], &yudh, Krt::kvanip, &["rAjayuDvan"]);
}

#[test]
fn sutra_3_2_96() {
    let kf = d("qukf\\Y", Tanadi);
    let yudh = d("yu\\Da~\\", Divadi);
    assert_has_upapada_krdanta("saha", &[], &kf, Krt::kvanip, &["sahakftvan"]);
    assert_has_upapada_krdanta("saha", &[], &yudh, Krt::kvanip, &["sahayuDvan"]);
}

#[test]
fn sutra_3_2_97() {
    let jan = d("janI~\\", Divadi);
    assert_has_upapada_krdanta("upasara", &[], &jan, Krt::qa, &["upasaraja"]);
    assert_has_upapada_krdanta("mandura", &[], &jan, Krt::qa, &["manduraja"]);
}

#[test]
fn sutra_3_2_98() {
    let jan = d("janI~\\", Divadi);
    assert_has_upapada_krdanta("budDi", &[], &jan, Krt::qa, &["budDija"]);
    assert_has_upapada_krdanta("saMskAra", &[], &jan, Krt::qa, &["saMskAraja"]);
    assert_has_upapada_krdanta("duHKa", &[], &jan, Krt::qa, &["duHKaja"]);

    // TODO: a-jAtau
}

#[test]
fn sutra_3_2_99() {
    let jan = d("janI~\\", Divadi);
    assert_has_upapada_krdanta("pra", &[], &jan, Krt::qa, &["praja"]);
}

#[test]
fn sutra_3_2_100() {
    let jan = d("janI~\\", Divadi);
    assert_has_upapada_krdanta("pums", &["anu"], &jan, Krt::qa, &["pumanuja"]);
    assert_has_upapada_krdanta("strI", &["anu"], &jan, Krt::qa, &["stryanuja"]);
}

// 3.2.101 is miscellaneous

#[test]
fn sutra_3_2_102() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::kta, &["kfta"]);
    assert_has_krdanta(&[], &kf, Krt::ktavatu, &["kftavat"]);
    let bhuj = d("Bu\\ja~", Rudhadi);
    assert_has_krdanta(&[], &bhuj, Krt::kta, &["Bukta"]);
    assert_has_krdanta(&[], &bhuj, Krt::ktavatu, &["Buktavat"]);
}

#[test]
fn sutra_3_2_103() {
    assert_has_krdanta(&[], &d("zu\\Y", Svadi), Krt::Nvanip, &["sutvan"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::Nvanip, &["yajvan"]);
}

#[test]
fn sutra_3_2_104() {
    let jf = d("jFz", Divadi);
    assert_has_krdanta(&[], &jf, Krt::atfn, &["jarat"]);
    assert_has_krdanta(&[], &jf, Krt::kta, &["jIrRa"]);
    assert_has_krdanta(&[], &jf, Krt::ktavatu, &["jIrRavat"]);
}

#[test]
fn sutra_3_2_105() {
    assert_has_mip(&[], &d("df\\Si~r", Bhvadi), Lit, &["dadarSa"]);
    assert_has_mip(&["AN"], &d("tanu~^", Tanadi), Lit, &["AtatAna", "Atatana"]);
}

#[test]
fn sutra_3_2_106() {
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::kAnac, &["cikyAna", "cicyAna"]);
    assert_has_krdanta(&[], &d("zu\\Y", Svadi), Krt::kAnac, &["suzuvARa"]);
}

#[test]
fn sutra_3_2_107() {
    assert_has_krdanta(
        &[],
        &d("a\\da~", Adadi),
        Krt::kvasu,
        &["jakzivas", "Adivas"],
    );
    assert_has_krdanta(&[], &d("pA\\", Bhvadi), Krt::kvasu, &["papivas"]);
}

#[test]
fn sutra_3_2_108() {
    let sad = d("za\\dx~", Bhvadi);
    assert_has_krdanta(&["upa"], &sad, Krt::kvasu, &["upasedivas"]);
    assert_has_lun(&["upa"], &sad, &["upAsadat"]);
    assert_has_lan(&["upa"], &sad, &["upAsIdat"]);
    assert_has_lit(&["upa"], &sad, &["upasasAda"]);

    let vas = d("va\\sa~", Bhvadi);
    assert_has_krdanta(&["anu"], &vas, Krt::kvasu, &["anUzivas"]);
    assert_has_lun(&["anu"], &vas, &["anvavAtsIt"]);
    assert_has_lan(&["anu"], &vas, &["anvavasat"]);
    assert_has_lit(&["anu"], &vas, &["anUvAsa"]);

    let shru = d("Sru\\", Bhvadi);
    assert_has_krdanta(&["upa"], &shru, Krt::kvasu, &["upaSuSruvas"]);
    assert_has_lun(&["upa"], &shru, &["upASrOzIt"]);
    assert_has_lan(&["upa"], &shru, &["upASfRot"]);
    assert_has_lit(&["upa"], &shru, &["upaSuSrAva"]);
}

#[test]
fn sutra_3_2_110() {
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lun, &["akArzIt"]);
    assert_has_tip(&[], &d("hf\\Y", Bhvadi), Lun, &["ahArzIt"]);
}

#[test]
fn sutra_3_2_111() {
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lan, &["akarot"]);
    assert_has_tip(&[], &d("hf\\Y", Bhvadi), Lan, &["aharat"]);
}

#[test]
fn sutra_3_2_115() {
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lit, &["cakAra"]);
    assert_has_tip(&[], &d("hf\\Y", Bhvadi), Lit, &["jahAra"]);
}

#[test]
fn sutra_3_2_123() {
    assert_has_tip(&[], &d("qupa\\ca~^z", Bhvadi), Lat, &["pacati"]);
    assert_has_tip(&[], &d("paWa~", Bhvadi), Lat, &["paWati"]);
}

#[test]
fn sutra_3_2_124() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::Satf, &["pacat"]);
    assert_has_krdanta(&[], &pac, Krt::SAnac, &["pacamAna"]);
    // others
    assert_has_krdanta(&[], &d("asa~", Adadi), Krt::Satf, &["sat"]);
}

#[test]
fn sutra_3_2_125() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::Satf, &["pacat"]);
    assert_has_krdanta(&[], &pac, Krt::SAnac, &["pacamAna"]);
}

#[test]
fn sutra_3_2_127() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kr, Krt::Satf, &["kurvat"]);
    assert_has_krdanta(&[], &kr, Krt::SAnac, &["kurvARa"]);
    assert_has_lrt_sat(&kr, Krt::Satf, &["karizyat"]);
    assert_has_lrt_sat(&kr, Krt::SAnac, &["karizyamARa"]);
}

#[test]
fn sutra_3_2_128() {
    assert_has_krdanta(&[], &d("pUN", Bhvadi), Krt::SAnan, &["pavamAna"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::SAnan, &["yajamAna"]);
}

#[ignore]
#[test]
fn sutra_3_2_130() {
    assert_has_krdanta(&["aDi"], &d("i\\N", Adadi), Krt::Satf, &["aDIyat"]);
    assert_has_krdanta(&[], &d("Df\\Y", Bhvadi), Krt::Satf, &["DArayat"]);
}

#[test]
fn sutra_3_2_131() {
    let dvishat = krdanta(&[], &d("dvi\\za~^", Adadi), Krt::Satf);
    assert_has_sup_1s(&dvishat, Pum, &["dvizan"]);
    assert_has_sup_1d(&dvishat, Pum, &["dvizantO"]);
    assert_has_sup_1p(&dvishat, Pum, &["dvizantaH"]);
}

#[test]
fn sutra_3_2_132() {
    let su = d("zu\\Y", Svadi);
    assert_has_krdanta(&[], &su, Krt::Satf, &["sunvat"]);
}

#[test]
fn sutra_3_2_133() {
    let arh = d("arha~", Bhvadi);
    assert_has_krdanta(&[], &arh, Krt::Satf, &["arhat"]);
}

#[test]
fn sutra_3_2_135() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::tfn, &["kartf"]);
    assert_has_krdanta(&[], &d("vada~", Bhvadi), Krt::tfn, &["vaditf"]);
}

#[test]
fn sutra_3_2_136() {
    assert_has_krdanta(
        &["alam"],
        &d("qukf\\Y", Tanadi),
        Krt::izRuc,
        &["alaNkarizRu"],
    );
    assert_has_krdanta(
        &["nis", "AN"],
        &d("qukf\\Y", Tanadi),
        Krt::izRuc,
        &["nirAkarizRu"],
    );
    assert_has_krdanta(&["pra"], &d("janI~\\", Divadi), Krt::izRuc, &["prajanizRu"]);
    assert_has_krdanta(
        &["ud"],
        &d("qupa\\ca~^z", Bhvadi),
        Krt::izRuc,
        &["utpacizRu"],
    );
    assert_has_krdanta(&["ud"], &d("patx~", Bhvadi), Krt::izRuc, &["utpatizRu"]);
    assert_has_krdanta(&["ud"], &d("madI~", Divadi), Krt::izRuc, &["unmadizRu"]);
    assert_has_krdanta(&[], &d("ruca~\\", Bhvadi), Krt::izRuc, &["rocizRu"]);
    assert_has_krdanta(
        &["apa"],
        &d("trapU~\\z", Bhvadi),
        Krt::izRuc,
        &["apatrapizRu"],
    );
    assert_has_krdanta(&[], &d("vftu~\\", Bhvadi), Krt::izRuc, &["vartizRu"]);
    assert_has_krdanta(&[], &d("vfDu~\\", Bhvadi), Krt::izRuc, &["varDizRu"]);
    assert_has_krdanta(&[], &d("zaha~\\", Bhvadi), Krt::izRuc, &["sahizRu"]);
    assert_has_krdanta(&[], &d("cara~", Bhvadi), Krt::izRuc, &["carizRu"]);
}

#[test]
fn sutra_3_2_140() {
    assert_has_krdanta(&[], &d("trasI~", Divadi), Krt::knu, &["trasnu"]);
    assert_has_krdanta(&[], &d("gfDu~", Divadi), Krt::knu, &["gfDnu"]);
    assert_has_krdanta(&[], &d("YiDfzA~", Svadi), Krt::knu, &["DfzRu"]);
    assert_has_krdanta(&[], &d("kzi\\pa~^", Tudadi), Krt::knu, &["kzipRu"]);
}

#[test]
fn sutra_3_2_141() {
    assert_has_krdanta(&[], &d("Samu~", Divadi), Krt::GinuR, &["Samin"]);
    assert_has_krdanta(&[], &d("tamu~", Divadi), Krt::GinuR, &["tamin"]);
    assert_has_krdanta(&[], &d("damu~", Divadi), Krt::GinuR, &["damin"]);
    assert_has_krdanta(&[], &d("Sramu~", Divadi), Krt::GinuR, &["Sramin"]);
    assert_has_krdanta(&[], &d("Bramu~", Divadi), Krt::GinuR, &["Bramin"]);
    assert_has_krdanta(&[], &d("kzamU~", Divadi), Krt::GinuR, &["kzamin"]);
    assert_has_krdanta(&[], &d("klamu~", Divadi), Krt::GinuR, &["klamin"]);
    assert_has_krdanta(&["pra"], &d("madI~", Divadi), Krt::GinuR, &["pramAdin"]);
    // TODO: unmAdI (conflicts with unmadizRu)
    // assert_has_krdanta(&["ud"], &d("madI~", Divadi), Krt::GinuR, &["unmAdin"]);
    // azwAByaH
    assert_has_krdanta(&[], &d("asu~", Divadi), Krt::GinuR, &[]);
    assert_has_krdanta(&[], &d("asu~", Divadi), Krt::tfn, &["asitf"]);
}

#[test]
fn sutra_3_2_143() {
    assert_has_krdanta(&["vi"], &d("kaza~", Bhvadi), Krt::GinuR, &["vikAzin"]);
    assert_has_krdanta(&["vi"], &d("lasa~", Bhvadi), Krt::GinuR, &["vilAsin"]);
    assert_has_krdanta(&["vi"], &d("katTa~\\", Bhvadi), Krt::GinuR, &["vikatTin"]);
    assert_has_krdanta(&["vi"], &d("sranBu~\\", Bhvadi), Krt::GinuR, &["visramBin"]);
}

#[test]
fn sutra_3_2_144() {
    assert_has_krdanta(&["apa"], &d("laza~^", Bhvadi), Krt::GinuR, &["apalAzin"]);
    assert_has_krdanta(&["vi"], &d("laza~^", Bhvadi), Krt::GinuR, &["vilAzin"]);
}

#[test]
fn sutra_3_2_145() {
    use Krt::GinuR;
    assert_has_krdanta(&["pra"], &d("lapa~", Bhvadi), GinuR, &["pralApin"]);
    assert_has_krdanta(&["pra"], &d("sf\\", Bhvadi), GinuR, &["prasArin"]);
    assert_has_krdanta(&["pra"], &d("dru\\", Bhvadi), GinuR, &["pradrAvin"]);
    assert_has_krdanta(&["pra"], &d("maTe~", Bhvadi), GinuR, &["pramATin"]);
    assert_has_krdanta(&["pra"], &d("vada~", Bhvadi), GinuR, &["pravAdin"]);
    assert_has_krdanta(&["pra"], &d("va\\sa~", Bhvadi), GinuR, &["pravAsin"]);
    // vas-bhvAdi only
    assert_has_krdanta(&["pra"], &d("vasa~\\", Adadi), GinuR, &[]);
}

#[test]
fn sutra_3_2_146() {
    use Krt::vuY;
    assert_has_krdanta(&[], &d("Ridi~", Bhvadi), vuY, &["nindaka"]);
    assert_has_krdanta(&[], &d("hisi~", Rudhadi), vuY, &["hiMsaka"]);
    assert_has_krdanta(&[], &d("kliSU~", Kryadi), vuY, &["kleSaka"]);
    assert_has_krdanta(&[], &d("KAdf~", Bhvadi), vuY, &["KAdaka"]);
    assert_has_krdanta(&["vi"], &nic(&d("Ra\\Sa~", Divadi)), vuY, &["vinASaka"]);
    assert_has_krdanta(&["pari"], &d("kzi\\pa~", Divadi), vuY, &["parikzepaka"]);
    assert_has_krdanta(&["pari"], &d("rawa~", Bhvadi), vuY, &["parirAwaka"]);
    assert_has_krdanta(&["pari"], &nic(&d("vada~", Bhvadi)), vuY, &["parivAdaka"]);
    assert_has_krdanta(&["vi", "AN"], &d("BAza~\\", Bhvadi), vuY, &["vyABAzaka"]);
    assert_has_krdanta(&[], &d("asU", Kandvadi), vuY, &["asUyaka"]);
}

#[test]
fn sutra_3_2_147() {
    use Krt::vuY;
    let dev = &d("divu~", Curadi);
    let krush = &d("kru\\Sa~", Bhvadi);
    assert_has_krdanta(&["AN"], &dev, vuY, &["Adevaka"]);
    assert_has_krdanta(&["pari"], &dev, vuY, &["paridevaka"]);
    assert_has_krdanta(&["AN"], &krush, vuY, &["AkroSaka"]);
    assert_has_krdanta(&["pari"], &krush, vuY, &["parikroSaka"]);

    // upasarge?
    // devitf allowed by Kaumudi 2560
    assert_has_krdanta(&[], &dev, vuY, &[]);
    assert_has_krdanta(&[], &dev, Krt::tfn, &["devayitf", "devitf"]);
    assert_has_krdanta(&[], &krush, vuY, &[]);
    assert_has_krdanta(&[], &krush, Krt::tfn, &["krozwf"]);
}

#[test]
fn sutra_3_2_148() {
    use Krt::yuc;
    assert_has_krdanta(&[], &d("cala~", Bhvadi), yuc, &["calana"]);
    assert_has_krdanta(&[], &d("cupa~", Bhvadi), yuc, &["copana"]);
    assert_has_krdanta(&[], &d("Sabda~", Curadi), yuc, &["Sabdana"]);
    assert_has_krdanta(&[], &d("ru", Adadi), yuc, &["ravaRa"]);

    // akarmakAt?
    assert_has_krdanta(&[], &d("paWa~", Bhvadi), yuc, &[]);
    assert_has_krdanta(&[], &d("paWa~", Bhvadi), Krt::tfn, &["paWitf"]);
}

#[test]
fn sutra_3_2_154() {
    use Krt::ukaY;
    assert_has_krdanta(&["apa"], &d("laza~^", Bhvadi), ukaY, &["apalAzuka"]);
    assert_has_krdanta(&["pra"], &d("patx~", Bhvadi), ukaY, &["prapAtuka"]);
    assert_has_krdanta(&["upa"], &d("pa\\da~\\", Divadi), ukaY, &["upapAduka"]);
    assert_has_krdanta(&["upa"], &d("zWA\\", Bhvadi), ukaY, &["upasTAyuka"]);
    assert_has_krdanta(&["pra"], &d("BU", Bhvadi), ukaY, &["praBAvuka"]);
    assert_has_krdanta(&["pra"], &d("vfzu~", Bhvadi), ukaY, &["pravarzuka"]);
    assert_has_krdanta(&["AN"], &d("ha\\na~", Adadi), ukaY, &["AGAtuka"]);
    assert_has_krdanta(&[], &d("kamu~\\", Bhvadi), ukaY, &["kAmuka"]);
    assert_has_krdanta(&["AN"], &d("ga\\mx~", Bhvadi), ukaY, &["AgAmuka"]);
    assert_has_krdanta(&["kim"], &d("SF", Kryadi), ukaY, &["kiMSAruka"]);
}

#[test]
fn sutra_3_2_155() {
    use Krt::zAkan;
    assert_has_krdanta(&[], &d("jalpa~", Bhvadi), zAkan, &["jalpAka"]);
    assert_has_krdanta(&[], &d("Bikza~\\", Bhvadi), zAkan, &["BikzAka"]);
    assert_has_krdanta(&[], &d("kuwwa~", Curadi), zAkan, &["kuwwAka"]);
    assert_has_krdanta(&[], &d("lunwa~", Curadi), zAkan, &["luRwAka"]);
    assert_has_krdanta(&[], &d("vfN", Kryadi), zAkan, &["varAka"]);

    // stri
    let varaka = krdanta(&[], &d("vfN", Kryadi), zAkan);
    assert_has_sup_1s(&varaka, Stri, &["varAkI"]);
}

#[test]
fn sutra_3_2_156() {
    let prajavin = krdanta(&["pra"], &d("ju", Bhvadi), Krt::ini);
    assert_has_sup_1s(&prajavin, Pum, &["prajavI"]);
    assert_has_sup_1d(&prajavin, Pum, &["prajavinO"]);
}

#[test]
fn sutra_3_2_160() {
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), Krt::kmarac, &["sfmara"]);
    assert_has_krdanta(&[], &d("Gasx~", Bhvadi), Krt::kmarac, &["Gasmara"]);
    assert_has_krdanta(&[], &d("a\\da~", Adadi), Krt::kmarac, &["admara"]);
}

#[test]
fn sutra_3_2_161() {
    assert_has_krdanta(&[], &d("Ba\\njo~", Rudhadi), Krt::Gurac, &["BaNgura"]);
    assert_has_krdanta(&[], &d("BAsf~\\", Bhvadi), Krt::Gurac, &["BAsura"]);
    assert_has_krdanta(&[], &d("YimidA~", Divadi), Krt::Gurac, &["medura"]);
}

#[test]
fn sutra_3_2_162() {
    assert_has_krdanta(&[], &d("vida~", Adadi), Krt::kurac, &["vidura"]);
    assert_has_krdanta(&[], &d("Bi\\di~^r", Rudhadi), Krt::kurac, &["Bidura"]);
    assert_has_krdanta(&[], &d("Ci\\di~^r", Rudhadi), Krt::kurac, &["Cidura"]);
}

#[test]
fn sutra_3_2_163() {
    use Krt::kvarap;
    assert_has_krdanta(&[], &d("i\\R", Adadi), kvarap, &["itvara"]);
    assert_has_krdanta(&[], &d("Ra\\Sa~", Divadi), kvarap, &["naSvara"]);
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), kvarap, &["jitvara"]);
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), kvarap, &["sftvara"]);
}

#[test]
fn sutra_3_2_164() {
    use Krt::kvarap;
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), kvarap, &["gatvara"]);
}

#[test]
fn sutra_3_2_165() {
    assert_has_krdanta(&[], &d("jAgf", Adadi), Krt::Uka, &["jAgarUka"]);
}

#[test]
fn sutra_3_2_166() {
    assert_has_krdanta(&[], &yan(&d("ya\\ja~", Bhvadi)), Krt::Uka, &["yAyajUka"]);
    assert_has_krdanta(&[], &yan(&d("japa~", Bhvadi)), Krt::Uka, &["jaYjapUka"]);
    assert_has_krdanta(&[], &yan(&d("da\\nSa~", Bhvadi)), Krt::Uka, &["dandaSUka"]);
}

#[ignore]
#[test]
fn sutra_3_2_167() {
    assert_has_krdanta(&[], &d("Ra\\ma~", Bhvadi), Krt::ra, &["namra"]);
    assert_has_krdanta(&[], &d("kapi~\\", Bhvadi), Krt::ra, &["kampra"]);
    assert_has_krdanta(&[], &d("zmi\\N", Bhvadi), Krt::ra, &["smera"]);
    // TODO: ajasra, hiMsra
    assert_has_krdanta(&[], &d("kamu~\\", Bhvadi), Krt::ra, &["kamra"]);
    assert_has_krdanta(&[], &d("dIpI~\\", Divadi), Krt::ra, &["dIpra"]);
}

#[test]
fn sutra_3_2_168() {
    assert_has_krdanta(&[], &san(&d("qukf\\Y", Tanadi)), Krt::u, &["cikIrzu"]);
    assert_has_krdanta(&[], &san(&d("hf\\Y", Bhvadi)), Krt::u, &["jihIrzu"]);
    assert_has_krdanta(&["AN"], &d("Sasi~\\", Bhvadi), Krt::u, &["ASaMsu"]);
    assert_has_krdanta(&[], &d("Bikza~\\", Bhvadi), Krt::u, &["Bikzu"]);
}

#[ignore]
#[test]
fn sutra_3_2_173() {
    assert_has_krdanta(&[], &d("SF", Kryadi), Krt::Aru, &["SarAru"]);
    assert_has_krdanta(&[], &d("vadi~\\", Bhvadi), Krt::Aru, &["vandAru"]);
}

#[test]
fn sutra_3_2_174() {
    let bhi = d("YiBI\\", Juhotyadi);
    assert_has_krdanta(&[], &bhi, Krt::kru, &["BIru"]);
    assert_has_krdanta(&[], &bhi, Krt::klukan, &["BIluka"]);
}

#[test]
fn sutra_3_2_174_v1() {
    let bhi = d("YiBI\\", Juhotyadi);
    assert_has_krdanta(&[], &bhi, Krt::krukan, &["BIruka"]);
}

#[ignore]
#[test]
fn sutra_3_2_175() {
    assert_has_krdanta(&[], &d("zWA\\", Bhvadi), Krt::varac, &["sTAvara"]);
    assert_has_krdanta(&[], &d("ISa~\\", Adadi), Krt::varac, &["ISvara"]);
    assert_has_krdanta(&[], &d("BAsf~\\", Bhvadi), Krt::varac, &["BAsvara"]);
    assert_has_krdanta(&[], &d("pisf~", Bhvadi), Krt::varac, &["pesvara"]);
    assert_has_krdanta(&[], &d("kasa~", Bhvadi), Krt::varac, &["kasvara"]);
}

#[test]
fn sutra_3_2_176() {
    assert_has_krdanta(&[], &yan(&d("yA\\", Adadi)), Krt::varac, &["yAyAvara"]);
}

#[ignore]
#[test]
fn sutra_3_2_177() {
    assert_has_krdanta(&[], &d("BrAjf~\\", Bhvadi), Krt::kvip, &["BrAj"]);
    assert_has_krdanta(&[], &d("BAsf~\\", Bhvadi), Krt::kvip, &["BAs"]);
    assert_has_krdanta(&[], &d("DurvI~", Bhvadi), Krt::kvip, &["Dur"]);
    assert_has_krdanta(&["vi"], &d("dyuta~\\", Bhvadi), Krt::kvip, &["vidyut"]);
    assert_has_krdanta(&[], &d("urja~", Adadi), Krt::kvip, &["Urj"]);
    assert_has_krdanta(&[], &d("pF", Kryadi), Krt::kvip, &["pU"]);
    assert_has_krdanta(&[], &d("ju", Bhvadi), Krt::kvip, &["jU"]);
    assert_has_krdanta(&["grAva"], &d("zwu\\Y", Adadi), Krt::kvip, &["grAvastut"]);
}

#[test]
fn sutra_3_2_180() {
    let bhu = d("BU", Bhvadi);
    assert_has_krdanta(&["vi"], &bhu, Krt::qu, &["viBu"]);
    assert_has_krdanta(&["pra"], &bhu, Krt::qu, &["praBu"]);
    assert_has_krdanta(&["sam"], &bhu, Krt::qu, &["samBu"]);
}

#[test]
fn sutra_3_2_181() {
    assert_has_krdanta(&[], &d("De\\w", Bhvadi), Krt::zwran, &["DAtra"]);
    // TODO: stri
}

#[test]
fn sutra_3_2_182() {
    use Krt::zwran;
    assert_has_krdanta(&[], &d("dA\\p", Bhvadi), zwran, &["dAtra"]);
    assert_has_krdanta(&[], &d("RI\\Y", Bhvadi), zwran, &["netra"]);
    assert_has_krdanta(&[], &d("Sasu~", Bhvadi), zwran, &["Sastra"]);
    assert_has_krdanta(&[], &d("yu", Adadi), zwran, &["yotra"]);
    assert_has_krdanta(&[], &d("yu\\ji~^r", Rudhadi), zwran, &["yoktra"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), zwran, &["stotra"]);
    assert_has_krdanta(&[], &d("tu\\da~^", Tudadi), zwran, &["tottra"]);
    assert_has_krdanta(&[], &d("zi\\Y", Svadi), zwran, &["setra"]);
    assert_has_krdanta(&[], &d("zi\\ca~^", Tudadi), zwran, &["sektra"]);
    assert_has_krdanta(&[], &d("mi\\ha~", Bhvadi), zwran, &["meQra"]);
    assert_has_krdanta(&[], &d("patx~", Bhvadi), zwran, &["pattra"]);
    assert_has_krdanta(&[], &d("da\\nSa~", Bhvadi), zwran, &["daMzwra"]);
    assert_has_krdanta(&[], &d("Ra\\ha~^", Divadi), zwran, &["nadDra"]);
}

#[test]
fn sutra_3_2_183() {
    assert_has_krdanta(&[], &d("pUN", Bhvadi), Krt::zwran, &["potra"]);
    assert_has_krdanta(&[], &d("pUY", Kryadi), Krt::zwran, &["potra"]);
}

#[test]
fn sutra_3_2_184() {
    use Krt::itra;
    assert_has_krdanta(&[], &d("f\\", Bhvadi), itra, &["aritra"]);
    assert_has_krdanta(&[], &d("lUY", Kryadi), itra, &["lavitra"]);
    assert_has_krdanta(&[], &d("DUY", Svadi), itra, &["Davitra"]);
    assert_has_krdanta(&[], &d("zUN", Adadi), itra, &["savitra"]);
    assert_has_krdanta(&[], &d("Kanu~^", Bhvadi), itra, &["Kanitra"]);
    assert_has_krdanta(&[], &d("zaha~\\", Bhvadi), itra, &["sahitra"]);
    assert_has_krdanta(&[], &d("cara~", Bhvadi), itra, &["caritra"]);
}

#[test]
fn sutra_3_2_185() {
    assert_has_krdanta(&[], &d("pUN", Bhvadi), Krt::itra, &["pavitra"]);
    assert_has_krdanta(&[], &d("pUY", Kryadi), Krt::itra, &["pavitra"]);
}

#[test]
fn sutra_3_2_187() {
    assert_has_krdanta(&[], &d("YimidA~\\", Bhvadi), Krt::kta, &["minna"]);
    assert_has_krdanta(&[], &d("YikzvidA~", Divadi), Krt::kta, &["kzviRRa"]);
    assert_has_krdanta(&[], &d("YiDfzA~", Svadi), Krt::kta, &["Dfzwa"]);
}
