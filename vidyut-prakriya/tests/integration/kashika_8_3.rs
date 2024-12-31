extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Dhatu;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Pratipadika;
use vidyut_prakriya::args::Sanadi;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha::*;
use vidyut_prakriya::args::Unadi;

fn kamyac(prati: &str) -> Dhatu {
    Dhatu::nama(
        Pratipadika::basic(prati.try_into().expect("ok")),
        Some(Sanadi::kAmyac),
    )
}

#[test]
fn sutra_8_3_13() {
    assert_has_krdanta(&[], &d("li\\ha~^", Adadi), Krt::kta, &["lIQa"]);
    assert_has_krdanta(&["upa"], &d("guhU~^", Bhvadi), Krt::kta, &["upagUQa"]);
}

#[test]
fn sutra_8_3_14() {
    let raj = d("ra\\nja~^", Bhvadi);
    assert_has_krdanta(&["nis"], &raj, Krt::kta, &["nIrakta"]);
    assert_has_krdanta(&["nir"], &raj, Krt::kta, &["nIrakta"]);
    assert_has_krdanta(&["dus"], &raj, Krt::kta, &["dUrakta"]);
    assert_has_krdanta(&["dur"], &raj, Krt::kta, &["dUrakta"]);
}

#[test]
fn sutra_8_3_15() {
    // hari
    assert_has_sandhi("vfkzas", "CAdayati", &["vfkzaS CAdayati"]);
    assert_has_sandhi("plakzas", "CAdayati", &["plakzaS CAdayati"]);
    assert_has_sandhi("vfkzas", "tarati", &["vfkzas tarati"]);
    assert_has_sandhi("plakzas", "tarati", &["plakzas tarati"]);
    // avasAne
    assert_has_sup_1s("vfkza", Pum, &["vfkzaH"]);
    assert_has_sup_1s("plakza", Pum, &["plakzaH"]);

    // khar-avasAnayoH?
    assert_has_sandhi("agnis", "nayati", &["agnir nayati"]);
    assert_has_sandhi("vAyus", "nayati", &["vAyur nayati"]);
}

#[test]
fn sutra_8_3_16() {
    assert_has_sup_7p("payas", Napumsaka, &["payaHsu", "payassu"]);
    assert_has_sup_7p("sarpis", Napumsaka, &["sarpiHzu", "sarpizzu"]);
    assert_has_sup_7p("yaSas", Napumsaka, &["yaSaHsu", "yaSassu"]);
    // ruH
    let gir = krdanta(&[], &d("gF", Kryadi), Krt::kvip);
    assert_has_sup_7p(gir, Napumsaka, &["gIrzu"]);
    let dhur = krdanta(&[], &d("DurvI~", Bhvadi), Krt::kvip);
    assert_has_sup_7p(&dhur, Napumsaka, &["DUrzu"]);
}

#[test]
fn sutra_8_3_17() {
    assert_has_sandhi("Bos", "atra", &["Bo atra"]);
    assert_has_sandhi("Bagos", "atra", &["Bago atra"]);
    assert_has_sandhi("aGos", "atra", &["aGo atra"]);

    assert_has_sandhi("Bos", "dadAti", &["Bo dadAti"]);
    assert_has_sandhi("Bagos", "dadAti", &["Bago dadAti"]);
    assert_has_sandhi("aGos", "dadAti", &["aGo dadAti"]);

    assert_has_sandhi("kas", "Aste", &["ka Aste"]);
    assert_has_sandhi("puruzAs", "dadati", &["puruzA dadati"]);

    // Bo-Bago-... ?
    assert_has_sandhi("agnis", "atra", &["agnir atra"]);
    assert_has_sandhi("vAyus", "atra", &["vAyur atra"]);

    // aS?
    assert_has_sup_1s("vfkza", Pum, &["vfkzaH"]);
    assert_has_sup_1s("plakza", Pum, &["plakzaH"]);

    // TODO: others
}

#[test]
fn sutra_8_3_22() {
    assert_has_sandhi("Bos", "hasati", &["Bo hasati"]);
    assert_has_sandhi("Bagos", "hasati", &["Bago hasati"]);
    assert_has_sandhi("aGos", "hasati", &["aGo hasati"]);
    assert_has_sandhi("Bos", "yAti", &["Bo yAti"]);
    assert_has_sandhi("Bagos", "yAti", &["Bago yAti"]);
    assert_has_sandhi("aGos", "yAti", &["aGo yAti"]);
    assert_has_sandhi("vfkzAs", "hasanti", &["vfkzA hasanti"]);
}

#[test]
fn sutra_8_3_23() {
    assert_has_sandhi("kuRqam", "hasati", &["kuRqaM hasati"]);
    assert_has_sandhi("vanam", "hasati", &["vanaM hasati"]);
    assert_has_sandhi("kuRqam", "yAti", &["kuRqaM yAti"]);
    assert_has_sandhi("vanam", "yAti", &["vanaM yAti"]);

    // hali
    assert_has_sandhi("tvam", "atra", &["tvam atra"]);
    assert_has_sandhi("kim", "atra", &["kim atra"]);

    // padAntasya
    assert_has_ta_k(&[], &d("ga\\mx~", Bhvadi), Lat, &["gamyate"]);
    assert_has_ta_k(&[], &d("ra\\ma~\\", Bhvadi), Lat, &["ramyate"]);
}

