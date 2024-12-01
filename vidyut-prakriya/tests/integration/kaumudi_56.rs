extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn skip_sk_2650() {}

#[test]
fn sk_2651() {
    let bhu = yan_luk(&d("BU", Bhvadi));
    assert_has_tip(&[], &bhu, Lat, &["boBavIti", "boBoti"]);
    assert_has_tas(&[], &bhu, Lat, &["boBUtaH"]);
    assert_has_jhi(&[], &bhu, Lat, &["boBuvati"]);

    assert_has_tip(
        &[],
        &bhu,
        Lit,
        &["boBavAYcakAra", "boBavAmbaBUva", "boBavAmAsa"],
    );
    assert_has_tip(&[], &bhu, Lut, &["boBavitA"]);
    assert_has_tip(&[], &bhu, Lan, &["aboBavIt", "aboBot"]);
    assert_has_tas(&[], &bhu, Lan, &["aboBUtAm"]);
    assert_has_jhi(&[], &bhu, Lan, &["aboBavuH"]);
    assert_has_tip(&[], &bhu, VidhiLin, &["boBUyAt"]);
    assert_has_tas(&[], &bhu, VidhiLin, &["boBUyAtAm"]);
    assert_has_tas(&[], &bhu, AshirLin, &["boBUyAstAm"]);

    // ashtadhyayi.com has "aboBot", but wikisource has "aboBUt".
    assert_has_tip(&[], &bhu, Lun, &["aboBUvIt", "aboBUt"]);
    assert_has_tas(&[], &bhu, Lun, &["aboBUtAm"]);
    assert_has_jhi(&[], &bhu, Lun, &["aboBUvuH"]);
    assert_has_tip(&[], &bhu, Lrn, &["aboBavizyat"]);

    let spardh = yan_luk(&d("sparDa~\\", Bhvadi));
    assert_has_tip(&[], &spardh, Lat, &["pAsparDIti", "pAsparDi", "pAspardDi"]);
    assert_has_tas(&[], &spardh, Lat, &["pAsparDaH", "pAspardDaH"]);
    assert_has_jhi(&[], &spardh, Lat, &["pAsparDati"]);
    assert_has_sip(&[], &spardh, Lat, &["pAsparDIzi", "pAspartsi"]);
    assert_has_sip(
        &[],
        &spardh,
        Lot,
        &["pAsparDi", "pAspardDi", "pAsparDAt", "pAspardDAt"],
    );
    assert_has_tip(&[], &spardh, Lan, &["apAspart", "apAsparDIt"]);
    assert_has_sip(&[], &spardh, Lan, &["apAspAH", "apAspart", "apAsparDIH"]);

    let gadh = yan_luk(&d("gADf~\\", Bhvadi));
    assert_has_tip(&[], &gadh, Lat, &["jAgAdDi", "jAgADIti"]);
    assert_has_sip(&[], &gadh, Lat, &["jAGAtsi", "jAgADIzi"]);
    assert_has_sip(&[], &gadh, Lan, &["ajAGAt", "ajAGAH", "ajAgADIH"]);

    let nath = yan_luk(&d("nATf~\\", Bhvadi));
    assert_has_tip(&[], &nath, Lat, &["nAnAtti", "nAnATIti"]);
    assert_has_tas(&[], &nath, Lat, &["nAnAttaH", "nAnAttaH"]);

    let dadh = yan_luk(&d("daDa~\\", Bhvadi));
    assert_has_tip(&[], &dadh, Lat, &["dAdadDi", "dAdaDIti"]);
    assert_has_sip(&[], &dadh, Lat, &["dADatsi", "dAdaDIzi"]);
    assert_has_tip(&[], &dadh, Lan, &["adADat", "adAdaDIt"]);
    assert_has_tas(&[], &dadh, Lan, &["adAdadDAm"]);
    assert_has_jhi(&[], &dadh, Lan, &["adAdaDuH"]);
    assert_has_sip(&[], &dadh, Lan, &["adADaH", "adADat", "adAdaDIH"]);
    assert_has_tip(&[], &dadh, Lun, &["adAdADIt", "adAdaDIt"]);

    let skund = yan_luk(&d("skudi~\\", Bhvadi));
    assert_has_tip(&[], &skund, Lat, &["coskundIti", "coskunti", "coskuntti"]);
    assert_has_tip(&[], &skund, Lan, &["acoskun", "acoskundIt"]);
    assert_has_tas(&[], &skund, Lan, &["acoskuntAm", "acoskunttAm"]);
    assert_has_jhi(&[], &skund, Lan, &["acoskunduH"]);

    let mud = yan_luk(&d("muda~\\", Bhvadi));
    assert_has_tip(&[], &mud, Lat, &["momudIti", "momotti"]);
    assert_has_tip(
        &[],
        &mud,
        Lit,
        &["momodAYcakAra", "momodAmbaBUva", "momodAmAsa"],
    );
    assert_has_tip(&[], &mud, Lut, &["momoditA"]);
    assert_has_tip(&[], &mud, Lan, &["amomudIt", "amomot"]);
    assert_has_tas(&[], &mud, Lan, &["amomuttAm"]);
    assert_has_jhi(&[], &mud, Lan, &["amomuduH"]);
    assert_has_sip(&[], &mud, Lan, &["amomudIH", "amomoH", "amomot"]);
    assert_has_tip(&[], &mud, Lun, &["amomodIt"]);

    let kurd = yan_luk(&d("kurda~\\", Bhvadi));
    assert_has_tip(&[], &kurd, Lat, &["cokUrti", "cokUrtti", "cokUrdIti"]);
    assert_has_tip(&[], &kurd, Lan, &["acokUrt", "acokUrdIt"]);
    assert_has_sip(&[], &kurd, Lan, &["acokUH", "acokUrt", "acokUrdIH"]);

    let khurd = yan_luk(&d("Kurda~\\", Bhvadi));
    assert_has_sip(&[], &khurd, Lan, &["acoKUH", "acoKUrt", "acoKUrdIH"]);

    let gurd = yan_luk(&d("gurda~\\", Bhvadi));
    assert_has_sip(&[], &gurd, Lan, &["ajogUH", "ajogUrt", "ajogUrdIH"]);

    let vanc = yan_luk(&d("vancu~", Bhvadi));
    assert_has_tip(&[], &vanc, Lat, &["vanIvaYcIti", "vanIvaNkti"]);
    assert_has_tas(&[], &vanc, Lat, &["vanIvaktaH"]);
    assert_has_jhi(&[], &vanc, Lat, &["vanIvacati"]);
    assert_has_tip(&[], &vanc, Lan, &["avanIvaYcIt", "avanIvan"]);

    let gam = yan_luk(&d("ga\\mx~", Bhvadi));
    assert_has_tip(&[], &gam, Lat, &["jaNgamIti", "jaNganti"]);
    // ashtadhyayi.com has "jaNgantaH", but wikisource has "jaNgataH".
    assert_has_tas(&[], &gam, Lat, &["jaNgataH"]);
    assert_has_jhi(&[], &gam, Lat, &["jaNgmati"]);
    assert_has_mip(&[], &gam, Lat, &["jaNganmi", "jaNgamImi"]);
    assert_has_vas(&[], &gam, Lat, &["jaNganvaH"]);
    assert_has_tip(&[], &gam, Lut, &["jaNgamitA"]);
    assert_has_sip(&[], &gam, Lot, &["jaNgahi", "jaNgatAt"]);
    assert_has_tip(&[], &gam, Lan, &["ajaNgan", "ajaNgamIt"]);
    assert_has_tip(&[], &gam, Lun, &["ajaNgamIt"]);
    assert_has_tas(&[], &gam, Lun, &["ajaNgamizwAm"]);

    let han = yan_luk(&d("ha\\na~", Adadi));
    assert_has_tip(&[], &han, Lat, &["jaNGanIti", "jaNGanti"]);
    assert_has_tas(&[], &han, Lat, &["jaNGataH"]);
    assert_has_jhi(&[], &han, Lat, &["jaNGnati"]);
    assert_has_tip(&[], &han, Lut, &["jaNGanitA"]);
    assert_has_sip(&[], &han, Lot, &["jaNGahi", "jaNGatAt"]);
    assert_has_tip(&[], &han, Lan, &["ajaNGanIt", "ajaNGan"]);
    assert_has_tip(&[], &han, VidhiLin, &["jaNGanyAt"]);
    // TODO: not sure why vaDyAt is the form here.
    assert_has_tip(&[], &han, AshirLin, &["vaDyAt"]);
    assert_has_tip(&[], &han, Lun, &["avaDIt"]);
    assert_has_tas(&[], &han, Lun, &["avaDizwAm"]);

    let car = yan_luk(&d("cara~", Bhvadi));
    assert_has_tip(&[], &car, Lat, &["caYcurIti", "caYcUrti"]);
    assert_has_tas(&[], &car, Lat, &["caYcUrtaH"]);
    assert_has_jhi(&[], &car, Lat, &["caYcurati"]);
    // ashtadhyayi.com has "jaNgantaH", but other sources have "acaYcurIt".
    assert_has_tip(&[], &car, Lan, &["acaYcurIt", "acaYcUH"]);

    let khan = yan_luk(&d("Kanu~^", Bhvadi));
    assert_has_tip(&[], &khan, Lat, &["caNKanIti", "caNKanti"]);
    assert_has_tas(&[], &khan, Lat, &["caNKAtaH"]);
    assert_has_jhi(&[], &khan, Lat, &["caNKnati"]);
    assert_has_sip(&[], &khan, Lot, &["caNKAhi", "caNKAtAt"]);
    assert_has_mip(&[], &khan, Lot, &["caNKanAni"]);
    assert_has_tip(&[], &khan, Lan, &["acaNKanIt", "acaNKan"]);
    assert_has_tas(&[], &khan, Lan, &["acaNKAtAm"]);
    assert_has_jhi(&[], &khan, Lan, &["acaNKnuH"]);
    assert_has_tip(&[], &khan, AshirLin, &["caNKAyAt", "caNKanyAt"]);
    assert_has_tip(&[], &khan, Lun, &["acaNKanIt", "acaNKAnIt"]);

    let yu = yan_luk(&d("yu", Adadi));
    assert_has_tip(&[], &yu, Lat, &["yoyoti", "yoyavIti"]);
    assert_has_tip(&[], &yu, Lan, &["ayoyavIt", "ayoyot"]);
    assert_has_tip(&[], &yu, VidhiLin, &["yoyuyAt"]);
    assert_has_tip(&[], &yu, AshirLin, &["yoyUyAt"]);
    assert_has_tip(&[], &yu, Lun, &["ayoyAvIt"]);

    let nu = yan_luk(&d("Ru", Adadi));
    assert_has_tip(&[], &nu, Lat, &["nonavIti", "nonoti"]);

    let haa = yan_luk(&d("o~hA\\k", Juhotyadi));
    assert_has_tip(&[], &haa, Lat, &["jAheti", "jAhAti"]);
    assert_has_tas(&[], &haa, Lat, &["jAhItaH"]);
    assert_has_jhi(&[], &haa, Lat, &["jAhati"]);
    assert_has_sip(&[], &haa, Lat, &["jAhAsi", "jAhezi"]);
    assert_has_thas(&[], &haa, Lat, &["jAhITaH"]);
    assert_has_tha(&[], &haa, Lat, &["jAhITa"]);
    assert_has_sip(&[], &haa, Lot, &["jAhIhi", "jAhItAt"]);
    assert_has_tip(&[], &haa, Lan, &["ajAhet", "ajAhAt"]);
    assert_has_tas(&[], &haa, Lan, &["ajAhItAm"]);
    assert_has_tas(&[], &haa, Lun, &["ajAhAsizwAm"]);
    assert_has_tip(&[], &haa, Lrn, &["ajAhizyat"]);

    let svap = yan_luk(&d("Yizva\\pa~", Adadi));
    assert_has_tip(&[], &svap, Lat, &["sAsvapIti", "sAsvapti"]);
    assert_has_tas(&[], &svap, Lat, &["sAsvaptaH"]);
    assert_has_jhi(&[], &svap, Lat, &["sAsvapati"]);
    assert_has_tip(&[], &svap, Lan, &["asAsvapIt", "asAsvap"]);
    assert_has_tip(&[], &svap, VidhiLin, &["sAsvapyAt"]);
    assert_has_tip(&[], &svap, AshirLin, &["sAsupyAt"]);
    assert_has_tip(&[], &svap, Lun, &["asAsvApIt", "asAsvapIt"]);
}

