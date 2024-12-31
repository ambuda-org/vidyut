extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn sk_2423() {
    let ad = d("a\\da~", Adadi);
    assert_has_tip(&[], &ad, Lat, &["atti"]);
    assert_has_tas(&[], &ad, Lat, &["attaH"]);
    assert_has_jhi(&[], &ad, Lat, &["adanti"]);
}

#[test]
fn sk_2424() {
    // AduH justified by KV 2.4.40.
    let ad = d("a\\da~", Adadi);
    assert_has_tip(&[], &ad, Lit, &["jaGAsa", "Ada"]);
    assert_has_tas(&[], &ad, Lit, &["jakzatuH", "AdatuH"]);
    assert_has_jhi(&[], &ad, Lit, &["jakzuH", "AduH"]);
    assert_has_sip(&[], &ad, Lit, &["jaGasiTa", "AdiTa"]);

    assert_has_tip(&[], &ad, Lut, &["attA"]);
    assert_has_tip(&[], &ad, Lrt, &["atsyati"]);
}

#[test]
fn sk_2425() {
    let ad = d("a\\da~", Adadi);
    assert_has_sip(&[], &ad, Lot, &["adDi", "attAt"]);
    assert_has_mip(&[], &ad, Lot, &["adAni"]);
}

#[test]
fn sk_2426() {
    let ad = d("a\\da~", Adadi);
    assert_has_tip(&[], &ad, Lan, &["Adat"]);
    assert_has_tas(&[], &ad, Lan, &["AttAm"]);
    assert_has_jhi(&[], &ad, Lan, &["Adan"]);
    assert_has_sip(&[], &ad, Lan, &["AdaH"]);
    assert_has_thas(&[], &ad, Lan, &["Attam"]);
    assert_has_tha(&[], &ad, Lan, &["Atta"]);
    assert_has_mip(&[], &ad, Lan, &["Adam"]);
    assert_has_vas(&[], &ad, Lan, &["Adva"]);
    assert_has_mas(&[], &ad, Lan, &["Adma"]);

    assert_has_tip(&[], &ad, VidhiLin, &["adyAt"]);
    assert_has_tas(&[], &ad, VidhiLin, &["adyAtAm"]);
    assert_has_jhi(&[], &ad, VidhiLin, &["adyuH"]);

    assert_has_tas(&[], &ad, AshirLin, &["adyAstAm"]);
    assert_has_jhi(&[], &ad, AshirLin, &["adyAsuH"]);
}

#[test]
fn sk_2427() {
    let ad = d("a\\da~", Adadi);
    assert_has_tip(&[], &ad, Lun, &["aGasat"]);

    let han = d("ha\\na~", Adadi);
    assert_has_tip(&["pra", "ni"], &han, Lat, &["praRihanti"]);
}

#[test]
fn sk_2428() {
    let han = d("ha\\na~", Adadi);
    assert_has_tas(&[], &han, Lat, &["hataH"]);
    assert_has_jhi(&[], &han, Lat, &["Gnanti"]);
}

#[test]
fn sk_2429() {
    let han = d("ha\\na~", Adadi);
    assert_has_mip(&["pra"], &han, Lat, &["prahaRmi", "prahanmi"]);
    assert_has_vas(&["pra"], &han, Lat, &["prahaRvaH", "prahanvaH"]);

    assert_has_tip(&[], &han, Lit, &["jaGAna"]);
    assert_has_tas(&[], &han, Lit, &["jaGnatuH"]);
    assert_has_jhi(&[], &han, Lit, &["jaGnuH"]);
}

#[test]
fn sk_2430() {
    let han = d("ha\\na~", Adadi);
    assert_has_sip(&[], &han, Lit, &["jaGaniTa", "jaGanTa"]);
    assert_has_tip(&[], &han, Lut, &["hantA"]);
    assert_has_tip(&[], &han, Lrt, &["hanizyati"]);
    assert_has_tip(&[], &han, Lot, &["hantu", "hatAt"]);
    assert_has_jhi(&[], &han, Lot, &["Gnantu"]);
}

#[test]
fn sk_2431() {
    let han = d("ha\\na~", Adadi);
    assert_has_sip(&[], &han, Lot, &["jahi", "hatAt"]);
    assert_has_mip(&[], &han, Lot, &["hanAni"]);
    assert_has_vas(&[], &han, Lot, &["hanAva"]);
    assert_has_mas(&[], &han, Lot, &["hanAma"]);

    assert_has_tip(&[], &han, Lan, &["ahan"]);
    assert_has_tas(&[], &han, Lan, &["ahatAm"]);
    assert_has_jhi(&[], &han, Lan, &["aGnan"]);
    assert_has_mip(&[], &han, Lan, &["ahanam"]);
}

#[test]
fn skip_sk_2432_to_sk_2433() {}

#[test]
fn sk_2434() {
    let han = d("ha\\na~", Adadi);
    assert_has_tip(&[], &han, AshirLin, &["vaDyAt"]);
    assert_has_tas(&[], &han, AshirLin, &["vaDyAstAm"]);
    assert_has_tip(&[], &han, VidhiLin, &["hanyAt"]);
    assert_has_tip(&["pra"], &han, VidhiLin, &["prahaRyAt"]);
    assert_has_tip(&[], &han, Lun, &["avaDIt"]);

    let dvish = d("dvi\\za~^", Adadi);
    assert_has_tip(&[], &dvish, Lat, &["dvezwi"]);
    assert_has_ta(&[], &dvish, Lat, &["dvizwe"]);
    assert_has_tip(&[], &dvish, Lut, &["dvezwA"]);
    assert_has_tip(&[], &dvish, Lrt, &["dvekzyati"]);
    assert_has_ta(&[], &dvish, Lrt, &["dvekzyate"]);
    assert_has_tip(&[], &dvish, Lot, &["dvezwu", "dvizwAt"]);
    assert_has_sip(&[], &dvish, Lot, &["dviqQi", "dvizwAt"]);
    assert_has_mip(&[], &dvish, Lot, &["dvezARi"]);
    assert_has_iw(&[], &dvish, Lot, &["dvezE"]);
    assert_has_vahi(&[], &dvish, Lot, &["dvezAvahE"]);
    assert_has_tip(&[], &dvish, Lan, &["advew"]);
}

