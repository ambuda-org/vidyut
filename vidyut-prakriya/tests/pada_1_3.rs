extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::*;

#[test]
fn sutra_1_3_17() {
    let vish = Dhatu::new("vi\\Sa~", Gana::Tudadi);
    assert_has_lat(&["ni"], &vish, &["niviSate"]);
    assert_has_lat(&["pra"], &vish, &["praviSati"]);
    assert_has_lan(&["ni"], &vish, &["nyaviSata"]);
}

#[test]
fn sutra_1_3_18() {
    let kri = Dhatu::new("qukrI\\Y", Gana::Kryadi);
    assert_has_lat(&["pari"], &kri, &["parikrIRIte"]);
    assert_has_lat(&["vi"], &kri, &["vikrIRIte"]);
    assert_has_lat(&["ava"], &kri, &["avakrIRIte"]);
}

#[test]
fn sutra_1_3_19() {
    let ji = Dhatu::new("ji\\", Gana::Bhvadi);
    assert_has_lat(&["vi"], &ji, &["vijayate"]);
    assert_has_lat(&["parA"], &ji, &["parAjayate"]);
}

#[test]
fn sutra_1_3_20() {
    let daa = Dhatu::new("qudA\\Y", Gana::Juhotyadi);
    assert_has_lat(&["AN"], &daa, &["Adatte", "AdadAti"]);
}

#[test]
fn sutra_1_3_21() {
    let krid = Dhatu::new("krIqf~", Gana::Bhvadi);
    assert_has_lat(&["anu"], &krid, &["anukrIqate"]);
    assert_has_lat(&["sam"], &krid, &["saNkrIqate"]);
    assert_has_lat(&["pari"], &krid, &["parikrIqate"]);
    assert_has_lat(&["AN"], &krid, &["AkrIqate"]);

    // TODO: varttikas
}

#[test]
fn sutra_1_3_22() {
    let stha = Dhatu::new("zWA\\", Gana::Bhvadi);
    assert_has_lat(&["sam"], &stha, &["santizWate"]);
    assert_has_lat(&["ava"], &stha, &["avatizWate"]);
    assert_has_lat(&["pra"], &stha, &["pratizWate"]);
    assert_has_lat(&["vi"], &stha, &["vitizWate"]);
}

#[test]
fn sutra_1_3_22_v1() {
    let stha = Dhatu::new("zWA\\", Gana::Bhvadi);
    assert_has_lat(&["AN"], &stha, &["AtizWati", "AtizWate"]);
}

#[test]
fn sutra_1_3_23() {
    let stha = Dhatu::new("zWA\\", Gana::Bhvadi);
    assert_has_lat(&[], &stha, &["tizWati", "tizWate"]);
}

#[test]
fn sutra_1_3_24() {
    let stha = Dhatu::new("zWA\\", Gana::Bhvadi);
    assert_has_lat(&["ud"], &stha, &["uttizWati", "uttizWate"]);
}

#[test]
fn sutra_1_3_25() {
    let stha = Dhatu::new("zWA\\", Gana::Bhvadi);
    assert_has_lat(&["upa"], &stha, &["upatizWati", "upatizWate"]);
}

#[test]
fn sutra_1_3_26() {
    let tap = Dhatu::new("ta\\pa~", Gana::Bhvadi);
    assert_has_lat(&["ud"], &tap, &["uttapati", "uttapate"]);
    assert_has_lat(&["vi"], &tap, &["vitapati", "vitapate"]);
}

#[test]
fn sutra_1_3_27() {
    let yam = Dhatu::new("ya\\ma~", Gana::Bhvadi);
    let han = Dhatu::new("ha\\na~", Gana::Adadi);
    assert_has_lat(&["AN"], &yam, &["AyacCati", "AyacCate"]);
    assert_has_lat(&["AN"], &han, &["Ahanti", "Ahate"]);
}

