extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::{BaseKrt as Krt, Gana};

#[test]
fn sk_324() {
    assert_has_sup_1s("lih", Pum, &["liw"]);
    assert_has_sup_1d("lih", Pum, &["lihO"]);
    assert_has_sup_1p("lih", Pum, &["lihaH"]);
    assert_has_sup_2s("lih", Pum, &["liham"]);
    assert_has_sup_2d("lih", Pum, &["lihO"]);
    assert_has_sup_2p("lih", Pum, &["lihaH"]);
    assert_has_sup_3s("lih", Pum, &["lihA"]);
    assert_has_sup_3d("lih", Pum, &["liqByAm"]);
    assert_has_sup_7p("lih", Pum, &["liwtsu", "liwsu"]);
}

#[test]
fn skip_sk_325() {}

#[test]
fn sk_326() {
    assert_has_sup_1s("gardaB", Pum, &["gardap"]);

    let duh = d("du\\ha~^", Gana::Adadi);
    assert_has_krdanta(&[], &duh, Krt::kta, &["dugDa"]);
    assert_has_krdanta(&[], &duh, Krt::tfc, &["dogDf"]);

    let duh = krdanta(&[], &d("du\\ha~^", Adadi), Krt::kvip);
    assert_has_sup_1s(&duh, Pum, &["Duk"]);
    assert_has_sup_1d(&duh, Pum, &["duhO"]);
    assert_has_sup_1p(&duh, Pum, &["duhaH"]);
    assert_has_sup_7p(&duh, Pum, &["Dukzu"]);
}

#[test]
fn sk_327() {
    let druh = create_krdanta("druh", &[], &d("dru\\ha~", Divadi), Krt::kvip);
    assert_has_sup_1s(&druh, Pum, &["Druk", "Druw"]);
    assert_has_sup_1d(&druh, Pum, &["druhO"]);
    assert_has_sup_1p(&druh, Pum, &["druhaH"]);
    assert_has_sup_5d(&druh, Pum, &["DrugByAm", "DruqByAm"]);
    assert_has_sup_7p(&druh, Pum, &["Drukzu", "Druwtsu", "Druwsu"]);

    let vishvavah =
        create_upapada_krdanta("viSvavAh", "viSva", &[], &d("va\\ha~^", Bhvadi), Krt::Rvi);
    assert_has_sup_1s(&vishvavah, Pum, &["viSvavAw"]);
    assert_has_sup_1d(&vishvavah, Pum, &["viSvavAhO"]);
    assert_has_sup_1p(&vishvavah, Pum, &["viSvavAhaH"]);
    assert_has_sup_2s(&vishvavah, Pum, &["viSvavAham"]);
    assert_has_sup_2d(&vishvavah, Pum, &["viSvavAhO"]);
}

#[test]
fn skip_sk_328_to_sk_329() {}

#[test]
fn sk_330() {
    let vishvavah =
        create_upapada_krdanta("viSvavAh", "viSva", &[], &d("va\\ha~^", Bhvadi), Krt::Rvi);
    assert_has_sup_2p(&vishvavah, Pum, &["viSvOhaH"]);
    assert_has_sup_3s(&vishvavah, Pum, &["viSvOhA"]);
}

#[test]
fn skip_sk_331() {}

#[test]
fn sk_332() {
    assert_has_sup_1s("anaquh", Pum, &["anaqvAn"]);
}

#[test]
fn sk_333() {
    assert_has_sup_ss("anaquh", Pum, &["anaqvan"]);
    assert_has_sup_1d("anaquh", Pum, &["anaqvAhO"]);
    assert_has_sup_1p("anaquh", Pum, &["anaqvAhaH"]);
    assert_has_sup_3s("anaquh", Pum, &["anaquhA"]);
}

#[test]
fn sk_334() {
    let vid = d("vida~", Adadi);
    let vidvas = create_krdanta("vidvas", &[], &vid, Krt::Satf);

    assert_has_sup_3d("anaquh", Pum, &["anaqudByAm"]);
    assert_has_sup_1s(&vidvas, Pum, &["vidvAn"]);
    assert_has_krdanta(&[], &d("sransu~\\", Bhvadi), Krt::kta, &["srasta"]);
    assert_has_krdanta(&[], &d("Dvansu~\\", Bhvadi), Krt::kta, &["Dvasta"]);
}

