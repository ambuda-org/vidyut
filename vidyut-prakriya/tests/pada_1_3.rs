extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

#[test]
fn sutra_1_3_1() {
    assert_has_lat(&[], &d("BU", Bhvadi), &["Bavati"]);
    assert_has_lat(&[], &d("eDa~\\", Bhvadi), &["eDate"]);
    assert_has_lat(&[], &d("sparDa~\\", Bhvadi), &["sparDate"]);
}

#[test]
fn sutra_1_3_12() {
    // anudAttet
    assert_has_lat(&[], &d("Asa~\\", Adadi), &["Aste"]);
    assert_has_lat(&[], &d("vasa~\\", Adadi), &["vaste"]);
    // Nit
    assert_has_lat(&[], &d("zUN", Adadi), &["sUte"]);
    assert_has_lat(&[], &d("SIN", Adadi), &["Sete"]);
}

#[test]
fn sutra_1_3_13() {
    assert_has_lat_karmani(&[], &d("glE\\", Bhvadi), &["glAyate"]);
    assert_has_lat_karmani(&[], &d("Yizva\\pa~", Adadi), &["supyate"]);
    assert_has_lat_karmani(&[], &d("Asa~\\", Adadi), &["Asyate"]);
    assert_has_lat_karmani(&[], &d("qukf\\Y", Tanadi), &["kriyate"]);
    assert_has_lat_karmani(&[], &d("hf\\Y", Bhvadi), &["hriyate"]);
    assert_has_lat_karmani(&[], &d("lUY", Kryadi), &["lUyate"]);
}

#[test]
fn sutra_1_3_17() {
    let vish = d("vi\\Sa~", Tudadi);
    assert_has_lat(&["ni"], &vish, &["niviSate"]);
    assert_has_lat(&["pra"], &vish, &["praviSati"]);
    assert_has_lan(&["ni"], &vish, &["nyaviSata"]);
}

#[test]
fn sutra_1_3_18() {
    let kri = d("qukrI\\Y", Kryadi);
    assert_has_lat(&["pari"], &kri, &["parikrIRIte"]);
    assert_has_lat(&["vi"], &kri, &["vikrIRIte"]);
    assert_has_lat(&["ava"], &kri, &["avakrIRIte"]);
}

#[test]
fn sutra_1_3_19() {
    let ji = d("ji\\", Bhvadi);
    assert_has_lat(&["vi"], &ji, &["vijayate"]);
    assert_has_lat(&["parA"], &ji, &["parAjayate"]);
}

#[test]
fn sutra_1_3_20() {
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_lat(&["AN"], &daa, &["Adatte", "AdadAti"]);
}

#[test]
fn sutra_1_3_21() {
    let krid = d("krIqf~", Bhvadi);
    assert_has_lat(&["anu"], &krid, &["anukrIqate"]);
    assert_has_lat(&["sam"], &krid, &["saNkrIqate"]);
    assert_has_lat(&["pari"], &krid, &["parikrIqate"]);
    assert_has_lat(&["AN"], &krid, &["AkrIqate"]);

    // TODO: varttikas
}

#[test]
fn sutra_1_3_22() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_lat(&["sam"], &stha, &["santizWate"]);
    assert_has_lat(&["ava"], &stha, &["avatizWate"]);
    assert_has_lat(&["pra"], &stha, &["pratizWate"]);
    assert_has_lat(&["vi"], &stha, &["vitizWate"]);
}

#[test]
fn sutra_1_3_22_v1() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_lat(&["AN"], &stha, &["AtizWati", "AtizWate"]);
}

#[test]
fn sutra_1_3_23() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_lat(&[], &stha, &["tizWati", "tizWate"]);
}

#[test]
fn sutra_1_3_24() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_lat(&["ud"], &stha, &["uttizWati", "uttizWate"]);
}

#[test]
fn sutra_1_3_25() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_lat(&["upa"], &stha, &["upatizWati", "upatizWate"]);
}

#[test]
fn sutra_1_3_26() {
    let tap = d("ta\\pa~", Bhvadi);
    assert_has_lat(&["ud"], &tap, &["uttapati", "uttapate"]);
    assert_has_lat(&["vi"], &tap, &["vitapati", "vitapate"]);
}

