extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn skip_sk_2151_to_sk_2167() {}

#[test]
fn sk_2168() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &bhu, Lat, &["Bavati"]);
    assert_has_tas(&[], &bhu, Lat, &["BavataH"]);
}

#[test]
fn sk_2169() {
    let bhu = d("BU", Bhvadi);
    assert_has_jhi(&[], &bhu, Lat, &["Bavanti"]);
    assert_has_sip(&[], &bhu, Lat, &["Bavasi"]);
    assert_has_thas(&[], &bhu, Lat, &["BavaTaH"]);
    assert_has_tha(&[], &bhu, Lat, &["BavaTa"]);
}

#[test]
fn sk_2170() {
    let bhu = d("BU", Bhvadi);
    assert_has_mip(&[], &bhu, Lat, &["BavAmi"]);
    assert_has_vas(&[], &bhu, Lat, &["BavAvaH"]);
    assert_has_mas(&[], &bhu, Lat, &["BavAmaH"]);

    // Repeat of forms from SK 2168 and SK 2169.
    assert_has_tip(&[], &bhu, Lat, &["Bavati"]);
    assert_has_tas(&[], &bhu, Lat, &["BavataH"]);
    assert_has_jhi(&[], &bhu, Lat, &["Bavanti"]);
    assert_has_sip(&[], &bhu, Lat, &["Bavasi"]);
    assert_has_thas(&[], &bhu, Lat, &["BavaTaH"]);
    assert_has_tha(&[], &bhu, Lat, &["BavaTa"]);

    let bhuj = d("Bu\\ja~", Rudhadi);
    assert_has_thaas(&[], &bhuj, Lrt, &["Bokzyase"]);
    assert_has_aathaam(&[], &bhuj, Lrt, &["BokzyeTe"]);
    assert_has_dhvam(&[], &bhuj, Lrt, &["BokzyaDve"]);
    assert_has_iw(&[], &bhuj, Lrt, &["Bokzye"]);
    assert_has_vahi(&[], &bhuj, Lrt, &["BokzyAvahe"]);
    assert_has_mahin(&[], &bhuj, Lrt, &["BokzyAmahe"]);

    let man = d("ma\\na~\\", Divadi);
    assert_has_thaas(&[], &man, Lat, &["manyase"]);
    assert_has_aathaam(&[], &man, Lat, &["manyeTe"]);
    assert_has_dhvam(&[], &man, Lat, &["manyaDve"]);
    assert_has_ta(&[], &man, Lat, &["manyate"]);
}

#[test]
fn skip_sk_2171_to_sk_2182() {}

#[test]
fn sk_2183() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &bhu, Lit, &["baBUva"]);
    assert_has_tas(&[], &bhu, Lit, &["baBUvatuH"]);
    assert_has_jhi(&[], &bhu, Lit, &["baBUvuH"]);
}

#[test]
fn sk_2184() {
    let bhu = d("BU", Bhvadi);
    assert_has_sip(&[], &bhu, Lit, &["baBUviTa"]);
    assert_has_thas(&[], &bhu, Lit, &["baBUvaTuH"]);
    assert_has_tha(&[], &bhu, Lit, &["baBUva"]);
    assert_has_mip(&[], &bhu, Lit, &["baBUva"]);
    assert_has_vas(&[], &bhu, Lit, &["baBUviva"]);
    assert_has_mas(&[], &bhu, Lit, &["baBUvima"]);
}

#[test]
fn skip_sk_2185_to_sk_2189() {}

#[test]
fn sk_2190() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &bhu, Lut, &["BavitA"]);
}

#[test]
fn skip_sk_2191() {}

#[test]
fn sk_2192() {
    let bhu = d("BU", Bhvadi);
    assert_has_tas(&[], &bhu, Lut, &["BavitArO"]);
    assert_has_jhi(&[], &bhu, Lut, &["BavitAraH"]);
    assert_has_sip(&[], &bhu, Lut, &["BavitAsi"]);
    assert_has_thas(&[], &bhu, Lut, &["BavitAsTaH"]);
    assert_has_tha(&[], &bhu, Lut, &["BavitAsTa"]);
    assert_has_mip(&[], &bhu, Lut, &["BavitAsmi"]);
    assert_has_vas(&[], &bhu, Lut, &["BavitAsvaH"]);
    assert_has_mas(&[], &bhu, Lut, &["BavitAsmaH"]);
}

#[test]
fn sk_2193() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &bhu, Lrt, &["Bavizyati"]);
    assert_has_tas(&[], &bhu, Lrt, &["BavizyataH"]);
    assert_has_jhi(&[], &bhu, Lrt, &["Bavizyanti"]);
    assert_has_sip(&[], &bhu, Lrt, &["Bavizyasi"]);
    assert_has_thas(&[], &bhu, Lrt, &["BavizyaTaH"]);
    assert_has_tha(&[], &bhu, Lrt, &["BavizyaTa"]);
    assert_has_mip(&[], &bhu, Lrt, &["BavizyAmi"]);
    assert_has_vas(&[], &bhu, Lrt, &["BavizyAvaH"]);
    assert_has_mas(&[], &bhu, Lrt, &["BavizyAmaH"]);
}

#[test]
fn skip_sk_2194_to_sk_2195() {}

#[test]
fn sk_2196() {
    let bhu = d("BU", Bhvadi);
    // BavatAt is from SK 2197.
    assert_has_tip(&[], &bhu, Lot, &["Bavatu", "BavatAt"]);
}

#[test]
fn sk_2197() {
    let bhu = d("BU", Bhvadi);
    // Bavatu is from SK 2196.
    assert_has_tip(&[], &bhu, Lot, &["Bavatu", "BavatAt"]);
}

#[test]
fn skip_sk_2198_to_sk_2199() {}

#[test]
fn sk_2200() {
    let bhu = d("BU", Bhvadi);
    assert_has_tas(&[], &bhu, Lot, &["BavatAm"]);
    assert_has_jhi(&[], &bhu, Lot, &["Bavantu"]);
}

#[test]
fn skip_sk_2201() {}

#[test]
fn sk_2202() {
    let bhu = d("BU", Bhvadi);
    assert_has_sip(&[], &bhu, Lot, &["Bava", "BavatAt"]);
    assert_has_thas(&[], &bhu, Lot, &["Bavatam"]);
    assert_has_tha(&[], &bhu, Lot, &["Bavata"]);
}

#[test]
fn skip_sk_2203() {}

#[test]
fn sk_2204() {
    let bhu = d("BU", Bhvadi);
    assert_has_mip(&[], &bhu, Lot, &["BavAni"]);
    assert_has_vas(&[], &bhu, Lot, &["BavAva"]);
    assert_has_mas(&[], &bhu, Lot, &["BavAma"]);
}

#[test]
fn skip_sk_2205_to_sk_2206() {}

#[test]
fn sk_2207() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &bhu, Lan, &["aBavat"]);
    assert_has_tas(&[], &bhu, Lan, &["aBavatAm"]);
    assert_has_jhi(&[], &bhu, Lan, &["aBavan"]);
    assert_has_sip(&[], &bhu, Lan, &["aBavaH"]);
    assert_has_thas(&[], &bhu, Lan, &["aBavatam"]);
    assert_has_tha(&[], &bhu, Lan, &["aBavata"]);
    assert_has_mip(&[], &bhu, Lan, &["aBavam"]);
    assert_has_vas(&[], &bhu, Lan, &["aBavAva"]);
    assert_has_mas(&[], &bhu, Lan, &["aBavAma"]);
}

#[test]
fn skip_sk_2208_to_sk_2211() {}

#[test]
fn sk_2212() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &bhu, VidhiLin, &["Bavet"]);
    assert_has_tas(&[], &bhu, VidhiLin, &["BavetAm"]);
}

#[test]
fn skip_sk_2213() {}

#[test]
fn sk_2214() {
    let bhu = d("BU", Bhvadi);
    assert_has_jhi(&[], &bhu, VidhiLin, &["BaveyuH"]);
    assert_has_sip(&[], &bhu, VidhiLin, &["BaveH"]);
    assert_has_thas(&[], &bhu, VidhiLin, &["Bavetam"]);
    assert_has_tha(&[], &bhu, VidhiLin, &["Baveta"]);
    assert_has_mip(&[], &bhu, VidhiLin, &["Baveyam"]);
    assert_has_vas(&[], &bhu, VidhiLin, &["Baveva"]);
    assert_has_mas(&[], &bhu, VidhiLin, &["Bavema"]);
}

#[test]
fn skip_sk_2215_to_sk_2216() {}

#[test]
fn sk_2217() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &bhu, AshirLin, &["BUyAt"]);
    assert_has_tas(&[], &bhu, AshirLin, &["BUyAstAm"]);
    assert_has_jhi(&[], &bhu, AshirLin, &["BUyAsuH"]);
    assert_has_sip(&[], &bhu, AshirLin, &["BUyAH"]);
    assert_has_thas(&[], &bhu, AshirLin, &["BUyAstam"]);
    assert_has_tha(&[], &bhu, AshirLin, &["BUyAsta"]);
    assert_has_mip(&[], &bhu, AshirLin, &["BUyAsam"]);
    assert_has_vas(&[], &bhu, AshirLin, &["BUyAsva"]);
    assert_has_mas(&[], &bhu, AshirLin, &["BUyAsma"]);
}

#[test]
fn skip_sk_2218_to_sk_2224() {}

#[test]
fn sk_2225() {
    let bhu = d("BU", Bhvadi);
    let edh = d("eDa~\\", Bhvadi);

    assert_has_tip(&[], &bhu, Lun, &["aBUt"]);
    assert_has_iw(&[], &edh, Lun, &["EDizi"]);
    assert_has_ta(&[], &edh, Lun, &["EDizwa"]);
    assert_has_tas(&[], &bhu, Lun, &["aBUtAm"]);
}

#[test]
fn skip_sk_2226() {}

#[test]
fn sk_2227() {
    let bhu = d("BU", Bhvadi);
    assert_has_jhi(&[], &bhu, Lun, &["aBUvan"]);
    assert_has_sip(&[], &bhu, Lun, &["aBUH"]);
    assert_has_thas(&[], &bhu, Lun, &["aBUtam"]);
    assert_has_tha(&[], &bhu, Lun, &["aBUta"]);
    assert_has_mip(&[], &bhu, Lun, &["aBUvam"]);
    assert_has_vas(&[], &bhu, Lun, &["aBUva"]);
    assert_has_mas(&[], &bhu, Lun, &["aBUma"]);
}

#[test]
fn sk_2228() {
    // TODO: support mAN-yoga
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &bhu, Lun, &["aBUt"]);
    assert_has_tip(&[], &bhu, Lan, &["aBavat"]);
}

#[test]
fn sk_2229() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&[], &bhu, Lrn, &["aBavizyat"]);
    assert_has_tas(&[], &bhu, Lrn, &["aBavizyatAm"]);
    assert_has_jhi(&[], &bhu, Lrn, &["aBavizyan"]);
    assert_has_sip(&[], &bhu, Lrn, &["aBavizyaH"]);
    assert_has_thas(&[], &bhu, Lrn, &["aBavizyatam"]);
    assert_has_tha(&[], &bhu, Lrn, &["aBavizyata"]);
    assert_has_mip(&[], &bhu, Lrn, &["aBavizyam"]);
    assert_has_vas(&[], &bhu, Lrn, &["aBavizyAva"]);
    assert_has_mas(&[], &bhu, Lrn, &["aBavizyAma"]);
}

#[test]
fn skip_sk_2230() {}

#[test]
fn sk_2231() {
    let bhu = d("BU", Bhvadi);
    assert_has_mip(&["pra"], &bhu, Lot, &["praBavARi"]);
    assert_has_mip(&["dur"], &bhu, Lot, &["durBavAni"]);
    assert_has_mip(&["antar"], &bhu, Lot, &["antarBavAni"]);
}

#[test]
fn sk_2232() {
    let bhu = d("BU", Bhvadi);
    assert_has_tip(&["pra", "ni"], &bhu, Lat, &["praRiBavati", "praniBavati"]);
    assert_has_tip(&["pra"], &bhu, Lat, &["praBavati"]);
    assert_has_tip(&["parA"], &bhu, Lat, &["parABavati"]);
    assert_has_tip(&["sam"], &bhu, Lat, &["samBavati"]);
    assert_has_tip(&["anu"], &bhu, Lat, &["anuBavati"]);
    assert_has_tip(&["aBi"], &bhu, Lat, &["aBiBavati"]);
    assert_has_tip(&["ud"], &bhu, Lat, &["udBavati"]);
    assert_has_tip(&["pari"], &bhu, Lat, &["pariBavati"]);
}

#[test]
fn sk_2233() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_ta(&[], &edh, Lat, &["eDate"]);
}

#[test]
fn skip_sk_2234() {}

#[test]
fn sk_2235() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_aataam(&[], &edh, Lat, &["eDete"]);
    assert_has_jha(&[], &edh, Lat, &["eDante"]);
}

#[test]
fn sk_2236() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_thaas(&[], &edh, Lat, &["eDase"]);
    assert_has_aathaam(&[], &edh, Lat, &["eDeTe"]);
    assert_has_dhvam(&[], &edh, Lat, &["eDaDve"]);
    assert_has_iw(&[], &edh, Lat, &["eDe"]);
    assert_has_vahi(&[], &edh, Lat, &["eDAvahe"]);
    assert_has_mahin(&[], &edh, Lat, &["eDAmahe"]);
}

#[test]
fn skip_sk_2237_to_sk_2243() {}

#[test]
fn sk_2244() {
    let vrasc = d("o~vrascU~", Tudadi);
    assert_has_tip(&[], &vrasc, Lit, &["vavraSca"]);
}

#[test]
fn sk_2245() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_ta(&[], &edh, Lit, &["eDAYcakre", "eDAmbaBUva", "eDAmAsa"]);
    assert_has_aataam(
        &[],
        &edh,
        Lit,
        &["eDAYcakrAte", "eDAmbaBUvatuH", "eDAmAsatuH"],
    );
    assert_has_jha(&[], &edh, Lit, &["eDAYcakrire", "eDAmbaBUvuH", "eDAmAsuH"]);
}

#[test]
fn sk_2246() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_thaas(&[], &edh, Lit, &["eDAYcakfze", "eDAmbaBUviTa", "eDAmAsiTa"]);
    assert_has_aathaam(
        &[],
        &edh,
        Lit,
        &["eDAYcakrATe", "eDAmbaBUvaTuH", "eDAmAsaTuH"],
    );
}

#[test]
fn sk_2247() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_dhvam(&[], &edh, Lit, &["eDAYcakfQve", "eDAmbaBUva", "eDAmAsa"]);
    assert_has_iw(&[], &edh, Lit, &["eDAYcakre", "eDAmbaBUva", "eDAmAsa"]);
    assert_has_vahi(
        &[],
        &edh,
        Lit,
        &["eDAYcakfvahe", "eDAmbaBUviva", "eDAmAsiva"],
    );
    assert_has_mahin(
        &[],
        &edh,
        Lit,
        &["eDAYcakfmahe", "eDAmbaBUvima", "eDAmAsima"],
    );
}

#[test]
fn sk_2248() {
    let edh = d("eDa~\\", Bhvadi);

    // Only the "Asa/AtasuH" forms are mentioned here.
    assert_has_aataam(
        &[],
        &edh,
        Lit,
        &["eDAYcakrAte", "eDAmbaBUvatuH", "eDAmAsatuH"],
    );
    assert_has_jha(&[], &edh, Lit, &["eDAYcakrire", "eDAmbaBUvuH", "eDAmAsuH"]);

    assert_has_ta(&[], &edh, Lut, &["eDitA"]);
    assert_has_aataam(&[], &edh, Lut, &["eDitArO"]);
    assert_has_jha(&[], &edh, Lut, &["eDitAraH"]);
    assert_has_thaas(&[], &edh, Lut, &["eDitAse"]);
    assert_has_aathaam(&[], &edh, Lut, &["eDitAsATe"]);
}

#[test]
fn sk_2249() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_dhvam(&[], &edh, Lut, &["eDitADve"]);
}

#[test]
fn sk_2250() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_iw(&[], &edh, Lut, &["eDitAhe"]);
    assert_has_vahi(&[], &edh, Lut, &["eDitAsvahe"]);
    assert_has_mahin(&[], &edh, Lut, &["eDitAsmahe"]);

    assert_has_ta(&[], &edh, Lrt, &["eDizyate"]);
    assert_has_aataam(&[], &edh, Lrt, &["eDizyete"]);
    assert_has_jha(&[], &edh, Lrt, &["eDizyante"]);
    assert_has_thaas(&[], &edh, Lrt, &["eDizyase"]);
    assert_has_aathaam(&[], &edh, Lrt, &["eDizyeTe"]);
    assert_has_dhvam(&[], &edh, Lrt, &["eDizyaDve"]);
    assert_has_iw(&[], &edh, Lrt, &["eDizye"]);
    assert_has_vahi(&[], &edh, Lrt, &["eDizyAvahe"]);
    assert_has_mahin(&[], &edh, Lrt, &["eDizyAmahe"]);
}

#[test]
fn sk_2251() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_ta(&[], &edh, Lot, &["eDatAm"]);
    assert_has_aataam(&[], &edh, Lot, &["eDetAm"]);
    assert_has_jha(&[], &edh, Lot, &["eDantAm"]);
}

#[test]
fn sk_2252() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_thaas(&[], &edh, Lot, &["eDasva"]);
    assert_has_aathaam(&[], &edh, Lot, &["eDeTAm"]);
    assert_has_dhvam(&[], &edh, Lot, &["eDaDvam"]);
}

#[test]
fn sk_2253() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_iw(&[], &edh, Lot, &["eDE"]);
    assert_has_vahi(&[], &edh, Lot, &["eDAvahE"]);
    assert_has_mahin(&[], &edh, Lot, &["eDAmahE"]);
}

#[test]
fn sk_2254() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_ta(&[], &edh, Lan, &["EData"]);
    assert_has_aataam(&[], &edh, Lan, &["EDetAm"]);
    assert_has_jha(&[], &edh, Lan, &["EDanta"]);
    assert_has_thaas(&[], &edh, Lan, &["EDaTAH"]);
    assert_has_aathaam(&[], &edh, Lan, &["EDeTAm"]);
    assert_has_dhvam(&[], &edh, Lan, &["EDaDvam"]);
    assert_has_iw(&[], &edh, Lan, &["EDe"]);
    assert_has_vahi(&[], &edh, Lan, &["EDAvahi"]);
    assert_has_mahin(&[], &edh, Lan, &["EDAmahi"]);
}

#[test]
fn sk_2255() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_ta(&[], &edh, VidhiLin, &["eDeta"]);
    assert_has_aataam(&[], &edh, VidhiLin, &["eDeyAtAm"]);
}

#[test]
fn sk_2256() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_jha(&[], &edh, VidhiLin, &["eDeran"]);
    assert_has_thaas(&[], &edh, VidhiLin, &["eDeTAH"]);
    assert_has_aathaam(&[], &edh, VidhiLin, &["eDeyATAm"]);
    assert_has_dhvam(&[], &edh, VidhiLin, &["eDeDvam"]);
}

#[test]
fn sk_2257() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_iw(&[], &edh, VidhiLin, &["eDeya"]);
    assert_has_vahi(&[], &edh, VidhiLin, &["eDevahi"]);
    assert_has_mahin(&[], &edh, VidhiLin, &["eDemahi"]);

    assert_has_ta(&[], &edh, AshirLin, &["eDizIzwa"]);
    assert_has_aataam(&[], &edh, AshirLin, &["eDizIyAstAm"]);
    assert_has_jha(&[], &edh, AshirLin, &["eDizIran"]);
    assert_has_thaas(&[], &edh, AshirLin, &["eDizIzWAH"]);
    assert_has_aathaam(&[], &edh, AshirLin, &["eDizIyAsTAm"]);
    assert_has_dhvam(&[], &edh, AshirLin, &["eDizIDvam"]);
    assert_has_iw(&[], &edh, AshirLin, &["eDizIya"]);
    assert_has_vahi(&[], &edh, AshirLin, &["eDizIvahi"]);
    assert_has_mahin(&[], &edh, AshirLin, &["eDizImahi"]);

    assert_has_ta(&[], &edh, Lun, &["EDizwa"]);
    assert_has_aataam(&[], &edh, Lun, &["EDizAtAm"]);
}

#[test]
fn sk_2258() {
    let edh = d("eDa~\\", Bhvadi);
    assert_has_jha(&[], &edh, Lun, &["EDizata"]);
    assert_has_thaas(&[], &edh, Lun, &["EDizWAH"]);
    assert_has_aathaam(&[], &edh, Lun, &["EDizATAm"]);
    // TODO: EDiQvam
    assert_has_dhvam(&[], &edh, Lun, &["EDiDvam"]);
    assert_has_iw(&[], &edh, Lun, &["EDizi"]);
    assert_has_vahi(&[], &edh, Lun, &["EDizvahi"]);
    assert_has_mahin(&[], &edh, Lun, &["EDizmahi"]);

    assert_has_ta(&[], &edh, Lrn, &["EDizyata"]);
    assert_has_aataam(&[], &edh, Lrn, &["EDizyetAm"]);
    assert_has_jha(&[], &edh, Lrn, &["EDizyanta"]);
    assert_has_thaas(&[], &edh, Lrn, &["EDizyaTAH"]);
    assert_has_aathaam(&[], &edh, Lrn, &["EDizyeTAm"]);
    assert_has_dhvam(&[], &edh, Lrn, &["EDizyaDvam"]);
    assert_has_iw(&[], &edh, Lrn, &["EDizye"]);
    assert_has_vahi(&[], &edh, Lrn, &["EDizyAvahi"]);
    assert_has_mahin(&[], &edh, Lrn, &["EDizyAmahi"]);

    let spardh = d("sparDa~\\", Bhvadi);
    assert_has_ta(&[], &spardh, Lat, &["sparDate"]);
}

#[test]
fn sk_2259() {
    let spardh = d("sparDa~\\", Bhvadi);
    assert_has_ta(&[], &spardh, Lit, &["pasparDe"]);
    assert_has_ta(&[], &spardh, Lut, &["sparDitA"]);
    assert_has_ta(&[], &spardh, Lrt, &["sparDizyate"]);
    assert_has_ta(&[], &spardh, Lot, &["sparDatAm"]);
    assert_has_ta(&[], &spardh, Lan, &["asparData"]);
    assert_has_ta(&[], &spardh, VidhiLin, &["sparDeta"]);
    assert_has_ta(&[], &spardh, AshirLin, &["sparDizIzwa"]);
    assert_has_ta(&[], &spardh, Lun, &["asparDizwa"]);
    assert_has_ta(&[], &spardh, Lrn, &["asparDizyata"]);

    let gadh = d("gADf~\\", Bhvadi);
    assert_has_ta(&[], &gadh, Lat, &["gADate"]);
    assert_has_ta(&[], &gadh, Lit, &["jagADe"]);

    let badh = d("bADf~\\", Bhvadi);
    assert_has_ta(&[], &badh, Lat, &["bADate"]);

    let nath = d("nATf~\\", Bhvadi);
    assert_has_ta(&[], &nath, Lat, &["nATate"]);
    assert_has_tip(&[], &nath, Lat, &["nATati"]);

    let nadh = d("nADf~\\", Bhvadi);
    assert_has_ta(&[], &nadh, Lat, &["nADate"]);

    let dadh = d("daDa~\\", Bhvadi);
    assert_has_ta(&[], &dadh, Lat, &["daDate"]);
}

#[test]
fn skip_sk_2260() {}

#[test]
fn sk_2261() {
    let dadh = d("daDa~\\", Bhvadi);
    assert_has_ta(&[], &dadh, Lit, &["deDe"]);
    assert_has_aataam(&[], &dadh, Lit, &["deDAte"]);
    assert_has_jha(&[], &dadh, Lit, &["deDire"]);

    // "nananTa" is from SK 2399.
    assert_has_tas(&[], &d("divu~", Divadi), Lit, &["didivatuH"]);
    assert_has_ta(&[], &d("rAsf~\\", Bhvadi), Lit, &["rarAse"]);
    assert_has_tas(&[], &d("tsara~", Bhvadi), Lit, &["tatsaratuH"]);
    assert_has_tas(&[], &d("kaRa~", Bhvadi), Lit, &["cakaRatuH"]);
    assert_has_sip(&[], &d("Ra\\ma~", Bhvadi), Lit, &["nemiTa", "nananTa"]);
    assert_has_ta(&[], &d("zaha~\\", Bhvadi), Lit, &["sehe"]);
}

#[test]
fn sk_2262() {
    let skund = d("skudi~\\", Bhvadi);
    assert_has_ta(&[], &skund, Lat, &["skundate"]);
    assert_has_ta(&[], &skund, Lit, &["cuskunde"]);

    let shvind = d("Svidi~\\", Bhvadi);
    assert_has_ta(&[], &shvind, Lat, &["Svindate"]);
    assert_has_ta(&[], &shvind, Lit, &["SiSvinde"]);

    let vand = d("vadi~\\", Bhvadi);
    assert_has_ta(&[], &vand, Lat, &["vandate"]);
    assert_has_ta(&[], &vand, Lit, &["vavande"]);

    let bhand = d("Badi~\\", Bhvadi);
    assert_has_ta(&[], &bhand, Lat, &["Bandate"]);
    assert_has_ta(&[], &bhand, Lit, &["baBande"]);

    let mand = d("madi~\\", Bhvadi);
    assert_has_ta(&[], &mand, Lat, &["mandate"]);
    assert_has_ta(&[], &mand, Lit, &["mamande"]);

    let spand = d("spadi~\\", Bhvadi);
    assert_has_ta(&[], &spand, Lat, &["spandate"]);
    assert_has_ta(&[], &spand, Lit, &["paspande"]);

    let klind = d("klidi~\\", Bhvadi);
    assert_has_ta(&[], &klind, Lat, &["klindate"]);
    assert_has_ta(&[], &klind, Lit, &["ciklinde"]);

    let mud = d("muda~\\", Bhvadi);
    assert_has_ta(&[], &mud, Lat, &["modate"]);

    let dad = d("dada~\\", Bhvadi);
    assert_has_ta(&[], &dad, Lat, &["dadate"]);
}

#[test]
fn sk_2263() {
    let dad = d("dada~\\", Bhvadi);
    assert_has_ta(&[], &dad, Lit, &["dadade"]);
    assert_has_aataam(&[], &dad, Lit, &["dadadAte"]);
    assert_has_jha(&[], &dad, Lit, &["dadadire"]);
}

#[test]
fn sk_2264() {
    let svad = d("zvada~\\", Bhvadi);
    assert_has_ta(&["anu"], &svad, Lat, &["anusvadate"]);
    assert_has_ta(&[], &svad, Lit, &["sasvade"]);

    let svard = d("svarda~\\", Bhvadi);
    assert_has_ta(&[], &svard, Lat, &["svardate"]);
    assert_has_ta(&[], &svard, Lit, &["sasvarde"]);
}

