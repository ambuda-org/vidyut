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

fn nic(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Nic])
}

fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San])
}

fn yan(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Yan])
}

fn yan_luk(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::YanLuk])
}

#[test]
fn sutra_7_4_1() {
    assert_has_lun_p(&[], &nic(&d("qukf\\Y", Tanadi)), &["acIkarat"]);
    assert_has_lun_p(&[], &nic(&d("hf\\Y", Bhvadi)), &["ajIharat"]);
    assert_has_lun_p(&[], &nic(&d("lUY", Kryadi)), &["alIlavat"]);
    assert_has_lun_p(&[], &nic(&d("pUY", Kryadi)), &["apIpavat"]);

    // TODO: many more.
}

#[test]
fn sutra_7_4_2() {
    assert_has_lun_p(&[], &nic(&d("SAsu~", Bhvadi)), &["aSaSAsat"]);
    assert_has_lun_p(&[], &nic(&d("bADf~\\", Bhvadi)), &["ababADat"]);
    assert_has_lun_p(&[], &nic(&d("quyAcf~^", Bhvadi)), &["ayayAcat"]);
    assert_has_lun_p(&[], &nic(&d("QOkf~\\", Bhvadi)), &["aquQOkat"]);
}

#[test]
fn sutra_7_4_3() {
    assert_has_lun_p(
        &[],
        &nic(&d("wuBrAjf~\\", Bhvadi)),
        &["abiBrajat", "abaBrAjat"],
    );
    assert_has_lun_p(&[], &nic(&d("BAsf~\\", Bhvadi)), &["abIBasat", "abaBAsat"]);
    assert_has_lun_p(&[], &nic(&d("BAza~\\", Bhvadi)), &["abIBazat", "abaBAzat"]);
    assert_has_lun_p(&[], &nic(&d("dIpI~\\", Divadi)), &["adIdipat", "adidIpat"]);
    assert_has_lun_p(&[], &nic(&d("jIva~", Bhvadi)), &["ajIjivat", "ajijIvat"]);
    assert_has_lun_p(&[], &nic(&d("mIla~", Bhvadi)), &["amImilat", "amimIlat"]);
    assert_has_lun_p(&[], &d("pIqa~", Curadi), &["apIpiqat", "apipIqat"]);
}

#[test]
fn sutra_7_4_10() {
    let svf = d("svf", Bhvadi);
    assert_has_parasmai_tinanta(&[], &svf, Lit, Prathama, Dvi, &["sasvaratuH"]);
    assert_has_parasmai_tinanta(&[], &svf, Lit, Prathama, Bahu, &["sasvaruH"]);

    let dhvf = d("Dvf\\", Bhvadi);
    assert_has_parasmai_tinanta(&[], &dhvf, Lit, Prathama, Dvi, &["daDvaratuH"]);
    assert_has_parasmai_tinanta(&[], &dhvf, Lit, Prathama, Bahu, &["daDvaruH"]);

    let smf = d("smf", Bhvadi);
    assert_has_parasmai_tinanta(&[], &smf, Lit, Prathama, Dvi, &["sasmaratuH"]);
    assert_has_parasmai_tinanta(&[], &smf, Lit, Prathama, Bahu, &["sasmaruH"]);

    // ftaH
    let kzi = d("kzi\\", Tudadi);
    assert_has_parasmai_tinanta(&[], &kzi, Lit, Prathama, Dvi, &["cikziyatuH"]);
    assert_has_parasmai_tinanta(&[], &kzi, Lit, Prathama, Bahu, &["cikziyuH"]);
    // saMyogAdeH
    let kf = d("qukf\\Y", Tanadi);
    assert_has_parasmai_tinanta(&[], &kf, Lit, Prathama, Dvi, &["cakratuH"]);
    assert_has_parasmai_tinanta(&[], &kf, Lit, Prathama, Bahu, &["cakruH"]);
    // vrddhi otherwise
    assert_has_lit_p(&[], &svf, &["sasvAra"]);
    assert_has_lit_p(&[], &smf, &["sasmAra"]);
    // liwi
    assert_has_krdanta(&[], &smf, Krt::kta, &["smfta"]);
    assert_has_krdanta(&[], &smf, Krt::ktavatu, &["smftavat"]);
    // saMskf
    assert_has_parasmai_tinanta(
        &["sam"],
        &kf,
        Lit,
        Prathama,
        Dvi,
        &["saYcaskratuH", "saYcakratuH"],
    );
    assert_has_parasmai_tinanta(
        &["sam"],
        &kf,
        Lit,
        Prathama,
        Bahu,
        &["saYcaskruH", "saYcakruH"],
    );
}