#[test]
fn sutra_1_3_27() {
    let yam = d("ya\\ma~", Bhvadi);
    let han = d("ha\\na~", Adadi);
    assert_has_lat(&["AN"], &yam, &["AyacCati", "AyacCate"]);
    assert_has_lat(&["AN"], &han, &["Ahanti", "Ahate"]);
}

#[test]
fn sutra_1_3_28() {
    let d = d;
    assert_has_lat(
        &["sam"],
        &d("ga\\mx~", Bhvadi),
        &["saNgacCati", "saNgacCate"],
    );
    assert_has_lat(&["sam"], &d("fCa~", Bhvadi), &["samfcCati", "samfcCate"]);
    assert_has_lat(
        &["sam"],
        &d("pra\\Ca~", Tudadi),
        &["sampfcCati", "sampfcCate"],
    );
    assert_has_lat(&["sam"], &d("svf", Bhvadi), &["saMsvarati", "saMsvarate"]);
    assert_has_lat(&["sam"], &d("f\\", Bhvadi), &["samfcCati", "samfcCate"]);
    assert_has_lat(&["sam"], &d("Sru\\", Bhvadi), &["saMSfRoti", "saMSfRute"]);
    assert_has_lat(
        &["sam"],
        &d("vida~", Adadi),
        &["saMvetti", "saMveda", "saMvitte"],
    );
}

/*
#[test]
fn sutra_1_3_28_v1() {
    let dfs = d("ha\\na~", Adadi);
    assert_has_lat(&["sam"], &dfs, &["sampaSyati", "sampaSyate"]);
}
*/

#[test]
fn sutra_1_3_30() {
    let han = d("hve\\Y", Bhvadi);
    assert_has_lat(&["ni"], &han, &["nihvayate"]);
    assert_has_lat(&["sam"], &han, &["saMhvayate"]);
    assert_has_lat(&["upa"], &han, &["upahvayate"]);
    assert_has_lat(&["vi"], &han, &["vihvayate"]);
}

#[test]
fn sutra_1_3_31() {
    let han = d("hve\\Y", Bhvadi);
    assert_has_lat(&["AN"], &han, &["Ahvayati", "Ahvayate"]);
}

#[test]
fn sutra_1_3_38() {
    let kram = d("kramu~", Bhvadi);
    assert_has_lat(&[], &kram, &["krAmati", "krAmyati", "kramate", "kramyate"]);
}

#[test]
fn sutra_1_3_39() {
    let kram = d("kramu~", Bhvadi);
    assert_has_lat(
        &["upa"],
        &kram,
        &["upakrAmati", "upakrAmyati", "upakramate", "upakramyate"],
    );
    assert_has_lat(
        &["parA"],
        &kram,
        &["parAkrAmati", "parAkrAmyati", "parAkramate", "parAkramyate"],
    );
}

#[test]
fn sutra_1_3_40() {
    let kram = d("kramu~", Bhvadi);
    assert_has_lat(
        &["AN"],
        &kram,
        &["AkrAmati", "AkrAmyati", "Akramate", "Akramyate"],
    );
}

#[test]
fn sutra_1_3_41() {
    let kram = d("kramu~", Bhvadi);
    assert_has_lat(
        &["vi"],
        &kram,
        &["vikrAmati", "vikrAmyati", "vikramate", "vikramyate"],
    );
}

#[test]
fn sutra_1_3_42() {
    let kram = d("kramu~", Bhvadi);
    assert_has_lat(
        &["pra"],
        &kram,
        &["prakrAmati", "prakrAmyati", "prakramate", "prakramyate"],
    );
    assert_has_lat(
        &["upa"],
        &kram,
        &["upakrAmati", "upakrAmyati", "upakramate", "upakramyate"],
    );
}

#[test]
fn sutra_1_3_47() {
    let vad = d("vada~", Bhvadi);
    assert_has_lat(&[], &vad, &["vadati", "vadate"]);
}

#[test]
fn sutra_1_3_51() {
    let gf = d("gF", Tudadi);
    assert_has_lat(&["ava"], &gf, &["avagirate", "avagilate"]);
}

