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

#[test]
fn sutra_6_1_1() {
    assert_has_lit_p(
        &[],
        &d("jAgf", Adadi),
        &["jajAgAra", "jAgarAYcakAra", "jAgarAmbaBUva", "jAgarAmAsa"],
    );
    assert_has_lit_p(&[], &d("qupa\\ca~^z", Bhvadi), &["papAca"]);
    assert_has_lit_p(&[], &d("i\\R", Adadi), &["iyAya"]);
    assert_has_lit_p(&[], &d("f\\", Juhotyadi), &["Ara"]);
}

#[test]
fn sutra_6_1_2() {
    let san = |u, g| d(u, g).with_sanadi(&[Sanadi::San]);
    assert_has_lat_p(&[], &san("awa~", Bhvadi), &["awiwizati"]);
    assert_has_lat_p(&[], &san("aSa~", Kryadi), &["aSiSizati"]);
    assert_has_lat_p(&[], &san("f\\", Kryadi), &["aririzati"]);
}

#[test]
fn sutra_6_1_3() {
    let san = |u, g| d(u, g).with_sanadi(&[Sanadi::San]);
    assert_has_lat_p(&[], &san("undI~", Rudhadi), &["undidizati"]);
    assert_has_lat_p(&[], &san("adqa~", Bhvadi), &["aqqiqizati"]);
    assert_has_lat_p(&[], &san("arca~", Bhvadi), &["arcicizati"]);
    // ndrA
    assert_has_lat(&[], &san("Ikza~\\", Bhvadi), &["Icikzizate"]);
    // saMyoga
    assert_has_lat_p(&["pra"], &san("ana~", Adadi), &["prARiRizati"]);
    // ajAdi
}

#[test]
fn sutra_6_1_4() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    let san = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::San]);
    let yan = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::Yan]);

    assert_has_lit_p(&[], &pac, &["papAca"]);
    assert_has_lat_p(&[], &san(&pac), &["pipakzati"]);
    assert_has_lat(&[], &yan(&pac), &["pApacyate"]);
    assert_has_lat_p(&[], &d("hu\\", Juhotyadi), &["juhoti"]);
    assert_has_lun_p(&[], &nic(&pac), &["apIpacat"]);
}

#[test]
fn sutra_6_1_5() {
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_parasmai_tinanta(&[], &daa, Lat, Prathama, Bahu, &["dadati"]);
    assert_has_krdanta(&[], &daa, Krt::Satf, &["dadat"]);
    assert_has_parasmai_tinanta(
        &[],
        &d("quDA\\Y", Juhotyadi),
        Lot,
        Prathama,
        Bahu,
        &["daDatu"],
    );
    // uBe
    assert_has_parasmai_tinanta(
        &[],
        &d("Ri\\ji~^r", Juhotyadi),
        Lot,
        Uttama,
        Eka,
        &["nenijAni"],
    );
}

#[test]
fn sutra_6_1_6() {
    let assert_has_lat_3p = |d, exp| assert_has_tinanta(&[], &d, Lat, Prathama, Bahu, exp);

    assert_has_lat_3p(d("jakza~", Adadi), &["jakzati"]);
    assert_has_lat_3p(d("jAgf", Adadi), &["jAgrati"]);
    assert_has_lat_3p(d("daridrA", Adadi), &["daridrati"]);
    assert_has_lat_3p(d("cakAsf~", Adadi), &["cakAsati"]);
    assert_has_lat_3p(d("SAsu~", Adadi), &["SAsati"]);
    assert_has_lat_3p(d("dIDIN", Adadi), &["dIDyate"]);
    assert_has_lat_3p(d("vevIN", Adadi), &["vevyate"]);
}

#[test]
fn sutra_6_1_8() {
    assert_has_lit_p(&[], &d("qupa\\ca~^z", Bhvadi), &["papAca"]);
    assert_has_lit_p(&[], &d("paWa~", Bhvadi), &["papAWa"]);
    assert_has_lit_p(&["pra"], &d("UrRuY", Adadi), &["prorRunAva"]);
    // liwi
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), Krt::tfc, &["hartf"]);
    // TODO: nonAva
}