#[test]
fn sutra_7_4_11() {
    let fch = d("fCa~", Tudadi);
    assert_has_parasmai_tinanta(&[], &fch, Lit, Prathama, Eka, &["AnarCa", "AnarcCa"]);
    assert_has_parasmai_tinanta(&[], &fch, Lit, Prathama, Dvi, &["AnarCatuH", "AnarcCatuH"]);
    assert_has_parasmai_tinanta(&[], &fch, Lit, Prathama, Bahu, &["AnarCuH", "AnarcCuH"]);

    let f = d("f\\", Bhvadi);
    assert_has_parasmai_tinanta(&[], &f, Lit, Prathama, Eka, &["Ara"]);
    assert_has_parasmai_tinanta(&[], &f, Lit, Prathama, Dvi, &["AratuH"]);
    assert_has_parasmai_tinanta(&[], &f, Lit, Prathama, Bahu, &["AruH"]);

    let kff = d("kF", Tudadi);
    assert_has_parasmai_tinanta(&["ni"], &kff, Lit, Prathama, Dvi, &["nicakaratuH"]);
    assert_has_parasmai_tinanta(&["ni"], &kff, Lit, Prathama, Bahu, &["nicakaruH"]);

    let gff = d("gF", Tudadi);
    assert_has_parasmai_tinanta(
        &["ni"],
        &gff,
        Lit,
        Prathama,
        Dvi,
        &["nijagaratuH", "nijagalatuH"],
    );
    assert_has_parasmai_tinanta(
        &["ni"],
        &gff,
        Lit,
        Prathama,
        Bahu,
        &["nijagaruH", "nijagaluH"],
    );

    // vrddhi
    assert_has_lit(&["ni"], &kff, &["nicakAra"]);
    assert_has_lit(&["ni"], &gff, &["nijagAra", "nijagAla"]);
}

#[test]
fn sutra_7_4_12() {
    let shf = d("SFY", Kryadi);
    assert_has_parasmai_tinanta(
        &["vi"],
        &shf,
        Lit,
        Prathama,
        Dvi,
        &["viSaSratuH", "viSaSaratuH"],
    );
    assert_has_parasmai_tinanta(
        &["vi"],
        &shf,
        Lit,
        Prathama,
        Bahu,
        &["viSaSruH", "viSaSaruH"],
    );

    let shf2 = d("SF", Kryadi);
    assert_has_parasmai_tinanta(
        &["vi"],
        &shf2,
        Lit,
        Prathama,
        Dvi,
        &["viSaSratuH", "viSaSaratuH"],
    );
    assert_has_parasmai_tinanta(
        &["vi"],
        &shf2,
        Lit,
        Prathama,
        Bahu,
        &["viSaSruH", "viSaSaruH"],
    );

    let df = d("dF", Kryadi);
    assert_has_parasmai_tinanta(
        &["vi"],
        &df,
        Lit,
        Prathama,
        Dvi,
        &["vidadratuH", "vidadaratuH"],
    );
    assert_has_parasmai_tinanta(
        &["vi"],
        &df,
        Lit,
        Prathama,
        Bahu,
        &["vidadruH", "vidadaruH"],
    );

    let pf = d("pF", Juhotyadi);
    assert_has_parasmai_tinanta(
        &["ni"],
        &pf,
        Lit,
        Prathama,
        Dvi,
        &["nipapratuH", "nipaparatuH"],
    );
    assert_has_parasmai_tinanta(
        &["ni"],
        &pf,
        Lit,
        Prathama,
        Bahu,
        &["nipapruH", "nipaparuH"],
    );
}

