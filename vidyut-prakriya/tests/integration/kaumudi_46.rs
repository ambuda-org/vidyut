extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[ignore]
#[test]
fn sk_2505() {
    let div = d("divu~", Divadi);
    assert_has_tip(&[], &div, Lat, &["dIvyati"]);
    assert_has_tip(&[], &div, Lit, &["dideva"]);
    assert_has_tip(&[], &div, Lut, &["devitA"]);
    assert_has_tip(&[], &div, Lrt, &["devizyati"]);
    assert_has_tip(&[], &div, Lot, &["dIvyatu", "dIvyatAt"]);
    assert_has_tip(&[], &div, Lan, &["adIvyat"]);
    assert_has_tip(&[], &div, VidhiLin, &["dIvyet"]);
    assert_has_tip(&[], &div, AshirLin, &["dIvyAt"]);
    assert_has_tip(&[], &div, Lun, &["adevIt"]);
    assert_has_tip(&[], &div, Lrn, &["adevizyat"]);

    let siv = d("zivu~", Divadi);
    assert_has_tip(&["pari"], &siv, Lat, &["parizIvyati"]);
    assert_has_tip(&["pari"], &siv, Lit, &["parizizeva"]);
    assert_has_tip(&["ni"], &siv, Lun, &["nyazevIt", "nyasevIt"]);

    let snus = d("zRusu~", Divadi);
    assert_has_tip(&[], &snus, Lat, &["snusyati"]);
    assert_has_tip(&[], &snus, Lit, &["suzRosa"]);

    let snas = d("zRasu~", Divadi);
    assert_has_tip(&[], &snas, Lat, &["snasyati"]);
    assert_has_tip(&[], &snas, Lit, &["sasnAsa"]);

    let knas = d("knasu~", Divadi);
    assert_has_tip(&[], &knas, Lit, &["caknAsa"]);

    let vyush = d("vyuza~", Divadi);
    assert_has_tip(&[], &vyush, Lit, &["vuvyoza"]);

    let nrt = d("nftI~", Divadi);
    assert_has_tip(&[], &nrt, Lat, &["nftyati"]);
    assert_has_tip(&[], &nrt, Lit, &["nanarta"]);
}

#[test]
fn sk_2506() {
    let nrt = d("nftI~", Divadi);
    assert_has_tip(&[], &nrt, Lrt, &["nartizyati", "nartsyati"]);
    assert_has_tip(&[], &nrt, VidhiLin, &["nftyet"]);
    assert_has_tip(&[], &nrt, AshirLin, &["nftyAt"]);
    assert_has_tip(&[], &nrt, Lun, &["anartIt"]);

    let tras = d("trasI~", Divadi);
    assert_has_tip(&[], &tras, Lat, &["trasyati", "trasati"]);
    assert_has_tas(&[], &tras, Lit, &["tresatuH", "tatrasatuH"]);

    let kship = d("kzi\\pa~", Divadi);
    assert_has_tip(&[], &kship, Lat, &["kzipyati"]);
    assert_has_tip(&[], &kship, Lut, &["kzeptA"]);

    let pushp = d("puzpa~", Divadi);
    assert_has_tip(&[], &pushp, Lat, &["puzpyati"]);
    assert_has_tip(&[], &pushp, Lit, &["pupuzpa"]);

    let tim = d("tima~", Divadi);
    assert_has_tip(&[], &tim, Lat, &["timyati"]);

    let stim = d("zwima~", Divadi);
    assert_has_tip(&[], &stim, Lat, &["stimyati"]);

    let stiim = d("zwIma~", Divadi);
    assert_has_tip(&[], &stiim, Lat, &["stImyati"]);

    let vrid = d("vrIqa~", Divadi);
    assert_has_tip(&[], &vrid, Lat, &["vrIqyati"]);

    let ish = d("iza~", Divadi);
    assert_has_tip(&[], &ish, Lat, &["izyati"]);

    let sah = d("zaha~", Divadi);
    assert_has_tip(&[], &sah, Lat, &["sahyati"]);

    let suh = d("zuha~", Divadi);
    assert_has_tip(&[], &suh, Lat, &["suhyati"]);

    let jrr = d("jFz", Divadi);
    assert_has_tip(&[], &jrr, Lat, &["jIryati"]);
    assert_has_tas(&[], &jrr, Lit, &["jajaratuH", "jeratuH"]);
    assert_has_tip(&[], &jrr, Lut, &["jaritA", "jarItA"]);
    assert_has_tip(&[], &jrr, VidhiLin, &["jIryet"]);
    assert_has_tip(&[], &jrr, AshirLin, &["jIryAt"]);
    // ajArIt is from KV 3.1.58.
    assert_has_tip(&[], &jrr, Lun, &["ajarat", "ajArIt"]);
    assert_has_tas(&[], &jrr, Lun, &["ajaratAm", "ajArizwAm"]);

    let jhrr = d("JFz", Divadi);
    assert_has_tip(&[], &jhrr, Lat, &["JIryati"]);
    assert_has_tas(&[], &jhrr, Lit, &["jaJaratuH"]);
    assert_has_tip(&[], &jhrr, Lun, &["aJArIt"]);

    let su = d("zUN", Divadi);
    assert_has_ta(&[], &su, Lat, &["sUyate"]);
    assert_has_ta(&[], &su, Lit, &["suzuve"]);
    assert_has_thaas(&[], &su, Lit, &["suzuvize"]);
    assert_has_vahi(&[], &su, Lit, &["suzuvivahe"]);
    assert_has_ta(&[], &su, Lut, &["sotA", "savitA"]);

    let du = d("dUN", Divadi);
    assert_has_ta(&[], &du, Lat, &["dUyate"]);

    let di = d("dI\\N", Divadi);
    assert_has_ta(&[], &di, Lat, &["dIyate"]);
}

