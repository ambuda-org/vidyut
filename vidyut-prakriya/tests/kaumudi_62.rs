extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn sk_2756() {
    let bhu = d("BU", Bhvadi);
    assert_has_ta_k(&[], &bhu, Lat, &["BUyate"]);
    assert_has_ta_k(&[], &bhu, Lit, &["baBUve"]);
}

#[test]
fn sk_2757() {
    let bhu = d("BU", Bhvadi);
    assert_has_ta_k(&[], &bhu, Lut, &["BAvitA", "BavitA"]);
    assert_has_ta_k(&[], &bhu, Lrt, &["BAvizyate", "Bavizyate"]);
    assert_has_ta_k(&[], &bhu, Lot, &["BUyatAm"]);
    assert_has_ta_k(&[], &bhu, Lan, &["aBUyata"]);
    assert_has_ta_k(&[], &bhu, VidhiLin, &["BUyeta"]);
    assert_has_ta_k(&[], &bhu, AshirLin, &["BAvizIzwa", "BavizIzwa"]);
}

#[test]
fn sk_2758() {
    let bhu = d("BU", Bhvadi);
    assert_has_ta_k(&["anu"], &bhu, Lat, &["anuBUyate"]);
    assert_has_aataam_k(&["anu"], &bhu, Lat, &["anuBUyete"]);
    assert_has_jha_k(&["anu"], &bhu, Lat, &["anuBUyante"]);
    assert_has_thaas_k(&["anu"], &bhu, Lat, &["anuBUyase"]);
    assert_has_iw_k(&["anu"], &bhu, Lat, &["anuBUye"]);
    assert_has_ta_k(&["anu"], &bhu, Lun, &["anvaBAvi"]);
    assert_has_aataam_k(&["anu"], &bhu, Lun, &["anvaBAvizAtAm", "anvaBavizAtAm"]);

    let bhu_nic = nic(&bhu);
    assert_has_ta_k(&[], &bhu_nic, Lat, &["BAvyate"]);
    assert_has_ta_k(
        &[],
        &bhu_nic,
        Lit,
        &["BAvayAYcakre", "BAvayAmbaBUve", "BAvayAmAse"],
    );
    assert_has_ta_k(&[], &bhu_nic, Lut, &["BAvitA", "BAvayitA"]);
    assert_has_ta_k(&[], &bhu_nic, Lrt, &["BAvizyate", "BAvayizyate"]);
    assert_has_ta_k(&[], &bhu_nic, Lot, &["BAvyatAm"]);
    assert_has_ta_k(&[], &bhu_nic, Lan, &["aBAvyata"]);
    assert_has_ta_k(&[], &bhu_nic, VidhiLin, &["BAvyeta"]);
    assert_has_ta_k(&[], &bhu_nic, AshirLin, &["BAvizIzwa", "BAvayizIzwa"]);
    assert_has_ta_k(&[], &bhu_nic, Lun, &["aBAvi"]);
    assert_has_aataam_k(&[], &bhu_nic, Lun, &["aBAvizAtAm", "aBAvayizAtAm"]);

    let bhu_yan = san(&bhu);
    assert_has_ta_k(&[], &bhu_yan, Lat, &["buBUzyate"]);
    assert_has_ta_k(
        &[],
        &bhu_yan,
        Lit,
        &["buBUzAYcakre", "buBUzAmbaBUve", "buBUzAmAse"],
    );
    assert_has_ta_k(&[], &bhu_yan, Lut, &["buBUzitA"]);
    assert_has_ta_k(&[], &bhu_yan, Lrt, &["buBUzizyate"]);

    let bhu_yan = yan(&bhu);
    assert_has_ta_k(&[], &bhu_yan, Lat, &["boBUyyate"]);

    let bhu_yan_luk = yan_luk(&bhu);
    assert_has_ta_k(&[], &bhu_yan_luk, Lat, &["boBUyate"]);
    assert_has_ta_k(
        &[],
        &bhu_yan_luk,
        Lit,
        &["boBavAYcakre", "boBavAmbaBUve", "boBavAmAse"],
    );
    assert_has_ta_k(&[], &bhu_yan_luk, Lut, &["boBAvitA", "boBavitA"]);

    let stu = d("zwu\\Y", Adadi);
    assert_has_ta_k(&[], &stu, Lat, &["stUyate"]);
    assert_has_ta_k(&[], &stu, Lit, &["tuzwuve"]);
    assert_has_ta_k(&[], &stu, Lut, &["stAvitA", "stotA"]);
    assert_has_ta_k(&[], &stu, Lrt, &["stAvizyate", "stozyate"]);
    assert_has_ta_k(&[], &stu, Lun, &["astAvi"]);
    assert_has_aataam_k(&[], &stu, Lun, &["astAvizAtAm", "astozAtAm"]);

    let r = d("f\\", Bhvadi);
    assert_has_ta_k(&[], &r, Lat, &["aryate"]);

    let smr = d("smf\\", Bhvadi);
    assert_has_ta_k(&[], &smr, Lat, &["smaryate"]);
    assert_has_ta_k(&[], &smr, Lit, &["sasmare"]);

    assert_has_ta_k(&[], &r, Lut, &["AritA", "artA"]);
    assert_has_ta_k(&[], &smr, Lut, &["smAritA", "smartA"]);

    let kr = d("qukf\\Y", Tanadi);
    assert_has_ta_k(&["sam"], &kr, Lat, &["saMskriyate", "saNkriyate"]);

    let srans = d("sransu~\\", Bhvadi);
    assert_has_ta_k(&[], &srans, Lat, &["srasyate"]);

    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_ta_k(&[], &yaj, Lat, &["ijyate"]);

    let shi = d("SIN", Adadi);
    assert_has_ta_k(&[], &shi, Lat, &["Sayyate"]);
}