#[test]
fn sutra_8_3_24() {
    assert_has_sup_1p("payas", Napumsaka, &["payAMsi"]);
    assert_has_sup_1p("yaSas", Napumsaka, &["yaSAMsi"]);

    let sarpis = create_krdanta("sarpis", &[], &d("sf\\px~", Bhvadi), Unadi::isi);
    assert_has_sup_1p(&sarpis, Napumsaka, &["sarpIMzi"]);

    // makArasya
    let kram = d("kramu~", Bhvadi);
    assert_has_ta(&["AN"], &kram, Lrt, &["AkraMsyate"]);
    assert_has_ta_k(&["AN"], &san(&kram), Lat, &["AcikraMsyate"]);
    assert_has_lat(&["aDi"], &san(&d("i\\N", Adadi)), &["aDijigAMsate"]);

    // Jali
    assert_has_ta_k(&[], &d("ra\\ma~\\", Bhvadi), Lat, &["ramyate"]);
    assert_has_ta_k(&[], &d("ga\\mx~", Bhvadi), Lat, &["gamyate"]);
}

#[test]
fn sutra_8_3_29() {
    assert_has_sandhi("Svaliw", "sAye", &["Svaliw sAye", "Svaliw tsAye"]);
    assert_has_sandhi("maDuliw", "sAye", &["maDuliw sAye", "maDuliw tsAye"]);
}

#[test]
fn sutra_8_3_34() {
    // hari
    assert_has_sandhi("vfkzas", "CAdayati", &["vfkzaS CAdayati"]);
    assert_has_sandhi("plakzas", "CAdayati", &["plakzaS CAdayati"]);
    assert_has_sandhi("vfkzas", "WakAraH", &["vfkzaz WakAraH"]);
    assert_has_sandhi("plakzas", "WakAraH", &["plakzaz WakAraH"]);
    assert_has_sandhi("vfkzas", "TakAraH", &["vfkzas TakAraH"]);
    assert_has_sandhi("plakzas", "TakAraH", &["plakzas TakAraH"]);
    assert_has_sandhi("vfkzas", "cinoti", &["vfkzaS cinoti"]);
    assert_has_sandhi("plakzas", "cinoti", &["plakzaS cinoti"]);
    assert_has_sandhi("vfkzas", "wIvati", &["vfkzaz wIvati"]);
    assert_has_sandhi("plakzas", "wIvati", &["plakzaz wIvati"]);
    assert_has_sandhi("vfkzas", "tarati", &["vfkzas tarati"]);
    assert_has_sandhi("plakzas", "tarati", &["plakzas tarati"]);
}

#[test]
fn sutra_8_3_35() {
    assert_has_sandhi("SaSas", "kzuram", &["SaSaH kzuram"]);
    assert_has_sandhi("puruzas", "kzuram", &["puruzaH kzuram"]);
    assert_has_sandhi("adBis", "psAtam", &["adBiH psAtam"]);
    assert_has_sandhi("vAsas", "kzOmam", &["vAsaH kzOmam"]);
    assert_has_sandhi("puruzas", "tsaruH", &["puruzaH tsaruH"]);
}

#[test]
fn sutra_8_3_36() {
    assert_has_sandhi("vfkzas", "Sete", &["vfkzaS Sete", "vfkzaH Sete"]);
    assert_has_sandhi("plakzas", "Sete", &["plakzaS Sete", "plakzaH Sete"]);
    assert_has_sandhi("vfkzas", "zaRqe", &["vfkzaz zaRqe", "vfkzaH zaRqe"]);
    assert_has_sandhi("vfkzas", "sAye", &["vfkzas sAye", "vfkzaH sAye"]);
}

#[test]
fn sutra_8_3_38() {
    assert_has_taddhita("payas", T::pASap, &["payaspASa"]);
    assert_has_taddhita("payas", T::kalpap, &["payaskalpa"]);
    assert_has_taddhita("yaSas", T::kalpap, &["yaSaskalpa"]);

    assert_has_taddhita("payas", T::ka, &["payaska"]);
    assert_has_taddhita("yaSas", T::ka, &["yaSaska"]);

    assert_has_tip(&[], &kamyac("payas"), Lat, &["payaskAmyati"]);
    assert_has_tip(&[], &kamyac("yaSas"), Lat, &["yaSaskAmyati"]);

    // TODO: jihvA/upaDmAnIya
}

#[test]
fn sutra_8_3_39() {
    assert_has_taddhita("sarpis", T::pASap, &["sarpizpASa"]);
    assert_has_taddhita("yajus", T::pASap, &["yajuzpASa"]);
    assert_has_taddhita("sarpis", T::kalpap, &["sarpizkalpa"]);
    assert_has_taddhita("yajus", T::kalpap, &["yajuzkalpa"]);
    assert_has_taddhita("sarpis", T::ka, &["sarpizka"]);
    assert_has_taddhita("yajus", T::ka, &["yajuzka"]);

    assert_has_tip(&[], &kamyac("sarpis"), Lat, &["sarpizkAmyati"]);
    assert_has_tip(&[], &kamyac("yajus"), Lat, &["yajuzkAmyati"]);

    // a-padAdO?
    // TODO: need to create these from unAdi-pratyayas.
    // assert_has_sandhi("agniH", "karoti", &["agniH karoti"]);
    // assert_has_sandhi("vAyuH", "karoti", &["vAyuH karoti"]);
    // assert_has_sandhi("agniH", "pacati", &["agniH pacati"]);
    // assert_has_sandhi("vAyuH", "pacati", &["vAyuH pacati"]);

    // TODO: sarpiste? how do we construct that?
}

