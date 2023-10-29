extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::*;

#[test]
fn sutra_8_4_1() {
    assert_has_krdanta(&["AN"], &d("stFY", Kryadi), Krt::kta, &["AstIrRa"]);
    assert_has_krdanta(&["vi"], &d("SFY", Kryadi), Krt::kta, &["viSIrRa"]);
    assert_has_krdanta(&["ava"], &d("gUrI~\\", Divadi), Krt::kta, &["avagUrRa"]);

    // za
    assert_has_lat_p(&[], &d("kuza~", Kryadi), &["kuzRAti"]);
    assert_has_lat_p(&[], &d("puza~", Kryadi), &["puzRAti"]);
    assert_has_lat_p(&[], &d("muza~", Kryadi), &["muzRAti"]);

    // TODO: samAnapade
    assert_has_sandhi("agnis", "nayati", &["agnir nayati"]);
    assert_has_sandhi("vAyus", "nayati", &["vAyur nayati"]);
}

#[ignore]
#[test]
fn sutra_8_4_1_v1() {
    assert_has_sup_6p(&nyap("tri"), Stri, &["tisfRAm"]);
    assert_has_sup_6p(&nyap("catur"), Stri, &["catasfRAm"]);
    assert_has_sup_6p(&nyap("mAtf"), Stri, &["mAtFRAm"]);
    assert_has_sup_6p("pitf", Pum, &["pitFRAm"]);
}

#[test]
fn sutra_8_4_2() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::lyuw, &["karaRa"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), Krt::lyuw, &["haraRa"]);
    assert_has_sup_3s("kiri", Pum, &["kiriRA"]);
    assert_has_sup_3s("giri", Pum, &["giriRA"]);
    assert_has_sup_3s("kuru", Pum, &["kuruRA"]);
    assert_has_sup_3s("guru", Pum, &["guruRA"]);
    // kavarga
    assert_has_sup_3s("arka", Pum, &["arkeRa"]);
    assert_has_sup_3s("mUrKa", Pum, &["mUrKeRa"]);
    assert_has_sup_3s("garga", Pum, &["gargeRa"]);
    assert_has_sup_3s("arGa", Pum, &["arGeRa"]);
    // pavarga
    assert_has_sup_3s("darpa", Pum, &["darpeRa"]);
    assert_has_sup_3s("rePa", Pum, &["rePeRa"]);
    assert_has_sup_3s("garBa", Pum, &["garBeRa"]);
    assert_has_sup_3s("carman", Pum, &["carmaRA"]);
    assert_has_sup_3s("varman", Pum, &["varmaRA"]);

    // AN
    let nah = d("Ra\\ha~^", Divadi);
    assert_has_krdanta(&["pari", "AN"], &nah, Krt::kta, &["paryARadDa"]);
    assert_has_krdanta(&["nis", "AN"], &nah, Krt::kta, &["nirARadDa"]);

    // num (anusvAra)
    let bfmh = d("bfhi~", Curadi);
    assert_has_krdanta(&[], &bfmh, Krt::lyuw, &["bfMhaRa"]);
    assert_has_krdanta(&[], &bfmh, Krt::anIyar, &["bfMhaRIya"]);
    let tfmh = d("tfhi~", Bhvadi);
    assert_has_krdanta(&[], &tfmh, Krt::lyuw, &["tfMhaRa"]);
    assert_has_krdanta(&[], &tfmh, Krt::anIyar, &["tfMhaRIya"]);

    // num (not anusvAra)
    let inv = d("ivi~", Bhvadi);
    assert_has_krdanta(&["pra"], &inv, Krt::lyuw, &["prenvana"]);
    assert_has_krdanta(&["pra"], &inv, Krt::anIyar, &["prenvanIya"]);
}

// 8.4.3 - 8.4.13 are for a pUrvapada.

