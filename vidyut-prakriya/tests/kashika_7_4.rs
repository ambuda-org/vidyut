extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::*;

fn yan_san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Yan, Sanadi::San])
}

fn nic_san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Nic, Sanadi::San])
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

#[ignore]
#[test]
fn sutra_7_4_4() {
    let paa_nic = nic(&d("pA\\", Bhvadi));
    assert_has_tip(&[], &paa_nic, Lun, &["apIpyat"]);
    assert_has_tas(&[], &paa_nic, Lun, &["apIpyatAm"]);
    assert_has_jhi(&[], &paa_nic, Lun, &["apIpyan"]);
}

#[test]
fn sutra_7_4_5() {
    let stha_nic = nic(&d("zWA\\", Bhvadi));
    assert_has_tip(&[], &stha_nic, Lun, &["atizWipat"]);
    assert_has_tas(&[], &stha_nic, Lun, &["atizWipatAm"]);
    assert_has_jhi(&[], &stha_nic, Lun, &["atizWipan"]);
}

#[test]
fn sutra_7_4_6() {
    let ghra_nic = nic(&d("GrA\\", Bhvadi));
    assert_has_tip(&[], &ghra_nic, Lun, &["ajiGripat", "ajiGrapat"]);
    assert_has_tas(&[], &ghra_nic, Lun, &["ajiGripatAm", "ajiGrapatAm"]);
    assert_has_jhi(&[], &ghra_nic, Lun, &["ajiGripan", "ajiGrapan"]);
}

#[test]
fn sutra_7_4_7() {
    assert_has_lun_p(&[], &d("kFta~", Curadi), &["acikIrtat", "acIkftat"]);
    assert_has_lun_p(&[], &nic(&d("vftu~\\", Bhvadi)), &["avavartat", "avIvftat"]);
    assert_has_lun_p(&[], &d("mfjU~", Curadi), &["amamArjat", "amImfjat"]);
}

#[test]
fn sutra_7_4_9() {
    let de = d("de\\N", Bhvadi);
    assert_has_ta(&["ava"], &de, Lit, &["avadigye"]);
    assert_has_aataam(&["ava"], &de, Lit, &["avadigyAte"]);
    assert_has_jha(&["ava"], &de, Lit, &["avadigyire"]);

    // TODO: show that `day` is not included.
    // let day = d("daya~\\", Bhvadi);
}

#[test]
fn sutra_7_4_10() {
    let svf = d("svf", Bhvadi);
    assert_has_tas(&[], &svf, Lit, &["sasvaratuH"]);
    assert_has_jhi(&[], &svf, Lit, &["sasvaruH"]);

    let dhvf = d("Dvf\\", Bhvadi);
    assert_has_tas(&[], &dhvf, Lit, &["daDvaratuH"]);
    assert_has_jhi(&[], &dhvf, Lit, &["daDvaruH"]);

    let smf = d("smf", Bhvadi);
    assert_has_tas(&[], &smf, Lit, &["sasmaratuH"]);
    assert_has_jhi(&[], &smf, Lit, &["sasmaruH"]);

    // ftaH
    let kzi = d("kzi\\", Tudadi);
    assert_has_tas(&[], &kzi, Lit, &["cikziyatuH"]);
    assert_has_jhi(&[], &kzi, Lit, &["cikziyuH"]);
    // saMyogAdeH
    let kf = d("qukf\\Y", Tanadi);
    assert_has_tas(&[], &kf, Lit, &["cakratuH"]);
    assert_has_jhi(&[], &kf, Lit, &["cakruH"]);
    // vrddhi otherwise
    assert_has_lit_p(&[], &svf, &["sasvAra"]);
    assert_has_lit_p(&[], &smf, &["sasmAra"]);
    // liwi
    assert_has_krdanta(&[], &smf, Krt::kta, &["smfta"]);
    assert_has_krdanta(&[], &smf, Krt::ktavatu, &["smftavat"]);
    // saMskf
    assert_has_tas(&["sam"], &kf, Lit, &["saYcaskratuH", "saYcakratuH"]);
    assert_has_jhi(&["sam"], &kf, Lit, &["saYcaskruH", "saYcakruH"]);
}