#[test]
fn sutra_8_3_40() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&["namas"], &kr, Krt::tfc, &["namaskartf"]);
    assert_has_krdanta(&["namas"], &kr, Krt::tumun, &["namaskartum"]);
    assert_has_krdanta(&["namas"], &kr, Krt::tavya, &["namaskartavya"]);

    // TODO: gatyoH?
}

#[test]
fn sutra_8_3_42() {
    let kr = d("qukf\\Y", Tanadi);
    assert_has_tip(&["tiras"], &kr, Lat, &["tiraskaroti", "tiraHkaroti"]);
    assert_has_krdanta(&["tiras"], &kr, Krt::tfc, &["tiraskartf", "tiraHkartf"]);
    assert_has_krdanta(&["tiras"], &kr, Krt::tumun, &["tiraskartum", "tiraHkartum"]);
    assert_has_krdanta(
        &["tiras"],
        &kr,
        Krt::tavya,
        &["tiraskartavya", "tiraHkartavya"],
    );
}

#[test]
fn sutra_8_3_55() {
    assert_has_tip(&[], &d("zi\\ca~^", Tudadi), Lit, &["sizeca"]);
    assert_has_tip(&[], &d("Yizva\\pa~", Adadi), Lit, &["suzvApa"]);
    assert_has_sup_7p("agni", Pum, &["agnizu"]);
    assert_has_sup_7p("vAyu", Pum, &["vAyuzu"]);

    let kf = d("qukf\\Y", Tanadi);
    assert_has_dhvam(&[], &kf, Lun, &["akfQvam"]);
    assert_has_dhvam(&[], &kf, Lit, &["cakfQve"]);

    // TODO: apadAntasya
}

#[test]
fn sutra_8_3_59() {
    assert_has_tip(&[], &d("zivu~", Divadi), Lit, &["sizeva"]);
    assert_has_tip(&[], &d("Yizva\\pa~", Adadi), Lit, &["suzvApa"]);
    assert_has_sup_7p("agni", Pum, &["agnizu"]);
    assert_has_sup_7p("vAyu", Pum, &["vAyuzu"]);
    assert_has_sup_7p("kartf", Pum, &["kartfzu"]);
    assert_has_sup_7p("hartf", Pum, &["hartfzu"]);

    // TODO: others
}

#[test]
fn sutra_8_3_60() {
    let shas = d("SAsu~", Adadi);
    assert_has_tip(&["anu"], &shas, Lun, &["anvaSizat"]);
    assert_has_tas(&["anu"], &shas, Lun, &["anvaSizatAm"]);
    assert_has_jhi(&["anu"], &shas, Lun, &["anvaSizan"]);
    assert_has_krdanta(&[], &shas, Krt::kta, &["Sizwa"]);
    assert_has_krdanta(&[], &shas, Krt::ktavatu, &["Sizwavat"]);

    let vas = d("va\\sa~", Bhvadi);
    assert_has_krdanta(&[], &vas, Krt::kta, &["uzita"]);
    assert_has_krdanta(&[], &vas, Krt::ktavatu, &["uzitavat"]);
    assert_has_krdanta(&[], &vas, Krt::ktvA, &["uzitvA"]);

    let ad = d("a\\da~", Adadi);
    assert_has_tas(&[], &ad, Lit, &["jakzatuH", "AdatuH"]);
    assert_has_jhi(&[], &ad, Lit, &["jakzuH", "AduH"]);

    // TODO: others
}

#[test]
fn sutra_8_3_62() {
    let svid_nic_san = nic_san(&d("YizvidA~", Bhvadi));
    assert_has_tip(&[], &svid_nic_san, Lat, &["sisvedayizati"]);

    let svad_nic_san = nic_san(&d("zvada~\\", Bhvadi));
    assert_has_tip(&[], &svad_nic_san, Lat, &["sisvAdayizati"]);

    let sah_nic_san = nic_san(&d("zaha~\\", Bhvadi));
    assert_has_tip(&[], &sah_nic_san, Lat, &["sisAhayizati"]);
}

#[test]
fn sutra_8_3_61() {
    let svap = d("Yizva\\pa~", Adadi);
    let sic = d("zi\\ca~^", Tudadi);
    assert_has_tip(&[], &san(&d("zwu\\Y", Adadi)), Lat, &["tuzwUzati"]);
    assert_has_tip(&[], &nic_san(&d("zevf~\\", Bhvadi)), Lat, &["sizevayizati"]);
    assert_has_tip(
        &[],
        &nic_san(&d("za\\nja~", Bhvadi)),
        Lat,
        &["sizaYjayizati"],
    );
    assert_has_tip(&[], &nic_san(&svap), Lat, &["suzvApayizati"]);
    assert_has_tip(&[], &san(&sic), Lat, &["sisikzati"]);
    assert_has_tip(&[], &san(&d("zu\\", Bhvadi)), Lat, &["susUzati"]);
    // zaRi
    assert_has_tip(&[], &sic, Lit, &["sizeca"]);
    assert_has_lat(&[], &san(&svap), &["suzupsati"]);
    assert_has_tip(&[], &san(&d("zWA\\", Bhvadi)), Lat, &["tizWAsati"]);

    // TODO: others
}

