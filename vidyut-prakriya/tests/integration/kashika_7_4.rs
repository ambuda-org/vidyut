extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::*;

fn yan_san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::yaN, Sanadi::san])
}

fn nic_san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Ric, Sanadi::san])
}

fn sanadi(p: Pratipadika, s: Sanadi) -> Dhatu {
    Dhatu::nama(p, Some(s))
}

#[test]
fn sutra_7_4_1() {
    assert_has_tip(&[], &nic(&d("qukf\\Y", Tanadi)), Lun, &["acIkarat"]);
    assert_has_tip(&[], &nic(&d("hf\\Y", Bhvadi)), Lun, &["ajIharat"]);
    assert_has_tip(&[], &nic(&d("lUY", Kryadi)), Lun, &["alIlavat"]);
    assert_has_tip(&[], &nic(&d("pUY", Kryadi)), Lun, &["apIpavat"]);

    // TODO: many more.
}

#[test]
fn sutra_7_4_2() {
    assert_has_tip(&[], &nic(&d("SAsu~", Bhvadi)), Lun, &["aSaSAsat"]);
    assert_has_tip(&[], &nic(&d("bADf~\\", Bhvadi)), Lun, &["ababADat"]);
    assert_has_tip(&[], &nic(&d("quyAcf~^", Bhvadi)), Lun, &["ayayAcat"]);
    assert_has_tip(&[], &nic(&d("QOkf~\\", Bhvadi)), Lun, &["aquQOkat"]);
}

#[test]
fn sutra_7_4_3() {
    assert_has_tip(
        &[],
        &nic(&d("wuBrAjf~\\", Bhvadi)),
        Lun,
        &["abiBrajat", "abaBrAjat"],
    );
    assert_has_tip(
        &[],
        &nic(&d("BAsf~\\", Bhvadi)),
        Lun,
        &["abIBasat", "abaBAsat"],
    );
    assert_has_tip(
        &[],
        &nic(&d("BAza~\\", Bhvadi)),
        Lun,
        &["abIBazat", "abaBAzat"],
    );
    assert_has_tip(
        &[],
        &nic(&d("dIpI~\\", Divadi)),
        Lun,
        &["adIdipat", "adidIpat"],
    );
    assert_has_tip(
        &[],
        &nic(&d("jIva~", Bhvadi)),
        Lun,
        &["ajIjivat", "ajijIvat"],
    );
    assert_has_tip(
        &[],
        &nic(&d("mIla~", Bhvadi)),
        Lun,
        &["amImilat", "amimIlat"],
    );
    assert_has_tip(&[], &d("pIqa~", Curadi), Lun, &["apIpiqat", "apipIqat"]);
}

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
    assert_has_tip(&[], &d("kFta~", Curadi), Lun, &["acikIrtat", "acIkftat"]);
    assert_has_tip(
        &[],
        &nic(&d("vftu~\\", Bhvadi)),
        Lun,
        &["avavartat", "avIvftat"],
    );
    assert_has_tip(
        &[],
        &nic(&d("mfjU~", Adadi)),
        Lun,
        &["amamArjat", "amImfjat"],
    );
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
    assert_has_tip(&[], &svf, Lit, &["sasvAra"]);
    assert_has_tip(&[], &smf, Lit, &["sasmAra"]);

    // liwi
    assert_has_krdanta(&[], &smf, Krt::kta, &["smfta"]);
    assert_has_krdanta(&[], &smf, Krt::ktavatu, &["smftavat"]);

    // saMskf
    assert_has_tas(&["sam"], &kf, Lit, &["saYcaskaratuH", "saYcakratuH"]);
    assert_has_jhi(&["sam"], &kf, Lit, &["saYcaskaruH", "saYcakruH"]);
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
    assert_has_ta_k(&[], &shi, Lat, &["Sayyate"]);
    assert_has_lat(&[], &yan(&shi), &["SASayyate"]);
    assert_has_krdanta(&["pra"], &shi, Krt::ktvA, &["praSayya"]);
    assert_has_krdanta(&["upa"], &shi, Krt::ktvA, &["upaSayya"]);
    // yi
    assert_has_lit(&[], &shi, &["SiSye"]);
}