#[test]
fn sutra_1_3_28() {
    let d = Dhatu::new;
    assert_has_lat(
        &["sam"],
        &d("ga\\mx~", Gana::Bhvadi),
        &["saNgacCati", "saNgacCate"],
    );
    assert_has_lat(
        &["sam"],
        &d("fCa~", Gana::Bhvadi),
        &["samfcCati", "samfcCate"],
    );
    assert_has_lat(
        &["sam"],
        &d("pra\\Ca~", Gana::Tudadi),
        &["sampfcCati", "sampfcCate"],
    );
    assert_has_lat(
        &["sam"],
        &d("svf", Gana::Bhvadi),
        &["saMsvarati", "saMsvarate"],
    );
    assert_has_lat(
        &["sam"],
        &d("f\\", Gana::Bhvadi),
        &["samfcCati", "samfcCate"],
    );
    assert_has_lat(
        &["sam"],
        &d("Sru\\", Gana::Bhvadi),
        &["saMSfRoti", "saMSfRute"],
    );
    assert_has_lat(
        &["sam"],
        &d("vida~", Gana::Adadi),
        &["saMvetti", "saMveda", "saMvitte"],
    );
}

/*
#[test]
fn sutra_1_3_28_v1() {
    let dfs = Dhatu::new("ha\\na~", Gana::Adadi);
    assert_has_lat(&["sam"], &dfs, &["sampaSyati", "sampaSyate"]);
}
*/

#[test]
fn sutra_1_3_30() {
    let han = Dhatu::new("hve\\Y", Gana::Bhvadi);
    assert_has_lat(&["ni"], &han, &["nihvayate"]);
    assert_has_lat(&["sam"], &han, &["saMhvayate"]);
    assert_has_lat(&["upa"], &han, &["upahvayate"]);
    assert_has_lat(&["vi"], &han, &["vihvayate"]);
}

#[test]
fn sutra_1_3_31() {
    let han = Dhatu::new("hve\\Y", Gana::Bhvadi);
    assert_has_lat(&["AN"], &han, &["Ahvayati", "Ahvayate"]);
}

#[test]
fn sutra_1_3_38() {
    let kram = Dhatu::new("kramu~", Gana::Bhvadi);
    assert_has_lat(&[], &kram, &["krAmati", "krAmyati", "kramate", "kramyate"]);
}

#[test]
fn sutra_1_3_39() {
    let kram = Dhatu::new("kramu~", Gana::Bhvadi);
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
    let kram = Dhatu::new("kramu~", Gana::Bhvadi);
    assert_has_lat(
        &["AN"],
        &kram,
        &["AkrAmati", "AkrAmyati", "Akramate", "Akramyate"],
    );
}

#[test]
fn sutra_1_3_41() {
    let kram = Dhatu::new("kramu~", Gana::Bhvadi);
    assert_has_lat(
        &["vi"],
        &kram,
        &["vikrAmati", "vikrAmyati", "vikramate", "vikramyate"],
    );
}

#[test]
fn sutra_1_3_42() {
    let kram = Dhatu::new("kramu~", Gana::Bhvadi);
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
    let vad = Dhatu::new("vada~", Gana::Bhvadi);
    assert_has_lat(&[], &vad, &["vadati", "vadate"]);
}

#[test]
fn sutra_1_3_51() {
    let gf = Dhatu::new("gF", Gana::Tudadi);
    assert_has_lat(&["ava"], &gf, &["avagirate", "avagilate"]);
}

#[test]
fn sutra_1_3_52() {
    let gf = Dhatu::new("gF", Gana::Tudadi);
    assert_has_lat(
        &["sam"],
        &gf,
        &["saNgirati", "saNgilati", "saNgirate", "saNgilate"],
    );
}

#[test]
fn sutra_1_3_53() {
    let car = Dhatu::new("cara~", Gana::Bhvadi);
    assert_has_lat(&["ud"], &car, &["uccarati", "uccarate"]);
}

#[test]
fn sutra_1_3_54() {
    let car = Dhatu::new("cara~", Gana::Bhvadi);
    assert_has_lat(&["sam"], &car, &["saYcarati", "saYcarate"]);
}