#[test]
fn sutra_1_3_52() {
    let gf = d("gF", Tudadi);
    assert_has_lat(
        &["sam"],
        &gf,
        &["saNgirati", "saNgilati", "saNgirate", "saNgilate"],
    );
}

#[test]
fn sutra_1_3_53() {
    let car = d("cara~", Bhvadi);
    assert_has_lat(&["ud"], &car, &["uccarati", "uccarate"]);
}

#[test]
fn sutra_1_3_54() {
    let car = d("cara~", Bhvadi);
    assert_has_lat(&["sam"], &car, &["saYcarati", "saYcarate"]);
}

#[test]
fn sutra_1_3_55() {
    let daa = d("dA\\R", Bhvadi);
    assert_has_lat(&["sam", "pra"], &daa, &["samprayacCati", "samprayacCate"]);
}

#[test]
fn sutra_1_3_56() {
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_lat(&["upa"], &yam, &["upayacCati", "upayacCate"]);
}

#[ignore]
#[test]
fn sutra_1_3_57() {
    let jna = d("jYA\\", Kryadi).with_sanadi(&[Sanadi::San]);
    assert_has_lat(&[], &jna, &["jijYAsate"]);

    let sru = d("Sru\\", Svadi).with_sanadi(&[Sanadi::San]);
    assert_has_lat(&[], &sru, &["SuSrUzate"]);

    let smf = d("smf\\", Bhvadi).with_sanadi(&[Sanadi::San]);
    assert_has_lat(&[], &smf, &["susmUrzate"]);

    let dfs = d("df\\Si~r", Bhvadi).with_sanadi(&[Sanadi::San]);
    assert_has_lat(&[], &dfs, &["didfkzate"]);
}

#[test]
fn sutra_1_3_58() {
    let jna = d("jYA\\", Kryadi).with_sanadi(&[Sanadi::San]);
    assert_has_lat(&["anu"], &jna, &["anujijYAsati"]);
}

#[test]
fn sutra_1_3_59() {
    let sru = d("Sru\\", Svadi).with_sanadi(&[Sanadi::San]);
    assert_has_lat(&["prati"], &sru, &["pratiSuSrUzati"]);
    assert_has_lat(&["AN"], &sru, &["ASuSrUzati"]);
}

#[test]
fn sutra_1_3_60() {
    let shad = d("Sa\\dx~", Tudadi);
    let shad_san = shad.clone().with_sanadi(&[Sanadi::San]);
    assert_has_lat(&[], &shad, &["SIyate"]);
    assert_has_lrn(&[], &shad, &["aSatsyat"]);
    assert_has_lrt(&[], &shad, &["Satsyati"]);
    assert_has_lat(&[], &shad_san, &["SiSatsati"]);
}

#[test]
fn sutra_1_3_61() {
    let mr = d("mf\\N", Tudadi);
    assert_has_lun(&[], &mr, &["amfta"]);
    assert_has_ashirlin(&[], &mr, &["mfzIzwa"]);
    assert_has_lat(&[], &mr, &["mriyate"]);
    assert_has_lrt(&[], &mr, &["marizyati"]);
    assert_has_lrn(&[], &mr, &["amarizyat"]);
}

#[ignore]
#[test]
fn sutra_1_3_62() {
    let d = d;
    let san = |u, gana| d(u, gana).with_sanadi(&[Sanadi::San]);

    assert_has_lat(&[], &san("Asa~\\", Adadi), &["Asisizate"]);
    assert_has_lat(&[], &san("SIN", Adadi), &["SiSayizate"]);
    assert_has_lat(&["ni"], &san("vi\\Sa~", Tudadi), &["nivivikzate"]);
    assert_has_lat(
        &["AN"],
        &san("kramu~", Bhvadi),
        &["AcikraMsate", "AcikraMsati"],
    );

    // From S. C. Vasu's commentary
    assert_has_lat(&[], &d("gupa~\\", Bhvadi), &["jugupsate"]);
    assert_has_lat(&[], &d("tija~\\", Bhvadi), &["titikzate"]);
    assert_has_lat(&[], &d("mAna~\\", Bhvadi), &["mImAMsate"]);
    assert_has_lat(&[], &d("baDa~\\", Bhvadi), &["bIBatsate"]);
}