#[test]
fn sutra_8_4_14() {
    let nam = d("Ra\\ma~", Bhvadi);
    assert_has_lat(&["pra"], &nam, &["praRamati"]);
    assert_has_lat(&["pari"], &nam, &["pariRamati"]);

    let ni = d("RI\\Y", Bhvadi);
    assert_has_krdanta(&["pra"], &ni, Krt::Rvul, &["praRAyaka"]);
    assert_has_krdanta(&["pari"], &ni, Krt::Rvul, &["pariRAyaka"]);

    let nard = d("narda~", Bhvadi);
    assert_has_lat(&["pra"], &nard, &["pranardati"]);
    assert_has_krdanta(&["pra"], &nard, Krt::Rvul, &["pranardaka"]);
}

#[test]
fn sutra_8_4_15() {
    let hi = d("hi\\", Svadi);
    let mi = d("mI\\Y", Kryadi);
    assert_has_lat(&["pra"], &hi, &["prahiRoti"]);
    assert_has_lat_p(&["pra"], &mi, &["pramIRAti"]);
}

#[test]
fn sutra_8_4_16() {
    let vap = d("quva\\pa~^", Bhvadi);
    let yaa = d("yA\\", Adadi);
    assert_has_mip(&["pra"], &vap, Lot, &["pravapARi"]);
    assert_has_mip(&["pari"], &vap, Lot, &["parivapARi"]);
    assert_has_mip(&["pra"], &yaa, Lot, &["prayARi"]);
    assert_has_mip(&["pari"], &yaa, Lot, &["pariyARi"]);
}

#[test]
fn sutra_8_4_17() {
    let gad = d("gada~", Bhvadi);
    assert_has_lat(&["pra", "ni"], &gad, &["praRigadati"]);
    assert_has_lat(&["pari", "ni"], &gad, &["pariRigadati"]);

    let nad = d("Rada~", Bhvadi);
    assert_has_lat(&["pra", "ni"], &nad, &["praRinadati"]);
    assert_has_lat(&["pari", "ni"], &nad, &["pariRinadati"]);

    let pat = d("patx~", Bhvadi);
    assert_has_lat(&["pra", "ni"], &pat, &["praRipatati"]);
    assert_has_lat(&["pari", "ni"], &pat, &["pariRipatati"]);

    let pad = d("pa\\da~\\", Divadi);
    assert_has_lat(&["pra", "ni"], &pad, &["praRipadyate"]);
    assert_has_lat(&["pari", "ni"], &pad, &["pariRipadyate"]);

    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_lat_p(&["pra", "ni"], &daa, &["praRidadAti"]);
    assert_has_lat_p(&["pari", "ni"], &daa, &["pariRidadAti"]);

    let dhaa = d("quDA\\Y", Juhotyadi);
    assert_has_lat_p(&["pra", "ni"], &dhaa, &["praRidaDAti"]);
    assert_has_lat_p(&["pari", "ni"], &dhaa, &["pariRidaDAti"]);

    let maa = d("mA\\N", Juhotyadi);
    assert_has_lat(&["pra", "ni"], &maa, &["praRimimIte"]);
    assert_has_lat(&["pari", "ni"], &maa, &["pariRimimIte"]);

    let me = d("me\\N", Bhvadi);
    assert_has_lat(&["pra", "ni"], &me, &["praRimayate"]);
    assert_has_lat(&["pari", "ni"], &me, &["pariRimayate"]);

    let so = d("zo\\", Divadi);
    assert_has_lat(&["pra", "ni"], &so, &["praRizyati"]);
    assert_has_lat(&["pari", "ni"], &so, &["pariRizyati"]);

    let han = d("ha\\na~", Adadi);
    assert_has_lat(&["pra", "ni"], &han, &["praRihanti"]);
    assert_has_lat(&["pari", "ni"], &han, &["pariRihanti"]);

    let yaa = d("yA\\", Adadi);
    assert_has_lat(&["pra", "ni"], &yaa, &["praRiyAti"]);
    assert_has_lat(&["pari", "ni"], &yaa, &["pariRiyAti"]);

    let vaa = d("vA\\", Adadi);
    assert_has_lat(&["pra", "ni"], &vaa, &["praRivAti"]);
    assert_has_lat(&["pari", "ni"], &vaa, &["pariRivAti"]);

    let draa = d("drA\\", Adadi);
    assert_has_lat(&["pra", "ni"], &draa, &["praRidrAti"]);
    assert_has_lat(&["pari", "ni"], &draa, &["pariRidrAti"]);

    let psaa = d("psA\\", Adadi);
    assert_has_lat(&["pra", "ni"], &psaa, &["praRipsAti"]);
    assert_has_lat(&["pari", "ni"], &psaa, &["pariRipsAti"]);

    let vap = d("quva\\pa~^", Bhvadi);
    assert_has_lat_p(&["pra", "ni"], &vap, &["praRivapati"]);
    assert_has_lat_p(&["pari", "ni"], &vap, &["pariRivapati"]);

    let vah = d("va\\ha~^", Bhvadi);
    assert_has_lat_p(&["pra", "ni"], &vah, &["praRivahati"]);
    assert_has_lat_p(&["pari", "ni"], &vah, &["pariRivahati"]);

    let sham = d("Samu~", Divadi);
    assert_has_lat(&["pra", "ni"], &sham, &["praRiSAmyati"]);
    assert_has_lat(&["pari", "ni"], &sham, &["pariRiSAmyati"]);

    let ci = d("ci\\Y", Svadi);
    assert_has_lat_p(&["pra", "ni"], &ci, &["praRicinoti"]);
    assert_has_lat_p(&["pari", "ni"], &ci, &["pariRicinoti"]);

    let dih = d("di\\ha~^", Adadi);
    assert_has_lat_p(&["pra", "ni"], &dih, &["praRidegDi"]);
    assert_has_lat_p(&["pari", "ni"], &dih, &["pariRidegDi"]);

    // Also applies for aw-vyavAya
    assert_has_lan(&["pra", "ni"], &gad, &["praRyagadat"]);
    assert_has_lan(&["pari", "ni"], &gad, &["pariRyagadat"]);
}