#[test]
fn sutra_8_3_63() {
    let su = d("zu\\Y", Svadi);
    assert_has_tip(&["aBi"], &su, Lat, &["aBizuRoti"]);
    assert_has_tip(&["pari"], &su, Lat, &["parizuRoti"]);
    assert_has_tip(&["vi"], &su, Lat, &["vizuRoti"]);
    assert_has_tip(&["ni"], &su, Lat, &["nizuRoti"]);
    assert_has_tip(&["aBi"], &su, Lan, &["aByazuRot"]);
    assert_has_tip(&["pari"], &su, Lan, &["paryazuRot"]);
    assert_has_tip(&["vi"], &su, Lan, &["vyazuRot"]);
    assert_has_tip(&["ni"], &su, Lan, &["nyazuRot"]);
}

#[test]
fn sutra_8_3_64() {
    // TODO: others
    let stha = d("zWA\\", Bhvadi);
    assert_has_tip(&["aBi"], &stha, Lit, &["aBitazWO"]);
    assert_has_tip(&["pari"], &stha, Lit, &["paritazWO"]);

    let sic = d("zi\\ca~^", Tudadi);
    assert_has_tip(&["aBi"], &san(&sic), Lat, &["aBizizikzati"]);
    assert_has_tip(&["pari"], &san(&sic), Lat, &["parizizikzati"]);

    let sush = d("zu\\Y", Svadi);
    assert_has_tip(&["aBi"], &san(&sush), Lat, &["aBisusUzati"]);
}

#[test]
fn sutra_8_3_65() {
    let su = d("zu\\Y", Svadi);
    assert_has_tip(&["aBi"], &su, Lat, &["aBizuRoti"]);
    assert_has_tip(&["pari"], &su, Lat, &["parizuRoti"]);
    assert_has_tip(&["aBi"], &su, Lan, &["aByazuRot"]);
    assert_has_tip(&["pari"], &su, Lan, &["paryazuRot"]);

    let suu = d("zU", Tudadi);
    assert_has_tip(&["aBi"], &suu, Lat, &["aBizuvati"]);
    assert_has_tip(&["pari"], &suu, Lat, &["parizuvati"]);
    assert_has_tip(&["aBi"], &suu, Lan, &["aByazuvat"]);
    assert_has_tip(&["pari"], &suu, Lan, &["paryazuvat"]);

    let so = d("zo\\", Divadi);
    assert_has_tip(&["aBi"], &so, Lat, &["aBizyati"]);
    assert_has_tip(&["pari"], &so, Lat, &["parizyati"]);
    assert_has_tip(&["aBi"], &so, Lan, &["aByazyat"]);
    assert_has_tip(&["pari"], &so, Lan, &["paryazyat"]);

    let stu = d("zwu\\Y", Adadi);
    assert_has_tip(&["aBi"], &stu, Lat, &["aBizwOti", "aBizwavIti"]);
    assert_has_tip(&["pari"], &stu, Lat, &["parizwOti", "parizwavIti"]);
    assert_has_tip(&["aBi"], &stu, Lan, &["aByazwOt", "aByazwavIt"]);
    // non-shatva forms are by 8.3.71.
    assert_has_tip(
        &["pari"],
        &stu,
        Lan,
        &["paryazwOt", "paryazwavIt", "paryastOt", "paryastavIt"],
    );

    let stubh = d("zwuBu~\\", Bhvadi);
    assert_has_ta(&["aBi"], &stubh, Lat, &["aBizwoBate"]);
    assert_has_ta(&["pari"], &stubh, Lat, &["parizwoBate"]);
    assert_has_ta(&["aBi"], &stubh, Lan, &["aByazwoBata"]);
    assert_has_ta(&["pari"], &stubh, Lan, &["paryazwoBata"]);

    let sthaa = d("zWA\\", Bhvadi);
    assert_has_tip(&["aBi"], &sthaa, Lrt, &["aBizWAsyati"]);
    assert_has_tip(&["pari"], &sthaa, Lrt, &["parizWAsyati"]);
    assert_has_tip(&["aBi"], &sthaa, Lun, &["aByazWAt"]);
    assert_has_tip(&["pari"], &sthaa, Lun, &["paryazWAt"]);
    assert_has_tip(&["aBi"], &sthaa, Lit, &["aBitazWO"]);
    assert_has_tip(&["pari"], &sthaa, Lit, &["paritazWO"]);

    // TODO: senaya

    let sidh = d("ziDU~", Bhvadi);
    assert_has_tip(&["aBi"], &sidh, Lat, &["aBizeDati"]);
    assert_has_tip(&["pari"], &sidh, Lat, &["parizeDati"]);
    assert_has_tip(&["aBi"], &sidh, Lan, &["aByazeDat"]);
    assert_has_tip(&["pari"], &sidh, Lan, &["paryazeDat"]);

    let sic = d("zi\\ca~^", Tudadi);
    assert_has_tip(&["aBi"], &sic, Lat, &["aBiziYcati"]);
    assert_has_tip(&["pari"], &sic, Lat, &["pariziYcati"]);
    assert_has_tip(&["aBi"], &sic, Lan, &["aByaziYcat"]);
    assert_has_tip(&["pari"], &sic, Lan, &["paryaziYcat"]);
    assert_has_tip(&["aBi"], &san(&sic), Lat, &["aBizizikzati"]);
    assert_has_tip(&["pari"], &san(&sic), Lat, &["parizizikzati"]);

    let sanj = d("za\\nja~", Bhvadi);
    assert_has_tip(&["aBi"], &sanj, Lat, &["aBizajati"]);
    assert_has_tip(&["pari"], &sanj, Lat, &["parizajati"]);
    assert_has_tip(&["aBi"], &sanj, Lan, &["aByazajat"]);
    assert_has_tip(&["pari"], &sanj, Lan, &["paryazajat"]);
    assert_has_tip(&["aBi"], &san(&sanj), Lat, &["aBizizaNkzati"]);
    assert_has_tip(&["pari"], &san(&sanj), Lat, &["parizizaNkzati"]);

    let svanj = d("zva\\nja~\\", Bhvadi);
    assert_has_ta(&["aBi"], &svanj, Lat, &["aBizvajate"]);
    assert_has_ta(&["pari"], &svanj, Lat, &["parizvajate"]);
    assert_has_ta(&["aBi"], &svanj, Lan, &["aByazvajata"]);
    // non-shatva forms are by 8.3.71.
    assert_has_ta(&["pari"], &svanj, Lan, &["paryazvajata", "paryasvajata"]);
    assert_has_ta(&["aBi"], &san(&svanj), Lat, &["aBizizvaNkzate"]);
    assert_has_ta(&["pari"], &san(&svanj), Lat, &["parizizvaNkzate"]);

    // upasargAt?
    assert_has_sandhi("daDi", "siYcati", &["daDi siYcati"]);
    assert_has_sandhi("maDu", "siYcati", &["maDu siYcati"]);
}