#[test]
fn sk_2507() {
    let di = d("dI\\N", Divadi);
    assert_has_ta(&[], &di, Lit, &["didIye"]);
}

#[test]
fn sk_2508() {
    let di = d("dI\\N", Divadi);
    assert_has_ta(&[], &di, Lut, &["dAtA"]);
    assert_has_ta(&[], &di, Lrt, &["dAsyate"]);
    assert_has_ta(&[], &di, Lun, &["adAsta"]);
    assert_has_thaas(&[], &di, Lun, &["adAsTAH"]);

    let di = d("qIN", Divadi);
    assert_has_ta(&[], &di, Lat, &["qIyate"]);
    assert_has_ta(&[], &di, Lit, &["qiqye"]);

    let dhi = d("DI\\N", Divadi);
    assert_has_ta(&[], &dhi, Lat, &["DIyate"]);
    assert_has_ta(&[], &dhi, Lit, &["diDye"]);
    assert_has_ta(&[], &dhi, Lut, &["DetA"]);

    let mi = d("mI\\N", Divadi);
    assert_has_ta(&[], &mi, Lat, &["mIyate"]);

    let ri = d("rI\\N", Divadi);
    assert_has_ta(&[], &ri, Lat, &["rIyate"]);
}

#[test]
fn sk_2509() {
    let li = d("lI\\N", Divadi);
    assert_has_ta(&[], &li, Lut, &["letA", "lAtA"]);
    assert_has_ta(&[], &li, Lrt, &["lezyate", "lAsyate"]);
    assert_has_ta(&[], &li, Lat, &["lIyate"]);
    assert_has_ta(&[], &li, Lit, &["lilye"]);

    let vri = d("vrI\\N", Divadi);
    assert_has_ta(&[], &vri, Lat, &["vrIyate"]);
    assert_has_ta(&[], &vri, Lit, &["vivriye"]);

    let p = d("pI\\N", Divadi);
    assert_has_ta(&[], &p, Lat, &["pIyate"]);

    let maa = d("mA\\N", Divadi);
    assert_has_ta(&[], &maa, Lat, &["mAyate"]);
    assert_has_ta(&[], &maa, Lit, &["mame"]);

    let ii = d("I\\N", Divadi);
    assert_has_ta(&[], &ii, Lat, &["Iyate"]);
    assert_has_ta(&[], &ii, Lit, &["ayAYcakre", "ayAmbaBUva", "ayAmAsa"]);

    let pri = d("prI\\N", Divadi);
    assert_has_ta(&[], &pri, Lat, &["prIyate"]);
    assert_has_ta(&[], &pri, Lit, &["pipriye"]);
}

#[test]
fn sk_2510() {
    let sho = d("So\\", Divadi);
    assert_has_tip(&[], &sho, Lat, &["Syati"]);
    assert_has_tas(&[], &sho, Lat, &["SyataH"]);
    assert_has_jhi(&[], &sho, Lat, &["Syanti"]);
    assert_has_tip(&[], &sho, Lit, &["SaSO"]);
    assert_has_tas(&[], &sho, Lit, &["SaSatuH"]);
    assert_has_tip(&[], &sho, Lut, &["SAtA"]);
    assert_has_tip(&[], &sho, Lrt, &["SAsyati"]);
    assert_has_tip(&[], &sho, Lun, &["aSAt", "aSAsIt"]);
    assert_has_tas(&[], &sho, Lun, &["aSAtAm", "aSAsizwAm"]);
    assert_has_jhi(&[], &sho, Lun, &["aSuH", "aSAsizuH"]);

    let cho = d("Co\\", Divadi);
    assert_has_tip(&[], &cho, Lat, &["Cyati"]);

    let so = d("zo\\", Divadi);
    assert_has_tip(&[], &so, Lat, &["syati"]);
    assert_has_tip(&[], &so, Lit, &["sasO"]);
    assert_has_tip(&["aBi"], &so, Lat, &["aBizyati"]);
    assert_has_tip(&["aBi"], &so, Lan, &["aByazyat"]);
    assert_has_tip(&["aBi"], &so, Lit, &["aBisasO"]);

    let do_ = d("do\\", Divadi);
    assert_has_tip(&[], &do_, Lat, &["dyati"]);
    assert_has_tip(&[], &do_, Lit, &["dadO"]);
    assert_has_tip(&["pra", "ni"], &do_, Lut, &["praRidAtA"]);
    assert_has_tip(&[], &do_, AshirLin, &["deyAt"]);
    assert_has_tip(&[], &do_, Lun, &["adAt"]);
}