#[test]
fn sutra_7_4_16() {
    assert_has_lun(&[], &d("sf\\", Bhvadi), &["asarat"]);
    assert_has_lun(&[], &d("f\\", Juhotyadi), &["Arat"]);

    let dfs = d("df\\Si~r", Bhvadi);
    assert_has_parasmai_tinanta(&[], &dfs, Lun, Prathama, Eka, &["adarSat", "adrAkzIt"]);
    assert_has_parasmai_tinanta(&[], &dfs, Lun, Prathama, Dvi, &["adarSatAm", "adrAzwAm"]);
    assert_has_parasmai_tinanta(&[], &dfs, Lun, Prathama, Bahu, &["adarSan", "adrAkzuH"]);
}

#[test]
fn sutra_7_4_17() {
    let asu = d("asu~", Divadi);
    assert_has_parasmai_tinanta(&[], &asu, Lun, Prathama, Eka, &["AsTat"]);
    assert_has_parasmai_tinanta(&[], &asu, Lun, Prathama, Dvi, &["AsTatAm"]);
    assert_has_parasmai_tinanta(&[], &asu, Lun, Prathama, Bahu, &["AsTan"]);
}

#[test]
fn sutra_7_4_18() {
    let svi = d("wuo~Svi", Bhvadi);
    assert_has_parasmai_tinanta(
        &[],
        &svi,
        Lun,
        Prathama,
        Eka,
        &["aSvat", "aSvayIt", "aSiSviyat"],
    );
    assert_has_parasmai_tinanta(
        &[],
        &svi,
        Lun,
        Prathama,
        Dvi,
        &["aSvatAm", "aSvayizwAm", "aSiSviyatAm"],
    );
    assert_has_parasmai_tinanta(
        &[],
        &svi,
        Lun,
        Prathama,
        Bahu,
        &["aSvan", "aSvayizuH", "aSiSviyan"],
    );
}

#[test]
fn sutra_7_4_19() {
    let pat = d("patx~", Bhvadi);
    assert_has_parasmai_tinanta(&[], &pat, Lun, Prathama, Eka, &["apaptat"]);
    assert_has_parasmai_tinanta(&[], &pat, Lun, Prathama, Dvi, &["apaptatAm"]);
    assert_has_parasmai_tinanta(&[], &pat, Lun, Prathama, Bahu, &["apaptan"]);
}

#[test]
fn sutra_7_4_20() {
    let vac = d("va\\ca~", Adadi);
    assert_has_parasmai_tinanta(&[], &vac, Lun, Prathama, Eka, &["avocat"]);
    assert_has_parasmai_tinanta(&[], &vac, Lun, Prathama, Dvi, &["avocatAm"]);
    assert_has_parasmai_tinanta(&[], &vac, Lun, Prathama, Bahu, &["avocan"]);
}

#[test]
fn sutra_7_4_21() {
    let shi = d("SIN", Adadi);
    assert_has_tinanta(&[], &shi, Lat, Prathama, Eka, &["Sete"]);
    assert_has_tinanta(&[], &shi, Lat, Prathama, Dvi, &["SayAte"]);
    assert_has_tinanta(&[], &shi, Lat, Prathama, Bahu, &["Serate"]);
}

#[test]
fn sutra_7_4_22() {
    let shi = d("SIN", Adadi);
    assert_has_lat_karmani(&[], &shi, &["Sayyate"]);
    assert_has_lat(&[], &yan(&shi), &["SASayyate"]);
    assert_has_krdanta(&["pra"], &shi, Krt::ktvA, &["praSayya"]);
    assert_has_krdanta(&["upa"], &shi, Krt::ktvA, &["upaSayya"]);
    // yi
    assert_has_lit(&[], &shi, &["SiSye"]);
}

