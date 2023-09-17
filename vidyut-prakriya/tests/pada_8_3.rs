extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti as V;
use vidyut_prakriya::args::*;

fn san(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::San])
}

fn yan(dhatu: &Dhatu) -> Dhatu {
    dhatu.clone().with_sanadi(&[Sanadi::Yan])
}

#[test]
fn sutra_8_3_13() {
    assert_has_krdanta(&[], &d("li\\ha~^", Adadi), Krt::kta, &["lIQa"]);
    assert_has_krdanta(&["upa"], &d("guhU~^", Bhvadi), Krt::kta, &["upagUQa"]);
}

#[test]
fn sutra_8_3_14() {
    let raj = Dhatu::new("ra\\nja~^", Gana::Bhvadi);
    assert_has_krdanta(&["nis"], &raj, Krt::kta, &["nIrakta"]);
    assert_has_krdanta(&["nir"], &raj, Krt::kta, &["nIrakta"]);
    assert_has_krdanta(&["dus"], &raj, Krt::kta, &["dUrakta"]);
    assert_has_krdanta(&["dur"], &raj, Krt::kta, &["dUrakta"]);
}

#[test]
fn sutra_8_3_15() {
    // TODO: Kar
    assert_has_subantas("vfkza", Pum, V::Prathama, Eka, &["vfkzaH"]);
    assert_has_subantas("plakza", Pum, V::Prathama, Eka, &["plakzaH"]);
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
fn sutra_8_3_23() {
    assert_has_lat_karmani(&[], &d("ga\\mx~", Bhvadi), &["gamyate"]);
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
fn sutra_8_3_63() {
    let su = Dhatu::new("zu\\Y", Gana::Svadi);

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
    let stha = Dhatu::new("zWA\\", Gana::Bhvadi);

    assert_has_lit_p(&["aBi"], &stha, &["aBitazWO"]);
    assert_has_lit_p(&["pari"], &stha, &["paritazWO"]);
}

#[test]
fn sutra_8_3_66() {
    let sad = Dhatu::new("za\\dx~", Gana::Bhvadi);

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
    let stanbh = Dhatu::new("sta\\nBu~", Gana::Kryadi);

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
    let ava_stanbh = Dhatu::new("sta\\nBu~", Gana::Kryadi).with_prefixes(&["ava"]);
    assert_padas(
        derive_krdantas(&ava_stanbh, Krt::ktvA),
        &["avazwaBya", "avastaBya"],
    );
}

#[test]
fn sutra_8_3_69() {
    let svan = Dhatu::new("svana~", Gana::Bhvadi);

    assert_has_lat_p(&["vi"], &svan, &["vizvaRati", "visvanati"]);
    assert_has_lan_p(&["vi"], &svan, &["vyazvaRat", "vyasvanat"]);
    assert_has_lit_p(&["vi"], &svan, &["vizazvARa", "visasvAna"]);

    assert_has_lat_p(&["ava"], &svan, &["avazvaRati", "avasvanati"]);
    assert_has_lan_p(&["ava"], &svan, &["avAzvaRat", "avAsvanat"]);
    assert_has_lit_p(&["ava"], &svan, &["avazazvARa", "avasasvAna"]);
}

#[test]
fn sutra_8_3_70() {
    let upa_san_lat = |prefixes, d: &Dhatu| {
        derive_lakara(
            prefixes,
            &d.clone().with_sanadi(&[Sanadi::San]),
            Lakara::Lat,
        )
    };

    let sev = Dhatu::new("zevf~\\", Gana::Bhvadi);
    assert_has_lat(&["pari"], &sev, &["parizevate"]);
    assert_has_lat(&["ni"], &sev, &["nizevate"]);
    assert_has_lat(&["vi"], &sev, &["vizevate"]);
    assert_has_lan(&["pari"], &sev, &["paryazevata"]);
    assert_has_lan(&["ni"], &sev, &["nyazevata"]);
    assert_has_lan(&["vi"], &sev, &["vyazevata"]);
    assert_padas(upa_san_lat(&["pari"], &sev), &["parizizevizate"]);
    assert_padas(upa_san_lat(&["ni"], &sev), &["nizizevizate"]);
    assert_padas(upa_san_lat(&["vi"], &sev), &["vizizevizate"]);

    // TODO: sita, saya

    let siv = Dhatu::new("zivu~", Gana::Divadi);
    assert_has_lat(&["pari"], &siv, &["parizIvyati"]);
    assert_has_lat(&["ni"], &siv, &["nizIvyati"]);
    assert_has_lat(&["vi"], &siv, &["vizIvyati"]);
    assert_has_lan(&["pari"], &siv, &["paryazIvyat", "paryasIvyat"]);
    assert_has_lan(&["ni"], &siv, &["nyazIvyat", "nyasIvyat"]);
    assert_has_lan(&["vi"], &siv, &["vyazIvyat", "vyasIvyat"]);

    let sah = Dhatu::new("zaha~\\", Gana::Bhvadi);
    assert_has_lat(&["pari"], &sah, &["parizahate"]);
    assert_has_lat(&["ni"], &sah, &["nizahate"]);
    assert_has_lat(&["vi"], &sah, &["vizahate"]);
    assert_has_lan(&["pari"], &sah, &["paryazahata", "paryasahata"]);
    assert_has_lan(&["ni"], &sah, &["nyazahata", "nyasahata"]);
    assert_has_lan(&["vi"], &sah, &["vyazahata", "vyasahata"]);

    /*
        let kr = Dhatu::new("qukf\\Y", Gana::Tanadi);
        assert_has_lat_p(&["pari"], &kr, &["parizkaroti", "parikaroti"]);
        assert_has_lan_p(
            &["pari"],
            &kr,
            &["paryazkarot", "paryaskarot", "paryakarot"],
        );
    */

    let stu = Dhatu::new("zwu\\Y", Gana::Adadi);
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

    let svanj = Dhatu::new("zva\\nja~\\", Gana::Bhvadi);
    assert_has_lat(&["pari"], &svanj, &["parizvajate"]);
    assert_has_lat(&["ni"], &svanj, &["nizvajate"]);
    assert_has_lat(&["vi"], &svanj, &["vizvajate"]);
    assert_has_lan(&["pari"], &svanj, &["paryazvajata", "paryasvajata"]);
}

#[test]
fn sutra_8_3_72() {
    let syand = Dhatu::new("syandU~\\", Gana::Bhvadi);
    assert_has_lat(&["anu"], &syand, &["anuzyandate", "anusyandate"]);
    assert_has_lat(&["vi"], &syand, &["vizyandate", "visyandate"]);
    assert_has_lat(&["pari"], &syand, &["parizyandate", "parisyandate"]);
    assert_has_lat(&["aBi"], &syand, &["aBizyandate", "aBisyandate"]);
    assert_has_lat(&["ni"], &syand, &["nizyandate", "nisyandate"]);
}

#[test]
fn sutra_8_3_73() {
    let skand = Dhatu::new("ska\\ndi~r", Gana::Bhvadi);
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
    let pari_skand = Dhatu::new("ska\\ndi~r", Gana::Bhvadi).with_prefixes(&["pari"]);
    assert_padas(
        derive_krdantas(&pari_skand, Krt::tfc),
        &["parizkantf", "parizkanttf", "pariskantf", "pariskanttf"],
    );
    assert_padas(
        derive_krdantas(&pari_skand, Krt::tumun),
        &["parizkantum", "parizkanttum", "pariskantum", "pariskanttum"],
    );
    assert_padas(
        derive_krdantas(&pari_skand, Krt::tavya),
        &[
            "parizkantavya",
            "parizkanttavya",
            "pariskantavya",
            "pariskanttavya",
        ],
    );
    assert_padas(
        derive_krdantas(&pari_skand, Krt::kta),
        &["pariskanna", "parizkaRRa"],
    );
}

#[test]
fn sutra_8_3_76() {
    let sphur = Dhatu::new("sPura~", Gana::Tudadi);
    assert_has_lat(
        &["nis"],
        &sphur,
        &["nizzPurati", "nissPurati", "niHzPurati", "niHsPurati"],
    );
    assert_has_lat(&["ni"], &sphur, &["nizPurati", "nisPurati"]);
    assert_has_lat(&["vi"], &sphur, &["vizPurati", "visPurati"]);

    let sphul = Dhatu::new("sPula~", Gana::Tudadi);
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
    let skanbh = Dhatu::new("ska\\nBu~", Gana::Kryadi);
    assert_has_lat(&["vi"], &skanbh, &["vizkaBnAti", "vizkaBnoti"]);
}

#[test]
fn sutra_8_3_79() {
    let shidhvam = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::AshirLin)
        .pada(Pada::Atmane)
        .build()
        .unwrap();

    let lun = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::Lun)
        .pada(Pada::Atmane)
        .build()
        .unwrap();

    let lit = TinantaArgs::builder()
        .prayoga(Prayoga::Kartari)
        .purusha(Purusha::Madhyama)
        .vacana(Vacana::Bahu)
        .lakara(Lakara::Lit)
        .pada(Pada::Atmane)
        .build()
        .unwrap();

    // Examples from Kashika Vrtti
    let lu = Dhatu::new("lUY", Gana::Kryadi);
    let actual = derive_tinantas(&lu, &shidhvam);
    assert_padas(actual, &["lavizIDvam", "lavizIQvam"]);

    let pu = Dhatu::new("pUY", Gana::Kryadi);
    let actual = derive_tinantas(&pu, &shidhvam);
    assert_padas(actual, &["pavizIDvam", "pavizIQvam"]);

    let actual = derive_tinantas(&lu, &lun);
    assert_padas(actual, &["alaviDvam", "alaviQvam"]);

    let actual = derive_tinantas(&lu, &lit);
    assert_padas(actual, &["luluviDve", "luluviQve"]);

    let aas = Dhatu::new("Asa~\\", Gana::Adadi);
    let actual = derive_tinantas(&aas, &shidhvam);
    assert_padas(actual, &["AsizIDvam"]);

    // Other cases
    let kf = Dhatu::new("qukf\\Y", Gana::Tanadi);
    let actual = derive_tinantas(&kf, &lit);
    assert_padas(actual, &["cakfQve"]);
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