#[test]
fn sk_2265() {
    let urd = d("urda~\\", Bhvadi);
    assert_has_ta(&[], &urd, Lat, &["Urdate"]);
    assert_has_ta(&[], &urd, Lit, &["UrdAYcakre", "UrdAmbaBUva", "UrdAmAsa"]);

    let kurd = d("kurda~\\", Bhvadi);
    assert_has_ta(&[], &kurd, Lat, &["kUrdate"]);
    assert_has_ta(&[], &kurd, Lit, &["cukUrde"]);

    let gurd = d("gurda~\\", Bhvadi);
    assert_has_ta(&[], &gurd, Lat, &["gUrdate"]);

    let gud = d("guda~\\", Bhvadi);
    assert_has_ta(&[], &gud, Lat, &["godate"]);
    assert_has_ta(&[], &gud, Lit, &["jugude"]);

    let sud = d("zUda~\\", Bhvadi);
    assert_has_ta(&[], &sud, Lat, &["sUdate"]);
    assert_has_ta(&[], &sud, Lit, &["suzUde"]);

    let hrad = d("hrAda~\\", Bhvadi);
    assert_has_ta(&[], &hrad, Lat, &["hrAdate"]);
    assert_has_ta(&[], &hrad, Lit, &["jahrAde"]);

    let hlad = d("hlAdI~\\", Bhvadi);
    assert_has_ta(&[], &hlad, Lat, &["hlAdate"]);

    let svad = d("svAda~\\", Bhvadi);
    assert_has_ta(&[], &svad, Lat, &["svAdate"]);

    let pard = d("parda~\\", Bhvadi);
    assert_has_ta(&[], &pard, Lat, &["pardate"]);

    let yat = d("yatI~\\", Bhvadi);
    assert_has_ta(&[], &yat, Lat, &["yatate"]);
    assert_has_ta(&[], &yat, Lit, &["yete"]);

    let yut = d("yutf~\\", Bhvadi);
    assert_has_ta(&[], &yut, Lat, &["yotate"]);
    assert_has_ta(&[], &yut, Lit, &["yuyute"]);

    let jut = d("jutf~\\", Bhvadi);
    assert_has_ta(&[], &jut, Lat, &["jotate"]);
    assert_has_ta(&[], &jut, Lit, &["jujute"]);

    let vith = d("viTf~\\", Bhvadi);
    assert_has_ta(&[], &vith, Lit, &["viviTe"]);

    let veth = d("veTf~\\", Bhvadi);
    assert_has_ta(&[], &veth, Lit, &["viveTe"]);

    let shranth = d("SraTi~\\", Bhvadi);
    assert_has_ta(&[], &shranth, Lat, &["SranTate"]);

    let granth = d("graTi~\\", Bhvadi);
    assert_has_ta(&[], &granth, Lat, &["granTate"]);

    let katth = d("katTa~\\", Bhvadi);
    assert_has_ta(&[], &katth, Lat, &["katTate"]);

    let at = d("ata~", Bhvadi);
    assert_has_tip(&[], &at, Lat, &["atati"]);
    assert_has_tip(&[], &at, Lit, &["Ata"]);
    assert_has_tas(&[], &at, Lit, &["AtatuH"]);
    assert_has_jhi(&[], &at, Lit, &["AtuH"]);
}

#[test]
fn sk_2266() {
    let at = d("ata~", Bhvadi);
    assert_has_tip(&[], &at, Lun, &["AtIt"]);
    assert_has_tas(&[], &at, Lun, &["AtizwAm"]);
    assert_has_jhi(&[], &at, Lun, &["AtizuH"]);
}

#[test]
fn skip_sk_2267() {}

#[test]
fn sk_2268() {
    let at = d("ata~", Bhvadi);
    // TODO: mAN-yoga
    assert_has_tip(&[], &at, Lun, &["AtIt"]);
    assert_has_tas(&[], &at, Lun, &["AtizwAm"]);
    assert_has_jhi(&[], &at, Lun, &["AtizuH"]);

    let cit = d("citI~", Bhvadi);
    assert_has_tip(&[], &cit, Lat, &["cetati"]);
    assert_has_tip(&[], &cit, Lit, &["ciceta"]);
    assert_has_tip(&[], &cit, Lun, &["acetIt"]);
    assert_has_tas(&[], &cit, Lun, &["acetizwAm"]);
    assert_has_jhi(&[], &cit, Lun, &["acetizuH"]);

    let cyut = d("cyuti~r", Bhvadi);
    assert_has_tip(&[], &cyut, Lat, &["cyotati"]);
    assert_has_tip(&[], &cyut, Lit, &["cucyota"]);
}

#[test]
fn sk_2269() {
    let cyut = d("cyuti~r", Bhvadi);
    assert_has_tip(&[], &cyut, Lun, &["acyutat", "acyotIt"]);

    let shcyut = d("Scyuti~r", Bhvadi);
    assert_has_tip(&[], &shcyut, Lat, &["Scyotati"]);
    assert_has_tip(&[], &shcyut, Lit, &["cuScyota"]);
    assert_has_tip(&[], &shcyut, Lun, &["aScyutat", "aScyotIt"]);

    let shcut = d("Scuti~r", Bhvadi);
    assert_has_tip(&[], &shcut, Lat, &["Scotati"]);

    let manth = d("manTa~", Bhvadi);
    assert_has_tip(&[], &manth, AshirLin, &["maTyAt"]);

    let mathi = d("maTi~", Bhvadi);
    assert_has_tip(&[], &mathi, AshirLin, &["manTyAt"]);

    let kuthi = d("kuTi~", Bhvadi);
    assert_has_tip(&[], &kuthi, AshirLin, &["kunTyAt"]);

    let sidh = d("ziDa~", Bhvadi);
    assert_has_tip(&[], &sidh, Lat, &["seDati"]);
    assert_has_tip(&[], &sidh, Lit, &["sizeDa"]);
    assert_has_tip(&[], &sidh, Lut, &["seDitA"]);
    assert_has_tip(&[], &sidh, Lun, &["aseDIt"]);
}

#[test]
fn skip_sk_2270_to_sk_2275() {}

#[test]
fn sk_2276() {
    let sidh = d("ziDa~", Bhvadi);
    assert_has_tip(&["ni"], &sidh, Lan, &["nyazeDat"]);
    assert_has_tip(&["ni"], &sidh, Lun, &["nyazeDIt"]);
    assert_has_tip(&["ni"], &sidh, Lrn, &["nyazeDizyat"]);
}

#[test]
fn sk_2277() {
    let sidh = d("ziDa~", Bhvadi);
    assert_has_tip(&["ni"], &sidh, Lit, &["nizizeDa"]);
    assert_has_tas(&["ni"], &sidh, Lit, &["niziziDatuH"]);
}

#[test]
fn sk_2278() {
    let sidh = d("ziDa~", Bhvadi);
    assert_has_tip(&["vi"], &sidh, Lat, &["viseDati"]);
}

#[test]
fn skip_sk_2279() {}

#[test]
fn sk_2280() {
    let sidh = d("ziDU~", Bhvadi);
    assert_has_sip(&[], &sidh, Lit, &["sizedDa", "sizeDiTa"]);
    assert_has_tip(&[], &sidh, Lut, &["sedDA", "seDitA"]);
    assert_has_tip(&[], &sidh, Lrt, &["setsyati", "seDizyati"]);
    // aseDIt is from SK 2281.
    assert_has_tip(&[], &sidh, Lun, &["asEtsIt", "aseDIt"]);
}

#[test]
fn sk_2281() {
    let sidh = d("ziDU~", Bhvadi);
    assert_has_tas(&[], &sidh, Lun, &["asEdDAm", "aseDizwAm"]);
    assert_has_jhi(&[], &sidh, Lun, &["asEtsuH", "aseDizuH"]);
    assert_has_sip(&[], &sidh, Lun, &["asEtsIH", "aseDIH"]);
    assert_has_thas(&[], &sidh, Lun, &["asEdDam", "aseDizwam"]);
    assert_has_tha(&[], &sidh, Lun, &["asEdDa", "aseDizwa"]);
    assert_has_mip(&[], &sidh, Lun, &["asEtsam", "aseDizam"]);
    assert_has_vas(&[], &sidh, Lun, &["asEtsva", "aseDizva"]);
    assert_has_mas(&[], &sidh, Lun, &["asEtsma", "aseDizma"]);

    let khaad = d("KAdf~", Bhvadi);
    assert_has_tip(&[], &khaad, Lat, &["KAdati"]);
    assert_has_tip(&[], &khaad, Lit, &["caKAda"]);

    let khad = d("Kada~", Bhvadi);
    assert_has_tip(&[], &khad, Lat, &["Kadati"]);
}

#[test]
fn sk_2282() {
    let khad = d("Kada~", Bhvadi);
    assert_has_tip(&[], &khad, Lit, &["caKAda"]);
}

#[test]
fn sk_2283() {
    let khad = d("Kada~", Bhvadi);
    assert_has_mip(&[], &khad, Lit, &["caKAda", "caKada"]);
}

#[test]
fn sk_2284() {
    let khad = d("Kada~", Bhvadi);
    assert_has_tip(&[], &khad, Lun, &["aKAdIt", "aKadIt"]);

    let bad = d("bada~", Bhvadi);
    assert_has_tip(&[], &bad, Lat, &["badati"]);
    assert_has_tip(&[], &bad, Lit, &["babAda"]);
    assert_has_tas(&[], &bad, Lit, &["bedatuH"]);
    assert_has_sip(&[], &bad, Lit, &["bediTa"]);
    assert_has_mip(&[], &bad, Lit, &["babAda", "babada"]);
    assert_has_tip(&[], &bad, Lun, &["abAdIt", "abadIt"]);

    let gad = d("gada~", Bhvadi);
    assert_has_tip(&[], &gad, Lat, &["gadati"]);
}

#[test]
fn sk_2285() {
    let gad = d("gada~", Bhvadi);
    assert_has_tip(&["pra", "ni"], &gad, Lat, &["praRigadati"]);
    assert_has_tip(&[], &gad, Lit, &["jagAda"]);

    let rad = d("rada~", Bhvadi);
    assert_has_tip(&[], &rad, Lit, &["rarAda"]);
    assert_has_tas(&[], &rad, Lit, &["redatuH"]);
}

#[test]
fn skip_sk_2286() {}

#[test]
fn sk_2287() {
    let nad = d("Rada~", Bhvadi);
    assert_has_tip(&["pra"], &nad, Lat, &["praRadati"]);
    assert_has_tip(&["pra", "ni"], &nad, Lat, &["praRinadati"]);
}

#[test]
fn sk_2288() {
    let ard = d("arda~", Bhvadi);
    assert_has_tip(&[], &ard, Lit, &["Anarda"]);
    assert_has_tip(&[], &ard, Lun, &["ArdIt"]);

    let nard = d("narda~", Bhvadi);
    assert_has_tip(&["pra"], &nard, Lat, &["pranardati"]);

    let gard = d("garda~", Bhvadi);
    assert_has_tip(&[], &gard, Lat, &["gardati"]);
    assert_has_tip(&[], &gard, Lit, &["jagarda"]);

    let tard = d("tarda~", Bhvadi);
    assert_has_tip(&[], &tard, Lat, &["tardati"]);

    let kard = d("karda~", Bhvadi);
    assert_has_tip(&[], &kard, Lat, &["kardati"]);

    let khard = d("Karda~", Bhvadi);
    assert_has_tip(&[], &khard, Lat, &["Kardati"]);
    assert_has_tip(&[], &khard, Lit, &["caKarda"]);

    let ant = d("ati~", Bhvadi);
    assert_has_tip(&[], &ant, Lat, &["antati"]);
    assert_has_tip(&[], &ant, Lit, &["Ananta"]);

    let and = d("adi~", Bhvadi);
    assert_has_tip(&[], &and, Lat, &["andati"]);
    assert_has_tip(&[], &and, Lit, &["Ananda"]);

    let ind = d("idi~", Bhvadi);
    assert_has_tip(&[], &ind, Lat, &["indati"]);
    assert_has_tip(&[], &ind, Lit, &["indAYcakAra", "indAmbaBUva", "indAmAsa"]);

    let bind = d("bidi~", Bhvadi);
    assert_has_tip(&[], &bind, Lat, &["bindati"]);

    let gand = d("gaqi~", Bhvadi);
    assert_has_tip(&[], &gand, Lat, &["gaRqati"]);

    let nind = d("Ridi~", Bhvadi);
    assert_has_tip(&[], &nind, Lat, &["nindati"]);
    assert_has_tip(&["pra"], &nind, Lat, &["praRindati"]);
}

#[test]
fn sk_2289() {
    let nand = d("wunadi~", Bhvadi);
    assert_has_tip(&[], &nand, Lat, &["nandati"]);
    assert_has_tip(&[], &nand, AshirLin, &["nandyAt"]);

    let cand = d("cadi~", Bhvadi);
    assert_has_tip(&[], &cand, Lit, &["cacanda"]);

    let trand = d("tradi~", Bhvadi);
    assert_has_tip(&[], &trand, Lit, &["tatranda"]);

    let kand = d("kadi~", Bhvadi);
    assert_has_tip(&[], &kand, Lit, &["cakanda"]);

    let krand = d("kradi~", Bhvadi);
    assert_has_tip(&[], &krand, Lit, &["cakranda"]);

    let kland = d("kladi~", Bhvadi);
    assert_has_tip(&[], &kland, Lit, &["caklanda"]);

    let klind = d("klidi~", Bhvadi);
    assert_has_tip(&[], &klind, Lit, &["ciklinda"]);

    let shund = d("SunDa~", Bhvadi);
    assert_has_tip(&[], &shund, Lit, &["SuSunDa"]);
    assert_has_tip(&[], &shund, AshirLin, &["SuDyAt"]);

    let shik = d("SIkf~\\", Bhvadi);
    assert_has_ta(&[], &shik, Lat, &["SIkate"]);
    assert_has_ta(&[], &shik, Lit, &["SiSIke"]);

    let lok = d("lokf~\\", Bhvadi);
    assert_has_ta(&[], &lok, Lat, &["lokate"]);
    assert_has_ta(&[], &lok, Lit, &["luloke"]);

    let shlok = d("Slokf~\\", Bhvadi);
    assert_has_ta(&[], &shlok, Lat, &["Slokate"]);

    let drek = d("drekf~\\", Bhvadi);
    assert_has_ta(&[], &drek, Lit, &["didreke"]);

    let dhrek = d("Drekf~\\", Bhvadi);
    assert_has_ta(&[], &dhrek, Lit, &["diDreke"]);

    let rek = d("rekf~\\", Bhvadi);
    assert_has_ta(&[], &rek, Lat, &["rekate"]);

    let sek = d("sekf~\\", Bhvadi);
    assert_has_ta(&[], &sek, Lit, &["siseke"]);

    let shak = d("Saki~\\", Bhvadi);
    assert_has_ta(&[], &shak, Lat, &["SaNkate"]);
    assert_has_ta(&[], &shak, Lit, &["SaSaNke"]);

    let ak = d("aki~\\", Bhvadi);
    assert_has_ta(&[], &ak, Lat, &["aNkate"]);
    assert_has_ta(&[], &ak, Lit, &["AnaNke"]);

    let vank = d("vaki~\\", Bhvadi);
    assert_has_ta(&[], &vank, Lat, &["vaNkate"]);

    let mank = d("maki~\\", Bhvadi);
    assert_has_ta(&[], &mank, Lat, &["maNkate"]);

    let kak = d("kaka~\\", Bhvadi);
    assert_has_ta(&[], &kak, Lat, &["kakate"]);
    assert_has_ta(&[], &kak, Lit, &["cakake"]);

    let kuk = d("kuka~\\", Bhvadi);
    assert_has_ta(&[], &kuk, Lat, &["kokate"]);
    assert_has_ta(&[], &kuk, Lit, &["cukuke"]);

    let vfk = d("vfka~\\", Bhvadi);
    assert_has_ta(&[], &vfk, Lat, &["varkate"]);
    assert_has_ta(&[], &vfk, Lit, &["vavfke"]);

    let cak = d("caka~\\", Bhvadi);
    assert_has_ta(&[], &cak, Lat, &["cakate"]);
    assert_has_ta(&[], &cak, Lit, &["ceke"]);

    let kank = d("kaki~\\", Bhvadi);
    assert_has_ta(&[], &kank, Lat, &["kaNkate"]);

    let dhauk = d("QOkf~\\", Bhvadi);
    assert_has_ta(&[], &dhauk, Lit, &["quQOke"]);

    let trauk = d("trOkf~\\", Bhvadi);
    assert_has_ta(&[], &trauk, Lit, &["tutrOke"]);

    let shvashk = d("zvazka~\\", Bhvadi);
    assert_has_ta(&[], &shvashk, Lat, &["zvazkate"]);
    assert_has_ta(&[], &shvashk, Lit, &["zazvazke"]);

    let angh = d("aGi~\\", Bhvadi);
    assert_has_ta(&[], &angh, Lat, &["aNGate"]);
    assert_has_ta(&[], &angh, Lit, &["AnaNGe"]);

    let vangh = d("vaGi~\\", Bhvadi);
    assert_has_ta(&[], &vangh, Lat, &["vaNGate"]);

    let mangh = d("maGi~\\", Bhvadi);
    assert_has_ta(&[], &mangh, Lat, &["maNGate"]);

    let ragh = d("rAGf~\\", Bhvadi);
    assert_has_ta(&[], &ragh, Lat, &["rAGate"]);

    let lagh = d("lAGf~\\", Bhvadi);
    assert_has_ta(&[], &lagh, Lat, &["lAGate"]);

    let dragh = d("drAGf~\\", Bhvadi);
    assert_has_ta(&[], &dragh, Lat, &["drAGate"]);

    let shlagh = d("SlAGf~\\", Bhvadi);
    assert_has_ta(&[], &shlagh, Lat, &["SlAGate"]);

    let phakk = d("Pakka~", Bhvadi);
    assert_has_tip(&[], &phakk, Lat, &["Pakkati"]);
    assert_has_tip(&[], &phakk, Lit, &["paPakka"]);

    let tak = d("taka~", Bhvadi);
    assert_has_tip(&[], &tak, Lat, &["takati"]);

    let tank = d("taki~", Bhvadi);
    assert_has_tip(&[], &tank, Lat, &["taNkati"]);

    let bukk = d("bukka~", Bhvadi);
    assert_has_tip(&[], &bukk, Lat, &["bukkati"]);

    let kakh = d("kaKa~", Bhvadi);
    assert_has_tip(&["pra", "ni"], &kakh, Lat, &["pranikaKati"]);

    let okh = d("oKf~", Bhvadi);
    assert_has_tip(&[], &okh, Lat, &["oKati"]);
    assert_has_tip(&[], &okh, Lit, &["oKAYcakAra", "oKAmbaBUva", "oKAmAsa"]);

    let shakh = d("SAKf~", Bhvadi);
    assert_has_tip(&[], &shakh, Lat, &["SAKati"]);
}

#[test]
fn sk_2290() {
    let ukh = d("uKa~", Bhvadi);
    assert_has_tip(&[], &ukh, Lit, &["uvoKa"]);
    assert_has_tas(&[], &ukh, Lit, &["UKatuH"]);
    assert_has_jhi(&[], &ukh, Lit, &["UKuH"]);

    let unkh = d("uKi~", Bhvadi);
    assert_has_tip(&[], &unkh, Lat, &["uNKati"]);

    let vakh = d("vaKa~", Bhvadi);
    assert_has_tas(&[], &vakh, Lit, &["vavaKatuH"]);

    let vankh = d("vaKi~", Bhvadi);
    assert_has_tip(&[], &vankh, Lat, &["vaNKati"]);

    let makh = d("maKa~", Bhvadi);
    assert_has_tas(&[], &makh, Lit, &["meKatuH"]);

    let yung = d("yugi~", Bhvadi);
    assert_has_tip(&[], &yung, Lat, &["yuNgati"]);

    let ghagh = d("GaGa~", Bhvadi);
    assert_has_tip(&[], &ghagh, Lat, &["GaGati"]);
    assert_has_tip(&[], &ghagh, Lit, &["jaGAGa"]);

    let mangh = d("maGi~", Bhvadi);
    assert_has_tip(&[], &mangh, Lat, &["maNGati"]);

    let shingh = d("SiGi~", Bhvadi);
    assert_has_tip(&[], &shingh, Lat, &["SiNGati"]);

    let varc = d("varca~\\", Bhvadi);
    assert_has_ta(&[], &varc, Lat, &["varcate"]);

    let sac = d("zaca~\\", Bhvadi);
    assert_has_ta(&[], &sac, Lat, &["sacate"]);
    assert_has_ta(&[], &sac, Lit, &["sece"]);
    assert_has_ta(&[], &sac, Lut, &["sacitA"]);

    let loc = d("locf~\\", Bhvadi);
    assert_has_ta(&[], &loc, Lat, &["locate"]);
    assert_has_ta(&[], &loc, Lit, &["luloce"]);

    let shac = d("Saca~\\", Bhvadi);
    assert_has_ta(&[], &shac, Lit, &["Sece"]);

    let shvac = d("Svaca~\\", Bhvadi);
    assert_has_ta(&[], &shvac, Lat, &["Svacate"]);

    let shvanc = d("Svaci~\\", Bhvadi);
    assert_has_ta(&[], &shvanc, Lat, &["SvaYcate"]);

    let kac = d("kaca~\\", Bhvadi);
    assert_has_ta(&[], &kac, Lat, &["kacate"]);

    let kanc = d("kaci~\\", Bhvadi);
    assert_has_ta(&[], &kanc, Lit, &["cakaYce"]);

    let kanc = d("kAci~\\", Bhvadi);
    assert_has_ta(&[], &kanc, Lit, &["cakAYce"]);

    let mac = d("maca~\\", Bhvadi);
    assert_has_ta(&[], &mac, Lit, &["mece"]);

    let munc = d("muci~\\", Bhvadi);
    assert_has_ta(&[], &munc, Lit, &["mumuYce"]);

    let manc = d("maci~\\", Bhvadi);
    assert_has_ta(&[], &manc, Lit, &["mamaYce"]);

    let panc = d("paci~\\", Bhvadi);
    assert_has_ta(&[], &panc, Lat, &["paYcate"]);

    let stuc = d("zwuca~\\", Bhvadi);
    assert_has_ta(&[], &stuc, Lat, &["stocate"]);
    assert_has_ta(&[], &stuc, Lit, &["tuzwuce"]);

    let fj = d("fja~\\", Bhvadi);
    assert_has_ta(&[], &fj, Lat, &["arjate"]);
    assert_has_ta(&[], &fj, Lit, &["Anfje"]);

    let fnj = d("fji~\\", Bhvadi);
    assert_has_ta(&[], &fnj, Lat, &["fYjate"]);
    assert_has_ta(&["pra"], &fnj, Lat, &["prArYjate"]);
    assert_has_ta(&[], &fnj, Lit, &["fYjAYcakre", "fYjAmbaBUva", "fYjAmAsa"]);
    assert_has_ta(&[], &fnj, Lun, &["ArYjizwa"]);

    // TODO: SK has baBfjje, which seems like a typo.
    let bhrnj = d("BfjI~\\", Bhvadi);
    assert_has_ta(&[], &bhrnj, Lat, &["Barjate"]);
    assert_has_ta(&[], &bhrnj, Lit, &["baBfje"]);
    assert_has_ta(&[], &bhrnj, Lun, &["aBarjizwa"]);

    let ej = d("ejf~\\", Bhvadi);
    assert_has_ta(&[], &ej, Lit, &["ejAYcakre", "ejAmbaBUva", "ejAmAsa"]);

    let ij = d("Ija~\\", Bhvadi);
    assert_has_ta(&[], &ij, Lit, &["IjAYcakre", "IjAmbaBUva", "IjAmAsa"]);

    let shuc = d("Suca~", Bhvadi);
    assert_has_tip(&[], &shuc, Lat, &["Socati"]);

    let kuc = d("kuca~", Bhvadi);
    assert_has_tip(&[], &kuc, Lat, &["kocati"]);

    let kunc = d("kunca~", Bhvadi);
    assert_has_tip(&[], &kunc, AshirLin, &["kucyAt"]);

    let krunc = d("krunca~", Bhvadi);
    assert_has_tip(&[], &krunc, AshirLin, &["krucyAt"]);

    let lunc = d("lunca~", Bhvadi);
    assert_has_tip(&[], &lunc, AshirLin, &["lucyAt"]);

    let anc = d("ancu~", Bhvadi);
    assert_has_tip(&[], &anc, AshirLin, &["acyAt", "aYcyAt"]);

    let vanc = d("vancu~", Bhvadi);
    assert_has_tip(&[], &vanc, AshirLin, &["vacyAt"]);

    let canc = d("cancu~", Bhvadi);
    assert_has_tip(&[], &canc, AshirLin, &["cacyAt"]);

    let tanc = d("tancu~", Bhvadi);
    assert_has_tip(&[], &tanc, AshirLin, &["tacyAt"]);

    let tvanc = d("tvancu~", Bhvadi);
    assert_has_tip(&[], &tvanc, AshirLin, &["tvacyAt"]);

    let mrunc = d("mruncu~", Bhvadi);
    assert_has_tip(&[], &mrunc, AshirLin, &["mrucyAt"]);
    assert_has_tip(&[], &mrunc, Lun, &["amruYcIt"]);

    let mlunc = d("mluncu~", Bhvadi);
    assert_has_tip(&[], &mlunc, Lun, &["amluYcIt"]);
}