#[test]
fn sutra_7_4_23() {
    let uh = d("Uha~\\", Bhvadi);
    assert_has_lat_karmani(&["sam"], &uh, &["samuhyate"]);
    assert_has_krdanta(&["sam"], &uh, Krt::ktvA, &["samuhya"]);
    assert_has_lat_karmani(&["aBi"], &uh, &["aByuhyate"]);
    assert_has_krdanta(&["aBi"], &uh, Krt::ktvA, &["aByuhya"]);

    assert_has_lat_karmani(&[], &uh, &["Uhyate"]);

    let ih = d("Iha~\\", Bhvadi);
    assert_has_lat_karmani(&["sam"], &ih, &["samIhyate"]);
    assert_has_krdanta(&["sam"], &uh, Krt::kta, &["samUhita"]);

    assert_has_lat_karmani(&["AN"], &uh, &["ohyate"]);
    assert_has_lat_karmani(&["sam", "AN"], &uh, &["samohyate"]);
}

#[test]
fn sutra_7_4_24() {
    let i = d("i\\R", Adadi);
    assert_has_ashirlin(&["ud"], &i, &["udiyAt"]);
    assert_has_ashirlin(&["sam"], &i, &["samiyAt"]);
    assert_has_ashirlin(&["anu"], &i, &["anviyAt"]);

    assert_has_ashirlin(&[], &i, &["IyAt"]);

    assert_has_ashirlin(&["AN"], &i, &["eyAt"]);
    assert_has_ashirlin(&["sam", "AN"], &i, &["sameyAt"]);
}

#[test]
fn sutra_7_4_28() {
    let d = d;

    let kf = d("qukf\\Y", Tanadi);
    let hf = d("hf\\Y", Bhvadi);
    let bhf = d("quBf\\Y", Juhotyadi);

    assert_has_lat_karmani(&["AN"], &d("df\\N", Tudadi), &["Adriyate"]);
    assert_has_lat_karmani(&["AN"], &d("Df\\Y", Bhvadi), &["ADriyate"]);
    assert_has_lat_karmani(&[], &kf, &["kriyate"]);
    assert_has_lat_karmani(&[], &hf, &["hriyate"]);
    assert_has_ashirlin_p(&[], &kf, &["kriyAt"]);
    assert_has_ashirlin_p(&[], &hf, &["hriyAt"]);
    assert_has_vidhilin_p(&[], &bhf, &["biBfyAt"]);
    assert_has_ashirlin_a(&[], &kf, &["kfzIzwa"]);
    assert_has_ashirlin_a(&[], &hf, &["hfzIzwa"]);
}

#[test]
fn sutra_7_4_29() {
    let f = d("f\\", Bhvadi);
    assert_has_lat_karmani(&[], &f, &["aryate"]);
    assert_has_ashirlin_p(&[], &f, &["aryAt"]);

    let smf = d("smf", Bhvadi);
    assert_has_lat_karmani(&[], &smf, &["smaryate"]);
    assert_has_ashirlin_p(&[], &smf, &["smaryAt"]);

    let kf = d("qukf\\Y", Tanadi);
    assert_has_lat_karmani(&["sam"], &kf, &["saMskriyate", "saNkriyate"]);
    assert_has_ashirlin_p(&["sam"], &kf, &["saMskriyAt", "saNkriyAt"]);

    // TODO: also derives svarizIzwa and svArizIzwa, which I need to check.
    /*
    let svf = d("svf", Bhvadi);
    assert_has_ashirlin_karmani(&[], &svf, &["svfzIzwa"]);
    */
}

#[test]
fn sutra_7_4_30() {
    assert_has_lat(&[], &yan(&d("f\\", Bhvadi)), &["arAryate"]);
    assert_has_lat(&[], &yan(&d("svf", Bhvadi)), &["sAsvaryate"]);
    assert_has_lat(&[], &yan(&d("Dvf\\", Bhvadi)), &["dADvaryate"]);
    assert_has_lat(&[], &yan(&d("smf", Bhvadi)), &["sAsmaryate"]);
}

#[test]
fn sutra_7_4_31() {
    assert_has_lat(&[], &yan(&d("GrA\\", Bhvadi)), &["jeGrIyate"]);
    assert_has_lat(&[], &yan(&d("DmA\\", Bhvadi)), &["deDmIyate"]);
}