#[test]
fn sutra_7_4_23() {
    let uh = d("Uha~\\", Bhvadi);
    assert_has_ta_k(&["sam"], &uh, Lat, &["samuhyate"]);
    assert_has_krdanta(&["sam"], &uh, Krt::ktvA, &["samuhya"]);
    assert_has_ta_k(&["aBi"], &uh, Lat, &["aByuhyate"]);
    assert_has_krdanta(&["aBi"], &uh, Krt::ktvA, &["aByuhya"]);

    assert_has_ta_k(&[], &uh, Lat, &["Uhyate"]);

    let ih = d("Iha~\\", Bhvadi);
    assert_has_ta_k(&["sam"], &ih, Lat, &["samIhyate"]);
    assert_has_krdanta(&["sam"], &uh, Krt::kta, &["samUhita"]);

    assert_has_ta_k(&["AN"], &uh, Lat, &["ohyate"]);
    assert_has_ta_k(&["sam", "AN"], &uh, Lat, &["samohyate"]);
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
    assert_has_ta_k(&[], &ci, Lat, &["cIyate"]);
    assert_has_lat(&[], &yan(&ci), &["cecIyate"]);
    let stu = d("zwu\\Y", Adadi);
    assert_has_ta_k(&[], &stu, Lat, &["stUyate"]);
    assert_has_lat(&[], &yan(&stu), &["tozwUyate"]);

    assert_has_tip(&[], &ci, AshirLin, &["cIyAt"]);
    assert_has_tip(&[], &stu, AshirLin, &["stUyAt"]);

    // a-krti?
    assert_has_krdanta(&["pra"], &d("qukf\\Y", Tanadi), Krt::ktvA, &["prakftya"]);
    assert_has_krdanta(&["pra"], &d("hf\\Y", Bhvadi), Krt::ktvA, &["prahftya"]);

    // a-sArvadhAtuke?
    assert_has_tip(&[], &ci, VidhiLin, &["cinuyAt"]);
    assert_has_tip(&[], &d("zu\\Y", Svadi), VidhiLin, &["sunuyAt"]);
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

    assert_has_ta_k(&["AN"], &d("df\\N", Tudadi), Lat, &["Adriyate"]);
    assert_has_ta_k(&["AN"], &d("Df\\Y", Bhvadi), Lat, &["ADriyate"]);
    assert_has_ta_k(&[], &kf, Lat, &["kriyate"]);
    assert_has_ta_k(&[], &hf, Lat, &["hriyate"]);
    assert_has_tip(&[], &kf, AshirLin, &["kriyAt"]);
    assert_has_tip(&[], &hf, AshirLin, &["hriyAt"]);
    assert_has_tip(&[], &bhf, VidhiLin, &["biBfyAt"]);
    assert_has_ta(&[], &kf, AshirLin, &["kfzIzwa"]);
    assert_has_ta(&[], &hf, AshirLin, &["hfzIzwa"]);
}