#[test]
fn sk_2291() {
    let mruc = d("mrucu~", Bhvadi);
    assert_has_tip(&[], &mruc, Lun, &["amrucat", "amrocIt"]);

    let mluc = d("mlucu~", Bhvadi);
    assert_has_tip(&[], &mluc, Lun, &["amlucat", "amlocIt"]);

    // TODO: SK has agrocat?
    let gruc = d("grucu~", Bhvadi);
    assert_has_tip(&[], &gruc, Lit, &["jugroca"]);
    assert_has_tip(&[], &gruc, Lun, &["agrucat", "agrocIt"]);

    let gluc = d("glucu~", Bhvadi);
    assert_has_tip(&[], &gluc, Lit, &["jugloca"]);
    assert_has_tip(&[], &gluc, Lun, &["aglucat", "aglocIt"]);

    let kuj = d("kuju~", Bhvadi);
    assert_has_tip(&[], &kuj, Lun, &["akojIt"]);

    let khuj = d("Kuju~", Bhvadi);
    assert_has_tip(&[], &khuj, Lun, &["aKojIt"]);

    let glunc = d("gluncu~", Bhvadi);
    assert_has_tip(&[], &glunc, Lun, &["aglucat", "agluYcIt"]);

    let sasj = d("zasja~", Bhvadi);
    assert_has_tip(&[], &sasj, Lat, &["sajjati"]);
    assert_has_ta(&[], &sasj, Lat, &["sajjate"]);

    let gunj = d("guji~", Bhvadi);
    assert_has_tip(&[], &gunj, Lat, &["guYjati"]);
    assert_has_tip(&[], &gunj, AshirLin, &["guYjyAt"]);

    let arc = d("arca~", Bhvadi);
    assert_has_tip(&[], &arc, Lit, &["Anarca"]);

    let mlech = d("mleCa~", Bhvadi);
    assert_has_tip(&[], &mlech, Lat, &["mlecCati"]);
    assert_has_tip(&[], &mlech, Lit, &["mimlecCa"]);

    let lach = d("laCa~", Bhvadi);
    assert_has_tip(&[], &lach, Lit, &["lalacCa"]);

    let lanch = d("lACi~", Bhvadi);
    assert_has_tip(&[], &lanch, Lit, &["lalAYCa"]);

    let vanch = d("vACi~", Bhvadi);
    assert_has_tip(&[], &vanch, Lat, &["vAYCati"]);

    let anch = d("ACi~", Bhvadi);
    assert_has_tip(&[], &anch, Lat, &["AYCati"]);
    assert_has_tip(&[], &anch, Lit, &["AYCa", "AnAYCa"]);

    let hrich = d("hrICa~", Bhvadi);
    assert_has_tip(&[], &hrich, Lit, &["jihrIcCa"]);

    let hurch = d("hurCA~", Bhvadi);
    assert_has_tip(&[], &hurch, Lat, &["hUrCati"]);

    let murch = d("murCA~", Bhvadi);
    assert_has_tip(&[], &murch, Lat, &["mUrCati"]);

    let sphurch = d("sPurCA~", Bhvadi);
    assert_has_tip(&[], &sphurch, Lat, &["sPUrCati"]);

    let yuch = d("yuCa~", Bhvadi);
    assert_has_tip(&[], &yuch, Lat, &["yucCati"]);

    let unch = d("uCi~", Bhvadi);
    assert_has_tip(&[], &unch, Lat, &["uYCati"]);
    assert_has_tip(&[], &unch, Lit, &["uYCAYcakAra", "uYCAmbaBUva", "uYCAmAsa"]);

    let uch = d("uCI~", Bhvadi);
    assert_has_tip(&["vi"], &uch, Lat, &["vyucCati"]);

    let dhraj = d("Draja~", Bhvadi);
    assert_has_tip(&[], &dhraj, Lat, &["Drajati"]);

    let dhranj = d("Draji~", Bhvadi);
    assert_has_tip(&[], &dhranj, Lat, &["DraYjati"]);

    let dhfj = d("Dfja~", Bhvadi);
    assert_has_tip(&[], &dhfj, Lat, &["Darjati"]);

    let dhrnj = d("Dfji~", Bhvadi);
    assert_has_tip(&[], &dhrnj, Lat, &["DfYjati"]);

    let dhvaj = d("Dvaja~", Bhvadi);
    assert_has_tip(&[], &dhvaj, Lat, &["Dvajati"]);

    let dhvanj = d("Dvaji~", Bhvadi);
    assert_has_tip(&[], &dhvanj, Lat, &["DvaYjati"]);

    let kuj = d("kUja~", Bhvadi);
    assert_has_tip(&[], &kuj, Lit, &["cukUja"]);

    let arj = d("arja~", Bhvadi);
    assert_has_tip(&[], &arj, Lat, &["arjati"]);
    assert_has_tip(&[], &arj, Lit, &["Anarja"]);

    let sarj = d("zarja~", Bhvadi);
    assert_has_tip(&[], &sarj, Lat, &["sarjati"]);
    assert_has_tip(&[], &sarj, Lit, &["sasarja"]);

    let garj = d("garja~", Bhvadi);
    assert_has_tip(&[], &garj, Lat, &["garjati"]);

    let tarj = d("tarja~", Bhvadi);
    assert_has_tip(&[], &tarj, Lat, &["tarjati"]);

    let karj = d("karja~", Bhvadi);
    assert_has_tip(&[], &karj, Lit, &["cakarja"]);

    let kharj = d("Karja~", Bhvadi);
    assert_has_tip(&[], &kharj, Lit, &["caKarja"]);

    let aj = d("aja~", Bhvadi);
    assert_has_tip(&[], &aj, Lat, &["ajati"]);
}

#[test]
fn sk_2292() {
    let aj = d("aja~", Bhvadi);
    assert_has_tip(&[], &aj, Lit, &["vivAya"]);
    assert_has_tas(&[], &aj, Lit, &["vivyatuH"]);
    assert_has_jhi(&[], &aj, Lit, &["vivyuH"]);
}

#[test]
fn skip_sk_2293_to_sk_2295() {}

#[test]
fn sk_2296() {
    // Ajiva and Ajima are justified.
    let aj = d("aja~", Bhvadi);
    assert_has_sip(&[], &aj, Lit, &["vivayiTa", "viveTa", "AjiTa"]);
    assert_has_thas(&[], &aj, Lit, &["vivyaTuH"]);
    assert_has_tha(&[], &aj, Lit, &["vivya"]);
    assert_has_mip(&[], &aj, Lit, &["vivAya", "vivaya"]);
    assert_has_vas(&[], &aj, Lit, &["vivyiva", "Ajiva"]);
    assert_has_mas(&[], &aj, Lit, &["vivyima", "Ajima"]);

    assert_has_tip(&[], &aj, Lut, &["vetA", "ajitA"]);
    assert_has_tip(&[], &aj, Lrt, &["vezyati", "ajizyati"]);
    assert_has_tip(&[], &aj, Lot, &["ajatu", "ajatAt"]);
    assert_has_tip(&[], &aj, Lan, &["Ajat"]);
    assert_has_tip(&[], &aj, VidhiLin, &["ajet"]);
    assert_has_tip(&[], &aj, AshirLin, &["vIyAt"]);
}

#[test]
fn sk_2297() {
    let aj = d("aja~", Bhvadi);
    assert_has_tip(&[], &aj, Lun, &["avEzIt", "AjIt"]);
    assert_has_tip(&[], &aj, Lrn, &["avezyat", "Ajizyat"]);

    let tej = d("teja~", Bhvadi);
    assert_has_tip(&[], &tej, Lat, &["tejati"]);

    let khaj = d("Kaja~", Bhvadi);
    assert_has_tip(&[], &khaj, Lat, &["Kajati"]);

    let khanj = d("Kaji~", Bhvadi);
    assert_has_tip(&[], &khanj, Lat, &["KaYjati"]);

    let ej = d("ejf~", Bhvadi);
    assert_has_tip(&[], &ej, Lit, &["ejAYcakAra", "ejAmbaBUva", "ejAmAsa"]);

    let sphurj = d("wuo~sPUrjA~", Bhvadi);
    assert_has_tip(&[], &sphurj, Lat, &["sPUrjati"]);
    assert_has_tip(&[], &sphurj, Lit, &["pusPUrja"]);

    let kzi = d("kzi\\", Bhvadi);
    assert_has_tip(&[], &kzi, Lat, &["kzayati"]);
    assert_has_tip(&[], &kzi, Lit, &["cikzAya"]);
    assert_has_tas(&[], &kzi, Lit, &["cikziyatuH"]);
    assert_has_jhi(&[], &kzi, Lit, &["cikziyuH"]);
    assert_has_sip(&[], &kzi, Lit, &["cikzayiTa", "cikzeTa"]);
    assert_has_vas(&[], &kzi, Lit, &["cikziyiva"]);
    assert_has_mas(&[], &kzi, Lit, &["cikziyima"]);
    assert_has_tip(&[], &kzi, Lut, &["kzetA"]);
}

#[test]
fn sk_2298() {
    let kshi = d("kzi\\", Bhvadi);
    assert_has_tip(&[], &kshi, AshirLin, &["kzIyAt"]);
    assert_has_tip(&[], &kshi, Lun, &["akzEzIt"]);

    let kshij = d("kzIja~", Bhvadi);
    assert_has_tip(&[], &kshij, Lit, &["cikzIja"]);

    let tuj = d("tuja~", Bhvadi);
    assert_has_tip(&[], &tuj, Lat, &["tojati"]);
    assert_has_tip(&[], &tuj, Lit, &["tutoja"]);

    let vraj = d("vraja~", Bhvadi);
    assert_has_tip(&[], &vraj, Lun, &["avrAjIt"]);

    let att = d("awwa~\\", Bhvadi);
    assert_has_ta(&[], &att, Lat, &["awwate"]);
    assert_has_ta(&[], &att, Lit, &["Anawwe"]);

    // TODO: SK has vivezwa?
    let vesht = d("vezwa~\\", Bhvadi);
    assert_has_ta(&[], &vesht, Lit, &["vivezwe"]);

    let cesht = d("cezwa~\\", Bhvadi);
    assert_has_ta(&[], &cesht, Lun, &["acezwizwa"]);

    let gosht = d("gozwa~\\", Bhvadi);
    assert_has_ta(&[], &gosht, Lit, &["jugozwe"]);

    let losht = d("lozwa~\\", Bhvadi);
    assert_has_ta(&[], &losht, Lit, &["lulozwe"]);

    let ghatt = d("Gawwa~\\", Bhvadi);
    assert_has_ta(&[], &ghatt, Lit, &["jaGawwe"]);

    let at = d("aWi~\\", Bhvadi);
    assert_has_ta(&[], &at, Lat, &["aRWate"]);
    assert_has_ta(&[], &at, Lit, &["AnaRWe"]);

    let vanth = d("vaWi~\\", Bhvadi);
    assert_has_ta(&[], &vanth, Lit, &["vavaRWe"]);

    let manth = d("maWi~\\", Bhvadi);
    assert_has_ta(&[], &manth, Lat, &["maRWate"]);

    let kanth = d("kaWi~\\", Bhvadi);
    assert_has_ta(&[], &kanth, Lat, &["kaRWate"]);

    let munth = d("muWi~\\", Bhvadi);
    assert_has_ta(&[], &munth, Lat, &["muRWate"]);

    let heth = d("heWa~\\", Bhvadi);
    assert_has_ta(&[], &heth, Lit, &["jiheWe"]);

    let eth = d("eWa~\\", Bhvadi);
    assert_has_ta(&[], &eth, Lit, &["eWAYcakre", "eWAmbaBUva", "eWAmAsa"]);

    let hind = d("hiqi~\\", Bhvadi);
    assert_has_ta(&[], &hind, Lat, &["hiRqate"]);
    assert_has_ta(&[], &hind, Lit, &["jihiRqe"]);

    let hund = d("huqi~\\", Bhvadi);
    assert_has_ta(&[], &hund, Lit, &["juhuRqe"]);

    let kund = d("kuqi~\\", Bhvadi);
    assert_has_ta(&[], &kund, Lit, &["cukuRqe"]);

    let vand = d("vaqi~\\", Bhvadi);
    assert_has_ta(&[], &vand, Lit, &["vavaRqe"]);

    let bhand = d("Baqi~\\", Bhvadi);
    assert_has_ta(&[], &bhand, Lit, &["baBaRqe"]);

    let pind = d("piqi~\\", Bhvadi);
    assert_has_ta(&[], &pind, Lit, &["pipiRqe"]);

    let mund = d("muqi~\\", Bhvadi);
    assert_has_ta(&[], &mund, Lat, &["muRqate"]);

    let tund = d("tuqi~\\", Bhvadi);
    assert_has_ta(&[], &tund, Lat, &["tuRqate"]);

    let hund = d("huqi~\\", Bhvadi);
    assert_has_ta(&[], &hund, Lat, &["huRqate"]);

    let cand = d("caqi~\\", Bhvadi);
    assert_has_ta(&[], &cand, Lat, &["caRqate"]);

    let shand = d("Saqi~\\", Bhvadi);
    assert_has_ta(&[], &shand, Lat, &["SaRqate"]);

    let tand = d("taqi~\\", Bhvadi);
    assert_has_ta(&[], &tand, Lat, &["taRqate"]);

    let pand = d("paqi~\\", Bhvadi);
    assert_has_ta(&[], &pand, Lat, &["paRqate"]);

    let kand = d("kaqi~\\", Bhvadi);
    assert_has_ta(&[], &kand, Lat, &["kaRqate"]);

    let hed = d("heqf~\\", Bhvadi);
    assert_has_ta(&[], &hed, Lit, &["jiheqe"]);

    let hod = d("hoqf~\\", Bhvadi);
    assert_has_ta(&[], &hod, Lit, &["juhoqe"]);

    let bad = d("bAqf~\\", Bhvadi);
    assert_has_ta(&[], &bad, Lat, &["bAqate"]);

    let drad = d("drAqf~\\", Bhvadi);
    assert_has_ta(&[], &drad, Lat, &["drAqate"]);

    let dhrad = d("DrAqf~\\", Bhvadi);
    assert_has_ta(&[], &dhrad, Lat, &["DrAqate"]);

    let shad = d("SAqf~\\", Bhvadi);
    assert_has_ta(&[], &shad, Lat, &["SAqate"]);

    let shaut = d("SOwf~", Bhvadi);
    assert_has_tip(&[], &shaut, Lat, &["SOwati"]);
    assert_has_tip(&[], &shaut, Lit, &["SuSOwa"]);

    let yaut = d("yOwf~", Bhvadi);
    assert_has_tip(&[], &yaut, Lat, &["yOwati"]);

    // TODO: SK has mleqati?
    let mlet = d("mlewf~", Bhvadi);
    assert_has_tip(&[], &mlet, Lat, &["mlewati"]);

    let mred = d("mreqf~", Bhvadi);
    assert_has_tip(&[], &mred, Lat, &["mreqati"]);

    let kat = d("kawe~", Bhvadi);
    assert_has_tip(&[], &kat, Lit, &["cakAwa"]);
}

#[test]
fn sk_2299() {
    let kat = d("kawe~", Bhvadi);
    assert_has_tip(&[], &kat, Lun, &["akawIt"]);

    let at = d("awa~", Bhvadi);
    assert_has_tip(&[], &at, Lit, &["Awa"]);
    assert_has_tas(&[], &at, Lit, &["AwatuH"]);
    assert_has_jhi(&[], &at, Lit, &["AwuH"]);

    let pat = d("pawa~", Bhvadi);
    assert_has_tip(&[], &pat, Lit, &["papAwa"]);
    assert_has_tas(&[], &pat, Lit, &["pewatuH"]);
    assert_has_jhi(&[], &pat, Lit, &["pewuH"]);

    let rat = d("rawa~", Bhvadi);
    assert_has_tip(&[], &rat, Lit, &["rarAwa"]);

    let lat = d("lawa~", Bhvadi);
    assert_has_tip(&[], &lat, Lit, &["lalAwa"]);

    let shat = d("Sawa~", Bhvadi);
    assert_has_tip(&[], &shat, Lit, &["SaSAwa"]);

    let vat = d("vawa~", Bhvadi);
    assert_has_tip(&[], &vat, Lit, &["vavAwa"]);
    assert_has_tas(&[], &vat, Lit, &["vavawatuH"]);
    assert_has_jhi(&[], &vat, Lit, &["vavawuH"]);
    assert_has_sip(&[], &vat, Lit, &["vavawiTa"]);

    let kit = d("kiwa~", Bhvadi);
    assert_has_tip(&[], &kit, Lat, &["kewati"]);

    let khit = d("Kiwa~", Bhvadi);
    assert_has_tip(&[], &khit, Lat, &["Kewati"]);

    let shit = d("Siwa~", Bhvadi);
    assert_has_tip(&[], &shit, Lat, &["Sewati"]);
    assert_has_tip(&[], &shit, Lit, &["SiSewa"]);

    let sit = d("ziwa~", Bhvadi);
    assert_has_tip(&[], &sit, Lat, &["sewati"]);
    assert_has_tip(&[], &sit, Lit, &["sizewa"]);

    let it = d("iwa~", Bhvadi);
    assert_has_tip(&[], &it, Lat, &["ewati"]);

    let kit = d("kiwa~", Bhvadi);
    assert_has_tip(&[], &kit, Lat, &["kewati"]);

    let kat = d("kawI~", Bhvadi);
    assert_has_tip(&[], &kat, Lat, &["kawati"]);

    // TODO: why are these here? I don't follow.
    let i = d("i\\", Bhvadi);
    assert_has_tip(&[], &i, Lat, &["ayati"]);
    assert_has_tip(&[], &i, Lit, &["iyAya"]);
    assert_has_tas(&[], &i, Lit, &["iyatuH"]);
    assert_has_jhi(&[], &i, Lit, &["iyuH"]);
    assert_has_sip(&[], &i, Lit, &["iyayiTa", "iyeTa"]);
    assert_has_mip(&[], &i, Lit, &["iyAya", "iyaya"]);

    let ii = d("I\\", Bhvadi);
    assert_has_tip(&[], &ii, Lit, &["ayAYcakAra", "ayAmbaBUva", "ayAmAsa"]);

    let kund = d("kuqi~", Bhvadi);
    assert_has_tip(&[], &kund, Lat, &["kuRqati"]);

    let mund = d("muqi~", Bhvadi);
    assert_has_tip(&[], &mund, Lat, &["muRqati"]);

    let pund = d("puqi~", Bhvadi);
    assert_has_tip(&[], &pund, Lat, &["puRqati"]);

    let runt = d("ruwi~", Bhvadi);
    assert_has_tip(&[], &runt, Lat, &["ruRwati"]);

    let lunt = d("luwi~", Bhvadi);
    assert_has_tip(&[], &lunt, Lat, &["luRwati"]);

    let sphutir = d("sPuwi~r", Bhvadi);
    assert_has_tip(&[], &sphutir, Lun, &["asPuwat", "asPowIt"]);

    let sphuti = d("sPuwi~", Bhvadi);
    assert_has_tip(&[], &sphuti, Lat, &["sPuRwati"]);

    let path = d("paWa~", Bhvadi);
    assert_has_tas(&[], &path, Lit, &["peWatuH"]);
    assert_has_sip(&[], &path, Lit, &["peWiTa"]);
    assert_has_tip(&[], &path, Lun, &["apaWIt", "apAWIt"]);

    let vath = d("vaWa~", Bhvadi);
    assert_has_tas(&[], &vath, Lit, &["vavaWatuH"]);
    assert_has_sip(&[], &vath, Lit, &["vavaWiTa"]);

    let hath = d("haWa~", Bhvadi);
    assert_has_tip(&[], &hath, Lat, &["haWati"]);
    assert_has_tip(&[], &hath, Lit, &["jahAWa"]);

    let uth = d("uWa~", Bhvadi);
    assert_has_tip(&[], &uth, Lat, &["oWati"]);

    let uuth = d("UWa~", Bhvadi);
    assert_has_tip(&[], &uuth, Lat, &["UWati"]);
    assert_has_tip(&[], &uuth, Lit, &["UWAYcakAra", "UWAmbaBUva", "UWAmAsa"]);

    let shuth = d("SuWa~", Bhvadi);
    assert_has_tip(&[], &shuth, Lat, &["SoWati"]);

    // TODO: how is this justified? SK doesn't mention SuWi.
    let shunth = d("SuWi~", Bhvadi);
    assert_has_tip(&[], &shunth, Lat, &["SuRWati"]);

    let kunth = d("kuWi~", Bhvadi);
    assert_has_tip(&[], &kunth, Lat, &["kuRWati"]);

    let cudd = d("cuqqa~", Bhvadi);
    assert_has_tip(&[], &cudd, Lat, &["cuqqati"]);
    assert_has_tip(&[], &cudd, Lit, &["cucuqqa"]);

    let add = d("aqqa~", Bhvadi);
    assert_has_tip(&[], &add, Lat, &["aqqati"]);
    assert_has_tip(&[], &add, Lit, &["Anaqqa"]);

    let kadd = d("kaqqa~", Bhvadi);
    assert_has_tip(&[], &kadd, Lat, &["kaqqati"]);

    let krid = d("krIqf~", Bhvadi);
    assert_has_tip(&[], &krid, Lit, &["cikrIqa"]);

    let tud = d("tuqf~", Bhvadi);
    assert_has_tip(&[], &tud, Lat, &["toqati"]);
    assert_has_tip(&[], &tud, Lit, &["tutoqa"]);

    let hud = d("huqf~", Bhvadi);
    assert_has_tip(&[], &hud, AshirLin, &["huqyAt"]);

    let huud = d("hUqf~", Bhvadi);
    assert_has_tip(&[], &huud, AshirLin, &["hUqyAt"]);

    let hod = d("hoqf~", Bhvadi);
    assert_has_tip(&[], &hod, AshirLin, &["hoqyAt"]);

    let ad = d("aqa~", Bhvadi);
    assert_has_tip(&[], &ad, Lat, &["aqati"]);
    assert_has_tip(&[], &ad, Lit, &["Aqa"]);
    assert_has_tas(&[], &ad, Lit, &["AqatuH"]);
    assert_has_jhi(&[], &ad, Lit, &["AquH"]);

    let lad = d("laqa~", Bhvadi);
    assert_has_tip(&[], &lad, Lat, &["laqati"]);

    let kad = d("kaqa~", Bhvadi);
    assert_has_tip(&[], &kad, Lat, &["kaqati"]);

    let kand = d("kaqi~", Bhvadi);
    assert_has_tip(&[], &kand, Lat, &["kaRqati"]);

    let gand = d("gaqi~", Bhvadi);
    assert_has_tip(&[], &gand, Lat, &["gaRqati"]);

    let tip = d("ti\\pf~\\", Bhvadi);
    assert_has_ta(&[], &tip, Lat, &["tepate"]);
    assert_has_ta(&[], &tip, Lit, &["titipe"]);
    assert_has_thaas(&[], &tip, Lit, &["titipize"]);
    assert_has_ta(&[], &tip, Lut, &["teptA"]);
    assert_has_ta(&[], &tip, Lrt, &["tepsyate"]);
}

#[test]
fn sk_2300() {
    let tip = d("ti\\pf~\\", Bhvadi);
    assert_has_ta(&[], &tip, AshirLin, &["tipsIzwa"]);
    assert_has_aataam(&[], &tip, AshirLin, &["tipsIyAstAm"]);
    assert_has_jha(&[], &tip, AshirLin, &["tipsIran"]);
    assert_has_ta(&[], &tip, Lun, &["atipta"]);
    assert_has_aataam(&[], &tip, Lun, &["atipsAtAm"]);
    assert_has_jha(&[], &tip, Lun, &["atipsata"]);

    let tep = d("tepf~\\", Bhvadi);
    assert_has_ta(&[], &tep, Lat, &["tepate"]);
    assert_has_ta(&[], &tep, Lit, &["titepe"]);

    let stip = d("zwipf~\\", Bhvadi);
    assert_has_ta(&[], &stip, Lit, &["tizwipe"]);
    assert_has_aataam(&[], &stip, Lit, &["tizwipAte"]);
    assert_has_jha(&[], &stip, Lit, &["tizwipire"]);

    let step = d("zwepf~\\", Bhvadi);
    assert_has_ta(&[], &step, Lit, &["tizwepe"]);
    assert_has_aataam(&[], &step, Lit, &["tizwepAte"]);
    assert_has_jha(&[], &step, Lit, &["tizwepire"]);

    let glep = d("glepf~\\", Bhvadi);
    assert_has_ta(&[], &glep, Lat, &["glepate"]);

    let vep = d("wuvepf~\\", Bhvadi);
    assert_has_ta(&[], &vep, Lat, &["vepate"]);

    let trap = d("trapU~\\z", Bhvadi);
    assert_has_ta(&[], &trap, Lat, &["trapate"]);
}

#[test]
fn sk_2301() {
    let trap = d("trapU~\\z", Bhvadi);
    assert_has_ta(&[], &trap, Lit, &["trepe"]);
    assert_has_aataam(&[], &trap, Lit, &["trepAte"]);
    assert_has_jha(&[], &trap, Lit, &["trepire"]);

    assert_has_ta(&[], &trap, Lut, &["trapitA", "traptA"]);
    assert_has_ta(&[], &trap, AshirLin, &["trapizIzwa", "trapsIzwa"]);

    let kamp = d("kapi~\\", Bhvadi);
    assert_has_ta(&[], &kamp, Lat, &["kampate"]);
    assert_has_ta(&[], &kamp, Lit, &["cakampe"]);

    let ramb = d("rabi~\\", Bhvadi);
    assert_has_ta(&[], &ramb, Lit, &["rarambe"]);

    let lamb = d("labi~\\", Bhvadi);
    assert_has_ta(&[], &lamb, Lit, &["lalambe"]);

    let amb = d("abi~\\", Bhvadi);
    assert_has_ta(&[], &amb, Lit, &["Anambe"]);

    let kab = d("kabf~\\", Bhvadi);
    assert_has_ta(&[], &kab, Lit, &["cakabe"]);

    let klib = d("klIbf~\\", Bhvadi);
    assert_has_ta(&[], &klib, Lit, &["ciklIbe"]);

    let kshib = d("kzIbf~\\", Bhvadi);
    assert_has_ta(&[], &kshib, Lat, &["kzIbate"]);

    let shib = d("SIBf~\\", Bhvadi);
    assert_has_ta(&[], &shib, Lat, &["SIBate"]);

    let rebh = d("reBf~\\", Bhvadi);
    assert_has_ta(&[], &rebh, Lit, &["rireBe"]);

    let ambh = d("aBi~\\", Bhvadi);
    assert_has_ta(&[], &ambh, Lat, &["amBate"]);

    let rambh = d("raBi~\\", Bhvadi);
    assert_has_ta(&[], &rambh, Lat, &["ramBate"]);

    let stambh = d("zwaBi~\\", Bhvadi);
    assert_has_ta(&[], &stambh, Lat, &["stamBate"]);
    // utTtamBate is justified. See comments on 8.4.61 for why "T" is used.
    assert_has_ta(&["ud"], &stambh, Lat, &["uttamBate", "utTtamBate"]);
    assert_has_ta(&["vi"], &stambh, Lat, &["vistamBate"]);

    // TODO: zwamBate, wazwamBe
}

#[test]
fn sk_2302() {
    // TODO: SK has jamBizwa?
    let jambh = d("jaBI~\\", Bhvadi);
    assert_has_ta(&[], &jambh, Lat, &["jamBate"]);
    assert_has_ta(&[], &jambh, Lit, &["jajamBe"]);
    assert_has_ta(&[], &jambh, Lut, &["jamBitA"]);
    assert_has_ta(&[], &jambh, Lun, &["ajamBizwa"]);

    let jrmbh = d("jfBi~\\", Bhvadi);
    assert_has_ta(&[], &jrmbh, Lat, &["jfmBate"]);
    assert_has_ta(&[], &jrmbh, Lit, &["jajfmBe"]);

    let shalbh = d("SalBa~\\", Bhvadi);
    assert_has_ta(&[], &shalbh, Lit, &["SaSalBe"]);

    let valbh = d("valBa~\\", Bhvadi);
    assert_has_ta(&[], &valbh, Lit, &["vavalBe"]);

    let galbh = d("galBa~\\", Bhvadi);
    assert_has_ta(&[], &galbh, Lat, &["galBate"]);

    let shranb = d("SranBu~\\", Bhvadi);
    assert_has_ta(&[], &shranb, Lat, &["SramBate"]);

    let sranbh = d("sranBu~\\", Bhvadi);
    assert_has_ta(&[], &sranbh, Lat, &["sramBate"]);

    let stubh = d("zwuBu~\\", Bhvadi);
    assert_has_ta(&[], &stubh, Lat, &["stoBate"]);
    assert_has_ta(&["vi"], &stubh, Lat, &["vizwoBate"]);
    assert_has_ta(&[], &stubh, Lit, &["tuzwuBe"]);
    assert_has_ta(&["vi"], &stubh, Lun, &["vyazwoBizwa"]);
}