#[test]
fn sk_2511() {
    let jan = d("janI~\\", Divadi);
    assert_has_ta(&[], &jan, Lat, &["jAyate"]);
    assert_has_ta(&[], &jan, Lit, &["jajYe"]);
    assert_has_aataam(&[], &jan, Lit, &["jajYAte"]);
    assert_has_jha(&[], &jan, Lit, &["jajYire"]);
    assert_has_ta(&[], &jan, Lut, &["janitA"]);
    assert_has_ta(&[], &jan, Lrt, &["janizyate"]);
}

#[test]
fn sk_2512() {
    let jan = d("janI~\\", Divadi);
    assert_has_ta(&[], &jan, Lun, &["ajani", "ajanizwa"]);

    let dip = d("dIpI~\\", Divadi);
    assert_has_ta(&[], &dip, Lat, &["dIpyate"]);
    assert_has_ta(&[], &dip, Lit, &["didIpe"]);
    assert_has_ta(&[], &dip, Lun, &["adIpi", "adIpizwa"]);

    let pur = d("pUrI~\\", Divadi);
    assert_has_ta(&[], &pur, Lat, &["pUryate"]);
    assert_has_ta(&[], &pur, Lun, &["apUri", "apUrizwa"]);

    let tur = d("tUrI~\\", Divadi);
    assert_has_ta(&[], &tur, Lat, &["tUryate"]);

    let dhur = d("DUrI~\\", Divadi);
    assert_has_ta(&[], &dhur, Lat, &["DUryate"]);
    assert_has_ta(&[], &dhur, Lit, &["duDUre"]);

    let gur = d("gUrI~\\", Divadi);
    assert_has_ta(&[], &gur, Lat, &["gUryate"]);
    assert_has_ta(&[], &gur, Lit, &["jugUre"]);

    let tap = d("ta\\pa~\\", Divadi);
    assert_has_ta(&[], &tap, Lat, &["tapyate"]);
    assert_has_ta(&[], &tap, Lut, &["taptA"]);
    assert_has_ta(&[], &tap, Lrt, &["tapsyate"]);

    let vrt = d("vftu~\\", Divadi);
    assert_has_ta(&[], &vrt, Lat, &["vftyate"]);

    let klish = d("kliSa~\\", Divadi);
    assert_has_ta(&[], &klish, Lat, &["kliSyate"]);
    assert_has_ta(&[], &klish, Lut, &["kleSitA"]);

    let kash = d("kASf~\\", Divadi);
    assert_has_ta(&[], &kash, Lat, &["kASyate"]);

    let vash = d("vASf~\\", Divadi);
    assert_has_ta(&[], &vash, Lat, &["vASyate"]);
    assert_has_ta(&[], &vash, Lit, &["vavASe"]);

    let mrsh = d("mfza~^", Divadi);
    assert_has_tip(&[], &mrsh, Lat, &["mfzyati"]);
    assert_has_ta(&[], &mrsh, Lat, &["mfzyate"]);
    assert_has_tip(&[], &mrsh, Lit, &["mamarza"]);
    assert_has_ta(&[], &mrsh, Lit, &["mamfze"]);

    let shuc = d("I~Suci~^r", Divadi);
    assert_has_tip(&[], &shuc, Lat, &["Sucyati"]);
    assert_has_ta(&[], &shuc, Lat, &["Sucyate"]);
    assert_has_tip(&[], &shuc, Lit, &["SuSoca"]);
    assert_has_ta(&[], &shuc, Lit, &["SuSuce"]);
    assert_has_tip(&[], &shuc, Lun, &["aSucat", "aSocIt"]);
    assert_has_ta(&[], &shuc, Lun, &["aSocizwa"]);

    let nah = d("Ra\\ha~^", Divadi);
    assert_has_tip(&[], &nah, Lat, &["nahyati"]);
    assert_has_ta(&[], &nah, Lat, &["nahyate"]);
    assert_has_tip(&[], &nah, Lit, &["nanAha"]);
    assert_has_sip(&[], &nah, Lit, &["nanadDa", "nehiTa"]);
    assert_has_ta(&[], &nah, Lit, &["nehe"]);
    assert_has_tip(&[], &nah, Lut, &["nadDA"]);
    assert_has_tip(&[], &nah, Lrt, &["natsyati"]);
    assert_has_tip(&[], &nah, Lun, &["anAtsIt"]);

    let ranj = d("ra\\nja~^", Divadi);
    assert_has_tip(&[], &ranj, Lat, &["rajyati"]);
    assert_has_ta(&[], &ranj, Lat, &["rajyate"]);

    let shap = d("Sa\\pa~^", Divadi);
    assert_has_tip(&[], &shap, Lat, &["Sapyati"]);
    assert_has_ta(&[], &shap, Lat, &["Sapyate"]);

    let pad = d("pa\\da~\\", Divadi);
    assert_has_ta(&[], &pad, Lat, &["padyate"]);
    assert_has_ta(&[], &pad, Lit, &["pede"]);
    assert_has_ta(&[], &pad, Lut, &["pattA"]);
    assert_has_ta(&[], &pad, VidhiLin, &["padyeta"]);
    assert_has_ta(&[], &pad, AshirLin, &["patsIzwa"]);
}

