extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn skip_sk_2489() {}

#[test]
fn sk_2490() {
    let hu = d("hu\\", Juhotyadi);
    assert_has_tip(&[], &hu, Lat, &["juhoti"]);
    assert_has_tas(&[], &hu, Lat, &["juhutaH"]);
    assert_has_jhi(&[], &hu, Lat, &["juhvati"]);
}

#[test]
fn sk_2491() {
    let hu = d("hu\\", Juhotyadi);
    assert_has_tip(
        &[],
        &hu,
        Lit,
        &["juhAva", "juhavAYcakAra", "juhavAmbaBUva", "juhavAmAsa"],
    );
    assert_has_tip(&[], &hu, Lut, &["hotA"]);
    assert_has_tip(&[], &hu, Lrt, &["hozyati"]);
    assert_has_tip(&[], &hu, Lot, &["juhotu", "juhutAt"]);
    assert_has_sip(&[], &hu, Lot, &["juhuDi", "juhutAt"]);
    assert_has_mip(&[], &hu, Lot, &["juhavAni"]);
    assert_has_jhi(&[], &hu, Lan, &["ajuhavuH"]);
    assert_has_tip(&[], &hu, VidhiLin, &["juhuyAt"]);
    assert_has_tip(&[], &hu, AshirLin, &["hUyAt"]);
    assert_has_tip(&[], &hu, Lun, &["ahOzIt"]);

    let bhi = d("YiBI\\", Juhotyadi);
    assert_has_tip(&[], &bhi, Lat, &["biBeti"]);
}

#[test]
fn sk_2492() {
    let bhi = d("YiBI\\", Juhotyadi);
    assert_has_tas(&[], &bhi, Lat, &["biBitaH", "biBItaH"]);
    assert_has_jhi(&[], &bhi, Lat, &["biByati"]);

    assert_has_tip(
        &[],
        &bhi,
        Lit,
        &["biBAya", "biBayAYcakAra", "biBayAmbaBUva", "biBayAmAsa"],
    );
    assert_has_tip(&[], &bhi, Lut, &["BetA"]);

    let hri = d("hrI\\", Juhotyadi);
    assert_has_tip(&[], &hri, Lat, &["jihreti"]);
    assert_has_tas(&[], &hri, Lat, &["jihrItaH"]);
    assert_has_jhi(&[], &hri, Lat, &["jihriyati"]);
    assert_has_tip(
        &[],
        &hri,
        Lit,
        &["jihrAya", "jihrayAYcakAra", "jihrayAmbaBUva", "jihrayAmAsa"],
    );
}

#[test]
fn skip_sk_2493() {}

#[test]
fn sk_2494() {
    let pr = d("pF", Juhotyadi);
    assert_has_tip(&[], &pr, Lat, &["piparti"]);
    assert_has_tas(&[], &pr, Lat, &["pipUrtaH"]);
    assert_has_jhi(&[], &pr, Lat, &["pipurati"]);
    assert_has_tip(&[], &pr, Lit, &["papAra"]);
}

#[test]
fn sk_2495() {
    let prr = d("pF", Juhotyadi);
    assert_has_tas(&[], &prr, Lit, &["papratuH", "paparatuH"]);
    assert_has_jhi(&[], &prr, Lit, &["papruH", "paparuH"]);
    assert_has_tip(&[], &prr, Lut, &["paritA", "parItA"]);
    assert_has_tip(&[], &prr, Lan, &["apipaH"]);
    assert_has_tas(&[], &prr, Lan, &["apipUrtAm"]);
    assert_has_jhi(&[], &prr, Lan, &["apiparuH"]);
    assert_has_tip(&[], &prr, VidhiLin, &["pipUryAt"]);
    assert_has_tip(&[], &prr, AshirLin, &["pUryAt"]);
    assert_has_tip(&[], &prr, Lun, &["apArIt"]);
    assert_has_tas(&[], &prr, Lun, &["apArizwAm"]);

    let pr = d("pf\\", Juhotyadi);
    assert_has_tip(&[], &pr, Lat, &["piparti"]);
    assert_has_tas(&[], &pr, Lat, &["pipftaH"]);
    assert_has_jhi(&[], &pr, Lat, &["piprati"]);
    assert_has_tip(&[], &pr, VidhiLin, &["pipfyAt"]);
    assert_has_tip(&[], &pr, AshirLin, &["priyAt"]);
    assert_has_tip(&[], &pr, Lun, &["apArzIt"]);
}