#[test]
fn skip_sk_2303() {}

#[test]
fn sk_2304() {
    let gup = d("gupU~", Bhvadi);
    assert_has_tip(&[], &gup, Lat, &["gopAyati"]);
}

#[test]
fn skip_sk_2305_to_sk_2307() {}

#[test]
fn sk_2308() {
    let gupu = d("gupU~", Bhvadi);
    assert_has_tip(
        &[],
        &gupu,
        Lit,
        &["jugopa", "gopAyAYcakAra", "gopAyAmbaBUva", "gopAyAmAsa"],
    );
    assert_has_tas(
        &[],
        &gupu,
        Lit,
        &[
            "jugupatuH",
            "gopAyAYcakratuH",
            "gopAyAmbaBUvatuH",
            "gopAyAmAsatuH",
        ],
    );
    assert_has_sip(
        &[],
        &gupu,
        Lit,
        &[
            "jugopiTa",
            "jugopTa",
            "gopAyAYcakarTa",
            "gopAyAmbaBUviTa",
            "gopAyAmAsiTa",
        ],
    );
    assert_has_tip(&[], &gupu, Lut, &["gopAyitA", "gopitA", "goptA"]);
    assert_has_tip(&[], &gupu, AshirLin, &["gopAyyAt", "gupyAt"]);
    assert_has_tip(&[], &gupu, Lun, &["agopAyIt", "agopIt", "agOpsIt"]);

    let dhup = d("DUpa~", Bhvadi);
    assert_has_tip(&[], &dhup, Lat, &["DUpAyati"]);
    assert_has_tip(
        &[],
        &dhup,
        Lit,
        &["duDUpa", "DUpAyAYcakAra", "DUpAyAmbaBUva", "DUpAyAmAsa"],
    );
    assert_has_sip(&[], &dhup, Lut, &["DUpAyitAsi", "DUpitAsi"]);

    let sap = d("zapa~", Bhvadi);
    assert_has_tip(&[], &sap, Lat, &["sapati"]);

    let cup = d("cupa~", Bhvadi);
    assert_has_tip(&[], &cup, Lat, &["copati"]);
    assert_has_tip(&[], &cup, Lit, &["cucopa"]);
    assert_has_tip(&[], &cup, Lut, &["copitA"]);

    let tup = d("tupa~", Bhvadi);
    assert_has_tip(&[], &tup, Lat, &["topati"]);
    assert_has_tip(&[], &tup, Lit, &["tutopa"]);

    let tump = d("tunpa~", Bhvadi);
    assert_has_tip(&[], &tump, Lat, &["tumpati"]);
    assert_has_tip(&[], &tump, Lit, &["tutumpa"]);
    assert_has_tas(&[], &tump, Lit, &["tutumpatuH"]);
    assert_has_tip(&[], &tump, AshirLin, &["tupyAt"]);
    assert_has_tip(&["pra"], &tump, Lat, &["prastumpati", "pratumpati"]);
    // Forms other than pratostumpIti are justified by normal yan-luk rules.
    assert_has_tip(
        &["pra"],
        &yan_luk(&tump),
        Lat,
        &[
            "pratostumpIti",
            "pratostumpti",
            "pratotumpIti",
            "pratotumpti",
        ],
    );

    let trup = d("trupa~", Bhvadi);
    assert_has_tip(&[], &trup, Lat, &["tropati"]);

    let trump = d("trunpa~", Bhvadi);
    assert_has_tip(&[], &trump, Lat, &["trumpati"]);

    let tuph = d("tuPa~", Bhvadi);
    assert_has_tip(&[], &tuph, Lat, &["toPati"]);

    let tumph = d("tunPa~", Bhvadi);
    assert_has_tip(&[], &tumph, Lat, &["tumPati"]);

    let truph = d("truPa~", Bhvadi);
    assert_has_tip(&[], &truph, Lat, &["troPati"]);

    let trumph = d("trunPa~", Bhvadi);
    assert_has_tip(&[], &trumph, Lat, &["trumPati"]);

    let parp = d("parpa~", Bhvadi);
    assert_has_tip(&[], &parp, Lat, &["parpati"]);
    assert_has_tip(&[], &parp, Lit, &["paparpa"]);

    let raph = d("raPa~", Bhvadi);
    assert_has_tip(&[], &raph, Lat, &["raPati"]);

    let ramph = d("raPi~", Bhvadi);
    assert_has_tip(&[], &ramph, Lat, &["ramPati"]);

    let arb = d("arba~", Bhvadi);
    assert_has_tip(&[], &arb, Lat, &["arbati"]);
    assert_has_tip(&[], &arb, Lit, &["Anarba"]);

    let parb = d("parba~", Bhvadi);
    assert_has_tip(&[], &parb, Lat, &["parbati"]);

    let larb = d("larba~", Bhvadi);
    assert_has_tip(&[], &larb, Lat, &["larbati"]);

    let barb = d("barba~", Bhvadi);
    assert_has_tip(&[], &barb, Lat, &["barbati"]);

    let marb = d("marba~", Bhvadi);
    assert_has_tip(&[], &marb, Lat, &["marbati"]);

    let karb = d("karba~", Bhvadi);
    assert_has_tip(&[], &karb, Lat, &["karbati"]);

    let kharb = d("Karba~", Bhvadi);
    assert_has_tip(&[], &kharb, Lat, &["Karbati"]);

    let garb = d("garba~", Bhvadi);
    assert_has_tip(&[], &garb, Lat, &["garbati"]);

    let sharb = d("Sarba~", Bhvadi);
    assert_has_tip(&[], &sharb, Lat, &["Sarbati"]);

    let sarb = d("zarba~", Bhvadi);
    assert_has_tip(&[], &sarb, Lat, &["sarbati"]);

    let carb = d("carba~", Bhvadi);
    assert_has_tip(&[], &carb, Lat, &["carbati"]);

    let kumb = d("kubi~", Bhvadi);
    assert_has_tip(&[], &kumb, Lat, &["kumbati"]);

    let lumb = d("lubi~", Bhvadi);
    assert_has_tip(&[], &lumb, Lat, &["lumbati"]);

    let tumb = d("tubi~", Bhvadi);
    assert_has_tip(&[], &tumb, Lat, &["tumbati"]);

    let cumb = d("cubi~", Bhvadi);
    assert_has_tip(&[], &cumb, Lat, &["cumbati"]);

    let srbh = d("zfBu~", Bhvadi);
    assert_has_tip(&[], &srbh, Lat, &["sarBati"]);
    assert_has_tip(&[], &srbh, Lit, &["sasarBa"]);
    assert_has_tip(&[], &srbh, Lut, &["sarBitA"]);

    let srmbh = d("zfnBu~", Bhvadi);
    assert_has_tip(&[], &srmbh, Lat, &["sfmBati"]);
    assert_has_tip(&[], &srmbh, Lit, &["sasfmBa"]);
    assert_has_tip(&[], &srmbh, AshirLin, &["sfByAt"]);

    let sibh = d("ziBu~", Bhvadi);
    assert_has_tip(&[], &sibh, Lat, &["seBati"]);

    let simbh = d("zinBu~", Bhvadi);
    assert_has_tip(&[], &simbh, Lat, &["simBati"]);

    let ghinn = d("GiRi~\\", Bhvadi);
    assert_has_ta(&[], &ghinn, Lat, &["GiRRate"]);
    assert_has_ta(&[], &ghinn, Lit, &["jiGiRRe"]);

    let ghunn = d("GuRi~\\", Bhvadi);
    assert_has_ta(&[], &ghunn, Lat, &["GuRRate"]);
    assert_has_ta(&[], &ghunn, Lit, &["juGuRRe"]);

    let ghrnn = d("GfRi~\\", Bhvadi);
    assert_has_ta(&[], &ghrnn, Lat, &["GfRRate"]);
    assert_has_ta(&[], &ghrnn, Lit, &["jaGfRRe"]);

    let ghun = d("GuRa~\\", Bhvadi);
    assert_has_ta(&[], &ghun, Lat, &["GoRate"]);

    let ghurn = d("GurRa~\\", Bhvadi);
    assert_has_ta(&[], &ghurn, Lat, &["GUrRate"]);

    let pan = d("paRa~\\", Bhvadi);
    assert_has_ta(&[], &pan, Lat, &["paRate"]);
    assert_has_ta(&[], &pan, Lit, &["peRe"]);
    assert_has_tip(&[], &pan, Lat, &["paRAyati"]);
    assert_has_tip(
        &[],
        &pan,
        Lit,
        &["paRAyAYcakAra", "paRAyAmbaBUva", "paRAyAmAsa"],
    );

    assert_has_tip(&[], &pan, Lat, &["paRAyati"]);
    assert_has_tip(
        &[],
        &pan,
        Lit,
        &["paRAyAYcakAra", "paRAyAmbaBUva", "paRAyAmAsa"],
    );
    assert_has_ta(&[], &pan, Lit, &["peRe"]);
    assert_has_sip(&[], &pan, Lut, &["paRAyitAsi"]);
    assert_has_thaas(&[], &pan, Lut, &["paRitAse"]);
    assert_has_tip(&[], &pan, AshirLin, &["paRAyyAt"]);

    let pan2 = d("pana~\\", Bhvadi);
    assert_has_tip(&[], &pan2, Lat, &["panAyati"]);
    assert_has_tip(
        &[],
        &pan2,
        Lit,
        &["panAyAYcakAra", "panAyAmbaBUva", "panAyAmAsa"],
    );

    let bham = d("BAma~\\", Bhvadi);
    assert_has_ta(&[], &bham, Lat, &["BAmate"]);

    let ksham = d("kzamU~\\z", Bhvadi);
    assert_has_ta(&[], &ksham, Lat, &["kzamate"]);
    assert_has_ta(&[], &ksham, Lit, &["cakzame"]);
    assert_has_thaas(&[], &ksham, Lit, &["cakzamize", "cakzaMse"]);
    assert_has_dhvam(&[], &ksham, Lit, &["cakzamiDve", "cakzanDve"]);
    // cakzaRvahe is from SK 2309.
    assert_has_vahi(&[], &ksham, Lit, &["cakzamivahe", "cakzaRvahe"]);
}

#[test]
fn sk_2309() {
    // "cakzamivane" is from SK 2308.
    let ksham = d("kzamU~\\z", Bhvadi);
    assert_has_vahi(&[], &ksham, Lit, &["cakzamivahe", "cakzaRvahe"]);
    assert_has_mahin(&[], &ksham, Lit, &["cakzamimahe", "cakzaRmahe"]);
    assert_has_ta(&[], &ksham, Lrt, &["kzamizyate", "kzaMsyate"]);
    assert_has_ta(&[], &ksham, VidhiLin, &["kzameta"]);
    assert_has_ta(&[], &ksham, AshirLin, &["kzamizIzwa", "kzaMsIzwa"]);
    assert_has_ta(&[], &ksham, Lun, &["akzamizwa", "akzaMsta"]);
}

#[test]
fn sk_2310() {
    let kam = d("kamu~\\", Bhvadi);
    assert_has_ta(&[], &kam, Lat, &["kAmayate"]);
}

#[test]
fn sk_2311() {
    let kam = d("kamu~\\", Bhvadi);
    assert_has_ta(
        &[],
        &kam,
        Lit,
        &["cakame", "kAmayAYcakre", "kAmayAmbaBUva", "kAmayAmAsa"],
    );
    assert_has_ta(&[], &kam, Lut, &["kAmayitA", "kamitA"]);
    assert_has_ta(&[], &kam, Lrt, &["kAmayizyate", "kamizyate"]);
}

#[test]
fn skip_sk_2312() {}

#[test]
fn sk_2313() {
    let taksh = d("takzU~", Bhvadi);
    assert_has_tip(&[], &nic(&taksh), Lun, &["atatakzat"]);

    let at = d("awa~", Bhvadi);
    assert_has_tip(&[], &nic(&at), Lun, &["Awiwat"]);

    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &nic(&kf), Krt::Rvul, &["kAraka"]);
    assert_has_ta_k(&[], &nic(&kf), Lat, &["kAryate"]);
}

#[test]
fn skip_sk_2314_to_sk_2317() {}

#[test]
fn sk_2318() {
    let an = d("aRa~", Bhvadi);
    assert_has_tip(&[], &an, Lat, &["aRati"]);

    let ran = d("raRa~", Bhvadi);
    assert_has_tip(&[], &ran, Lat, &["raRati"]);

    let van = d("vaRa~", Bhvadi);
    assert_has_tip(&[], &van, Lat, &["vaRati"]);
    assert_has_tas(&[], &van, Lit, &["vavaRatuH"]);
    assert_has_sip(&[], &van, Lit, &["vavaRiTa"]);

    let dhan = d("DaRa~", Bhvadi);
    assert_has_tip(&[], &dhan, Lat, &["DaRati"]);

    let on = d("oRf~", Bhvadi);
    assert_has_tip(&[], &on, Lat, &["oRati"]);
    assert_has_tip(&[], &on, Lit, &["oRAYcakAra", "oRAmbaBUva", "oRAmAsa"]);

    let shon = d("SoRf~", Bhvadi);
    assert_has_tip(&[], &shon, Lat, &["SoRati"]);
    assert_has_tip(&[], &shon, Lit, &["SuSoRa"]);

    let shron = d("SroRf~", Bhvadi);
    assert_has_tip(&[], &shron, Lat, &["SroRati"]);

    let pain = d("pERf~", Bhvadi);
    assert_has_tip(&[], &pain, Lit, &["pipERa"]);

    // danDraRIti is justified.
    let dhran = d("DraRa~", Bhvadi);
    assert_has_tip(&[], &dhran, Lat, &["DraRati"]);
    assert_has_tip(&[], &yan_luk(&dhran), Lat, &["danDranti", "danDraRIti"]);

    let ban = d("baRa~", Bhvadi);
    assert_has_tas(&[], &ban, Lit, &["beRatuH"]);
    assert_has_sip(&[], &ban, Lit, &["beRiTa"]);

    let stan = d("zwana~", Bhvadi);
    assert_has_tip(&[], &stan, Lat, &["stanati"]);

    let van = d("vana~", Bhvadi);
    assert_has_tip(&[], &van, Lat, &["vanati"]);

    let san = d("zaRa~", Bhvadi);
    assert_has_tip(&[], &san, Lat, &["sanati"]);
    assert_has_tip(&[], &san, Lit, &["sasAna"]);
    assert_has_tas(&[], &san, Lit, &["senatuH"]);
}

#[test]
fn sk_2319() {
    let san = d("zaRa~", Bhvadi);
    assert_has_tip(&[], &san, AshirLin, &["sAyAt", "sanyAt"]);

    let am = d("ama~", Bhvadi);
    assert_has_tip(&[], &am, Lat, &["amati"]);
    assert_has_tip(&[], &am, Lit, &["Ama"]);

    let dram = d("drama~", Bhvadi);
    assert_has_tip(&[], &dram, Lat, &["dramati"]);
    assert_has_tip(&[], &dram, Lit, &["dadrAma"]);
    assert_has_tip(&[], &dram, Lun, &["adramIt"]);

    let hamm = d("hamma~", Bhvadi);
    assert_has_tip(&[], &hamm, Lat, &["hammati"]);
    assert_has_tip(&[], &hamm, Lit, &["jahamma"]);

    // TODO: SK has "mimIta," but what is it?
    let mim = d("mImf~", Bhvadi);
    assert_has_tip(&[], &mim, Lat, &["mImati"]);
    assert_has_tip(&[], &mim, Lit, &["mimIma"]);
}

#[test]
fn sk_2320() {
    let cam = d("camu~", Bhvadi);
    assert_has_tip(&[], &cam, Lat, &["camati"]);
    assert_has_tip(&["vi"], &cam, Lat, &["vicamati"]);
    assert_has_tip(&[], &cam, Lun, &["acamIt"]);

    // TODO: jimi~ will create jimmati?
    let jim = d("jima~", Bhvadi);
    assert_has_tip(&[], &jim, Lat, &["jemati"]);
}

#[test]
fn skip_sk_2321() {}

#[test]
fn sk_2322() {
    let kram = d("kramu~", Bhvadi);
    assert_has_tip(&[], &kram, Lat, &["krAmyati", "krAmati"]);
    assert_has_tip(&[], &kram, Lit, &["cakrAma"]);
    assert_has_tip(
        &[],
        &kram,
        Lot,
        &["krAmyatu", "krAmatu", "krAmyatAt", "krAmatAt"],
    );
}

#[test]
fn sk_2323() {
    let kram = d("kramu~", Bhvadi);
    assert_has_tip(&[], &kram, Lun, &["akramIt"]);

    let ay = d("aya~\\", Bhvadi);
    assert_has_ta(&[], &ay, Lat, &["ayate"]);
}

#[test]
fn sk_2324() {
    let ay = d("aya~\\", Bhvadi);
    assert_has_ta(&[], &ay, Lit, &["ayAYcakre", "ayAmbaBUva", "ayAmAsa"]);
    assert_has_ta(&[], &ay, VidhiLin, &["ayeta"]);
    assert_has_ta(&[], &ay, AshirLin, &["ayizIzwa"]);
}

#[test]
fn sk_2325() {
    let ay = d("aya~\\", Bhvadi);
    assert_has_dhvam(&[], &ay, AshirLin, &["ayizIQvam", "ayizIDvam"]);
    assert_has_ta(&[], &ay, Lun, &["Ayizwa"]);
    assert_has_dhvam(&[], &ay, Lun, &["AyiQvam", "AyiDvam"]);
}

#[test]
fn sk_2326() {
    let ay = d("aya~\\", Bhvadi);
    assert_has_ta(&["pra"], &ay, Lat, &["plAyate"]);
    assert_has_ta(&["parA"], &ay, Lat, &["palAyate"]);
    assert_has_ta(&["nis"], &ay, Lat, &["nirayate"]);
    assert_has_ta(&["dus"], &ay, Lat, &["durayate"]);
    assert_has_ta(&["nir"], &ay, Lat, &["nilayate"]);
    assert_has_ta(&["dur"], &ay, Lat, &["dulayate"]);

    let vay = d("vaya~\\", Bhvadi);
    assert_has_ta(&[], &vay, Lit, &["vavaye"]);

    let pay = d("paya~\\", Bhvadi);
    assert_has_ta(&[], &pay, Lit, &["peye"]);

    let may = d("maya~\\", Bhvadi);
    assert_has_ta(&[], &may, Lit, &["meye"]);

    let cay = d("caya~\\", Bhvadi);
    assert_has_ta(&[], &cay, Lit, &["ceye"]);

    let tay = d("taya~\\", Bhvadi);
    assert_has_ta(&[], &tay, Lit, &["teye"]);

    let nay = d("Raya~\\", Bhvadi);
    assert_has_ta(&["pra"], &nay, Lat, &["praRayate"]);
    assert_has_ta(&[], &nay, Lit, &["neye"]);

    let day = d("daya~\\", Bhvadi);
    assert_has_ta(&[], &day, Lit, &["dayAYcakre", "dayAmbaBUva", "dayAmAsa"]);

    let uy = d("UyI~\\", Bhvadi);
    assert_has_ta(&[], &uy, Lit, &["UyAYcakre", "UyAmbaBUva", "UyAmAsa"]);

    let puy = d("pUyI~\\", Bhvadi);
    assert_has_ta(&[], &puy, Lat, &["pUyate"]);
    assert_has_ta(&[], &puy, Lit, &["pupUye"]);

    let knuy = d("knUyI~\\", Bhvadi);
    assert_has_ta(&[], &knuy, Lit, &["cuknUye"]);

    let kshmay = d("kzmAyI~\\", Bhvadi);
    assert_has_ta(&[], &kshmay, Lit, &["cakzmAye"]);

    let sphay = d("sPAyI~\\", Bhvadi);
    assert_has_ta(&[], &sphay, Lat, &["sPAyate"]);
    assert_has_ta(&[], &sphay, Lit, &["pasPAye"]);

    let pyay = d("o~pyAyI~\\", Bhvadi);
    assert_has_ta(&[], &pyay, Lat, &["pyAyate"]);
}

#[test]
fn sk_2327() {
    let pyay = d("o~pyAyI~\\", Bhvadi);
    assert_has_ta(&[], &pyay, Lit, &["pipye"]);
    assert_has_aataam(&[], &pyay, Lit, &["pipyAte"]);
    assert_has_jha(&[], &pyay, Lit, &["pipyire"]);
}

#[test]
fn sk_2328() {}

#[test]
fn sk_2329() {
    let pyay = d("o~pyAyI~\\", Bhvadi);
    assert_has_ta(&[], &pyay, Lun, &["apyAyi", "apyAyizwa"]);

    let tay = d("tAyf~\\", Bhvadi);
    assert_has_ta(&[], &tay, Lat, &["tAyate"]);
    assert_has_ta(&[], &tay, Lit, &["tatAye"]);
    assert_has_ta(&[], &tay, Lun, &["atAyi", "atAyizwa"]);

    let val = d("vala~\\", Bhvadi);
    assert_has_ta(&[], &val, Lit, &["vavale"]);

    let vall = d("valla~\\", Bhvadi);
    assert_has_ta(&[], &vall, Lit, &["vavalle"]);

    let mal = d("mala~\\", Bhvadi);
    assert_has_ta(&[], &mal, Lit, &["mele"]);

    let mall = d("malla~\\", Bhvadi);
    assert_has_ta(&[], &mall, Lit, &["mamalle"]);

    let bhal = d("Bala~\\", Bhvadi);
    assert_has_ta(&[], &bhal, Lit, &["baBale"]);

    let bhall = d("Balla~\\", Bhvadi);
    assert_has_ta(&[], &bhall, Lit, &["baBalle"]);

    let kal = d("kala~\\", Bhvadi);
    assert_has_ta(&[], &kal, Lat, &["kalate"]);
    assert_has_ta(&[], &kal, Lit, &["cakale"]);

    let kall = d("kalla~\\", Bhvadi);
    assert_has_ta(&[], &kall, Lat, &["kallate"]);

    let sev = d("zevf~\\", Bhvadi);
    assert_has_ta(&["pari"], &sev, Lat, &["parizevate"]);
    assert_has_ta(&[], &sev, Lit, &["sizeve"]);

    let gev = d("gevf~\\", Bhvadi);
    assert_has_ta(&[], &gev, Lat, &["gevate"]);
    assert_has_ta(&[], &gev, Lit, &["jigeve"]);

    let glev = d("glevf~\\", Bhvadi);
    assert_has_ta(&[], &glev, Lit, &["jigleve"]);

    let pev = d("pevf~\\", Bhvadi);
    assert_has_ta(&[], &pev, Lit, &["pipeve"]);

    let mev = d("mevf~\\", Bhvadi);
    assert_has_ta(&[], &mev, Lat, &["mevate"]);

    let mlev = d("mlevf~\\", Bhvadi);
    assert_has_ta(&[], &mlev, Lat, &["mlevate"]);

    let mavy = d("mavya~", Bhvadi);
    assert_has_tip(&[], &mavy, Lit, &["mamavya"]);

    let hay = d("haya~", Bhvadi);
    assert_has_tip(&[], &hay, Lun, &["ahayIt"]);

    let shucy = d("Sucya~", Bhvadi);
    assert_has_tip(&[], &shucy, Lit, &["SuSucya"]);

    let al = d("ala~", Bhvadi);
    assert_has_tip(&[], &al, Lat, &["alati"]);
    assert_has_tip(&[], &al, Lit, &["Ala"]);
}

#[test]
fn sk_2330() {
    // TODO: mAn-yoga
    let al = d("ala~", Bhvadi);
    assert_has_tip(&[], &al, Lun, &["AlIt"]);

    // "ayaM svarited ityeke"
    let al = d("ala~^", Bhvadi);
    assert_has_ta(&[], &al, Lat, &["alate"]);

    let phal = d("YiPalA~", Bhvadi);
    assert_has_tas(&[], &phal, Lit, &["PelatuH"]);
    assert_has_jhi(&[], &phal, Lit, &["PeluH"]);
    assert_has_tip(&[], &phal, Lun, &["aPAlIt"]);

    let shil = d("SIla~", Bhvadi);
    assert_has_tip(&[], &shil, Lat, &["SIlati"]);

    let tul = d("tUla~", Bhvadi);
    assert_has_tip(&[], &tul, Lit, &["tutUla"]);

    let phal = d("Pala~", Bhvadi);
    assert_has_tas(&[], &phal, Lit, &["PelatuH"]);
    assert_has_jhi(&[], &phal, Lit, &["PeluH"]);

    let til = d("tila~", Bhvadi);
    assert_has_tip(&[], &til, Lat, &["telati"]);

    let till = d("tilla~", Bhvadi);
    assert_has_tip(&[], &till, Lat, &["tillati"]);

    let skhal = d("sKala~", Bhvadi);
    assert_has_tip(&[], &skhal, Lit, &["casKAla"]);
    assert_has_tip(&[], &skhal, Lun, &["asKAlIt"]);

    let gal = d("gala~", Bhvadi);
    assert_has_tip(&[], &gal, Lat, &["galati"]);
    assert_has_tip(&[], &gal, Lun, &["agAlIt"]);

    let sanl = d("zala~", Bhvadi);
    assert_has_tip(&[], &sanl, Lat, &["salati"]);

    let shval = d("Svala~", Bhvadi);
    assert_has_tip(&[], &shval, Lit, &["SaSvAla"]);
    assert_has_tip(&[], &shval, Lun, &["aSvAlIt"]);

    let shvall = d("Svalla~", Bhvadi);
    assert_has_tip(&[], &shvall, Lun, &["aSvallIt"]);

    let khol = d("Kolf~", Bhvadi);
    assert_has_tip(&[], &khol, Lat, &["Kolati"]);

    let khor = d("Korf~", Bhvadi);
    assert_has_tip(&[], &khor, Lat, &["Korati"]);

    let dhor = d("Dorf~", Bhvadi);
    assert_has_tip(&[], &dhor, Lat, &["Dorati"]);

    let tsar = d("tsara~", Bhvadi);
    assert_has_tip(&[], &tsar, Lit, &["tatsAra"]);
    assert_has_tip(&[], &tsar, Lun, &["atsArIt"]);

    let kmar = d("kmara~", Bhvadi);
    assert_has_tip(&[], &kmar, Lit, &["cakmAra"]);

    let abh = d("aBra~", Bhvadi);
    assert_has_tip(&[], &abh, Lat, &["aBrati"]);
    assert_has_tip(&[], &abh, Lit, &["AnaBra"]);

    let shthiv = d("zWivu~", Bhvadi);
    assert_has_tip(&[], &shthiv, Lat, &["zWIvati"]);
    assert_has_tip(&[], &shthiv, Lit, &["tizWeva", "wizWeva"]);
    assert_has_tas(&[], &shthiv, Lit, &["tizWivatuH", "wizWivatuH"]);
    assert_has_jhi(&[], &shthiv, Lit, &["tizWivuH", "wizWivuH"]);
    assert_has_tip(&[], &shthiv, AshirLin, &["zWIvyAt"]);

    let ji = d("ji\\", Bhvadi);
    assert_has_tip(&[], &ji, Lat, &["jayati"]);
}