#[test]
fn sutra_8_3_66() {
    let sad = d("za\\dx~", Bhvadi);

    assert_has_tip(&["ni"], &sad, Lat, &["nizIdati"]);
    assert_has_tip(&["vi"], &sad, Lat, &["vizIdati"]);

    assert_has_tip(&["ni"], &sad, Lan, &["nyazIdat"]);
    assert_has_tip(&["vi"], &sad, Lan, &["vyazIdat"]);

    assert_has_tip(&["ni"], &sad, Lit, &["nizasAda"]);
    assert_has_tip(&["vi"], &sad, Lit, &["vizasAda"]);

    assert_has_tip(&["prati"], &sad, Lat, &["pratisIdati"]);
}

#[test]
fn sutra_8_3_67() {
    let stanbh = d("stanBu~", Kryadi);

    assert_has_tip(&["aBi"], &stanbh, Lat, &["aBizwaBnAti", "aBizwaBnoti"]);
    assert_has_tip(&["pari"], &stanbh, Lat, &["parizwaBnAti", "parizwaBnoti"]);

    assert_has_tip(&["aBi"], &stanbh, Lan, &["aByazwaBnAt", "aByazwaBnot"]);
    assert_has_tip(&["pari"], &stanbh, Lan, &["paryazwaBnAt", "paryazwaBnot"]);

    assert_has_tip(&["aBi"], &stanbh, Lit, &["aBitazwamBa"]);
    assert_has_tip(&["pari"], &stanbh, Lit, &["paritazwamBa"]);

    assert_has_tip(
        &["prati"],
        &stanbh,
        Lat,
        &["pratizwaBnAti", "pratizwaBnoti"],
    );
    assert_has_tip(
        &["prati"],
        &stanbh,
        Lan,
        &["pratyazwaBnAt", "pratyazwaBnot"],
    );
    assert_has_tip(&["prati"], &stanbh, Lit, &["pratitazwamBa"]);
}

#[test]
fn sutra_8_3_68() {
    let stanbh = d("stanBu~", Kryadi);
    assert_has_krdanta(&["ava"], &stanbh, Krt::ktvA, &["avazwaBya", "avastaBya"]);
}

#[test]
fn sutra_8_3_69() {
    let svan = d("svana~", Bhvadi);

    assert_has_tip(&["vi"], &svan, Lat, &["vizvaRati", "visvanati"]);
    assert_has_tip(&["vi"], &svan, Lan, &["vyazvaRat", "vyasvanat"]);
    assert_has_tip(&["vi"], &svan, Lit, &["vizazvARa", "visasvAna"]);

    assert_has_tip(&["ava"], &svan, Lat, &["avazvaRati", "avasvanati"]);
    assert_has_tip(&["ava"], &svan, Lan, &["avAzvaRat", "avAsvanat"]);
    assert_has_tip(&["ava"], &svan, Lit, &["avazazvARa", "avasasvAna"]);
}