#[test]
fn sk_2496() {
    let bhr = d("quBf\\Y", Juhotyadi);
    assert_has_tip(&[], &bhr, Lat, &["biBarti"]);
    assert_has_tas(&[], &bhr, Lat, &["biBftaH"]);
    assert_has_jhi(&[], &bhr, Lat, &["biBrati"]);
    assert_has_dhvam(&[], &bhr, Lat, &["biBfDve"]);
    assert_has_tip(
        &[],
        &bhr,
        Lit,
        &["baBAra", "biBarAYcakAra", "biBarAmbaBUva", "biBarAmAsa"],
    );
    assert_has_sip(
        &[],
        &bhr,
        Lit,
        &[
            "baBarTa",
            "biBarAYcakarTa",
            "biBarAmbaBUviTa",
            "biBarAmAsiTa",
        ],
    );
    assert_has_vas(
        &[],
        &bhr,
        Lit,
        &["baBfva", "biBarAYcakfva", "biBarAmbaBUviva", "biBarAmAsiva"],
    );
    assert_has_sip(&[], &bhr, Lot, &["biBfhi", "biBftAt"]);
    assert_has_mip(&[], &bhr, Lot, &["biBarARi"]);
    assert_has_tip(&[], &bhr, Lan, &["abiBaH"]);
    assert_has_tas(&[], &bhr, Lan, &["abiBftAm"]);
    assert_has_jhi(&[], &bhr, Lan, &["abiBaruH"]);
    assert_has_tip(&[], &bhr, VidhiLin, &["biBfyAt"]);
    assert_has_tip(&[], &bhr, AshirLin, &["BriyAt"]);
    assert_has_ta(&[], &bhr, AshirLin, &["BfzIzwa"]);
    assert_has_tip(&[], &bhr, Lun, &["aBArzIt"]);
    assert_has_ta(&[], &bhr, Lun, &["aBfta"]);
}

#[test]
fn sk_2497() {
    let maa = d("mA\\N", Juhotyadi);
    assert_has_ta(&[], &maa, Lat, &["mimIte"]);
    assert_has_aataam(&[], &maa, Lat, &["mimAte"]);
    assert_has_jha(&[], &maa, Lat, &["mimate"]);
    assert_has_ta(&["pra", "ni"], &maa, Lun, &["praRyamAsta"]);

    let ohaan = d("o~hA\\N", Juhotyadi);
    assert_has_ta(&[], &ohaan, Lat, &["jihIte"]);
    assert_has_aataam(&[], &ohaan, Lat, &["jihAte"]);
    assert_has_jha(&[], &ohaan, Lat, &["jihate"]);
    assert_has_ta(&[], &ohaan, Lit, &["jahe"]);
    assert_has_ta(&[], &ohaan, Lut, &["hAtA"]);
    assert_has_ta(&[], &ohaan, Lrt, &["hAsyate"]);

    let ohaak = d("o~hA\\k", Juhotyadi);
    assert_has_tip(&[], &ohaak, Lat, &["jahAti"]);
}

#[test]
fn sk_2498() {
    let ohaak = d("o~hA\\k", Juhotyadi);
    assert_has_tas(&[], &ohaak, Lat, &["jahitaH", "jahItaH"]);
    assert_has_jhi(&[], &ohaak, Lat, &["jahati"]);
    assert_has_tip(&[], &ohaak, Lit, &["jahO"]);
}

#[test]
fn sk_2499() {
    let ohaak = d("o~hA\\k", Juhotyadi);
    assert_has_sip(
        &[],
        &ohaak,
        Lot,
        &["jahAhi", "jahihi", "jahIhi", "jahitAt", "jahItAt"],
    );
    assert_has_tip(&[], &ohaak, Lan, &["ajahAt"]);
    assert_has_jhi(&[], &ohaak, Lan, &["ajahuH"]);
}