#[test]
fn sutra_8_4_18() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_lat_p(&["pra", "ni"], &pac, &["praRipacati", "pranipacati"]);

    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_lat_p(&["pra", "ni"], &bhid, &["praRiBinatti", "praniBinatti"]);

    let kf = d("qukf\\Y", Tanadi);
    assert_has_lat_p(&["pra", "ni"], &kf, &["pranikaroti"]);

    let khaad = d("KAdf~", Bhvadi);
    assert_has_lat_p(&["pra", "ni"], &khaad, &["praniKAdati"]);

    let pish = d("pi\\zx~", Rudhadi);
    assert_has_lat_p(&["pra", "ni"], &pish, &["pranipinazwi"]);

    // Examples of why the rule has "upadesha"
    assert_has_lit_p(&["pra", "ni"], &kf, &["pranicakAra"]);
    assert_has_lit_p(&["pra", "ni"], &khaad, &["pranicaKAda"]);
    assert_has_lrt(&["pra", "ni"], &pish, &["pranipekzyati"]);

    let vish = d("vi\\Sa~", Tudadi);
    assert_has_lut(&["pra", "ni"], &vish, &["praRivezwA", "pranivezwA"]);
    // Per Neelesh, the `pranivekzyati` in the KV online is likely a typo.
    assert_has_lrt(&["pra", "ni"], &vish, &["praRivekzyate", "pranivekzyate"]);
}

#[test]
fn sutra_8_4_19() {
    let an = d("ana~", Adadi);
    assert_has_lat(&["pra"], &an, &["prARiti"]);
    assert_has_lat(&["parA"], &an, &["parARiti"]);
}

#[test]
fn sutra_8_4_21() {
    let an = d("ana~", Adadi);
    let nic = |d: &Dhatu| d.clone().with_sanadi(&[Sanadi::Nic]);
    assert_has_lat(&["pra"], &san(&an), &["prARiRizati"]);
    assert_has_lun_p(&["pra"], &nic(&an), &["prARiRat"]);
    assert_has_lat(&["parA"], &san(&an), &["parARiRizati"]);
    assert_has_lun_p(&["parA"], &nic(&an), &["parARiRat"]);
}