#[test]
fn sk_2331() {
    let ji = d("ji\\", Bhvadi);
    assert_has_tip(&[], &ji, Lit, &["jigAya"]);
    assert_has_tas(&[], &ji, Lit, &["jigyatuH"]);
    assert_has_jhi(&[], &ji, Lit, &["jigyuH"]);
    assert_has_sip(&[], &ji, Lit, &["jigayiTa", "jigeTa"]);
    assert_has_mip(&[], &ji, Lit, &["jigAya", "jigaya"]);
    assert_has_vas(&[], &ji, Lit, &["jigyiva"]);
    assert_has_mas(&[], &ji, Lit, &["jigyima"]);

    assert_has_tip(&[], &ji, Lut, &["jetA"]);
    assert_has_tip(&[], &ji, AshirLin, &["jIyAt"]);
    assert_has_tip(&[], &ji, Lun, &["ajEzIt"]);

    let jiv = d("jIva~", Bhvadi);
    assert_has_tip(&[], &jiv, Lit, &["jijIva"]);

    let piv = d("pIva~", Bhvadi);
    assert_has_tip(&[], &piv, Lit, &["pipIva"]);

    let miv = d("mIva~", Bhvadi);
    assert_has_tip(&[], &miv, Lit, &["mimIva"]);

    let tiv = d("tIva~", Bhvadi);
    assert_has_tip(&[], &tiv, Lit, &["titIva"]);

    let niv = d("RIva~", Bhvadi);
    assert_has_tip(&[], &niv, Lit, &["ninIva"]);

    let turv = d("turvI~", Bhvadi);
    assert_has_tip(&[], &turv, Lit, &["tutUrva"]);

    let gurv = d("gurvI~", Bhvadi);
    assert_has_tip(&[], &gurv, Lat, &["gUrvati"]);
    assert_has_tip(&[], &gurv, Lit, &["jugUrva"]);

    let arv = d("arva~", Bhvadi);
    assert_has_tip(&[], &arv, Lit, &["Anarva"]);

    let sharv = d("Sarva~", Bhvadi);
    assert_has_tip(&[], &sharv, Lat, &["Sarvati"]);

    let sarv = d("zarva~", Bhvadi);
    assert_has_tip(&[], &sarv, Lat, &["sarvati"]);

    let inv = d("ivi~", Bhvadi);
    assert_has_tip(&[], &inv, Lat, &["invati"]);
    assert_has_tip(&[], &inv, Lit, &["invAYcakAra", "invAmbaBUva", "invAmAsa"]);

    let pinv = d("pivi~", Bhvadi);
    assert_has_tip(&[], &pinv, Lat, &["pinvati"]);
    assert_has_tip(&[], &pinv, Lit, &["pipinva"]);

    let hinv = d("hivi~", Bhvadi);
    assert_has_tip(&[], &hinv, Lat, &["hinvati"]);

    let dinv = d("divi~", Bhvadi);
    assert_has_tip(&[], &dinv, Lat, &["dinvati"]);
}

#[test]
fn sk_2332() {
    let dhinv = d("Divi~", Bhvadi);
    assert_has_tip(&[], &dhinv, Lat, &["Dinoti"]);
    assert_has_tas(&[], &dhinv, Lat, &["DinutaH"]);
    assert_has_jhi(&[], &dhinv, Lat, &["Dinvanti"]);
}

#[test]
fn sk_2333() {
    let dhinv = d("Divi~", Bhvadi);
    assert_has_vas(&[], &dhinv, Lat, &["DinvaH", "DinuvaH"]);
    assert_has_mas(&[], &dhinv, Lat, &["DinmaH", "DinumaH"]);
    assert_has_mip(&[], &dhinv, Lat, &["Dinomi"]);
}

#[test]
fn sk_2334() {
    let dhinv = d("Divi~", Bhvadi);
    assert_has_sip(&[], &dhinv, Lot, &["Dinu", "DinutAt"]);
    assert_has_vas(&[], &dhinv, Lot, &["DinavAva"]);
    assert_has_mas(&[], &dhinv, Lot, &["DinavAma"]);

    let jinv = d("jivi~", Bhvadi);
    assert_has_tip(&[], &jinv, Lat, &["jinvati"]);

    let rinv = d("rivi~", Bhvadi);
    assert_has_tip(&[], &rinv, Lat, &["riRvati"]);

    let ranv = d("ravi~", Bhvadi);
    assert_has_tip(&[], &ranv, Lat, &["raRvati"]);

    let dhanv = d("Davi~", Bhvadi);
    assert_has_tip(&[], &dhanv, Lat, &["Danvati"]);

    let mav = d("mava~", Bhvadi);
    assert_has_tip(&[], &mav, Lat, &["mavati"]);
    assert_has_tas(&[], &mav, Lit, &["mevatuH"]);
    assert_has_jhi(&[], &mav, Lit, &["mevuH"]);
    assert_has_tip(&[], &mav, Lun, &["amavIt", "amAvIt"]);

    let av = d("ava~", Bhvadi);
    assert_has_tip(&[], &av, Lat, &["avati"]);
    assert_has_tip(&[], &av, Lit, &["Ava"]);
    // TODO: replace below with avIt (mAN-yoga)
    assert_has_tip(&[], &av, Lun, &["AvIt"]);

    let dhav = d("DAvu~^", Bhvadi);
    assert_has_tip(&[], &dhav, Lat, &["DAvati"]);
    assert_has_ta(&[], &dhav, Lat, &["DAvate"]);
    assert_has_tip(&[], &dhav, Lit, &["daDAva"]);
    assert_has_ta(&[], &dhav, Lit, &["daDAve"]);

    let dhuksh = d("Dukza~\\", Bhvadi);
    assert_has_ta(&[], &dhuksh, Lat, &["Dukzate"]);
    assert_has_ta(&[], &dhuksh, Lit, &["duDukze"]);

    let dhiksh = d("Dikza~\\", Bhvadi);
    assert_has_ta(&[], &dhiksh, Lat, &["Dikzate"]);
    assert_has_ta(&[], &dhiksh, Lit, &["diDikze"]);

    let vfksh = d("vfkza~\\", Bhvadi);
    assert_has_ta(&[], &vfksh, Lat, &["vfkzate"]);
    assert_has_ta(&[], &vfksh, Lit, &["vavfkze"]);

    let shiksh = d("Sikza~\\", Bhvadi);
    assert_has_ta(&[], &shiksh, Lat, &["Sikzate"]);

    let bhiksh = d("Bikza~\\", Bhvadi);
    assert_has_ta(&[], &bhiksh, Lat, &["Bikzate"]);

    let klesh = d("kleSa~\\", Bhvadi);
    assert_has_ta(&[], &klesh, Lat, &["kleSate"]);
    assert_has_ta(&[], &klesh, Lit, &["cikleSe"]);

    let daksh = d("dakza~\\", Bhvadi);
    assert_has_ta(&[], &daksh, Lat, &["dakzate"]);
    assert_has_ta(&[], &daksh, Lit, &["dadakze"]);

    let diksh = d("dIkza~\\", Bhvadi);
    assert_has_ta(&[], &diksh, Lat, &["dIkzate"]);
    assert_has_ta(&[], &diksh, Lit, &["didIkze"]);

    let iksh = d("Ikza~\\", Bhvadi);
    assert_has_ta(&[], &iksh, Lit, &["IkzAYcakre", "IkzAmbaBUva", "IkzAmAsa"]);

    let ish = d("Iza~\\", Bhvadi);
    assert_has_ta(&[], &ish, Lit, &["IzAYcakre", "IzAmbaBUva", "IzAmAsa"]);

    let bhash = d("BAza~\\", Bhvadi);
    assert_has_ta(&[], &bhash, Lat, &["BAzate"]);

    let bhash = d("BAza~\\", Bhvadi);
    assert_has_ta(&[], &bhash, Lat, &["BAzate"]);

    let varsh = d("varza~\\", Bhvadi);
    assert_has_ta(&[], &varsh, Lit, &["vavarze"]);

    let gesh = d("gezf~\\", Bhvadi);
    assert_has_ta(&[], &gesh, Lit, &["jigeze"]);

    let jesh = d("jezf~\\", Bhvadi);
    assert_has_ta(&[], &jesh, Lat, &["jezate"]);

    let nesh = d("Rezf~\\", Bhvadi);
    assert_has_ta(&[], &nesh, Lat, &["nezate"]);

    let esh = d("ezf~\\", Bhvadi);
    assert_has_ta(&[], &esh, Lit, &["ezAYcakre", "ezAmbaBUva", "ezAmAsa"]);

    let presh = d("prezf~\\", Bhvadi);
    assert_has_ta(&[], &presh, Lit, &["pipreze"]);

    let resh = d("rezf~\\", Bhvadi);
    assert_has_ta(&[], &resh, Lat, &["rezate"]);

    let hesh = d("hezf~\\", Bhvadi);
    assert_has_ta(&[], &hesh, Lat, &["hezate"]);

    let hresh = d("hrezf~\\", Bhvadi);
    assert_has_ta(&[], &hresh, Lat, &["hrezate"]);

    let kas = d("kAsf~\\", Bhvadi);
    assert_has_ta(&[], &kas, Lit, &["kAsAYcakre", "kAsAmbaBUva", "kAsAmAsa"]);

    let bhas = d("BAsf~\\", Bhvadi);
    assert_has_ta(&[], &bhas, Lit, &["baBAse"]);

    let naas = d("RAsf~\\", Bhvadi);
    assert_has_ta(&[], &naas, Lat, &["nAsate"]);
    assert_has_ta(&["pra"], &naas, Lat, &["praRAsate"]);

    let nas = d("Rasa~\\", Bhvadi);
    assert_has_ta(&[], &nas, Lat, &["nasate"]);

    let bhyas = d("Byasa~\\", Bhvadi);
    assert_has_ta(&[], &bhyas, Lat, &["Byasate"]);
    assert_has_ta(&[], &bhyas, Lit, &["baByase"]);

    let shas = d("Sasi~\\", Bhvadi);
    assert_has_ta(&["AN"], &shas, Lat, &["ASaMsate"]);
    assert_has_thaas(&["AN"], &shas, Lat, &["ASaMsase"]);

    let gras = d("grasu~\\", Bhvadi);
    assert_has_ta(&[], &gras, Lit, &["jagrase"]);

    let glas = d("glasu~\\", Bhvadi);
    assert_has_ta(&[], &glas, Lit, &["jaglase"]);

    let ih = d("Iha~\\", Bhvadi);
    assert_has_ta(&[], &ih, Lit, &["IhAYcakre", "IhAmbaBUva", "IhAmAsa"]);

    // TODO: Text lists "vahi" but uses "bahi" ?
    let vanh = d("vahi~\\", Bhvadi);
    assert_has_ta(&[], &vanh, Lat, &["vaMhate"]);
    assert_has_ta(&[], &vanh, Lit, &["vavaMhe"]);

    let manh = d("mahi~\\", Bhvadi);
    assert_has_ta(&[], &manh, Lat, &["maMhate"]);

    let anh = d("ahi~\\", Bhvadi);
    assert_has_ta(&[], &anh, Lat, &["aMhate"]);
    assert_has_ta(&[], &anh, Lit, &["AnaMhe"]);

    let garh = d("garha~\\", Bhvadi);
    assert_has_ta(&[], &garh, Lit, &["jagarhe"]);

    let galh = d("galha~\\", Bhvadi);
    assert_has_ta(&[], &galh, Lit, &["jagalhe"]);

    let plih = d("pliha~\\", Bhvadi);
    assert_has_ta(&[], &plih, Lit, &["piplihe"]);

    let bah = d("bAhf~\\", Bhvadi);
    assert_has_ta(&[], &bah, Lit, &["babAhe"]);

    let kash = d("kASf~\\", Bhvadi);
    assert_has_ta(&[], &kash, Lit, &["cakASe"]);

    let uh = d("Uha~\\", Bhvadi);
    assert_has_ta(&[], &uh, Lit, &["UhAYcakre", "UhAmbaBUva", "UhAmAsa"]);

    let gah = d("gAhU~\\", Bhvadi);
    assert_has_ta(&[], &gah, Lat, &["gAhate"]);
    assert_has_ta(&[], &gah, Lit, &["jagAhe"]);
    assert_has_thaas(&[], &gah, Lit, &["jagAhize", "jaGAkze"]);
    assert_has_dhvam(&[], &gah, Lit, &["jagAhiQve", "jagAhiDve", "jaGAQve"]);
    // gAQA is from SK 2335.
    assert_has_ta(&[], &gah, Lut, &["gAhitA", "gAQA"]);
}

#[test]
fn sk_2335() {
    let gah = d("gAhU~\\", Bhvadi);
    assert_has_ta(&[], &gah, Lut, &["gAhitA", "gAQA"]);
    assert_has_ta(&[], &gah, Lrt, &["gAhizyate", "GAkzyate"]);
    assert_has_ta(&[], &gah, AshirLin, &["gAhizIzwa", "GAkzIzwa"]);
    assert_has_ta(&[], &gah, Lun, &["agAhizwa", "agAQa"]);
    // agAhizatAm, agAhizata, agAhizWAH, and agAhiDvam/agAhiQvam are justified.
    assert_has_aataam(&[], &gah, Lun, &["agAhizAtAm", "aGAkzAtAm"]);
    assert_has_jha(&[], &gah, Lun, &["agAhizata", "aGAkzata"]);
    assert_has_thaas(&[], &gah, Lun, &["agAhizWAH", "agAQAH"]);
    assert_has_dhvam(&[], &gah, Lun, &["agAhiDvam", "agAhiQvam", "aGAQvam"]);
    assert_has_iw(&[], &gah, Lun, &["agAhizi", "aGAkzi"]);

    let grh = d("gfhU~\\", Bhvadi);
    assert_has_ta(&[], &grh, Lat, &["garhate"]);
    assert_has_ta(&[], &grh, Lit, &["jagfhe"]);
    assert_has_thaas(&[], &grh, Lit, &["jagfhize", "jaGfkze"]);
    // jagfhiDve and jagfhiQve are justified.
    assert_has_dhvam(&[], &grh, Lit, &["jagfhiDve", "jagfhiQve", "jaGfQve"]);
    assert_has_ta(&[], &grh, Lut, &["garhitA", "garQA"]);
    assert_has_ta(&[], &grh, Lrt, &["garhizyate", "Garkzyate"]);
    assert_has_ta(&[], &grh, AshirLin, &["garhizIzwa", "GfkzIzwa"]);
    assert_has_ta(&[], &grh, Lun, &["agarhizwa", "aGfkzata"]);
}

#[test]
fn sk_2336() {
    let grh = d("gfhU~\\", Bhvadi);
    assert_has_ta(&[], &grh, Lun, &["agarhizwa", "aGfkzata"]);
}

#[test]
fn sk_2337() {
    let grh = d("gfhU~\\", Bhvadi);
    // agarhizAtAm and agarhizanta are justified.
    assert_has_aataam(&[], &grh, Lun, &["agarhizAtAm", "aGfkzAtAm"]);
    assert_has_jha(&[], &grh, Lun, &["agarhizata", "aGfkzanta"]);

    let glah = d("glaha~\\", Bhvadi);
    assert_has_ta(&[], &glah, Lat, &["glahate"]);

    let ghush = d("Guzi~\\", Bhvadi);
    assert_has_ta(&[], &ghush, Lat, &["GuMzate"]);
    assert_has_ta(&[], &ghush, Lit, &["juGuMze"]);

    let ghush = d("Guzi~r", Bhvadi);
    assert_has_tip(&[], &ghush, Lat, &["Gozati"]);
    assert_has_tip(&[], &ghush, Lit, &["juGoza"]);
    assert_has_tip(&[], &ghush, Lut, &["GozitA"]);
    assert_has_tip(&[], &ghush, Lun, &["aGuzat", "aGozIt"]);
}

#[test]
fn sk_2338() {
    let aksh = d("akzU~", Bhvadi);
    assert_has_tip(&[], &aksh, Lat, &["akzRoti", "akzati"]);
    assert_has_tas(&[], &aksh, Lat, &["akzRutaH", "akzataH"]);
    assert_has_jhi(&[], &aksh, Lat, &["akzRuvanti", "akzanti"]);
    assert_has_tip(&[], &aksh, Lit, &["Anakza"]);
    assert_has_sip(&[], &aksh, Lit, &["AnakziTa", "AnazWa"]);
    assert_has_tip(&[], &aksh, Lut, &["akzitA", "azwA"]);
    assert_has_tip(&[], &aksh, Lrt, &["akzizyati", "akzyati"]);

    assert_has_tip(
        &[],
        &aksh,
        Lot,
        &["akzRotu", "akzatu", "akzRutAt", "akzatAt"],
    );
    assert_has_sip(&[], &aksh, Lot, &["akzRuhi", "akzRutAt", "akza", "akzatAt"]);
    assert_has_mip(&[], &aksh, Lot, &["akzRavAni", "akzARi"]);
    assert_has_tip(&[], &aksh, Lan, &["AkzRot", "Akzat"]);
    assert_has_mip(&[], &aksh, Lan, &["AkzRavam", "Akzam"]);
    assert_has_tip(&[], &aksh, VidhiLin, &["akzRuyAt", "akzet"]);
    assert_has_tip(&[], &aksh, AshirLin, &["akzyAt"]);

    // TODO: mAn-yoga
    assert_has_tip(&[], &aksh, Lun, &["AkzIt"]);
    assert_has_tas(&[], &aksh, Lun, &["AkzizwAm", "AzwAm"]);
    assert_has_jhi(&[], &aksh, Lun, &["AkzizuH", "AkzuH"]);
}

#[test]
fn sk_2339() {
    let taksh = d("takzU~", Bhvadi);
    assert_has_sip(&[], &taksh, Lit, &["tatakziTa", "tatazWa"]);
    assert_has_tip(&[], &taksh, Lun, &["atakzIt", "atAkzIt"]);
    assert_has_tas(&[], &taksh, Lun, &["atakzizwAm", "atAzwAm"]);
    assert_has_tip(&["sam"], &taksh, Lat, &["santakzati", "santakzRoti"]);

    let uksh = d("ukza~", Bhvadi);
    assert_has_tip(&[], &uksh, Lit, &["ukzAYcakAra", "ukzAmbaBUva", "ukzAmAsa"]);

    let niksh = d("Rikza~", Bhvadi);
    assert_has_tip(&["pra"], &niksh, Lat, &["praRikzati"]);

    let trksh = d("tfkza~", Bhvadi);
    assert_has_tip(&[], &trksh, Lat, &["tfkzati"]);

    let strksh = d("zwfkza~", Bhvadi);
    assert_has_tip(&[], &strksh, Lat, &["stfkzati"]);

    let naksh = d("Rakza~", Bhvadi);
    assert_has_tip(&[], &naksh, Lat, &["nakzati"]);

    let surksh = d("sUrkza~", Bhvadi);
    assert_has_tip(&[], &surksh, Lit, &["susUrkza"]);

    let cush = d("cUza~", Bhvadi);
    assert_has_tip(&[], &cush, Lit, &["cucUza"]);

    let bhush = d("BUza~", Bhvadi);
    assert_has_tip(&[], &bhush, Lat, &["BUzati"]);

    let ush = d("Uza~", Bhvadi);
    assert_has_tip(&[], &ush, Lit, &["UzAYcakAra", "UzAmbaBUva", "UzAmAsa"]);

    let kash = d("kaza~", Bhvadi);
    assert_has_tip(&[], &kash, Lit, &["cakAza"]);

    let khash = d("Kaza~", Bhvadi);
    assert_has_tip(&[], &khash, Lit, &["caKAza"]);

    let shish = d("Si\\za~", Bhvadi);
    assert_has_tip(&[], &shish, Lit, &["SiSeza"]);
    assert_has_sip(&[], &shish, Lit, &["SiSeziTa"]);
    assert_has_tip(&[], &shish, Lut, &["SezwA"]);
    assert_has_tip(&[], &shish, Lun, &["aSikzat"]);
    assert_has_tip(&[], &shish, Lrn, &["aSekzyat"]);

    let jash = d("jaza~", Bhvadi);
    assert_has_tas(&[], &jash, Lit, &["jezatuH"]);

    let jhash = d("Jaza~", Bhvadi);
    assert_has_tas(&[], &jhash, Lit, &["jaJazatuH"]);

    let shash = d("Saza~", Bhvadi);
    assert_has_tas(&[], &shash, Lit, &["SezatuH"]);

    let vash = d("vaza~", Bhvadi);
    assert_has_tas(&[], &vash, Lit, &["vavazatuH"]);

    let mash = d("maza~", Bhvadi);
    assert_has_tas(&[], &mash, Lit, &["mezatuH"]);
}

#[test]
fn sk_2340() {
    let rush = d("ruza~", Bhvadi);
    assert_has_tip(&[], &rush, Lut, &["rozitA", "rozwA"]);
    assert_has_tip(&[], &rush, Lrt, &["rozizyati"]);

    let rish = d("riza~", Bhvadi);
    assert_has_tip(&[], &rish, Lut, &["rezitA", "rezwA"]);
    assert_has_tip(&[], &rish, Lrt, &["rezizyati"]);

    let bhash = d("Baza~", Bhvadi);
    assert_has_tip(&[], &bhash, Lat, &["Bazati"]);
    assert_has_tip(&[], &bhash, Lit, &["baBAza"]);

    let ush = d("uza~", Bhvadi);
    assert_has_tip(&[], &ush, Lat, &["ozati"]);
}

#[test]
fn sk_2341() {
    let ush = d("uza~", Bhvadi);
    assert_has_tip(
        &[],
        &ush,
        Lit,
        &["uvoza", "ozAYcakAra", "ozAmbaBUva", "ozAmAsa"],
    );
    assert_has_tas(
        &[],
        &ush,
        Lit,
        &["UzatuH", "ozAYcakratuH", "ozAmbaBUvatuH", "ozAmAsatuH"],
    );
    assert_has_sip(
        &[],
        &ush,
        Lit,
        &["uvoziTa", "ozAYcakarTa", "ozAmbaBUviTa", "ozAmAsiTa"],
    );

    let jish = d("jizu~", Bhvadi);
    assert_has_tip(&[], &jish, Lit, &["jijeza"]);

    let vish = d("vi\\zu~", Bhvadi);
    assert_has_sip(&[], &vish, Lit, &["viveziTa"]);
    assert_has_vas(&[], &vish, Lit, &["viviziva"]);
    assert_has_tip(&[], &vish, Lut, &["vezwA"]);
    assert_has_tip(&[], &vish, Lrt, &["vekzyati"]);
    assert_has_tip(&[], &vish, Lun, &["avikzat"]);

    let push = d("puza~", Bhvadi);
    assert_has_tip(&[], &push, Lat, &["pozati"]);
    assert_has_tip(&[], &push, Lut, &["pozitA"]);
    assert_has_tip(&[], &push, Lrt, &["pozizyati"]);
    assert_has_tip(&[], &push, Lun, &["apozIt"]);

    let shrish = d("Srizu~", Bhvadi);
    assert_has_tip(&[], &shrish, Lat, &["Srezati"]);
    assert_has_tip(&[], &shrish, Lit, &["SiSreza"]);
    assert_has_tip(&[], &shrish, Lut, &["SrezitA"]);

    let shlish = d("Slizu~", Bhvadi);
    assert_has_tip(&[], &shlish, Lat, &["Slezati"]);
    assert_has_tip(&[], &shlish, Lit, &["SiSleza"]);
    assert_has_tip(&[], &shlish, Lut, &["SlezitA"]);

    let prush = d("pruzu~", Bhvadi);
    assert_has_tip(&[], &prush, Lit, &["puproza"]);

    let plush = d("pluzu~", Bhvadi);
    assert_has_tip(&[], &plush, Lit, &["puploza"]);

    let prsh = d("pfzu~", Bhvadi);
    assert_has_tip(&[], &prsh, Lat, &["parzati"]);
    assert_has_tip(&[], &prsh, Lit, &["paparza"]);
    assert_has_tip(&[], &prsh, AshirLin, &["pfzyAt"]);

    let tus = d("tusa~", Bhvadi);
    assert_has_tip(&[], &tus, Lit, &["tutosa"]);

    let hras = d("hrasa~", Bhvadi);
    assert_has_tip(&[], &hras, Lit, &["jahrAsa"]);

    let hlas = d("hlasa~", Bhvadi);
    assert_has_tip(&[], &hlas, Lit, &["jahlAsa"]);

    let ras = d("rasa~", Bhvadi);
    assert_has_tip(&[], &ras, Lit, &["rarAsa"]);

    let ghas = d("Ga\\sx~", Bhvadi);
    assert_has_tip(&[], &ghas, Lat, &["Gasati"]);
    assert_has_tip(&[], &ghas, Lut, &["GastA"]);
}

#[test]
fn sk_2342() {
    let ghas = d("Ga\\sx~", Bhvadi);
    assert_has_tip(&[], &ghas, Lrt, &["Gatsyati"]);
    assert_has_tip(&[], &ghas, Lot, &["Gasatu", "GasatAt"]);
    assert_has_tip(&[], &ghas, Lun, &["aGasat"]);
    assert_has_tip(&[], &ghas, VidhiLin, &["Gaset"]);
}