#[test]
fn sutra_8_3_70() {
    let sev = d("zevf~\\", Bhvadi);
    assert_has_lat(&["pari"], &sev, &["parizevate"]);
    assert_has_lat(&["ni"], &sev, &["nizevate"]);
    assert_has_lat(&["vi"], &sev, &["vizevate"]);
    assert_has_lan(&["pari"], &sev, &["paryazevata"]);
    assert_has_lan(&["ni"], &sev, &["nyazevata"]);
    assert_has_lan(&["vi"], &sev, &["vyazevata"]);
    assert_has_lat(&["pari"], &san(&sev), &["parizizevizate"]);
    assert_has_lat(&["ni"], &san(&sev), &["nizizevizate"]);
    assert_has_lat(&["vi"], &san(&sev), &["vizizevizate"]);

    // TODO: sita, saya

    let siv = d("zivu~", Divadi);
    assert_has_lat(&["pari"], &siv, &["parizIvyati"]);
    assert_has_lat(&["ni"], &siv, &["nizIvyati"]);
    assert_has_lat(&["vi"], &siv, &["vizIvyati"]);
    assert_has_lan(&["pari"], &siv, &["paryazIvyat", "paryasIvyat"]);
    assert_has_lan(&["ni"], &siv, &["nyazIvyat", "nyasIvyat"]);
    assert_has_lan(&["vi"], &siv, &["vyazIvyat", "vyasIvyat"]);

    let sah = d("zaha~\\", Bhvadi);
    assert_has_lat(&["pari"], &sah, &["parizahate"]);
    assert_has_lat(&["ni"], &sah, &["nizahate"]);
    assert_has_lat(&["vi"], &sah, &["vizahate"]);
    assert_has_lan(&["pari"], &sah, &["paryazahata", "paryasahata"]);
    assert_has_lan(&["ni"], &sah, &["nyazahata", "nyasahata"]);
    assert_has_lan(&["vi"], &sah, &["vyazahata", "vyasahata"]);

    /*
        let kr = d("qukf\\Y", Tanadi);
        assert_has_tip(&["pari"], &kr, Lat, &["parizkaroti", "parikaroti"]);
        assert_has_lan_p(
            &["pari"],
            &kr,
            &["paryazkarot", "paryaskarot", "paryakarot"],
        );
    */

    let stu = d("zwu\\Y", Adadi);
    assert_has_tip(&["pari"], &stu, Lat, &["parizwOti", "parizwavIti"]);
    assert_has_tip(&["ni"], &stu, Lat, &["nizwOti", "nizwavIti"]);
    assert_has_tip(&["vi"], &stu, Lat, &["vizwOti", "vizwavIti"]);
    assert_has_tip(
        &["pari"],
        &stu,
        Lan,
        &["paryazwOt", "paryastOt", "paryazwavIt", "paryastavIt"],
    );
    assert_has_tip(
        &["ni"],
        &stu,
        Lan,
        &["nyazwOt", "nyastOt", "nyazwavIt", "nyastavIt"],
    );
    assert_has_tip(
        &["vi"],
        &stu,
        Lan,
        &["vyazwOt", "vyastOt", "vyazwavIt", "vyastavIt"],
    );

    let svanj = d("zva\\nja~\\", Bhvadi);
    assert_has_lat(&["pari"], &svanj, &["parizvajate"]);
    assert_has_lat(&["ni"], &svanj, &["nizvajate"]);
    assert_has_lat(&["vi"], &svanj, &["vizvajate"]);
    assert_has_lan(&["pari"], &svanj, &["paryazvajata", "paryasvajata"]);
}

#[test]
fn sutra_8_3_71() {}

#[test]
fn sutra_8_3_72() {
    let syand = d("syandU~\\", Bhvadi);
    assert_has_lat(&["anu"], &syand, &["anuzyandate", "anusyandate"]);
    assert_has_lat(&["vi"], &syand, &["vizyandate", "visyandate"]);
    assert_has_lat(&["pari"], &syand, &["parizyandate", "parisyandate"]);
    assert_has_lat(&["aBi"], &syand, &["aBizyandate", "aBisyandate"]);
    assert_has_lat(&["ni"], &syand, &["nizyandate", "nisyandate"]);
}

#[test]
fn sutra_8_3_73() {
    let skand = d("ska\\ndi~r", Bhvadi);
    assert_has_krdanta(
        &["vi"],
        &skand,
        Krt::tfc,
        &["vizkantf", "vizkanttf", "viskantf", "viskanttf"],
    );
    assert_has_krdanta(
        &["vi"],
        &skand,
        Krt::tumun,
        &["vizkantum", "vizkanttum", "viskantum", "viskanttum"],
    );
    assert_has_krdanta(
        &["vi"],
        &skand,
        Krt::tavya,
        &["vizkantavya", "vizkanttavya", "viskantavya", "viskanttavya"],
    );
    assert_has_krdanta(&["vi"], &skand, Krt::kta, &["viskanna"]);
}

#[test]
fn sutra_8_3_74() {
    let skand = d("ska\\ndi~r", Bhvadi);
    assert_has_krdanta(
        &["pari"],
        &skand,
        Krt::tfc,
        &["parizkantf", "parizkanttf", "pariskantf", "pariskanttf"],
    );
    assert_has_krdanta(
        &["pari"],
        &skand,
        Krt::tumun,
        &["parizkantum", "parizkanttum", "pariskantum", "pariskanttum"],
    );
    assert_has_krdanta(
        &["pari"],
        &skand,
        Krt::tavya,
        &[
            "parizkantavya",
            "parizkanttavya",
            "pariskantavya",
            "pariskanttavya",
        ],
    );
    assert_has_krdanta(&["pari"], &skand, Krt::kta, &["pariskanna", "parizkaRRa"]);
}