#[test]
fn sutra_6_1_9() {
    let san = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::San]);
    let yan = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::Yan]);

    let pac = d("qupa\\ca~^z", Bhvadi);
    let f = d("f\\", Juhotyadi);
    assert_has_lat_p(&[], &san(&pac), &["pipakzati"]);
    assert_has_lat(&[], &san(&d("patx~", Bhvadi)), &["pipatizati"]);
    assert_has_lat_p(&[], &san(&f), &["aririzati"]);
    assert_has_lat_p(&[], &san(&d("undI~", Rudhadi)), &["undidizati"]);
    // yan
    assert_has_lat(&[], &yan(&pac), &["pApacyate"]);
    assert_has_lat(&[], &yan(&d("awa~", Bhvadi)), &["awAwyate"]);
    assert_has_lat(&[], &yan(&d("ya\\ja~^", Bhvadi)), &["yAyajyate"]);
    assert_has_lat(&[], &yan(&f), &["arAryate"]);
    assert_has_lat(&["pra"], &yan(&d("UrRuY", Adadi)), &["prorRonUyate"]);
    // anaByAsasya
    assert_has_lat(&[], &san(&d("gupa~\\", Bhvadi)), &["jugupsizate"]);
    let lu_yan_san = d("lUY", Kryadi).with_sanadi(&[Sanadi::Yan, Sanadi::San]);
    assert_has_lat(&[], &lu_yan_san, &["lolUyizate"]);
}

#[test]
fn sutra_6_1_10() {
    assert_has_lat_p(&[], &d("hu\\", Juhotyadi), &["juhoti"]);
    assert_has_lat_p(&[], &d("YiBI\\", Juhotyadi), &["biBeti"]);
    assert_has_lat_p(&[], &d("hrI\\", Juhotyadi), &["jihreti"]);
}
#[test]
fn sutra_6_1_11() {
    let nic = |u, g| Dhatu::new(u, g).with_sanadi(&[Sanadi::Nic]);

    assert_has_lun_p(&[], &nic("qupa\\ca~^z", Bhvadi), &["apIpacat"]);
    assert_has_lun_p(&[], &nic("paWa~", Bhvadi), &["apIpaWat"]);
    assert_has_lun_p(&[], &nic("awa~", Bhvadi), &["Awiwat"]);
    assert_has_lun_p(&[], &nic("aSU~\\", Svadi), &["ASiSat"]);
    assert_has_lun_p(&[], &nic("aSa~\\", Kryadi), &["ASiSat"]);
}

#[test]
fn sutra_6_1_15() {
    let vac = d("va\\ca~", Adadi);
    assert_has_krdanta(&[], &vac, Krt::kta, &["ukta"]);
    assert_has_krdanta(&[], &vac, Krt::ktavatu, &["uktavat"]);

    let svap = d("Yizva\\pa~", Adadi);
    assert_has_krdanta(&[], &svap, Krt::kta, &["supta"]);
    assert_has_krdanta(&[], &svap, Krt::ktavatu, &["suptavat"]);

    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_krdanta(&[], &yaj, Krt::kta, &["izwa"]);
    assert_has_krdanta(&[], &yaj, Krt::ktavatu, &["izwavat"]);

    let vap = d("quva\\pa~^", Bhvadi);
    assert_has_krdanta(&[], &vap, Krt::kta, &["upta"]);
    assert_has_krdanta(&[], &vap, Krt::ktavatu, &["uptavat"]);

    let vah = d("va\\ha~^", Bhvadi);
    assert_has_krdanta(&[], &vah, Krt::kta, &["UQa"]);
    assert_has_krdanta(&[], &vah, Krt::ktavatu, &["UQavat"]);

    let vas = d("va\\sa~", Bhvadi);
    assert_has_krdanta(&[], &vas, Krt::kta, &["uzita"]);
    assert_has_krdanta(&[], &vas, Krt::ktavatu, &["uzitavat"]);

    let ve = d("ve\\Y", Bhvadi);
    assert_has_krdanta(&[], &ve, Krt::kta, &["uta"]);
    assert_has_krdanta(&[], &ve, Krt::ktavatu, &["utavat"]);

    let vye = d("vye\\Y", Bhvadi);
    assert_has_krdanta(&["sam"], &vye, Krt::kta, &["saMvIta"]);
    assert_has_krdanta(&["sam"], &vye, Krt::ktavatu, &["saMvItavat"]);

    let hve = d("hve\\Y", Bhvadi);
    assert_has_krdanta(&["AN"], &hve, Krt::ktavatu, &["AhUtavat"]);

    let vad = d("vada~", Bhvadi);
    assert_has_krdanta(&[], &vad, Krt::kta, &["udita"]);
    assert_has_krdanta(&[], &vad, Krt::ktavatu, &["uditavat"]);

    let svi = d("wuo~Svi", Bhvadi);
    assert_has_krdanta(&[], &svi, Krt::kta, &["SUna"]);
    assert_has_krdanta(&[], &svi, Krt::ktavatu, &["SUnavat"]);
}