#[test]
fn sk_336() {
    let sudiv = bahuvrihi("su", "div");
    assert_has_sup_1s(&sudiv, Pum, &["sudyOH"]);
    assert_has_sup_1d(&sudiv, Pum, &["sudivO"]);
    assert_has_sup_1p(&sudiv, Pum, &["sudivaH"]);
    assert_has_sup_2s(&sudiv, Pum, &["sudivam"]);
    assert_has_sup_2d(&sudiv, Pum, &["sudivO"]);
}

#[test]
fn sk_337() {
    let sudiv = bahuvrihi("su", "div");
    assert_has_sup_3d(&sudiv, Pum, &["sudyuByAm"]);
    assert_has_sup_3p(&sudiv, Pum, &["sudyuBiH"]);

    assert_has_sup_1p("catur", Pum, &["catvAraH"]);
    assert_has_sup_2p("catur", Pum, &["caturaH"]);
    assert_has_sup_3p("catur", Pum, &["caturBiH"]);
    assert_has_sup_4p("catur", Pum, &["caturByaH"]);
    assert_has_sup_5p("catur", Pum, &["caturByaH"]);
}

#[test]
fn sk_338() {
    // TODO: support dvitvam forms.
    assert_has_sup_6p("catur", Pum, &["caturRAm"]);
}

#[test]
fn skip_sk_339() {}

#[test]
fn sk_340() {
    assert_has_sup_7p("catur", Pum, &["caturzu"]);

    let priyacatur = bahuvrihi("priya", "catur");
    assert_has_sup_1s(&priyacatur, Pum, &["priyacatvAH"]);
    assert_has_sup_ss(&priyacatur, Pum, &["priyacatvaH"]);
    assert_has_sup_1d(&priyacatur, Pum, &["priyacatvArO"]);
    assert_has_sup_1p(&priyacatur, Pum, &["priyacatvAraH"]);

    // TODO: introduce other catur-forms.

    assert_has_sup_1s("kamal", Pum, &["kamal"]);
    assert_has_sup_1d("kamal", Pum, &["kamalO"]);
    assert_has_sup_1p("kamal", Pum, &["kamalaH"]);
    assert_has_sup_7p("kamal", Pum, &["kamalzu"]);
}

#[test]
fn sk_342() {
    assert_has_sup_1s("kim", Pum, &["kaH"]);
    assert_has_sup_1d("kim", Pum, &["kO"]);
    assert_has_sup_1p("kim", Pum, &["ke"]);
    assert_has_sup_2s("kim", Pum, &["kam"]);
    assert_has_sup_2d("kim", Pum, &["kO"]);
    assert_has_sup_2p("kim", Pum, &["kAn"]);
}

#[test]
fn skip_sk_343() {}

#[test]
fn sk_344() {
    assert_has_sup_1s("idam", Pum, &["ayam"]);
}

#[test]
fn sk_345() {
    assert_has_sup_1d("idam", Pum, &["imO"]);
    assert_has_sup_1p("idam", Pum, &["ime"]);
}

#[test]
fn sk_346() {
    assert_has_sup_3s("idam", Pum, &["anena"]);
}

#[test]
fn skip_sk_347() {}

#[test]
fn sk_348() {
    assert_has_sup_3d("idam", Pum, &["AByAm"]);
}

#[test]
fn sk_349() {
    assert_has_sup_3p("idam", Pum, &["eBiH"]);
    assert_has_sup_4s("idam", Pum, &["asmE"]);
    assert_has_sup_4d("idam", Pum, &["AByAm"]);
    assert_has_sup_4p("idam", Pum, &["eByaH"]);
    assert_has_sup_5s("idam", Pum, &["asmAt"]);
    assert_has_sup_5d("idam", Pum, &["AByAm"]);
    assert_has_sup_5p("idam", Pum, &["eByaH"]);
    assert_has_sup_6s("idam", Pum, &["asya"]);
    assert_has_sup_6d("idam", Pum, &["anayoH"]);
    assert_has_sup_6p("idam", Pum, &["ezAm"]);
    assert_has_sup_7s("idam", Pum, &["asmin"]);
    assert_has_sup_7d("idam", Pum, &["anayoH"]);
    assert_has_sup_7p("idam", Pum, &["ezu"]);

    // TODO: ayaka
}

#[test]
fn skip_sk_350() {}

#[test]
fn sk_354() {
    assert_has_sup_1s("yajvan", Pum, &["yajvA"]);
    assert_has_sup_1d("yajvan", Pum, &["yajvAnO"]);
    assert_has_sup_1p("yajvan", Pum, &["yajvAnaH"]);
}