#[test]
fn sutra_7_4_11() {
    let fch = d("fCa~", Tudadi);
    assert_has_tip(&[], &fch, Lit, &["AnarCa", "AnarcCa"]);
    assert_has_tas(&[], &fch, Lit, &["AnarCatuH", "AnarcCatuH"]);
    assert_has_jhi(&[], &fch, Lit, &["AnarCuH", "AnarcCuH"]);

    let f = d("f\\", Bhvadi);
    assert_has_tip(&[], &f, Lit, &["Ara"]);
    assert_has_tas(&[], &f, Lit, &["AratuH"]);
    assert_has_jhi(&[], &f, Lit, &["AruH"]);

    let kff = d("kF", Tudadi);
    assert_has_tas(&["ni"], &kff, Lit, &["nicakaratuH"]);
    assert_has_jhi(&["ni"], &kff, Lit, &["nicakaruH"]);

    let gff = d("gF", Tudadi);
    assert_has_tas(&["ni"], &gff, Lit, &["nijagaratuH", "nijagalatuH"]);
    assert_has_jhi(&["ni"], &gff, Lit, &["nijagaruH", "nijagaluH"]);

    // vrddhi
    assert_has_lit(&["ni"], &kff, &["nicakAra"]);
    assert_has_lit(&["ni"], &gff, &["nijagAra", "nijagAla"]);
}

#[test]
fn sutra_7_4_12() {
    let shf = d("SFY", Kryadi);
    assert_has_tas(&["vi"], &shf, Lit, &["viSaSratuH", "viSaSaratuH"]);
    assert_has_jhi(&["vi"], &shf, Lit, &["viSaSruH", "viSaSaruH"]);

    let shf2 = d("SF", Kryadi);
    assert_has_tas(&["vi"], &shf2, Lit, &["viSaSratuH", "viSaSaratuH"]);
    assert_has_jhi(&["vi"], &shf2, Lit, &["viSaSruH", "viSaSaruH"]);

    let df = d("dF", Kryadi);
    assert_has_tas(&["vi"], &df, Lit, &["vidadratuH", "vidadaratuH"]);
    assert_has_jhi(&["vi"], &df, Lit, &["vidadruH", "vidadaruH"]);

    let pf = d("pF", Juhotyadi);
    assert_has_tas(&["ni"], &pf, Lit, &["nipapratuH", "nipaparatuH"]);
    assert_has_jhi(&["ni"], &pf, Lit, &["nipapruH", "nipaparuH"]);
}

#[test]
fn sutra_7_4_16() {
    assert_has_lun(&[], &d("sf\\", Juhotyadi), &["asarat"]);
    assert_has_lun(&[], &d("f\\", Juhotyadi), &["Arat"]);

    let dfs = d("df\\Si~r", Bhvadi);
    assert_has_tip(&[], &dfs, Lun, &["adarSat", "adrAkzIt"]);
    assert_has_tas(&[], &dfs, Lun, &["adarSatAm", "adrAzwAm"]);
    assert_has_jhi(&[], &dfs, Lun, &["adarSan", "adrAkzuH"]);
}

#[test]
fn sutra_7_4_17() {
    let asu = d("asu~", Divadi);
    assert_has_tip(&[], &asu, Lun, &["AsTat"]);
    assert_has_tas(&[], &asu, Lun, &["AsTatAm"]);
    assert_has_jhi(&[], &asu, Lun, &["AsTan"]);
}

#[test]
fn sutra_7_4_18() {
    let svi = d("wuo~Svi", Bhvadi);
    assert_has_tip(&[], &svi, Lun, &["aSvat", "aSvayIt", "aSiSviyat"]);
    assert_has_tas(&[], &svi, Lun, &["aSvatAm", "aSvayizwAm", "aSiSviyatAm"]);
    assert_has_jhi(&[], &svi, Lun, &["aSvan", "aSvayizuH", "aSiSviyan"]);
}

#[test]
fn sutra_7_4_19() {
    let pat = d("patx~", Bhvadi);
    assert_has_tip(&[], &pat, Lun, &["apaptat"]);
    assert_has_tas(&[], &pat, Lun, &["apaptatAm"]);
    assert_has_jhi(&[], &pat, Lun, &["apaptan"]);
}

#[test]
fn sutra_7_4_20() {
    let vac = d("va\\ca~", Adadi);
    assert_has_tip(&[], &vac, Lun, &["avocat"]);
    assert_has_tas(&[], &vac, Lun, &["avocatAm"]);
    assert_has_jhi(&[], &vac, Lun, &["avocan"]);
}