#[test]
fn sutra_8_4_22() {
    let han = d("ha\\na~", Adadi);
    assert_has_lat_karmani(&["pra"], &han, &["prahaRyate"]);
    assert_has_lat_karmani(&["pari"], &han, &["parihaRyate"]);
    assert_has_krdanta(&["pra"], &han, Krt::lyuw, &["prahaRana"]);
    assert_has_krdanta(&["pari"], &han, Krt::lyuw, &["parihaRana"]);

    // atpUrvasya
    assert_has_jhi(&["pra"], &han, Lat, &["praGnanti"]);
    assert_has_jhi(&["pari"], &han, Lat, &["pariGnanti"]);

    // taparakaraRa
    assert_has_lun_karmani(&["pra"], &han, &["prAGAni", "prAvaDi"]);
    assert_has_lun_karmani(&["pari"], &han, &["paryaGAni", "paryavaDi"]);
}

#[test]
fn sutra_8_4_23() {
    let han = d("ha\\na~", Adadi);
    assert_has_vas(&["pra"], &han, Lat, &["prahaRvaH", "prahanvaH"]);
    assert_has_vas(&["pari"], &han, Lat, &["parihaRvaH", "parihanvaH"]);
    assert_has_mas(&["pra"], &han, Lat, &["prahaRmaH", "prahanmaH"]);
    assert_has_mas(&["pari"], &han, Lat, &["parihaRmaH", "parihanmaH"]);
}

#[test]
fn sutra_8_4_35() {
    let paa = d("pA\\", Bhvadi);
    assert_has_krdanta(&["nis"], &paa, Krt::lyuw, &["nizpAna"]);
}

#[test]
fn sutra_8_4_36() {
    let nas = d("Ra\\Sa~", Divadi);
    assert_has_krdanta(&["pra"], &nas, Krt::kta, &["pranazwa"]);
    assert_has_krdanta(&["pari"], &nas, Krt::kta, &["parinazwa"]);
    // zAntasya
    assert_has_lat(&["pra"], &nas, &["praRaSyati"]);
    assert_has_lat(&["pari"], &nas, &["pariRaSyati"]);
    // others
    assert_has_lrt(&["pra"], &nas, &["pranaNkzyati", "praRaSizyati"]);
    assert_has_lrt(&["pari"], &nas, &["parinaNkzyati", "pariRaSizyati"]);
}

#[test]
fn sutra_8_4_37() {
    assert_has_sup_2p("vfkza", Pum, &["vfkzAn"]);
    assert_has_sup_2p("plakza", Pum, &["plakzAn"]);
    assert_has_sup_2p("ari", Pum, &["arIn"]);
    assert_has_sup_2p("giri", Pum, &["girIn"]);
}

#[test]
fn sutra_8_4_39() {
    let kzubh = d("kzuBa~", Kryadi);
    assert_has_tip(&[], &kzubh, Lat, &["kzuBnAti"]);
    assert_has_tas(&[], &kzubh, Lat, &["kzuBnItaH"]);
    assert_has_jhi(&[], &kzubh, Lat, &["kzuBnanti"]);
    // TODO: others
}