#[test]
fn sk_355() {
    assert_has_sup_2p("yajvan", Pum, &["yajvanaH"]);
    assert_has_sup_3s("yajvan", Pum, &["yajvanA"]);
    assert_has_sup_3d("yajvan", Pum, &["yajvaByAm"]);

    assert_has_sup_2p("brahman", Pum, &["brahmaRaH"]);
    assert_has_sup_3s("brahman", Pum, &["brahmaRA"]);
    assert_has_sup_3d("brahman", Pum, &["brahmaByAm"]);
}

#[test]
fn skip_sk_356() {}

#[ignore]
#[test]
fn sk_357() {
    let vrtrahan =
        create_upapada_krdanta("vftrahan", "vftra", &[], &d("ha\\na~", Adadi), Krt::kvip);
    assert_has_sup_1s(&vrtrahan, Pum, &["vftrahA"]);
    assert_has_sup_ss(&vrtrahan, Pum, &["vftrahan"]);
    assert_has_sup_1d(&vrtrahan, Pum, &["vftrahaRO"]);
    assert_has_sup_1p(&vrtrahan, Pum, &["vftrahaRaH"]);
    assert_has_sup_2s(&vrtrahan, Pum, &["vftrahaRam"]);
    assert_has_sup_2d(&vrtrahan, Pum, &["vftrahaRO"]);
}

#[test]
fn skip_sk_358() {}

#[test]
fn skip_sk_360() {}

#[ignore]
#[test]
fn sk_361() {
    assert_has_sup_1s("maGavan", Pum, &["maGavAn", "maGavA"]);
    assert_has_sup_1s("maGavan", Pum, &["maGavantO", "maGavAnO"]);
    assert_has_sup_1s("maGavan", Pum, &["maGavantaH", "maGavAnaH"]);
    assert_has_sup_ss("maGavan", Pum, &["maGavan"]);
    assert_has_sup_ss("maGavan", Pum, &["maGavantam", "maGavAnam"]);
    assert_has_sup_ss("maGavan", Pum, &["maGavantO", "maGavAnO"]);
    assert_has_sup_ss("maGavan", Pum, &["maGavataH", "maGavAnaH"]);
    assert_has_sup_ss("maGavan", Pum, &["maGavatA", "maGonA"]);
    assert_has_sup_ss("maGavan", Pum, &["maGavadByAm", "maGavaByAm"]);
    assert_has_sup_ss("maGavan", Pum, &["maGavadByAm", "maGavaByAm"]);

    // TODO: others (maGavA?)
}

#[ignore]
#[test]
fn sk_362() {
    assert_has_sup_6s("maGavan", Pum, &["maGonaH", "maGavataH"]);
    assert_has_sup_2p("maGavan", Pum, &["maGavataH"]);
    assert_has_sup_3s("maGavan", Pum, &["maGavatA"]);
}

#[test]
fn skip_sk_365_to_sk_366() {}

#[test]
fn sk_367() {
    assert_has_sup_1s("paTin", Pum, &["panTAH"]);
    assert_has_sup_1d("paTin", Pum, &["panTAnO"]);
    assert_has_sup_1p("paTin", Pum, &["panTAnaH"]);
    assert_has_sup_2s("paTin", Pum, &["panTAnam"]);
    assert_has_sup_2d("paTin", Pum, &["panTAnO"]);
}

#[test]
fn sk_368() {
    assert_has_sup_2p("paTin", Pum, &["paTaH"]);
    assert_has_sup_3s("paTin", Pum, &["paTA"]);
    assert_has_sup_3d("paTin", Pum, &["paTiByAm"]);

    assert_has_sup_1s("maTin", Pum, &["manTAH"]);
    assert_has_sup_1s("fBukzin", Pum, &["fBukzAH"]);

    // TODO: anfBukzI and others
}

#[test]
fn skip_sk_369() {
    assert_has_sup_1p("paYcan", Pum, &["paYca"]);
    assert_has_sup_2p("paYcan", Pum, &["paYca"]);
    assert_has_sup_3p("paYcan", Pum, &["paYcaBiH"]);
    assert_has_sup_4p("paYcan", Pum, &["paYcaByaH"]);
    assert_has_sup_5p("paYcan", Pum, &["paYcaByaH"]);

    // TODO: others
}

#[test]
fn skip_sk_371() {}

#[test]
fn skip_sk_374_to_sk_376() {}