#[test]
fn sutra_6_1_18() {
    let svap_nic = d("Yizva\\pa~", Adadi).with_sanadi(&[Sanadi::Nic]);
    assert_has_parasmai_tinanta(&[], &svap_nic, Lun, Prathama, Eka, &["asUzupat"]);
    assert_has_parasmai_tinanta(&[], &svap_nic, Lun, Prathama, Dvi, &["asUzupatAm"]);
    assert_has_parasmai_tinanta(&[], &svap_nic, Lun, Prathama, Bahu, &["asUzupan"]);
    // caNi
    assert_has_lat_karmani(&[], &svap_nic, &["svApyate"]);
    assert_has_krdanta(&[], &svap_nic, Krt::kta, &["svApita"]);
}

#[test]
fn sutra_6_1_19() {
    let yan = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::Yan]);

    let svap = d("Yizva\\pa~", Adadi);
    let syam = d("syamu~", Bhvadi);
    let ve = d("vye\\Y", Bhvadi);
    assert_has_lat(&[], &yan(&svap), &["sozupyate"]);
    assert_has_lat(&[], &yan(&syam), &["sesimyate"]);
    assert_has_lat(&[], &yan(&ve), &["vevIyate"]);
    // yaNi
    assert_has_krdanta(&[], &svap, Krt::najiN, &["svapnaj"]);
}

#[test]
fn sutra_6_1_20() {
    let yan = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::Yan]);
    let vash = Dhatu::new("vaSa~", Adadi);
    assert_has_tinanta(&[], &yan(&vash), Lat, Prathama, Eka, &["vAvaSyate"]);
    assert_has_tinanta(&[], &yan(&vash), Lat, Prathama, Dvi, &["vAvaSyete"]);
    assert_has_tinanta(&[], &yan(&vash), Lat, Prathama, Bahu, &["vAvaSyante"]);
    assert_has_parasmai_tinanta(&[], &vash, Lat, Prathama, Dvi, &["uzwaH"]);
    assert_has_parasmai_tinanta(&[], &vash, Lat, Prathama, Bahu, &["uSanti"]);
}

#[ignore]
#[test]
fn sutra_6_1_21() {
    let caay_yan = Dhatu::new("cAyf~^", Bhvadi).with_sanadi(&[Sanadi::Yan]);
    assert_has_tinanta(&[], &caay_yan, Lat, Prathama, Eka, &["cekIyate"]);
    assert_has_tinanta(&[], &caay_yan, Lat, Prathama, Dvi, &["cekIyete"]);
    assert_has_tinanta(&[], &caay_yan, Lat, Prathama, Bahu, &["cekIyante"]);
    assert_has_krdanta(&[], &caay_yan, Krt::kta, &["cekIta"]);
}

#[test]
fn sutra_6_1_22() {
    let sphay = Dhatu::new("sPAyI~\\", Bhvadi);
    assert_has_krdanta(&[], &sphay, Krt::kta, &["sPIta"]);
    assert_has_krdanta(&[], &sphay, Krt::ktavatu, &["sPItavat"]);
    assert_has_krdanta(&[], &sphay, Krt::ktin, &["sPAti"]);
}

