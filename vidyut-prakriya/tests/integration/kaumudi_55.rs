extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::Vyakarana;

#[test]
fn skip_sk_2629() {}

#[test]
fn sk_2630() {
    let bhu = d("BU", Bhvadi);
    assert_has_ta(&[], &yan(&bhu), Lat, &["boBUyate"]);
    assert_has_ta(
        &[],
        &yan(&bhu),
        Lit,
        &["boBUyAYcakre", "boBUyAmAsa", "boBUyAmbaBUva"],
    );
    assert_has_ta(&[], &yan(&bhu), Lun, &["aboBUyizwa"]);

    let ruc = d("ruca~\\", Bhvadi);
    assert_has_ta(&[], &yan(&ruc), Lat, &["rorucyate"]);

    let shubh = d("SuBa~\\", Bhvadi);
    assert_has_ta(&[], &yan(&shubh), Lat, &["SoSuByate"]);

    let suca = d("sUca", Curadi);
    assert_has_ta(&[], &yan(&suca), Lat, &["sosUcyate"]);

    let sutra = d("sUtra", Curadi);
    assert_has_ta(&[], &yan(&sutra), Lat, &["sosUtryate"]);

    let mutra = d("mUtra", Curadi);
    assert_has_ta(&[], &yan(&mutra), Lat, &["momUtryate"]);
}

#[test]
fn sk_2631() {
    let suc = &d("sUca", Curadi);
    assert_has_ta(
        &[],
        &yan(&suc),
        Lit,
        &["sosUcAYcakre", "sosUcAmbaBUva", "sosUcAmAsa"],
    );
    assert_has_ta(&[], &yan(&suc), Lut, &["sosUcitA"]);

    let sutra_yan = d("sUtra", Curadi);
    assert_has_ta(&[], &yan(&sutra_yan), Lut, &["sosUtritA"]);

    let mutra = d("mUtra", Curadi);
    assert_has_ta(&[], &yan(&mutra), Lut, &["momUtritA"]);
}

#[test]
fn sk_2632() {
    let at = d("awa~", Bhvadi);
    assert_has_ta(&[], &yan(&at), Lat, &["awAwyate"]);
}

#[test]
fn sk_2633() {
    let r = d("f\\", Juhotyadi);
    assert_has_ta(&[], &yan(&r), Lat, &["arAryate"]);
    assert_has_ta(&[], &yan(&r), Lut, &["arAritA"]);

    let ash = d("aSU~\\", Svadi);
    assert_has_ta(&[], &yan(&ash), Lut, &["aSASitA"]);

    let urnu = d("UrRuY", Adadi);
    assert_has_ta(&[], &yan(&urnu), Lat, &["UrRonUyate"]);

    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_ta(&[], &yan(&bhid), Lat, &["beBidyate"]);
    assert_has_ta(&[], &yan(&bhid), Lut, &["beBiditA"]);
}

#[test]
fn sk_2634() {
    let vraj = d("vraja~", Bhvadi);
    assert_has_ta(&[], &yan(&vraj), Lat, &["vAvrajyate"]);
}

#[test]
fn sk_2635() {
    let lup = d("lupa~", Divadi);
    assert_has_ta(&[], &yan(&lup), Lat, &["lolupyate"]);

    let sad = d("za\\dx~", Bhvadi);
    assert_has_ta(&[], &yan(&sad), Lat, &["sAsadyate"]);
}

#[test]
fn skip_sk_2636() {}

#[test]
fn sk_2637() {
    let t = Tester::new(Vyakarana::new()).with_ignore_va_padantasya(false);

    let car = d("cara~", Bhvadi);
    t.assert_has_ta(&[], &yan(&car), Lat, &["caYcUryate", "caMcUryate"]);

    let phal = d("YiPalA~", Bhvadi);
    t.assert_has_ta(&[], &yan(&phal), Lat, &["pamPulyate", "paMPulyate"]);
}

#[test]
fn sk_2638() {
    let jap = d("japa~", Bhvadi);
    assert_has_ta(&[], &yan(&jap), Lat, &["jaYjapyate"]);
}