#[test]
fn sk_2513() {
    let pad = d("pa\\da~\\", Divadi);
    assert_has_ta(&[], &pad, Lun, &["apAdi"]);
    assert_has_aataam(&[], &pad, Lun, &["apatsAtAm"]);
    assert_has_jha(&[], &pad, Lun, &["apatsata"]);

    let khid = d("Ki\\da~\\", Divadi);
    assert_has_ta(&[], &khid, Lat, &["Kidyate"]);
    assert_has_ta(&[], &khid, Lit, &["ciKide"]);
    assert_has_ta(&[], &khid, Lut, &["KettA"]);
    assert_has_ta(&[], &khid, Lun, &["aKitta"]);

    let vid = d("vi\\da~\\", Divadi);
    assert_has_ta(&[], &vid, Lat, &["vidyate"]);
    assert_has_ta(&[], &vid, Lut, &["vettA"]);

    let budh = d("bu\\Da~\\", Divadi);
    assert_has_ta(&[], &budh, Lat, &["buDyate"]);
    assert_has_ta(&[], &budh, Lit, &["bubuDe"]);
    assert_has_ta(&[], &budh, Lut, &["bodDA"]);
    assert_has_ta(&[], &budh, Lrt, &["Botsyate"]);
    assert_has_ta(&[], &budh, AshirLin, &["ButsIzwa"]);
    assert_has_ta(&[], &budh, Lun, &["aboDi", "abudDa"]);
    assert_has_aataam(&[], &budh, Lun, &["aButsAtAm"]);

    let yudh = d("yu\\Da~\\", Divadi);
    assert_has_ta(&[], &yudh, Lat, &["yuDyate"]);
    assert_has_ta(&[], &yudh, Lit, &["yuyuDe"]);
    assert_has_ta(&[], &yudh, Lut, &["yodDA"]);
    assert_has_ta(&[], &yudh, Lun, &["ayudDa"]);

    // TODO: yuDyati

    let rudh = d("ru\\Da~\\", Divadi);
    assert_has_ta(&["anu"], &rudh, Lat, &["anuruDyate"]);

    let an = d("aRa~\\", Divadi);
    assert_has_ta(&[], &an, Lat, &["aRyate"]);
    assert_has_ta(&[], &an, Lit, &["ARe"]);
    assert_has_ta(&[], &an, Lut, &["aRitA"]);

    let man = d("ma\\na~\\", Divadi);
    assert_has_ta(&[], &man, Lat, &["manyate"]);
    assert_has_ta(&[], &man, Lit, &["mene"]);
    assert_has_ta(&[], &man, Lut, &["mantA"]);

    let yuj = d("yu\\ja~\\", Divadi);
    assert_has_ta(&[], &yuj, Lat, &["yujyate"]);
    assert_has_ta(&[], &yuj, Lut, &["yoktA"]);

    let srj = d("sf\\ja~\\", Divadi);
    assert_has_ta(&["sam"], &srj, Lat, &["saMsfjyate"]);
    assert_has_thaas(&[], &srj, Lit, &["sasfjize"]);
    assert_has_ta(&[], &srj, Lut, &["srazwA"]);
    assert_has_ta(&[], &srj, Lrt, &["srakzyate"]);
    assert_has_ta(&[], &srj, AshirLin, &["sfkzIzwa"]);
    assert_has_ta(&[], &srj, Lun, &["asfzwa"]);
    assert_has_aataam(&[], &srj, Lun, &["asfkzAtAm"]);

    let lish = d("li\\Sa~\\", Divadi);
    assert_has_ta(&[], &lish, Lat, &["liSyate"]);
    assert_has_ta(&[], &lish, Lut, &["lezwA"]);
    assert_has_ta(&[], &lish, Lrt, &["lekzyate"]);
    assert_has_ta(&[], &lish, AshirLin, &["likzIzwa"]);
    assert_has_ta(&[], &lish, Lun, &["alikzata"]);
    assert_has_aataam(&[], &lish, Lun, &["alikzAtAm"]);

    let raadh = d("rA\\Da~", Divadi);
    assert_has_tip(&[], &raadh, Lit, &["rarADa"]);
    assert_has_tas(&[], &raadh, Lit, &["rarADatuH"]);
    assert_has_sip(&[], &raadh, Lit, &["rarADiTa"]);
    assert_has_tip(&[], &raadh, Lut, &["rAdDA"]);
    assert_has_tip(&[], &raadh, Lrt, &["rAtsyati"]);

    let vyadh = d("vya\\Da~", Divadi);
    assert_has_tip(&[], &vyadh, Lat, &["viDyati"]);
    assert_has_tip(&[], &vyadh, Lit, &["vivyADa"]);
    assert_has_tas(&[], &vyadh, Lit, &["viviDatuH"]);
    assert_has_sip(&[], &vyadh, Lit, &["vivyadDa", "vivyaDiTa"]);
    assert_has_tip(&[], &vyadh, Lut, &["vyadDA"]);
    assert_has_tip(&[], &vyadh, Lrt, &["vyatsyati"]);
    assert_has_tip(&[], &vyadh, VidhiLin, &["viDyet"]);
    assert_has_tip(&[], &vyadh, AshirLin, &["viDyAt"]);
    assert_has_tip(&[], &vyadh, Lun, &["avyAtsIt"]);

    let push = d("pu\\za~", Divadi);
    assert_has_tip(&[], &push, Lat, &["puzyati"]);
    assert_has_tip(&[], &push, Lit, &["pupoza"]);
    assert_has_sip(&[], &push, Lit, &["pupoziTa"]);
    assert_has_tip(&[], &push, Lut, &["pozwA"]);
    assert_has_tip(&[], &push, Lrt, &["pokzyati"]);
    assert_has_tip(&[], &push, Lun, &["apuzat"]);

    let shush = d("Su\\za~", Divadi);
    assert_has_tip(&[], &shush, Lun, &["aSuzat"]);

    let shlish = d("Sli\\za~", Divadi);
    assert_has_tip(&[], &shlish, Lat, &["Slizyati"]);
    assert_has_tip(&[], &shlish, Lit, &["SiSleza"]);
    assert_has_tip(&[], &shlish, Lut, &["SlezwA"]);
    assert_has_tip(&[], &shlish, Lrt, &["Slekzyati"]);
}