#[ignore]
#[test]
fn sutra_6_1_23() {
    let styai = Dhatu::new("styE\\", Bhvadi);
    assert_has_krdanta(&["pra"], &styai, Krt::kta, &["prastIta"]);
    assert_has_krdanta(&["pra"], &styai, Krt::ktavatu, &["prastItavat"]);
    let zwyai = Dhatu::new("zwyE\\", Bhvadi);
    assert_has_krdanta(&["pra"], &zwyai, Krt::kta, &["prastIta"]);
    assert_has_krdanta(&["pra"], &zwyai, Krt::ktavatu, &["prastItavat"]);
}

#[test]
fn sutra_6_1_45() {
    let glai = d("glE\\", Bhvadi);
    assert_has_krdanta(&[], &glai, Krt::tfc, &["glAtf"]);
    assert_has_krdanta(&[], &glai, Krt::tumun, &["glAtum"]);
    assert_has_krdanta(&[], &glai, Krt::tavya, &["glAtavya"]);
    let so = d("So\\", Divadi);
    assert_has_krdanta(&["ni"], &so, Krt::tfc, &["niSAtf"]);
    assert_has_krdanta(&["ni"], &so, Krt::tumun, &["niSAtum"]);
    assert_has_krdanta(&["ni"], &so, Krt::tavya, &["niSAtavya"]);
    // ecaH
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), Krt::tfc, &["hartf"]);
    // upadeSe
    assert_has_krdanta(&[], &d("ci\\Y", Svadi), Krt::tfc, &["cetf"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), Krt::tfc, &["stotf"]);
    // aSiti
    let mlai = d("mlE\\", Bhvadi);
    assert_has_lat_p(&[], &glai, &["glAyati"]);
    assert_has_lat_p(&[], &mlai, &["mlAyati"]);
    assert_has_lit_karmani(&[], &glai, &["jagle"]);
    assert_has_lit_karmani(&[], &mlai, &["mamle"]);
}

#[test]
fn sutra_6_1_46() {
    let vye = d("vye\\Y", Bhvadi);
    assert_has_lit_p(&["sam"], &vye, &["saMvivyAya"]);
    assert_has_parasmai_tinanta(&["sam"], &vye, Lit, Madhyama, Eka, &["saMvivyayiTa"]);
}

#[test]
fn sutra_6_1_47() {
    assert_has_krdanta(
        &["vi"],
        &d("sPura~", Bhvadi),
        Krt::GaY,
        &["visPAra", "vizPAra"],
    );
    assert_has_krdanta(
        &["vi"],
        &d("sPula~", Bhvadi),
        Krt::GaY,
        &["visPAla", "vizPAla"],
    );
}

#[test]
fn sutra_6_1_48() {
    assert_has_lat_p(&[], &nic(&d("qukrI\\Y", Kryadi)), &["krApayati"]);
    assert_has_lat_p(&["aDi"], &nic(&d("i\\N", Adadi)), &["aDyApayati"]);
    assert_has_lat_p(&[], &nic(&d("ji\\", Bhvadi)), &["jApayati"]);
}

#[test]
fn sutra_6_1_49() {
    assert_has_lat_p(&[], &nic(&d("zi\\Du~", Divadi)), &["sADayati", "seDayati"]);
}