#[test]
fn sk_2500() {
    let ohaak = d("o~hA\\k", Juhotyadi);
    assert_has_tip(&[], &ohaak, VidhiLin, &["jahyAt"]);
    assert_has_tip(&[], &ohaak, AshirLin, &["heyAt"]);
    assert_has_tip(&[], &ohaak, Lun, &["ahAsIt"]);

    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_tip(&["pra", "ni"], &daa, Lat, &["praRidadAti"]);
    assert_has_tas(&[], &daa, Lat, &["dattaH"]);
    assert_has_jhi(&[], &daa, Lat, &["dadati"]);
    assert_has_ta(&[], &daa, Lat, &["datte"]);
    assert_has_tip(&[], &daa, Lit, &["dadO"]);
    assert_has_sip(&[], &daa, Lot, &["dehi", "dattAt"]);
    assert_has_tip(&[], &daa, Lan, &["adadAt"]);
    assert_has_tas(&[], &daa, Lan, &["adattAm"]);
    assert_has_jhi(&[], &daa, Lan, &["adaduH"]);
    assert_has_tip(&[], &daa, VidhiLin, &["dadyAt"]);
    assert_has_tip(&[], &daa, AshirLin, &["deyAt"]);
    assert_has_tip(&[], &daa, Lun, &["adAt"]);
    assert_has_tas(&[], &daa, Lun, &["adAtAm"]);
    assert_has_jhi(&[], &daa, Lun, &["aduH"]);
    assert_has_ta(&[], &daa, Lun, &["adita"]);

    let dhaa = d("quDA\\Y", Juhotyadi);
    assert_has_tip(&["pra", "ni"], &dhaa, Lat, &["praRidaDAti"]);
}

#[test]
fn sk_2501() {
    let dhaa = d("quDA\\Y", Juhotyadi);
    assert_has_tas(&[], &dhaa, Lat, &["DattaH"]);
    assert_has_jhi(&[], &dhaa, Lat, &["daDati"]);
    assert_has_thas(&[], &dhaa, Lat, &["DatTaH"]);
    assert_has_vas(&[], &dhaa, Lat, &["daDvaH"]);
    assert_has_ta(&[], &dhaa, Lat, &["Datte"]);
    assert_has_thaas(&[], &dhaa, Lat, &["Datse"]);
    assert_has_dhvam(&[], &dhaa, Lat, &["DadDve"]);
    assert_has_sip(&[], &dhaa, Lot, &["Dehi", "DattAt"]);
    assert_has_ta(&[], &dhaa, Lun, &["aDita"]);
}

#[test]
fn sk_2502() {
    let nij = d("Ri\\ji~^r", Juhotyadi);
    assert_has_tip(&[], &nij, Lat, &["nenekti"]);
    assert_has_tas(&[], &nij, Lat, &["neniktaH"]);
    assert_has_jhi(&[], &nij, Lat, &["nenijati"]);
    assert_has_tip(&[], &nij, Lit, &["nineja"]);
    assert_has_tip(&[], &nij, Lut, &["nektA"]);
    assert_has_tip(&[], &nij, Lrt, &["nekzyati"]);
    assert_has_tip(&[], &nij, Lot, &["nenektu", "neniktAt"]);
    // TODO: SK has nenigDa?
    assert_has_sip(&[], &nij, Lot, &["nenigDi", "neniktAt"]);
}