#[test]
fn sk_2514() {
    let shlish = d("Sli\\za~", Divadi);
    assert_has_tip(&[], &shlish, Lun, &["aSlikzat", "aSlizat"]);
    assert_has_tip(&["sam", "AN"], &shlish, Lun, &["samASlizat", "samASlikzat"]);
    assert_has_ta_k(&[], &shlish, Lun, &["aSlezi"]);
    assert_has_aataam_k(&[], &shlish, Lun, &["aSlikzAtAm"]);
    assert_has_jha_k(&[], &shlish, Lun, &["aSlikzata", "aSlikzanta"]);
    assert_has_thaas_k(&[], &shlish, Lun, &["aSlizWAH", "aSlikzaTAH"]);
    assert_has_dhvam_k(&[], &shlish, Lun, &["aSliqQvam", "aSlikzaDvam"]);

    let shak = d("Sa\\ka~^", Divadi);
    assert_has_tip(&[], &shak, Lat, &["Sakyati"]);
    assert_has_tip(&[], &shak, Lit, &["SaSAka"]);
    assert_has_sip(&[], &shak, Lit, &["SekiTa", "SaSakTa"]);
    assert_has_ta(&[], &shak, Lit, &["Seke"]);
    assert_has_tip(&[], &shak, Lut, &["SaktA", "SakitA"]);
    assert_has_tip(&[], &shak, Lrt, &["Sakzyati", "Sakizyati"]);
    assert_has_ta(&[], &shak, Lrt, &["Sakzyate", "Sakizyate"]);
    assert_has_tip(&[], &shak, Lun, &["aSakat"]);
    assert_has_ta(&[], &shak, Lun, &["aSakta", "aSakizwa"]);

    let svid = d("zvi\\dA~", Divadi);
    assert_has_tip(&[], &svid, Lat, &["svidyati"]);
    assert_has_tip(&[], &svid, Lit, &["sizveda"]);
    assert_has_sip(&[], &svid, Lit, &["sizvediTa"]);
    assert_has_tip(&[], &svid, Lut, &["svettA"]);
    assert_has_tip(&[], &svid, Lun, &["asvidat"]);

    let krudh = d("kru\\Da~", Divadi);
    assert_has_tip(&[], &krudh, Lut, &["krodDA"]);
    assert_has_tip(&[], &krudh, Lrt, &["krotsyati"]);

    let kshudh = d("kzu\\Da~", Divadi);
    assert_has_tip(&[], &kshudh, Lut, &["kzodDA"]);

    let shudh = d("Su\\Da~", Divadi);
    assert_has_tip(&[], &shudh, Lat, &["SuDyati"]);
    assert_has_tip(&[], &shudh, Lit, &["SuSoDa"]);
    assert_has_tip(&[], &shudh, Lut, &["SodDA"]);

    let sidh = d("zi\\Du~", Divadi);
    assert_has_tip(&[], &sidh, Lat, &["siDyati"]);
    assert_has_tip(&[], &sidh, Lut, &["sedDA"]);
    assert_has_tip(&[], &sidh, Lrt, &["setsyati"]);
    assert_has_tip(&[], &sidh, Lun, &["asiDat"]);

    let radh = d("ra\\Da~", Divadi);
    assert_has_tip(&[], &radh, Lat, &["raDyati"]);
    assert_has_tas(&[], &radh, Lit, &["raranDatuH"]);
}