#[test]
fn sk_377() {
    let yuj = create_krdanta("yuj", &[], &d("yu\\ji~^r", Rudhadi), Krt::kvin);
    assert_has_sup_1s(&yuj, Pum, &["yuN"]);
    assert_has_sup_1d(&yuj, Pum, &["yuYjO"]);
    assert_has_sup_1p(&yuj, Pum, &["yuYjaH"]);
    assert_has_sup_2s(&yuj, Pum, &["yuYjam"]);
    assert_has_sup_2d(&yuj, Pum, &["yuYjO"]);
    assert_has_sup_2p(&yuj, Pum, &["yujaH"]);
    assert_has_sup_3d(&yuj, Pum, &["yugByAm"]);
}

#[test]
fn sk_378() {
    let yuj = create_krdanta("yuj", &[], &d("yu\\ji~^r", Rudhadi), Krt::kvin);
    let suyuj = bahuvrihi("su", yuj);
    assert_has_sup_1s(&suyuj, Pum, &["suyuk"]);
    assert_has_sup_1d(&suyuj, Pum, &["suyujO"]);
    assert_has_sup_1p(&suyuj, Pum, &["suyujaH"]);

    // TODO: do others here

    let khanj = krdanta(&[], &d("Kaji~", Bhvadi), Krt::kvip);
    assert_has_sup_1s(&khanj, Pum, &["Kan"]);
    assert_has_sup_1d(&khanj, Pum, &["KaYjO"]);
    assert_has_sup_1p(&khanj, Pum, &["KaYjaH"]);

    let raj = krdanta(&[], &d("rAjf~^", Bhvadi), Krt::kvip);
    assert_has_sup_1s(&raj, Pum, &["rAw"]);
    assert_has_sup_1d(&raj, Pum, &["rAjO"]);
    assert_has_sup_1p(&raj, Pum, &["rAjaH"]);
    assert_has_sup_7p(&raj, Pum, &["rAwsu", "rAwtsu"]);
}

#[test]
fn skip_sk_382_to_sk_384() {}

#[test]
fn skip_sk_386() {}

#[test]
fn sk_388() {
    assert_has_sup_1p("yuzmad", Pum, &["yUyam"]);
    assert_has_sup_1p("asmad", Pum, &["vayam"]);

    assert_has_sup_1p(karmadharaya("parama", "yuzmad"), Pum, &["paramayUyam"]);
    assert_has_sup_1p(karmadharaya("parama", "asmad"), Pum, &["paramavayam"]);
    assert_has_sup_1p(karmadharaya("ati", "yuzmad"), Pum, &["atiyUyam"]);
    assert_has_sup_1p(karmadharaya("ati", "asmad"), Pum, &["ativayam"]);
}

#[test]
fn skip_sk_389() {}

#[test]
fn sk_390() {
    assert_has_sup_2s("yuzmad", Pum, &["tvAm"]);
    assert_has_sup_2s("asmad", Pum, &["mAm"]);
    assert_has_sup_2d("yuzmad", Pum, &["yuvAm"]);
    assert_has_sup_2d("asmad", Pum, &["AvAm"]);
}

#[test]
fn sk_391() {
    assert_has_sup_2p("yuzmad", Pum, &["yuzmAn"]);
    assert_has_sup_2p("asmad", Pum, &["asmAn"]);
}

#[test]
fn sk_392() {
    assert_has_sup_3s("yuzmad", Pum, &["tvayA"]);
    assert_has_sup_3s("asmad", Pum, &["mayA"]);
}

#[test]
fn sk_393() {
    assert_has_sup_3d("yuzmad", Pum, &["yuvAByAm"]);
    assert_has_sup_3d("asmad", Pum, &["AvAByAm"]);
    assert_has_sup_3p("yuzmad", Pum, &["yuzmABiH"]);
    assert_has_sup_3p("asmad", Pum, &["asmABiH"]);
}

#[test]
fn sk_394() {
    assert_has_sup_4s("yuzmad", Pum, &["tuByam"]);
    assert_has_sup_4s("asmad", Pum, &["mahyam"]);

    assert_has_sup_4s(karmadharaya("parama", "yuzmad"), Pum, &["paramatuByam"]);
    assert_has_sup_4s(karmadharaya("parama", "asmad"), Pum, &["paramamahyam"]);
    assert_has_sup_4s(karmadharaya("ati", "yuzmad"), Pum, &["atituByam"]);
    assert_has_sup_4s(karmadharaya("ati", "asmad"), Pum, &["atimahyam"]);

    assert_has_sup_4d("yuzmad", Pum, &["yuvAByAm"]);
    assert_has_sup_4d("asmad", Pum, &["AvAByAm"]);
}