#[ignore]
#[test]
fn sutra_7_4_40() {
    let do_ = d("do\\", Divadi);
    assert_has_krdanta(&["nis"], &do_, Krt::kta, &["nirdita"]);
    assert_has_krdanta(&["nis"], &do_, Krt::ktavatu, &["nirdita"]);
    let so = d("zo\\", Divadi);
    assert_has_krdanta(&["ava"], &so, Krt::kta, &["avasita"]);
    assert_has_krdanta(&["ava"], &so, Krt::ktavatu, &["avasitavat"]);
    let maa = d("mA\\", Adadi);
    assert_has_krdanta(&[], &maa, Krt::kta, &["mita"]);
    assert_has_krdanta(&[], &maa, Krt::ktavatu, &["mitavat"]);
    let stha = d("zWA\\", Bhvadi);
    assert_has_krdanta(&[], &stha, Krt::kta, &["sTita"]);
    assert_has_krdanta(&[], &stha, Krt::ktavatu, &["sTitavat"]);
    // ti
    assert_has_krdanta(&["ava"], &do_, Krt::kta, &["avadAya"]);
    // kiti
    assert_has_krdanta(&["ava"], &do_, Krt::tfc, &["avadAtf"]);
}

#[test]
fn sutra_7_4_49() {
    let vas = d("va\\sa~", Bhvadi);
    let ad = d("a\\da~", Adadi);
    assert_has_lrt(&[], &vas, &["vatsyati"]);
    assert_has_lrn(&[], &vas, &["avatsyat"]);
    assert_has_lat(&[], &san(&vas), &["vivatsati"]);
    assert_has_lat(&[], &san(&ad), &["jiGatsati"]);
    // saH
    assert_has_lrt(&[], &d("va\\ca~", Adadi), &["vakzyati"]);
    // si
    assert_has_krdanta(&[], &ad, Krt::GaY, &["GAsa"]);
    // ArDaDAtuke
    assert_has_atmane_tinanta(&[], &d("Asa~\\", Adadi), Lat, Madhyama, Eka, &["Asse"]);
    assert_has_atmane_tinanta(&[], &d("vasa~\\", Adadi), Lat, Madhyama, Eka, &["vasse"]);
}

#[test]
fn sutra_7_4_50() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_tinanta(&[], &kf, Lut, Madhyama, Eka, &["kartAsi", "kartAse"]);
    assert_has_tinanta(&[], &d("asa~", Adadi), Lat, Madhyama, Eka, &["asi"]);
    // TODO: vyatise
}

#[test]
fn sutra_7_4_51() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_tinanta(&[], &kf, Lut, Prathama, Dvi, &["kartArO"]);
    assert_has_tinanta(&[], &kf, Lut, Prathama, Bahu, &["kartAraH"]);
    let i = d("i\\N", Adadi);
    assert_has_tinanta(&["aDi"], &i, Lut, Prathama, Dvi, &["aDyetArO"]);
    assert_has_tinanta(&["aDi"], &i, Lut, Prathama, Bahu, &["aDyetAraH"]);
}

#[test]
fn sutra_7_4_52() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_atmane_tinanta(&[], &kf, Lut, Uttama, Eka, &["kartAhe"]);
    // TODO: vyatihe
}

#[test]
fn sutra_7_4_53() {
    let didhi = d("dIDIN", Adadi);
    let vevi = d("vevIN", Adadi);

    assert_has_krdanta(&["AN"], &didhi, Krt::ktvA, &["AdIDya"]);
    assert_has_krdanta(&["AN"], &vevi, Krt::ktvA, &["Avevya"]);
    assert_has_lat_karmani(&["AN"], &didhi, &["AdIDyate"]);
    assert_has_lat_karmani(&["AN"], &vevi, &["Avevyate"]);
    assert_has_krdanta(&["AN"], &didhi, Krt::tfc, &["AdIDitf"]);
    assert_has_krdanta(&["AN"], &vevi, Krt::tfc, &["Avevitf"]);
    assert_has_vidhilin(&["AN"], &didhi, &["AdIDIta"]);
    assert_has_vidhilin(&["AN"], &vevi, &["AvevIta"]);
    // yIvarRayoH
    assert_has_krdanta(&["AN"], &didhi, Krt::lyuw, &["AdIDyana"]);
    assert_has_krdanta(&["AN"], &vevi, Krt::lyuw, &["Avevyana"]);
}