#[test]
fn sk_2515() {
    let radh = d("ra\\Da~", Divadi);
    assert_has_sip(&[], &radh, Lit, &["raranDiTa", "raradDa"]);
    assert_has_vas(&[], &radh, Lit, &["raranDiva", "reDva"]);
}

#[test]
fn sk_2516() {
    let radh = d("ra\\Da~", Divadi);
    assert_has_tip(&[], &radh, Lut, &["raDitA", "radDA"]);
    assert_has_tip(&[], &radh, Lrt, &["raDizyati", "ratsyati"]);
    assert_has_tip(&[], &radh, Lun, &["araDat"]);

    let nash = d("Ra\\Sa~", Divadi);
    assert_has_tip(&[], &nash, Lat, &["naSyati"]);
    assert_has_tip(&[], &nash, Lit, &["nanASa"]);
    assert_has_tas(&[], &nash, Lit, &["neSatuH"]);
    // nanaMzWa is from SK 2517.
    assert_has_sip(&[], &nash, Lit, &["neSiTa", "nanaMzWa"]);
}

#[test]
fn sk_2517() {
    let nash = d("Ra\\Sa~", Divadi);
    assert_has_sip(&[], &nash, Lit, &["neSiTa", "nanaMzWa"]);
    assert_has_vas(&[], &nash, Lit, &["neSiva", "neSva"]);
    assert_has_mas(&[], &nash, Lit, &["neSima", "neSma"]);
    assert_has_tip(&[], &nash, Lut, &["naSitA", "naMzwA"]);
    assert_has_tip(&[], &nash, Lrt, &["naSizyati", "naNkzyati"]);
    assert_has_tip(&[], &nash, VidhiLin, &["naSyet"]);
    assert_has_tip(&[], &nash, AshirLin, &["naSyAt"]);
    assert_has_tip(&[], &nash, Lun, &["anaSat"]);
    assert_has_tip(&["pra"], &nash, Lat, &["praRaSyati"]);
}

#[test]
fn sk_2518() {
    let nash = d("Ra\\Sa~", Divadi);
    assert_has_tip(&["pra"], &nash, Lut, &["praRaSitA", "pranaMzwA"]);
    assert_has_tip(&["pra"], &nash, Lrt, &["pranaNkzyati", "praRaSizyati"]);

    let trp = d("tf\\pa~", Divadi);
    assert_has_sip(&[], &trp, Lit, &["tatarpiTa", "tatrapTa", "tatarpTa"]);
    assert_has_tip(&[], &trp, Lut, &["tarpitA", "tarptA", "traptA"]);
    assert_has_tip(
        &[],
        &trp,
        Lun,
        &["atArpsIt", "atrApsIt", "atarpIt", "atfpat"],
    );

    let druh = d("dru\\ha~", Divadi);
    assert_has_sip(&[], &druh, Lit, &["dudrogDa", "dudroQa", "dudrohiTa"]);
    assert_has_tip(&[], &druh, Lut, &["drohitA", "drogDA", "droQA"]);
    assert_has_tip(&[], &druh, Lrt, &["drohizyati", "Drokzyati"]);
    assert_has_tip(&[], &druh, Lun, &["adruhat"]);

    let muh = d("mu\\ha~", Divadi);
    assert_has_tip(&[], &muh, Lat, &["muhyati"]);
    assert_has_sip(&[], &muh, Lit, &["mumohiTa", "mumogDa", "mumoQa"]);
    assert_has_tip(&[], &muh, Lut, &["mogDA", "moQA", "mohitA"]);
    assert_has_tip(&[], &muh, Lrt, &["mohizyati", "mokzyati"]);
    assert_has_tip(&[], &muh, Lun, &["amuhat"]);

    let snuh = d("zRu\\ha~", Divadi);
    assert_has_tip(&[], &snuh, Lat, &["snuhyati"]);
    assert_has_tip(&[], &snuh, Lit, &["suzRoha"]);
    assert_has_sip(&[], &snuh, Lit, &["suzRohiTa", "suzRogDa", "suzRoQa"]);
    assert_has_vas(&[], &snuh, Lit, &["suzRuhiva", "suzRuhva"]);
    assert_has_tip(&[], &snuh, Lut, &["snohitA", "snogDA", "snoQA"]);
    assert_has_tip(&[], &snuh, Lrt, &["snohizyati", "snokzyati"]);
    assert_has_tip(&[], &snuh, Lun, &["asnuhat"]);

    let snih = d("zRi\\ha~", Divadi);
    assert_has_tip(&[], &snih, Lat, &["snihyati"]);
    assert_has_tip(&[], &snih, Lit, &["sizReha"]);
}