#[test]
fn sutra_6_1_50() {
    let mii = d("mI\\Y", Kryadi);
    assert_has_krdanta(&["pra"], &mii, Krt::tfc, &["pramAtf"]);
    assert_has_krdanta(&["pra"], &mii, Krt::tavya, &["pramAtavya"]);
    assert_has_krdanta(&["pra"], &mii, Krt::tumun, &["pramAtum"]);
    assert_has_krdanta(&["pra"], &mii, Krt::ktvA, &["pramAya"]);
    let mi = d("qumi\\Y", Svadi);
    assert_has_krdanta(&["ni"], &mi, Krt::tfc, &["nimAtf"]);
    assert_has_krdanta(&["ni"], &mi, Krt::tavya, &["nimAtavya"]);
    assert_has_krdanta(&["ni"], &mi, Krt::tumun, &["nimAtum"]);
    assert_has_krdanta(&["ni"], &mi, Krt::ktvA, &["nimAya"]);
    let di = d("dI\\N", Divadi);
    assert_has_krdanta(&["upa"], &di, Krt::tfc, &["upadAtf"]);
    assert_has_krdanta(&["upa"], &di, Krt::tavya, &["upadAtavya"]);
    assert_has_krdanta(&["upa"], &di, Krt::tumun, &["upadAtum"]);
    assert_has_krdanta(&["upa"], &di, Krt::ktvA, &["upadAya"]);
    // TODO: upadAya, IzadupadAna
    // assert_has_krdanta(&["upa"], &di, Krt::GaY, &["upadAya"]);
    // assert_has_krdanta(&["upa"], &di, Krt::yuc, &["upadAna"]);
}

#[test]
fn sutra_6_1_51() {
    let li = d("lI\\N", Divadi);
    assert_has_krdanta(&["vi"], &li, Krt::tfc, &["vilAtf", "viletf"]);
    assert_has_krdanta(&["vi"], &li, Krt::tavya, &["vilAtavya", "viletavya"]);
    assert_has_krdanta(&["vi"], &li, Krt::tumun, &["vilAtum", "viletum"]);
    assert_has_krdanta(&["vi"], &li, Krt::ktvA, &["vilAya", "vilIya"]);
}

#[test]
fn sutra_6_1_54() {
    assert_has_lat_p(&[], &nic(&d("ci\\Y", Svadi)), &["cApayati", "cAyayati"]);
    assert_has_lat_p(&[], &nic(&d("sPura~", Tudadi)), &["sPArayati", "sPorayati"]);
}

#[test]
fn sutra_6_1_55() {
    assert_has_lat_p(
        &["pra"],
        &nic(&d("vI\\", Adadi)),
        &["pravApayati", "pravAyayati"],
    );
}

#[test]
fn sutra_6_1_56() {
    assert_has_lat_a(
        &[],
        &nic(&d("YiBI\\", Juhotyadi)),
        &["BIzayate", "BApayate"],
    );
}

#[ignore]
#[test]
fn sutra_6_1_57() {
    assert_has_lat(
        &["vi"],
        &nic(&d("zmi\\N", Bhvadi)),
        &["vismApayate", "vismAyayati"],
    );
}

#[test]
fn sutra_6_1_58() {
    let sfj = d("sf\\ja~", Tudadi);
    assert_has_krdanta(&[], &sfj, Krt::tfc, &["srazwf"]);
    assert_has_krdanta(&[], &sfj, Krt::tumun, &["srazwum"]);
    assert_has_krdanta(&[], &sfj, Krt::tavya, &["srazwavya"]);

    let dfs = d("df\\Si~r", Bhvadi);
    assert_has_krdanta(&[], &dfs, Krt::tfc, &["drazwf"]);
    assert_has_krdanta(&[], &dfs, Krt::tumun, &["drazwum"]);
    assert_has_krdanta(&[], &dfs, Krt::tavya, &["drazwavya"]);

    assert_has_lun_p(&[], &sfj, &["asrAkzIt"]);
    assert_has_lun_p(&[], &dfs, &["adrAkzIt", "adarSat"]);
    // jhali
    assert_has_krdanta(&[], &sfj, Krt::lyuw, &["sarjana"]);
    assert_has_krdanta(&[], &dfs, Krt::lyuw, &["darSana"]);
    // akiti
    assert_has_krdanta(&[], &sfj, Krt::kta, &["sfzwa"]);
    assert_has_krdanta(&[], &dfs, Krt::kta, &["dfzwa"]);
}