#[test]
fn sk_2503() {
    let nij = d("Ri\\ji~^r", Juhotyadi);
    assert_has_mip(&[], &nij, Lot, &["nenijAni"]);
    assert_has_tip(&[], &nij, Lan, &["anenek"]);
    assert_has_tas(&[], &nij, Lan, &["aneniktAm"]);
    assert_has_jhi(&[], &nij, Lan, &["anenijuH"]);
    assert_has_tip(&[], &nij, VidhiLin, &["nenijyAt"]);
    assert_has_tip(&[], &nij, AshirLin, &["nijyAt"]);
    assert_has_tip(&[], &nij, Lun, &["anijat", "anEkzIt"]);
    assert_has_ta(&[], &nij, Lun, &["anikta"]);

    let vij = d("vi\\ji~^r", Juhotyadi);
    assert_has_tip(&[], &vij, Lat, &["vevekti"]);
    assert_has_ta(&[], &vij, Lat, &["vevikte"]);
    assert_has_sip(&[], &vij, Lit, &["vivejiTa"]);

    let vish = d("vi\\zx~^", Juhotyadi);
    assert_has_tip(&[], &vish, Lat, &["vevezwi"]);
    assert_has_ta(&[], &vish, Lat, &["vevizwe"]);
    assert_has_tip(&[], &vish, Lun, &["avizat"]);
    assert_has_ta(&[], &vish, Lun, &["avikzata"]);
    assert_has_aataam(&[], &vish, Lun, &["avikzAtAm"]);
    assert_has_jha(&[], &vish, Lun, &["avikzanta"]);

    let r = d("f\\", Juhotyadi);
    assert_has_tip(&[], &r, Lat, &["iyarti"]);
    assert_has_tas(&[], &r, Lat, &["iyftaH"]);
    assert_has_jhi(&[], &r, Lat, &["iyrati"]);
    assert_has_tip(&[], &r, Lit, &["Ara"]);
    assert_has_tas(&[], &r, Lit, &["AratuH"]);
    assert_has_sip(&[], &r, Lit, &["AriTa"]);
    assert_has_tip(&[], &r, Lut, &["artA"]);
    assert_has_tip(&[], &r, Lrt, &["arizyati"]);
    assert_has_mip(&[], &r, Lot, &["iyarARi"]);
    assert_has_tip(&[], &r, Lan, &["EyaH"]);
    assert_has_tas(&[], &r, Lan, &["EyftAm"]);
    assert_has_jhi(&[], &r, Lan, &["EyaruH"]);
    assert_has_tip(&[], &r, VidhiLin, &["iyfyAt"]);
    assert_has_tip(&[], &r, AshirLin, &["aryAt"]);
    assert_has_tip(&[], &r, Lun, &["Arat"]);

    let sr = d("sf\\", Juhotyadi);
    assert_has_tip(&[], &sr, Lat, &["sasarti"]);

    let bhas = d("Basa~", Juhotyadi);
    assert_has_tip(&[], &bhas, Lat, &["baBasti"]);
    assert_has_tas(&[], &bhas, Lat, &["babDaH"]);
    assert_has_jhi(&[], &bhas, Lat, &["bapsati"]);

    let ki = d("ki\\", Juhotyadi);
    assert_has_tip(&[], &ki, Lat, &["ciketi"]);

    let tur = d("tura~", Juhotyadi);
    assert_has_tip(&[], &tur, Lat, &["tutorti"]);
    assert_has_tas(&[], &tur, Lat, &["tutUrtaH"]);
    assert_has_jhi(&[], &tur, Lat, &["tuturati"]);

    let dhish = d("Diza~", Juhotyadi);
    assert_has_tip(&[], &dhish, Lat, &["diDezwi"]);
    assert_has_tas(&[], &dhish, Lat, &["diDizwaH"]);

    let dhan = d("Dana~", Juhotyadi);
    assert_has_tip(&[], &dhan, Lat, &["daDanti"]);
    assert_has_tas(&[], &dhan, Lat, &["daDantaH"]);
    assert_has_jhi(&[], &dhan, Lat, &["daDanati"]);

    let jan = d("jana~", Juhotyadi);
    assert_has_tip(&[], &jan, Lat, &["jajanti"]);
}

#[test]
fn sk_2504() {
    let jan = d("jana~", Juhotyadi);
    assert_has_tas(&[], &jan, Lat, &["jajAtaH"]);
    assert_has_jhi(&[], &jan, Lat, &["jajYati"]);
    assert_has_sip(&[], &jan, Lat, &["jajaMsi"]);
    assert_has_tip(&[], &jan, Lit, &["jajAna"]);
    assert_has_tip(&[], &jan, VidhiLin, &["jajanyAt", "jajAyAt"]);
    assert_has_tip(&[], &jan, AshirLin, &["janyAt", "jAyAt"]);

    let gaa = d("gA\\", Juhotyadi);
    assert_has_tip(&[], &gaa, Lat, &["jigAti"]);
    assert_has_tas(&[], &gaa, Lat, &["jigItaH"]);
    assert_has_jhi(&[], &gaa, Lat, &["jigati"]);
}