#[test]
fn sk_2435() {
    let dvish = d("dvi\\za~^", Adadi);
    assert_has_jhi(&[], &dvish, Lan, &["advizuH", "advizan"]);
    assert_has_mip(&[], &dvish, Lan, &["advezam"]);
    assert_has_ta(&[], &dvish, VidhiLin, &["dvizIta"]);
    assert_has_ta(&[], &dvish, AshirLin, &["dvikzIzwa"]);
    assert_has_tip(&[], &dvish, Lun, &["advikzat"]);

    let duh = d("du\\ha~^", Adadi);
    assert_has_tip(&[], &duh, Lat, &["dogDi"]);
    assert_has_tas(&[], &duh, Lat, &["dugDaH"]);
    assert_has_sip(&[], &duh, Lat, &["Dokzi"]);
    assert_has_ta(&[], &duh, Lat, &["dugDe"]);
    assert_has_thaas(&[], &duh, Lat, &["Dukze"]);
    assert_has_dhvam(&[], &duh, Lat, &["DugDve"]);
    assert_has_tip(&[], &duh, Lot, &["dogDu", "dugDAt"]);
    assert_has_sip(&[], &duh, Lot, &["dugDi", "dugDAt"]);
    assert_has_mip(&[], &duh, Lot, &["dohAni"]);
    assert_has_thaas(&[], &duh, Lot, &["Dukzva"]);
    assert_has_dhvam(&[], &duh, Lot, &["DugDvam"]);
    assert_has_iw(&[], &duh, Lot, &["dohE"]);
    assert_has_tip(&[], &duh, Lan, &["aDok"]);
    assert_has_mip(&[], &duh, Lan, &["adoham"]);
    assert_has_dhvam(&[], &duh, Lan, &["aDugDvam"]);
    assert_has_tip(&[], &duh, Lun, &["aDukzat"]);
    // adugDa is from KV 7.3.73
    assert_has_ta(&[], &duh, Lun, &["aDukzata", "adugDa"]);

    let dih = d("di\\ha~^", Adadi);
    assert_has_tip(&["pra", "ni"], &dih, Lat, &["praRidegDi"]);

    let lih = d("li\\ha~^", Adadi);
    assert_has_tip(&[], &lih, Lat, &["leQi"]);
    assert_has_tas(&[], &lih, Lat, &["lIQaH"]);
    assert_has_jhi(&[], &lih, Lat, &["lihanti"]);
    assert_has_sip(&[], &lih, Lat, &["lekzi"]);
    assert_has_ta(&[], &lih, Lat, &["lIQe"]);
    assert_has_thaas(&[], &lih, Lat, &["likze"]);
    assert_has_dhvam(&[], &lih, Lat, &["lIQve"]);
    assert_has_tip(&[], &lih, Lot, &["leQu", "lIQAt"]);
    assert_has_ta(&[], &lih, Lot, &["lIQAm"]);
    assert_has_mip(&[], &lih, Lot, &["lehAni"]);
    assert_has_tip(&[], &lih, Lan, &["alew"]);
    assert_has_ta(&[], &lih, Lun, &["alikzata", "alIQa"]);
    assert_has_tas(&[], &lih, Lun, &["alikzatAm"]);
    // alihvahi is justified.
    assert_has_vahi(&[], &lih, Lun, &["alikzAvahi", "alihvahi"]);

    let caksh = d("ca\\kzi~\\N", Adadi);
    assert_has_ta(&[], &caksh, Lat, &["cazwe"]);
    assert_has_aataam(&[], &caksh, Lat, &["cakzAte"]);
}

#[test]
fn skip_sk_2436() {}

#[test]
fn sk_2437() {
    let caksh = d("ca\\kzi~\\N", Adadi);
    assert_has_tip(&[], &caksh, Lit, &["caKyO", "cakSO"]);
    assert_has_ta(&[], &caksh, Lit, &["caKye", "cakSe", "cacakze"]);
    assert_has_tip(&[], &caksh, Lut, &["KyAtA", "kSAtA"]);
    assert_has_tip(&[], &caksh, Lrt, &["KyAsyati", "kSAsyati"]);
    assert_has_ta(&[], &caksh, Lrt, &["KyAsyate", "kSAsyate"]);
    assert_has_ta(&[], &caksh, Lan, &["acazwa"]);
    assert_has_ta(&[], &caksh, VidhiLin, &["cakzIta"]);
    assert_has_tip(
        &[],
        &caksh,
        AshirLin,
        &["KyAyAt", "KyeyAt", "kSAyAt", "kSeyAt"],
    );
}

#[test]
fn sk_2438() {
    let caksh = d("ca\\kzi~\\N", Adadi);
    assert_has_tip(&[], &caksh, Lun, &["aKyat", "akSAsIt"]);
    assert_has_ta(&[], &caksh, Lun, &["aKyata", "akSAsta"]);

    let ir = d("Ira~\\", Adadi);
    assert_has_ta(&[], &ir, Lat, &["Irte"]);
    assert_has_ta(&[], &ir, Lit, &["IrAYcakre", "IrAmAsa", "IrAmbaBUva"]);
    assert_has_ta(&[], &ir, Lut, &["IritA"]);
    assert_has_ta(&[], &ir, Lrt, &["Irizyate"]);
    assert_has_ta(&[], &ir, Lot, &["IrtAm"]);
    assert_has_thaas(&[], &ir, Lot, &["Irzva"]);
    assert_has_dhvam(&[], &ir, Lot, &["IrDvam"]);
    assert_has_ta(&[], &ir, Lun, &["Erizwa"]);

    let id = d("Iqa~\\", Adadi);
    assert_has_ta(&[], &id, Lat, &["Iwwe"]);
}

#[test]
fn skip_sk_2439() {}