#[test]
fn sutra_8_4_40() {
    assert_has_sandhi("vfkzas", "Sete", &["vfkzaS Sete", "vfkzaH Sete"]);
    assert_has_sandhi("plakzas", "Sete", &["plakzaS Sete", "plakzaH Sete"]);
    assert_has_sandhi("vfkzas", "cinoti", &["vfkzaS cinoti"]);
    assert_has_sandhi("plakzas", "cinoti", &["plakzaS cinoti"]);
    assert_has_sandhi("vfkzas", "CAdayati", &["vfkzaS CAdayati"]);
    assert_has_sandhi("plakzas", "CAdayati", &["plakzaS CAdayati"]);

    assert_has_sandhi("agnicit", "Sete", &["agnicic Cete", "agnicic Sete"]);
    assert_has_sandhi("somasut", "Sete", &["somasuc Cete", "somasuc Sete"]);
    assert_has_sandhi("agnicit", "cinoti", &["agnicic cinoti"]);
    assert_has_sandhi("somasut", "cinoti", &["somasuc cinoti"]);
    assert_has_sandhi("agnicit", "CAdayati", &["agnicic CAdayati"]);
    assert_has_sandhi("somasut", "CAdayati", &["somasuc CAdayati"]);
    assert_has_sandhi("agnicit", "jayati", &["agnicij jayati"]);
    assert_has_sandhi("somasut", "jayati", &["somasuj jayati"]);
    assert_has_sandhi("agnicit", "JakAras", &["agnicij JakAraH"]);
    assert_has_sandhi("somasut", "JakAras", &["somasuj JakAraH"]);
    assert_has_sandhi("agnicit", "YakAras", &["agniciY YakAraH"]);
    assert_has_sandhi("somasut", "YakAras", &["somasuY YakAraH"]);

    assert_has_lat_p(&[], &d("wuma\\sjo~", Tudadi), &["majjati"]);
    assert_has_lat_p(&[], &d("Bra\\sja~^", Tudadi), &["Bfjjati"]);
    assert_has_lat_p(&[], &d("o~vrascU~", Tudadi), &["vfScati"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Tudadi), Krt::naN, &["yajYa"]);
    assert_has_krdanta(&[], &d("quyAcf~^", Tudadi), Krt::naN, &["yAcYA"]);
}

#[test]
fn sutra_8_4_41() {
    assert_has_sandhi("vfkzas", "zaRqe", &["vfkzaz zaRqe", "vfkzaH zaRqe"]);
    assert_has_sandhi("plakzas", "zaRqe", &["plakzaz zaRqe", "plakzaH zaRqe"]);
    assert_has_sandhi("vfkzas", "wIkate", &["vfkzaz wIkate"]);
    assert_has_sandhi("plakzas", "wIkate", &["plakzaz wIkate"]);
    assert_has_sandhi("vfkzas", "WakAraH", &["vfkzaz WakAraH"]);
    assert_has_sandhi("plakzas", "WakAraH", &["plakzaz WakAraH"]);

    let pish = d("pi\\zx~", Rudhadi);
    assert_has_krdanta(&[], &pish, Krt::tfc, &["pezwf"]);
    assert_has_krdanta(&[], &pish, Krt::tumun, &["pezwum"]);
    assert_has_krdanta(&[], &pish, Krt::tavya, &["pezwavya"]);
    let kf = d("qukf\\Y", Tanadi);
    assert_has_ashirlin_a(&[], &kf, &["kfzIzwa"]);
    assert_has_thaas(&[], &kf, AshirLin, &["kfzIzWAH"]);

    assert_has_sandhi("agnicit", "wIkate", &["agniciw wIkate"]);
    assert_has_sandhi("somasut", "wIkate", &["somasuw wIkate"]);
    assert_has_sandhi("agnicit", "WakAraH", &["agniciw WakAraH"]);
    assert_has_sandhi("somasut", "WakAraH", &["somasuw WakAraH"]);
    assert_has_sandhi("agnicit", "qInaH", &["agniciq qInaH"]);
    assert_has_sandhi("somasut", "qInaH", &["somasuq qInaH"]);
    assert_has_sandhi("agnicit", "QOkate", &["agniciq QOkate"]);
    assert_has_sandhi("somasut", "QOkate", &["somasuq QOkate"]);
    assert_has_sandhi("agnicit", "RakAraH", &["agniciR RakAraH"]);
    assert_has_sandhi("somasut", "RakAraH", &["somasuR RakAraH"]);

    // TODO: awwati, aqqati
}

#[ignore]
#[test]
fn sutra_8_4_42() {
    assert_has_sandhi("Svaliw", "sAye", &["Svaliw sAye", "Svaliw tsAye"]);
    assert_has_sandhi("maDuliw", "tarati", &["maDuliw tarati"]);
    // padAntAt?
    assert_has_lat_a(&[], &d("Iqa~\\", Adadi), &["Iwwe"]);
    // woH?
    assert_has_taddhitanta(&prati("sarpis"), T::tamap, &["sarpizwama"]);
    // // an-Am?
    assert_has_sup_6p("zaz", Pum, &["zaRRAm"]);
}