#[ignore]
#[test]
fn sutra_1_3_63() {
    assert_has_lit(
        &[],
        &d("Ikza~\\", Bhvadi),
        &["IkzAYcakre", "IkzAmbaBUva", "IkzAmAsa"],
    );
    assert_has_lit(
        &[],
        &d("Iha~\\", Bhvadi),
        &["IhAYcakre", "IhAmbaBUva", "IhAmAsa"],
    );
}

#[test]
fn sutra_1_3_64() {
    let yuj = d("yu\\ji~^r", Rudhadi);

    assert_has_lat(&["pra"], &yuj, &["prayuNkte", "prayunakti"]);
    assert_has_lat(&["upa"], &yuj, &["upayuNkte", "upayunakti"]);
}

#[test]
fn sutra_1_3_65() {
    let ksnu = d("kzRu", Adadi);

    assert_has_lat(&["sam"], &ksnu, &["saNkzRute"]);
}

// TODO: 1.3.64.v1

#[test]
fn sutra_1_3_66() {
    let bhuj = d("Bu\\ja~", Rudhadi);
    assert_has_lat(&[], &bhuj, &["Bunakti", "BuNkte"]);
}

#[test]
fn sutra_1_3_73() {
    let vad = d("vada~", Bhvadi);
    assert_has_lat(&["apa"], &vad, &["apavadati", "apavadate"]);
}

#[test]
fn sutra_1_3_75() {
    let yam = d("ya\\ma~", Bhvadi);
    assert_has_lat(&["sam"], &yam, &["saMyacCati", "saMyacCate"]);
    assert_has_lat(&["ud"], &yam, &["udyacCati", "udyacCate"]);
    assert_has_lat(&["AN"], &yam, &["AyacCati", "AyacCate"]);
}

#[test]
fn sutra_1_3_76() {
    let jna = d("jYA\\", Kryadi);
    assert_has_lat(&[], &jna, &["jAnAti", "jAnIte"]);
    assert_has_lat(&["pra"], &jna, &["prajAnAti"]);
}

#[test]
fn sutra_1_3_78() {
    assert_has_lat(&[], &d("yA\\", Adadi), &["yAti"]);
    assert_has_lat(&[], &d("vA\\", Adadi), &["vAti"]);

    let vish = d("vi\\Sa~", Tudadi);
    assert_has_lat(&["AN"], &vish, &["AviSati"]);
    assert_has_lat(&["pra"], &vish, &["praviSati"]);
}

#[test]
fn sutra_1_3_79() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_lat(&["anu"], &kr, &["anukaroti"]);
    assert_has_lat(&["parA"], &kr, &["parAkaroti"]);
}

#[test]
fn sutra_1_3_80() {
    let ksip = d("kzi\\pa~^", Tudadi);
    assert_has_lat(&["aBi"], &ksip, &["aBikzipati"]);
    assert_has_lat(&["prati"], &ksip, &["pratikzipati"]);
    assert_has_lat(&["ati"], &ksip, &["atikzipati"]);

    assert_has_lat(&["AN"], &ksip, &["Akzipati", "Akzipate"]);
}

#[test]
fn sutra_1_3_81() {
    let vah = d("va\\ha~^", Bhvadi);
    assert_has_lat(&["pra"], &vah, &["pravahati"]);
    assert_has_lat(&["AN"], &vah, &["Avahati", "Avahate"]);
}

#[test]
fn sutra_1_3_82() {
    let mfs = d("mfza~^", Divadi);
    assert_has_lat(&["pari"], &mfs, &["parimfzyati"]);
    assert_has_lat(&["AN"], &mfs, &["Amfzyati", "Amfzyate"]);
}

#[test]
fn sutra_1_3_83() {
    let ram = d("ra\\mu~\\", Bhvadi);
    assert_has_lat(&["vi"], &ram, &["viramati"]);
    assert_has_lat(&["AN"], &ram, &["Aramati"]);
    assert_has_lat(&["pari"], &ram, &["pariramati"]);
    assert_has_lat(&["aBi"], &ram, &["aBiramate"]);
}

#[test]
fn sutra_1_3_85() {
    let ram = d("ra\\ma~\\", Bhvadi);
    assert_has_lat(&["upa"], &ram, &["uparamati", "uparamate"]);
}