#[test]
fn sk_395() {
    assert_has_sup_4p("yuzmad", Pum, &["yuzmaByam"]);
    assert_has_sup_4p("asmad", Pum, &["asmaByam"]);
}

#[test]
fn sk_396() {
    assert_has_sup_5s("yuzmad", Pum, &["tvat"]);
    assert_has_sup_5s("asmad", Pum, &["mat"]);
    assert_has_sup_5d("yuzmad", Pum, &["yuvAByAm"]);
    assert_has_sup_5d("asmad", Pum, &["AvAByAm"]);
}

#[test]
fn sk_397() {
    assert_has_sup_5p("yuzmad", Pum, &["yuzmat"]);
    assert_has_sup_5p("asmad", Pum, &["asmat"]);
}

#[test]
fn skip_sk_398() {}

#[test]
fn sk_399() {
    assert_has_sup_6s("yuzmad", Pum, &["tava"]);
    assert_has_sup_6s("asmad", Pum, &["mama"]);
    assert_has_sup_6d("yuzmad", Pum, &["yuvayoH"]);
    assert_has_sup_6d("asmad", Pum, &["AvayoH"]);
}

#[test]
fn skip_sk_401_to_sk_406() {}

#[test]
fn skip_sk_411() {}

#[test]
fn skip_sk_416() {}

#[test]
fn skip_sk_426() {}

#[test]
fn sk_427() {
    let dadat = create_krdanta("dadat", &[], &d("qudA\\Y", Juhotyadi), Krt::Satf);
    assert_has_sup_1s(&dadat, Pum, &["dadat"]);
    assert_has_sup_1d(&dadat, Pum, &["dadatO"]);
    assert_has_sup_1p(&dadat, Pum, &["dadataH"]);
}

#[test]
fn sk_428() {
    let jakshat = create_krdanta("jakzat", &[], &d("jakza~", Adadi), Krt::Satf);
    assert_has_sup_1s(&jakshat, Pum, &["jakzat"]);
    assert_has_sup_1d(&jakshat, Pum, &["jakzatO"]);
    assert_has_sup_1p(&jakshat, Pum, &["jakzataH"]);

    let jagrat = create_krdanta("jAgrat", &[], &d("jAgf", Adadi), Krt::Satf);
    assert_has_sup_1s(&jagrat, Pum, &["jAgrat"]);

    let daridrat = create_krdanta("daridrat", &[], &d("daridrA", Adadi), Krt::Satf);
    assert_has_sup_1s(&daridrat, Pum, &["daridrat"]);

    let shasat = create_krdanta("SAsat", &[], &d("SAsu~", Adadi), Krt::Satf);
    assert_has_sup_1s(&shasat, Pum, &["SAsat"]);

    let cakasat = create_krdanta("cakAsat", &[], &d("cakAsf~", Adadi), Krt::Satf);
    assert_has_sup_1s(&cakasat, Pum, &["cakAsat"]);

    // TODO: didhyat, vevyat (needs parasmaipadam, available ad-hoc)
    // let didhyat = create_krdanta("dIDyat", &[], &d("dIDIN", Adadi), Krt::Satf);
    // assert_has_sup_1s(&didhyat, Pum, &["dIDyat"]);
    // let vevyat = create_krdanta("vevyat", &[], &d("vevIN", Adadi), Krt::Satf);
    // assert_has_sup_1s(&vevyat, Pum, &["vevyat"]);

    assert_has_sup_1s("gup", Pum, &["gup"]);
    assert_has_sup_1d("gup", Pum, &["gupO"]);
    assert_has_sup_1p("gup", Pum, &["gupaH"]);
    assert_has_sup_3d("gup", Pum, &["gubByAm"]);
}

#[test]
fn skip_sk_429() {}

#[test]
fn skip_sk_431() {
    let nash = create_krdanta("naS", &[], &d("Ra\\Sa~", Divadi), Krt::kvip);
    assert_has_sup_1s(&nash, Pum, &["nak", "naw"]);
    assert_has_sup_1d(&nash, Pum, &["naSO"]);
    assert_has_sup_1p(&nash, Pum, &["naSaH"]);
    assert_has_sup_3d(&nash, Pum, &["nagByAm", "naqByAm"]);
}