#[test]
fn sutra_8_3_76() {
    let sphur = d("sPura~", Tudadi);
    assert_has_lat(
        &["nis"],
        &sphur,
        &["nizzPurati", "nissPurati", "niHzPurati", "niHsPurati"],
    );
    assert_has_lat(&["ni"], &sphur, &["nizPurati", "nisPurati"]);
    assert_has_lat(&["vi"], &sphur, &["vizPurati", "visPurati"]);

    let sphul = d("sPula~", Tudadi);
    assert_has_lat(
        &["nis"],
        &sphul,
        &["nizzPulati", "nissPulati", "niHzPulati", "niHsPulati"],
    );
    assert_has_lat(&["ni"], &sphul, &["nizPulati", "nisPulati"]);
    assert_has_lat(&["vi"], &sphul, &["vizPulati", "visPulati"]);
}

#[test]
fn sutra_8_3_77() {
    let skanbh = d("skanBu~", Kryadi);
    assert_has_lat(&["vi"], &skanbh, &["vizkaBnAti", "vizkaBnoti"]);
}

#[test]
fn sutra_8_3_78() {
    let cyu = d("cyu\\N", Bhvadi);
    let plu = d("plu\\N", Bhvadi);

    // zIQam
    assert_has_dhvam(&[], &cyu, AshirLin, &["cyozIQvam"]);
    assert_has_dhvam(&[], &plu, AshirLin, &["plozIQvam"]);

    // luN
    assert_has_dhvam(&[], &cyu, Lun, &["acyoQvam"]);
    assert_has_dhvam(&[], &plu, Lun, &["aploQvam"]);

    // liw
    assert_has_dhvam(&[], &d("qukf\\Y", Tanadi), Lit, &["cakfQve"]);
    assert_has_dhvam(&[], &d("vfY", Svadi), Lit, &["vavfQve"]);

    // kavarga-nivrtti
    assert_has_dhvam(&[], &d("qupa\\ca~^z", Bhvadi), AshirLin, &["pakzIDvam"]);
    assert_has_dhvam(&[], &d("ya\\ja~^", Bhvadi), AshirLin, &["yakzIDvam"]);

    // zIDvaMluNliwAm?
    assert_has_dhvam(&[], &d("zwu\\Y", Adadi), Lat, &["stuDve", "stuvIDve"]);
    assert_has_dhvam(&[], &d("zwu\\Y", Adadi), Lan, &["astuDvam", "astuvIDvam"]);

    // aNgAt?
    assert_has_dhvam(
        &["pari"],
        &d("vi\\zx~^", Juhotyadi),
        VidhiLin,
        &["parivevizIDvam"],
    );
}

#[test]
fn sutra_8_3_79() {
    let lu = d("lUY", Kryadi);
    assert_has_dhvam(&[], &lu, AshirLin, &["lavizIDvam", "lavizIQvam"]);

    let pu = d("pUY", Kryadi);
    assert_has_dhvam(&[], &pu, AshirLin, &["pavizIDvam", "pavizIQvam"]);

    assert_has_dhvam(&[], &lu, Lun, &["alaviDvam", "alaviQvam"]);
    assert_has_dhvam(&[], &lu, Lit, &["luluviDve", "luluviQve"]);

    let aas = d("Asa~\\", Adadi);
    assert_has_dhvam(&[], &aas, AshirLin, &["AsizIDvam"]);

    // Other cases
    let kf = d("qukf\\Y", Tanadi);
    assert_has_dhvam(&[], &kf, Lit, &["cakfQve"]);
}

#[ignore]
#[test]
fn sutra_8_3_82() {
    assert_has_sasthi_tatpurusha("agni", "stut", &["agnizwut"]);
    assert_has_sasthi_tatpurusha("agni", "stoma", &["agnizwoma"]);
    assert_has_sasthi_tatpurusha("agni", "soma", &["agnizoma"]);
}

#[test]
fn sutra_8_3_96() {
    assert_has_avyaya_tatpurusha("vi", "sTala", &["vizWala"]);
    assert_has_avyaya_tatpurusha("ku", "sTala", &["kuzWala"]);
    assert_has_sasthi_tatpurusha("Sami", "sTala", &["SamizWala"]);
    assert_has_avyaya_tatpurusha("pari", "sTala", &["parizWala"]);
}

#[ignore]
#[test]
fn sutra_8_3_101() {
    let sarpis = create_krdanta("sarpis", &[], &d("sf\\px~", Bhvadi), Unadi::isi);
    let yajus = create_krdanta("yajus", &[], &d("ya\\ja~^", Bhvadi), Unadi::usi);

    assert_has_taddhita(&sarpis, T::tarap, &["sarpizwara"]);
    assert_has_taddhita(&yajus, T::tarap, &["yajuzwara"]);

    assert_has_taddhita(&sarpis, T::tamap, &["sarpizwama"]);
    assert_has_taddhita(&yajus, T::tamap, &["yajuzwama"]);

    assert_has_artha_taddhita("catur", Avasana, T::tayap, &["catuzwaya"]);

    assert_has_taddhita(&sarpis, T::tva, &["sarpizwva"]);
    assert_has_taddhita(&yajus, T::tva, &["yajuzwva"]);

    assert_has_taddhita(&sarpis, T::tal, &["sarpizwA"]);
    assert_has_taddhita(&yajus, T::tal, &["yajuzwA"]);

    assert_has_taddhita(&sarpis, T::tasi, &["sarpizwas"]);
    assert_has_taddhita(&yajus, T::tasi, &["yajuzwas"]);

    // let _avis = SamasaPada::avyaya(Pratipadika::basic("Avis").into());
    // assert_has_taddhita(&avis, T::tyap, &["Avizwya"]);

    // hrasva?
    assert_has_taddhita("gir", T::tarap, &["gIstara"]);
    assert_has_taddhita("Dur", T::tarap, &["DUstara"]);

    // tAdau?
    assert_has_taddhita(&sarpis, T::sAti, &["sarpissAt"]);
}