#[test]
fn sk_2343() {
    let ghas = d("Ga\\sx~", Bhvadi);
    assert_has_tip(&[], &ghas, Lun, &["aGasat"]);

    let pis = d("pisf~", Bhvadi);
    assert_has_tas(&[], &pis, Lit, &["pipisatuH"]);

    let pes = d("pesf~", Bhvadi);
    assert_has_tas(&[], &pes, Lit, &["pipesatuH"]);

    let has = d("hase~", Bhvadi);
    assert_has_tip(&[], &has, Lun, &["ahasIt"]);

    let nish = d("RiSa~", Bhvadi);
    assert_has_tip(&["pra"], &nish, Lat, &["praReSati"]);

    let shav = d("Sava~", Bhvadi);
    assert_has_tip(&[], &shav, Lat, &["Savati"]);
    assert_has_tip(&[], &shav, Lun, &["aSavIt", "aSAvIt"]);

    let shash = d("SaSa~", Bhvadi);
    assert_has_tip(&[], &shash, Lit, &["SaSASa"]);
    assert_has_tas(&[], &shash, Lit, &["SeSatuH"]);
    assert_has_jhi(&[], &shash, Lit, &["SeSuH"]);
    assert_has_sip(&[], &shash, Lit, &["SeSiTa"]);

    let shas = d("Sasu~", Bhvadi);
    assert_has_tas(&[], &shas, Lit, &["SaSasatuH"]);
    assert_has_jhi(&[], &shas, Lit, &["SaSasuH"]);
    assert_has_sip(&[], &shas, Lit, &["SaSasiTa"]);

    let shans = d("Sansu~", Bhvadi);
    assert_has_tip(&[], &shans, Lit, &["SaSaMsa"]);
    assert_has_tip(&[], &shans, AshirLin, &["SasyAt"]);

    let cah = d("caha~", Bhvadi);
    assert_has_tip(&[], &cah, Lun, &["acahIt"]);

    let mah = d("maha~", Bhvadi);
    assert_has_tip(&[], &mah, Lun, &["amahIt"]);

    let ranh = d("rahi~", Bhvadi);
    assert_has_tip(&[], &ranh, Lat, &["raMhati"]);
    assert_has_tip(&[], &ranh, AshirLin, &["raMhyAt"]);

    let drh = d("dfha~", Bhvadi);
    assert_has_tip(&[], &drh, Lat, &["darhati"]);
    assert_has_tip(&[], &drh, Lit, &["dadarha"]);
    assert_has_tas(&[], &drh, Lit, &["dadfhatuH"]);

    let drnh = d("dfhi~", Bhvadi);
    assert_has_tip(&[], &drnh, Lat, &["dfMhati"]);

    let brh = d("bfha~", Bhvadi);
    assert_has_tip(&[], &brh, Lat, &["barhati"]);

    let brnh = d("bfhi~", Bhvadi);
    assert_has_tip(&[], &brnh, Lat, &["bfMhati"]);

    let brhir = d("bfhi~r", Bhvadi);
    assert_has_tip(&[], &brhir, Lun, &["abfhat", "abarhIt"]);

    let tuh = d("tuhi~r", Bhvadi);
    assert_has_tip(&[], &tuh, Lat, &["tohati"]);
    assert_has_tip(&[], &tuh, Lit, &["tutoha"]);
    assert_has_tip(&[], &tuh, Lun, &["atuhat", "atohIt"]);

    let duh = d("duhi~r", Bhvadi);
    assert_has_tip(&[], &duh, Lat, &["dohati"]);
    assert_has_tip(&[], &duh, Lit, &["dudoha"]);
    assert_has_tip(&[], &duh, Lun, &["aduhat", "adohIt"]);

    let uh = d("uhi~r", Bhvadi);
    assert_has_tip(&[], &uh, Lat, &["ohati"]);
    assert_has_tip(&[], &uh, Lit, &["uvoha"]);
    assert_has_tas(&[], &uh, Lit, &["UhatuH"]);
    assert_has_tip(&[], &uh, Lut, &["ohitA"]);
    // TODO: mAn-yoga
    assert_has_tip(&[], &uh, Lun, &["OhIt", "Ohat"]);

    let arh = d("arha~", Bhvadi);
    assert_has_tip(&[], &arh, Lit, &["Anarha"]);

    let dyut = d("dyuta~\\", Bhvadi);
    assert_has_ta(&[], &dyut, Lat, &["dyotate"]);
}

#[test]
fn sk_2344() {
    let dyut = d("dyuta~\\", Bhvadi);
    assert_has_ta(&[], &dyut, Lit, &["didyute"]);
    assert_has_aataam(&[], &dyut, Lit, &["didyutAte"]);
    assert_has_ta(&[], &dyut, Lut, &["dyotitA"]);
}

#[test]
fn sk_2345() {
    let dyut = d("dyuta~\\", Bhvadi);
    assert_has_tip(&[], &dyut, Lun, &["adyutat"]);
    assert_has_ta(&[], &dyut, Lun, &["adyotizwa"]);

    let shvit = d("SvitA~\\", Bhvadi);
    assert_has_ta(&[], &shvit, Lat, &["Svetate"]);
    assert_has_ta(&[], &shvit, Lit, &["SiSvite"]);
    assert_has_tip(&[], &shvit, Lun, &["aSvitat"]);
    assert_has_ta(&[], &shvit, Lun, &["aSvetizwa"]);

    let mid = d("YimidA~\\", Bhvadi);
    assert_has_ta(&[], &mid, Lat, &["medate"]);
}

#[test]
fn sk_2346() {
    let mid = d("YimidA~\\", Bhvadi);
    assert_has_ta(&[], &mid, Lit, &["mimide"]);
    assert_has_tip(&[], &mid, Lun, &["amidat"]);
    assert_has_ta(&[], &mid, Lun, &["amedizwa"]);

    let svid = d("YizvidA~\\", Bhvadi);
    assert_has_ta(&[], &svid, Lat, &["svedate"]);
    assert_has_ta(&[], &svid, Lit, &["sizvide"]);
    assert_has_tip(&[], &svid, Lun, &["asvidat"]);
    assert_has_ta(&[], &svid, Lun, &["asvedizwa"]);

    let kshvid = d("YikzvidA~\\", Bhvadi);
    assert_has_tip(&[], &kshvid, Lun, &["akzvidat"]);
    assert_has_ta(&[], &kshvid, Lun, &["akzvedizwa"]);

    let ruc = d("ruca~\\", Bhvadi);
    assert_has_ta(&[], &ruc, Lat, &["rocate"]);
    assert_has_tip(&[], &ruc, Lun, &["arucat"]);
    assert_has_ta(&[], &ruc, Lun, &["arocizwa"]);

    let ghut = d("Guwa~\\", Bhvadi);
    assert_has_ta(&[], &ghut, Lat, &["Gowate"]);
    assert_has_ta(&[], &ghut, Lit, &["juGuwe"]);
    assert_has_tip(&[], &ghut, Lun, &["aGuwat"]);
    assert_has_ta(&[], &ghut, Lun, &["aGowizwa"]);

    let rut = d("ruwa~\\", Bhvadi);
    assert_has_tip(&[], &rut, Lun, &["aruwat"]);
    assert_has_ta(&[], &rut, Lun, &["arowizwa"]);

    let nabh = d("RaBa~\\", Bhvadi);
    assert_has_tip(&[], &nabh, Lun, &["anaBat"]);
    assert_has_ta(&[], &nabh, Lun, &["anaBizwa"]);

    let tubh = d("tuBa~\\", Bhvadi);
    assert_has_tip(&[], &tubh, Lun, &["atuBat"]);
    assert_has_ta(&[], &tubh, Lun, &["atoBizwa"]);

    let srans = d("sransu~\\", Bhvadi);
    assert_has_tip(&[], &srans, Lun, &["asrasat"]);
    assert_has_ta(&[], &srans, Lun, &["asraMsizwa"]);

    let srambh = d("sranBu~\\", Bhvadi);
    assert_has_tip(&[], &srambh, Lun, &["asraBat"]);
    assert_has_ta(&[], &srambh, Lun, &["asramBizwa"]);

    let vrt = d("vftu~\\", Bhvadi);
    assert_has_ta(&[], &vrt, Lat, &["vartate"]);
}

#[test]
fn skip_sk_2347() {}

#[test]
fn sk_2348() {
    let vrt = d("vftu~\\", Bhvadi);
    assert_has_tip(&[], &vrt, Lrt, &["vartsyati"]);
    assert_has_ta(&[], &vrt, Lrt, &["vartizyate"]);
    assert_has_tip(&[], &vrt, Lun, &["avftat"]);
    assert_has_ta(&[], &vrt, Lun, &["avartizwa"]);
    assert_has_tip(&[], &vrt, Lrn, &["avartsyat"]);

    let syand = d("syandU~\\", Bhvadi);
    assert_has_ta(&[], &syand, Lat, &["syandate"]);
    assert_has_ta(&[], &syand, Lit, &["sasyande"]);
    assert_has_thaas(&[], &syand, Lit, &["sasyandize", "sasyantse"]);
    assert_has_dhvam(
        &[],
        &syand,
        Lit,
        &["sasyandiDve", "sasyanDve", "sasyandDve"],
    );
    assert_has_ta(&[], &syand, Lut, &["syanditA", "syantA", "syanttA"]);
    assert_has_tip(&[], &syand, Lrt, &["syantsyati"]);
    assert_has_ta(&[], &syand, Lrt, &["syandizyate", "syantsyate"]);
    assert_has_ta(&[], &syand, AshirLin, &["syandizIzwa", "syantsIzwa"]);
    assert_has_tip(&[], &syand, Lun, &["asyadat"]);
    assert_has_ta(&[], &syand, Lun, &["asyandizwa", "asyanta", "asyantta"]);
    // asyandizAtAm is justified by asandizwa above.
    assert_has_aataam(&[], &syand, Lun, &["asyantsAtAm", "asyandizAtAm"]);
    assert_has_tip(&[], &syand, Lrn, &["asyantsyat"]);
    assert_has_ta(&[], &syand, Lrn, &["asyandizyata", "asyantsyata"]);
}

#[test]
fn sk_2349() {
    let syand = d("syandU~\\", Bhvadi);
    assert_has_ta(&["anu"], &syand, Lat, &["anuzyandate", "anusyandate"]);
}

#[test]
fn sk_2350() {
    let klp = d("kfpU~\\", Bhvadi);
    assert_has_ta(&[], &klp, Lat, &["kalpate"]);
    assert_has_ta(&[], &klp, Lit, &["cakxpe"]);
    assert_has_thaas(&[], &klp, Lit, &["cakxpize", "cakxpse"]);
}

#[test]
fn skip_sk_2351() {}

#[test]
fn sk_2352() {
    let klp = d("kfpU~\\", Bhvadi);
    assert_has_sip(&[], &klp, Lut, &["kalptAsi"]);
    assert_has_tha(&[], &klp, Lut, &["kalptAsTa"]);
    assert_has_thaas(&[], &klp, Lut, &["kalpitAse", "kalptAse"]);
    assert_has_tip(&[], &klp, Lrt, &["kalpsyati"]);
    assert_has_ta(&[], &klp, Lrt, &["kalpizyate", "kalpsyate"]);
    assert_has_ta(&[], &klp, AshirLin, &["kalpizIzwa", "kxpsIzwa"]);
    assert_has_tip(&[], &klp, Lun, &["akxpat"]);
    assert_has_ta(&[], &klp, Lun, &["akalpizwa", "akxpta"]);
    assert_has_tip(&[], &klp, Lrn, &["akalpsyat"]);
    assert_has_ta(&[], &klp, Lrn, &["akalpizyata", "akalpsyata"]);

    let ghatt = d_ghatadi("Gawa~\\", Bhvadi);
    assert_has_ta(&[], &ghatt, Lat, &["Gawate"]);
    assert_has_ta(&[], &ghatt, Lit, &["jaGawe"]);
    assert_has_tip(&[], &nic(&ghatt), Lat, &["Gawayati"]);
    assert_has_tip(&["vi"], &nic(&ghatt), Lat, &["viGawayati"]);

    // TODO: why is this here?
    let shru = d("Sru\\", Bhvadi);
    assert_has_sip(&[], &shru, Lot, &["SfRu", "SfRutAt"]);

    let vyath = d("vyaTa~\\", Bhvadi);
    assert_has_ta(&[], &vyath, Lat, &["vyaTate"]);
}

// TODO: review carefully for missing forms.
#[test]
fn sk_2353() {
    // The Kaumudi defines Gaw-Adi in a way that is inconsistent with dhatupatha.tsv. To allow
    // these tests to function, explicitly mark dhatus as Gaw-Adi with `d_ghatadi`:
    let d_g = d_ghatadi;

    let vyath = d_g("vyaTa~\\", Bhvadi);
    assert_has_ta(&[], &vyath, Lit, &["vivyaTe"]);

    let prath = d_g("praTa~\\", Bhvadi);
    assert_has_ta(&[], &prath, Lit, &["papraTe"]);

    let pras = d_g("prasa~\\", Bhvadi);
    assert_has_ta(&[], &pras, Lit, &["paprase"]);

    let kshanj = d_g("kzaji~\\", Bhvadi);
    assert_has_ta_k(&[], &nic(&kshanj), Lun, &["akzaYji", "akzAYji"]);
    assert_has_krdanta(&[], &nic(&kshanj), Krt::Ramul, &["kzaYjam", "kzAYjam"]);

    let jvar = d_g("jvara~", Bhvadi);
    assert_has_tip(&[], &jvar, Lat, &["jvarati"]);

    let gad = d_g("gaqa~", Bhvadi);
    assert_has_tip(&[], &gad, Lat, &["gaqati"]);

    let hed = d_g("heqa~", Bhvadi);
    assert_has_tip(&[], &hed, Lat, &["heqati"]);
    assert_has_tip(&[], &hed, Lit, &["jiheqa"]);
    assert_has_tip(&[], &nic(&hed), Lat, &["hiqayati"]);
    assert_has_ta_k(&[], &nic(&hed), Lun, &["ahiqi", "ahIqi"]);

    // This dhatu is outside of ghaT-Adi, so it doesn't receive mittva.
    let hed_anadare = d("heqf~\\", Bhvadi);
    assert_has_tip(&[], &nic(&hed_anadare), Lat, &["heqayati"]);

    let stak = d_g("zwaka~", Bhvadi);
    assert_has_tip(&[], &stak, Lat, &["stakati"]);

    let kakh = d_g("kaKe~", Bhvadi);
    assert_has_tip(&[], &kakh, Lun, &["akaKIt"]);

    let kan = d_g("kaRa~", Bhvadi);
    assert_has_tip(&[], &kan, Lit, &["cakARa"]);

    let ran = d_g("raRa~", Bhvadi);
    assert_has_tip(&[], &ran, Lit, &["rarARa"]);

    let krath = d_g("kraTa~", Bhvadi);
    assert_has_tip(&[], &nic(&krath), Lat, &["krATayati"]);
    assert_has_ta_k(&[], &nic(&krath), Lun, &["akraTi", "akrATi"]);
    assert_has_krdanta(&[], &nic(&krath), Krt::Ramul, &["kraTam", "krATam"]);

    let van = d_g("vana~", Bhvadi);
    assert_has_tip(&["pra"], &nic(&van), Lat, &["pravanayati"]);

    let jval = d_g("jvala~", Bhvadi);
    assert_has_tip(&["pra"], &nic(&jval), Lat, &["prajvalayati"]);

    let hval = d_g("hvala~", Bhvadi);
    assert_has_tip(&["pra"], &nic(&hval), Lat, &["prahvalayati"]);

    let hmal = d_g("hmala~", Bhvadi);
    assert_has_tip(&["pra"], &nic(&hmal), Lat, &["prahmalayati"]);

    let dr = d_g("dF", Bhvadi);
    assert_has_tip(&[], &nic(&dr), Lat, &["darayati"]);

    // nayAdanyatra nArayati -- but, in our dhatupatha, nF only ever has the meaning "naye"! As a
    // quick HACK, check the nF outside of bhvAdi-gana.
    let nr = d("nF", Kryadi);
    assert_has_tip(&[], &nic(&nr), Lat, &["nArayati"]);

    let shra = d_g("SrA", Bhvadi);
    assert_has_tip(&[], &nic(&shra), Lat, &["Srapayati"]);

    // pAkAd anyatra SrApayati -- but, in our dhatupatha, SrA only ever has the meaning "pAke"! As
    // a quick HACK, check the SrA outside of bhvAdi-gana.
    let shra_fake = d("SrA\\", Adadi);
    assert_has_tip(&[], &nic(&shra_fake), Lat, &["SrApayati"]);

    let cal = d_g("cala~", Bhvadi);
    assert_has_tip(&[], &nic(&cal), Lat, &["calayati"]);

    let chad = d_g("CadiH", Bhvadi);
    assert_has_tip(&[], &nic(&chad), Lat, &["Cadayati"]);

    let lad = d_g("laqa~", Bhvadi);
    assert_has_tip(&[], &nic(&lad), Lat, &["laqayati"]);

    let mad = d_g("madI~", Bhvadi);
    assert_has_tip(&[], &nic(&mad), Lat, &["madayati"]);

    let dhvan = d_g("Dvana~", Bhvadi);
    assert_has_tip(&[], &nic(&dhvan), Lat, &["Dvanayati"]);

    let dal = d("dala~", Bhvadi);
    assert_has_tip(&[], &nic(&dal), Lat, &["dalayati"]);

    let val = d("vala~\\", Bhvadi);
    assert_has_tip(&[], &nic(&val), Lat, &["valayati"]);

    let skhal = d("sKala~", Bhvadi);
    assert_has_tip(&[], &nic(&skhal), Lat, &["sKalayati"]);

    let trap = d("trapU~\\z", Bhvadi);
    assert_has_tip(&[], &nic(&trap), Lat, &["trapayati"]);

    let kshai = d("kzE\\", Bhvadi);
    assert_has_tip(&[], &nic(&kshai), Lat, &["kzapayati"]);

    let svan = d_g("svana~", Bhvadi);
    assert_has_tip(&[], &nic(&svan), Lat, &["svanayati"]);

    let jr = d("jFz", Divadi);
    assert_has_tip(&[], &nic(&jr), Lat, &["jarayati"]);

    let jrnati = d("jF", Kryadi);
    assert_has_tip(&[], &nic(&jrnati), Lat, &["jArayati"]);

    let jval = d_g("jvala~", Bhvadi);
    assert_has_tip(&[], &nic(&jval), Lat, &["jvalayati", "jvAlayati"]);
    assert_has_tip(&["pra"], &nic(&jval), Lat, &["prajvalayati"]);

    let kam = d("kamu~\\", Bhvadi);
    assert_has_ta(&[], &kam, Lat, &["kAmayate"]);

    let am = d("ama~", Bhvadi);
    assert_has_tip(&[], &nic(&am), Lat, &["Amayati"]);

    let cam = d("camu~", Bhvadi);
    assert_has_tip(&[], &nic(&cam), Lat, &["cAmayati"]);

    let sham = d_g("Samo~", Bhvadi);
    assert_has_tip(&["ni"], &nic(&sham), Lat, &["niSAmayati", "niSamayati"]);
    assert_has_krdanta(&["ni"], &nic(&sham), Krt::ktvA, &["niSAmya", "niSamayya"]);

    let yam_pariveshane = d_g("yama~", Bhvadi);
    assert_has_tip(&["AN"], &nic(&yam_pariveshane), Lat, &["AyAmayati"]);

    let yam_uparame = d_g("ya\\ma~", Bhvadi);
    assert_has_tip(&[], &nic(&yam_uparame), Lat, &["yamayati"]);

    let skhad = d_g("sKadi~\\r", Bhvadi);
    assert_has_tip(&["ava"], &nic(&skhad), Lat, &["avasKAdayati"]);
    assert_has_tip(&["pari"], &nic(&skhad), Lat, &["parisKAdayati"]);

    assert_has_tip(
        &["apa"],
        &nic(&skhad),
        Lat,
        &["apasKAdayati", "apasKadayati"],
    );
}

#[test]
fn sk_2354() {
    // paPaRiTa is in KV 6.4.125.
    let phan = d_ghatadi("PaRa~", Bhvadi);
    assert_has_tas(&[], &phan, Lit, &["PeRatuH", "paPaRatuH"]);
    assert_has_jhi(&[], &phan, Lit, &["PeRuH", "paPaRuH"]);
    assert_has_sip(&[], &phan, Lit, &["PeRiTa", "paPaRiTa"]);
    assert_has_tip(&[], &nic(&phan), Lat, &["PaRayati"]);

    let raj = d("rAjf~^", Bhvadi);
    assert_has_tip(&[], &raj, Lat, &["rAjati"]);
    assert_has_ta(&[], &raj, Lat, &["rAjate"]);
    assert_has_tas(&[], &raj, Lit, &["rejatuH", "rarAjatuH"]);
    assert_has_ta(&[], &raj, Lit, &["reje", "rarAje"]);

    let bhraj = d("wuBrAjf~\\", Bhvadi);
    assert_has_ta(&[], &bhraj, Lit, &["Breje", "baBrAje"]);

    let bhrash = d("wuBrASf~\\", Bhvadi);
    assert_has_ta(&[], &bhrash, Lat, &["BrASyate", "BrASate"]);
    assert_has_ta(&[], &bhrash, Lit, &["BreSe", "baBrASe"]);

    let bhlash = d("wuBlASf~\\", Bhvadi);
    assert_has_ta(&[], &bhlash, Lat, &["BlASyate", "BlASate"]);
    assert_has_ta(&[], &bhlash, Lit, &["BleSe", "baBlASe"]);

    let syam = d("syamu~", Bhvadi);
    assert_has_tas(&[], &syam, Lit, &["syematuH", "sasyamatuH"]);
    assert_has_tip(&[], &syam, Lun, &["asyamIt"]);

    let svan = d("svana~", Bhvadi);
    assert_has_tas(&[], &svan, Lit, &["svenatuH", "sasvanatuH"]);
    assert_has_tip(&[], &svan, Lun, &["asvAnIt", "asvanIt"]);

    // visvanati and avasvanati are from KV 8.3.69.
    assert_has_tip(&["vi"], &svan, Lat, &["vizvaRati", "visvanati"]);
    assert_has_tip(&["ava"], &svan, Lat, &["avazvaRati", "avasvanati"]);

    let dhvan = d("Dvana~", Bhvadi);
    assert_has_tas(&[], &dhvan, Lit, &["daDvanatuH"]);

    let sam = d("zama~", Bhvadi);
    assert_has_tip(&[], &sam, Lit, &["sasAma"]);

    let stam = d("zwama~", Bhvadi);
    assert_has_tip(&[], &stam, Lit, &["tastAma"]);

    let jval = d("jvala~", Bhvadi);
    assert_has_tip(&[], &jval, Lun, &["ajvAlIt"]);

    let bal = d("bala~", Bhvadi);
    assert_has_tip(&[], &bal, Lat, &["balati"]);
    assert_has_tas(&[], &bal, Lit, &["belatuH"]);
    assert_has_jhi(&[], &bal, Lit, &["beluH"]);

    let pul = d("pula~", Bhvadi);
    assert_has_tip(&[], &pul, Lat, &["polati"]);

    let kul = d("kula~", Bhvadi);
    assert_has_tip(&[], &kul, Lat, &["kolati"]);
    assert_has_tip(&[], &kul, Lit, &["cukola"]);

    let shal = d("Sala~", Bhvadi);
    assert_has_tip(&[], &shal, Lit, &["SaSAla"]);

    let hul = d("hula~", Bhvadi);
    assert_has_tip(&[], &hul, Lit, &["juhola"]);

    let pat = d("patx~", Bhvadi);
    assert_has_tip(&[], &pat, Lit, &["papAta"]);
    assert_has_tas(&[], &pat, Lit, &["petatuH"]);
    assert_has_tip(&[], &pat, Lut, &["patitA"]);
}

#[test]
fn sk_2355() {
    let pat = d("patx~", Bhvadi);
    assert_has_tip(&[], &pat, Lun, &["apaptat"]);
    assert_has_tip(&["pra", "ni"], &pat, Lun, &["praRyapaptat"]);

    let kvath = d("kvaTe~", Bhvadi);
    assert_has_tip(&[], &kvath, Lat, &["kvaTati"]);
    assert_has_tip(&[], &kvath, Lit, &["cakvATa"]);
    assert_has_tip(&[], &kvath, Lun, &["akvaTIt"]);

    let path = d("paTe~", Bhvadi);
    assert_has_tip(&[], &path, Lun, &["apaTIt"]);

    let math = d("maTe~", Bhvadi);
    assert_has_tas(&[], &math, Lit, &["meTatuH"]);
    assert_has_tip(&[], &math, Lun, &["amaTIt"]);

    let van = d("wuvama~", Bhvadi);
    assert_has_tip(&[], &van, Lit, &["vavAma"]);
    assert_has_tas(&[], &van, Lit, &["vavamatuH"]);

    let bhram = d("Bramu~", Bhvadi);
    assert_has_tip(&[], &bhram, Lat, &["Bramyati", "Bramati"]);
}

#[test]
fn sk_2356() {
    let bhram = d("Bramu~", Bhvadi);
    assert_has_tas(&[], &bhram, Lit, &["BrematuH", "baBramatuH"]);
    assert_has_tip(&[], &bhram, Lun, &["aBramIt"]);

    let kshar = d("kzara~", Bhvadi);
    assert_has_tip(&[], &kshar, Lun, &["akzArIt"]);

    // soQA is from SK 2862.
    let sah = d("zaha~\\", Bhvadi);
    assert_has_ta(&["pari"], &sah, Lat, &["parizahate"]);
    assert_has_ta(&[], &sah, Lit, &["sehe"]);
    assert_has_ta(&[], &sah, Lut, &["sahitA", "soQA"]);
}

#[test]
fn skip_sk_2357() {}

#[test]
fn sk_2358() {
    // sahitA is from SK 2356.
    let sah = d("zaha~\\", Bhvadi);
    assert_has_ta(&["pari"], &sah, Lut, &["parisoQA", "parizahitA"]);
}

#[test]
fn sk_2359() {
    let sah = d("zaha~\\", Bhvadi);
    assert_has_ta(&["pari"], &sah, Lan, &["paryasahata", "paryazahata"]);

    let ram = d("ra\\mu~\\", Bhvadi);
    assert_has_ta(&[], &ram, Lit, &["reme"]);
    assert_has_thaas(&[], &ram, Lit, &["remize"]);
    assert_has_ta(&[], &ram, Lut, &["rantA"]);
    assert_has_ta(&[], &ram, Lrt, &["raMsyate"]);
    assert_has_ta(&[], &ram, AshirLin, &["raMsIzwa"]);
    assert_has_ta(&[], &ram, Lun, &["araMsta"]);
}

#[test]
fn sk_2360() {
    let sad = d("za\\dx~", Bhvadi);
    assert_has_tip(&[], &sad, Lat, &["sIdati"]);
    assert_has_tip(&[], &sad, Lit, &["sasAda"]);
    assert_has_tas(&[], &sad, Lit, &["sedatuH"]);
    assert_has_sip(&[], &sad, Lit, &["sediTa", "sasatTa"]);
    assert_has_tip(&[], &sad, Lut, &["sattA"]);
    assert_has_tip(&[], &sad, Lrt, &["satsyati"]);
    assert_has_tip(&[], &sad, Lun, &["asadat"]);

    assert_has_tip(&["ni"], &sad, Lat, &["nizIdati"]);
    assert_has_tip(&["ni"], &sad, Lan, &["nyazIdat"]);
}

#[test]
fn sk_2361() {
    let sad = d("za\\dx~", Bhvadi);
    assert_has_tip(&["ni"], &sad, Lit, &["nizasAda"]);
    assert_has_tas(&["ni"], &sad, Lit, &["nizedatuH"]);
}