#[test]
fn sk_433() {
    let pipathis = krdanta(&[], &san(&d("paWa~", Bhvadi)), Krt::kvip);
    assert_has_sup_1s(&pipathis, Pum, &["pipaWIH"]);
    assert_has_sup_1d(&pipathis, Pum, &["pipaWizO"]);
    assert_has_sup_1p(&pipathis, Pum, &["pipaWizaH"]);
    assert_has_sup_3d(&pipathis, Pum, &["pipaWIrByAm"]);
}

#[test]
fn sk_434() {
    let pipathis = krdanta(&[], &san(&d("paWa~", Bhvadi)), Krt::kvip);
    assert_has_sup_7p(&pipathis, Pum, &["pipaWIzzu", "pipaWIHzu"]);

    let nins = d("Risi~\\", Adadi);
    assert_has_thaas(&[], &nins, Lot, &["niMssva"]);
    assert_has_thaas(&[], &nins, Lat, &["niMsse"]);

    assert_has_sup_7p("suhins", Pum, &["suhinsu"]);
    assert_has_sup_7p("pums", Pum, &["puMsu"]);

    let cikirsh = krdanta(&[], &san(&d("qukf\\Y", Tanadi)), Krt::kvip);
    assert_has_sup_1s(&cikirsh, Pum, &["cikIH"]);
    assert_has_sup_1d(&cikirsh, Pum, &["cikIrzO"]);
    assert_has_sup_1p(&cikirsh, Pum, &["cikIrzaH"]);

    // TODO: fix this. Issue is that our samyogAntasya lopa is buggy.
    // assert_has_sup_7p(&cikirsh, Pum, &["cikIrzu"]);

    let taksh = create_krdanta("takz", &[], &d("takzU~", Bhvadi), Krt::kvip);
    assert_has_sup_1s(&taksh, Pum, &["taw"]);
    assert_has_sup_1d(&taksh, Pum, &["takzO"]);
    assert_has_sup_1p(&taksh, Pum, &["takzaH"]);

    let goraksh = create_upapada_krdanta("gorakz", "go", &[], &d("rakza~", Bhvadi), Krt::kvip);
    assert_has_sup_1s(&goraksh, Pum, &["goraw"]);
    assert_has_sup_1d(&goraksh, Pum, &["gorakzO"]);
    assert_has_sup_1p(&goraksh, Pum, &["gorakzaH"]);

    let pis = create_krdanta("pis", &[], &d("pisf~", Bhvadi), Krt::kvip);
    let supis = bahuvrihi("su", pis);
    assert_has_sup_1s(&supis, Pum, &["supIH"]);
    assert_has_sup_1d(&supis, Pum, &["supisO"]);
    assert_has_sup_1p(&supis, Pum, &["supisaH"]);
    assert_has_sup_3s(&supis, Pum, &["supisA"]);
    assert_has_sup_3d(&supis, Pum, &["supIrByAm"]);
    assert_has_sup_7p(&supis, Pum, &["supIHzu", "supIzzu"]);
}

#[test]
fn sk_437() {
    assert_has_sup_1s("adas", Pum, &["asO"]);
    assert_has_sup_1d("adas", Pum, &["amU"]);

    // TODO: asakO, asukaH
}

#[test]
fn sk_438() {
    assert_has_sup_1p("adas", Pum, &["amI"]);
    assert_has_sup_2s("adas", Pum, &["amum"]);
    assert_has_sup_2d("adas", Pum, &["amU"]);
    assert_has_sup_2p("adas", Pum, &["amUn"]);
}

#[test]
fn sk_439() {
    assert_has_sup_3s("adas", Pum, &["amunA"]);
    assert_has_sup_3d("adas", Pum, &["amUByAm"]);
    assert_has_sup_3p("adas", Pum, &["amIBiH"]);
    assert_has_sup_4s("adas", Pum, &["amuzmE"]);
    assert_has_sup_4p("adas", Pum, &["amIByaH"]);
    assert_has_sup_5s("adas", Pum, &["amuzmAt"]);
    assert_has_sup_6s("adas", Pum, &["amuzya"]);
    assert_has_sup_6d("adas", Pum, &["amuyoH"]);
    assert_has_sup_6p("adas", Pum, &["amIzAm"]);
    assert_has_sup_7s("adas", Pum, &["amuzmin"]);
    assert_has_sup_7d("adas", Pum, &["amuyoH"]);
    assert_has_sup_7p("adas", Pum, &["amIzu"]);
}