#[test]
fn sutra_7_4_21() {
    let shi = d("SIN", Adadi);
    assert_has_ta(&[], &shi, Lat, &["Sete"]);
    assert_has_aataam(&[], &shi, Lat, &["SayAte"]);
    assert_has_jha(&[], &shi, Lat, &["Serate"]);
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
fn sutra_7_4_25() {
    // TODO: others
    let ci = d("ci\\Y", Svadi);
    assert_has_lat_karmani(&[], &ci, &["cIyate"]);
    assert_has_lat(&[], &yan(&ci), &["cecIyate"]);
    let stu = d("zwu\\Y", Adadi);
    assert_has_lat_karmani(&[], &stu, &["stUyate"]);
    assert_has_lat(&[], &yan(&stu), &["tozwUyate"]);

    assert_has_ashirlin_p(&[], &ci, &["cIyAt"]);
    assert_has_ashirlin_p(&[], &stu, &["stUyAt"]);

    // a-krti?
    assert_has_krdanta(&["pra"], &d("qukf\\Y", Tanadi), Krt::ktvA, &["prakftya"]);
    assert_has_krdanta(&["pra"], &d("hf\\Y", Bhvadi), Krt::ktvA, &["prahftya"]);

    // a-sArvadhAtuke?
    assert_has_vidhilin_p(&[], &ci, &["cinuyAt"]);
    assert_has_vidhilin_p(&[], &d("zu\\Y", Svadi), &["sunuyAt"]);
}

#[test]
fn sutra_7_4_26() {
    use TaddhitaArtha::AbhutaTadbhava;
    assert_has_artha_taddhita("Suca", AbhutaTadbhava, T::cvi, &["SucI"]);
    assert_has_artha_taddhita("pawu", AbhutaTadbhava, T::cvi, &["pawU"]);
}

#[test]
fn sutra_7_4_32() {
    use TaddhitaArtha::AbhutaTadbhava;
    assert_has_artha_taddhita("Sukla", AbhutaTadbhava, T::cvi, &["SuklI"]);
    assert_has_artha_taddhita("KawvA", AbhutaTadbhava, T::cvi, &["KawvI"]);
}

#[test]
fn sutra_7_4_27() {
    // TODO: others

    assert_has_lat(&[], &yan(&d("qukf\\Y", Tanadi)), &["cekrIyate"]);
    assert_has_lat(&[], &yan(&d("kF", Tudadi)), &["cekIryate"]);
    assert_has_lat(&["ni"], &yan(&d("gF", Tudadi)), &["nijegilyate"]);
}

#[test]
fn sutra_7_4_28() {
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
fn sutra_7_4_30_v1() {
    let han = d("ha\\na~", Adadi);
    assert_has_ta(&[], &yan(&han), Lat, &["jeGnIyate", "jaNGanyate"]);
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
fn sutra_7_4_42() {
    let dhaa = d("quDA\\Y", Juhotyadi);
    assert_has_krdanta(&[], &dhaa, Krt::kta, &["hita"]);
    assert_has_krdanta(&[], &dhaa, Krt::ktavatu, &["hitavat"]);
    assert_has_krdanta(&[], &dhaa, Krt::ktvA, &["hitvA"]);
}

#[test]
fn sutra_7_4_43() {
    assert_has_krdanta(&[], &d("o~hA\\k", Juhotyadi), Krt::ktvA, &["hitvA"]);
    assert_has_krdanta(&[], &d("o~hA\\N", Juhotyadi), Krt::ktvA, &["hAtvA"]);
}

#[test]
fn sutra_7_4_46() {
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_krdanta(&[], &daa, Krt::kta, &["datta"]);
    assert_has_krdanta(&[], &daa, Krt::ktavatu, &["dattavat"]);
    assert_has_krdanta(&[], &daa, Krt::ktin, &["datti"]);

    // daH?
    let dhe = d("De\\w", Juhotyadi);
    assert_has_krdanta(&[], &dhe, Krt::kta, &["DIta"]);
    assert_has_krdanta(&[], &dhe, Krt::ktavatu, &["DItavat"]);

    // GoH?
    assert_has_krdanta(&[], &d("dA\\p", Adadi), Krt::kta, &["dAta"]);
    assert_has_krdanta(&["ava"], &d("dE\\p", Bhvadi), Krt::kta, &["avadAta"]);
}

#[test]
fn sutra_7_4_47() {
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_krdanta(&["pra"], &daa, Krt::kta, &["pratta"]);
    assert_has_krdanta(&["ava"], &daa, Krt::kta, &["avatta"]);
    assert_has_krdanta(&["ni"], &daa, Krt::kta, &["nItta"]);
    assert_has_krdanta(&["pari"], &daa, Krt::kta, &["parItta"]);
    // acaH?
    assert_has_krdanta(&["nir"], &daa, Krt::kta, &["nirdatta"]);
    assert_has_krdanta(&["dur"], &daa, Krt::kta, &["durdatta"]);
    // upasargAt?
    assert_has_krdanta(&[], &daa, Krt::kta, &["datta"]);
    // GoH?
    assert_has_krdanta(&["ava"], &d("dA\\p", Adadi), Krt::kta, &["avadAta"]);
}

#[test]
fn sutra_7_4_48() {
    assert_has_sup_3p("ap", Pum, &["adBiH"]);
    assert_has_sup_4p("ap", Pum, &["adByaH"]);
    // BiH?
    assert_has_sup_7p("ap", Pum, &["apsu"]);
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
    assert_has_thaas(&[], &d("Asa~\\", Adadi), Lat, &["Asse"]);
    assert_has_thaas(&[], &d("vasa~\\", Adadi), Lat, &["vasse"]);
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
    assert_has_iw(&[], &kf, Lut, &["kartAhe"]);
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
fn sutra_7_4_58() {
    let qauk = d("QOkf~\\", Bhvadi);
    let trauk = d("trOkf~\\", Bhvadi);
    assert_has_lat(&[], &san(&qauk), &["quQOkizate"]);
    assert_has_lat(&[], &san(&trauk), &["tutrOkizate"]);
    // caNi
    assert_has_lun_p(&[], &nic(&d("mI\\Y", Kryadi)), &["amImapat"]);
    assert_has_lun_p(&[], &nic(&d("qudA\\Y", Juhotyadi)), &["adIdapat"]);
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
    assert_has_tip(&[], &aw, Lit, &["Awa"]);
    assert_has_tas(&[], &aw, Lit, &["AwatuH"]);
    assert_has_jhi(&[], &aw, Lit, &["AwuH"]);
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

#[test]
fn sutra_7_4_67() {
    let dyut = d("dyuta~\\", Bhvadi);
    assert_has_lit(&["vi"], &dyut, &["vididyute"]);
    assert_has_lun_p(&["vi"], &nic(&dyut), &["vyadidyutat"]);
    assert_has_lat(&["vi"], &san(&dyut), &["vididyotizate", "vididyutizate"]);
    assert_has_lat(&["vi"], &yan(&dyut), &["videdyutyate"]);

    // TODO
    let _svap = d("Yizva\\pa~", Adadi).with_sanadi(&[Sanadi::Nic, Sanadi::San]);
    // assert_has_lat_p(&[], &svap, &["suzvApayizati"]);
}

#[test]
fn sutra_7_4_68() {
    let vyath = d("vyaTa~\\", Bhvadi);
    assert_has_ta(&[], &vyath, Lit, &["vivyaTe"]);
    assert_has_aataam(&[], &vyath, Lit, &["vivyaTAte"]);
    assert_has_jha(&[], &vyath, Lit, &["vivyaTire"]);
    // liti
    assert_has_lat(&[], &yan(&vyath), &["vAvyaTyate"]);
}

#[test]
fn sutra_7_4_69() {
    let i = d("i\\R", Adadi);
    assert_has_tas(&[], &i, Lit, &["IyatuH"]);
    assert_has_jhi(&[], &i, Lit, &["IyuH"]);
    // kiti
    assert_has_tip(&[], &i, Lit, &["iyAya"]);
    assert_has_sip(&[], &i, Lit, &["iyayiTa", "iyeTa"]);
}

#[test]
fn sutra_7_4_70() {
    let aw = d("awa~", Bhvadi);
    assert_has_tip(&[], &aw, Lit, &["Awa"]);
    assert_has_tas(&[], &aw, Lit, &["AwatuH"]);
    assert_has_jhi(&[], &aw, Lit, &["AwuH"]);
    // AdeH
    assert_has_lit_p(&[], &d("qupa\\ca~^z", Bhvadi), &["papAca"]);
    assert_has_lit_p(&[], &d("paWa~", Bhvadi), &["papAWa"]);
}

#[test]
fn sutra_7_4_71() {
    let ang = d("agi~", Bhvadi);
    assert_has_tip(&[], &ang, Lit, &["AnaNga"]);
    assert_has_tas(&[], &ang, Lit, &["AnaNgatuH"]);
    assert_has_jhi(&[], &ang, Lit, &["AnaNguH"]);
    let anj = d("anjU~", Rudhadi);
    assert_has_tip(&[], &anj, Lit, &["AnaYja"]);
    assert_has_tas(&[], &anj, Lit, &["AnaYjatuH"]);
    assert_has_jhi(&[], &anj, Lit, &["AnaYjuH"]);
    // dvihalaH
    let aw = d("awa~", Bhvadi);
    assert_has_tip(&[], &aw, Lit, &["Awa"]);
    assert_has_tas(&[], &aw, Lit, &["AwatuH"]);
    assert_has_jhi(&[], &aw, Lit, &["AwuH"]);
    // TODO: AnfDatuH
}

#[test]
fn sutra_7_4_72() {
    let ashnoti = d("aSU~\\", Svadi);
    assert_has_tinanta(&["vi"], &ashnoti, Lit, Prathama, Eka, &["vyAnaSe"]);
    assert_has_tinanta(&["vi"], &ashnoti, Lit, Prathama, Dvi, &["vyAnaSAte"]);
    assert_has_tinanta(&["vi"], &ashnoti, Lit, Prathama, Bahu, &["vyAnaSire"]);
    // aSnAti
    let ashnati = d("aSa~", Kryadi);
    assert_has_tip(&[], &ashnati, Lit, &["ASa"]);
    assert_has_tas(&[], &ashnati, Lit, &["ASatuH"]);
    assert_has_jhi(&[], &ashnati, Lit, &["ASuH"]);
}

#[test]
fn sutra_7_4_73() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &bhu, Lit, &["baBUva"]);
    assert_has_tas(&[], &bhu, Lit, &["baBUvatuH"]);
    assert_has_jhi(&[], &bhu, Lit, &["baBUvuH"]);
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

#[test]
fn sutra_7_4_80() {
    // pu
    let pu = d("pUN", Bhvadi);
    let bhu = d("BU", Bhvadi);
    assert_has_lat(&[], &san(&pu), &["pipavizate"]);
    assert_has_lat_p(&[], &nic_san(&pu), &["pipAvayizati"]);
    assert_has_lat_p(&[], &nic_san(&bhu), &["biBAvayizati"]);
    let yu = d("yu", Adadi);
    let ru = d("ru", Adadi);
    let lu = d("lUY", Kryadi);
    assert_has_lat(&[], &san(&yu), &["yiyavizati", "yuyUzati"]);
    assert_has_lat_p(&[], &nic_san(&yu), &["yiyAvayizati"]);
    assert_has_lat_p(&[], &nic_san(&ru), &["rirAvayizati"]);
    assert_has_lat_p(&[], &nic_san(&lu), &["lilAvayizati"]);
    // ju (sautro dhatu)
    let ju = d("ju", Adadi);
    assert_has_lat_p(&[], &nic_san(&ju), &["jijAvayizati"]);
    // oH
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_lat_a(&[], &yan(&pac), &["pApacyate"]);
    assert_has_lat_a(&[], &yan_san(&pac), &["pApacizate"]);
    // puyaNjyapare
    let tu = d("tu\\", Adadi);
    assert_has_lat_p(&["ava"], &nic_san(&tu), &["avatutAvayizati"]);
    assert_has_lat_p(&[], &nic_san(&d("hu\\", Juhotyadi)), &["juhAvayizati"]);
    assert_has_lat(&[], &san(&bhu), &["buBUzati"]);
}

#[test]
fn sutra_7_4_81() {
    let sru = &d("sru\\", Bhvadi);
    let shru = &d("Sru\\", Bhvadi);
    assert_has_lat_p(&[], &nic_san(&sru), &["sisrAvayizati", "susrAvayizati"]);
    assert_has_lat_p(&[], &nic_san(&shru), &["SiSrAvayizati", "SuSrAvayizati"]);
    assert_has_lat_p(
        &[],
        &nic_san(&d("dru\\", Bhvadi)),
        &["didrAvayizati", "dudrAvayizati"],
    );
    assert_has_lat_p(
        &[],
        &nic_san(&d("pru\\N", Bhvadi)),
        &["piprAvayizati", "puprAvayizati"],
    );
    assert_has_lat_p(
        &[],
        &nic_san(&d("plu\\N", Bhvadi)),
        &["piplAvayizati", "puplAvayizati"],
    );
    assert_has_lat_p(
        &[],
        &nic_san(&d("cyu\\N", Bhvadi)),
        &["cicyAvayizati", "cucyAvayizati"],
    );
    // apare?
    assert_has_lat_p(&[], &san(&sru), &["susrUzati"]);
    assert_has_lat_a(&[], &san(&shru), &["SuSrUzate"]);
}

#[test]
fn sutra_7_4_82() {
    assert_has_lat(&[], &yan(&d("ci\\Y", Svadi)), &["cecIyate"]);
    assert_has_lat(&[], &yan(&d("lUY", Kryadi)), &["lolUyate"]);
    assert_has_lat(
        &[],
        &yan_luk(&d("hu\\", Juhotyadi)),
        &["johavIti", "johoti"],
    );
    assert_has_lat(
        &[],
        &yan_luk(&d("kru\\Sa~", Bhvadi)),
        &["cokruSIti", "cokrozwi"],
    );
}

#[test]
fn sutra_7_4_83() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_lat_a(&[], &yan(&pac), &["pApacyate"]);
    assert_has_lat(&[], &yan_luk(&pac), &["pApacIti", "pApakti"]);
    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_lat_a(&[], &yan(&yaj), &["yAyajyate"]);
    assert_has_lat(&[], &yan_luk(&yaj), &["yAyajIti", "yAyazwi"]);
}

#[test]
fn sutra_7_4_84() {
    let vanc = d("vancu~", Bhvadi);
    assert_has_lat(&[], &yan(&vanc), &["vanIvacyate"]);
    assert_has_lat(&[], &yan_luk(&vanc), &["vanIvaYcIti", "vanIvaNkti"]);

    let srans = d("sransu~\\", Bhvadi);
    assert_has_lat(&[], &yan(&srans), &["sanIsrasyate"]);
    assert_has_lat(&[], &yan_luk(&srans), &["sanIsraMsIti", "sanIsraMsti"]);

    let dhvans = d("Dvansu~\\", Bhvadi);
    assert_has_lat(&[], &yan(&dhvans), &["danIDvasyate"]);
    assert_has_lat(&[], &yan_luk(&dhvans), &["danIDvaMsIti", "danIDvaMsti"]);

    let bhrans = d("Bransu~\\", Bhvadi);
    assert_has_lat(&[], &yan(&bhrans), &["banIBrasyate"]);
    assert_has_lat(&[], &yan_luk(&bhrans), &["banIBraMsIti", "banIBraMsti"]);

    let kas = d("kasa~", Bhvadi);
    assert_has_lat(&[], &yan(&kas), &["canIkasyate"]);
    assert_has_lat(&[], &yan_luk(&kas), &["canIkasIti", "canIkasti"]);

    let pat = d("patx~", Bhvadi);
    assert_has_lat(&[], &yan(&pat), &["panIpatyate"]);
    assert_has_lat(&[], &yan_luk(&pat), &["panIpatIti", "panIpatti"]);

    let pad = d("pa\\da~\\", Divadi);
    assert_has_lat(&[], &yan(&pad), &["panIpadyate"]);
    assert_has_lat(&[], &yan_luk(&pad), &["panIpadIti", "panIpatti"]);

    let skand = d("ska\\ndi~r", Bhvadi);
    assert_has_lat(&[], &yan(&skand), &["canIskadyate"]);
    assert_has_lat(
        &[],
        &yan_luk(&skand),
        &["canIskandIti", "canIskantti", "canIskanti"],
    );
}

#[test]
fn sutra_7_4_85() {
    let tan = d("tanu~^", Tanadi);
    assert_has_lat(&[], &yan(&tan), &["tantanyate"]);
    assert_has_lat(&[], &yan_luk(&tan), &["tantanIti", "tantanti"]);

    let gam = d("ga\\mx~", Bhvadi);
    assert_has_lat(&[], &yan(&gam), &["jaNgamyate"]);
    assert_has_lat(&[], &yan_luk(&gam), &["jaNgamIti", "jaNganti"]);

    let yam = d("ya\\ma~", Bhvadi);
    assert_has_lat(&[], &yan(&yam), &["yaMyamyate"]);
    assert_has_lat(&[], &yan_luk(&yam), &["yaMyamIti", "yaMyanti"]);

    let ram = d("ra\\ma~\\", Bhvadi);
    assert_has_lat(&[], &yan(&ram), &["raMramyate"]);
    assert_has_lat(&[], &yan_luk(&ram), &["raMramIti", "raMranti"]);

    // ataH
    let tim = d("tima~", Bhvadi);
    assert_has_lat(&[], &yan(&tim), &["tetimyate"]);

    // TODO: taparakaraNa
    // let bham = d("BAma", Curadi);
    // assert_has_lat(&[], &yan(&bham), &["bABamyate"]);

    // anunAsikAntasya
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_lat(&[], &yan(&pac), &["pApacyate"]);
}

#[test]
fn sutra_7_4_86() {
    let jap = d("japa~", Bhvadi);
    assert_has_lat(&[], &yan(&jap), &["jaYjapyate"]);
    assert_has_lat(&[], &yan_luk(&jap), &["jaYjapIti", "jaYjapti"]);

    let jabh = d("jaBI~\\", Bhvadi);
    assert_has_lat(&[], &yan(&jabh), &["jaYjaByate"]);
    assert_has_lat(&[], &yan_luk(&jabh), &["jaYjaBIti", "jaYjabDi"]);

    let dah = d("da\\ha~", Bhvadi);
    assert_has_lat(&[], &yan(&dah), &["dandahyate"]);
    assert_has_lat(&[], &yan_luk(&dah), &["dandahIti", "dandagDi"]);

    let dash = d("da\\nSa~", Bhvadi);
    assert_has_lat(&[], &yan(&dash), &["dandaSyate"]);
    assert_has_lat(&[], &yan_luk(&dash), &["dandaSIti", "dandazwi"]);

    let bhanj = d("Ba\\njo~", Rudhadi);
    assert_has_lat(&[], &yan(&bhanj), &["bamBajyate"]);
    assert_has_lat(&[], &yan_luk(&bhanj), &["bamBaYjIti", "bamBaNkti"]);

    let pash = d("paSa~", Bhvadi);
    assert_has_lat(&[], &yan(&pash), &["pampaSyate"]);
    assert_has_lat(&[], &yan_luk(&pash), &["pampaSIti", "pampazwi"]);
}

#[ignore]
#[test]
fn sutra_7_4_87_and_sutra_7_4_88() {
    let car = d("cara~", Bhvadi);
    assert_has_lat(&[], &yan(&car), &["caYcUryate"]);
    // TODO: caYcUrIti or caYcurIti?
    assert_has_lat(&[], &yan_luk(&car), &["caYcurIti", "caYcUrti"]);

    let phal = d("Pala~", Bhvadi);
    assert_has_lat(&[], &yan(&phal), &["pamPulyate"]);
    assert_has_lat(&[], &yan_luk(&phal), &["pamPulIti", "pamPulti"]);
}

#[test]
fn sutra_7_4_90() {
    let vft = d("vftu~\\", Bhvadi);
    assert_has_lat(&[], &yan(&vft), &["varIvftyate"]);
    assert_has_lat(
        &[],
        &yan_luk(&vft),
        &[
            "varIvarti",
            "varIvartti",
            "varIvftIti",
            "varivarti",
            "varivartti",
            "varivftIti",
            "varvarti",
            "varvartti",
            "varvftIti",
        ],
    );

    let vfdh = d("vfDU~\\", Bhvadi);
    assert_has_lat(&[], &yan(&vfdh), &["varIvfDyate"]);
    assert_has_lat(
        &[],
        &yan_luk(&vfdh),
        &[
            "varIvarDi",
            "varIvardDi",
            "varIvfDIti",
            "varivarDi",
            "varivardDi",
            "varivfDIti",
            "varvarDi",
            "varvardDi",
            "varvfDIti",
        ],
    );

    let nft = d("nftI~", Divadi);
    assert_has_lat(&[], &yan(&nft), &["narInftyate"]);
    assert_has_lat(
        &[],
        &yan_luk(&nft),
        &[
            "narInarti",
            "narInartti",
            "narInftIti",
            "narinarti",
            "narinartti",
            "narinftIti",
            "narnarti",
            "narnartti",
            "narnftIti",
        ],
    );
}

#[test]
fn sutra_7_4_90_v1() {
    assert_has_lat(&[], &yan(&d("o~vrascU~", Tudadi)), &["varIvfScyate"]);
    assert_has_lat(&[], &yan(&d("pra\\Ca~", Tudadi)), &["parIpfcCyate"]);
}

#[test]
fn sutra_7_4_91() {
    assert_has_lat(
        &[],
        &yan_luk(&d("nftI~", Divadi)),
        &[
            "narnartti",
            "narinartti",
            "narInartti",
            "narnarti",
            "narinarti",
            "narInarti",
            "narnftIti",
            "narinftIti",
            "narInftIti",
        ],
    );
    assert_has_lat(
        &[],
        &yan_luk(&d("vftu~\\", Bhvadi)),
        &[
            "varvartti",
            "varivartti",
            "varIvartti",
            "varvarti",
            "varivarti",
            "varIvarti",
            "varvftIti",
            "varivftIti",
            "varIvftIti",
        ],
    );
}

#[ignore]
#[test]
fn sutra_7_4_92() {
    assert_has_lat(
        &[],
        &yan_luk(&d("qukf\\Y", Tanadi)),
        &[
            "carIkarIti",
            "carIkarti",
            "carikarIti",
            "carikarti",
            "carkarIti",
            "carkarti",
        ],
    );
    assert_has_lat(
        &[],
        &yan_luk(&d("hf\\Y", Bhvadi)),
        &[
            "jarIharIti",
            "jarIharti",
            "jarharIti",
            "jarharti",
            "jariharIti",
            "jariharti",
        ],
    );
    // taparakaraNa
    assert_has_lat(&[], &yan_luk(&d("kF", Tudadi)), &["cAkarti"]);
}

#[test]
fn sutra_7_4_93() {
    let pac = &d("qupa\\ca~^z", Bhvadi);

    // sanyataH
    assert_has_lun_p(&[], &nic(&d("qukf\\Y", Tanadi)), &["acIkarat"]);
    assert_has_lun_p(&[], &nic(&pac), &["apIpacat"]);
    // oH puyaRjyapare
    assert_has_lun_p(&[], &nic(&d("pUY", Kryadi)), &["apIpavat"]);
    assert_has_lun_p(&[], &nic(&d("lUY", Kryadi)), &["alIlavat"]);
    // svarati... (7.4.81)
    assert_has_lun_p(&[], &nic(&d("sru\\", Bhvadi)), &["asisravat", "asusravat"]);
    assert_has_lun_p(&[], &nic(&d("Sru\\", Svadi)), &["aSiSravat", "aSuSravat"]);
    assert_has_lun_p(&[], &nic(&d("dru\\", Bhvadi)), &["adidravat", "adudravat"]);
    assert_has_lun_p(&[], &nic(&d("pru\\N", Bhvadi)), &["apipravat", "apupravat"]);
    assert_has_lun_p(&[], &nic(&d("plu\\N", Bhvadi)), &["apiplavat", "apuplavat"]);
    assert_has_lun_p(&[], &nic(&d("cyu\\N", Bhvadi)), &["acicyavat", "acucyavat"]);
    // laghuni
    assert_has_lun_p(&[], &nic(&d("takzU~", Bhvadi)), &["atatakzat"]);
    assert_has_lun_p(&[], &nic(&d("rakza~", Bhvadi)), &["ararakzat"]);
    assert_has_lun_p(&[], &nic(&d("jAgf", Adadi)), &["ajajAgarat"]);
    // caN-pare?
    assert_has_mip(&[], &pac, Lit, &["papaca", "papAca"]);
    // para-grahaRa
    assert_has_lun_a(&[], &d("kamu~\\", Bhvadi), &["acakamata", "acIkamata"]);
    // an-ak-lopa
    assert_has_lun_p(&[], &d("kaTa", Curadi), &["acakaTat"]);
    // TODO: others
}

#[test]
fn sutra_7_4_94() {
    let pac = &d("qupa\\ca~^z", Bhvadi);
    assert_has_lun_p(&[], &nic(&d("qukf\\Y", Tanadi)), &["acIkarat"]);
    assert_has_lun_p(&[], &nic(&d("hf\\Y", Bhvadi)), &["ajIharat"]);
    assert_has_lun_p(&[], &nic(&d("lUY", Kryadi)), &["alIlavat"]);
    assert_has_lun_p(&[], &nic(&pac), &["apIpacat"]);
    // laghoH
    // abiBrajat is allowed by 7.4.3.
    assert_has_lun_p(
        &[],
        &nic(&d("wuBrAjf~\\", Bhvadi)),
        &["abiBrajat", "abaBrAjat"],
    );
    // laghuni
    assert_has_lun_p(&[], &nic(&d("takzU~", Bhvadi)), &["atatakzat"]);
    assert_has_lun_p(&[], &nic(&d("rakza~", Bhvadi)), &["ararakzat"]);
    // caNi
    assert_has_mip(&[], &pac, Lit, &["papaca", "papAca"]);
    // pare
    assert_has_lun_a(&[], &d("kamu~\\", Bhvadi), &["acakamata", "acIkamata"]);
    // anaglopa
    assert_has_lun_p(&[], &d("kaTa", Curadi), &["acakaTat"]);
}

#[test]
fn sutra_7_4_95() {
    assert_has_lun_p(&[], &nic(&d("smf", Bhvadi)), &["asasmarat"]);
    assert_has_lun_p(&[], &nic(&d("dF", Bhvadi)), &["adadarat"]);
    assert_has_lun_p(&[], &nic(&d("YitvarA~\\", Bhvadi)), &["atatvarat"]);
    assert_has_lun_p(&[], &d("praTa~", Curadi), &["apapraTat"]);
    assert_has_lun_p(&[], &nic(&d("mrada~\\", Bhvadi)), &["amamradat"]);
    assert_has_lun_p(&[], &nic(&d("stFY", Kryadi)), &["atastarat"]);
    assert_has_lun_p(&[], &nic(&d("spaSa~^", Bhvadi)), &["apaspaSat"]);
}

#[test]
fn sutra_7_4_96() {
    let vest = d("vezwa~\\", Bhvadi);
    let cest = d("cezwa~\\", Bhvadi);
    assert_has_lun_p(&[], &nic(&vest), &["avavezwat", "avivezwat"]);
    assert_has_lun_p(&[], &nic(&cest), &["acacezwat", "acicezwat"]);
}

#[test]
fn sutra_7_4_97() {
    let gan = d("gaRa", Curadi);
    assert_has_lun_p(&[], &gan, &["ajIgaRat", "ajagaRat"]);
}