#[ignore]
#[test]
fn sutra_7_4_54() {
    assert_has_lat_p(&[], &san(&d("mI\\Y", Kryadi)), &["mitsati"]);
    assert_has_lat_p(&["pra"], &san(&d("qumi\\Y", Svadi)), &["pramitsati"]);
    assert_has_lat(&[], &san(&d("mA\\", Adadi)), &["mitsati"]);
    assert_has_lat(&[], &san(&d("mA\\N", Juhotyadi)), &["mitsate"]);
    assert_has_lat(&["apa"], &san(&d("me\\N", Bhvadi)), &["apamitsate"]);
    assert_has_lat_p(&[], &san(&d("qudA\\Y", Juhotyadi)), &["ditsati"]);
    assert_has_lat_p(&[], &san(&d("quDA\\Y", Juhotyadi)), &["Ditsati"]);
    assert_has_lat(&["AN"], &san(&d("ra\\Ba~\\", Bhvadi)), &["Aripsate"]);
    assert_has_lat(&["AN"], &san(&d("qula\\Ba~\\z", Bhvadi)), &["Alipsate"]);
    assert_has_lat(&[], &san(&d("Sa\\kx~", Svadi)), &["Sikzati"]);
    assert_has_lat(&[], &san(&d("patx~", Bhvadi)), &["pitsati", "pipatizati"]);
    assert_has_lat(&["pra"], &san(&d("pa\\da~\\", Divadi)), &["prapitsate"]);
    // sani
    assert_has_lrt_p(&[], &d("qudA\\Y", Juhotyadi), &["dAsyati"]);
    // si -- see `pipatizati` above.
}

#[ignore]
#[test]
fn sutra_7_4_55() {
    let aap = d("A\\px~", Svadi);
    let jnap = d("jYapa~", Curadi);
    let rdh = d("fDu~", Divadi);
    assert_has_lat(&[], &san(&aap), &["Ipsati"]);
    assert_has_lat(&[], &san(&jnap), &["jYIpsati", "jijYapayizati"]);
    assert_has_lat(&[], &san(&rdh), &["Irtsati", "ardiDizati"]);
    // sani
    assert_has_lrt_p(&["pra"], &san(&aap), &["prApsyati"]);
    // si -- see examples above.
}

#[test]
fn sutra_7_4_59() {
    let qauk = d("QOkf~\\", Bhvadi);
    let trauk = d("trOkf~\\", Bhvadi);
    assert_has_lat(&[], &san(&qauk), &["quQOkizate"]);
    assert_has_lat(&[], &san(&trauk), &["tutrOkizate"]);
    assert_has_lit(&[], &qauk, &["quQOke"]);
    assert_has_lit(&[], &trauk, &["tutrOke"]);
    assert_has_lun_p(&[], &nic(&qauk), &["aquQOkat"]);
    assert_has_lun_p(&[], &nic(&trauk), &["atutrOkat"]);
    // TODO: others.
}

#[test]
fn sutra_7_4_60() {
    assert_has_lit_p(&[], &d("glE\\", Bhvadi), &["jaglO"]);
    assert_has_lit_p(&[], &d("mlE\\", Bhvadi), &["mamlO"]);
    assert_has_lit_p(&[], &d("qupa\\ca~^z", Bhvadi), &["papAca"]);
    assert_has_lit_p(&[], &d("paWa~", Bhvadi), &["papAWa"]);

    let aw = d("awa~", Bhvadi);
    assert_has_parasmai_tinanta(&[], &aw, Lit, Prathama, Eka, &["Awa"]);
    assert_has_parasmai_tinanta(&[], &aw, Lit, Prathama, Dvi, &["AwatuH"]);
    assert_has_parasmai_tinanta(&[], &aw, Lit, Prathama, Bahu, &["AwuH"]);
}