#[test]
fn sk_2362() {
    let shad = d("Sa\\dx~", Bhvadi);
    assert_has_ta(&[], &shad, Lat, &["SIyate"]);
    assert_has_tip(&[], &shad, Lit, &["SaSAda"]);
    assert_has_tas(&[], &shad, Lit, &["SedatuH"]);
    assert_has_sip(&[], &shad, Lit, &["SediTa", "SaSatTa"]);
    assert_has_tip(&[], &shad, Lut, &["SattA"]);
    assert_has_tip(&[], &shad, Lun, &["aSadat"]);

    let krush = d("kru\\Sa~", Bhvadi);
    assert_has_tip(&[], &krush, Lat, &["kroSati"]);
    assert_has_tip(&[], &krush, Lut, &["krozwA"]);
    assert_has_tip(&[], &krush, Lun, &["akrukzat"]);

    let kuc = d("kuca~", Bhvadi);
    assert_has_tip(&[], &kuc, Lat, &["kocati"]);
    assert_has_tip(&[], &kuc, Lit, &["cukoca"]);

    let budh = d("buDa~", Bhvadi);
    assert_has_tip(&[], &budh, Lat, &["boDati"]);
    assert_has_tip(&[], &budh, Lut, &["boDitA"]);
    assert_has_tip(&[], &budh, Lrt, &["boDizyati"]);

    let ruh = d("ru\\ha~", Bhvadi);
    assert_has_tip(&[], &ruh, Lat, &["rohati"]);
    assert_has_tip(&[], &ruh, Lit, &["ruroha"]);
    assert_has_sip(&[], &ruh, Lit, &["rurohiTa"]);
    assert_has_tip(&[], &ruh, Lut, &["roQA"]);
    assert_has_tip(&[], &ruh, Lrt, &["rokzyati"]);
    assert_has_tip(&[], &ruh, Lun, &["arukzat"]);

    let kas = d("kasa~", Bhvadi);
    assert_has_tip(&[], &kas, Lun, &["akAsIt", "akasIt"]);

    let hikk = d("hikka~^", Bhvadi);
    assert_has_tip(&[], &hikk, Lat, &["hikkati"]);
    assert_has_ta(&[], &hikk, Lat, &["hikkate"]);

    let anc = d("ancu~^", Bhvadi);
    assert_has_tip(&[], &anc, Lat, &["aYcati"]);
    assert_has_ta(&[], &anc, Lat, &["aYcate"]);

    let yac = d("wuyAcf~^", Bhvadi);
    assert_has_tip(&[], &yac, Lat, &["yAcati"]);
    assert_has_ta(&[], &yac, Lat, &["yAcate"]);

    let ret = d("rewf~^", Bhvadi);
    assert_has_tip(&[], &ret, Lat, &["rewati"]);
    assert_has_ta(&[], &ret, Lat, &["rewate"]);

    // TODO: text has aYcAtIt? surely acatIt?
    let cat = d("cate~^", Bhvadi);
    assert_has_tip(&[], &cat, Lit, &["cacAta"]);
    assert_has_ta(&[], &cat, Lit, &["cete"]);
    assert_has_tip(&[], &cat, Lun, &["acatIt"]);

    let cad = d("cade~^", Bhvadi);
    assert_has_tip(&[], &cad, Lit, &["cacAda"]);
    assert_has_ta(&[], &cad, Lit, &["cede"]);
    assert_has_tip(&[], &cad, Lun, &["acadIt"]);

    let proth = d("proTf~^", Bhvadi);
    assert_has_tip(&[], &proth, Lit, &["puproTa"]);
    assert_has_ta(&[], &proth, Lit, &["puproTe"]);

    let mid = d("midf~^", Bhvadi);
    assert_has_tip(&[], &mid, Lit, &["mimeda"]);
    assert_has_ta(&[], &mid, Lit, &["mimide"]);

    let med = d("medf~^", Bhvadi);
    assert_has_ta(&[], &med, Lit, &["mimede"]);

    let mith = d("miTf~^", Bhvadi);
    assert_has_tip(&[], &mith, Lit, &["mimeTa"]);

    let midh = d("miDf~^", Bhvadi);
    assert_has_tip(&[], &midh, Lat, &["meDati"]);
    assert_has_tip(&[], &midh, Lit, &["mimeDa"]);

    let nid = d("Ridf~^", Bhvadi);
    assert_has_tip(&[], &nid, Lit, &["nineda"]);
    assert_has_tas(&[], &nid, Lit, &["ninidatuH"]);
    // TODO: text has ninida?
    assert_has_jhi(&[], &nid, Lit, &["niniduH"]);

    let ned = d("Redf~^", Bhvadi);
    assert_has_ta(&[], &ned, Lit, &["ninede"]);

    let shrdh = d("SfDu~^", Bhvadi);
    assert_has_tip(&[], &shrdh, Lat, &["SarDati"]);
    assert_has_ta(&[], &shrdh, Lat, &["SarDate"]);
    assert_has_tip(&[], &shrdh, Lut, &["SarDitA"]);

    let mrdh = d("mfDu~^", Bhvadi);
    assert_has_tip(&[], &mrdh, Lat, &["marDati"]);
    assert_has_ta(&[], &mrdh, Lat, &["marDate"]);

    let budh = d("buDi~^r", Bhvadi);
    assert_has_tip(&[], &budh, Lat, &["boDati"]);
    // TODO: text has boDete?
    assert_has_ta(&[], &budh, Lat, &["boDate"]);
    assert_has_tip(&[], &budh, Lun, &["abuDat", "aboDIt"]);
    assert_has_ta(&[], &budh, Lun, &["aboDizwa"]);

    let bund = d("u~bundi~^r", Bhvadi);
    assert_has_ta(&[], &bund, Lit, &["bubunde"]);
    assert_has_tip(&[], &bund, Lun, &["abudat", "abundIt"]);

    let ven = d("veRf~^", Bhvadi);
    assert_has_tip(&[], &ven, Lat, &["veRati"]);
    assert_has_ta(&[], &ven, Lat, &["veRate"]);

    let khan = d("Kanu~^", Bhvadi);
    assert_has_tip(&[], &khan, Lat, &["Kanati"]);
    assert_has_ta(&[], &khan, Lat, &["Kanate"]);
}

#[test]
fn sk_2363() {
    let khan = d("Kanu~^", Bhvadi);
    assert_has_tas(&[], &khan, Lit, &["caKnatuH"]);
    assert_has_tip(&[], &khan, AshirLin, &["KAyAt", "KanyAt"]);

    let civ = d("cIvf~^", Bhvadi);
    assert_has_tip(&[], &civ, Lit, &["cicIva"]);
    assert_has_ta(&[], &civ, Lit, &["cicIve"]);

    let vyay = d("vyaya~^", Bhvadi);
    assert_has_tip(&[], &vyay, Lun, &["avyayIt"]);

    let dash = d("dASf~^", Bhvadi);
    assert_has_tip(&[], &dash, Lit, &["dadASa"]);
    assert_has_ta(&[], &dash, Lit, &["dadASe"]);

    let bhesh = d("Bezf~^", Bhvadi);
    assert_has_tip(&[], &bhesh, Lat, &["Bezati"]);
    assert_has_ta(&[], &bhesh, Lat, &["Bezate"]);

    let as_ = d("asa~^", Bhvadi);
    assert_has_tip(&[], &as_, Lat, &["asati"]);
    assert_has_ta(&[], &as_, Lat, &["asate"]);
    assert_has_tip(&[], &as_, Lit, &["Asa"]);
    assert_has_ta(&[], &as_, Lit, &["Ase"]);

    let spash = d("spaSa~^", Bhvadi);
    assert_has_tip(&[], &spash, Lat, &["spaSati"]);
    assert_has_ta(&[], &spash, Lat, &["spaSate"]);

    let lash = d("laza~^", Bhvadi);
    assert_has_tip(&[], &lash, Lat, &["lazyati", "lazati"]);
    assert_has_ta(&[], &lash, Lit, &["leze"]);

    let chash = d("Caza~^", Bhvadi);
    assert_has_tas(&[], &chash, Lit, &["cacCazatuH"]);
    assert_has_ta(&[], &chash, Lit, &["cacCaze"]);
}

#[test]
fn sk_2364() {
    let ruh = d("guhU~^", Bhvadi);
    assert_has_tip(&[], &ruh, Lat, &["gUhati"]);
    assert_has_ta(&[], &ruh, Lat, &["gUhate"]);
    assert_has_tip(&[], &ruh, Lut, &["gUhitA", "goQA"]);
    assert_has_tip(&[], &ruh, Lrt, &["gUhizyati", "Gokzyati"]);
    assert_has_tip(&[], &ruh, VidhiLin, &["gUhet"]);
    assert_has_tip(&[], &ruh, AshirLin, &["guhyAt"]);
    assert_has_tip(&[], &ruh, Lun, &["agUhIt", "aGukzat"]);
}

#[test]
fn sk_2365() {
    // TODO: double-check these.
    let guh = d("guhU~^", Bhvadi);
    assert_has_ta(&[], &guh, Lun, &["agUQa", "aGukzata", "agUhizwa"]);
    assert_has_aataam(&[], &guh, Lun, &["aGukzAtAm", "agUhizAtAm"]);
    assert_has_jha(&[], &guh, Lun, &["aGukzanta", "agUhizata"]);
    assert_has_vahi(&[], &guh, Lun, &["aguhvahi", "aGukzAvahi", "agUhizvahi"]);
    assert_has_mahin(&[], &guh, Lun, &["aGukzAmahi", "agUhizmahi"]);

    let shri = d("SriY", Bhvadi);
    assert_has_tip(&[], &shri, Lat, &["Srayati"]);
    assert_has_ta(&[], &shri, Lat, &["Srayate"]);
    assert_has_tas(&[], &shri, Lit, &["SiSriyatuH"]);
    assert_has_tip(&[], &shri, Lut, &["SrayitA"]);
    assert_has_tip(&[], &shri, Lun, &["aSiSriyat"]);

    let bhr = d("Bf\\Y", Bhvadi);
    assert_has_tip(&[], &bhr, Lat, &["Barati"]);
    assert_has_tip(&[], &bhr, Lit, &["baBAra"]);
    assert_has_tas(&[], &bhr, Lit, &["baBratuH"]);
    assert_has_sip(&[], &bhr, Lit, &["baBarTa"]);
    assert_has_vas(&[], &bhr, Lit, &["baBfva"]);
    assert_has_thaas(&[], &bhr, Lit, &["baBfze"]);
    assert_has_tip(&[], &bhr, Lut, &["BartA"]);
}

#[test]
fn sk_2366() {
    let bhr = d("Bf\\Y", Bhvadi);
    assert_has_tip(&[], &bhr, Lrt, &["Barizyati"]);
}

#[test]
fn sk_2367() {
    let bhr = d("Bf\\Y", Bhvadi);
    assert_has_tip(&[], &bhr, AshirLin, &["BriyAt"]);
}

#[test]
fn sk_2368() {
    let bhr = d("Bf\\Y", Bhvadi);
    assert_has_ta(&[], &bhr, AshirLin, &["BfzIzwa"]);
    assert_has_aataam(&[], &bhr, AshirLin, &["BfzIyAstAm"]);
    assert_has_tip(&[], &bhr, Lun, &["aBArzIt"]);
    assert_has_tas(&[], &bhr, Lun, &["aBArzwAm"]);
    assert_has_jhi(&[], &bhr, Lun, &["aBArzuH"]);
}

#[test]
fn sk_2369() {
    let bhr = d("Bf\\Y", Bhvadi);
    assert_has_ta(&[], &bhr, Lun, &["aBfta"]);
    assert_has_aataam(&[], &bhr, Lun, &["aBfzAtAm"]);

    let hr = d("hf\\Y", Bhvadi);
    assert_has_sip(&[], &hr, Lit, &["jaharTa"]);
    assert_has_vas(&[], &hr, Lit, &["jahriva"]);
    assert_has_thaas(&[], &hr, Lit, &["jahrize"]);
    assert_has_tip(&[], &hr, Lut, &["hartA"]);

    let dhr = d("Df\\Y", Bhvadi);
    assert_has_tip(&[], &dhr, Lat, &["Darati"]);
    assert_has_tip(&[], &dhr, Lun, &["aDArzIt"]);

    let ni = d("RI\\Y", Bhvadi);
    assert_has_sip(&[], &ni, Lit, &["ninayiTa", "nineTa"]);

    let dhe = d("De\\w", Bhvadi);
    assert_has_tip(&[], &dhe, Lat, &["Dayati"]);
}

#[test]
fn skip_sk_2370() {}

#[test]
fn sk_2371() {
    let dhe = d("De\\w", Bhvadi);
    assert_has_tip(&[], &dhe, Lit, &["daDO"]);
}

#[test]
fn sk_2372() {
    let dhe = d("De\\w", Bhvadi);
    assert_has_tas(&[], &dhe, Lit, &["daDatuH"]);
    assert_has_jhi(&[], &dhe, Lit, &["daDuH"]);
    assert_has_sip(&[], &dhe, Lit, &["daDiTa", "daDATa"]);
    assert_has_vas(&[], &dhe, Lit, &["daDiva"]);
    assert_has_tip(&[], &dhe, Lut, &["DAtA"]);
}

#[test]
fn skip_sk_2373() {}

#[test]
fn sk_2374() {
    let dhe = d("De\\w", Bhvadi);
    assert_has_tip(&[], &dhe, AshirLin, &["DeyAt"]);
    assert_has_tas(&[], &dhe, AshirLin, &["DeyAstAm"]);
    assert_has_jhi(&[], &dhe, AshirLin, &["DeyAsuH"]);
}

#[test]
fn sk_2375() {
    // aDAt and aDAtAm are from SK 2376.
    // aDAsIt etc. are from SK 2377.
    let dhe = d("De\\w", Bhvadi);
    assert_has_tip(&[], &dhe, Lun, &["aDAt", "adaDat", "aDAsIt"]);
    assert_has_tas(&[], &dhe, Lun, &["aDAtAm", "adaDatAm", "aDAsizwAm"]);
}

#[test]
fn sk_2376() {
    // adaDat etc. are from SK 2375.
    // aDAsIt etc. are from SK 2377.
    let dhe = d("De\\w", Bhvadi);
    assert_has_tip(&[], &dhe, Lun, &["aDAt", "adaDat", "aDAsIt"]);
    assert_has_tas(&[], &dhe, Lun, &["aDAtAm", "adaDatAm", "aDAsizwAm"]);
    assert_has_jhi(&[], &dhe, Lun, &["aDuH", "adaDan", "aDAsizuH"]);
}

#[test]
fn sk_2377() {
    // adaDat etc. are from SK 2375.
    // aDAt etc. are from SK 2376.
    let dhe = d("De\\w", Bhvadi);
    assert_has_tip(&[], &dhe, Lun, &["aDAt", "adaDat", "aDAsIt"]);
    assert_has_tas(&[], &dhe, Lun, &["aDAtAm", "adaDatAm", "aDAsizwAm"]);
    assert_has_jhi(&[], &dhe, Lun, &["aDuH", "adaDan", "aDAsizuH"]);

    let glai = d("glE\\", Bhvadi);
    assert_has_tip(&[], &glai, Lat, &["glAyati"]);
    assert_has_tip(&[], &glai, Lit, &["jaglO"]);
    assert_has_sip(&[], &glai, Lit, &["jagliTa", "jaglATa"]);
}

#[test]
fn sk_2378() {
    let glai = d("glE\\", Bhvadi);
    assert_has_tip(&[], &glai, AshirLin, &["glAyAt", "gleyAt"]);
    assert_has_tip(&[], &glai, Lun, &["aglAsIt"]);

    let mlai = d("mlE\\", Bhvadi);
    assert_has_tip(&[], &mlai, Lat, &["mlAyati"]);

    let styai = d("zwyE\\", Bhvadi);
    assert_has_tip(&[], &styai, Lat, &["styAyati"]);

    let kshai = d("kzE\\", Bhvadi);
    assert_has_tip(&[], &kshai, Lat, &["kzAyati"]);

    let jai = d("jE\\", Bhvadi);
    assert_has_tip(&[], &jai, Lit, &["jajO"]);

    let sai = d("zE\\", Bhvadi);
    assert_has_tip(&[], &sai, Lit, &["sasO"]);
    assert_has_tip(&[], &sai, Lut, &["sAtA"]);
    assert_has_tip(&[], &sai, AshirLin, &["sAyAt"]);
    assert_has_tip(&[], &sai, Lun, &["asAsIt"]);

    // TODO: SK has gEyAt?
    let gai = d("gE\\", Bhvadi);
    assert_has_tip(&[], &gai, AshirLin, &["geyAt"]);
    assert_has_tip(&[], &gai, Lun, &["agAsIt"]);

    let pai = d("pE\\", Bhvadi);
    assert_has_tip(&[], &pai, AshirLin, &["pAyAt"]);
    assert_has_tip(&[], &pai, Lun, &["apAsIt"]);

    let stai = d("zwE\\", Bhvadi);
    assert_has_tip(&[], &stai, Lat, &["stAyati"]);

    let snai = d("zRE\\", Bhvadi);
    assert_has_tip(&[], &snai, Lat, &["snAyati"]);

    let dai = d("dE\\p", Bhvadi);
    assert_has_tip(&[], &dai, Lat, &["dAyati"]);
    assert_has_tip(&[], &dai, AshirLin, &["dAyAt"]);
    assert_has_tip(&[], &dai, Lun, &["adAsIt"]);

    let pa = d("pA\\", Bhvadi);
    assert_has_tip(&[], &pa, Lat, &["pibati"]);
    assert_has_tip(&[], &pa, AshirLin, &["peyAt"]);
    assert_has_tip(&[], &pa, Lun, &["apAt"]);

    let ghra = d("GrA\\", Bhvadi);
    assert_has_tip(&[], &ghra, Lat, &["jiGrati"]);
    assert_has_tip(&[], &ghra, AshirLin, &["GrAyAt", "GreyAt"]);
    assert_has_tip(&[], &ghra, Lun, &["aGrAsIt", "aGrAt"]);

    let dhma = d("DmA\\", Bhvadi);
    assert_has_tip(&[], &dhma, Lat, &["Damati"]);

    let stha = d("zWA\\", Bhvadi);
    assert_has_tip(&[], &stha, Lat, &["tizWati"]);
    assert_has_tip(&["aDi"], &stha, Lit, &["aDitazWO"]);
    assert_has_tip(&["aDi"], &stha, Lut, &["aDizWAtA"]);
    assert_has_tip(&[], &stha, AshirLin, &["sTeyAt"]);

    let mna = d("mnA\\", Bhvadi);
    assert_has_tip(&[], &mna, Lat, &["manati"]);

    let da = d("dA\\R", Bhvadi);
    assert_has_tip(&["pra", "ni"], &da, Lat, &["praRiyacCati"]);
    assert_has_tip(&[], &da, AshirLin, &["deyAt"]);
    assert_has_tip(&[], &da, Lun, &["adAt"]);

    let hvr = d("hvf\\", Bhvadi);
    assert_has_tip(&[], &hvr, Lat, &["hvarati"]);
}

#[test]
fn sk_2379() {
    let hvr = d("hvf\\", Bhvadi);
    assert_has_tip(&[], &hvr, Lit, &["jahvAra"]);
    assert_has_tas(&[], &hvr, Lit, &["jahvaratuH"]);
    assert_has_jhi(&[], &hvr, Lit, &["jahvaruH"]);
    assert_has_sip(&[], &hvr, Lit, &["jahvarTa"]);
    assert_has_tip(&[], &hvr, Lut, &["hvartA"]);
    assert_has_tip(&[], &hvr, Lrt, &["hvarizyati"]);
}

#[test]
fn sk_2380() {
    let hvr = d("hvf\\", Bhvadi);
    assert_has_tip(&[], &hvr, AshirLin, &["hvaryAt"]);
    // LSK has ahvArzIt, which seems more correct
    assert_has_tip(&[], &hvr, Lun, &["ahvArzIt"]);

    let svr = d("svf", Bhvadi);
    assert_has_sip(&[], &svr, Lit, &["sasvariTa", "sasvarTa"]);
}

#[test]
fn sk_2381() {
    let svr = d("svf", Bhvadi);
    assert_has_vas(&[], &svr, Lit, &["sasvariva"]);
    assert_has_mas(&[], &svr, Lit, &["sasvarima"]);

    assert_has_tip(&[], &svr, Lrt, &["svarizyati"]);
    assert_has_tip(&[], &svr, AshirLin, &["svaryAt"]);
    // asvArIt is in S. C. Vasu's translations. Online editions tend to have "asvarIt" which seems
    // incorrect.
    assert_has_tip(&[], &svr, Lun, &["asvArIt", "asvArzIt"]);
    assert_has_tas(&[], &svr, Lun, &["asvArizwAm", "asvArzwAm"]);

    let sr = d("sf\\", Bhvadi);
    assert_has_sip(&[], &sr, Lit, &["sasarTa"]);
    assert_has_vas(&[], &sr, Lit, &["sasfva"]);
    assert_has_tip(&[], &sr, AshirLin, &["sriyAt"]);
    assert_has_tip(&[], &sr, Lun, &["asArzIt"]);
    assert_has_tas(&[], &sr, Lun, &["asArzwAm"]);
}

#[test]
fn sk_2382() {
    // sarati is allowed per KV on 7.3.78.
    let sr = d("sf\\", Bhvadi);
    assert_has_tip(&[], &sr, Lat, &["DAvati", "sarati"]);

    let r = d("f\\", Bhvadi);
    assert_has_tip(&[], &r, Lat, &["fcCati"]);
}

#[test]
fn sk_2383() {
    let r = d("f\\", Bhvadi);
    assert_has_tip(&[], &r, Lit, &["Ara"]);
    assert_has_tas(&[], &r, Lit, &["AratuH"]);
    assert_has_jhi(&[], &r, Lit, &["AruH"]);
}

#[test]
fn sk_2384() {
    let r = d("f\\", Bhvadi);
    assert_has_sip(&[], &r, Lit, &["AriTa"]);
    assert_has_tip(&[], &r, Lut, &["artA"]);
    assert_has_tip(&[], &r, Lrt, &["arizyati"]);
    assert_has_tip(&[], &r, AshirLin, &["aryAt"]);
    assert_has_tip(&[], &r, Lun, &["ArzIt"]);
    assert_has_tas(&[], &r, Lun, &["ArzwAm"]);

    let gr = d("gf\\", Bhvadi);
    assert_has_tip(&[], &gr, Lat, &["garati"]);
    assert_has_tip(&[], &gr, Lit, &["jagAra"]);
    assert_has_sip(&[], &gr, Lit, &["jagarTa"]);
    assert_has_vas(&[], &gr, Lit, &["jagriva"]);
    assert_has_tip(&[], &gr, AshirLin, &["griyAt"]);
    assert_has_tip(&[], &gr, Lun, &["agArzIt"]);

    let sru = d("sru\\", Bhvadi);
    assert_has_sip(&[], &sru, Lit, &["susroTa"]);
    assert_has_vas(&[], &sru, Lit, &["susruva"]);
    assert_has_tip(&[], &sru, AshirLin, &["srUyAt"]);
    assert_has_tip(&[], &sru, Lun, &["asusruvat"]);

    let su = d("zu\\", Bhvadi);
    assert_has_sip(&[], &su, Lit, &["suzoTa", "suzaviTa"]);
    assert_has_vas(&[], &su, Lit, &["suzuviva"]);
    assert_has_tip(&[], &su, Lut, &["sotA"]);
}

#[test]
fn sk_2385() {
    let su = d("zu\\", Bhvadi);
    // TODO: also support asAvIt?
    assert_has_tip(&[], &su, Lun, &["asOzIt"]);
}

#[test]
fn sk_2386() {
    let shru = d("Sru\\", Bhvadi);
    assert_has_tip(&[], &shru, Lat, &["SfRoti"]);
    assert_has_tas(&[], &shru, Lat, &["SfRutaH"]);
}

#[test]
fn sk_2387() {
    let shru = d("Sru\\", Bhvadi);
    assert_has_jhi(&[], &shru, Lat, &["SfRvanti"]);
    assert_has_mip(&[], &shru, Lat, &["SfRomi"]);
    assert_has_vas(&[], &shru, Lat, &["SfRvaH", "SfRuvaH"]);
    assert_has_mas(&[], &shru, Lat, &["SfRmaH", "SfRumaH"]);
    assert_has_sip(&[], &shru, Lit, &["SuSroTa"]);
    assert_has_vas(&[], &shru, Lit, &["SuSruva"]);
    assert_has_sip(&[], &shru, Lot, &["SfRu", "SfRutAt"]);
    assert_has_mip(&[], &shru, Lot, &["SfRavAni"]);
    assert_has_tip(&[], &shru, VidhiLin, &["SfRuyAt"]);
    assert_has_tip(&[], &shru, AshirLin, &["SrUyAt"]);
    assert_has_tip(&[], &shru, Lun, &["aSrOzIt"]);

    let dhru = d("Dru\\", Bhvadi);
    assert_has_tip(&[], &dhru, Lat, &["Dravati"]);

    let du = d("du\\", Bhvadi);
    assert_has_sip(&[], &du, Lit, &["dudoTa", "dudaviTa"]);
    assert_has_vas(&[], &du, Lit, &["duduviva"]);

    let dru = d("dru\\", Bhvadi);
    assert_has_sip(&[], &dru, Lit, &["dudroTa"]);
    assert_has_vas(&[], &dru, Lit, &["dudruva"]);
    assert_has_tip(&[], &dru, Lun, &["adudruvat"]);

    let ji = d("ji\\", Bhvadi);
    assert_has_tip(&[], &ji, Lat, &["jayati"]);
    assert_has_ta(&["parA"], &ji, Lat, &["parAjayate"]);

    let smi = d("zmi\\N", Bhvadi);
    assert_has_ta(&[], &smi, Lat, &["smayate"]);
    assert_has_ta(&[], &smi, Lit, &["sizmiye"]);
    assert_has_dhvam(&[], &smi, Lit, &["sizmiyiQve", "sizmiyiDve"]);

    let gu = d("gu\\N", Bhvadi);
    assert_has_ta(&[], &gu, Lat, &["gavate"]);
    assert_has_ta(&[], &gu, Lit, &["juguve"]);

    let gaa = d("gA\\N", Bhvadi);
    assert_has_ta(&[], &gaa, Lat, &["gAte"]);
    assert_has_aataam(&[], &gaa, Lat, &["gAte"]);
    assert_has_jha(&[], &gaa, Lat, &["gAte"]);
    assert_has_iw(&[], &gaa, Lan, &["age"]);
    assert_has_ta(&[], &gaa, VidhiLin, &["geta"]);
    assert_has_aataam(&[], &gaa, VidhiLin, &["geyAtAm"]);
    assert_has_jha(&[], &gaa, VidhiLin, &["geran"]);
    assert_has_ta(&[], &gaa, AshirLin, &["gAsIzwa"]);
    assert_has_ta(&[], &gaa, Lun, &["agAsta"]);

    let ku = d("ku\\N", Bhvadi);
    assert_has_ta(&[], &ku, Lat, &["kavate"]);
    assert_has_ta(&[], &ku, Lit, &["cukuve"]);

    let ghu = d("Gu\\N", Bhvadi);
    assert_has_ta(&[], &ghu, Lat, &["Gavate"]);

    let u = d("u\\N", Bhvadi);
    assert_has_ta(&[], &u, Lat, &["avate"]);
    assert_has_ta(&[], &u, Lit, &["Uve"]);
    assert_has_ta(&[], &u, Lut, &["otA"]);
    assert_has_ta(&[], &u, Lrt, &["ozyate"]);
    assert_has_ta(&[], &u, AshirLin, &["ozIzwa"]);
    assert_has_ta(&[], &u, Lun, &["Ozwa"]);

    let nu = d("Nu\\N", Bhvadi);
    assert_has_ta(&[], &nu, Lat, &["Navate"]);
    assert_has_ta(&[], &nu, Lit, &["YuNuve"]);
    assert_has_ta(&[], &nu, Lut, &["NotA"]);

    let ru = d("ru\\N", Bhvadi);
    assert_has_ta(&[], &ru, Lit, &["ruruve"]);
    // TODO: how does SK have ravitAse here?
    assert_has_thaas(&[], &ru, Lut, &["rotAse"]);

    let dhr = d("Df\\N", Bhvadi);
    assert_has_ta(&[], &dhr, Lat, &["Darate"]);
    assert_has_ta(&[], &dhr, Lit, &["daDre"]);

    let me = d("me\\N", Bhvadi);
    assert_has_ta(&["pra", "ni"], &me, Lat, &["praRimayate"]);

    let de = d("de\\N", Bhvadi);
    assert_has_ta(&[], &de, Lat, &["dayate"]);
}

