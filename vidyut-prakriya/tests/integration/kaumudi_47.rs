extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;

#[test]
fn sk_2523() {
    let su = d("zu\\Y", Svadi);
    assert_has_tip(&[], &su, Lat, &["sunoti"]);
    assert_has_tas(&[], &su, Lat, &["sunutaH"]);
    assert_has_jhi(&[], &su, Lat, &["sunvanti"]);

    assert_has_vas(&[], &su, Lat, &["sunvaH", "sunuvaH"]);
    assert_has_vahi(&[], &su, Lat, &["sunvahe", "sunuvahe"]);
    assert_has_tip(&[], &su, Lit, &["suzAva"]);
    assert_has_ta(&[], &su, Lit, &["suzuve"]);
    assert_has_tip(&[], &su, Lut, &["sotA"]);
    assert_has_sip(&[], &su, Lot, &["sunu", "sunutAt"]);
    assert_has_mip(&[], &su, Lot, &["sunavAni"]);
    assert_has_iw(&[], &su, Lot, &["sunavE"]);
    assert_has_tip(&[], &su, VidhiLin, &["sunuyAt"]);
    assert_has_tip(&[], &su, AshirLin, &["sUyAt"]);
    assert_has_tip(&[], &su, Lun, &["asAvIt"]);
    assert_has_ta(&[], &su, Lun, &["asozwa"]);

    assert_has_tip(&["aBi"], &su, Lat, &["aBizuRoti"]);
    assert_has_tip(&["aBi"], &su, Lan, &["aByazuRot"]);
    assert_has_tip(&["aBi"], &su, Lit, &["aBisuzAva"]);
}

#[test]
fn sk_2524() {
    let su = d("zu\\Y", Svadi);
    assert_has_tip(&["vi"], &su, Lrt, &["visozyati"]);

    let si = d("zi\\Y", Svadi);
    assert_has_tip(&[], &si, Lat, &["sinoti"]);
    assert_has_tip(&["vi"], &si, Lat, &["visinoti"]);
    assert_has_tip(&[], &si, Lit, &["sizAya"]);
    assert_has_ta(&[], &si, Lit, &["sizye"]);

    let shi = d("Si\\Y", Svadi);
    assert_has_tip(&[], &shi, Lut, &["SetA"]);

    let mi = d("qumi\\Y", Svadi);
    assert_has_tip(&[], &mi, Lit, &["mamO"]);
    assert_has_sip(&[], &mi, Lit, &["mamiTa", "mamATa"]);
    assert_has_ta(&[], &mi, Lit, &["mimye"]);
    assert_has_tip(&[], &mi, Lut, &["mAtA"]);
    assert_has_tip(&[], &mi, AshirLin, &["mIyAt"]);
    assert_has_ta(&[], &mi, AshirLin, &["mAsIzwa"]);
    assert_has_tip(&[], &mi, Lun, &["amAsIt"]);
    assert_has_tas(&[], &mi, Lun, &["amAsizwAm"]);
    assert_has_ta(&[], &mi, Lun, &["amAsta"]);

    let ci = d("ci\\Y", Svadi);
    assert_has_tip(&["pra", "ni"], &ci, Lat, &["praRicinoti"]);
}

#[test]
fn sk_2525() {
    let ci = d("ci\\Y", Svadi);
    assert_has_tip(&["pra", "ni"], &ci, Lit, &["praRicikAya", "praRicicAya"]);
    assert_has_ta(&[], &ci, Lit, &["cikye", "cicye"]);
    assert_has_tip(&[], &ci, Lun, &["acEzIt"]);
    assert_has_ta(&[], &ci, Lun, &["acezwa"]);

    let str = d("stf\\Y", Svadi);
    assert_has_tip(&[], &str, Lat, &["stfRoti"]);
    assert_has_ta(&[], &str, Lat, &["stfRute"]);
    assert_has_tip(&[], &str, AshirLin, &["staryAt"]);
}

#[test]
fn sk_2526() {
    let str = d("stf\\Y", Svadi);
    assert_has_ta(&[], &str, AshirLin, &["starizIzwa", "stfzIzwa"]);
    assert_has_ta(&[], &str, Lun, &["astarizwa", "astfta"]);

    let kr = d("kf\\Y", Svadi);
    assert_has_tip(&[], &kr, Lat, &["kfRoti"]);
    assert_has_ta(&[], &kr, Lat, &["kfRute"]);
    assert_has_tip(&[], &kr, Lit, &["cakAra"]);
    assert_has_sip(&[], &kr, Lit, &["cakarTa"]);
    assert_has_ta(&[], &kr, Lit, &["cakre"]);
    assert_has_tip(&[], &kr, AshirLin, &["kriyAt"]);
    assert_has_ta(&[], &kr, AshirLin, &["kfzIzwa"]);
    assert_has_tip(&[], &kr, Lun, &["akArzIt"]);
    assert_has_ta(&[], &kr, Lun, &["akfta"]);
}