#[test]
fn sutra_8_4_43() {
    assert_has_sandhi("agnicit", "zaRqe", &["agnicit zaRqe"]);
    assert_has_sandhi("BavAn", "zaRqe", &["BavAn zaRqe"]);
    assert_has_sandhi("mahAn", "zaRqe", &["mahAn zaRqe"]);
}

#[test]
fn sutra_8_4_44() {
    assert_has_krdanta(&[], &d("pra\\Ca~", Tudadi), Krt::naN, &["praSna"]);
    assert_has_krdanta(&[], &d("viCa~", Tudadi), Krt::naN, &["viSna"]);
}

#[test]
fn sutra_8_4_45() {
    assert_has_sandhi("vAk", "nayati", &["vAN nayati"]);
    assert_has_sandhi("Svaliw", "nayati", &["SvaliR nayati"]);
    assert_has_sandhi("agnicit", "nayati", &["agnicin nayati"]);
    assert_has_sandhi("trizWup", "nayati", &["trizWum nayati"]);
    // padAntasya
    assert_has_mip(&[], &d("vida~", Adadi), Lat, &["veda", "vedmi"]);
    assert_has_tip(&[], &d("kzuBa~", Kryadi), Lat, &["kzuBnAti"]);
}

// 8.4.46 - 8.4.52 are the "dve" rules

#[test]
fn sutra_8_4_53() {
    let labh = d("qula\\Ba~\\z", Bhvadi);
    assert_has_krdanta(&[], &labh, Krt::tfc, &["labDf"]);
    assert_has_krdanta(&[], &labh, Krt::tumun, &["labDum"]);
    assert_has_krdanta(&[], &labh, Krt::tavya, &["labDavya"]);
    let duh = d("du\\ha~^", Adadi);
    assert_has_krdanta(&[], &duh, Krt::tfc, &["dogDf"]);
    assert_has_krdanta(&[], &duh, Krt::tumun, &["dogDum"]);
    assert_has_krdanta(&[], &duh, Krt::tavya, &["dogDavya"]);
    let budh = d("bu\\Da~\\", Divadi);
    assert_has_krdanta(&[], &budh, Krt::tfc, &["bodDf"]);
    assert_has_krdanta(&[], &budh, Krt::tumun, &["bodDum"]);
    assert_has_krdanta(&[], &budh, Krt::tavya, &["bodDavya"]);
    // JaSi
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_tas(&[], &daa, Lat, &["dattaH"]);
    assert_has_thas(&[], &daa, Lat, &["datTaH"]);
    assert_has_mas(&[], &d("quDA\\Y", Juhotyadi), Lat, &["daDmaH"]);
}

#[test]
fn sutra_8_4_54() {
    assert_has_lat_p(&[], &san(&d("Kanu~^", Bhvadi)), &["ciKanizati"]);
    assert_has_lat_p(&[], &san(&d("Ci\\di~^r", Rudhadi)), &["cicCitsati"]);
    assert_has_lat_p(&[], &san(&d("zWA\\", Bhvadi)), &["tizWAsati"]);
    assert_has_lat(&[], &san(&d("BU", Bhvadi)), &["buBUzati"]);
    assert_has_lat(&[], &san(&d("a\\da~", Adadi)), &["jiGatsati"]);
    assert_has_lat(&[], &san(&d("QOkf~\\", Bhvadi)), &["quQOkizate"]);
    // TODO: others
}

#[test]
fn sutra_8_4_55() {
    let bhid = d("Bi\\di~^r", Rudhadi);
    assert_has_krdanta(&[], &bhid, Krt::tfc, &["Bettf"]);
    assert_has_krdanta(&[], &bhid, Krt::tumun, &["Bettum"]);
    assert_has_krdanta(&[], &bhid, Krt::tavya, &["Bettavya"]);
    assert_has_lat(&[], &san(&d("yu\\Da~\\", Divadi)), &["yuyutsate"]);
    assert_has_lat(&["AN"], &san(&d("ra\\Ba~\\", Bhvadi)), &["Aripsate"]);
    assert_has_lat(&["AN"], &san(&d("qula\\Ba~\\z", Bhvadi)), &["Alipsate"]);
}