#[test]
fn sutra_7_4_29() {
    let f = d("f\\", Bhvadi);
    assert_has_ta_k(&[], &f, Lat, &["aryate"]);
    assert_has_tip(&[], &f, AshirLin, &["aryAt"]);

    let smf = d("smf", Bhvadi);
    assert_has_ta_k(&[], &smf, Lat, &["smaryate"]);
    assert_has_tip(&[], &smf, AshirLin, &["smaryAt"]);

    let kf = d("qukf\\Y", Tanadi);
    assert_has_ta_k(&["sam"], &kf, Lat, &["saMskriyate", "saNkriyate"]);
    assert_has_tip(&["sam"], &kf, AshirLin, &["saMskriyAt", "saNkriyAt"]);

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

#[test]
fn sutra_7_4_33() {
    let kyac = |prati| sanadi(phit(prati), Sanadi::kyac);
    assert_has_tip(&[], &kyac("putra"), Lat, &["putrIyati"]);
    assert_has_tip(&[], &kyac("Gawa"), Lat, &["GawIyati"]);
    assert_has_tip(&[], &kyac("KawvA"), Lat, &["KawvIyati"]);
    assert_has_tip(&[], &kyac("mAlA"), Lat, &["mAlIyati"]);
}

#[test]
fn sutra_7_4_40() {
    let do_ = d("do\\", Divadi);
    assert_has_krdanta(&["nis"], &do_, Krt::kta, &["nirdita"]);
    assert_has_krdanta(&["nis"], &do_, Krt::ktavatu, &["nirditavat"]);

    let so = d("zo\\", Divadi);
    assert_has_krdanta(&["ava"], &so, Krt::kta, &["avasita"]);
    assert_has_krdanta(&["ava"], &so, Krt::ktavatu, &["avasitavat"]);

    let maa = d("mA\\", Adadi);
    assert_has_krdanta(&[], &maa, Krt::kta, &["mita"]);
    assert_has_krdanta(&[], &maa, Krt::ktavatu, &["mitavat"]);

    let stha = d("zWA\\", Bhvadi);
    assert_has_krdanta(&[], &stha, Krt::kta, &["sTita"]);
    assert_has_krdanta(&[], &stha, Krt::ktavatu, &["sTitavat"]);

    // ti?
    assert_has_krdanta(&["ava"], &do_, Krt::ktvA, &["avadAya"]);

    // kiti?
    assert_has_krdanta(&["ava"], &do_, Krt::tfc, &["avadAtf"]);
}

#[test]
fn sutra_7_4_41() {
    use Krt::{kta, ktavatu};
    let sho = d("So\\", Divadi);
    assert_has_krdanta(&["ni"], &sho, kta, &["niSita", "niSAta"]);
    assert_has_krdanta(&["ni"], &sho, ktavatu, &["niSitavat", "niSAtavat"]);

    let cho = d("Co\\", Divadi);
    assert_has_krdanta(&["ava"], &cho, kta, &["avacCita", "avacCAta"]);
    assert_has_krdanta(&["ava"], &cho, ktavatu, &["avacCitavat", "avacCAtavat"]);
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
    assert_has_tinantas(&[], &kf, Lut, Madhyama, Eka, &["kartAsi", "kartAse"]);
    assert_has_sip(&[], &d("asa~", Adadi), Lat, &["asi"]);
    // TODO: vyatise
}

#[test]
fn sutra_7_4_51() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_tinantas(&[], &kf, Lut, Prathama, Dvi, &["kartArO"]);
    assert_has_tinantas(&[], &kf, Lut, Prathama, Bahu, &["kartAraH"]);

    let i = d("i\\N", Adadi);
    assert_has_tinantas(&["aDi"], &i, Lut, Prathama, Dvi, &["aDyetArO"]);
    assert_has_tinantas(&["aDi"], &i, Lut, Prathama, Bahu, &["aDyetAraH"]);
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
    assert_has_ta_k(&["AN"], &didhi, Lat, &["AdIDyate"]);
    assert_has_ta_k(&["AN"], &vevi, Lat, &["Avevyate"]);
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
    assert_has_tip(&[], &san(&d("mI\\Y", Kryadi)), Lat, &["mitsati"]);
    assert_has_tip(&["pra"], &san(&d("qumi\\Y", Svadi)), Lat, &["pramitsati"]);
    assert_has_lat(&[], &san(&d("mA\\", Adadi)), &["mitsati"]);
    assert_has_lat(&[], &san(&d("mA\\N", Juhotyadi)), &["mitsate"]);
    assert_has_lat(&["apa"], &san(&d("me\\N", Bhvadi)), &["apamitsate"]);
    assert_has_tip(&[], &san(&d("qudA\\Y", Juhotyadi)), Lat, &["ditsati"]);
    assert_has_tip(&[], &san(&d("quDA\\Y", Juhotyadi)), Lat, &["Ditsati"]);
    assert_has_lat(&["AN"], &san(&d("ra\\Ba~\\", Bhvadi)), &["Aripsate"]);
    assert_has_lat(&["AN"], &san(&d("qula\\Ba~\\z", Bhvadi)), &["Alipsate"]);
    // SiSakizati seems justified because Sa\\kx~ is optionally sew.
    assert_has_lat(&[], &san(&d("Sa\\kx~", Svadi)), &["Sikzati"]);
    assert_has_lat(&[], &san(&d("patx~", Bhvadi)), &["pitsati", "pipatizati"]);
    assert_has_lat(&["pra"], &san(&d("pa\\da~\\", Divadi)), &["prapitsate"]);
    // sani
    assert_has_tip(&[], &d("qudA\\Y", Juhotyadi), Lrt, &["dAsyati"]);
    // si -- see `pipatizati` above.
}