#[test]
fn skip_sk_2652() {}

#[test]
fn sk_2653() {
    let vrt = yan_luk(&d("vftu~\\", Bhvadi));
    assert_has_tip(
        &[],
        &vrt,
        Lat,
        &[
            "varvftIti",
            "varivftIti",
            "varIvftIti",
            "varvarti",
            "varvartti",
            "varivarti",
            "varivartti",
            "varIvarti",
            "varIvartti",
        ],
    );
    assert_has_tas(&[], &vrt, Lat, &["varvfttaH", "varivfttaH", "varIvfttaH"]);
    assert_has_jhi(&[], &vrt, Lat, &["varvftati", "varivftati", "varIvftati"]);
    assert_has_tip(
        &[],
        &vrt,
        Lit,
        &[
            "varvartAmAsa",
            "varivartAmAsa",
            "varIvartAmAsa",
            "varvartAYcakAra",
            "varivartAYcakAra",
            "varIvartAYcakAra",
            "varvartAmbaBUva",
            "varivartAmbaBUva",
            "varIvartAmbaBUva",
        ],
    );
    assert_has_tip(
        &[],
        &vrt,
        Lut,
        &["varvartitA", "varivartitA", "varIvartitA"],
    );
    assert_has_tip(
        &[],
        &vrt,
        Lrt,
        &["varvartizyati", "varivartizyati", "varIvartizyati"],
    );
    assert_has_tip(
        &[],
        &vrt,
        Lan,
        &[
            "avarvftIt",
            "avarivftIt",
            "avarIvftIt",
            "avarvart",
            "avarivart",
            "avarIvart",
            "avarvftIt",
            "avarivftIt",
            "avarIvftIt",
        ],
    );
    assert_has_sip(
        &[],
        &vrt,
        Lan,
        &[
            "avarvart",
            "avarivart",
            "avarIvart",
            "avarvAH",
            "avarivAH",
            "avarIvAH",
            "avarvftIH",
            "avarivftIH",
            "avarIvftIH",
        ],
    );
    assert_has_tip(
        &[],
        &vrt,
        Lun,
        &["avarvartIt", "avarivartIt", "avarIvartIt"],
    );

    let kr = yan_luk(&d("qukf\\Y", Tanadi));
    assert_has_tip(
        &[],
        &kr,
        Lat,
        &[
            "carkarIti",
            "carikarIti",
            "carIkarIti",
            "carkarti",
            "carikarti",
            "carIkarti",
        ],
    );
    assert_has_tas(&[], &kr, Lat, &["carkftaH", "carikftaH", "carIkftaH"]);
    assert_has_jhi(&[], &kr, Lat, &["carkrati", "carikrati", "carIkrati"]);
    assert_has_tip(
        &[],
        &kr,
        Lit,
        &[
            "carkarAmAsa",
            "carikarAmAsa",
            "carIkarAmAsa",
            "carkarAYcakAra",
            "carikarAYcakAra",
            "carIkarAYcakAra",
            "carkarAmbaBUva",
            "carikarAmbaBUva",
            "carIkarAmbaBUva",
        ],
    );
    assert_has_tip(&[], &kr, Lut, &["carkaritA", "carikaritA", "carIkaritA"]);
    assert_has_tip(
        &[],
        &kr,
        Lan,
        &[
            "acarkarIt",
            "acarikarIt",
            "acarIkarIt",
            "acarkaH",
            "acarikaH",
            "acarIkaH",
        ],
    );
    assert_has_tip(&[], &kr, VidhiLin, &["carkfyAt", "carikfyAt", "carIkfyAt"]);
    assert_has_tip(
        &[],
        &kr,
        AshirLin,
        &["carkriyAt", "carikriyAt", "carIkriyAt"],
    );
    assert_has_tip(&[], &kr, Lun, &["acarkArIt", "acarikArIt", "acarIkArIt"]);

    let krr = yan_luk(&d("kF", Tudadi));
    assert_has_tip(&[], &krr, Lat, &["cAkarti", "cAkarIti"]);

    let x = yan_luk(&d("tF", Tudadi));
    assert_has_tip(&[], &x, Lat, &["tAtarti", "tAtarIti"]);
    assert_has_tas(&[], &x, Lat, &["tAtIrtaH"]);
    assert_has_jhi(&[], &x, Lat, &["tAtirati"]);
    assert_has_sip(&[], &x, Lot, &["tAtIrhi", "tAtIrtAt"]);
    assert_has_mip(&[], &x, Lot, &["tAtarARi"]);
}