#[test]
fn sutra_1_3_55() {
    let daa = Dhatu::new("dA\\R", Gana::Bhvadi);
    assert_has_lat(&["sam", "pra"], &daa, &["samprayacCati", "samprayacCate"]);
}

#[test]
fn sutra_1_3_56() {
    let yam = Dhatu::new("ya\\ma~", Gana::Bhvadi);
    assert_has_lat(&["upa"], &yam, &["upayacCati", "upayacCate"]);
}

#[ignore]
#[test]
fn sutra_1_3_57() {
    let jna = Dhatu::new("jYA\\", Gana::Kryadi).with_sanadi(&[Sanadi::San]);
    assert_has_lat(&[], &jna, &["jijYAsate"]);

    let sru = Dhatu::new("Sru\\", Gana::Svadi).with_sanadi(&[Sanadi::San]);
    assert_has_lat(&[], &sru, &["SuSrUzate"]);

    // TODO: sanAdis are buggy right now, so this fails.
    let smf = Dhatu::new("smf", Gana::Bhvadi).with_sanadi(&[Sanadi::San]);
    assert_has_lat(&[], &smf, &["susmUrzate"]);

    let dfs = Dhatu::new("df\\Si~r", Gana::Bhvadi).with_sanadi(&[Sanadi::San]);
    assert_has_lat(&[], &dfs, &["didfkzate"]);
}

#[test]
fn sutra_1_3_58() {
    let jna = Dhatu::new("jYA\\", Gana::Kryadi).with_sanadi(&[Sanadi::San]);
    assert_has_lat(&["anu"], &jna, &["anujijYAsati"]);
}

#[test]
fn sutra_1_3_59() {
    let sru = Dhatu::new("Sru\\", Gana::Svadi).with_sanadi(&[Sanadi::San]);
    assert_has_lat(&["prati"], &sru, &["pratiSuSrUzati"]);
    assert_has_lat(&["AN"], &sru, &["ASuSrUzati"]);
}

#[test]
fn sutra_1_3_60() {
    let shad = Dhatu::new("Sa\\dx~", Gana::Tudadi);
    let shad_san = shad.clone().with_sanadi(&[Sanadi::San]);
    assert_has_lat(&[], &shad, &["SIyate"]);
    assert_has_lrn(&[], &shad, &["aSatsyat"]);
    assert_has_lrt(&[], &shad, &["Satsyati"]);
    assert_has_lat(&[], &shad_san, &["SiSatsati"]);
}

#[test]
fn sutra_1_3_61() {
    let mr = Dhatu::new("mf\\N", Gana::Tudadi);
    assert_has_lun(&[], &mr, &["amfta"]);
    assert_has_ashirlin(&[], &mr, &["mfzIzwa"]);
    assert_has_lat(&[], &mr, &["mriyate"]);
    assert_has_lrt(&[], &mr, &["marizyati"]);
    assert_has_lrn(&[], &mr, &["amarizyat"]);
}

#[ignore]
#[test]
fn sutra_1_3_62() {
    let d = Dhatu::new;
    let san = |u, gana| Dhatu::new(u, gana).with_sanadi(&[Sanadi::San]);

    assert_has_lat(&[], &san("Asa~\\", Gana::Adadi), &["Asisizate"]);
    assert_has_lat(&[], &san("SIN", Gana::Adadi), &["SiSayizate"]);
    assert_has_lat(&["ni"], &san("vi\\Sa~", Gana::Tudadi), &["nivivikzate"]);
    assert_has_lat(
        &["AN"],
        &san("kramu~", Gana::Bhvadi),
        &["AcikraMsate", "AcikraMsati"],
    );

    // From S. C. Vasu's commentary
    assert_has_lat(&[], &d("gupa~\\", Gana::Bhvadi), &["jugupsate"]);
    assert_has_lat(&[], &d("tija~\\", Gana::Bhvadi), &["titikzate"]);
    assert_has_lat(&[], &d("mAna~\\", Gana::Bhvadi), &["mImAMsate"]);
    assert_has_lat(&[], &d("baDa~\\", Gana::Bhvadi), &["bIBatsate"]);
}