#[test]
fn sk_2388() {
    let de = d("de\\N", Bhvadi);
    assert_has_ta(&[], &de, Lit, &["digye"]);
}

#[test]
fn sk_2389() {
    let de = d("de\\N", Bhvadi);
    assert_has_ta(&[], &de, Lun, &["adita"]);
    assert_has_thaas(&[], &de, Lun, &["adiTAH"]);

    let shyai = d("SyE\\N", Bhvadi);
    assert_has_ta(&[], &shyai, Lat, &["SyAyate"]);

    let pyai = d("pyE\\N", Bhvadi);
    assert_has_ta(&[], &pyai, Lat, &["pyAyate"]);
    assert_has_ta(&[], &pyai, Lit, &["papye"]);

    let trai = d("trE\\N", Bhvadi);
    assert_has_ta(&[], &trai, Lat, &["trAyate"]);

    let pu = d("pUN", Bhvadi);
    assert_has_ta(&[], &pu, Lat, &["pavate"]);
    assert_has_ta(&[], &pu, Lit, &["pupuve"]);
}

#[test]
fn sk_2390() {
    let tr = d("tF", Bhvadi);
    assert_has_tip(&[], &tr, Lat, &["tarati"]);
    assert_has_tas(&[], &tr, Lit, &["teratuH"]);
    assert_has_jhi(&[], &tr, Lit, &["teruH"]);
}

#[test]
fn sk_2391() {
    let tr = d("tF", Bhvadi);
    assert_has_tip(&[], &tr, Lut, &["tarItA", "taritA"]);
    assert_has_sip(&[], &tr, Lit, &["teriTa"]);
    assert_has_tip(&[], &tr, AshirLin, &["tIryAt"]);
}

#[test]
fn sk_2392() {
    let tr = d("tF", Bhvadi);
    assert_has_tas(&[], &tr, Lun, &["atArizwAm"]);
}

#[test]
fn skip_sk_2393_to_sk_2394() {}

#[test]
fn sk_2395() {
    let gup = d("gupa~\\", Bhvadi);
    assert_has_ta(&[], &gup, Lat, &["jugupsate"]);
    assert_has_ta(
        &[],
        &gup,
        Lit,
        &["jugupsAYcakre", "jugupsAmbaBUva", "jugupsAmAsa"],
    );

    let tij = d("tija~\\", Bhvadi);
    assert_has_ta(&[], &tij, Lat, &["titikzate"]);

    let man = d("mAna~\\", Bhvadi);
    assert_has_ta(&[], &man, Lat, &["mImAMsate"]);

    let badh = d("baDa~\\", Bhvadi);
    assert_has_ta(&[], &badh, Lat, &["bIBatsate"]);

    let rabh = d("ra\\Ba~\\", Bhvadi);
    assert_has_ta(&["AN"], &rabh, Lat, &["AraBate"]);
    assert_has_ta(&["AN"], &rabh, Lit, &["AreBe"]);
    assert_has_ta(&[], &rabh, Lut, &["rabDA"]);
    assert_has_ta(&[], &rabh, Lrt, &["rapsyate"]);

    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_ta(&[], &labh, Lat, &["laBate"]);
}

#[test]
fn skip_sk_2396() {}

#[test]
fn sk_2397() {
    let svanj = d("zva\\nja~\\", Bhvadi);
    assert_has_ta(&[], &svanj, Lat, &["svajate"]);
    assert_has_ta(&["pari"], &svanj, Lat, &["parizvajate"]);

    // dadamBatuH is from SK 2533.
    let dambh = d("danBu~", Bhvadi);
    assert_has_tas(&[], &dambh, Lit, &["deBatuH", "dadamBatuH"]);

    assert_has_ta(&["pari"], &svanj, Lit, &["parizasvaje", "parizasvaYje"]);
    assert_has_thaas(&[], &svanj, Lit, &["sasvajize", "sasvaYjize"]);
    assert_has_ta(&[], &svanj, Lut, &["svaNktA"]);
    assert_has_ta(&[], &svanj, Lrt, &["svaNkzyate"]);
    assert_has_ta(&[], &svanj, VidhiLin, &["svajeta"]);
    assert_has_ta(&[], &svanj, AshirLin, &["svaNkzIzwa"]);
    assert_has_ta(&["prati"], &svanj, Lun, &["pratyazvaNkta"]);
    assert_has_ta(&["pari"], &svanj, Lun, &["paryazvaNkta", "paryasvaNkta"]);

    let had = d("ha\\da~\\", Bhvadi);
    assert_has_ta(&[], &had, Lat, &["hadate"]);
    assert_has_ta(&[], &had, Lit, &["jahade"]);
    assert_has_ta(&[], &had, Lut, &["hattA"]);
    assert_has_ta(&[], &had, Lrt, &["hatsyate"]);
    assert_has_ta(&[], &had, VidhiLin, &["hadeta"]);
    assert_has_ta(&[], &had, AshirLin, &["hatsIzwa"]);
    assert_has_ta(&[], &had, Lun, &["ahatta"]);

    let skand = d("ska\\ndi~r", Bhvadi);
    assert_has_sip(&[], &skand, Lit, &["caskandiTa", "caskanTa", "caskantTa"]);
    assert_has_tip(&[], &skand, Lut, &["skantA", "skanttA"]);
    assert_has_tip(&[], &skand, Lrt, &["skantsyati"]);
    assert_has_tip(&[], &skand, AshirLin, &["skadyAt"]);
    assert_has_tip(&[], &skand, Lun, &["askadat", "askAntsIt"]);
    assert_has_tas(&[], &skand, Lun, &["askadatAm", "askAntAm", "askAnttAm"]);
    assert_has_jhi(&[], &skand, Lun, &["askadan", "askAntsuH"]);
}

#[test]
fn sk_2398() {
    let skand = d("ska\\ndi~r", Bhvadi);
    assert_has_tip(
        &["vi"],
        &skand,
        Lut,
        &["vizkantA", "viskantA", "vizkanttA", "viskanttA"],
    );
    assert_has_krdanta(&["vi"], &skand, Krt::kta, &["viskanna"]);
}

#[test]
fn sk_2399() {
    let skand = d("ska\\ndi~r", Bhvadi);
    assert_has_tip(&["pari"], &skand, Lat, &["parizkandati", "pariskandati"]);
    assert_has_krdanta(&["pari"], &skand, Krt::kta, &["parizkaRRa", "pariskanna"]);

    let yabh = d("ya\\Ba~", Bhvadi);
    assert_has_sip(&[], &yabh, Lit, &["yeBiTa", "yayabDa"]);
    assert_has_tip(&[], &yabh, Lut, &["yabDA"]);
    assert_has_tip(&[], &yabh, Lrt, &["yapsyati"]);
    assert_has_tip(&[], &yabh, Lun, &["ayApsIt"]);

    let nam = d("Ra\\ma~", Bhvadi);
    assert_has_sip(&[], &nam, Lit, &["nemiTa", "nananTa"]);
    assert_has_tip(&[], &nam, Lut, &["nantA"]);
    assert_has_tip(&[], &nam, Lun, &["anaMsIt"]);
    assert_has_tas(&[], &nam, Lun, &["anaMsizwAm"]);
}

#[test]
fn sk_2400() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_tip(&[], &gam, Lat, &["gacCati"]);
    assert_has_tip(&[], &gam, Lit, &["jagAma"]);
    assert_has_tas(&[], &gam, Lit, &["jagmatuH"]);
    assert_has_jhi(&[], &gam, Lit, &["jagmuH"]);
    assert_has_sip(&[], &gam, Lit, &["jagamiTa", "jaganTa"]);
    assert_has_tip(&[], &gam, Lut, &["gantA"]);
}

#[test]
fn sk_2401() {
    let gam = d("ga\\mx~", Bhvadi);
    assert_has_tip(&[], &gam, Lrt, &["gamizyati"]);
    assert_has_tip(&[], &gam, Lun, &["agamat"]);

    let srp = d("sf\\px~", Bhvadi);
    assert_has_tip(&[], &srp, Lat, &["sarpati"]);
    assert_has_tip(&[], &srp, Lit, &["sasarpa"]);
}

#[test]
fn sk_2402() {
    let srp = d("sf\\px~", Bhvadi);
    assert_has_tip(&[], &srp, Lut, &["sraptA", "sarptA"]);
    assert_has_tip(&[], &srp, Lrt, &["srapsyati", "sarpsyati"]);
    assert_has_tip(&[], &srp, Lun, &["asfpat"]);

    let yam = d("ya\\ma~", Bhvadi);
    assert_has_tip(&[], &yam, Lat, &["yacCati"]);
    assert_has_sip(&[], &yam, Lit, &["yemiTa", "yayanTa"]);
    assert_has_tip(&[], &yam, Lut, &["yantA"]);
    assert_has_tip(&[], &yam, Lun, &["ayaMsIt"]);
    assert_has_tas(&[], &yam, Lun, &["ayaMsizwAm"]);

    let tap = d("ta\\pa~", Bhvadi);
    assert_has_tip(&[], &tap, Lut, &["taptA"]);
    assert_has_tip(&[], &tap, Lun, &["atApsIt"]);
}

#[test]
fn sk_2403() {
    let tyaj = d("tya\\ja~", Bhvadi);
    assert_has_sip(&[], &tyaj, Lit, &["tatyajiTa", "tatyakTa"]);
    assert_has_tip(&[], &tyaj, Lut, &["tyaktA"]);
    assert_has_tip(&[], &tyaj, Lun, &["atyAkzIt"]);

    let sanj = d("za\\nja~", Bhvadi);
    assert_has_tip(&[], &sanj, Lat, &["sajati"]);
    assert_has_tip(&[], &sanj, Lut, &["saNktA"]);

    let drsh = d("df\\Si~r", Bhvadi);
    assert_has_tip(&[], &drsh, Lat, &["paSyati"]);
}

#[test]
fn skip_sk_2404() {}

#[test]
fn sk_2405() {
    let drsh = d("df\\Si~r", Bhvadi);
    assert_has_sip(&[], &drsh, Lit, &["dadrazWa", "dadarSiTa"]);
    assert_has_tip(&[], &drsh, Lut, &["drazwA"]);
    assert_has_tip(&[], &drsh, Lrt, &["drakzyati"]);
    assert_has_tip(&[], &drsh, AshirLin, &["dfSyAt"]);
}

#[test]
fn sk_2406() {
    // adrAkzIt is from SK 2407.
    let dfs = d("df\\Si~r", Bhvadi);
    assert_has_tip(&[], &dfs, Lun, &["adarSat", "adrAkzIt"]);
}

#[test]
fn sk_2407() {
    // adarSat is from SK 2406.
    let dfs = d("df\\Si~r", Bhvadi);
    assert_has_tip(&[], &dfs, Lun, &["adarSat", "adrAkzIt"]);

    let dansh = d("da\\nSa~", Bhvadi);
    assert_has_tip(&[], &dansh, Lat, &["daSati"]);
    assert_has_sip(&[], &dansh, Lit, &["dadaMSiTa", "dadaMzWa"]);
    assert_has_tip(&[], &dansh, Lut, &["daMzwA"]);
    assert_has_tip(&[], &dansh, Lrt, &["daNkzyati"]);
    assert_has_tip(&[], &dansh, AshirLin, &["daSyAt"]);
    assert_has_tip(&[], &dansh, Lun, &["adANkzIt"]);

    let krsh = d("kf\\za~", Bhvadi);
    assert_has_tip(&[], &krsh, Lut, &["krazwA", "karzwA"]);
    assert_has_tip(&[], &krsh, Lrt, &["krakzyati", "karkzyati"]);
    // akrAkzuH is justified.
    assert_has_tip(&[], &krsh, Lun, &["akrAkzIt", "akArkzIt", "akfkzat"]);
    assert_has_tas(&[], &krsh, Lun, &["akrAzwAm", "akArzwAm", "akfkzatAm"]);
    assert_has_jhi(&[], &krsh, Lun, &["akrAkzuH", "akArkzuH", "akfkzan"]);

    let dah = d("da\\ha~", Bhvadi);
    assert_has_sip(&[], &dah, Lit, &["dehiTa", "dadagDa"]);
    assert_has_tip(&[], &dah, Lut, &["dagDA"]);
    assert_has_tip(&[], &dah, Lrt, &["Dakzyati"]);
    assert_has_tip(&[], &dah, Lun, &["aDAkzIt"]);
    assert_has_tas(&[], &dah, Lun, &["adAgDAm"]);
    assert_has_jhi(&[], &dah, Lun, &["aDAkzuH"]);

    let mih = d("mi\\ha~", Bhvadi);
    assert_has_tip(&[], &mih, Lit, &["mimeha"]);
    assert_has_sip(&[], &mih, Lit, &["mimehiTa"]);
    assert_has_tip(&[], &mih, Lut, &["meQA"]);
    assert_has_tip(&[], &mih, Lrt, &["mekzyati"]);
    assert_has_tip(&[], &mih, Lun, &["amikzat"]);

    // TODO: cikitsate ??
    let kit = d("kita~", Bhvadi);
    assert_has_tip(&[], &kit, Lat, &["cikitsati"]);
    let ket = d("keta", Curadi);
    assert_has_tip(&[], &ket, Lat, &["ketayati"]);

    let dan = d("dAna~^", Bhvadi);
    assert_has_tip(&[], &dan, Lat, &["dIdAMsati"]);

    let shan = d("SAna~^", Bhvadi);
    assert_has_tip(&[], &shan, Lat, &["SISAMsati"]);
    assert_has_ta(&[], &shan, Lat, &["SISAMsate"]);

    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_tip(&[], &pac, Lat, &["pacati"]);
    assert_has_ta(&[], &pac, Lat, &["pacate"]);
    assert_has_sip(&[], &pac, Lit, &["peciTa", "papakTa"]);
    assert_has_ta(&[], &pac, Lit, &["pece"]);
    assert_has_tip(&[], &pac, Lut, &["paktA"]);
    assert_has_ta(&[], &pac, AshirLin, &["pakzIzwa"]);

    let sac = d("zaca~^", Bhvadi);
    assert_has_tip(&[], &sac, Lat, &["sacati"]);
    assert_has_ta(&[], &sac, Lat, &["sacate"]);

    let bhaj = d("Ba\\ja~^", Bhvadi);
    assert_has_tas(&[], &bhaj, Lit, &["BejatuH"]);
    assert_has_jhi(&[], &bhaj, Lit, &["BejuH"]);
    assert_has_sip(&[], &bhaj, Lit, &["BejiTa", "baBakTa"]);
    assert_has_tip(&[], &bhaj, Lut, &["BaktA"]);
    assert_has_tip(&[], &bhaj, Lrt, &["Bakzyati"]);
    assert_has_ta(&[], &bhaj, Lrt, &["Bakzyate"]);
    assert_has_tip(&[], &bhaj, Lun, &["aBAkzIt"]);
    assert_has_ta(&[], &bhaj, Lun, &["aBakta"]);

    let ranj = d("ra\\nja~^", Bhvadi);
    assert_has_tip(&[], &ranj, Lat, &["rajati"]);
    assert_has_ta(&[], &ranj, Lat, &["rajate"]);
    assert_has_tip(&[], &ranj, Lun, &["arANkzIt"]);
    assert_has_ta(&[], &ranj, Lun, &["araNkta"]);

    let shap = d("Sa\\pa~^", Bhvadi);
    assert_has_tip(&[], &shap, Lit, &["SaSApa"]);
    assert_has_ta(&[], &shap, Lit, &["Sepe"]);
    assert_has_tip(&[], &shap, Lun, &["aSApsIt"]);
    assert_has_ta(&[], &shap, Lun, &["aSapta"]);

    let tvish = d("tvi\\za~^", Bhvadi);
    assert_has_tip(&[], &tvish, Lat, &["tvezati"]);
    assert_has_ta(&[], &tvish, Lat, &["tvezate"]);
    assert_has_ta(&[], &tvish, Lit, &["titvize"]);
    assert_has_tip(&[], &tvish, Lrt, &["tvekzyati"]);
    assert_has_ta(&[], &tvish, AshirLin, &["tvikzIzwa"]);
    assert_has_tip(&[], &tvish, Lun, &["atvikzat"]);
    assert_has_aataam(&[], &tvish, Lun, &["atvikzAtAm"]);

    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_tip(&[], &yaj, Lat, &["yajati"]);
    assert_has_ta(&[], &yaj, Lat, &["yajate"]);
}

#[test]
fn sk_2408() {
    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_tip(&[], &yaj, Lit, &["iyAja"]);
}

#[test]
fn sk_2409() {
    let yaj = d("ya\\ja~^", Bhvadi);
    assert_has_tas(&[], &yaj, Lit, &["IjatuH"]);
    assert_has_jhi(&[], &yaj, Lit, &["IjuH"]);
    assert_has_sip(&[], &yaj, Lit, &["iyajiTa", "iyazWa"]);
    assert_has_ta(&[], &yaj, Lit, &["Ije"]);

    assert_has_tip(&[], &yaj, Lut, &["yazwA"]);
    assert_has_tip(&[], &yaj, Lrt, &["yakzyati"]);
    assert_has_tip(&[], &yaj, AshirLin, &["ijyAt"]);
    assert_has_ta(&[], &yaj, AshirLin, &["yakzIzwa"]);
    assert_has_tip(&[], &yaj, Lun, &["ayAkzIt"]);
    assert_has_ta(&[], &yaj, Lun, &["ayazwa"]);

    let vap = d("quva\\pa~^", Bhvadi);
    assert_has_tip(&[], &vap, Lat, &["vapati"]);
    assert_has_tip(&[], &vap, Lit, &["uvApa"]);
    assert_has_ta(&[], &vap, Lit, &["Upe"]);
    assert_has_tip(&[], &vap, Lut, &["vaptA"]);
    assert_has_tip(&[], &vap, AshirLin, &["upyAt"]);
    assert_has_ta(&[], &vap, AshirLin, &["vapsIzwa"]);
    assert_has_tip(&["pra", "ni"], &vap, Lun, &["praRyavApsIt"]);
    assert_has_ta(&[], &vap, Lun, &["avapta"]);

    let vah = d("va\\ha~^", Bhvadi);
    assert_has_tip(&[], &vah, Lit, &["uvAha"]);
    assert_has_sip(&[], &vah, Lit, &["uvoQa", "uvahiTa"]);
    assert_has_ta(&[], &vah, Lit, &["Uhe"]);
    assert_has_tip(&[], &vah, Lut, &["voQA"]);
    assert_has_tip(&[], &vah, Lrt, &["vakzyati"]);

    assert_has_tip(&[], &vah, Lun, &["avAkzIt"]);
    assert_has_tas(&[], &vah, Lun, &["avoQAm"]);
    assert_has_jhi(&[], &vah, Lun, &["avAkzuH"]);
    assert_has_ta(&[], &vah, Lun, &["avoQa"]);
    assert_has_aataam(&[], &vah, Lun, &["avakzAtAm"]);
    assert_has_jha(&[], &vah, Lun, &["avakzata"]);
    assert_has_thaas(&[], &vah, Lun, &["avoQAH"]);
    assert_has_dhvam(&[], &vah, Lun, &["avoQvam"]);

    let vas = d("va\\sa~", Bhvadi);
    assert_has_tip(&[], &vas, Lat, &["vasati"]);
    assert_has_tip(&[], &vas, Lit, &["uvAsa"]);
}

#[test]
fn sk_2410() {
    let vas = d("va\\sa~", Bhvadi);
    assert_has_tas(&[], &vas, Lit, &["UzatuH"]);
    assert_has_jhi(&[], &vas, Lit, &["UzuH"]);
    assert_has_sip(&[], &vas, Lit, &["uvasiTa", "uvasTa"]);

    assert_has_tip(&[], &vas, Lut, &["vastA"]);
    assert_has_tip(&[], &vas, Lrt, &["vatsyati"]);
    assert_has_tip(&[], &vas, AshirLin, &["uzyAt"]);
    assert_has_tip(&[], &vas, Lun, &["avAtsIt"]);
    assert_has_tas(&[], &vas, Lun, &["avAttAm"]);

    let ve = d("ve\\Y", Bhvadi);
    assert_has_tip(&[], &ve, Lat, &["vayati"]);
    assert_has_ta(&[], &ve, Lat, &["vayate"]);
}

#[test]
fn sk_2411() {
    // vavO is from SK 2415.
    let ve = d("ve\\Y", Bhvadi);
    assert_has_tip(&[], &ve, Lit, &["uvAya", "vavO"]);
}

#[test]
fn skip_sk_2412() {}

#[test]
fn sk_2413() {
    // UvatuH and UvuH are from SK 2413.
    // vavatuH and vavuH are from SK 2415.
    let ve = d("ve\\Y", Bhvadi);
    assert_has_tas(&[], &ve, Lit, &["UyatuH", "UvatuH", "vavatuH"]);
    assert_has_jhi(&[], &ve, Lit, &["UyuH", "UvuH", "vavuH"]);
}

#[test]
fn sk_2414() {
    // vaviTa, vavATa, and vave are from SK 2415.
    let ve = d("ve\\Y", Bhvadi);
    assert_has_tas(&[], &ve, Lit, &["UyatuH", "UvatuH", "vavatuH"]);
    assert_has_jhi(&[], &ve, Lit, &["UyuH", "UvuH", "vavuH"]);
    assert_has_sip(&[], &ve, Lit, &["uvayiTa", "vaviTa", "vavATa"]);
    assert_has_ta(&[], &ve, Lit, &["Uye", "Uve", "vave"]);
}

#[test]
fn sk_2415() {
    // uvAya is from SK 2411.
    let ve = d("ve\\Y", Bhvadi);
    assert_has_tip(&[], &ve, Lit, &["uvAya", "vavO"]);
    assert_has_tas(&[], &ve, Lit, &["UyatuH", "UvatuH", "vavatuH"]);
    assert_has_jhi(&[], &ve, Lit, &["UyuH", "UvuH", "vavuH"]);
    assert_has_sip(&[], &ve, Lit, &["uvayiTa", "vaviTa", "vavATa"]);
    assert_has_ta(&[], &ve, Lit, &["Uye", "Uve", "vave"]);

    assert_has_tip(&[], &ve, Lut, &["vAtA"]);
    assert_has_tip(&[], &ve, AshirLin, &["UyAt"]);
    assert_has_ta(&[], &ve, AshirLin, &["vAsIzwa"]);
    assert_has_tip(&[], &ve, Lun, &["avAsIt"]);

    let vye = d("vye\\Y", Bhvadi);
    assert_has_tip(&[], &vye, Lat, &["vyayati"]);
}

#[test]
fn sk_2416() {
    let vye = d("vye\\Y", Bhvadi);
    assert_has_tip(&[], &vye, Lit, &["vivyAya"]);
    assert_has_tas(&[], &vye, Lit, &["vivyatuH"]);
    assert_has_jhi(&[], &vye, Lit, &["vivyuH"]);
    assert_has_sip(&[], &vye, Lit, &["vivyayiTa"]);
    assert_has_mip(&[], &vye, Lit, &["vivyAya", "vivyaya"]);

    assert_has_tip(&[], &vye, Lut, &["vyAtA"]);
    assert_has_tip(&[], &vye, AshirLin, &["vIyAt"]);
    assert_has_ta(&[], &vye, AshirLin, &["vyAsIzwa"]);
    assert_has_tip(&[], &vye, Lun, &["avyAsIt"]);
    assert_has_ta(&[], &vye, Lun, &["avyAsta"]);
}

#[test]
fn sk_2417() {
    let hve = d("hve\\Y", Bhvadi);
    assert_has_tip(&[], &hve, Lit, &["juhAva"]);
    assert_has_tas(&[], &hve, Lit, &["juhuvatuH"]);
    assert_has_jhi(&[], &hve, Lit, &["juhuvuH"]);
    assert_has_sip(&[], &hve, Lit, &["juhoTa", "juhaviTa"]);
    assert_has_ta(&[], &hve, Lit, &["juhuve"]);
    assert_has_tip(&[], &hve, Lut, &["hvAtA"]);
    assert_has_tip(&[], &hve, AshirLin, &["hUyAt"]);
    assert_has_ta(&[], &hve, AshirLin, &["hvAsIzwa"]);
}

#[test]
fn skip_sk_2418() {}

#[test]
fn sk_2419() {
    let hve = d("hve\\Y", Bhvadi);
    assert_has_tip(&[], &hve, Lun, &["ahvat"]);
    assert_has_tas(&[], &hve, Lun, &["ahvatAm"]);
    assert_has_jhi(&[], &hve, Lun, &["ahvan"]);
    assert_has_ta(&[], &hve, Lun, &["ahvata", "ahvAsta"]);

    let vad = d("vada~", Bhvadi);
    assert_has_tip(&[], &vad, Lat, &["vadati"]);
    assert_has_tip(&[], &vad, Lit, &["uvAda"]);
    assert_has_tas(&[], &vad, Lit, &["UdatuH"]);
    assert_has_sip(&[], &vad, Lit, &["uvadiTa"]);
    assert_has_tip(&[], &vad, Lut, &["vaditA"]);
    assert_has_tip(&[], &vad, AshirLin, &["udyAt"]);
    assert_has_tip(&[], &vad, Lun, &["avAdIt"]);

    let shvi = d("wuo~Svi", Bhvadi);
    assert_has_tip(&[], &shvi, Lat, &["Svayati"]);
}

#[test]
fn sk_2420() {
    let shvi = d("wuo~Svi", Bhvadi);
    assert_has_tip(&[], &shvi, Lit, &["SuSAva", "SiSvAya"]);
    assert_has_tas(&[], &shvi, Lit, &["SuSuvatuH", "SiSviyatuH"]);
    assert_has_tip(&[], &shvi, Lut, &["SvayitA"]);
    assert_has_tip(&[], &shvi, VidhiLin, &["Svayet"]);
    assert_has_tip(&[], &shvi, AshirLin, &["SUyAt"]);
}

#[test]
fn sk_2421() {
    let shvi = d("wuo~Svi", Bhvadi);
    // aSiSviyatAm and aSiSviyan are justified.
    // aSvayizwAm and aSvayizuH are justified.
    assert_has_tip(&[], &shvi, Lun, &["aSvat", "aSiSviyat", "aSvayIt"]);
    assert_has_tas(&[], &shvi, Lun, &["aSvatAm", "aSiSviyatAm", "aSvayizwAm"]);
    assert_has_jhi(&[], &shvi, Lun, &["aSvan", "aSiSviyan", "aSvayizuH"]);
}

#[test]
fn sk_2422() {
    let fti = d("fti", Bhvadi);
    assert_has_ta(&[], &fti, Lat, &["ftIyate"]);
    assert_has_ta(
        &[],
        &fti,
        Lit,
        &["ftIyAYcakre", "ftIyAmbaBUva", "ftIyAmAsa"],
    );
    assert_has_tip(&[], &fti, Lit, &["Anarta"]);
    assert_has_tip(&[], &fti, Lrt, &["artizyati"]);
    assert_has_tip(&[], &fti, Lun, &["ArtIt"]);
}