#[test]
fn sk_2654() {
    let x = yan_luk(&d("mava~", Bhvadi));
    assert_has_tip(&[], &x, Lat, &["mAmoti", "mAmavIti"]);
    assert_has_tas(&[], &x, Lat, &["mAmUtaH"]);
    assert_has_jhi(&[], &x, Lat, &["mAmavati"]);

    assert_has_sip(&[], &x, Lat, &["mAmozi", "mAmavIzi"]);
    assert_has_mip(&[], &x, Lat, &["mAmomi", "mAmavImi"]);
    assert_has_vas(&[], &x, Lat, &["mAmAvaH"]);
    assert_has_mas(&[], &x, Lat, &["mAmUmaH"]);
    assert_has_sip(&[], &x, Lot, &["mAmUhi", "mAmUtAt"]);
    assert_has_mip(&[], &x, Lot, &["mAmavAni"]);
    assert_has_tip(&[], &x, Lan, &["amAmot", "amAmavIt"]);
    assert_has_sip(&[], &x, Lan, &["amAmoH", "amAmavIH"]);
    assert_has_mip(&[], &x, Lan, &["amAmavam"]);
    assert_has_vas(&[], &x, Lan, &["amAmAva"]);
    assert_has_mas(&[], &x, Lan, &["amAmUma"]);

    let turv = yan_luk(&d("turvI~", Bhvadi));
    assert_has_tip(&[], &turv, Lat, &["totorti", "totUrvIti"]);
}