#[test]
fn sk_2527() {
    let vr = d("vfY", Svadi);
    assert_has_sip(&[], &vr, Lit, &["vavariTa"]);
    assert_has_vas(&[], &vr, Lit, &["vavfva"]);
    assert_has_vahi(&[], &vr, Lit, &["vavfvahe"]);
    // varItA by 7.2.38.
    assert_has_tip(&[], &vr, Lut, &["varitA", "varItA"]);
}

#[test]
fn skip_sk_2528() {}

#[test]
fn sk_2529() {
    let vr = d("vfY", Svadi);
    assert_has_ta(&[], &vr, AshirLin, &["varizIzwa", "vfzIzwa"]);
    assert_has_tip(&[], &vr, Lun, &["avArIt"]);
    assert_has_ta(&[], &vr, Lun, &["avarizwa", "avarIzwa", "avfta"]);

    let dhu = d("Du\\Y", Svadi);
    assert_has_tip(&[], &dhu, Lat, &["Dunoti"]);
    assert_has_ta(&[], &dhu, Lat, &["Dunute"]);
    assert_has_tip(&[], &dhu, Lun, &["aDOzIt"]);
    assert_has_tip(&[], &dhu, Lrn, &["aDozyat"]);

    let dhuu = d("DUY", Svadi);
    assert_has_tip(&[], &dhuu, Lat, &["DUnoti"]);
    assert_has_ta(&[], &dhuu, Lat, &["DUnute"]);
    assert_has_sip(&[], &dhuu, Lit, &["duDaviTa", "duDoTa"]);
    assert_has_vas(&[], &dhuu, Lit, &["duDuviva"]);
    assert_has_tip(&[], &dhuu, Lun, &["aDAvIt"]);
    assert_has_ta(&[], &dhuu, Lun, &["aDavizwa", "aDozwa"]);

    let du = d("wudu\\", Svadi);
    assert_has_tip(&[], &du, Lat, &["dunoti"]);
}

#[test]
fn sk_2530() {
    let hi = d("hi\\", Svadi);
    assert_has_tip(&["pra"], &hi, Lat, &["prahiRoti"]);
}

#[test]
fn sk_2531() {
    let hi = d("hi\\", Svadi);
    assert_has_tip(&[], &hi, Lit, &["jiGAya"]);

    let pr = d("pf\\", Svadi);
    assert_has_tip(&[], &pr, Lat, &["pfRoti"]);
    assert_has_tip(&[], &pr, Lut, &["partA"]);

    let spr = d("spf\\", Svadi);
    assert_has_tip(&[], &spr, Lat, &["spfRoti"]);
    assert_has_tip(&[], &spr, Lit, &["paspAra"]);

    let smr = d("smf\\", Svadi);
    assert_has_tip(&[], &smr, Lat, &["smfRoti"]);

    let aap = d("A\\px~", Svadi);
    assert_has_tip(&[], &aap, Lat, &["Apnoti"]);
    assert_has_tas(&[], &aap, Lat, &["ApnutaH"]);
    assert_has_jhi(&[], &aap, Lat, &["Apnuvanti"]);
    assert_has_vas(&[], &aap, Lat, &["ApnuvaH"]);
    assert_has_tip(&[], &aap, Lit, &["Apa"]);
    assert_has_tip(&[], &aap, Lut, &["AptA"]);
    assert_has_sip(&[], &aap, Lot, &["Apnuhi", "ApnutAt"]);
    assert_has_tip(&[], &aap, Lun, &["Apat"]);

    let shak = d("Sa\\kx~", Svadi);
    assert_has_tip(&[], &shak, Lun, &["aSakat"]);

    let raadh = d("rA\\Da~", Svadi);
    assert_has_tip(&[], &raadh, Lat, &["rADnoti"]);
}