#[test]
fn sk_2519() {
    let sham = d("Samu~", Divadi);
    assert_has_tas(&[], &sham, Lit, &["SematuH"]);
    assert_has_sip(&[], &sham, Lit, &["SemiTa"]);
    assert_has_tip(&[], &sham, Lut, &["SamitA"]);
    assert_has_tip(&[], &sham, Lun, &["aSamat"]);

    let tam = d("tamu~", Divadi);
    assert_has_tip(&[], &tam, Lat, &["tAmyati"]);
    assert_has_tip(&[], &tam, Lut, &["tamitA"]);
    assert_has_tip(&[], &tam, Lun, &["atamat"]);

    let dam = d("damu~", Divadi);
    assert_has_tip(&[], &dam, Lun, &["adamat"]);

    let shram = d("Sramu~", Divadi);
    assert_has_tip(&[], &shram, Lat, &["SrAmyati"]);
    assert_has_tip(&[], &shram, Lun, &["aSramat"]);

    // Bramati from KV, etc.
    let bhram = d("Bramu~", Divadi);
    assert_has_tip(&[], &bhram, Lat, &["BrAmyati", "Bramati"]);
    assert_has_tip(&[], &bhram, Lun, &["aBramat"]);

    let ksham = d("kzamU~", Divadi);
    assert_has_tip(&[], &ksham, Lat, &["kzAmyati"]);
    assert_has_sip(&[], &ksham, Lit, &["cakzamiTa", "cakzanTa"]);
    assert_has_vas(&[], &ksham, Lit, &["cakzamiva", "cakzaRva"]);
    assert_has_mas(&[], &ksham, Lit, &["cakzamima", "cakzaRma"]);
    assert_has_tip(&[], &ksham, Lut, &["kzamitA", "kzantA"]);

    // TODO: kzAmyati kzAnti kzamate kzamA

    let klam = d("klamu~", Divadi);
    assert_has_tip(&[], &klam, Lat, &["klAmyati", "klAmati"]);
    assert_has_tip(&[], &klam, Lun, &["aklamat"]);

    let mad = d("madI~", Divadi);
    assert_has_tip(&[], &mad, Lat, &["mAdyati"]);
    assert_has_tip(&[], &mad, Lun, &["amadat"]);

    let as_ = d("asu~", Divadi);
    assert_has_tip(&[], &as_, Lat, &["asyati"]);
    assert_has_tip(&[], &as_, Lit, &["Asa"]);
    assert_has_tip(&[], &as_, Lut, &["asitA"]);
}

#[test]
fn sk_2520() {
    let as_ = d("asu~", Divadi);
    assert_has_tip(&[], &as_, Lun, &["AsTat"]);
    assert_has_ta(&["pari"], &as_, Lun, &["paryAsTata"]);
}

#[test]
fn skip_sk_2521() {}