#[test]
fn skip_sk_2655() {}

#[test]
fn sk_2656() {
    let turv = yan_luk(&d("turvI~", Bhvadi));
    assert_has_tip(&[], &turv, Lat, &["totorti", "totUrvIti"]);
    assert_has_tas(&[], &turv, Lat, &["totUrtaH"]);
    assert_has_jhi(&[], &turv, Lat, &["totUrvati"]);

    let thurv = yan_luk(&d("TurvI~", Bhvadi));
    assert_has_tip(&[], &thurv, Lat, &["toTorti", "toTUrvIti"]);

    let durv = yan_luk(&d("durvI~", Bhvadi));
    assert_has_tip(&[], &durv, Lat, &["dodorti", "dodUrvIti"]);

    let dhurv = yan_luk(&d("DurvI~", Bhvadi));
    assert_has_tip(&[], &dhurv, Lat, &["doDorti", "doDUrvIti"]);

    // momUrCItaH is justified by momUrCIti.
    let murch = yan_luk(&d("murCA~", Bhvadi));
    assert_has_tip(&[], &murch, Lat, &["momUrCIti", "momorti"]);
    assert_has_tas(&[], &murch, Lat, &["momUrtaH"]);
    assert_has_jhi(&[], &murch, Lat, &["momUrCati"]);

    // TODO: this form seems impossible because there is no rule that lets us add yaN-pratyaya
    // here.
    let _aj = yan_luk(&d("aja~", Bhvadi));
    // assert_has_ta(&[], &aj, Lat, &["vevIyate"]);
}