#[test]
fn sutra_8_4_56() {
    use Vacana::*;
    use Vibhakti::*;
    assert_has_subantas_raw("vAc", Stri, Prathama, Eka, &["vAk", "vAg"]);
    assert_has_subantas_raw("tvac", Stri, Prathama, Eka, &["tvak", "tvag"]);
    assert_has_subantas_raw("Svalih", Stri, Prathama, Eka, &["Svaliw", "Svaliq"]);
    assert_has_subantas_raw("trizwuB", Stri, Prathama, Eka, &["trizwup", "trizwub"]);
}

#[test]
fn sutra_8_4_58() {
    let sank = d("Saki~\\", Bhvadi);
    assert_has_krdanta(&[], &sank, Krt::tfc, &["SaNkitf"]);
    assert_has_krdanta(&[], &sank, Krt::tumun, &["SaNkitum"]);
    assert_has_krdanta(&[], &sank, Krt::tavya, &["SaNkitavya"]);
    let unch = d("uCi~", Bhvadi);
    assert_has_krdanta(&[], &unch, Krt::tfc, &["uYCitf"]);
    assert_has_krdanta(&[], &unch, Krt::tumun, &["uYCitum"]);
    assert_has_krdanta(&[], &unch, Krt::tavya, &["uYCitavya"]);
    let kund = d("kuqi~\\", Bhvadi);
    assert_has_krdanta(&[], &kund, Krt::tfc, &["kuRqitf"]);
    assert_has_krdanta(&[], &kund, Krt::tumun, &["kuRqitum"]);
    assert_has_krdanta(&[], &kund, Krt::tavya, &["kuRqitavya"]);
    let nand = d("wunadi~", Bhvadi);
    assert_has_krdanta(&[], &nand, Krt::tfc, &["nanditf"]);
    assert_has_krdanta(&[], &nand, Krt::tumun, &["nanditum"]);
    assert_has_krdanta(&[], &nand, Krt::tavya, &["nanditavya"]);
    let kamp = d("kapi~\\", Bhvadi);
    assert_has_krdanta(&[], &kamp, Krt::tfc, &["kampitf"]);
    assert_has_krdanta(&[], &kamp, Krt::tumun, &["kampitum"]);
    assert_has_krdanta(&[], &kamp, Krt::tavya, &["kampitavya"]);
}

#[test]
fn sutra_8_4_60() {
    assert_has_sandhi("agnicit", "lunAti", &["agnicil lunAti"]);
    assert_has_sandhi("somasut", "lunAti", &["somasul lunAti"]);
    assert_has_sandhi("BavAn", "lunAti", &["BavA~l lunAti"]);
}

#[test]
fn sutra_8_4_62() {
    assert_has_sandhi("vAk", "hasati", &["vAg Gasati", "vAg hasati"]);
    assert_has_sandhi("svaliw", "hasati", &["svaliq Qasati", "svaliq hasati"]);
    assert_has_sandhi("agnicit", "hasati", &["agnicid Dasati", "agnicid hasati"]);
    assert_has_sandhi("somasut", "hasati", &["somasud Dasati", "somasud hasati"]);
    assert_has_sandhi("trizWup", "hasati", &["trizWub Basati", "trizWub hasati"]);

    // JayaH?
    assert_has_sandhi("prAN", "hasati", &["prAN hasati"]);
    assert_has_sandhi("BavAn", "hasati", &["BavAn hasati"]);
}

#[ignore]
#[test]
fn sutra_8_4_63() {
    assert_has_sandhi("vAk", "Sete", &["vAk Cete", "vAk Sete"]);
    assert_has_sandhi(
        "agnicit",
        "Sete",
        &["agnicic Cete", "agnicic Sete", "agnicit Sete"],
    );
    assert_has_sandhi(
        "somasut",
        "Sete",
        &["somasuc Cete", "somasuc Sete", "somasut Sete"],
    );
    assert_has_sandhi("swaliw", "Sete", &["svaliw Cete", "svaliw Sete"]);
    assert_has_sandhi("trizWup", "Sete", &["trizWup Cete", "trizWup Sete"]);
}