#[test]
fn sutra_7_4_55() {
    let aap = d("A\\px~", Svadi);
    let jnap = d("jYapa~", Curadi);
    let rdh = d("fDu~", Divadi);
    assert_has_tip(&[], &san(&aap), Lat, &["Ipsati"]);
    assert_has_tip(&[], &san(&jnap), Lat, &["jYIpsati", "jijYapayizati"]);
    assert_has_tip(&[], &san(&rdh), Lat, &["Irtsati", "ardiDizati"]);

    // sani?
    assert_has_tip(&["pra"], &aap, Lrt, &["prApsyati"]);

    // si? -- see jijYapayizati and ardiDizati above.
}

#[test]
fn sutra_7_4_56() {
    assert_has_tip(
        &[],
        &san(&d("danBu~", Svadi)),
        Lat,
        &["Dipsati", "DIpsati", "didamBizati"],
    );
}

#[test]
fn sutra_7_4_58() {
    let qauk = d("QOkf~\\", Bhvadi);
    let trauk = d("trOkf~\\", Bhvadi);
    assert_has_lat(&[], &san(&qauk), &["quQOkizate"]);
    assert_has_lat(&[], &san(&trauk), &["tutrOkizate"]);
    // caNi
    assert_has_tip(&[], &nic(&d("mI\\Y", Kryadi)), Lun, &["amImapat"]);
    assert_has_tip(&[], &nic(&d("qudA\\Y", Juhotyadi)), Lun, &["adIdapat"]);
}

#[test]
fn sutra_7_4_59() {
    let qauk = d("QOkf~\\", Bhvadi);
    let trauk = d("trOkf~\\", Bhvadi);
    assert_has_lat(&[], &san(&qauk), &["quQOkizate"]);
    assert_has_lat(&[], &san(&trauk), &["tutrOkizate"]);
    assert_has_lit(&[], &qauk, &["quQOke"]);
    assert_has_lit(&[], &trauk, &["tutrOke"]);
    assert_has_tip(&[], &nic(&qauk), Lun, &["aquQOkat"]);
    assert_has_tip(&[], &nic(&trauk), Lun, &["atutrOkat"]);
    // TODO: others.
}

#[test]
fn sutra_7_4_60() {
    assert_has_tip(&[], &d("glE\\", Bhvadi), Lit, &["jaglO"]);
    assert_has_tip(&[], &d("mlE\\", Bhvadi), Lit, &["mamlO"]);
    assert_has_tip(&[], &d("qupa\\ca~^z", Bhvadi), Lit, &["papAca"]);
    assert_has_tip(&[], &d("paWa~", Bhvadi), Lit, &["papAWa"]);

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
    assert_has_tip(&[], &san(&d("zWA\\", Bhvadi)), Lat, &["tizWAsati"]);
    assert_has_lat(&[], &san(&d("spadi~\\", Bhvadi)), &["pispandizate"]);
    // SarpUrva
    assert_has_tip(&[], &d("qupa\\ca~^z", Bhvadi), Lit, &["papAca"]);
    // KayaH
    assert_has_lit(&[], &d("zRE\\", Bhvadi), &["sasnO"]);
}