#[test]
fn sk_2522() {
    let yas = d("yasu~", Divadi);
    assert_has_tip(&[], &yas, Lat, &["yasyati", "yasati"]);
    assert_has_tip(&["sam"], &yas, Lat, &["saMyasyati", "saMyasati"]);
    assert_has_tip(&["pra"], &yas, Lat, &["prayasyati"]);

    let jas = d("jasu~", Divadi);
    assert_has_tip(&[], &jas, Lat, &["jasyati"]);

    let tas = d("tasu~", Divadi);
    assert_has_tip(&[], &tas, Lat, &["tasyati"]);
    assert_has_tip(&[], &tas, Lun, &["atasat"]);

    let das = d("dasu~", Divadi);
    assert_has_tip(&[], &das, Lat, &["dasyati"]);
    assert_has_tip(&[], &das, Lun, &["adasat"]);

    let vas = d("vasu~", Divadi);
    assert_has_tip(&[], &vas, Lat, &["vasyati"]);
    assert_has_tip(&[], &vas, Lit, &["vavAsa"]);
    assert_has_tas(&[], &vas, Lit, &["vavasatuH"]);

    let bas = d("basu~", Divadi);
    assert_has_tas(&[], &bas, Lit, &["besatuH"]);
    assert_has_jhi(&[], &bas, Lit, &["besuH"]);

    let vyush = d("vyuza~", Divadi);
    assert_has_tip(&[], &vyush, Lun, &["avyuzat"]);

    let plush = d("pluza~", Divadi);
    assert_has_tip(&[], &plush, Lun, &["apluzat"]);

    let bis = d("bisa~", Divadi);
    assert_has_tip(&[], &bis, Lat, &["bisyati"]);
    assert_has_tip(&[], &bis, Lun, &["abisat"]);

    let kus = d("kusa~", Divadi);
    assert_has_tip(&[], &kus, Lun, &["akusat"]);

    let uc = d("uca~", Divadi);
    assert_has_tip(&[], &uc, Lat, &["ucyati"]);
    assert_has_tip(&[], &uc, Lit, &["uvoca"]);
    assert_has_tas(&[], &uc, Lit, &["UcatuH"]);
    // TODO: mAN-yoga
    assert_has_sip(&[], &uc, Lun, &["OcaH"]);

    let bhrsh = d("BfSu~", Divadi);
    assert_has_tip(&[], &bhrsh, Lit, &["baBarSa"]);
    assert_has_tip(&[], &bhrsh, Lun, &["aBfSat"]);

    let bhransh = d("BranSu~", Divadi);
    assert_has_tip(&[], &bhransh, Lat, &["BraSyati"]);
    assert_has_tip(&[], &bhransh, Lun, &["aBraSat"]);

    let vrsh = d("vfSa~", Divadi);
    assert_has_tip(&[], &vrsh, Lat, &["vfSyati"]);
    assert_has_tip(&[], &vrsh, Lun, &["avfSat"]);

    let krsh = d("kfSa~", Divadi);
    assert_has_tip(&[], &krsh, Lat, &["kfSyati"]);

    let rush = d("ruza~", Divadi);
    assert_has_tip(&[], &rush, Lut, &["rozitA", "rozwA"]);

    let rish = d("riza~", Divadi);
    assert_has_tip(&[], &rish, Lut, &["rezitA", "rezwA"]);

    let yup = d("yupa~", Divadi);
    assert_has_tip(&[], &yup, Lat, &["yupyati"]);

    let rup = d("rupa~", Divadi);
    assert_has_tip(&[], &rup, Lat, &["rupyati"]);

    let lup = d("lupa~", Divadi);
    assert_has_tip(&[], &lup, Lat, &["lupyati"]);
    assert_has_tip(&[], &lup, Lut, &["lopitA"]);

    let lubh = d("luBa~", Divadi);
    assert_has_tip(&[], &lubh, Lut, &["loBitA", "lobDA"]);
    assert_has_tip(&[], &lubh, Lrt, &["loBizyati"]);
    assert_has_tip(&[], &lubh, VidhiLin, &["luByet"]);
    assert_has_tip(&[], &lubh, AshirLin, &["luByAt"]);
    assert_has_tip(&[], &lubh, Lun, &["aluBat"]);

    let kshubh = d("kzuBa~", Divadi);
    assert_has_tip(&[], &kshubh, Lat, &["kzuByati"]);

    let klid = d("klidU~", Divadi);
    assert_has_tip(&[], &klid, Lat, &["klidyati"]);
    assert_has_sip(&[], &klid, Lit, &["ciklediTa", "cikletTa"]);
    assert_has_vas(&[], &klid, Lit, &["ciklidiva", "ciklidva"]);
    assert_has_mas(&[], &klid, Lit, &["ciklidima", "ciklidma"]);
    assert_has_tip(&[], &klid, Lut, &["kleditA", "klettA"]);

    let mid = d("YimidA~", Divadi);
    assert_has_tip(&[], &mid, Lat, &["medyati"]);
    assert_has_tip(&[], &mid, Lun, &["amidat"]);

    let rdh = d("fDu~", Divadi);
    assert_has_tip(&[], &rdh, Lit, &["AnarDa"]);
    assert_has_tip(&[], &rdh, Lun, &["ArDat"]);

    let grdh = d("gfDu~", Divadi);
    assert_has_tip(&[], &grdh, Lun, &["agfDat"]);
}