#[test]
fn sutra_7_4_61() {
    assert_has_lat(
        &[],
        &san(&d("Scyuti~r", Bhvadi)),
        &["cuScyotizati", "cuScyutizati"],
    );
    assert_has_lat_p(&[], &san(&d("zWA\\", Bhvadi)), &["tizWAsati"]);
    assert_has_lat(&[], &san(&d("spadi~\\", Bhvadi)), &["pispandizate"]);
    // SarpUrva
    assert_has_lit_p(&[], &d("qupa\\ca~^z", Bhvadi), &["papAca"]);
    // KayaH
    assert_has_lit(&[], &d("zRE\\", Bhvadi), &["sasnO"]);
}

#[test]
fn sutra_7_4_62() {
    assert_has_lit_p(&[], &d("qukf\\Y", Tanadi), &["cakAra"]);
    assert_has_lit_p(&[], &d("Kanu~^", Bhvadi), &["caKAna"]);
    assert_has_lit_p(&[], &d("ga\\mx~", Bhvadi), &["jagAma"]);
    assert_has_lit_p(&[], &d("ha\\na~", Adadi), &["jaGAna"]);
    assert_has_lit_p(&[], &d("hf\\Y", Bhvadi), &["jahAra"]);
    assert_has_lat_p(&[], &san(&d("hf\\Y", Bhvadi)), &["jihIrzati"]);
    assert_has_lit_p(&[], &d("o~hA\\k", Juhotyadi), &["jahO"]);
}

#[test]
fn sutra_7_4_63() {
    assert_has_lat(&[], &yan(&d("ku\\N", Bhvadi)), &["kokUyate"]);
    // only for kuN-bhvAdi
    assert_has_lat(&[], &yan(&d("ku\\", Adadi)), &["cokUyate"]);
    assert_has_lat(&[], &yan(&d("ku\\N", Tudadi)), &["cokUyate"]);
    // yaNi
    assert_has_lit(&[], &d("ku\\N", Bhvadi), &["cukuve"]);
}

#[test]
fn sutra_7_4_66() {
    assert_has_lit(&[], &d("vftu~\\", Bhvadi), &["vavfte"]);
    assert_has_lit(&[], &d("vfDu~\\", Bhvadi), &["vavfDe"]);
    assert_has_lit(&[], &d("SfDu~\\", Bhvadi), &["SaSfDe"]);
    // TODO: yaNluk examples.
}

#[ignore]
#[test]
fn sutra_7_4_67() {
    let dyut = d("dyuta~\\", Bhvadi);
    assert_has_lit(&["vi"], &dyut, &["vididyute"]);
    assert_has_lun(&["vi"], &nic(&dyut), &["vyadidyutat"]);
    assert_has_lat(&["vi"], &san(&dyut), &["vididyotizate", "vidyutizate"]);
    assert_has_lit(&["vi"], &yan(&dyut), &["videdyutyate"]);

    let svap = nic(&d("Yizva\\pa~", Adadi));
    assert_has_lat_p(&[], &san(&svap), &["suzvApayizati"]);
}

#[test]
fn sutra_7_4_68() {
    let vyath = d("vyaTa~\\", Bhvadi);
    assert_has_atmane_tinanta(&[], &vyath, Lit, Prathama, Eka, &["vivyaTe"]);
    assert_has_atmane_tinanta(&[], &vyath, Lit, Prathama, Dvi, &["vivyaTAte"]);
    assert_has_atmane_tinanta(&[], &vyath, Lit, Prathama, Bahu, &["vivyaTire"]);
    // liti
    assert_has_lat(&[], &yan(&vyath), &["vAvyaTyate"]);
}

#[test]
fn sutra_7_4_69() {
    let i = d("i\\R", Adadi);
    assert_has_parasmai_tinanta(&[], &i, Lit, Prathama, Dvi, &["IyatuH"]);
    assert_has_parasmai_tinanta(&[], &i, Lit, Prathama, Bahu, &["IyuH"]);
    // kiti
    assert_has_parasmai_tinanta(&[], &i, Lit, Prathama, Eka, &["iyAya"]);
    assert_has_parasmai_tinanta(&[], &i, Lit, Madhyama, Eka, &["iyayiTa", "iyeTa"]);
}