#[test]
fn sk_2440() {
    let id = d("Iqa~\\", Adadi);
    assert_has_thaas(&[], &id, Lat, &["Iqize"]);
    assert_has_dhvam(&[], &id, Lat, &["IqiDve"]);
    assert_has_thaas(&[], &id, Lot, &["Iqizva"]);
    assert_has_dhvam(&[], &id, Lot, &["IqiDvam"]);
    assert_has_dhvam(&[], &id, Lan, &["EqQvam"]);

    let ish = d("ISa~\\", Adadi);
    assert_has_ta(&[], &ish, Lat, &["Izwe"]);
    assert_has_thaas(&[], &ish, Lat, &["ISize"]);
    assert_has_dhvam(&[], &ish, Lat, &["ISiDve"]);

    let aas = d("Asa~\\", Adadi);
    assert_has_ta(&[], &aas, Lat, &["Aste"]);
    assert_has_ta(&[], &aas, Lit, &["AsAYcakre", "AsAmbaBUva", "AsAmAsa"]);
    assert_has_thaas(&[], &aas, Lot, &["Assva"]);
    assert_has_dhvam(&[], &aas, Lot, &["ADvam"]);
    assert_has_ta(&[], &aas, Lun, &["Asizwa"]);

    let shas = d("SAsu~\\", Adadi);
    assert_has_ta(&["AN"], &shas, Lat, &["ASAste"]);
    assert_has_aataam(&["AN"], &shas, Lat, &["ASAsAte"]);

    let vas = d("vasa~\\", Adadi);
    assert_has_ta(&[], &vas, Lat, &["vaste"]);
    assert_has_thaas(&[], &vas, Lat, &["vasse"]);
    assert_has_dhvam(&[], &vas, Lat, &["vaDve"]);
    assert_has_ta(&[], &vas, Lit, &["vavase"]);
    assert_has_ta(&[], &vas, Lut, &["vasitA"]);

    let kans = d("kasi~\\", Adadi);
    assert_has_ta(&[], &kans, Lat, &["kaMste"]);
    assert_has_aataam(&[], &kans, Lat, &["kaMsAte"]);
    assert_has_jha(&[], &kans, Lat, &["kaMsate"]);

    let kas = d("kasa~\\", Adadi);
    assert_has_ta(&[], &kas, Lat, &["kaste"]);

    let kash = d("kaSa~\\", Adadi);
    assert_has_ta(&[], &kash, Lat, &["kazwe"]);
    assert_has_aataam(&[], &kash, Lat, &["kaSAte"]);
    assert_has_thaas(&[], &kash, Lat, &["kakze"]);
    assert_has_dhvam(&[], &kash, Lat, &["kaqQve"]);

    let nins = d("Risi~\\", Adadi);
    assert_has_ta(&[], &nins, Lat, &["niMste"]);

    let nij = d("Riji~\\", Adadi);
    assert_has_ta(&[], &nij, Lat, &["niNkte"]);
    assert_has_thaas(&[], &nij, Lat, &["niNkze"]);
    assert_has_ta(&[], &nij, Lut, &["niYjitA"]);

    let shij = d("Siji~\\", Adadi);
    assert_has_ta(&[], &shij, Lat, &["SiNkte"]);

    let pinj = d("piji~\\", Adadi);
    assert_has_ta(&[], &pinj, Lat, &["piNkte"]);

    let prnj = d("pfji~\\", Adadi);
    assert_has_ta(&[], &prnj, Lat, &["pfNkte"]);

    let vrj = d("vfjI~\\", Adadi);
    assert_has_ta(&[], &vrj, Lat, &["vfkte"]);
    assert_has_aataam(&[], &vrj, Lat, &["vfjAte"]);
    assert_has_thaas(&[], &vrj, Lat, &["vfkze"]);

    let vrji = d("vfji~\\", Adadi);
    assert_has_ta(&[], &vrji, Lat, &["vfNkte"]);

    let prc = d("pfcI~\\", Adadi);
    assert_has_ta(&[], &prc, Lat, &["pfkte"]);

    let su = d("zUN", Adadi);
    assert_has_ta(&[], &su, Lat, &["sUte"]);
    assert_has_ta(&[], &su, Lit, &["suzuve"]);
    assert_has_thaas(&[], &su, Lit, &["suzuvize"]);
    assert_has_ta(&[], &su, Lut, &["sotA", "savitA"]);
    assert_has_iw(&[], &su, Lot, &["suvE"]);
    assert_has_ta(&[], &su, AshirLin, &["savizIzwa", "sozIzwa"]);
    assert_has_ta(&[], &su, Lun, &["asavizwa", "asozwa"]);
}

#[test]
fn sk_2441() {
    let shi = d("SIN", Adadi);
    assert_has_ta(&[], &shi, Lat, &["Sete"]);
    assert_has_aataam(&[], &shi, Lat, &["SayAte"]);
}

#[test]
fn sk_2442() {
    let shi = d("SIN", Adadi);
    assert_has_jha(&[], &shi, Lat, &["Serate"]);
    assert_has_thaas(&[], &shi, Lat, &["Seze"]);
    assert_has_dhvam(&[], &shi, Lat, &["SeDve"]);
    assert_has_iw(&[], &shi, Lat, &["Saye"]);
    assert_has_vahi(&[], &shi, Lat, &["Sevahe"]);
    assert_has_ta(&[], &shi, Lit, &["SiSye"]);
    assert_has_ta(&[], &shi, Lut, &["SayitA"]);
    assert_has_ta(&[], &shi, Lun, &["aSayizwa"]);
}

#[test]
fn sk_2443() {
    let yu = d("yu", Adadi);
    assert_has_tip(&[], &yu, Lat, &["yOti"]);
    assert_has_tas(&[], &yu, Lat, &["yutaH"]);
    assert_has_jhi(&[], &yu, Lat, &["yuvanti"]);
    assert_has_tip(&[], &yu, Lit, &["yuyAva"]);
    assert_has_tip(&[], &yu, Lut, &["yavitA"]);
    assert_has_tip(&[], &yu, VidhiLin, &["yuyAt"]);
    assert_has_tip(&[], &yu, AshirLin, &["yUyAt"]);
    assert_has_tip(&[], &yu, Lun, &["ayAvIt"]);
}

#[test]
fn sk_2444() {
    let ru = d("ru", Adadi);
    assert_has_tip(&[], &ru, Lat, &["ravIti", "rOti"]);
    assert_has_tas(&[], &ru, Lat, &["ruvItaH", "rutaH"]);
    assert_has_jhi(&[], &ru, Lat, &["ruvanti"]);

    let sham = d("Samu~", Divadi);
    assert_has_tip(&[], &sham, Lat, &["SAmyati"]);

    // TODO: sk has rUvIyAt?
    assert_has_tip(&[], &ru, VidhiLin, &["ruvIyAt", "ruyAt"]);
    assert_has_tip(&[], &ru, AshirLin, &["rUyAt"]);
    assert_has_tip(&[], &ru, Lun, &["arAvIt"]);
    assert_has_tip(&[], &ru, Lrn, &["aravizyat"]);

    let tu = d("tu\\", Adadi);
    assert_has_tip(&[], &tu, Lat, &["tavIti", "tOti"]);
    assert_has_tas(&[], &tu, Lat, &["tuvItaH", "tutaH"]);
    assert_has_tip(&[], &tu, Lut, &["totA"]);
    assert_has_tip(&[], &tu, Lrt, &["tozyati"]);

    let nu = d("Ru", Adadi);
    assert_has_tip(&[], &nu, Lat, &["nOti"]);
    assert_has_tip(&[], &nu, Lut, &["navitA"]);

    let kshu = d("wukzu", Adadi);
    assert_has_tip(&[], &kshu, Lat, &["kzOti"]);
    assert_has_tip(&[], &kshu, Lut, &["kzavitA"]);

    let kshnu = d("kzRu", Adadi);
    assert_has_tip(&[], &kshnu, Lat, &["kzROti"]);
    assert_has_tip(&[], &kshnu, Lut, &["kzRavitA"]);

    let snu = d("zRu", Adadi);
    assert_has_tip(&[], &snu, Lat, &["snOti"]);
    assert_has_tip(&[], &snu, Lit, &["suzRAva"]);
    assert_has_tip(&[], &snu, Lut, &["snavitA"]);
    assert_has_tip(&[], &snu, VidhiLin, &["snuyAt"]);
    assert_has_tip(&[], &snu, AshirLin, &["snUyAt"]);
}

