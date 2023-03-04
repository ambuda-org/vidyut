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

#[test]
fn sutra_7_4_1() {
    let d = |u, gana| Dhatu::new(u, gana).with_sanadi(&[Sanadi::Nic]);
    assert_has_lun_p(&[], &d("qukf\\Y", Tanadi), &["acIkarat"]);
    assert_has_lun_p(&[], &d("hf\\Y", Bhvadi), &["ajIharat"]);
    assert_has_lun_p(&[], &d("lUY", Kryadi), &["alIlavat"]);
    assert_has_lun_p(&[], &d("pUY", Kryadi), &["apIpavat"]);

    // TODO: many more.
}

#[test]
fn sutra_7_4_2() {
    let nic = |u, g| Dhatu::new(u, g).with_sanadi(&[Sanadi::Nic]);
    assert_has_lun_p(&[], &nic("SAsu~", Bhvadi), &["aSaSAsat"]);
    assert_has_lun_p(&[], &nic("bADf~\\", Bhvadi), &["ababADat"]);
    assert_has_lun_p(&[], &nic("quyAcf~^", Bhvadi), &["ayayAcat"]);
    assert_has_lun_p(&[], &nic("QOkf~\\", Bhvadi), &["aquQOkat"]);
}

#[test]
fn sutra_7_4_3() {
    let nic = |u, g| Dhatu::new(u, g).with_sanadi(&[Sanadi::Nic]);
    assert_has_lun_p(&[], &nic("wuBrAjf~\\", Bhvadi), &["abiBrajat", "abaBrAjat"]);
    assert_has_lun_p(&[], &nic("BAsf~\\", Bhvadi), &["abIBasat", "abaBAsat"]);
    assert_has_lun_p(&[], &nic("BAza~\\", Bhvadi), &["abIBazat", "abaBAzat"]);
    assert_has_lun_p(&[], &nic("dIpI~\\", Divadi), &["adIdipat", "adidIpat"]);
    assert_has_lun_p(&[], &nic("jIva~", Bhvadi), &["ajIjivat", "ajijIvat"]);
    assert_has_lun_p(&[], &nic("mIla~", Bhvadi), &["amImilat", "amimIlat"]);
    assert_has_lun_p(&[], &Dhatu::new("pIqa~", Curadi), &["apIpiqat", "apipIqat"]);
}