#[test]
fn sk_2532() {
    let radh = d("rA\\Da~", Svadi);
    assert_has_tas(&["apa"], &radh, Lit, &["apareDatuH"]);
    assert_has_jhi(&[], &radh, Lit, &["reDuH"]);
    assert_has_sip(&[], &radh, Lit, &["reDiTa"]);
    assert_has_tip(&[], &radh, Lut, &["rAdDA"]);

    let sadh = d("sA\\Da~", Svadi);
    assert_has_tip(&[], &sadh, Lat, &["sADnoti"]);
    assert_has_tip(&[], &sadh, Lut, &["sAdDA"]);
    assert_has_tip(&[], &sadh, Lun, &["asAtsIt"]);
    assert_has_tas(&[], &sadh, Lun, &["asAdDAm"]);

    let ash = d("aSU~\\", Svadi);
    assert_has_ta(&[], &ash, Lat, &["aSnute"]);
}

#[test]
fn sk_2533() {
    let ash = d("aSU~\\", Svadi);
    assert_has_ta(&[], &ash, Lit, &["AnaSe"]);
    assert_has_ta(&[], &ash, Lut, &["aSitA", "azwA"]);
    assert_has_ta(&[], &ash, Lrt, &["aSizyate", "akzyate"]);
    assert_has_ta(&[], &ash, VidhiLin, &["aSnuvIta"]);
    assert_has_ta(&[], &ash, AshirLin, &["akzIzwa", "aSizIzwa"]);
    assert_has_ta(&[], &ash, Lun, &["ASizwa", "Azwa"]);
    // ASizAtAm is justified by ASizwa above.
    assert_has_aataam(&[], &ash, Lun, &["ASizAtAm", "AkzAtAm"]);

    let stigh = d("zwiGa~\\", Svadi);
    assert_has_ta(&[], &stigh, Lat, &["stiGnute"]);
    assert_has_ta(&[], &stigh, Lit, &["tizwiGe"]);
    assert_has_ta(&[], &stigh, Lut, &["steGitA"]);

    let tik = d("tika~", Svadi);
    assert_has_tip(&[], &tik, Lat, &["tiknoti"]);

    let tig = d("tiga~", Svadi);
    assert_has_tip(&[], &tig, Lat, &["tignoti"]);

    let sagh = d("zaGa~", Svadi);
    assert_has_tip(&[], &sagh, Lat, &["saGnoti"]);

    let dhrsh = d("YiDfzA~", Svadi);
    assert_has_tip(&[], &dhrsh, Lat, &["DfzRoti"]);
    assert_has_tip(&[], &dhrsh, Lit, &["daDarza"]);
    assert_has_tip(&[], &dhrsh, Lut, &["DarzitA"]);

    let danbh = d("danBu~", Svadi);
    assert_has_tip(&[], &danbh, Lat, &["daBnoti"]);
    assert_has_tip(&[], &danbh, Lit, &["dadamBa"]);
    assert_has_tas(&[], &danbh, Lit, &["deBatuH", "dadamBatuH"]);
    // dadamBuH is justified by dadamBatuH above.
    assert_has_jhi(&[], &danbh, Lit, &["deBuH", "dadamBuH"]);
    assert_has_sip(&[], &danbh, Lit, &["dadamBiTa"]);
    assert_has_mip(&[], &danbh, Lit, &["dadamBa"]);
    // TODO: tip/sip/mip of deBa, deBiTa, deBa
    assert_has_tip(&[], &danbh, AshirLin, &["daByAt"]);

    let trp = d("tfpa~", Svadi);
    assert_has_tip(&[], &trp, Lat, &["tfpnoti"]);

    let ah = d("aha~", Svadi);
    assert_has_tip(&[], &ah, Lat, &["ahnoti"]);

    let dagh = d("daGa~", Svadi);
    assert_has_tip(&[], &dagh, Lat, &["daGnoti"]);

    let cam = d("camu~", Svadi);
    assert_has_tip(&[], &cam, Lat, &["camnoti"]);

    let ri = d("ri\\", Svadi);
    assert_has_tip(&[], &ri, Lat, &["riRoti"]);

    let kshi = d("kzi\\", Svadi);
    assert_has_tip(&[], &kshi, Lat, &["kziRoti"]);

    let rkshi = d("f\\kzi", Svadi);
    assert_has_tip(&[], &rkshi, Lat, &["fkziRoti"]);

    let ciri = d("ciri", Svadi);
    assert_has_tip(&[], &ciri, Lat, &["ciriRoti"]);

    let jiri = d("jiri", Svadi);
    assert_has_tip(&[], &jiri, Lat, &["jiriRoti"]);

    let dash = d("dASa~", Svadi);
    assert_has_tip(&[], &dash, Lat, &["dASnoti"]);
}