#[test]
fn sutra_7_4_62() {
    assert_has_tip(&[], &d("qukf\\Y", Tanadi), Lit, &["cakAra"]);
    assert_has_tip(&[], &d("Kanu~^", Bhvadi), Lit, &["caKAna"]);
    assert_has_tip(&[], &d("ga\\mx~", Bhvadi), Lit, &["jagAma"]);
    assert_has_tip(&[], &d("ha\\na~", Adadi), Lit, &["jaGAna"]);
    assert_has_tip(&[], &d("hf\\Y", Bhvadi), Lit, &["jahAra"]);
    assert_has_tip(&[], &san(&d("hf\\Y", Bhvadi)), Lat, &["jihIrzati"]);
    assert_has_tip(&[], &d("o~hA\\k", Juhotyadi), Lit, &["jahO"]);
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
    assert_has_tip(&["vi"], &nic(&dyut), Lun, &["vyadidyutat"]);
    assert_has_lat(&["vi"], &san(&dyut), &["vididyotizate", "vididyutizate"]);
    assert_has_lat(&["vi"], &yan(&dyut), &["videdyutyate"]);

    // TODO
    let _svap = d("Yizva\\pa~", Adadi).with_sanadi(&[Sanadi::Ric, Sanadi::san]);
    // assert_has_tip(&[], &svap, Lat, &["suzvApayizati"]);
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
    assert_has_tip(&[], &d("qupa\\ca~^z", Bhvadi), Lit, &["papAca"]);
    assert_has_tip(&[], &d("paWa~", Bhvadi), Lit, &["papAWa"]);
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
    assert_has_ta(&["vi"], &ashnoti, Lit, &["vyAnaSe"]);
    assert_has_aataam(&["vi"], &ashnoti, Lit, &["vyAnaSAte"]);
    assert_has_jha(&["vi"], &ashnoti, Lit, &["vyAnaSire"]);
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
    assert_has_tip(&[], &nij, Lat, &["nenekti"]);
    assert_has_tip(&[], &d("vi\\ji~^r", Juhotyadi), Lat, &["vevekti"]);
    assert_has_tip(&[], &d("vi\\zx~^", Juhotyadi), Lat, &["vevezwi"]);
    // SlO
    assert_has_tip(&[], &nij, Lit, &["nineja"]);
}

#[test]
fn sutra_7_4_76() {
    let bhf = d("quBf\\Y", Juhotyadi);
    assert_has_tip(&[], &bhf, Lat, &["biBarti"]);
    assert_has_ta(&[], &d("mA\\N", Juhotyadi), Lat, &["mimIte"]);
    assert_has_ta(&[], &d("o~hA\\N", Juhotyadi), Lat, &["jihIte"]);
    // trayARAm
    assert_has_tip(&[], &d("o~hA\\k", Juhotyadi), Lat, &["jahAti"]);
    // SlO
    assert_has_tip(
        &[],
        &bhf,
        Lit,
        &["baBAra", "biBarAYcakAra", "biBarAmAsa", "biBarAmbaBUva"],
    );
}

#[test]
fn sutra_7_4_77() {
    assert_has_tip(&[], &d("f\\", Juhotyadi), Lat, &["iyarti"]);
    assert_has_tip(&[], &d("pF", Juhotyadi), Lat, &["piparti"]);
}

#[test]
fn sutra_7_4_79() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tip(&[], &san(&pac), Lat, &["pipakzati"]);
    assert_has_tip(&[], &san(&d("ya\\ja~^", Bhvadi)), Lat, &["yiyakzati"]);
    assert_has_tip(&[], &san(&d("zWA\\", Bhvadi)), Lat, &["tizWAsati"]);
    assert_has_tip(&[], &san(&d("pA\\", Bhvadi)), Lat, &["pipAsati"]);
    // sani
    assert_has_tip(&[], &pac, Lit, &["papAca"]);
    // ataH
    assert_has_tip(&[], &san(&d("lUY", Kryadi)), Lat, &["lulUzati"]);
    // taparakaraNa
    let _pac_yan_san = pac.with_sanadi(&[Sanadi::yaN, Sanadi::san]);
    // assert_has_ta(&[], &pac_yan_san, Lat, &["pApacizate"]);
    // TODO: pApacizate
}