#[test]
fn sutra_7_4_10() {
    let svf = Dhatu::new("svf", Bhvadi);
    assert_has_parasmai_tinanta(&[], &svf, Lit, Prathama, Dvi, &["sasvaratuH"]);
    assert_has_parasmai_tinanta(&[], &svf, Lit, Prathama, Bahu, &["sasvaruH"]);

    let dhvf = Dhatu::new("Dvf\\", Bhvadi);
    assert_has_parasmai_tinanta(&[], &dhvf, Lit, Prathama, Dvi, &["daDvaratuH"]);
    assert_has_parasmai_tinanta(&[], &dhvf, Lit, Prathama, Bahu, &["daDvaruH"]);

    let smf = Dhatu::new("smf", Bhvadi);
    assert_has_parasmai_tinanta(&[], &smf, Lit, Prathama, Dvi, &["sasmaratuH"]);
    assert_has_parasmai_tinanta(&[], &smf, Lit, Prathama, Bahu, &["sasmaruH"]);

    // ftaH
    let kzi = Dhatu::new("kzi\\", Tudadi);
    assert_has_parasmai_tinanta(&[], &kzi, Lit, Prathama, Dvi, &["cikziyatuH"]);
    assert_has_parasmai_tinanta(&[], &kzi, Lit, Prathama, Bahu, &["cikziyuH"]);
    // saMyogAdeH
    let kf = Dhatu::new("qukf\\Y", Tanadi);
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
    let fch = Dhatu::new("fCa~", Tudadi);
    assert_has_parasmai_tinanta(&[], &fch, Lit, Prathama, Eka, &["AnarCa", "AnarcCa"]);
    assert_has_parasmai_tinanta(&[], &fch, Lit, Prathama, Dvi, &["AnarCatuH", "AnarcCatuH"]);
    assert_has_parasmai_tinanta(&[], &fch, Lit, Prathama, Bahu, &["AnarCuH", "AnarcCuH"]);

    let f = Dhatu::new("f\\", Bhvadi);
    assert_has_parasmai_tinanta(&[], &f, Lit, Prathama, Eka, &["Ara"]);
    assert_has_parasmai_tinanta(&[], &f, Lit, Prathama, Dvi, &["AratuH"]);
    assert_has_parasmai_tinanta(&[], &f, Lit, Prathama, Bahu, &["AruH"]);

    let kff = Dhatu::new("kF", Tudadi);
    assert_has_parasmai_tinanta(&["ni"], &kff, Lit, Prathama, Dvi, &["nicakaratuH"]);
    assert_has_parasmai_tinanta(&["ni"], &kff, Lit, Prathama, Bahu, &["nicakaruH"]);

    let gff = Dhatu::new("gF", Tudadi);
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
    let shf = Dhatu::new("SFY", Kryadi);
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

    let shf2 = Dhatu::new("SF", Kryadi);
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

    let df = Dhatu::new("dF", Kryadi);
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

    let pf = Dhatu::new("pF", Juhotyadi);
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
fn sutra_7_4_23() {
    let uh = Dhatu::new("Uha~\\", Bhvadi);
    assert_has_lat_karmani(&["sam"], &uh, &["samuhyate"]);
    assert_has_krdanta(&["sam"], &uh, Krt::ktvA, &["samuhya"]);
    assert_has_lat_karmani(&["aBi"], &uh, &["aByuhyate"]);
    assert_has_krdanta(&["aBi"], &uh, Krt::ktvA, &["aByuhya"]);

    assert_has_lat_karmani(&[], &uh, &["Uhyate"]);

    let ih = Dhatu::new("Iha~\\", Bhvadi);
    assert_has_lat_karmani(&["sam"], &ih, &["samIhyate"]);
    assert_has_krdanta(&["sam"], &uh, Krt::kta, &["samUhita"]);

    assert_has_lat_karmani(&["AN"], &uh, &["ohyate"]);
    assert_has_lat_karmani(&["sam", "AN"], &uh, &["samohyate"]);
}

#[test]
fn sutra_7_4_24() {
    let i = Dhatu::new("i\\R", Adadi);
    assert_has_ashirlin(&["ud"], &i, &["udiyAt"]);
    assert_has_ashirlin(&["sam"], &i, &["samiyAt"]);
    assert_has_ashirlin(&["anu"], &i, &["anviyAt"]);

    assert_has_ashirlin(&[], &i, &["IyAt"]);

    assert_has_ashirlin(&["AN"], &i, &["eyAt"]);
    assert_has_ashirlin(&["sam", "AN"], &i, &["sameyAt"]);
}

#[test]
fn sutra_7_4_28() {
    let d = Dhatu::new;

    let kf = Dhatu::new("qukf\\Y", Tanadi);
    let hf = Dhatu::new("hf\\Y", Bhvadi);
    let bhf = Dhatu::new("quBf\\Y", Juhotyadi);

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
    let f = Dhatu::new("f\\", Bhvadi);
    assert_has_lat_karmani(&[], &f, &["aryate"]);
    assert_has_ashirlin_p(&[], &f, &["aryAt"]);

    let smf = Dhatu::new("smf", Bhvadi);
    assert_has_lat_karmani(&[], &smf, &["smaryate"]);
    assert_has_ashirlin_p(&[], &smf, &["smaryAt"]);

    let kf = Dhatu::new("qukf\\Y", Tanadi);
    assert_has_lat_karmani(&["sam"], &kf, &["saMskriyate", "saNkriyate"]);
    assert_has_ashirlin_p(&["sam"], &kf, &["saMskriyAt", "saNkriyAt"]);

    // TODO: also derives svarizIzwa and svArizIzwa, which I need to check.
    /*
    let svf = Dhatu::new("svf", Bhvadi);
    assert_has_ashirlin_karmani(&[], &svf, &["svfzIzwa"]);
    */
}

#[test]
fn sutra_7_4_59() {
    let san = |dhatu: &Dhatu| dhatu.clone().with_sanadi(&[Sanadi::San]);
    let nic = |dhatu: &Dhatu| dhatu.clone().with_sanadi(&[Sanadi::Nic]);

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
    let san = |u, g| d(u, g).with_sanadi(&[Sanadi::San]);
    assert_has_lat(
        &[],
        &san("Scyuti~r", Bhvadi),
        &["cuScyotizati", "cuScyutizati"],
    );
    assert_has_lat_p(&[], &san("zWA\\", Bhvadi), &["tizWAsati"]);
    assert_has_lat(&[], &san("spadi~\\", Bhvadi), &["pispandizate"]);
    // SarpUrva
    assert_has_lit_p(&[], &d("qupa\\ca~^z", Bhvadi), &["papAca"]);
    // KayaH
    assert_has_lit(&[], &d("zRE\\", Bhvadi), &["sasnO"]);
}

#[test]
fn sutra_7_4_62() {
    let san = |u, g| d(u, g).with_sanadi(&[Sanadi::San]);
    assert_has_lit_p(&[], &d("qukf\\Y", Tanadi), &["cakAra"]);
    assert_has_lit_p(&[], &d("Kanu~^", Bhvadi), &["caKAna"]);
    assert_has_lit_p(&[], &d("ga\\mx~", Bhvadi), &["jagAma"]);
    assert_has_lit_p(&[], &d("ha\\na~", Adadi), &["jaGAna"]);
    assert_has_lit_p(&[], &d("hf\\Y", Bhvadi), &["jahAra"]);
    assert_has_lat_p(&[], &san("hf\\Y", Bhvadi), &["jihIrzati"]);
    assert_has_lit_p(&[], &d("o~hA\\k", Juhotyadi), &["jahO"]);
}

#[test]
fn sutra_7_4_63() {
    let yan = |u, g| d(u, g).with_sanadi(&[Sanadi::Yan]);
    assert_has_lat(&[], &yan("ku\\N", Bhvadi), &["kokUyate"]);
    // only for kuN-bhvAdi
    assert_has_lat(&[], &yan("ku\\", Adadi), &["cokUyate"]);
    assert_has_lat(&[], &yan("ku\\N", Tudadi), &["cokUyate"]);
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
    let yan = |dhatu: &Dhatu| dhatu.clone().with_sanadi(&[Sanadi::Yan]);
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

#[ignore]
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
    assert_has_lat_p(&[], &d("lUY", Kryadi), &["lulUzati"]);
    // taparakaraNa
    let pac_yan_san = pac.with_sanadi(&[Sanadi::Yan, Sanadi::San]);
    assert_has_lat_a(&[], &pac_yan_san, &["pApacizate"]);
}