#[test]
fn sutra_8_3_110() {
    assert_has_krdanta(&["vi"], &d("sransu~\\", Bhvadi), Krt::Rvul, &["visraMsaka"]);
    assert_has_krdanta(&["vi"], &d("sranBu~\\", Bhvadi), Krt::kta, &["visrabDa"]);
    assert_has_krdanta(&["vi"], &d("sf\\px~", Tudadi), Krt::kasun, &["visfpas"]);
    assert_has_krdanta(&["vi"], &d("sf\\ja~", Tudadi), Krt::lyuw, &["visarjana"]);
    assert_has_krdanta(&[], &d("spf\\Sa~", Tudadi), Krt::kamul, &["spfSam"]);
    assert_has_krdanta(&["ni"], &d("spfha", Curadi), Krt::kamul, &["nispfham"]);
    // TODO: others?
}

#[test]
fn sutra_8_3_111() {
    assert_has_taddhita("agni", T::sAti, &["agnisAt"]);
    assert_has_taddhita("daDi", T::sAti, &["daDisAt"]);
    assert_has_sandhi("daDi", "siYcati", &["daDi siYcati"]);
    assert_has_sandhi("maDu", "siYcati", &["maDu siYcati"]);
}

#[test]
fn sutra_8_3_112() {
    let sic = d("zi\\ca~^", Tudadi);
    assert_has_lat(&[], &yan(&sic), &["sesicyate"]);
    assert_has_lat(&["aBi"], &yan(&sic), &["aBisesicyate"]);
    // yani
    assert_has_tip(&["aBi"], &san(&sic), Lat, &["aBizizikzati"]);
}

#[test]
fn sutra_8_3_113() {
    let sidh = d("ziDa~", Bhvadi);
    assert_has_tip(&["aBi"], &nic(&sidh), Lat, &["aBiseDayati"]);
    assert_has_tip(&["pari"], &nic(&sidh), Lat, &["pariseDayati"]);
    // gatau?
    assert_has_tip(&["prati"], &nic(&sidh), Lat, &["pratizeDayati"]);
}

#[test]
fn sutra_8_3_115() {
    let sah = d("zaha~\\", Bhvadi);
    assert_has_krdanta(&["pari"], &sah, Krt::kta, &["parisoQa", "parizahita"]);
    assert_has_krdanta(&["pari"], &sah, Krt::tumun, &["parisoQum", "parizahitum"]);
    assert_has_krdanta(
        &["pari"],
        &sah,
        Krt::tavya,
        &["parisoQavya", "parizahitavya"],
    );

    // soQa?
    assert_has_lat(&["pari"], &sah, &["parizahate"]);
}

#[test]
fn sutra_8_3_116() {
    let stanbh = nic(&d("stanBu~", Kryadi));
    assert_has_tip(&["pari"], &stanbh, Lun, &["paryatastamBat"]);
    assert_has_tip(&["aBi"], &stanbh, Lun, &["aByatastamBat"]);

    let siv = nic(&d("zivu~", Divadi));
    assert_has_tip(&["pari"], &siv, Lun, &["paryasIzivat"]);
    assert_has_tip(&["ni"], &siv, Lun, &["nyasIzivat"]);

    let sah = nic(&d("zaha~\\", Bhvadi));
    assert_has_tip(&["pari"], &sah, Lun, &["paryasIzahat"]);
    assert_has_tip(&["vi"], &sah, Lun, &["vyasIzahat"]);
}

#[test]
fn sutra_8_3_117() {
    let sush = d("zu\\Y", Svadi);
    assert_has_tip(&["aBi"], &sush, Lrt, &["aBisozyati"]);
    assert_has_tip(&["pari"], &sush, Lrt, &["parisozyati"]);
    assert_has_tip(&["aBi"], &sush, Lrn, &["aByasozyat"]);
    assert_has_tip(&["pari"], &sush, Lrn, &["paryasozyat"]);

    // sani?
    assert_has_lat(&["aBi"], &san(&sush), &["aBisusUzati", "aBisusUzate"]);

    // syasanoH?
    assert_has_tip(&[], &sush, Lit, &["suzAva"]);

    // TODO: aBisusUH
}

#[test]
fn sutra_8_3_118() {
    let sad = d("za\\dx~", Bhvadi);
    assert_has_tip(&["ni"], &sad, Lit, &["nizasAda"]);
}

#[test]
fn sutra_8_3_118_v1() {
    let svanj = d("zva\\nja~\\", Bhvadi);
    assert_has_lit(&["pari"], &svanj, &["parizasvaje", "parizasvaYje"]);
}