#[test]
fn sk_2639() {
    let gr = d("gF", Tudadi);
    assert_has_ta(&[], &yan(&gr), Lat, &["jegilyate"]);

    let da = d("qudA\\Y", Juhotyadi);
    assert_has_ta(&[], &yan(&da), Lat, &["dedIyate"]);

    let pa = d("pA\\", Bhvadi);
    assert_has_ta(&[], &yan(&pa), Lat, &["pepIyate"]);

    let so = d("zo\\", Divadi);
    assert_has_ta(&[], &yan(&so), Lat, &["sezIyate"]);

    let shvi = d("wuo~Svi", Bhvadi);
    assert_has_ta(&[], &yan(&shvi), Lat, &["SoSUyate", "SeSvIyate"]);

    let smr = d("smf", Bhvadi);
    assert_has_ta(&[], &yan(&smr), Lat, &["sAsmaryate"]);

    let kr = d("qukf\\Y", Tanadi);
    assert_has_ta(&[], &yan(&kr), Lat, &["cekrIyate"]);

    // saYcekrIyate is justified since suw-Agama is optional.
    assert_has_ta(&["sam"], &yan(&kr), Lat, &["saYceskrIyate", "saYcekrIyate"]);
}

#[test]
fn sk_2640() {
    let sic = d("zi\\ca~^", Tudadi);
    assert_has_ta(&["ni"], &yan(&sic), Lat, &["nisesicyate"]);
}

#[test]
fn sk_2641() {
    let ku = d("ku\\N", Bhvadi);
    assert_has_ta(&[], &yan(&ku), Lat, &["kokUyate"]);
}

#[test]
fn sk_2642() {
    let vanc = d("vancu~", Bhvadi);
    assert_has_ta(&[], &yan(&vanc), Lat, &["vanIvacyate"]);

    let srans = d("sransu~\\", Bhvadi);
    assert_has_ta(&[], &yan(&srans), Lat, &["sanIsrasyate"]);
}

#[test]
fn sk_2643() {
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_ta(&[], &yan(&yam), Lat, &["yaMyamyate"]);

    let bham = d("BAma~\\", Bhvadi);
    assert_has_ta(&[], &yan(&bham), Lat, &["bABAmyate"]);

    let jan = d("janI~\\", Divadi);
    assert_has_ta(&[], &yan(&jan), Lat, &["jAjAyate", "jaYjanyate"]);

    let han = d("ha\\na~", Adadi);
    assert_has_ta(&[], &yan(&han), Lat, &["jeGnIyate", "jaNGanyate"]);
}

#[test]
fn sk_2644() {
    let vrt = d("vftu~\\", Bhvadi);
    assert_has_ta(&[], &yan(&vrt), Lat, &["varIvftyate"]);

    let nrt = d("nftI~", Divadi);
    assert_has_ta(&[], &yan(&nrt), Lat, &["narInftyate"]);

    let grah = d("graha~^", Kryadi);
    assert_has_ta(&[], &yan(&grah), Lat, &["jarIgfhyate"]);

    let klp = d("kfpU~\\", Bhvadi);
    assert_has_ta(&[], &yan(&klp), Lat, &["calIkxpyate"]);

    let vrasc = d("o~vrascU~", Tudadi);
    assert_has_ta(&[], &yan(&vrasc), Lat, &["varIvfScyate"]);

    let prach = d("pra\\Ca~", Tudadi);
    assert_has_ta(&[], &yan(&prach), Lat, &["parIpfcCyate"]);
}

#[test]
fn sk_2645() {
    let svap = d("Yizva\\pa~", Adadi);
    assert_has_ta(&[], &yan(&svap), Lat, &["sozupyate"]);

    let syam = d("syamu~", Bhvadi);
    assert_has_ta(&[], &yan(&syam), Lat, &["sesimyate"]);

    let vye = d("vye\\Y", Bhvadi);
    assert_has_ta(&[], &yan(&vye), Lat, &["vevIyate"]);
}

#[test]
fn sk_2646() {
    let vash = d("vaSa~", Adadi);
    assert_has_ta(&[], &yan(&vash), Lat, &["vAvaSyate"]);
}

#[test]
fn sk_2647() {
    let cay = d("cAyf~^", Bhvadi);
    assert_has_ta(&[], &yan(&cay), Lat, &["cekIyate"]);
}

#[test]
fn sk_2648() {
    let ghra = d("GrA\\", Bhvadi);
    assert_has_ta(&[], &yan(&ghra), Lat, &["jeGrIyate"]);

    let dhma = d("DmA\\", Bhvadi);
    assert_has_ta(&[], &yan(&dhma), Lat, &["deDmIyate"]);
}

#[test]
fn sk_2649() {
    let shi = d("SIN", Adadi);
    assert_has_ta(&[], &yan(&shi), Lat, &["SASayyate"]);

    let dhauk = d("QOkf~\\", Bhvadi);
    assert_has_ta(&[], &yan(&dhauk), Lat, &["qoQOkyate"]);

    let trauk = d("trOkf~\\", Bhvadi);
    assert_has_ta(&[], &yan(&trauk), Lat, &["totrOkyate"]);
}