#[test]
fn sk_2445() {
    let urnu = d("UrRuY", Adadi);
    assert_has_tip(&[], &urnu, Lat, &["UrROti", "UrRoti"]);
    assert_has_tas(&[], &urnu, Lat, &["UrRutaH"]);
    assert_has_jhi(&[], &urnu, Lat, &["UrRuvanti"]);
    assert_has_ta(&[], &urnu, Lat, &["UrRute"]);
    assert_has_aataam(&[], &urnu, Lat, &["UrRuvAte"]);
    assert_has_jha(&[], &urnu, Lat, &["UrRuvate"]);
}

#[test]
fn sk_2446() {
    let urnu = d("UrRuY", Adadi);
    assert_has_tip(&[], &urnu, Lit, &["UrRunAva"]);
    assert_has_tas(&[], &urnu, Lit, &["UrRunuvatuH"]);
    assert_has_jhi(&[], &urnu, Lit, &["UrRunuvuH"]);
}

#[test]
fn sk_2447() {
    let urnu = d("UrRuY", Adadi);
    assert_has_sip(&[], &urnu, Lit, &["UrRunuviTa", "UrRunaviTa"]);
    assert_has_tip(&[], &urnu, Lut, &["UrRuvitA", "UrRavitA"]);
    assert_has_tip(&[], &urnu, Lot, &["UrROtu", "UrRotu", "UrRutAt"]);
    assert_has_mip(&[], &urnu, Lot, &["UrRavAni"]);
    assert_has_iw(&[], &urnu, Lot, &["UrRavE"]);
}

#[test]
fn sk_2448() {
    let urnu = d("UrRuY", Adadi);
    assert_has_tip(&[], &urnu, Lan, &["OrRot"]);
    assert_has_sip(&[], &urnu, Lan, &["OrRoH"]);
    assert_has_tip(&[], &urnu, VidhiLin, &["UrRuyAt"]);
    assert_has_sip(&[], &urnu, VidhiLin, &["UrRuyAH"]);
    assert_has_ta(&[], &urnu, AshirLin, &["UrRavizIzwa", "UrRuvizIzwa"]);
    // OrRAvIt is from SK 2449.
    // OrRuvIt is justified.
    assert_has_tip(&[], &urnu, Lun, &["OrRavIt", "OrRuvIt", "OrRAvIt"]);
    // OrRavizwAm is from SK 2449.
    // OrRAvizwAm is justified.
    assert_has_tas(&[], &urnu, Lun, &["OrRuvizwAm", "OrRavizwAm", "OrRAvizwAm"]);
}

#[test]
fn sk_2449() {
    let urnu = d("UrRuY", Adadi);
    assert_has_tip(&[], &urnu, Lun, &["OrRavIt", "OrRuvIt", "OrRAvIt"]);
    assert_has_tas(&[], &urnu, Lun, &["OrRuvizwAm", "OrRavizwAm", "OrRAvizwAm"]);
    // OrRavizuH and OrRAvizuH are justified.
    assert_has_jhi(&[], &urnu, Lun, &["OrRuvizuH", "OrRavizuH", "OrRAvizuH"]);

    let dyu = d("dyu\\", Adadi);
    assert_has_tip(&[], &dyu, Lat, &["dyOti"]);
    assert_has_tip(&[], &dyu, Lut, &["dyotA"]);

    let su = d("zu\\", Adadi);
    assert_has_tip(&[], &su, Lut, &["sotA"]);
    assert_has_tip(&[], &su, Lun, &["asOzIt"]);

    let ku = d("ku\\", Adadi);
    assert_has_tip(&[], &ku, Lut, &["kotA"]);

    let stu = d("zwu\\Y", Adadi);
    assert_has_tip(&[], &stu, Lat, &["stOti", "stavIti"]);
    assert_has_tas(&[], &stu, Lat, &["stutaH", "stuvItaH"]);
    assert_has_ta(&[], &stu, Lat, &["stute", "stuvIte"]);
    assert_has_tip(&[], &stu, Lun, &["astAvIt"]);
    assert_has_tip(&["aBi"], &stu, Lan, &["aByazwOt", "aByazwavIt"]);
    assert_has_tip(
        &["pari"],
        &stu,
        Lan,
        &["paryazwOt", "paryazwavIt", "paryastOt", "paryastavIt"],
    );
}

#[test]
fn sk_2450() {
    let bru = d("brUY", Adadi);
    assert_has_tip(&[], &bru, Lat, &["Aha", "bravIti"]);
    assert_has_tas(&[], &bru, Lat, &["AhatuH", "brUtaH"]);
    assert_has_jhi(&[], &bru, Lat, &["AhuH", "bruvanti"]);
}

#[test]
fn sk_2451() {
    let bru = d("brUY", Adadi);
    assert_has_sip(&[], &bru, Lat, &["AtTa", "bravIzi"]);
    assert_has_thas(&[], &bru, Lat, &["AhaTuH", "brUTaH"]);
}

#[test]
fn sk_2452() {
    let bru = d("brUY", Adadi);
    assert_has_tip(&[], &bru, Lat, &["bravIti", "Aha"]);
    assert_has_tas(&[], &bru, Lat, &["brUtaH", "AhatuH"]);
    assert_has_jhi(&[], &bru, Lat, &["bruvanti", "AhuH"]);
    assert_has_ta(&[], &bru, Lat, &["brUte"]);
}

#[test]
fn sk_2453() {
    let bru = d("brUY", Adadi);
    assert_has_tip(&[], &bru, Lit, &["uvAca"]);
    assert_has_tas(&[], &bru, Lit, &["UcatuH"]);
    assert_has_jhi(&[], &bru, Lit, &["UcuH"]);
    assert_has_sip(&[], &bru, Lit, &["uvaciTa", "uvakTa"]);
    assert_has_ta(&[], &bru, Lit, &["Uce"]);
    assert_has_tip(&[], &bru, Lut, &["vaktA"]);
    assert_has_tip(&[], &bru, Lot, &["bravItu", "brUtAt"]);
    assert_has_tip(&[], &bru, VidhiLin, &["brUyAt"]);
    assert_has_mip(&[], &bru, Lot, &["bravARi"]);
    assert_has_iw(&[], &bru, Lot, &["bravE"]);
    // TODO: why brUyAt again?
    assert_has_tip(&[], &bru, VidhiLin, &["brUyAt"]);
    assert_has_tip(&[], &bru, AshirLin, &["ucyAt"]);
}

#[test]
fn sk_2454() {
    let bru = d("brUY", Adadi);
    assert_has_tip(&[], &bru, Lun, &["avocat"]);
    assert_has_ta(&[], &bru, Lun, &["avocata"]);

    let i = d("i\\R", Adadi);
    assert_has_tip(&[], &i, Lat, &["eti"]);
    assert_has_tas(&[], &i, Lat, &["itaH"]);
}

#[test]
fn sk_2455() {
    let i = d("i\\R", Adadi);
    assert_has_jhi(&[], &i, Lat, &["yanti"]);
    assert_has_tip(&[], &i, Lit, &["iyAya"]);
}