#[test]
fn sutra_1_3_64() {
    let yuj = Dhatu::new("yu\\ji~^r", Gana::Rudhadi);

    assert_has_lat(&["pra"], &yuj, &["prayuNkte", "prayunakti"]);
    assert_has_lat(&["upa"], &yuj, &["upayuNkte", "upayunakti"]);
}

#[test]
fn sutra_1_3_65() {
    let ksnu = Dhatu::new("kzRu", Gana::Adadi);

    assert_has_lat(&["sam"], &ksnu, &["saNkzRute"]);
}

// TODO: 1.3.64.v1

#[test]
fn sutra_1_3_66() {
    let bhuj = Dhatu::new("Bu\\ja~", Gana::Rudhadi);
    assert_has_lat(&[], &bhuj, &["Bunakti", "BuNkte"]);
}

#[test]
fn sutra_1_3_73() {
    let vad = Dhatu::new("vada~", Gana::Bhvadi);
    assert_has_lat(&["apa"], &vad, &["apavadati", "apavadate"]);
}

#[test]
fn sutra_1_3_75() {
    let yam = Dhatu::new("ya\\ma~", Gana::Bhvadi);
    assert_has_lat(&["sam"], &yam, &["saMyacCati", "saMyacCate"]);
    assert_has_lat(&["ud"], &yam, &["udyacCati", "udyacCate"]);
    assert_has_lat(&["AN"], &yam, &["AyacCati", "AyacCate"]);
}

#[test]
fn sutra_1_3_76() {
    let jna = Dhatu::new("jYA\\", Gana::Kryadi);
    assert_has_lat(&[], &jna, &["jAnAti", "jAnIte"]);
    assert_has_lat(&["pra"], &jna, &["prajAnAti"]);
}

#[test]
fn sutra_1_3_79() {
    let kr = Dhatu::new("qukf\\Y", Gana::Tanadi);
    assert_has_lat(&["anu"], &kr, &["anukaroti"]);
    assert_has_lat(&["parA"], &kr, &["parAkaroti"]);
}

#[test]
fn sutra_1_3_80() {
    let ksip = Dhatu::new("kzi\\pa~^", Gana::Tudadi);
    assert_has_lat(&["aBi"], &ksip, &["aBikzipati"]);
    assert_has_lat(&["prati"], &ksip, &["pratikzipati"]);
    assert_has_lat(&["ati"], &ksip, &["atikzipati"]);

    assert_has_lat(&["AN"], &ksip, &["Akzipati", "Akzipate"]);
}

#[test]
fn sutra_1_3_81() {
    let vah = Dhatu::new("va\\ha~^", Gana::Bhvadi);
    assert_has_lat(&["pra"], &vah, &["pravahati"]);
    assert_has_lat(&["AN"], &vah, &["Avahati", "Avahate"]);
}

#[test]
fn sutra_1_3_82() {
    let mfs = Dhatu::new("mfza~^", Gana::Divadi);
    assert_has_lat(&["pari"], &mfs, &["parimfzyati"]);
    assert_has_lat(&["AN"], &mfs, &["Amfzyati", "Amfzyate"]);
}

#[test]
fn sutra_1_3_83() {
    let ram = Dhatu::new("ra\\mu~\\", Gana::Bhvadi);
    assert_has_lat(&["vi"], &ram, &["viramati"]);
    assert_has_lat(&["AN"], &ram, &["Aramati"]);
    assert_has_lat(&["pari"], &ram, &["pariramati"]);
    assert_has_lat(&["aBi"], &ram, &["aBiramate"]);
}

#[test]
fn sutra_1_3_85() {
    let ram = Dhatu::new("ra\\ma~\\", Gana::Bhvadi);
    assert_has_lat(&["upa"], &ram, &["uparamati", "uparamate"]);
}