#[test]
fn sutra_7_4_80() {
    // pu
    let pu = d("pUN", Bhvadi);
    let bhu = d("BU", Bhvadi);
    assert_has_lat(&[], &san(&pu), &["pipavizate"]);
    assert_has_tip(&[], &nic_san(&pu), Lat, &["pipAvayizati"]);
    assert_has_tip(&[], &nic_san(&bhu), Lat, &["biBAvayizati"]);
    let yu = d("yu", Adadi);
    let ru = d("ru", Adadi);
    let lu = d("lUY", Kryadi);
    assert_has_lat(&[], &san(&yu), &["yiyavizati", "yuyUzati"]);
    assert_has_tip(&[], &nic_san(&yu), Lat, &["yiyAvayizati"]);
    assert_has_tip(&[], &nic_san(&ru), Lat, &["rirAvayizati"]);
    assert_has_tip(&[], &nic_san(&lu), Lat, &["lilAvayizati"]);
    // ju (sautro dhatu)
    let ju = d("ju", Adadi);
    assert_has_tip(&[], &nic_san(&ju), Lat, &["jijAvayizati"]);
    // oH
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_ta(&[], &yan(&pac), Lat, &["pApacyate"]);
    assert_has_ta(&[], &yan_san(&pac), Lat, &["pApacizate"]);
    // puyaNjyapare
    let tu = d("tu\\", Adadi);
    assert_has_tip(&["ava"], &nic_san(&tu), Lat, &["avatutAvayizati"]);
    assert_has_tip(&[], &nic_san(&d("hu\\", Juhotyadi)), Lat, &["juhAvayizati"]);
    assert_has_lat(&[], &san(&bhu), &["buBUzati"]);
}

#[test]
fn sutra_7_4_81() {
    let sru = &d("sru\\", Bhvadi);
    let shru = &d("Sru\\", Bhvadi);
    assert_has_tip(
        &[],
        &nic_san(&sru),
        Lat,
        &["sisrAvayizati", "susrAvayizati"],
    );
    assert_has_tip(
        &[],
        &nic_san(&shru),
        Lat,
        &["SiSrAvayizati", "SuSrAvayizati"],
    );
    assert_has_tip(
        &[],
        &nic_san(&d("dru\\", Bhvadi)),
        Lat,
        &["didrAvayizati", "dudrAvayizati"],
    );
    assert_has_tip(
        &[],
        &nic_san(&d("pru\\N", Bhvadi)),
        Lat,
        &["piprAvayizati", "puprAvayizati"],
    );
    assert_has_tip(
        &[],
        &nic_san(&d("plu\\N", Bhvadi)),
        Lat,
        &["piplAvayizati", "puplAvayizati"],
    );
    assert_has_tip(
        &[],
        &nic_san(&d("cyu\\N", Bhvadi)),
        Lat,
        &["cicyAvayizati", "cucyAvayizati"],
    );
    // apare?
    assert_has_tip(&[], &san(&sru), Lat, &["susrUzati"]);
    assert_has_ta(&[], &san(&shru), Lat, &["SuSrUzate"]);
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
    assert_has_ta(&[], &yan(&pac), Lat, &["pApacyate"]);
    assert_has_lat(&[], &yan_luk(&pac), &["pApacIti", "pApakti"]);
    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_ta(&[], &yan(&yaj), Lat, &["yAyajyate"]);
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

    // ataH?
    let tim = d("tima~", Bhvadi);
    assert_has_lat(&[], &yan(&tim), &["tetimyate"]);

    // taparakaraNam?
    let bham = d("BAma~\\", Bhvadi);
    assert_has_lat(&[], &yan(&bham), &["bABAmyate"]);

    // anunAsikAntasya?
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_lat(&[], &yan(&pac), &["pApacyate"]);
}

#[test]
fn skip_sutra_7_4_85_v1() {}

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
    assert_has_lat(&[], &yan_luk(&d("kF", Tudadi)), &["cAkarti", "cAkarIti"]);
}