#[test]
fn sutra_6_1_59() {
    let tfp = d("tf\\pa~", Divadi);
    let dfp = d("df\\pa~", Divadi);
    assert_has_krdanta(&[], &tfp, Krt::tfc, &["traptf", "tarptf", "tarpitf"]);
    assert_has_krdanta(&[], &dfp, Krt::tfc, &["draptf", "darptf", "darpitf"]);
    // anudAttasya
    let vfh = d("vfhU~", Tudadi);
    assert_has_krdanta(&[], &vfh, Krt::tfc, &["varQf", "varhitf"]);
    assert_has_krdanta(&[], &vfh, Krt::tumun, &["varQum", "varhitum"]);
    assert_has_krdanta(&[], &vfh, Krt::tavya, &["varQavya", "varhitavya"]);
    // fdupaDasya
    assert_has_krdanta(&[], &d("Bi\\di~^r", Rudhadi), Krt::tfc, &["Bettf"]);
    assert_has_krdanta(&[], &d("Ci\\di~^r", Rudhadi), Krt::tfc, &["Cettf"]);
    // Jali
    assert_has_krdanta(&[], &tfp, Krt::lyuw, &["tarpaRa"]);
    assert_has_krdanta(&[], &dfp, Krt::lyuw, &["darpaRa"]);
    // akiti
    assert_has_krdanta(&[], &tfp, Krt::kta, &["tfpta", "tfpita"]);
    assert_has_krdanta(&[], &dfp, Krt::kta, &["dfpta", "dfpita"]);
}

#[test]
fn sutra_6_1_91() {
    let r = Dhatu::new("f\\", Gana::Bhvadi);
    assert_has_lat(&["upa"], &r, &["upArCati", "upArcCati"]);
    assert_has_lat(&["pra"], &r, &["prArCati", "prArcCati"]);

    let rdh = Dhatu::new("fDu~", Gana::Svadi);
    assert_has_lat(&["upa"], &rdh, &["upArDnoti"]);

    // Exception -- KawvA is not an upasarga
    assert_has_lat(&["KawvA"], &r, &["KawvarCati", "KawvarcCati"]);
}

#[test]
fn sutra_6_1_136() {
    let kr = Dhatu::new("qukf\\Y", Gana::Tanadi);
    assert_has_lan_p(&["sam"], &kr, &["samaskarot", "samakarot"]);
    assert_has_lun_p(&["sam"], &kr, &["samaskArzIt", "samakArzIt"]);
    assert_has_lit_p(&["sam"], &kr, &["saYcaskAra", "saYcakAra"]);
    assert_has_lit_p(&["pari"], &kr, &["paricaskAra", "paricakAra"]);
}

#[test]
fn sutra_6_1_137() {
    let kr = Dhatu::new("qukf\\Y", Gana::Tanadi);

    assert_has_lut_p(&["sam"], &kr, &["saMskartA", "saNkartA"]);
    assert_has_krdanta(&["sam"], &kr, Krt::tumun, &["saMskartum", "saNkartum"]);
    assert_has_krdanta(&["sam"], &kr, Krt::tavya, &["saMskartavya", "saNkartavya"]);

    assert_has_lut_p(&["pari"], &kr, &["parizkartA", "parikartA"]);
    assert_has_krdanta(&["pari"], &kr, Krt::tumun, &["parizkartum", "parikartum"]);
    assert_has_krdanta(
        &["pari"],
        &kr,
        Krt::tavya,
        &["parizkartavya", "parikartavya"],
    );

    assert_has_lut_p(&["upa"], &kr, &["upaskartA", "upakartA"]);
    assert_has_krdanta(&["upa"], &kr, Krt::tumun, &["upaskartum", "upakartum"]);
    assert_has_krdanta(&["upa"], &kr, Krt::tavya, &["upaskartavya", "upakartavya"]);
}

#[test]
fn sutra_6_1_140() {
    let kr = Dhatu::new("kF", Gana::Tudadi);
    assert_has_lat(&["upa"], &kr, &["upaskirati", "upakirati"]);
}

#[ignore]
#[test]
fn sutra_6_1_141() {
    let kr = Dhatu::new("kF", Gana::Tudadi);
    assert_has_krdanta(&["prati"], &kr, Krt::kta, &["pratiskIrRa", "pratikIrRa"]);
}