#[test]
fn sk_2759() {
    let tan = d("tanu~^", Tanadi);
    assert_has_ta_k(&[], &tan, Lat, &["tAyate", "tanyate"]);

    let jan = d("janI~\\", Divadi);
    assert_has_ta_k(&[], &jan, Lat, &["jAyate", "janyate"]);
}

#[test]
fn sk_2760() {
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_ta_k(&[], &daa, Lat, &["dIyate"]);

    let dhaa = d("quDA\\Y", Juhotyadi);
    assert_has_ta_k(&[], &dhaa, Lat, &["DIyate"]);

    let glai = d("glE\\", Bhvadi);
    assert_has_ta_k(&[], &glai, Lit, &["jagle"]);
}

#[ignore]
#[test]
fn sk_2762() {
    let x = d("Samu~", Divadi);
    assert_has_tip(&[], &nic(&x), Lut, &["SAmitA", "SamitA", "SamayitA"]);
    assert_has_ta(
        &[],
        &nic(&x),
        Lrt,
        &["SAmizyate", "Samizyate", "Samayizyate"],
    );

    assert_has_ta(&[], &nic(&x), Lat, &["SaMSamyate"]);
    assert_has_ta(
        &[],
        &nic(&x),
        Lut,
        &["SaMSAmitA", "SaMSamitA", "SaMSamayitA"],
    );
}

#[test]
fn sk_2763() {
    let sham = d("Samu~", Bhvadi);
    assert_has_ta_k(&[], &sham, Lun, &["aSami"]);

    let dam = d("damu~", Bhvadi);
    assert_has_ta_k(&[], &dam, Lun, &["adami"]);

    let gam = d("ga\\mx~", Bhvadi);
    assert_has_ta_k(&[], &gam, Lun, &["agAmi"]);

    let vad = d("vada~", Bhvadi);
    assert_has_ta_k(&[], &vad, Lun, &["avAdi"]);

    let cam = d("camu~", Bhvadi);
    assert_has_ta_k(&["AN"], &cam, Lun, &["AcAmi"]);

    let kam = d("kamu~\\", Bhvadi);
    assert_has_ta_k(&[], &kam, Lun, &["akAmi"]);

    let vam = d("wuvama~", Bhvadi);
    assert_has_ta_k(&[], &vam, Lun, &["avAmi"]);

    // aGAni is from SK 2761.
    let han = d("ha\\na~", Adadi);
    assert_has_ta_k(&[], &han, Lun, &["avaDi", "aGAni"]);

    let jagr = d("jAgf", Adadi);
    assert_has_ta_k(&[], &jagr, Lun, &["ajAgAri"]);
}

#[test]
fn sk_2764() {
    let bhanj = d("Ba\\njo~", Rudhadi);
    assert_has_ta_k(&[], &bhanj, Lun, &["aBAji", "aBaYji"]);
}

#[test]
fn sk_2765() {
    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_ta_k(&[], &labh, Lun, &["alamBi", "alABi"]);
    assert_has_ta_k(&["pra"], &labh, Lun, &["prAlamBi"]);

    let ni = d("RI\\Y", Bhvadi);
    assert_has_ta_k(&[], &ni, Lat, &["nIyate"]);

    let hr = d("hf\\Y", Bhvadi);
    assert_has_ta_k(&[], &hr, Lat, &["hriyate"]);

    let krsh = d("kf\\za~", Bhvadi);
    assert_has_ta_k(&[], &krsh, Lat, &["kfzyate"]);

    let vah = d("va\\ha~^", Bhvadi);
    assert_has_ta_k(&[], &vah, Lat, &["uhyate"]);

    let budh = d("bu\\Da~\\", Divadi);
    assert_has_ta_k(&[], &nic(&budh), Lat, &["boDyate"]);

    let gam = d("ga\\mx~", Bhvadi);
    assert_has_ta_k(&[], &gam, Lat, &["gamyate"]);

    let aas = d("Asa~\\", Bhvadi);
    assert_has_ta_k(&[], &aas, Lat, &["Asyate"]);
}