#[test]
fn sk_2456() {
    let i = d("i\\R", Adadi);
    assert_has_tas(&[], &i, Lit, &["IyatuH"]);
    assert_has_jhi(&[], &i, Lit, &["IyuH"]);
    assert_has_sip(&[], &i, Lit, &["iyayiTa", "iyeTa"]);
    assert_has_tip(&[], &i, Lut, &["etA"]);
    assert_has_sip(&[], &i, Lot, &["ihi", "itAt"]);
    assert_has_mip(&[], &i, Lot, &["ayAni"]);
    assert_has_tip(&[], &i, Lan, &["Et"]);
    assert_has_tas(&[], &i, Lan, &["EtAm"]);
    assert_has_jhi(&[], &i, Lan, &["Ayan"]);
    assert_has_tip(&[], &i, VidhiLin, &["iyAt"]);
    assert_has_tip(&[], &i, AshirLin, &["IyAt"]);
}

#[test]
fn sk_2457() {
    let i = d("i\\R", Adadi);
    assert_has_tip(&["nir"], &i, AshirLin, &["niriyAt"]);
    assert_has_tip(&["aBi"], &i, AshirLin, &["aBIyAt"]);
    assert_has_tip(&["sam", "AN"], &i, AshirLin, &["sameyAt"]);

    assert_has_tip(&["sam"], &d("I", Bhvadi), AshirLin, &["samIyAt"]);
}

#[test]
fn sk_2458() {
    let i = d("i\\R", Adadi);
    assert_has_tip(&[], &i, Lun, &["agAt"]);
    assert_has_tas(&[], &i, Lun, &["agAtAm"]);
    assert_has_jhi(&[], &i, Lun, &["aguH"]);

    let i = d("i\\N", Adadi);
    assert_has_ta(&["aDi"], &i, Lat, &["aDIte"]);
    assert_has_aataam(&["aDi"], &i, Lat, &["aDIyAte"]);
    assert_has_jha(&["aDi"], &i, Lat, &["aDIyate"]);
}

#[test]
fn sk_2459() {
    let i = d("i\\N", Adadi);
    assert_has_ta(&["aDi"], &i, Lit, &["aDijage"]);
    assert_has_aataam(&["aDi"], &i, Lit, &["aDijagAte"]);
    assert_has_jha(&["aDi"], &i, Lit, &["aDijagire"]);
    assert_has_ta(&["aDi"], &i, Lut, &["aDyetA"]);
    assert_has_ta(&["aDi"], &i, Lrt, &["aDyezyate"]);
    assert_has_iw(&["aDi"], &i, Lot, &["aDyayE"]);
    assert_has_ta(&["aDi"], &i, Lan, &["aDyEta"]);
    assert_has_aataam(&["aDi"], &i, Lan, &["aDyEyAtAm"]);
    assert_has_jha(&["aDi"], &i, Lan, &["aDyEyata"]);
    assert_has_iw(&["aDi"], &i, Lan, &["aDyEyi"]);
    assert_has_vahi(&["aDi"], &i, Lan, &["aDyEvahi"]);
    assert_has_ta(&["aDi"], &i, VidhiLin, &["aDIyIta"]);
    assert_has_aataam(&["aDi"], &i, VidhiLin, &["aDIyIyAtAm"]);
    assert_has_dhvam(&["aDi"], &i, VidhiLin, &["aDIyIDvam"]);
    assert_has_iw(&["aDi"], &i, VidhiLin, &["aDIyIya"]);
    assert_has_ta(&["aDi"], &i, AshirLin, &["aDyezIzwa"]);
}

#[test]
fn skip_sk_2460_to_sk_2461() {}

// TODO: avyan
#[ignore]
#[test]
fn sk_2462() {
    let in_ = d("i\\N", Adadi);
    assert_has_ta(&["aDi"], &in_, Lun, &["aDyagIzwa", "aDyEzwa"]);
    assert_has_ta(&["aDi"], &in_, Lrn, &["aDyagIzyata", "aDyEzyata"]);

    let ik = d("i\\k", Adadi);
    assert_has_jhi(&["aDi"], &ik, Lat, &["aDiyanti", "aDIyanti"]);
    assert_has_tip(&["aDi"], &ik, Lun, &["aDyagAt"]);

    let vi = d("vI\\", Adadi);
    assert_has_tip(&[], &vi, Lat, &["veti"]);
    assert_has_tas(&[], &vi, Lat, &["vItaH"]);
    assert_has_jhi(&[], &vi, Lat, &["viyanti"]);
    assert_has_sip(&[], &vi, Lat, &["vezi"]);
    assert_has_mip(&[], &vi, Lat, &["vemi"]);
    assert_has_sip(&[], &vi, Lot, &["vIhi", "vItAt"]);
    assert_has_tip(&[], &vi, Lan, &["avet"]);
    assert_has_tas(&[], &vi, Lan, &["avItAm"]);
    assert_has_jhi(&[], &vi, Lan, &["aviyan", "avyan"]);

    let ii = d("I\\", Adadi);
    assert_has_tip(&[], &ii, Lat, &["eti"]);
    assert_has_tas(&[], &ii, Lat, &["ItaH"]);
    assert_has_jhi(&[], &ii, Lat, &["iyanti"]);
    assert_has_tip(&[], &ii, AshirLin, &["IyAt"]);
    assert_has_tip(&[], &ii, Lun, &["EzIt"]);

    let yaa = d("yA\\", Adadi);
    assert_has_tip(&["pra", "ni"], &yaa, Lat, &["praRiyAti"]);
    assert_has_tas(&[], &yaa, Lat, &["yAtaH"]);
    assert_has_jhi(&[], &yaa, Lat, &["yAnti"]);
}

#[test]
fn sk_2463() {
    let yaa = d("yA\\", Adadi);
    assert_has_jhi(&[], &yaa, Lan, &["ayuH", "ayAn"]);
    assert_has_tip(&[], &yaa, AshirLin, &["yAyAt"]);
    assert_has_tas(&[], &yaa, VidhiLin, &["yAyAtAm"]);
    assert_has_tas(&[], &yaa, AshirLin, &["yAyAstAm"]);

    let paa = d("pA\\", Adadi);
    assert_has_tas(&[], &paa, AshirLin, &["pAyAstAm"]);
    assert_has_tip(&[], &paa, Lun, &["apAsIt"]);

    let daa = d("dA\\p", Adadi);
    assert_has_tip(&["pra", "ni"], &daa, Lat, &["praRidAti", "pranidAti"]);
    assert_has_tas(&[], &daa, AshirLin, &["dAyAstAm"]);
    assert_has_tip(&[], &daa, Lun, &["adAsIt"]);

    let maa = d("mA\\", Adadi);
    assert_has_tip(&["pari"], &maa, Lat, &["parimAti"]);
    assert_has_tip(&["pra", "ni"], &maa, Lat, &["praRimAti", "pranimAti"]);

    let vac = d("va\\ca~", Adadi);
    assert_has_tip(&[], &vac, Lat, &["vakti"]);
    assert_has_tas(&[], &vac, Lat, &["vaktaH"]);
    assert_has_sip(&[], &vac, Lot, &["vagDi", "vaktAt"]);
    assert_has_tip(&[], &vac, VidhiLin, &["vacyAt"]);
    assert_has_tip(&[], &vac, AshirLin, &["ucyAt"]);
    assert_has_tip(&[], &vac, Lun, &["avocat"]);
}