#[test]
fn sutra_7_4_70() {
    let aw = d("awa~", Bhvadi);
    assert_has_parasmai_tinanta(&[], &aw, Lit, Prathama, Eka, &["Awa"]);
    assert_has_parasmai_tinanta(&[], &aw, Lit, Prathama, Dvi, &["AwatuH"]);
    assert_has_parasmai_tinanta(&[], &aw, Lit, Prathama, Bahu, &["AwuH"]);
    // AdeH
    assert_has_lit_p(&[], &d("qupa\\ca~^z", Bhvadi), &["papAca"]);
    assert_has_lit_p(&[], &d("paWa~", Bhvadi), &["papAWa"]);
}

#[test]
fn sutra_7_4_73() {
    let bhu = d("BU", Bhvadi);
    assert_has_parasmai_tinanta(&[], &bhu, Lit, Prathama, Eka, &["baBUva"]);
    assert_has_parasmai_tinanta(&[], &bhu, Lit, Prathama, Dvi, &["baBUvatuH"]);
    assert_has_parasmai_tinanta(&[], &bhu, Lit, Prathama, Bahu, &["baBUvuH"]);
    assert_has_lat(&[], &san(&bhu), &["buBUzati"]);
    assert_has_lat(&[], &yan(&bhu), &["boBUyate"]);
}

#[test]
fn sutra_7_4_75() {
    let nij = d("Ri\\ji~^r", Juhotyadi);
    assert_has_lat_p(&[], &nij, &["nenekti"]);
    assert_has_lat_p(&[], &d("vi\\ji~^r", Juhotyadi), &["vevekti"]);
    assert_has_lat_p(&[], &d("vi\\zx~^", Juhotyadi), &["vevezwi"]);
    // SlO
    assert_has_lit_p(&[], &nij, &["nineja"]);
}

#[test]
fn sutra_7_4_76() {
    let bhf = d("quBf\\Y", Juhotyadi);
    assert_has_lat_p(&[], &bhf, &["biBarti"]);
    assert_has_lat_a(&[], &d("mA\\N", Juhotyadi), &["mimIte"]);
    assert_has_lat_a(&[], &d("o~hA\\N", Juhotyadi), &["jihIte"]);
    // trayARAm
    assert_has_lat_p(&[], &d("o~hA\\k", Juhotyadi), &["jahAti"]);
    // SlO
    assert_has_lit_p(
        &[],
        &bhf,
        &["baBAra", "biBarAYcakAra", "biBarAmAsa", "biBarAmbaBUva"],
    );
}

#[test]
fn sutra_7_4_77() {
    assert_has_lat_p(&[], &d("f\\", Juhotyadi), &["iyarti"]);
    assert_has_lat_p(&[], &d("pF", Juhotyadi), &["piparti"]);
}

#[test]
fn sutra_7_4_79() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_lat_p(&[], &san(&pac), &["pipakzati"]);
    assert_has_lat_p(&[], &san(&d("ya\\ja~^", Bhvadi)), &["yiyakzati"]);
    assert_has_lat_p(&[], &san(&d("zWA\\", Bhvadi)), &["tizWAsati"]);
    assert_has_lat_p(&[], &san(&d("pA\\", Bhvadi)), &["pipAsati"]);
    // sani
    assert_has_lit_p(&[], &pac, &["papAca"]);
    // ataH
    assert_has_lat_p(&[], &san(&d("lUY", Kryadi)), &["lulUzati"]);
    // taparakaraNa
    let _pac_yan_san = pac.with_sanadi(&[Sanadi::Yan, Sanadi::San]);
    // assert_has_lat_a(&[], &pac_yan_san, &["pApacizate"]);
    // TODO: pApacizate
}

#[ignore]
#[test]
fn sutra_7_4_83() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_lat_a(&[], &yan(&pac), &["pApacyate"]);
    assert_has_lat_p(&[], &yan_luk(&pac), &["pApacIti"]);
    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_lat_a(&[], &yan(&yaj), &["yAyajyate"]);
    assert_has_lat_p(&[], &yan_luk(&yaj), &["yAyajIti"]);
}