#[test]
fn sutra_7_4_93() {
    let pac = &d("qupa\\ca~^z", Bhvadi);

    // sanyataH
    assert_has_tip(&[], &nic(&d("qukf\\Y", Tanadi)), Lun, &["acIkarat"]);
    assert_has_tip(&[], &nic(&pac), Lun, &["apIpacat"]);
    // oH puyaRjyapare
    assert_has_tip(&[], &nic(&d("pUY", Kryadi)), Lun, &["apIpavat"]);
    assert_has_tip(&[], &nic(&d("lUY", Kryadi)), Lun, &["alIlavat"]);
    // svarati... (7.4.81)
    assert_has_tip(
        &[],
        &nic(&d("sru\\", Bhvadi)),
        Lun,
        &["asisravat", "asusravat"],
    );
    assert_has_tip(
        &[],
        &nic(&d("Sru\\", Svadi)),
        Lun,
        &["aSiSravat", "aSuSravat"],
    );
    assert_has_tip(
        &[],
        &nic(&d("dru\\", Bhvadi)),
        Lun,
        &["adidravat", "adudravat"],
    );
    assert_has_tip(
        &[],
        &nic(&d("pru\\N", Bhvadi)),
        Lun,
        &["apipravat", "apupravat"],
    );
    assert_has_tip(
        &[],
        &nic(&d("plu\\N", Bhvadi)),
        Lun,
        &["apiplavat", "apuplavat"],
    );
    assert_has_tip(
        &[],
        &nic(&d("cyu\\N", Bhvadi)),
        Lun,
        &["acicyavat", "acucyavat"],
    );
    // laghuni
    assert_has_tip(&[], &nic(&d("takzU~", Bhvadi)), Lun, &["atatakzat"]);
    assert_has_tip(&[], &nic(&d("rakza~", Bhvadi)), Lun, &["ararakzat"]);
    assert_has_tip(&[], &nic(&d("jAgf", Adadi)), Lun, &["ajajAgarat"]);
    // caN-pare?
    assert_has_mip(&[], &pac, Lit, &["papaca", "papAca"]);
    // para-grahaRa
    assert_has_ta(&[], &d("kamu~\\", Bhvadi), Lun, &["acakamata", "acIkamata"]);
    // an-ak-lopa
    assert_has_tip(&[], &d("kaTa", Curadi), Lun, &["acakaTat"]);
    // TODO: others
}

#[test]
fn sutra_7_4_94() {
    let pac = &d("qupa\\ca~^z", Bhvadi);
    assert_has_tip(&[], &nic(&d("qukf\\Y", Tanadi)), Lun, &["acIkarat"]);
    assert_has_tip(&[], &nic(&d("hf\\Y", Bhvadi)), Lun, &["ajIharat"]);
    assert_has_tip(&[], &nic(&d("lUY", Kryadi)), Lun, &["alIlavat"]);
    assert_has_tip(&[], &nic(&pac), Lun, &["apIpacat"]);
    // laghoH
    // abiBrajat is allowed by 7.4.3.
    assert_has_tip(
        &[],
        &nic(&d("wuBrAjf~\\", Bhvadi)),
        Lun,
        &["abiBrajat", "abaBrAjat"],
    );
    // laghuni
    assert_has_tip(&[], &nic(&d("takzU~", Bhvadi)), Lun, &["atatakzat"]);
    assert_has_tip(&[], &nic(&d("rakza~", Bhvadi)), Lun, &["ararakzat"]);
    // caNi
    assert_has_mip(&[], &pac, Lit, &["papaca", "papAca"]);
    // pare
    assert_has_ta(&[], &d("kamu~\\", Bhvadi), Lun, &["acakamata", "acIkamata"]);
    // anaglopa
    assert_has_tip(&[], &d("kaTa", Curadi), Lun, &["acakaTat"]);
}

#[test]
fn sutra_7_4_95() {
    assert_has_tip(&[], &nic(&d("smf\\", Bhvadi)), Lun, &["asasmarat"]);
    assert_has_tip(&[], &nic(&d("dF", Bhvadi)), Lun, &["adadarat"]);
    assert_has_tip(&[], &nic(&d("YitvarA~\\", Bhvadi)), Lun, &["atatvarat"]);
    assert_has_tip(&[], &d("praTa~", Curadi), Lun, &["apapraTat"]);
    assert_has_tip(&[], &nic(&d("mrada~\\", Bhvadi)), Lun, &["amamradat"]);
    assert_has_tip(&[], &nic(&d("stFY", Kryadi)), Lun, &["atastarat"]);
    assert_has_tip(&[], &nic(&d("spaSa~^", Bhvadi)), Lun, &["apaspaSat"]);
}

#[test]
fn sutra_7_4_96() {
    let vest = d("vezwa~\\", Bhvadi);
    let cest = d("cezwa~\\", Bhvadi);
    assert_has_tip(&[], &nic(&vest), Lun, &["avavezwat", "avivezwat"]);
    assert_has_tip(&[], &nic(&cest), Lun, &["acacezwat", "acicezwat"]);
}

#[test]
fn sutra_7_4_97() {
    let gan = d("gaRa", Curadi);
    assert_has_tip(&[], &gan, Lun, &["ajIgaRat", "ajagaRat"]);
}