#[test]
fn sk_2464() {
    let vid = d("vida~", Adadi);
    assert_has_tip(&[], &vid, Lat, &["veda", "vetti"]);
    assert_has_tas(&[], &vid, Lat, &["vidatuH", "vittaH"]);
    assert_has_jhi(&[], &vid, Lat, &["viduH", "vidanti"]);
    assert_has_sip(&[], &vid, Lat, &["vetTa", "vetsi"]);
    assert_has_thas(&[], &vid, Lat, &["vidaTuH", "vitTaH"]);
    assert_has_tha(&[], &vid, Lat, &["vida", "vitTa"]);
    assert_has_mip(&[], &vid, Lat, &["veda", "vedmi"]);
    assert_has_vas(&[], &vid, Lat, &["vidva", "vidvaH"]);
    assert_has_mas(&[], &vid, Lat, &["vidma", "vidmaH"]);

    assert_has_tip(
        &[],
        &vid,
        Lit,
        &["viveda", "vidAYcakAra", "vidAmbaBUva", "vidAmAsa"],
    );
    assert_has_tas(
        &[],
        &vid,
        Lit,
        &[
            "vividatuH",
            "vidAYcakratuH",
            "vidAmbaBUvatuH",
            "vidAmAsatuH",
        ],
    );
}

#[test]
fn skip_sk_2465() {}

#[test]
fn sk_2466() {
    let vid = d("vida~", Adadi);
    assert_has_tip(
        &[],
        &vid,
        Lot,
        &["vidANkarotu", "vidANkurutAt", "vettu", "vittAt"],
    );
}

#[test]
fn sk_2467() {
    let vid = d("vida~", Adadi);
    assert_has_tip(
        &[],
        &vid,
        Lot,
        &["vidANkarotu", "vidANkurutAt", "vettu", "vittAt"],
    );
    assert_has_sip(
        &[],
        &vid,
        Lot,
        &["vidANkuru", "vidANkurutAt", "vidDi", "vittAt"],
    );
    assert_has_mip(&[], &vid, Lot, &["vidANkaravARi", "vedAni"]);

    assert_has_tip(&[], &vid, Lan, &["avet"]);
    assert_has_tas(&[], &vid, Lan, &["avittAm"]);
    assert_has_jhi(&[], &vid, Lan, &["aviduH"]);
}

#[test]
fn sk_2468() {
    let vid = d("vida~", Adadi);
    assert_has_sip(&[], &vid, Lan, &["aveH", "avet"]);

    let as_ = d("asa~", Adadi);
    assert_has_tip(&[], &as_, Lat, &["asti"]);
}

#[test]
fn sk_2469() {
    let as_ = d("asa~", Adadi);
    assert_has_tas(&[], &as_, Lat, &["staH"]);
    assert_has_jhi(&[], &as_, Lat, &["santi"]);
    assert_has_sip(&[], &as_, Lat, &["asi"]);
    assert_has_thas(&[], &as_, Lat, &["sTaH"]);
    assert_has_tha(&[], &as_, Lat, &["sTa"]);
    assert_has_mip(&[], &as_, Lat, &["asmi"]);
    assert_has_vas(&[], &as_, Lat, &["svaH"]);
    assert_has_mas(&[], &as_, Lat, &["smaH"]);
}

#[test]
fn sk_2470() {
    let as_ = d("asa~", Adadi);
    assert_has_tip(&[], &as_, Lit, &["baBUva"]);
    assert_has_tip(&[], &as_, Lut, &["BavitA"]);
    assert_has_tip(&[], &as_, Lot, &["astu", "stAt"]);
    assert_has_tas(&[], &as_, Lot, &["stAm"]);
    assert_has_jhi(&[], &as_, Lot, &["santu"]);
}

#[test]
fn sk_2471() {
    let as_ = d("asa~", Adadi);

    assert_has_sip(&[], &as_, Lot, &["eDi", "stAt"]);
    assert_has_thas(&[], &as_, Lot, &["stam"]);
    assert_has_tha(&[], &as_, Lot, &["sta"]);
    assert_has_mip(&[], &as_, Lot, &["asAni"]);
    assert_has_vas(&[], &as_, Lot, &["asAva"]);
    assert_has_mas(&[], &as_, Lot, &["asAma"]);

    assert_has_tip(&[], &as_, Lan, &["AsIt"]);
    assert_has_tas(&[], &as_, Lan, &["AstAm"]);
    assert_has_jhi(&[], &as_, Lan, &["Asan"]);
    assert_has_tip(&[], &as_, VidhiLin, &["syAt"]);
    assert_has_tip(&[], &as_, AshirLin, &["BUyAt"]);
    assert_has_tip(&[], &as_, Lun, &["aBUt"]);
}

#[ignore]
#[test]
fn sk_2472() {
    let as_ = d("asa~", Adadi);
    assert_has_tip(&["prAdus"], &as_, VidhiLin, &["prAduHzyAt", "prAduzzyAt"]);
    assert_has_jhi(&["ni"], &as_, Lat, &["nizanti"]);
    assert_has_jhi(&["prAdus"], &as_, Lat, &["prAduHzanti", "prAduzzanti"]);
    assert_has_tas(&["aBi"], &as_, Lat, &["aBistaH"]);
}

#[test]
fn sk_2473() {
    let mrj = d("mfjU~", Adadi);
    assert_has_tip(&[], &mrj, Lat, &["mArzwi"]);
    assert_has_tas(&[], &mrj, Lat, &["mfzwaH"]);
    assert_has_jhi(&[], &mrj, Lat, &["mfjanti", "mArjanti"]);
    assert_has_tip(&[], &mrj, Lit, &["mamArja"]);
    assert_has_tas(&[], &mrj, Lit, &["mamArjatuH", "mamfjatuH"]);
    assert_has_sip(&[], &mrj, Lit, &["mamArjiTa", "mamArzWa"]);
    assert_has_tip(&[], &mrj, Lut, &["mArjitA", "mArzwA"]);
    assert_has_sip(&[], &mrj, Lot, &["mfqQi", "mfzwAt"]);
    assert_has_tip(&[], &mrj, Lan, &["amArw"]);
    assert_has_mip(&[], &mrj, Lan, &["amArjam"]);
    assert_has_tip(&[], &mrj, Lun, &["amArjIt", "amArkzIt"]);
}

