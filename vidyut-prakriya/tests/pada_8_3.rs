extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti as V;
use vidyut_prakriya::args::*;

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
    assert_has_subantas("vfkza", Pum, V::Prathama, Eka, &["vfkzaH"]);
    assert_has_subantas("plakza", Pum, V::Prathama, Eka, &["plakzaH"]);

    // khar-avasAnayoH?
    assert_has_sandhi("agnis", "nayati", &["agnir nayati"]);
    assert_has_sandhi("vAyus", "nayati", &["vAyur nayati"]);
}

#[ignore]
#[test]
fn sutra_8_3_16() {
    assert_has_subantas(
        "payas",
        Napumsaka,
        V::Saptami,
        Bahu,
        &["payaHsu", "payassu"],
    );
    // TODO: sarpizzu?
    // assert_has_subantas("sarpis", Napumsaka, V::Saptami, Bahu, &["sarpiHzu"]);
    assert_has_subantas(
        "yaSas",
        Napumsaka,
        V::Saptami,
        Bahu,
        &["yaSaHsu", "yaSassu"],
    );
    // ruH
    assert_has_subantas("gir", Napumsaka, V::Saptami, Bahu, &["gIrzu"]);
    assert_has_subantas("Dur", Napumsaka, V::Saptami, Bahu, &["DUrzu"]);
}

#[test]
fn sutra_8_3_22() {
    assert_has_sandhi("Bo", "hasati", &["Bo hasati"]);
    assert_has_sandhi("Bago", "hasati", &["Bago hasati"]);
    assert_has_sandhi("aDo", "hasati", &["aDo hasati"]);
    assert_has_sandhi("Bo", "yAti", &["Bo yAti"]);
    assert_has_sandhi("Bago", "yAti", &["Bago yAti"]);
    assert_has_sandhi("aDo", "yAti", &["aDo yAti"]);
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
    assert_has_lat_karmani(&[], &d("ga\\mx~", Bhvadi), &["gamyate"]);
    assert_has_lat_karmani(&[], &d("ra\\ma~\\", Bhvadi), &["ramyate"]);
}

#[ignore]
#[test]
fn sutra_8_3_24() {
    assert_has_subantas("payas", Napumsaka, V::Prathama, Bahu, &["payAMsi"]);
    assert_has_subantas("yaSas", Napumsaka, V::Prathama, Bahu, &["yaSAMsi"]);

    assert_has_subantas("sarpis", Napumsaka, V::Prathama, Bahu, &["sarpIMzi"]);

    // makArasya
    let kram = d("kramu~", Bhvadi);
    assert_has_lrt_a(&["AN"], &kram, &["AkraMsyate"]);
    assert_has_lat_karmani(&["AN"], &san(&kram), &["AcikraMsyate"]);
    assert_has_lat(&["aDi"], &san(&d("i\\N", Adadi)), &["aDijigaMsate"]);

    // Jali
    assert_has_lat_karmani(&[], &d("ra\\ma~\\", Bhvadi), &["ramyate"]);
    assert_has_lat_karmani(&[], &d("ga\\mx~", Bhvadi), &["gamyate"]);
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
fn sutra_8_3_55() {
    assert_has_lit_p(&[], &d("zi\\ca~^", Tudadi), &["sizeca"]);
    assert_has_lit_p(&[], &d("Yizva\\pa~", Adadi), &["suzvApa"]);
    assert_has_subantas("agni", Pum, V::Saptami, Bahu, &["agnizu"]);
    assert_has_subantas("vAyu", Pum, V::Saptami, Bahu, &["vAyuzu"]);

    let kf = d("qukf\\Y", Tanadi);
    assert_has_dhvam(&[], &kf, Lun, &["akfQvam"]);
    assert_has_dhvam(&[], &kf, Lit, &["cakfQve"]);

    // TODO: apadAntasya
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
fn sutra_8_3_61() {
    let svap = d("Yizva\\pa~", Adadi);
    let sic = d("zi\\ca~^", Tudadi);
    assert_has_lat_p(&[], &san(&d("zwu\\Y", Adadi)), &["tuzwUzati"]);
    assert_has_lat_p(&[], &nic_san(&d("zevf~\\", Bhvadi)), &["sizevayizati"]);
    assert_has_lat_p(&[], &nic_san(&d("za\\nja~", Bhvadi)), &["sizaYjayizati"]);
    assert_has_lat_p(&[], &nic_san(&svap), &["suzvApayizati"]);
    assert_has_lat_p(&[], &san(&sic), &["sisikzati"]);
    assert_has_lat_p(&[], &san(&d("zu\\", Bhvadi)), &["susUzati"]);
    // zaRi
    assert_has_lit_p(&[], &sic, &["sizeca"]);
    assert_has_lat(&[], &san(&svap), &["suzupsati"]);
    assert_has_lat_p(&[], &san(&d("zWA\\", Bhvadi)), &["tizWAsati"]);
    // TODO: others
}

#[test]
fn sutra_8_3_63() {
    let su = d("zu\\Y", Svadi);
    assert_has_lat_p(&["aBi"], &su, &["aBizuRoti"]);
    assert_has_lat_p(&["pari"], &su, &["parizuRoti"]);
    assert_has_lat_p(&["vi"], &su, &["vizuRoti"]);
    assert_has_lat_p(&["ni"], &su, &["nizuRoti"]);
    assert_has_lan_p(&["aBi"], &su, &["aByazuRot"]);
    assert_has_lan_p(&["pari"], &su, &["paryazuRot"]);
    assert_has_lan_p(&["vi"], &su, &["vyazuRot"]);
    assert_has_lan_p(&["ni"], &su, &["nyazuRot"]);
}

#[test]
fn sutra_8_3_64() {
    // TODO: others
    let stha = d("zWA\\", Bhvadi);

    assert_has_lit_p(&["aBi"], &stha, &["aBitazWO"]);
    assert_has_lit_p(&["pari"], &stha, &["paritazWO"]);
}

#[test]
fn sutra_8_3_66() {
    let sad = d("za\\dx~", Bhvadi);

    assert_has_lat_p(&["ni"], &sad, &["nizIdati"]);
    assert_has_lat_p(&["vi"], &sad, &["vizIdati"]);

    assert_has_lan_p(&["ni"], &sad, &["nyazIdat"]);
    assert_has_lan_p(&["vi"], &sad, &["vyazIdat"]);

    assert_has_lit_p(&["ni"], &sad, &["nizasAda"]);
    assert_has_lit_p(&["vi"], &sad, &["vizasAda"]);

    assert_has_lat_p(&["prati"], &sad, &["pratisIdati"]);
}

#[test]
fn sutra_8_3_67() {
    let stanbh = d("sta\\nBu~", Kryadi);

    assert_has_lat_p(&["aBi"], &stanbh, &["aBizwaBnAti", "aBizwaBnoti"]);
    assert_has_lat_p(&["pari"], &stanbh, &["parizwaBnAti", "parizwaBnoti"]);

    assert_has_lan_p(&["aBi"], &stanbh, &["aByazwaBnAt", "aByazwaBnot"]);
    assert_has_lan_p(&["pari"], &stanbh, &["paryazwaBnAt", "paryazwaBnot"]);

    assert_has_lit_p(&["aBi"], &stanbh, &["aBitazwamBa"]);
    assert_has_lit_p(&["pari"], &stanbh, &["paritazwamBa"]);

    assert_has_lat_p(&["prati"], &stanbh, &["pratizwaBnAti", "pratizwaBnoti"]);
    assert_has_lan_p(&["prati"], &stanbh, &["pratyazwaBnAt", "pratyazwaBnot"]);
    assert_has_lit_p(&["prati"], &stanbh, &["pratitazwamBa"]);
}

#[test]
fn sutra_8_3_68() {
    let stanbh = d("sta\\nBu~", Kryadi);
    assert_has_krdanta(&["ava"], &stanbh, Krt::ktvA, &["avazwaBya", "avastaBya"]);
}

#[test]
fn sutra_8_3_69() {
    let svan = d("svana~", Bhvadi);

    assert_has_lat_p(&["vi"], &svan, &["vizvaRati", "visvanati"]);
    assert_has_lan_p(&["vi"], &svan, &["vyazvaRat", "vyasvanat"]);
    assert_has_lit_p(&["vi"], &svan, &["vizazvARa", "visasvAna"]);

    assert_has_lat_p(&["ava"], &svan, &["avazvaRati", "avasvanati"]);
    assert_has_lan_p(&["ava"], &svan, &["avAzvaRat", "avAsvanat"]);
    assert_has_lit_p(&["ava"], &svan, &["avazazvARa", "avasasvAna"]);
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
        assert_has_lat_p(&["pari"], &kr, &["parizkaroti", "parikaroti"]);
        assert_has_lan_p(
            &["pari"],
            &kr,
            &["paryazkarot", "paryaskarot", "paryakarot"],
        );
    */

    let stu = d("zwu\\Y", Adadi);
    assert_has_lat_p(&["pari"], &stu, &["parizwOti", "parizwavIti"]);
    assert_has_lat_p(&["ni"], &stu, &["nizwOti", "nizwavIti"]);
    assert_has_lat_p(&["vi"], &stu, &["vizwOti", "vizwavIti"]);
    assert_has_lan_p(
        &["pari"],
        &stu,
        &["paryazwOt", "paryastOt", "paryazwavIt", "paryastavIt"],
    );
    assert_has_lan_p(
        &["ni"],
        &stu,
        &["nyazwOt", "nyastOt", "nyazwavIt", "nyastavIt"],
    );
    assert_has_lan_p(
        &["vi"],
        &stu,
        &["vyazwOt", "vyastOt", "vyazwavIt", "vyastavIt"],
    );

    let svanj = d("zva\\nja~\\", Bhvadi);
    assert_has_lat(&["pari"], &svanj, &["parizvajate"]);
    assert_has_lat(&["ni"], &svanj, &["nizvajate"]);
    assert_has_lat(&["vi"], &svanj, &["vizvajate"]);
    assert_has_lan(&["pari"], &svanj, &["paryazvajata", "paryasvajata"]);
}

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
    let skanbh = d("ska\\nBu~", Kryadi);
    assert_has_lat(&["vi"], &skanbh, &["vizkaBnAti", "vizkaBnoti"]);
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

#[ignore]
#[test]
fn sutra_8_3_112() {
    let sic = d("zi\\ca~^", Tudadi);
    assert_has_lat(&[], &yan(&sic), &["sesicyate"]);
    assert_has_lat(&["aBi"], &yan(&sic), &["aBisesicyate"]);
    // yani
    assert_has_lat(&["aBi"], &san(&sic), &["aBizizikzati"]);
}

#[ignore]
#[test]
fn sutra_8_3_115() {
    let sah = d("zaha~\\", Bhvadi);
    assert_has_krdanta(&["pari"], &sah, Krt::kta, &["parisoQa", "parizahita"]);
    assert_has_krdanta(&["pari"], &sah, Krt::tumun, &["parisoQum"]);
    assert_has_krdanta(&["pari"], &sah, Krt::tavya, &["parisoQavya"]);

    // soQa?
    assert_has_lat(&["pari"], &sah, &["parizahate"]);
}