#[test]
fn sk_2474() {
    let rud = d("rudi~r", Adadi);
    assert_has_tip(&[], &rud, Lat, &["roditi"]);
    assert_has_tas(&[], &rud, Lat, &["ruditaH"]);
    assert_has_sip(&[], &rud, Lot, &["rudihi", "ruditAt"]);
}

#[test]
fn skip_sk_2475() {}

#[test]
fn sk_2476() {
    let rud = d("rudi~r", Adadi);
    assert_has_tip(&[], &rud, Lan, &["arodIt", "arodat"]);
    assert_has_tas(&[], &rud, Lan, &["aruditAm"]);
    assert_has_jhi(&[], &rud, Lan, &["arudan"]);
    assert_has_sip(&[], &rud, Lan, &["arodIH", "arodaH"]);
    assert_has_tip(&[], &rud, VidhiLin, &["rudyAt"]);
    assert_has_tip(&[], &rud, Lun, &["arudat", "arodIt"]);

    let svap = d("Yizva\\pa~", Adadi);
    assert_has_tip(&[], &svap, Lat, &["svapiti"]);
    assert_has_tas(&[], &svap, Lat, &["svapitaH"]);
    assert_has_tip(&[], &svap, Lit, &["suzvApa"]);
    assert_has_tas(&[], &svap, Lit, &["suzupatuH"]);
    assert_has_jhi(&[], &svap, Lit, &["suzupuH"]);
    assert_has_sip(&[], &svap, Lit, &["suzvapiTa", "suzvapTa"]);
}

#[ignore]
#[test]
fn sk_2477() {
    let svap = d("Yizva\\pa~", Adadi);
    assert_has_tas(&[], &svap, Lit, &["suzuzupatuH"]);
    assert_has_jhi(&[], &svap, Lit, &["suzuzupuH"]);
    assert_has_tip(&[], &svap, Lat, &["susuzvApa"]);
    assert_has_tip(&[], &svap, Lut, &["susvaptA"]);
    assert_has_tip(&[], &svap, Lat, &["asvapIt"]);
    assert_has_tip(&[], &svap, Lat, &["asvapat"]);
    assert_has_tip(&[], &svap, AshirLin, &["svapyAt"]);
    assert_has_tip(&[], &svap, AshirLin, &["supyAt"]);
    assert_has_tip(&[], &svap, AshirLin, &["suzupyAt"]);
    assert_has_tip(&[], &svap, Lat, &["asvApsIt"]);
    assert_has_tip(&[], &svap, Lat, &["Svasiti"]);
    assert_has_tip(&[], &svap, Lut, &["SvasitA"]);
    assert_has_tip(&[], &svap, Lat, &["aSvasIt"]);
    assert_has_tip(&[], &svap, Lat, &["aSvasat"]);
    assert_has_tip(&[], &svap, Lat, &["SvasyAtAm"]);
    assert_has_tas(&[], &svap, AshirLin, &["SvasyAstAm"]);
    assert_has_tip(&[], &svap, Lat, &["hmyantakzaRa"]);
    assert_has_tip(&[], &svap, Lat, &["aSvasIt"]);
    assert_has_tip(&[], &svap, Lat, &["aniti"]);
    assert_has_tip(&[], &svap, Lat, &["Ana"]);
    assert_has_tip(&[], &svap, Lut, &["anitA"]);
    assert_has_tip(&[], &svap, Lat, &["AnIt"]);
    assert_has_tip(&[], &svap, Lat, &["Anat"]);
}

#[test]
fn sk_2478() {
    let an = d("ana~", Adadi);
    assert_has_tip(&["pra"], &an, Lat, &["prARiti"]);

    let jaksh = d("jakza~", Adadi);
    assert_has_tip(&[], &jaksh, Lat, &["jakziti"]);
    assert_has_tas(&[], &jaksh, Lat, &["jakzitaH"]);
}

#[test]
fn sk_2479() {
    let jaksh = d("jakza~", Adadi);
    assert_has_jhi(&[], &jaksh, Lat, &["jakzati"]);
    assert_has_jhi(&[], &jaksh, Lan, &["ajakzuH"]);

    let jagr = d("jAgf", Adadi);
    assert_has_tip(&[], &jagr, Lat, &["jAgarti"]);
    assert_has_tas(&[], &jagr, Lat, &["jAgftaH"]);
    assert_has_jhi(&[], &jagr, Lat, &["jAgrati"]);
    assert_has_tip(
        &[],
        &jagr,
        Lit,
        &["jajAgAra", "jAgarAYcakAra", "jAgarAmbaBUva", "jAgarAmAsa"],
    );
}

#[test]
fn sk_2480() {
    let jagr = d("jAgf", Adadi);
    assert_has_tas(
        &[],
        &jagr,
        Lit,
        &[
            "jajAgaratuH",
            "jAgarAYcakratuH",
            "jAgarAmbaBUvatuH",
            "jAgarAmAsatuH",
        ],
    );
    assert_has_sip(&[], &jagr, Lan, &["ajAgaH"]);
    assert_has_tas(&[], &jagr, Lan, &["ajAgftAm"]);
}

#[test]
fn sk_2481() {
    let jagr = d("jAgf", Adadi);
    assert_has_jhi(&[], &jagr, Lan, &["ajAgaruH"]);
    assert_has_jhi(&[], &jagr, VidhiLin, &["jAgfyuH"]);
    assert_has_tip(&[], &jagr, AshirLin, &["jAgaryAt"]);
    assert_has_tas(&[], &jagr, AshirLin, &["jAgaryAstAm"]);
    assert_has_jhi(&[], &jagr, AshirLin, &["jAgaryAsuH"]);
    assert_has_tip(&[], &jagr, Lun, &["ajAgarIt"]);

    let daridra = d("daridrA", Adadi);
    assert_has_tip(&[], &daridra, Lat, &["daridrAti"]);
}

#[test]
fn sk_2482() {
    let daridra = d("daridrA", Adadi);
    assert_has_tas(&[], &daridra, Lat, &["daridritaH"]);
}

#[test]
fn sk_2483() {
    let daridra = d("daridrA", Adadi);
    assert_has_jhi(&[], &daridra, Lat, &["daridrati"]);

    assert_has_tip(
        &[],
        &daridra,
        Lit,
        &[
            "daridrAYcakAra",
            "daridrAmbaBUva",
            "daridrAmAsa",
            "dadaridrO",
        ],
    );
    assert_has_tas(
        &[],
        &daridra,
        Lit,
        &[
            "daridrAYcakratuH",
            "daridrAmbaBUvatuH",
            "daridrAmAsatuH",
            "dadaridratuH",
        ],
    );
    assert_has_tip(&[], &daridra, Lut, &["daridritA"]);
    assert_has_tip(&[], &daridra, Lan, &["adaridrAt"]);
    assert_has_tas(&[], &daridra, Lan, &["adaridritAm"]);
    assert_has_jhi(&[], &daridra, Lan, &["adaridruH"]);
    assert_has_tip(&[], &daridra, VidhiLin, &["daridriyAt"]);
    assert_has_tip(&[], &daridra, AshirLin, &["daridryAt"]);
    assert_has_tip(&[], &daridra, Lun, &["adaridrIt", "adaridrAsIt"]);

    let cakas = d("cakAsf~", Adadi);
    assert_has_tip(&[], &cakas, Lat, &["cakAsti"]);
    assert_has_jhi(&[], &cakas, Lat, &["cakAsati"]);
    assert_has_tip(
        &[],
        &cakas,
        Lit,
        &["cakAsAYcakAra", "cakAsAmAsa", "cakAsAmbaBUva"],
    );
    // We can't support cakAdDi per "sica evetyeke" because this blocks "tAs + Dve -> tADve".
    assert_has_sip(&[], &cakas, Lot, &["cakADi", "cakAstAt"]);
}

#[test]
fn sk_2484() {
    let cakas = d("cakAsf~", Adadi);
    // We generate "acakAd" but `assert_has_tip` drops it.
    assert_has_tip(&[], &cakas, Lan, &["acakAt"]);
    assert_has_jhi(&[], &cakas, Lan, &["acakAsuH"]);
}

#[test]
fn sk_2485() {
    let cakas = d("cakAsf~", Adadi);
    assert_has_sip(&[], &cakas, Lan, &["acakAH", "acakAt"]);

    let shas = d("SAsu~", Adadi);
    assert_has_tip(&[], &shas, Lat, &["SAsti"]);
}

#[test]
fn sk_2486() {
    let shas = d("SAsu~", Adadi);
    assert_has_tas(&[], &shas, Lat, &["SizwaH"]);
    assert_has_jhi(&[], &shas, Lat, &["SAsati"]);
    assert_has_tas(&[], &shas, Lit, &["SaSAsatuH"]);
    assert_has_tip(&[], &shas, Lot, &["SAstu", "SizwAt"]);
    assert_has_tas(&[], &shas, Lot, &["SizwAm"]);
    assert_has_jhi(&[], &shas, Lot, &["SAsatu"]);
}

#[test]
fn sk_2487() {
    let shas = d("SAsu~", Adadi);
    assert_has_sip(&[], &shas, Lot, &["SADi", "SizwAt"]);
    assert_has_tip(&[], &shas, Lan, &["aSAt"]);
    assert_has_tas(&[], &shas, Lan, &["aSizwAm"]);
    assert_has_jhi(&[], &shas, Lan, &["aSAsuH"]);
    assert_has_sip(&[], &shas, Lan, &["aSAH", "aSAt"]);
    assert_has_tip(&[], &shas, VidhiLin, &["SizyAt"]);
    assert_has_tip(&[], &shas, Lun, &["aSizat"]);
    assert_has_tip(&[], &shas, Lrn, &["aSAsizyat"]);

    let didhi = d("dIDIN", Adadi);
    assert_has_aataam(&[], &didhi, Lat, &["dIDyAte"]);
}

#[test]
fn sk_2488() {
    let didhi = d("dIDIN", Adadi);
    assert_has_iw(&[], &didhi, Lat, &["dIDye"]);
    assert_has_ta(
        &[],
        &didhi,
        Lit,
        &["dIDyAYcakre", "dIDyAmbaBUva", "dIDyAmAsa"],
    );
    assert_has_ta(&[], &didhi, Lut, &["dIDitA"]);
    assert_has_ta(&[], &didhi, Lrt, &["dIDizyate"]);

    let sas = d("zasa~", Adadi);
    assert_has_tip(&[], &sas, Lat, &["sasti"]);
    assert_has_tas(&[], &sas, Lat, &["sastaH"]);
    assert_has_jhi(&[], &sas, Lat, &["sasanti"]);
    assert_has_tip(&[], &sas, Lit, &["sasAsa"]);
    assert_has_tas(&[], &sas, Lit, &["sesatuH"]);
    assert_has_tip(&[], &sas, Lot, &["sastu", "sastAt"]);
    assert_has_sip(&[], &sas, Lot, &["saDi", "sastAt"]);
    assert_has_tip(&[], &sas, Lan, &["asat"]);
    assert_has_tas(&[], &sas, Lan, &["asastAm"]);
    assert_has_sip(&[], &sas, Lan, &["asaH", "asat"]);
    assert_has_tip(&[], &sas, AshirLin, &["sasyAt"]);
    assert_has_tip(&[], &sas, Lun, &["asAsIt", "asasIt"]);

    let sanst = d("zasti~", Adadi);
    assert_has_tip(&[], &sanst, Lat, &["santi", "santti", "saMsti", "saMstti"]);
    assert_has_tas(
        &[],
        &sanst,
        Lat,
        &["santaH", "santtaH", "saMstaH", "saMsttaH"],
    );
    assert_has_jhi(&[], &sanst, Lat, &["saMstanti"]);

    let vash = d("vaSa~", Adadi);
    assert_has_tip(&[], &vash, Lat, &["vazwi"]);
    assert_has_tas(&[], &vash, Lat, &["uzwaH"]);
    assert_has_jhi(&[], &vash, Lat, &["uSanti"]);
    assert_has_sip(&[], &vash, Lat, &["vakzi"]);
    assert_has_thas(&[], &vash, Lat, &["uzWaH"]);
    assert_has_tip(&[], &vash, Lit, &["uvASa"]);
    assert_has_tas(&[], &vash, Lit, &["USatuH"]);
    assert_has_tip(&[], &vash, Lut, &["vaSitA"]);
    assert_has_tip(&[], &vash, Lot, &["vazwu", "uzwAt"]);
    assert_has_sip(&[], &vash, Lot, &["uqQi", "uzwAt"]);
    assert_has_tip(&[], &vash, Lan, &["avaw"]); // Alsa avaq
    assert_has_tas(&[], &vash, Lan, &["OzwAm"]);
    assert_has_jhi(&[], &vash, Lan, &["OSan"]);
    assert_has_mip(&[], &vash, Lan, &["avaSam"]);
    assert_has_tas(&[], &vash, VidhiLin, &["uSyAtAm"]);
    assert_has_tas(&[], &vash, AshirLin, &["uSyAstAm"]);

    let hnu = d("hnu\\N", Adadi);
    assert_has_ta(&[], &hnu, Lat, &["hnute"]);
    assert_has_ta(&[], &hnu, Lit, &["juhnuve"]);
    assert_has_ta(&[], &hnu, VidhiLin, &["hnuvIta"]);
    assert_has_ta(&[], &hnu, AshirLin, &["hnozIzwa"]);
    assert_has_ta(&[], &hnu, Lun, &["ahnozwa"]);
}
