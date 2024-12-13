// Test cases from the Siddhantakaumudi commentary on the Unadipatha.

extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Unadi as U;
use vidyut_prakriya::args::Unadi::*;

#[test]
fn unadi_1_1() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), uR, &["kAru"]);
    assert_has_krdanta(&[], &d("pA\\", Bhvadi), uR, &["pAyu"]);
    assert_has_krdanta(&[], &d("vA\\", Adadi), uR, &["vAyu"]);
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), uR, &["jAyu"]);
    assert_has_krdanta(&[], &d("qumi\\Y", Svadi), uR, &["mAyu"]);
    assert_has_krdanta(&[], &d("zvada~\\", Bhvadi), uR, &["svAdu"]);
    assert_has_krdanta(&[], &d("sA\\Da~", Svadi), uR, &["sADu"]);
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), uR, &["ASu"]);
}

#[test]
fn unadi_1_2() {
    let t = Tester::with_chaandasa();
    t.assert_has_krdanta(&[], &d("i\\R", Adadi), uR, &["Ayu"]);
}

#[test]
fn unadi_1_3() {
    assert_has_krdanta(&[], &d("dF", Kryadi), YuR, &["dAru"]);
    assert_has_krdanta(&[], &d("zaRa~", Bhvadi), YuR, &["sAnu"]);
    assert_has_krdanta(&[], &d("janI~", Divadi), YuR, &["jAnu"]);
    assert_has_krdanta(&[], &d("cara~", Bhvadi), YuR, &["cAru"]);
    assert_has_krdanta(&[], &d("cawe~", Bhvadi), YuR, &["cAwu"]);
}

#[test]
fn unadi_1_4() {
    assert_has_krdanta(&["kim"], &d("Sru\\", Bhvadi), YuR, &["kiMSAru"]);
    assert_has_krdanta(&["jarA"], &d("i\\R", Adadi), YuR, &["jarAyu"]);
}

#[test]
fn unadi_1_5() {
    assert_has_krdanta(&[], &d("tF", Bhvadi), YuR, &["tAlu"]);
}

#[test]
fn unadi_1_6() {
    assert_has_krdanta(&["kfka"], &d("va\\ca~", Adadi), YuR, &["kfkavAku"]);
}

#[test]
fn unadi_1_7() {
    assert_has_krdanta(&[], &d("Bf\\Y", Bhvadi), u, &["Baru"]);
    assert_has_krdanta(&[], &d("quBf\\Y", Juhotyadi), u, &["Baru"]);
    assert_has_krdanta(&[], &d("mf\\N", Bhvadi), u, &["maru"]);
    assert_has_krdanta(&[], &d("SIN", Adadi), u, &["Sayu"]);
    assert_has_krdanta(&[], &d("cara~", Bhvadi), u, &["caru"]);
    assert_has_krdanta(&[], &d("tsara~", Bhvadi), u, &["tsaru"]);
    assert_has_krdanta(&[], &d("tanu~^", Bhvadi), u, &["tanu"]);
    assert_has_krdanta(&[], &d("Dana~", Juhotyadi), u, &["Danu"]);
    assert_has_krdanta(&[], &d("qumi\\Y", Bhvadi), u, &["mayu"]);
    assert_has_krdanta(&[], &d("wuma\\sjo~", Tudadi), u, &["madgu"]);
}

#[test]
fn unadi_1_8() {
    assert_has_krdanta(&[], &d("aRa~\\", Divadi), u, &["aRu"]);
    assert_has_krdanta(&[], &d("kawe~", Bhvadi), u, &["kawu"]);
    assert_has_krdanta(&[], &d("vawa~", Bhvadi), u, &["vawu"]);
}

#[test]
fn unadi_1_9() {
    assert_has_krdanta(&[], &d("aRa~\\", Divadi), u, &["aRu"]);
}

#[test]
fn unadi_1_10() {
    assert_has_krdanta(&[], &d("SF", Kryadi), u, &["Saru"]);
    assert_has_krdanta(&[], &d("svf", Bhvadi), u, &["svaru"]);
    assert_has_krdanta(&[], &d("zRi\\ha~", Divadi), u, &["snehu"]);
    assert_has_krdanta(&[], &d("trapU~\\z", Bhvadi), u, &["trapu"]);
    assert_has_krdanta(&[], &d("asa~", Adadi), u, &["asu"]);
    assert_has_krdanta(&[], &d("va\\sa~", Bhvadi), u, &["vasu"]);
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), u, &["hanu"]);
    assert_has_krdanta(&[], &d("klidU~", Divadi), u, &["kledu"]);
    assert_has_krdanta(&[], &d("ba\\nDa~", Kryadi), u, &["banDu"]);
    assert_has_krdanta(&[], &d("ma\\na~\\", Divadi), u, &["manu"]);
}

#[test]
fn unadi_1_11() {
    assert_has_krdanta(&[], &d("syandU~\\", Bhvadi), u, &["sinDu"]);
}

#[test]
fn unadi_1_12() {
    assert_has_krdanta(&[], &d("undI~", Rudhadi), u, &["indu"]);
}

#[test]
fn unadi_1_13() {
    assert_has_krdanta(&[], &d("Iza~\\", Adadi), u, &["izu"]);
}

#[test]
fn unadi_1_14() {
    assert_has_krdanta(&[], &d("ska\\ndi~r", Bhvadi), u, &["kandu"]);
}

#[test]
fn unadi_1_15() {
    assert_has_krdanta(&[], &d("sf\\ja~", Tudadi), u, &["rajju"]);
}

#[test]
fn unadi_1_16() {
    assert_has_krdanta(&[], &d("kftI~", Rudhadi), u, &["tarku"]);
}

#[test]
fn unadi_1_17() {
    assert_has_krdanta(&["ni"], &d("ancu~^", Bhvadi), u, &["nyaNku"]);
}

#[test]
fn unadi_1_19() {
    assert_has_krdanta(&[], &d("vala~\\", Bhvadi), u, &["valgu"]);
}

#[test]
fn unadi_1_20() {
    assert_has_krdanta(&[], &d("So\\", Divadi), u, &["SiSu"]);
}

#[test]
fn unadi_1_21() {
    assert_has_krdanta(&[], &d("yA\\", Adadi), u, &["yayu"]);
}

#[test]
fn unadi_1_22() {
    assert_has_krdanta(&[], &d("Bf\\Y", Bhvadi), ku, &["baBru"]);
}

#[test]
fn unadi_1_23() {
    assert_has_krdanta(&[], &d("pF", Kryadi), ku, &["puru"]);
    assert_has_krdanta(&[], &d("Bi\\di~^r", Rudhadi), ku, &["Bidu"]);
    assert_has_krdanta(&[], &d("vya\\Da~", Divadi), ku, &["viDu"]);
    assert_has_krdanta(&[], &d("gfDu~", Divadi), ku, &["gfDu"]);
    assert_has_krdanta(&[], &d("YiDfzA~", Svadi), ku, &["Dfzu"]);
}

#[test]
fn unadi_1_24() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), ku, &["kuru"]);
    assert_has_krdanta(&[], &d("gF", Kryadi), ku, &["guru"]);
}

#[ignore]
#[test]
fn unadi_1_25() {
    let stha = d("zWA\\", Bhvadi);
    assert_has_krdanta(&["apa"], &stha, ku, &["apazWu"]);
    assert_has_krdanta(&["dus"], &stha, ku, &["duHzWu", "duzzWu"]);
    assert_has_krdanta(&["su"], &stha, ku, &["suzWu"]);
}

#[test]
fn unadi_1_26() {
    assert_has_krdanta(&[], &d("rapa~", Bhvadi), ku, &["ripu"]);
}

#[ignore]
#[test]
fn unadi_1_27() {
    assert_has_krdanta(&[], &d("arja~", Curadi), ku, &["fju"]);
    assert_has_krdanta(&[], &d("df\\Si~r", Bhvadi), ku, &["paSu"]);
    assert_has_krdanta(&[], &d("kamu~\\", Bhvadi), ku, &["kantu"]);
    assert_has_krdanta(&[], &d("ama~", Bhvadi), ku, &["anDu"]);
    assert_has_krdanta(&[], &d("paSa~", Curadi), ku, &["pAMSu"]);
    assert_has_krdanta(&[], &d("bADf~\\", Bhvadi), ku, &["bAhu"]);
}

#[test]
fn unadi_1_28() {
    assert_has_krdanta(&[], &d("praTa~", Bhvadi), ku, &["pfTu"]);
    assert_has_krdanta(&[], &d("mrada~\\", Bhvadi), ku, &["mfdu"]);
    assert_has_krdanta(&[], &d("Bra\\sja~^", Bhvadi), ku, &["Bfgu"]);
}

#[test]
fn unadi_1_29() {
    assert_has_krdanta(&[], &d("laGi~\\", Bhvadi), ku, &["laGu"]);
    assert_has_krdanta(&[], &d("bahi~\\", Bhvadi), ku, &["bahu"]);
}

#[test]
fn unadi_1_30_and_unadi_1_31() {
    assert_has_krdanta(&[], &d("UrRuY", Adadi), ku, &["Uru", "uru"]);
}

#[test]
fn unadi_1_32() {
    assert_has_krdanta(&[], &d("Slizu~", Bhvadi), ku, &["Sliku"]);
}

#[ignore]
#[test]
fn unadi_1_33() {
    assert_has_krdanta(&["AN"], &d("Kanu~^", Bhvadi), ku, &["AKu"]);
    assert_has_krdanta(&["para"], &d("SF", Kryadi), ku, &["paraSu"]);
}

#[test]
fn unadi_1_34() {
    let dru = d("dru\\", Bhvadi);
    assert_has_krdanta(&["hari"], &dru, ku, &["haridru"]);
    assert_has_krdanta(&["mita"], &dru, ku, &["mitadru"]);
}

#[test]
fn unadi_1_35() {
    assert_has_krdanta(&["Sata"], &d("dru\\", Bhvadi), ku, &["Satadru"]);
    assert_has_krdanta(&[], &d("dru\\", Bhvadi), ku, &["dru"]);
}

#[test]
fn unadi_1_39() {
    assert_has_krdanta(&[], &d("vyaTa~\\", Bhvadi), urac, &["viTura"]);
}

#[test]
fn unadi_1_42() {
    assert_has_krdanta(&[], &d("asa~", Adadi), uran, &["asura", "Asura"]);
}

#[test]
fn unadi_1_45() {
    assert_has_krdanta(&[], &d("ava~", Bhvadi), wizac, &["aviza"]);
    // TODO: right mah?
    assert_has_krdanta(&[], &d("maha~", Bhvadi), wizac, &["mahiza"]);
}

#[test]
fn unadi_1_46() {
    assert_has_krdanta(&[], &d("ama~", Bhvadi), wizac, &["Amiza"]);
}

#[test]
fn unadi_1_47() {
    assert_has_krdanta(&[], &d("ru\\ha~", Bhvadi), wizac, &["rOhiza"]);
}

#[test]
fn unadi_1_50() {
    assert_has_krdanta(&[], &d("kila~", Tudadi), wizac, &["kilbiza"]);
}

#[ignore]
#[test]
fn unadi_1_52() {
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), kirac, &["ASira"]);
}

#[test]
fn unadi_1_55() {
    assert_has_krdanta(&[], &d("kamu~\\", Bhvadi), ilac, &["kapila"]);
}

#[test]
fn unadi_1_57() {
    assert_has_krdanta(&[], &d("maTe~", Bhvadi), ilac, &["miTilA"]);
    assert_has_krdanta(&[], &d("paTe~", Bhvadi), ilac, &["paTila"]);
}

#[test]
fn unadi_1_58() {
    assert_has_krdanta(&[], &d("patx~", Bhvadi), erak, &["patera"]);
    assert_has_krdanta(&[], &d("kaWa~", Bhvadi), erak, &["kaWera"]);
    assert_has_krdanta(&[], &d("kuWi~", Bhvadi), erak, &["kuWera"]);
    assert_has_krdanta(&[], &d("gaqi~", Bhvadi), erak, &["gaqera"]);
    assert_has_krdanta(&[], &d("guqa~", Tudadi), erak, &["guqera"]);
    assert_has_krdanta(&[], &d("da\\nSa~", Bhvadi), erak, &["daSera"]);
}

#[test]
fn unadi_1_59() {
    assert_has_krdanta(&[], &d("kubi~", Bhvadi), erak, &["kubera"]);
}

#[test]
fn unadi_1_60() {
    assert_has_krdanta(&[], &d("Sa\\dx~", Bhvadi), erak, &["Satera"]);
}

#[test]
fn unadi_1_61() {
    assert_has_krdanta(&[], &d("mUla~", Bhvadi), erak, &["mUlera"]);
    assert_has_krdanta(&[], &d("guDa~", Divadi), erak, &["guDera"]);
    assert_has_krdanta(&[], &d("guhU~^", Bhvadi), erak, &["guhera"]);
    assert_has_krdanta(&[], &d("mu\\ha~", Divadi), erak, &["muhera"]);
}

#[test]
fn unadi_1_62() {
    assert_has_krdanta(&[], &d("kabf~\\", Bhvadi), otac, &["kapota"]);
}

#[test]
fn unadi_1_63() {
    assert_has_krdanta(&[], &d("BA\\", Adadi), qavatu, &["Bavat"]);
}

#[test]
fn unadi_1_64() {
    assert_has_krdanta(&[], &d("kaWa~", Bhvadi), oran, &["kaWora"]);
    assert_has_krdanta(&[], &d("caka~", Bhvadi), oran, &["cakora"]);
}

#[test]
fn unadi_1_66() {
    assert_has_krdanta(&[], &d("kapi~\\", Bhvadi), olac, &["kapola"]);
    assert_has_krdanta(&[], &d("gaqa~", Bhvadi), olac, &["gaqola"]);
    assert_has_krdanta(&[], &d("gaqi~", Bhvadi), olac, &["gaRqola"]);
    assert_has_krdanta(&[], &d("kawe~", Bhvadi), olac, &["kawola"]);
    assert_has_krdanta(&[], &d("pawa~", Bhvadi), olac, &["pawola"]);
}

#[test]
fn unadi_1_67() {
    assert_has_krdanta(&[], &d("mI\\Y", Kryadi), Uran, &["mayUra"]);
}

#[test]
fn unadi_1_68() {
    assert_has_krdanta(&[], &d("syandU~\\", Bhvadi), Uran, &["sindUra"]);
}

#[test]
fn unadi_1_69() {
    assert_has_krdanta(&[], &d("zi\\Y", Svadi), tun, &["setu"]);
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), tun, &["tantu"]);
    // "gAntu" by 5.43
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), tun, &["gantu", "gAntu"]);
    assert_has_krdanta(&[], &d("masI~", Divadi), tun, &["mastu"]);
    assert_has_krdanta(&[], &d("zaca~\\", Bhvadi), tun, &["saktu"]);
    // TODO: return to this later
    // assert_has_krdanta(&[], &d("ava~", Bhvadi), Krt::tun, &["otu"]);
    assert_has_krdanta(&[], &d("quDA\\Y", Juhotyadi), tun, &["DAtu"]);
    assert_has_krdanta(&[], &d("kru\\Sa~", Bhvadi), tun, &["krozwu"]);
}

#[ignore]
#[test]
fn unadi_1_70() {
    assert_has_krdanta(&[], &d("pA\\", Bhvadi), tun, &["pitu"]);
}

#[test]
fn unadi_1_71() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), tu, &["ftu"]);
}

#[test]
fn unadi_1_72() {
    assert_has_krdanta(&[], &d("kamu~\\", Bhvadi), tu, &["kantu"]);
    assert_has_krdanta(&[], &d("ma\\na~\\", Divadi), tu, &["mantu"]);
    // "jartu" is from 5.46
    assert_has_krdanta(&[], &d("janI~\\", Divadi), tu, &["jantu", "jartu"]);
    assert_has_krdanta(&[], &d("gA\\", Juhotyadi), tu, &["gAtu"]);
    assert_has_krdanta(&[], &d("BA\\", Adadi), tu, &["BAtu"]);
    assert_has_krdanta(&[], &d("yA\\", Adadi), tu, &["yAtu"]);
    assert_has_krdanta(&[], &d("hi\\", Svadi), tu, &["hetu"]);
}

#[test]
fn unadi_1_73() {
    assert_has_krdanta(&[], &d("cAyf~^", Bhvadi), tu, &["ketu"]);
}

#[test]
fn unadi_1_74() {
    assert_has_krdanta(&[], &d("A\\px~", Svadi), tu, &["aptu"]);
}

#[test]
fn unadi_1_75_and_unadi_1_76() {
    assert_has_krdanta(&[], &d("va\\sa~", Bhvadi), tu, &["vastu", "vAstu"]);
}

#[test]
fn unadi_1_77() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), katu, &["kratu"]);
}

#[test]
fn unadi_1_78() {
    assert_has_krdanta(&[], &d("eDa~\\", Bhvadi), catu, &["eDatu"]);
    assert_has_krdanta(&[], &d("va\\ha~^", Bhvadi), catu, &["vahatu"]);
}

#[test]
fn unadi_1_79() {
    assert_has_krdanta(&[], &d("jIva~", Bhvadi), Atu, &["jIvAtu"]);
}

#[test]
fn unadi_1_80() {
    assert_has_krdanta(&[], &d("jIva~", Bhvadi), Atfkan, &["jEvAtfka"]);
}

#[ignore]
#[test]
fn unadi_1_81() {
    assert_has_krdanta(&[], &d("camu~", Bhvadi), U, &["camU"]);
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), U, &["tanU"]);
    assert_has_krdanta(&[], &d("Dana~", Juhotyadi), U, &["DanU"]);
    // TODO: where does "sarja" come from?
    assert_has_krdanta(&[], &d("sarja~", Bhvadi), U, &["sarjU"]);
    assert_has_krdanta(&[], &d("Karja~", Bhvadi), U, &["KarjU"]);
}

#[test]
fn unadi_1_82() {
    assert_has_krdanta(&[], &d("mfjU~", Adadi), U, &["marjU"]);
}

#[test]
fn unadi_1_83() {
    assert_has_krdanta(&[], &d("va\\ha~^", Bhvadi), U, &["vaDU"]);
}

#[test]
fn unadi_1_84() {
    assert_has_krdanta(&[], &d("kaza~", Bhvadi), U, &["kacCU"]);
}

#[test]
fn unadi_1_85() {
    assert_has_krdanta(&[], &d("kasa~\\", Adadi), U, &["kAsU"]);
    assert_has_krdanta(&[], &d("pa\\da~\\", Divadi), U, &["pAdU"]);
    assert_has_krdanta(&[], &d("f\\", Bhvadi), U, &["ArU"]);
}

#[test]
fn unadi_1_86() {
    assert_has_krdanta(&[], &d("aRa~", Bhvadi), U, &["AqU"]);
}

#[test]
fn unadi_1_89() {
    assert_has_krdanta(&[], &d("tF", Bhvadi), U, &["tardU"]);
}

#[test]
fn unadi_1_90() {
    assert_has_krdanta(&[], &d("daridrA", Adadi), U, &["dardrU"]);
}

#[test]
fn unadi_1_91() {
    assert_has_krdanta(&[], &d("nftI~", Divadi), kU, &["nftU"]);
    assert_has_krdanta(&[], &d("SfDu~\\", Bhvadi), kU, &["SfDU"]);
}

#[test]
fn unadi_1_92() {
    assert_has_krdanta(&[], &d("fti", Bhvadi), kU, &["rantU"]);
}

#[test]
fn unadi_1_94() {
    assert_has_krdanta(&[], &d("mf\\N", Tudadi), uti, &["marut"]);
    assert_has_krdanta(&[], &d("gf\\", Bhvadi), uti, &["garut"]);
}

#[test]
fn unadi_1_95() {
    assert_has_krdanta(&[], &d("gF", Tudadi), uti, &["garmut"]);
}

#[test]
fn unadi_1_96() {
    assert_has_krdanta(&[], &d("hfza~", Divadi), ulac, &["harzula"]);
    assert_has_krdanta(&[], &d("cawe~", Bhvadi), ulac, &["cawula"]);
}

#[test]
fn unadi_1_97() {
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), iti, &["harit"]);
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), iti, &["sarit"]);
    assert_has_krdanta(&[], &d("ru\\ha~", Bhvadi), iti, &["rohit"]);
    // TODO: where does yuz come from?
    assert_has_krdanta(&[], &d("yuza~", Bhvadi), iti, &["yozit"]);
}

#[ignore]
#[test]
fn unadi_1_98() {
    assert_has_krdanta(&[], &d("taqa~", Curadi), iti, &["taqit"]);
}

#[test]
fn unadi_1_99() {
    assert_has_krdanta(&[], &d("Samo~", Bhvadi), Qa, &["SaRQa"]);
}

#[test]
fn unadi_1_100() {
    assert_has_krdanta(&[], &d("kamu~\\", Bhvadi), aWa, &["kamaWa"]);
}

#[test]
fn unadi_1_101() {
    assert_has_krdanta(&[], &d("ra\\ma~\\", Bhvadi), aWa, &["rAmaWa"]);
}

#[test]
fn unadi_1_102() {
    assert_has_krdanta(&[], &d("Samo~", Bhvadi), Ka, &["SaNKa"]);
}

#[test]
fn unadi_1_103() {
    assert_has_krdanta(&[], &d("kaRa~", Bhvadi), Wa, &["kaRWa"]);
}

#[test]
fn unadi_1_105() {
    assert_has_krdanta(&[], &d("Sa\\pa~^", Bhvadi), kala, &["Sabala"]);
}

#[test]
fn unadi_1_107() {
    assert_has_krdanta(&[], &d("mfjU~", Adadi), kala, &["mala"]);
}

#[test]
fn unadi_1_108() {
    assert_has_krdanta(&[], &d("cupa~", Bhvadi), kala, &["capala"]);
}

#[test]
fn unadi_1_109() {
    assert_has_krdanta(&[], &d("Sa\\kx~", Svadi), kala, &["Sakala"]);
    assert_has_krdanta(&[], &d("Samo~", Bhvadi), kala, &["Samala"]);
}

#[test]
fn unadi_1_110() {
    assert_has_krdanta(&[], &d("Co\\", Divadi), kala, &["Cagala"]);
}

#[ignore]
#[test]
fn unadi_1_111() {
    assert_has_krdanta(&[], &d("damu~", Divadi), qa, &["daRqa"]);
    assert_has_krdanta(&[], &d("ra\\ma~\\", Bhvadi), qa, &["raRqa"]);
    assert_has_krdanta(&[], &d("Kanu~^", Bhvadi), qa, &["KaRqa"]);
    assert_has_krdanta(&[], &d("BaRa~", Bhvadi), qa, &["BaRqa"]);
    assert_has_krdanta(&[], &d("wuvama~", Bhvadi), qa, &["vaRqa"]);
    assert_has_krdanta(&[], &d("ama~", Bhvadi), qa, &["aRqa"]);
    assert_has_krdanta(&[], &d("", Bhvadi), qa, &["zaRqa"]);
    assert_has_krdanta(&[], &d("", Bhvadi), qa, &["SaRqa"]);
    assert_has_krdanta(&[], &d("", Bhvadi), qa, &["gaRqa"]);
    assert_has_krdanta(&[], &d("", Bhvadi), qa, &["caRqa"]);
    assert_has_krdanta(&[], &d("", Bhvadi), qa, &["paRqa"]);
}

#[test]
fn unadi_1_114() {
    assert_has_krdanta(&[], &d("patx~", Bhvadi), AlaY, &["pAtAla"]);
    assert_has_krdanta(&[], &d("caqi~\\", Bhvadi), AlaY, &["caRqAla"]);
}

#[test]
fn unadi_1_116() {
    assert_has_krdanta(&[], &d("patx~", Bhvadi), aNgac, &["pataNga"]);
}

#[test]
fn unadi_1_117() {
    assert_has_krdanta(&[], &d("tF", Bhvadi), aNgac, &["taraNga"]);
    assert_has_krdanta(&[], &d("lUY", Kryadi), aNgac, &["lavaNga"]);
}

#[test]
fn unadi_1_119() {
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), aNgac, &["sAraNga"]);
    assert_has_krdanta(&[], &d("vfY", Svadi), aNgac, &["vAraNga"]);
}

#[ignore]
#[test]
fn unadi_1_120() {
    // TODO: where does strI come from?
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), gan, &["gaNgA"]);
    assert_has_krdanta(&[], &d("a\\da~", Adadi), gan, &["adga"]);
}

#[ignore]
#[test]
fn unadi_1_121() {
    assert_has_krdanta(&[], &d("Co\\", Divadi), gan, &["CAga"]);
    assert_has_krdanta(&[], &d("pUN", Bhvadi), gan, &["pUga"]);
}

#[ignore]
#[test]
fn unadi_1_122() {
    assert_has_krdanta(&[], &d("Bf\\Y", Bhvadi), gan, &["BfNga"]);
}

#[ignore]
#[test]
fn unadi_1_123() {
    assert_has_krdanta(&[], &d("SF", Kryadi), gan, &["SfNga"]);
}

#[test]
fn unadi_1_124() {
    assert_has_krdanta(&[], &d("SF", Kryadi), gaR, &["SArNga"]);
}

#[test]
fn unadi_1_125() {
    assert_has_krdanta(&[], &d("muda~\\", Bhvadi), gak, &["mudga"]);
    // TODO: gf or gF?
    assert_has_krdanta(&[], &d("gf\\", Bhvadi), ga, &["garga"]);
}

#[test]
fn unadi_1_126() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), aRqan, &["karaRqa"]);
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), aRqan, &["saraRqa"]);
    assert_has_krdanta(&[], &d("Bf\\Y", Bhvadi), aRqan, &["BaraRqa"]);
    assert_has_krdanta(&[], &d("vfY", Svadi), aRqan, &["varaRqa"]);
}

#[test]
fn unadi_1_127() {
    assert_has_krdanta(&[], &d("SF", Kryadi), adi, &["Sarad"]);
    assert_has_krdanta(&[], &d("dF", Bhvadi), adi, &["darad"]);
    assert_has_krdanta(&[], &d("Basa~", Juhotyadi), adi, &["Basad"]);
}

#[test]
fn unadi_1_128() {
    assert_has_krdanta(&[], &d("dF", Kryadi), adi, &["dfzad"]);
}

#[test]
fn unadi_1_129() {
    assert_has_krdanta(&[], &d("tya\\ja~", Bhvadi), adi, &["tyad"]);
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), adi, &["tad"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), adi, &["yad"]);
}

#[test]
fn unadi_1_130() {
    assert_has_krdanta(&[], &d("i\\R", Adadi), adi, &["etad"]);
}

#[test]
fn unadi_1_131() {
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), awi, &["saraw"]);
}

#[test]
fn unadi_1_132() {
    assert_has_krdanta(&[], &d("laGi~\\", Bhvadi), awi, &["laGaw"]);
}

#[test]
fn unadi_1_135() {
    assert_has_krdanta(&[], &d("YiBI\\", Juhotyadi), aji, &["Bizaj"]);
}

#[test]
fn unadi_1_137() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), man, &["arma"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), man, &["stoma"]);
    assert_has_krdanta(&[], &d("zu\\Y", Svadi), man, &["soma"]);
    assert_has_krdanta(&[], &d("hu\\", Juhotyadi), man, &["homa"]);
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), man, &["sarma"]);
    assert_has_krdanta(&[], &d("Df\\N", Bhvadi), man, &["Darma"]);
    assert_has_krdanta(&[], &d("wukzu", Adadi), man, &["kzoma", "kzOma"]);
    assert_has_krdanta(&[], &d("BA\\", Adadi), man, &["BAma"]);
    assert_has_krdanta(&[], &d("yA\\", Adadi), man, &["yAma"]);
    assert_has_krdanta(&[], &d("vA\\", Adadi), man, &["vAma"]);
    assert_has_krdanta(&[], &d("pa\\da~\\", Divadi), man, &["padma"]);
    assert_has_krdanta(&[], &d("yakza~", Curadi), man, &["yakzma"]);
    assert_has_krdanta(&[], &d("RI\\Y", Bhvadi), man, &["nema"]);
}

#[test]
fn unadi_1_138() {
    assert_has_krdanta(&[], &d("o~hA\\k", Juhotyadi), man, &["jihma"]);
}

#[test]
fn unadi_1_140() {
    assert_has_krdanta(&[], &d("grasu~\\", Bhvadi), man, &["grAma"]);
}

#[test]
fn unadi_1_141() {
    assert_has_krdanta(&[], &d("ava~", Bhvadi), man, &["Uma"]);
    assert_has_krdanta(&[], &d("zivu~", Divadi), man, &["syUma"]);
    assert_has_krdanta(&[], &d("zi\\Y", Svadi), man, &["sima"]);
    assert_has_krdanta(&[], &d("Su\\za~", Divadi), man, &["Suzma"]);
}

#[test]
fn unadi_1_144() {
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), mak, &["hima"]);
}

#[test]
fn unadi_1_145() {
    assert_has_krdanta(&[], &d("YiBI\\", Juhotyadi), mak, &["BIma", "BIzma"]);
}

#[test]
fn unadi_1_146() {
    assert_has_krdanta(&[], &d("Gf\\", Bhvadi), mak, &["Garma"]);
}

#[test]
fn unadi_1_147() {
    assert_has_krdanta(&[], &d("grasu~\\", Bhvadi), mak, &["grIzma"]);
}

#[test]
fn unadi_1_150() {
    assert_has_krdanta(&[], &d("i\\R", Adadi), van, &["eva"]);
    assert_has_krdanta(&[], &d("SIN", Adadi), van, &["Seva"]);
}

#[test]
fn unadi_1_153() {
    assert_has_krdanta(&[], &d("kF", Tudadi), va, &["karva"]);
    assert_has_krdanta(&[], &d("gF", Tudadi), va, &["garva"]);
    assert_has_krdanta(&[], &d("SF", Kryadi), va, &["Sarva"]);
    assert_has_krdanta(&[], &d("dF", Kryadi), va, &["darva"]);
}

#[test]
fn unadi_1_155() {
    assert_has_krdanta(&[], &d("zapa~", Bhvadi), kanin, &["saptan"]);
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), kanin, &["azwan"]);
}

#[test]
fn unadi_1_156() {
    assert_has_krdanta(&["naY"], &d("o~hA\\k", Juhotyadi), kanin, &["ahan"]);
}

#[test]
fn unadi_2_1() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), eRu, &["kareRu"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), eRu, &["hareRu"]);
}

#[test]
fn unadi_2_2() {
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), kTan, &["haTa"]);
    assert_has_krdanta(&[], &d("kuza~", Kryadi), kTan, &["kuzWa"]);
    assert_has_krdanta(&[], &d("RI\\Y", Tanadi), kTan, &["nITa"]);
    assert_has_krdanta(&[], &d("kASf~", Bhvadi), kTan, &["kAzWa"]);
}

#[test]
fn unadi_2_3() {
    assert_has_krdanta(&["ava"], &d("Bf\\Y", Bhvadi), kTan, &["avaBfTa"]);
}

#[ignore]
#[test]
fn unadi_2_5() {
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), sTan, &["soTa"]);
}

#[test]
fn unadi_2_6() {
    assert_has_krdanta(&[], &d("jF", Kryadi), UTan, &["jarUTa"]);
    assert_has_krdanta(&[], &d("vfY", Kryadi), UTan, &["varUTa"]);
}

#[test]
fn unadi_2_8() {
    assert_has_krdanta(&["nir"], &d("f\\", Bhvadi), Tak, &["nirfTa"]);
}

#[test]
fn unadi_2_10() {
    assert_has_krdanta(&["ud"], &d("gE\\", Bhvadi), Tak, &["udgITa"]);
}

#[test]
fn unadi_2_11() {
    assert_has_krdanta(&["sam"], &d("i\\R", Adadi), Tak, &["samiTa"]);
}

#[test]
fn unadi_2_15() {
    assert_has_krdanta(&["vi"], &d("kusa~", Divadi), rak, &["vikusra"]);
}

#[test]
fn unadi_2_16() {
    assert_has_krdanta(&[], &d("ama~", Divadi), rak, &["Amra"]);
    assert_has_krdanta(&[], &d("tamu~", Divadi), rak, &["tAmra"]);
}

#[test]
fn unadi_2_17() {
    assert_has_krdanta(&[], &d("Ridi~", Bhvadi), rak, &["nidrA"]);
}

#[test]
fn unadi_2_18() {
    assert_has_krdanta(&[], &d("arda~", Bhvadi), rak, &["Ardra"]);
}

#[test]
fn unadi_2_19() {
    assert_has_krdanta(&[], &d("Suca~", Bhvadi), rak, &["SUdra"]);
}

#[test]
fn unadi_2_20() {
    assert_has_krdanta(&["dur"], &d("i\\R", Adadi), rak, &["dUra"]);
}

#[test]
fn unadi_2_21() {
    assert_has_krdanta(&[], &d("kftI~", Tudadi), rak, &["kfcCra", "krUra"]);
}

#[test]
fn unadi_2_22() {
    assert_has_krdanta(&[], &nic(&d("rudi~r", Adadi)), rak, &["rudra"]);
}

#[test]
fn unadi_2_25() {
    assert_has_krdanta(&[], &d("zu\\Y", Adadi), kran, &["sura"]);
    assert_has_krdanta(&[], &d("zU", Adadi), kran, &["sUra"]);
    assert_has_krdanta(&[], &d("quDA\\Y", Juhotyadi), kran, &["DIra"]);
    assert_has_krdanta(&[], &d("gfDu~", Divadi), kran, &["gfDra"]);
}

#[test]
fn unadi_2_28() {
    assert_has_krdanta(&[], &d("vfDu~\\", Bhvadi), ran, &["varDra"]);
    assert_has_krdanta(&[], &d("quva\\pa~^", Bhvadi), ran, &["vapra"]);
}

#[test]
fn unadi_2_30() {
    assert_has_krdanta(&["sam"], &d("kasa~\\", Adadi), ukan, &["saNkasuka"]);
}

#[test]
fn unadi_2_32() {
    assert_has_krdanta(&[], &d("YiBI\\", Juhotyadi), krukan, &["BIruka"]);
}

#[test]
fn unadi_2_35() {
    assert_has_krdanta(&[], &d("o~hA\\k", Juhotyadi), kvun, &["jahaka"]);
}

#[test]
fn unadi_2_36() {
    assert_has_krdanta(&[], &d("DmA\\", Bhvadi), kvun, &["Damaka"]);
}

#[test]
fn unadi_2_37() {
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), kvun, &["vaDaka"]);
}

#[test]
fn unadi_2_41() {
    assert_has_krdanta(&[], &d("o~vrascU~", Tudadi), kikan, &["vfScika"]);
    assert_has_krdanta(&[], &d("kf\\za~", Bhvadi), kikan, &["kfzika"]);
}

#[test]
fn unadi_2_43() {
    assert_has_krdanta(&[], &d("muza~", Bhvadi), kikan, &["mUzika"]);
}

#[test]
fn unadi_2_44() {
    assert_has_krdanta(&[], &d("syamu~", Bhvadi), kikan, &["sImika"]);
}

#[test]
fn unadi_2_45() {
    assert_has_krdanta(&[], &d("qukrI\\Y", Kryadi), ikan, &["krayika"]);
}

#[test]
fn unadi_2_47() {
    assert_has_krdanta(&[], &d("SyE\\N", Bhvadi), inac, &["Syena"]);
    assert_has_krdanta(&[], &d("styE\\", Bhvadi), inac, &["styena"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), inac, &["hariRa"]);
    assert_has_krdanta(&[], &d("ava~", Bhvadi), inac, &["avina"]);
}

#[test]
fn unadi_2_48() {
    assert_has_krdanta(&[], &d("vfjI~", Rudhadi), inac, &["vfjina"]);
}

#[test]
fn unadi_2_51() {
    assert_has_krdanta(&[], &d("dru\\", Bhvadi), inan, &["draviRa"]);
    assert_has_krdanta(&[], &d("dakza~\\", Bhvadi), inan, &["dakziRa"]);
}

#[test]
fn unadi_2_52() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), inan, &["iriRa"]);
}

#[test]
fn unadi_2_53() {
    assert_has_krdanta(&[], &d("wuvepf~\\", Bhvadi), inan, &["vipina"]);
    assert_has_krdanta(&[], &d("tuhi~r", Bhvadi), inan, &["tuhina"]);
}

#[test]
fn unadi_2_56() {
    assert_has_krdanta(&[], &d("ru\\ha~", Bhvadi), inan, &["rohiRa"]);
}

#[test]
fn unadi_2_58() {
    assert_has_krdanta(&[], &d("A\\px~", Svadi), kvip, &["ap"]);
}

#[test]
fn unadi_2_61() {
    assert_has_krdanta(&[], &d("hu\\", Juhotyadi), kvip, &["juhU"]);
}

#[test]
fn unadi_2_62() {
    assert_has_krdanta(&[], &d("sru\\", Bhvadi), ka, &["sruva"]);
}

#[test]
fn unadi_2_63() {
    assert_has_krdanta(&[], &d("sru\\", Bhvadi), cik, &["sruc"]);
}

#[test]
fn unadi_2_64() {
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), cik, &["tvac"]);
}

#[test]
fn unadi_2_65() {
    assert_has_krdanta(&[], &d("glE\\", Bhvadi), qO, &["glO"]);
    assert_has_krdanta(&[], &d("Ru", Adadi), qO, &["nO"]);
}

#[test]
fn unadi_2_67() {
    assert_has_krdanta(&[], &d("rA\\", Adadi), qE, &["rE"]);
}

#[test]
fn unadi_2_68() {
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), qo, &["go"]);
    assert_has_krdanta(&[], &d("dyuta~\\", Bhvadi), qo, &["dyo"]);
}

#[test]
fn unadi_2_69() {
    assert_has_krdanta(&[], &d("Bramu~", Bhvadi), qU, &["BrU"]);
    assert_has_krdanta(&["agre"], &d("ga\\mx~", Bhvadi), qU, &["agregU"]);
}

#[test]
fn unadi_2_70() {
    let dos = krdanta(&[], &d("damu~", Divadi), qosi);
    assert_has_sup_1s(&dos, Pum, &["doH"]);
    assert_has_sup_1d(&dos, Pum, &["dozO"]);
}

#[test]
fn unadi_2_71() {
    assert_has_krdanta(&[], &d("paRa~\\", Bhvadi), iji, &["vaRij"]);
}

#[test]
fn unadi_2_72() {
    assert_has_krdanta(&[], &d("vaSa~", Adadi), iji, &["uSij"]);
}

#[test]
fn unadi_2_73() {
    assert_has_krdanta(&[], &d("Bf\\Y", Bhvadi), iji, &["BUrij"]);
}

#[test]
fn unadi_2_74() {
    assert_has_krdanta(&[], &d("jasu~", Divadi), urin, &["jasuri"]);
    assert_has_krdanta(&[], &d("zaha~\\", Bhvadi), urin, &["sahuri"]);
}

#[test]
fn unadi_2_75() {
    assert_has_krdanta(&[], &d("zu\\", Bhvadi), yuc, &["savana"]);
    assert_has_krdanta(&[], &d("yu", Adadi), yuc, &["yavana"]);
    assert_has_krdanta(&[], &d("ru", Adadi), yuc, &["ravaRa"]);
    assert_has_krdanta(&[], &d("vfY", Kryadi), yuc, &["varaRa"]);
}

#[test]
fn unadi_2_77() {
    assert_has_krdanta(&[], &d("undI~", Rudhadi), yuc, &["odana"]);
}

#[test]
fn unadi_2_78() {
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), yuc, &["gagana"]);
}

#[test]
fn unadi_2_79() {
    // TODO: linga?
    assert_has_krdanta(&[], &d("syandU~\\", Bhvadi), yuc, &["syandana"]);
    assert_has_krdanta(&[], &d("ruca~\\", Bhvadi), yuc, &["rocana"]);
}

#[test]
fn unadi_2_80() {
    assert_has_krdanta(&[], &d("ra\\nja~^", Bhvadi), kyun, &["rajana"]);
}

#[test]
fn unadi_2_82() {
    assert_has_krdanta(&[], &d("kF", Tudadi), kyu, &["kiraRa"]);
    assert_has_krdanta(&[], &d("pF", Kryadi), kyu, &["puraRa"]);
    assert_has_krdanta(&[], &d("vfjI~\\", Adadi), kyu, &["vfjana"]);
    assert_has_krdanta(&[], &d("madi~\\", Bhvadi), kyu, &["mandana"]);
    assert_has_krdanta(&["ni"], &d("quDA\\Y", Juhotyadi), kyu, &["niDana"]);
}

#[test]
fn unadi_2_83() {
    assert_has_krdanta(&[], &d("YiDfzA~", Svadi), kyu, &["DizaRa"]);
}

#[test]
fn unadi_2_84() {
    assert_has_krdanta(&[], &d("pfzu~", Bhvadi), ati, &["pfzat"]);
    assert_has_krdanta(&[], &d("bfha~", Bhvadi), ati, &["bfhat"]);
    assert_has_krdanta(&[], &d("maha~", Bhvadi), ati, &["mahat"]);
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), ati, &["jagat"]);
}

#[test]
fn unadi_2_85() {
    assert_has_krdanta(&["sam"], &d("ci\\Y", Svadi), ati, &["saMScat"]);
    assert_has_krdanta(&[], &d("YitfzA~", Divadi), ati, &["tfzat"]);
    assert_has_krdanta(&["vi"], &d("ha\\na~", Adadi), ati, &["vehat"]);
}

#[test]
fn unadi_2_87() {
    assert_has_krdanta(&[], &d("fji~\\", Bhvadi), asAnac, &["fYjasAna"]);
    assert_has_krdanta(&[], &d("vfDu~\\", Bhvadi), asAnac, &["vfDasAna"]);
    assert_has_krdanta(&[], &d("madi~\\", Bhvadi), asAnac, &["mandasAna"]);
    assert_has_krdanta(&[], &d("zaha~\\", Bhvadi), asAnac, &["sahasAna"]);
}

#[test]
fn unadi_2_88() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), asAnac, &["arSasAna"]);
}

#[test]
fn unadi_2_89() {
    assert_has_krdanta(&["sam"], &d("zwu\\Y", Adadi), Anac, &["saMstavAna"]);
}

#[test]
fn unadi_2_90() {
    assert_has_krdanta(&[], &d("yu\\Da~\\", Divadi), Anac, &["yuDAna"]);
    assert_has_krdanta(&[], &d("buDa~", Bhvadi), Anac, &["buDAna"]);
    assert_has_krdanta(&[], &d("df\\Si~r", Bhvadi), Anac, &["dfSAna"]);
}

#[test]
fn unadi_2_91() {
    assert_has_krdanta(&[], &d("hurCA~", Bhvadi), Anac, &["juhurARa"]);
}

#[test]
fn unadi_2_92() {
    assert_has_krdanta(&[], &d("SvitA~\\", Bhvadi), Anac, &["SiSvidAna"]);
}

#[ignore]
#[test]
fn unadi_2_95() {
    assert_has_krdanta(&[], &d("patx~", Bhvadi), tfn, &["naptf"]);
    assert_has_krdanta(&[], &d("RI\\Y", Bhvadi), tfn, &["nezwf"]);
    assert_has_krdanta(&[], &d("tvi\\za~^", Bhvadi), tfn, &["tvazwf"]);
    assert_has_krdanta(&[], &d("hu\\", Bhvadi), tfn, &["hotf"]);
    assert_has_krdanta(&[], &d("pUY", Kryadi), tfn, &["potf"]);
    assert_has_krdanta(&[], &d("BrAjf~\\", Bhvadi), tfn, &["BrAtf"]);
    assert_has_krdanta(&[], &d("mA\\", Adadi), tfn, &["jAmAtf"]);
    assert_has_krdanta(&[], &d("mAna~", Curadi), tfn, &["mAtf"]);
    assert_has_krdanta(&[], &d("pA\\", Adadi), tfn, &["pitf"]);
    assert_has_krdanta(&[], &d("du\\ha~^", Adadi), tfn, &["duhitf"]);

    assert_has_krdanta(&[], &d("patx~", Bhvadi), tfc, &["naptf"]);
    assert_has_krdanta(&[], &d("RI\\Y", Bhvadi), tfc, &["nezwf"]);
    assert_has_krdanta(&[], &d("tvi\\za~^", Bhvadi), tfc, &["tvazwf"]);
    assert_has_krdanta(&[], &d("hu\\", Bhvadi), tfc, &["hotf"]);
    assert_has_krdanta(&[], &d("pUY", Kryadi), tfc, &["potf"]);
    assert_has_krdanta(&[], &d("BrAjf~\\", Bhvadi), tfc, &["BrAtf"]);
    assert_has_krdanta(&[], &d("mA\\", Adadi), tfc, &["jAmAtf"]);
    assert_has_krdanta(&[], &d("mAna~", Curadi), tfc, &["mAtf"]);
    assert_has_krdanta(&[], &d("pA\\", Adadi), tfc, &["pitf"]);
    assert_has_krdanta(&[], &d("du\\ha~^", Adadi), tfc, &["duhitf"]);
}

#[test]
fn unadi_2_96() {
    assert_has_krdanta(&["su"], &d("asa~", Adadi), fn_, &["svasf"]);
}

#[test]
fn unadi_2_97() {
    assert_has_krdanta(&[], &d("yatI~\\", Bhvadi), fn_, &["yAtf"]);
}

#[test]
fn unadi_2_99() {
    assert_has_krdanta(&[], &d("divu~", Divadi), f, &["devf"]);
}

#[test]
fn unadi_2_100() {
    let nr = krdanta(&[], &d("RI\\Y", Bhvadi), f);
    assert_has_sup_1s(&nr, Pum, &["nA"]);
    assert_has_sup_1d(&nr, Pum, &["narO"]);
    assert_has_sup_1p(&nr, Pum, &["naraH"]);
}

#[test]
fn unadi_2_104() {
    assert_has_krdanta(&[], &d("kf\\za~", Bhvadi), ani, &["carzaRi"]);
}

#[test]
fn unadi_2_105() {
    assert_has_krdanta(&[], &d("a\\da~", Adadi), ani, &["admani"]);
}

#[test]
fn unadi_2_106() {
    assert_has_krdanta(&[], &d("vftu~\\", Bhvadi), ani, &["vartani", "vartmani"]);
}

#[test]
fn unadi_2_107() {
    assert_has_krdanta(&[], &d("kzi\\pa~", Divadi), ani, &["kzipaRi"]);
}

#[test]
fn unadi_2_108() {
    assert_has_krdanta(&[], &d("arca~", Bhvadi), isi, &["arcis"]);
    assert_has_krdanta(&[], &d("I~Suci~^r", Divadi), isi, &["Socis"]);
    assert_has_krdanta(&[], &d("hu\\", Juhotyadi), isi, &["havis"]);
    assert_has_krdanta(&[], &d("sf\\px~", Bhvadi), isi, &["sarpis"]);
    assert_has_krdanta(&[], &d("Cada~", Curadi), isi, &["Cadis"]);
    assert_has_krdanta(&[], &d("Carda~", Curadi), isi, &["Cardis"]);
    // TODO: id-anta Cardi
}

#[test]
fn unadi_2_109() {
    assert_has_krdanta(&[], &d("bfhi~", Bhvadi), isi, &["barhis"]);
}

#[test]
fn unadi_2_110() {
    assert_has_krdanta(&[], &d("dyuta~\\", Bhvadi), isin, &["jyotis"]);
}

#[test]
fn unadi_2_111() {
    assert_has_krdanta(&["vasu"], &d("ruca~\\", Bhvadi), isin, &["vasurocis"]);
}

#[test]
fn unadi_2_112() {
    assert_has_krdanta(&[], &d("BU", Bhvadi), isin, &["Buvis"]);
}

#[test]
fn unadi_2_114() {
    assert_has_krdanta(&[], &d("pA\\", Bhvadi), isin, &["pATis"]);
}

#[test]
fn unadi_2_115() {
    assert_has_krdanta(&[], &d("janI~\\", Divadi), usi, &["janus"]);
}

#[test]
fn unadi_2_116() {
    assert_has_krdanta(&[], &d("ma\\na~\\", Divadi), usi, &["maDus"]);
}

#[test]
fn unadi_2_117() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), usi, &["arus"]);
    assert_has_krdanta(&[], &d("pF", Kryadi), usi, &["parus"]);
    assert_has_krdanta(&[], &d("quva\\pa~^", Bhvadi), usi, &["vapus"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), usi, &["yajus"]);
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), usi, &["tanus"]);
    assert_has_krdanta(&[], &d("Dana~", Juhotyadi), usi, &["Danus"]);
    assert_has_krdanta(&[], &d("ta\\pa~", Bhvadi), usi, &["tapus"]);
    // TODO: tanuzI, tanUMzi
}

#[test]
fn unadi_2_118() {
    let ayus = krdanta(&[], &d("i\\R", Adadi), usi);
    assert_has_sup_1s(&ayus, Napumsaka, &["AyuH"]);
    assert_has_sup_1d(&ayus, Napumsaka, &["AyuzI"]);
}

#[test]
fn unadi_2_119() {
    assert_has_krdanta(&[], &d("ca\\kzi~\\N", Adadi), usi, &["cakzus"]);
}

#[test]
fn unadi_2_120() {
    assert_has_krdanta(&[], &d("mu\\ha~", Divadi), usi, &["muhus"]);
}

#[test]
fn unadi_2_121() {
    let caksh = d("ca\\kzi~\\N", Adadi);
    assert_has_krdanta(&["AN"], &caksh, usi, &["Acakzus"]);
    assert_has_krdanta(&["pari"], &caksh, usi, &["paricakzus"]);
}

#[test]
fn unadi_2_122() {
    assert_has_krdanta(&[], &d("kF", Tudadi), zvarac, &["karvara"]);
    assert_has_krdanta(&[], &d("gF", Kryadi), zvarac, &["garvara"]);
    let sharvara = krdanta(&[], &d("SF", Kryadi), zvarac);
    assert_has_sup_1s(&sharvara, Stri, &["SarvarI"]);
    assert_has_krdanta(&[], &d("vfY", Svadi), zvarac, &["varvara"]);
    assert_has_krdanta(&[], &d("cate~^", Bhvadi), zvarac, &["catvara"]);
}

#[test]
fn unadi_3_3() {
    assert_has_krdanta(&[], &d("sPAyI~\\", Bhvadi), nak, &["Pena"]);
    assert_has_krdanta(&[], &d("mI\\N", Divadi), nak, &["mIna"]);
}

#[test]
fn unadi_3_4() {
    assert_has_krdanta(&[], &d("kf\\za~", Tudadi), nak, &["kfzRa"]);
}

#[test]
fn unadi_3_5() {
    assert_has_krdanta(&[], &d("ba\\nDa~", Kryadi), nak, &["braDna", "buDna"]);
}

#[test]
fn unadi_3_6() {
    assert_has_krdanta(&[], &d("quDA\\Y", Juhotyadi), na, &["DAna"]);
    assert_has_krdanta(&[], &d("pF", Kryadi), na, &["parRa"]);
    assert_has_krdanta(&[], &d("va\\sa~", Bhvadi), na, &["vasna"]);
    assert_has_krdanta(&[], &d("aja~", Bhvadi), na, &["vena"]);
    assert_has_krdanta(&[], &d("ata~", Bhvadi), na, &["atna"]);
    assert_has_krdanta(&[], &d("Sru\\", Bhvadi), na, &["SroRa"]);
}

#[ignore]
#[test]
fn unadi_3_7() {
    assert_has_krdanta(&[], &d("lakza~", Curadi), na, &["lakzaRa"]);
}

#[test]
fn unadi_3_8() {
    assert_has_krdanta(&[], &d("vana~", Bhvadi), na, &["vennA"]);
}

#[test]
fn unadi_3_11() {
    assert_has_krdanta(&[], &d("De\\w", Bhvadi), na, &["Dena"]);
}

#[test]
fn unadi_3_13() {
    assert_has_krdanta(&[], &d("zu\\Y", Svadi), na, &["sUnA"]);
}

#[test]
fn unadi_3_14() {
    assert_has_krdanta(&[], &d("ra\\ma~\\", Bhvadi), na, &["ratna"]);
}

#[test]
fn unadi_3_16() {
    assert_has_krdanta(&[], &d("gE\\", Bhvadi), izRuc, &["gezRu"]);
    assert_has_krdanta(&[], &d("qudA\\Y", Juhotyadi), izRuc, &["dezRu"]);
}

#[test]
fn unadi_3_18() {
    assert_has_krdanta(&[], &d("tija~", Curadi), ksna, &["tIkzRa"]);
}

#[test]
fn unadi_3_19() {
    assert_has_krdanta(&[], &d("Slizu~", Bhvadi), ksna, &["SlakzRa"]);
}

#[test]
fn unadi_3_22() {
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), ayu, &["sarayu"]);
}

#[test]
fn unadi_3_24() {
    assert_has_krdanta(&[], &d("cyu\\N", Bhvadi), pa, &["cyupa"]);
}

#[test]
fn unadi_3_25() {
    assert_has_krdanta(&[], &d("zwu\\Y", Adadi), pa, &["stUpa"]);
}

#[ignore]
#[test]
fn unadi_3_29() {
    assert_has_krdanta(&[], &d("stana", Curadi), itnuc, &["stanayitnu"]);
    assert_has_krdanta(&[], &nic(&d("hfza~", Divadi)), itnuc, &["harzayitnu"]);
    // TODO: popayitnu?
    assert_has_krdanta(&[], &d("gada", Curadi), itnuc, &["gadayitnu"]);
    assert_has_krdanta(&[], &d("mada~", Curadi), itnuc, &["madayitnu"]);
}

#[test]
fn unadi_3_30() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), ktnu, &["kftnu"]);
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), ktnu, &["hatnu"]);
}

#[test]
fn unadi_3_31() {
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), ktnu, &["jigatnu"]);
}

#[test]
fn unadi_3_32() {
    assert_has_krdanta(&[], &d("qudA\\Y", Juhotyadi), nu, &["dAnu"]);
    assert_has_krdanta(&[], &d("BA\\", Adadi), nu, &["BAnu"]);
}

#[test]
fn unadi_3_33() {
    assert_has_krdanta(&[], &d("va\\ca~", Adadi), nu, &["vagnu"]);
}

#[test]
fn unadi_3_34() {
    assert_has_krdanta(&[], &d("De\\w", Bhvadi), nu, &["Denu"]);
}

#[test]
fn unadi_3_35() {
    assert_has_krdanta(&[], &d("zUN", Adadi), nu, &["sUnu"]);
}

#[test]
fn unadi_3_36() {
    assert_has_krdanta(&[], &d("o~hA\\k", Juhotyadi), nu, &["jahnu"]);
}

#[test]
fn unadi_3_37() {
    assert_has_krdanta(&[], &d("zWA\\", Bhvadi), Ru, &["sTARu"]);
}

#[test]
fn unadi_3_38() {
    assert_has_krdanta(&[], &d("aja~", Bhvadi), Ru, &["veRu"]);
    assert_has_krdanta(&[], &d("vfN", Kryadi), Ru, &["varRu"]);
    assert_has_krdanta(&[], &d("rI\\N", Divadi), Ru, &["reRu"]);
}

#[test]
fn unadi_3_39() {
    assert_has_krdanta(&[], &d("vi\\zx~^", Juhotyadi), Ru, &["vizRu"]);
}

#[test]
fn unadi_3_43() {
    assert_has_krdanta(&[], &d("i\\R", Adadi), kan, &["eka"]);
    assert_has_krdanta(&[], &d("YiBI\\", Juhotyadi), kan, &["Beka"]);
    assert_has_krdanta(&[], &d("kE\\", Tanadi), kan, &["kAka"]);
    assert_has_krdanta(&[], &d("pA\\", Adadi), kan, &["pAka"]);
    assert_has_krdanta(&[], &d("Sala~", Bhvadi), kan, &["Salka"]);
    assert_has_krdanta(&[], &d("ata~", Bhvadi), kan, &["atka"]);
    assert_has_krdanta(&[], &d("marca~", Bhvadi), kan, &["marka", "markka"]);
}

#[test]
fn unadi_3_44() {
    assert_has_krdanta(&["ni"], &d("o~hA\\k", Juhotyadi), kan, &["nihAka"]);
}

#[test]
fn unadi_3_45() {
    assert_has_krdanta(&["ni"], &d("za\\dx~", Bhvadi), kan, &["nizka"]);
}

#[test]
fn unadi_3_46() {
    assert_has_krdanta(&[], &d("syamu~", Bhvadi), kan, &["syamIka", "syamika"]);
}

#[test]
fn unadi_3_47() {
    assert_has_krdanta(&[], &d("aja~", Bhvadi), kan, &["vIka"]);
    assert_has_krdanta(&[], &d("yu", Adadi), kan, &["yUka"]);
    assert_has_krdanta(&[], &d("Du\\Y", Svadi), kan, &["DUka"]);
    assert_has_krdanta(&[], &d("RI\\Y", Bhvadi), kan, &["nIka"]);
}

#[test]
fn unadi_3_48() {
    assert_has_krdanta(&[], &d("hrI\\", Juhotyadi), kan, &["hrIka", "hlIka"]);
}

#[test]
fn unadi_3_49() {
    let shak = d("Sa\\kx~", Svadi);
    assert_has_krdanta(&[], &shak, una, &["Sakuna"]);
    assert_has_krdanta(&[], &shak, unta, &["Sakunta"]);
    assert_has_krdanta(&[], &shak, unti, &["Sakunti"]);
    assert_has_krdanta(&[], &shak, uni, &["Sakuni"]);
}

#[test]
fn unadi_3_50() {
    assert_has_krdanta(&[], &d("BU", Bhvadi), Jic, &["Bavanti"]);
}

#[test]
fn unadi_3_51() {
    assert_has_krdanta(&[], &d("kzi\\pa~^", Tudadi), kanyuc, &["kzipaRyu"]);
    // ashtadhyayi.com has Bavanyu, but other sources have Buvanyu
    assert_has_krdanta(&[], &d("BU", Bhvadi), kanyuc, &["Buvanyu"]);
}

#[test]
fn unadi_3_52() {
    assert_has_krdanta(&[], &d("Rada~", Bhvadi), anuN, &["nadanu"]);
}

#[test]
fn unadi_3_53() {
    assert_has_krdanta(&[], &d("kF", Tudadi), unan, &["karuRa"]);
    assert_has_krdanta(&[], &d("vfY", Svadi), unan, &["varuRa"]);
    assert_has_krdanta(&[], &nic(&d("dF", Kryadi)), unan, &["dAruRa"]);
}

#[test]
fn unadi_3_54() {
    assert_has_krdanta(&[], &d("tF", Bhvadi), unan, &["taruRa", "taluna"]);
}

#[test]
fn unadi_3_55() {
    assert_has_krdanta(&[], &d("kzu\\Da~", Divadi), unan, &["kzuDuna"]);
    assert_has_krdanta(&[], &d("piSa~", Tudadi), unan, &["piSuna"]);
    assert_has_krdanta(&[], &d("miTf~^", Bhvadi), unan, &["miTuna"]);
}

#[ignore]
#[test]
fn unadi_3_56() {
    assert_has_krdanta(&[], &d("Pala~", Bhvadi), unan, &["Palguna"]);
}

#[test]
fn unadi_3_57() {
    assert_has_krdanta(&[], &d("aSa~", Kryadi), unan, &["laSuna"]);
}

#[test]
fn unadi_3_58_and_unadi_3_59() {
    assert_has_krdanta(&[], &d("arja~", Curadi), unan, &["arjuna"]);
}

#[test]
fn unadi_3_60() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), unan, &["aruRa"]);
}

#[test]
fn unadi_3_62() {
    // TODO: why not varza and tarza?
    // assert_has_krdanta(&[], &d("vF", Kryadi), Krt::sa, &["varsa"]);
    // assert_has_krdanta(&[], &d("tF", Bhvadi), Krt::sa, &["tarsa"]);
    assert_has_krdanta(&[], &d("vada~", Bhvadi), sa, &["vatsa"]);
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), sa, &["haMsa"]);
    assert_has_krdanta(&[], &d("kamu~\\", Bhvadi), sa, &["kaMsa"]);
    assert_has_krdanta(&[], &d("kaza~", Bhvadi), sa, &["kakza"]);
}

#[test]
fn unadi_3_63() {
    assert_has_krdanta(&[], &d("pluza~", Divadi), sa, &["plakza"]);
}

#[test]
fn unadi_3_64() {
    assert_has_krdanta(&[], &d("ma\\na~\\", Divadi), sa, &["mAMsa"]);
}

#[test]
fn unadi_3_65() {
    assert_has_krdanta(&[], &d("aSa~", Kryadi), sa, &["akza"]);
}

#[test]
fn unadi_3_66() {
    // TODO: strI?
    assert_has_krdanta(&[], &d("zRu", Adadi), sa, &["snuza"]);
    assert_has_krdanta(&[], &d("o~vrascU~", Tudadi), sa, &["vfkza"]);
    assert_has_krdanta(&[], &d("kftI~", Tudadi), sa, &["kftsa"]);
    assert_has_krdanta(&[], &d("fzI~", Tudadi), sa, &["fkza"]);
}

#[test]
fn unadi_3_67() {
    assert_has_krdanta(&[], &d("fzI~", Tudadi), sa, &["fkza"]);
}

#[ignore]
#[test]
fn unadi_3_68() {
    assert_has_krdanta(&[], &d("undI~", Rudhadi), sa, &["utsa"]);
    assert_has_krdanta(&[], &d("guDa~", Divadi), sa, &["gutsa"]);
    assert_has_krdanta(&[], &d("kuza~", Kryadi), sa, &["ukza"]);
}

#[test]
fn unadi_3_69() {
    assert_has_krdanta(&[], &d("gfDu~", Divadi), sa, &["gftsa"]);
    assert_has_krdanta(&[], &d("paRa~\\", Bhvadi), sa, &["pakza"]);
}

#[test]
fn unadi_3_70() {
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), sara, &["akzara"]);
}

#[test]
fn unadi_3_71() {
    assert_has_krdanta(&[], &d("va\\sa~", Bhvadi), sara, &["vatsara"]);
}

#[test]
fn unadi_3_72() {
    assert_has_krdanta(&["sam"], &d("va\\sa~", Bhvadi), sara, &["saMvatsara"]);
}

#[test]
fn unadi_3_73() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), sara, &["kfsara"]);
    assert_has_krdanta(&[], &d("DUY", Svadi), sara, &["DUsara"]);
    assert_has_krdanta(&[], &d("madI~", Bhvadi), sara, &["matsara"]);
}

#[test]
fn unadi_3_74() {
    assert_has_krdanta(&[], &d("patx~", Bhvadi), sara, &["patsala"]);
}

#[test]
fn unadi_3_75() {
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), ksaran, &["tasara"]);
    assert_has_krdanta(&[], &d("fzI~", Tudadi), ksaran, &["fkzara"]);
}

#[test]
fn unadi_3_77() {
    assert_has_krdanta(&[], &d("kaWa~", Bhvadi), kAku, &["kaWAku"]);
    assert_has_krdanta(&[], &d("kuza~", Kryadi), kAku, &["kuzAku"]);
}

#[test]
fn unadi_3_78() {
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), kAku, &["sfdAku"]);
}

#[test]
fn unadi_3_79() {
    assert_has_krdanta(&[], &d("vftu~\\", Bhvadi), kAku, &["vArtAku"]);
}

#[test]
fn unadi_3_80() {
    assert_has_krdanta(&[], &d("parda~\\", Bhvadi), kAku, &["pfdAku"]);
}

#[test]
fn unadi_3_81() {
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), anyuc, &["saraRyu"]);
    assert_has_krdanta(&[], &d("yu", Adadi), AgUc, &["yavAgU"]);
    assert_has_krdanta(&[], &d("va\\ca~", Adadi), aknuc, &["vacaknu"]);
}

#[test]
fn unadi_3_82() {
    assert_has_krdanta(&[], &d("SIN", Adadi), Anaka, &["SayAnaka"]);
    assert_has_krdanta(&[], &d("YiBI\\", Juhotyadi), Anaka, &["BayAnaka"]);
}

#[test]
fn unadi_3_83() {
    assert_has_krdanta(&[], &d("lUY", Kryadi), ARaka, &["lavARaka"]);
    assert_has_krdanta(&[], &d("DUY", Svadi), ARaka, &["DavARaka"]);
    assert_has_krdanta(&[], &d("SiGi~", Bhvadi), ARaka, &["SiNGARaka", "SiNGARa"]);
    assert_has_krdanta(&[], &d("quDA\\Y", Juhotyadi), ARaka, &["DARaka"]);
}

#[test]
fn unadi_3_86() {
    assert_has_krdanta(&[], &d("hase~", Bhvadi), tan, &["hasta"]);
    // "mfta" by 3.88
    assert_has_krdanta(&[], &d("mf\\N", Tanadi), tan, &["marta", "mfta"]);
    assert_has_krdanta(&[], &d("gF", Tudadi), tan, &["garta"]);
    assert_has_krdanta(&[], &d("i\\R", Adadi), tan, &["eta"]);
    assert_has_krdanta(&[], &d("vA\\", Adadi), tan, &["vAta"]);
    assert_has_krdanta(&[], &d("ama~", Bhvadi), tan, &["anta"]);
    assert_has_krdanta(&[], &d("damu~", Divadi), tan, &["danta"]);
    assert_has_krdanta(&[], &d("lUY", Kryadi), tan, &["lota"]);
    assert_has_krdanta(&[], &d("pUY", Kryadi), tan, &["pota"]);
    // TODO: enable this after fixing cchvoh
    // assert_has_krdanta(&[], &d("DurvI~", Bhvadi), Krt::tan, &["DUrta"]);
}

#[test]
fn unadi_3_88() {
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), tan, &["tata"]);
    // "marta" by 3.86
    assert_has_krdanta(&[], &d("mf\\N", Tudadi), tan, &["mfta", "marta"]);
}

#[test]
fn unadi_3_89() {
    assert_has_krdanta(&[], &d("anjU~", Rudhadi), kta, &["akta"]);
    assert_has_krdanta(&[], &d("Gf\\", Bhvadi), kta, &["Gfta"]);
    assert_has_krdanta(&[], &d("zi\\Y", Svadi), kta, &["sita"]);
}

#[test]
fn unadi_3_90() {
    assert_has_krdanta(&[], &d("du\\", Bhvadi), kta, &["dUta"]);
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), kta, &["tAta"]);
}

#[test]
fn unadi_3_95() {
    assert_has_krdanta(&[], &d("piSa~", Tudadi), itan, &["piSita"]);
}

#[test]
fn unadi_3_96() {
    assert_has_krdanta(&[], &d("Sru\\", Bhvadi), Ayya, &["SravAyya"]);
    assert_has_krdanta(&[], &d("dakza~\\", Bhvadi), Ayya, &["dakzAyya"]);
    assert_has_krdanta(&[], &d("spfha", Curadi), Ayya, &["spfhayAyya"]);
    assert_has_krdanta(&[], &d("gfha", Curadi), Ayya, &["gfhayAyya"]);
}

#[test]
fn unadi_3_100() {
    assert_has_krdanta(&[], &d("rAjf~^", Bhvadi), anya, &["rAjanya"]);
}

#[test]
fn unadi_3_101() {
    assert_has_krdanta(&[], &d("SF", Kryadi), anya, &["SaraRya"]);
    assert_has_krdanta(&[], &d("ra\\ma~\\", Bhvadi), anya, &["ramaRya"]);
}

#[test]
fn unadi_3_102() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), anya, &["araRya"]);
}

#[test]
fn unadi_3_103() {
    assert_has_krdanta(&[], &d("pfzu~", Bhvadi), anya, &["parjanya"]);
}

#[test]
fn unadi_3_104() {
    assert_has_krdanta(&[], &d("vada~", Bhvadi), Anya, &["vadAnya"]);
}

#[test]
fn unadi_3_107() {
    assert_has_krdanta(&[], &d("vfY", Kryadi), atran, &["varatra"]);
}

#[test]
fn unadi_3_108() {
    assert_has_krdanta(&["su"], &d("vida~", Adadi), katra, &["suvidatra"]);
}

#[test]
fn unadi_3_109() {
    assert_has_krdanta(&[], &d("kftI~", Tudadi), katra, &["kfntatra"]);
}

#[test]
fn unadi_3_114() {
    assert_has_krdanta(&[], &d("Bf\\Y", Bhvadi), aTa, &["BaraTa"]);
}

#[test]
fn unadi_3_115() {
    assert_has_krdanta(&[], &d("rudi~r", Adadi), aTa, &["rudaTa"]);
    assert_has_krdanta(&[], &d("vida~", Adadi), aTa, &["vidaTa"]);
}

#[test]
fn unadi_3_116() {
    let vas = d("va\\sa~", Bhvadi);
    assert_has_krdanta(&["AN"], &vas, aTa, &["AvasaTa"]);
    assert_has_krdanta(&["sam"], &vas, aTa, &["saMvasaTa"]);
}

#[test]
fn unadi_3_119() {
    assert_has_krdanta(&[], &d("va\\ha~^", Bhvadi), asac, &["vAhasa"]);
    assert_has_krdanta(&[], &d("yu", Adadi), asac, &["yAvasa"]);
}

#[test]
fn unadi_3_120() {
    assert_has_krdanta(&[], &d("vaya~\\", Bhvadi), asac, &["vAyasa"]);
}

#[test]
fn unadi_3_121() {
    assert_has_krdanta(&[], &d("divu~", Divadi), asac, &["divasa"]);
}

#[test]
fn unadi_3_123() {
    assert_has_krdanta(&[], &d("fzI~", Tudadi), aBac, &["fzaBa"]);
    assert_has_krdanta(&[], &d("vfzu~", Bhvadi), aBac, &["vfzaBa"]);
}

#[test]
fn unadi_3_124() {
    assert_has_krdanta(&[], &d("ruza~", Bhvadi), aBac, &["luzaBa"]);
}

#[test]
fn unadi_3_125() {
    assert_has_krdanta(&[], &d("rAsf~\\", Bhvadi), aBac, &["rAsaBa"]);
    assert_has_krdanta(&[], &d("valla~\\", Bhvadi), aBac, &["vallaBa"]);
}

#[test]
fn unadi_3_126() {
    assert_has_krdanta(&[], &d("jF", Kryadi), Jac, &["jaranta"]);
    assert_has_krdanta(&[], &d("vi\\Sa~", Tudadi), Jac, &["veSanta"]);
}

#[test]
fn unadi_3_127() {
    assert_has_krdanta(&[], &d("ru\\ha~", Bhvadi), Jac, &["rohanta"]);
    assert_has_krdanta(&[], &d("wunadi~", Bhvadi), Jac, &["nandanta"]);
    assert_has_krdanta(&[], &d("jIva~", Tudadi), Jac, &["jIvanta"]);
    // TODO: zit
}

#[test]
fn unadi_3_128() {
    assert_has_krdanta(&[], &d("tF", Bhvadi), Jac, &["taranta"]);
    assert_has_krdanta(&[], &d("BU", Bhvadi), Jac, &["Bavanta"]);
    assert_has_krdanta(&[], &d("va\\ha~^", Bhvadi), Jac, &["vahanta"]);
    assert_has_krdanta(&[], &d("va\\sa~", Bhvadi), Jac, &["vasanta"]);
    assert_has_krdanta(&[], &d("BAsf~\\", Bhvadi), Jac, &["BAsanta"]);
    assert_has_krdanta(&[], &d("sA\\Da~", Svadi), Jac, &["sADanta"]);
    assert_has_krdanta(&[], &nic(&d("gaqi~", Bhvadi)), Jac, &["gaRqayanta"]);
    assert_has_krdanta(&[], &nic(&d("maqi~\\", Tudadi)), Jac, &["maRqayanta"]);
    assert_has_krdanta(&[], &d("ji\\", Bhvadi), Jac, &["jayanta"]);
    // nandayanta?
}

#[test]
fn unadi_3_129() {
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), Jac, &["hemanta"]);
}

#[test]
fn unadi_3_130() {
    assert_has_krdanta(&[], &d("Badi~\\", Bhvadi), Jac, &["Badanta"]);
}

#[test]
fn unadi_3_133() {
    assert_has_krdanta(&[], &d("ku\\N", Bhvadi), kraran, &["kurara"]);
}

#[test]
fn unadi_3_135() {
    assert_has_krdanta(&[], &d("gaqa~", Bhvadi), Aran, &["kaqAra"]);
}

#[ignore]
#[test]
fn unadi_3_140() {
    assert_has_krdanta(&[], &d("dI\\N", Divadi), Aran, &["dInAra"]);
}

#[ignore]
#[test]
fn unadi_3_141() {
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), apa, &["sarzapa"]);
}

#[test]
fn unadi_3_143() {
    assert_has_krdanta(&[], &d("kvaRa~", Bhvadi), kapan, &["kuRapa"]);
}

#[test]
fn unadi_3_144() {
    assert_has_krdanta(&[], &d("kvaRa~", Bhvadi), kapa, &["kuRapa"]);
}

#[test]
fn unadi_3_146() {
    assert_has_krdanta(&[], &d("vftu~\\", Bhvadi), tikan, &["varttikA", "vartikA"]);
}

#[ignore]
#[test]
fn unadi_3_147() {
    assert_has_krdanta(&[], &d("kftI~", Tudadi), tikan, &["kfttikA"]);
    assert_has_krdanta(&[], &d("Bi\\di~^r", Rudhadi), tikan, &["BittikA"]);
}

#[test]
fn unadi_3_148() {
    assert_has_krdanta(&[], &d("iza~", Divadi), takan, &["izwakA"]);
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), takan, &["azwakA"]);
}

#[test]
fn unadi_3_149() {
    let i_ = d("i\\R", Adadi);
    assert_has_krdanta(&[], &i_, taSan, &["etaSa"]);
    assert_has_krdanta(&[], &i_, taSasun, &["etaSas"]);
}

#[test]
fn unadi_3_150() {
    assert_has_krdanta(&[], &d("vI\\", Adadi), tanan, &["vetana"]);
    assert_has_krdanta(&[], &d("patx~", Bhvadi), tanan, &["pattana"]);
}

#[test]
fn unadi_3_151() {
    assert_has_krdanta(&[], &d("dF", Bhvadi), Ba, &["darBa"]);
    assert_has_krdanta(&[], &d("dala~", Bhvadi), Ba, &["dalBa"]);
}

#[test]
fn unadi_3_152() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), Ban, &["arBa"]);
    assert_has_krdanta(&[], &d("gF", Tudadi), Ban, &["garBa"]);
}

#[test]
fn unadi_3_153() {
    assert_has_krdanta(&[], &d("i\\R", Adadi), Ban, &["iBa"]);
}

#[test]
fn unadi_3_154() {
    assert_has_krdanta(&[], &d("asa~", Adadi), kTin, &["asTi"]);
    assert_has_krdanta(&[], &d("za\\nja~", Bhvadi), kTin, &["sakTi"]);
}

#[test]
fn unadi_3_155() {
    assert_has_krdanta(&[], &d("pluza~", Kryadi), ksi, &["plukzi"]);
    assert_has_krdanta(&[], &d("kuza~", Kryadi), ksi, &["kukzi"]);
    assert_has_krdanta(&[], &d("Su\\za~", Divadi), ksi, &["Sukzi"]);
}

#[test]
fn unadi_3_156() {
    assert_has_krdanta(&[], &d("aSU~", Svadi), ksi, &["akzi"]);
}

#[test]
fn unadi_3_157() {
    assert_has_krdanta(&[], &d("izu~", Tudadi), ksu, &["ikzu"]);
}

#[test]
fn unadi_3_158() {
    assert_has_krdanta(&[], &d("ava~", Bhvadi), I, &["avI"]);
    assert_has_krdanta(&[], &d("tF", Bhvadi), I, &["tarI"]);
    assert_has_krdanta(&[], &d("stFY", Kryadi), I, &["starI"]);
    assert_has_krdanta(&[], &d("tatri~", Curadi), I, &["tantrI"]);
}

#[test]
fn unadi_3_159() {
    assert_has_krdanta(&[], &d("yA\\", Adadi), I, &["yayI"]);
    assert_has_krdanta(&[], &d("pA\\", Adadi), I, &["papI"]);
}

#[test]
fn unadi_3_160() {
    assert_has_krdanta(&[], &d("lakza~", Curadi), I, &["lakzmI"]);
}

#[test]
fn unadi_4_2() {
    assert_has_krdanta(&[], &d("f\\", Juhotyadi), katnic, &["ratni"]);
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), yatuc, &["tanyatu"]);
    assert_has_krdanta(&[], &d("anjU~", Rudhadi), alic, &["aYjali"]);
    assert_has_krdanta(&[], &d("vana~", Bhvadi), izWuc, &["vanizWu"]);
    assert_has_krdanta(&[], &d("anjU~", Rudhadi), izWac, &["aYjizWa"]);
    assert_has_krdanta(&[], &nic(&d("f\\", Juhotyadi)), isan, &["arpisa"]);
    // TODO: why is this aniw?
    // assert_has_krdanta(&[], &d("madI~", Divadi), Krt::syan, &["matsya"]);
    assert_has_krdanta(&[], &d("ata~", Bhvadi), iTin, &["atiTi"]);
    assert_has_krdanta(&[], &d("anga", Curadi), uli, &["aNguli"]);
    assert_has_krdanta(&[], &d("ku\\", Adadi), asa, &["kavasa"]);
    assert_has_krdanta(&[], &d("yu", Adadi), Asa, &["yavAsa"]);
    assert_has_krdanta(&[], &d("kfSa~", Divadi), Anuk, &["kfSAnu"]);
}

#[test]
fn unadi_4_3() {
    assert_has_krdanta(&[], &d("SF", Kryadi), karan, &["SarkarA"]);
}

#[test]
fn unadi_4_4() {
    assert_has_krdanta(&[], &d("puza~", Bhvadi), karan, &["puzkara"]);
}

#[ignore]
#[test]
fn unadi_4_5() {
    assert_has_krdanta(&[], &d("puza~", Bhvadi), kala, &["puzkala"]);
}

#[test]
fn unadi_4_6() {
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), ini, &["gamin"]);
}

#[test]
fn unadi_4_7() {
    assert_has_krdanta(&["AN"], &d("ga\\mx~", Bhvadi), ini, &["AgAmin"]);
}

#[test]
fn unadi_4_8() {
    assert_has_krdanta(&[], &d("BU", Bhvadi), ini, &["BAvin"]);
}

#[test]
fn unadi_4_9() {
    assert_has_krdanta(&["pra"], &d("zWA\\", Bhvadi), ini, &["prasTAyin"]);
}

#[ignore]
#[test]
fn unadi_4_10() {
    assert_has_krdanta(&["parama"], &d("zWA\\", Bhvadi), ini, &["paramezWin"]);
}

#[ignore]
#[test]
fn unadi_4_11() {
    assert_has_krdanta(&[], &d("maTi~", Bhvadi), ini, &["maTin"]);
}

#[test]
fn unadi_4_12() {
    assert_has_krdanta(&[], &d("patx~", Bhvadi), ini, &["paTin"]);
}

#[test]
fn unadi_4_13() {
    assert_has_krdanta(&[], &d("Kaja~", Bhvadi), Aka, &["KajAka"]);
}

#[test]
fn unadi_4_14() {
    assert_has_krdanta(&[], &d("bala~", Bhvadi), Aka, &["balAka"]);
    assert_has_krdanta(&[], &d("Sala~", Bhvadi), Aka, &["SalAka"]);
    assert_has_krdanta(&[], &d("patx~", Bhvadi), Aka, &["patAka"]);
}

#[ignore]
#[test]
fn unadi_4_15() {
    assert_has_krdanta(&[], &d("pA\\", Adadi), Aka, &["pinAka"]);
    assert_has_krdanta(&[], &d("taqa~", Curadi), Aka, &["taqAka"]);
}

#[test]
fn unadi_4_17() {
    assert_has_krdanta(&[], &d("ana~", Adadi), Ikan, &["anIka"]);
    assert_has_krdanta(&[], &d("hfza~", Divadi), Ikan, &["hfzIka"]);
}

#[test]
fn unadi_4_19() {
    assert_has_krdanta(&[], &d("SF", Kryadi), Ikan, &["SarSarIka"]);
    assert_has_krdanta(&[], &d("pF", Kryadi), Ikan, &["parparIka"]);
    assert_has_krdanta(&[], &d("vfY", Svadi), Ikan, &["varvarIka"]);
}

#[test]
fn unadi_4_21() {
    assert_has_krdanta(&[], &d("Iza~\\", Bhvadi), Ikan, &["izIka"]);
}

#[test]
fn unadi_4_22() {
    assert_has_krdanta(&[], &d("fja~\\", Bhvadi), Ikan, &["fjIka"]);
}

#[test]
fn unadi_4_23() {
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), Ikan, &["sfRIka"]);
}

#[test]
fn unadi_4_24() {
    let mrd = d("mfqa~", Tudadi);
    assert_has_krdanta(&[], &mrd, kIkac, &["mfqIka"]);
    assert_has_krdanta(&[], &mrd, kaNkaRa, &["mfqaNkaRa"]);
}

#[test]
fn unadi_4_26() {
    assert_has_krdanta(&[], &d("kF", Tudadi), Izan, &["karIza"]);
    assert_has_krdanta(&[], &d("tF", Bhvadi), Izan, &["tarIza"]);
}

#[test]
fn unadi_4_27() {
    assert_has_krdanta(&[], &d("SF", Kryadi), Izan, &["SirIza"]);
    assert_has_krdanta(&[], &d("pF", Kryadi), Izan, &["purIza"]);
}

#[test]
fn unadi_4_28() {
    assert_has_krdanta(&[], &d("arja~", Bhvadi), Izan, &["fjIza"]);
}

#[test]
fn unadi_4_29() {
    assert_has_krdanta(&[], &d("abi~\\", Bhvadi), Izan, &["ambarIza"]);
}

#[test]
fn unadi_4_30() {
    assert_has_krdanta(&[], &d("kF", Kryadi), Iran, &["karIra"]);
    assert_has_krdanta(&[], &d("SF", Kryadi), Iran, &["SarIra"]);
    assert_has_krdanta(&[], &d("pF", Kryadi), Iran, &["parIra"]);
    assert_has_krdanta(&[], &d("kawe~", Bhvadi), Iran, &["kawIra"]);
    assert_has_krdanta(&[], &d("pawa~", Bhvadi), Iran, &["pawIra"]);
    assert_has_krdanta(&[], &d("SOwf~", Bhvadi), Iran, &["SOwIra"]);
}

#[test]
fn unadi_4_31() {
    assert_has_krdanta(&[], &d("vaSa~", Adadi), Iran, &["vaSIra"]);
}

#[test]
fn unadi_4_32() {
    assert_has_krdanta(&[], &d("kaSa~\\", Adadi), Iran, &["kaSmIra"]);
}

#[test]
fn unadi_4_33() {
    assert_has_krdanta(&[], &d("qukf\\Y", Svadi), Iran, &["kurIra"]);
}

#[test]
fn unadi_4_34() {
    assert_has_krdanta(&[], &d("Gasx~", Bhvadi), Iran, &["kzIra"]);
}

#[test]
fn unadi_4_37() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), elimac, &["pacelima"]);
}

#[test]
fn unadi_4_38() {
    let shi = d("SIN", Adadi);
    assert_has_krdanta(&[], &shi, Duk, &["SIDu"]);
    assert_has_krdanta(&[], &shi, lak, &["SIla"]);
    assert_has_krdanta(&[], &shi, valaY, &["SEvala"]);
    assert_has_krdanta(&[], &shi, vAlan, &["SevAla", "SepAla"]);
}

#[test]
fn unadi_4_39() {
    assert_has_krdanta(&[], &d("mF", Kryadi), Uka, &["marUka"]);
    assert_has_krdanta(&[], &d("kaRa~", Bhvadi), UkaR, &["kARUka"]);
}

#[test]
fn unadi_4_40() {
    assert_has_krdanta(&[], &d("vala~\\", Bhvadi), Uka, &["valUka"]);
}

#[test]
fn unadi_4_42() {
    assert_has_krdanta(&[], &d("Sala~", Bhvadi), UkaR, &["SAlUka"]);
    assert_has_krdanta(&[], &d("maqi~", Bhvadi), UkaR, &["maRqUka"]);
}

#[test]
fn unadi_4_43() {
    assert_has_krdanta(&[], &d("RI\\Y", Bhvadi), mi, &["nemi"]);
}

#[test]
fn unadi_4_44() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), mi, &["Urmi"]);
}

#[test]
fn unadi_4_45() {
    assert_has_krdanta(&[], &d("BU", Bhvadi), mi, &["BUmi"]);
}

#[test]
fn unadi_4_46() {
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), mi, &["raSmi"]);
}

#[test]
fn unadi_4_47() {
    assert_has_krdanta(&[], &d("dala~", Bhvadi), mi, &["dalmi"]);
}

#[test]
fn unadi_4_49() {
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), ni, &["sfRi"]);
    assert_has_krdanta(&[], &d("vfzu~", Bhvadi), ni, &["vfzRi"]);
}

#[test]
fn unadi_4_50() {
    assert_has_krdanta(&[], &d("agi~", Bhvadi), ni, &["agni"]);
}

#[test]
fn unadi_4_51() {
    assert_has_krdanta(&[], &d("va\\ha~^", Bhvadi), ni, &["vahni"]);
    assert_has_krdanta(&[], &d("SriY", Bhvadi), ni, &["SreRi"]);
    assert_has_krdanta(&[], &d("Sru\\", Bhvadi), ni, &["SroRi"]);
    assert_has_krdanta(&[], &d("yu", Adadi), ni, &["yoni"]);
    assert_has_krdanta(&[], &d("dru\\", Bhvadi), ni, &["droRi"]);
    assert_has_krdanta(&[], &d("glE\\", Bhvadi), ni, &["glAni"]);
    assert_has_krdanta(&[], &d("o~hA\\N", Juhotyadi), ni, &["hAni"]);
    assert_has_krdanta(&[], &d("YitvarA~\\", Bhvadi), ni, &["tUrRi"]);
    assert_has_krdanta(&[], &d("mlE\\", Bhvadi), ni, &["mlAni"]);
}

#[test]
fn unadi_4_52() {
    assert_has_krdanta(&[], &d("Gf\\", Bhvadi), ni, &["GfRi"]);
    assert_has_krdanta(&[], &d("spf\\Sa~", Tudadi), ni, &["pfSni"]);
    assert_has_krdanta(&[], &d("pfzu~", Bhvadi), ni, &["pArzRi"]);
    assert_has_krdanta(&[], &d("cara~", Bhvadi), ni, &["cUrRi"]);
    assert_has_krdanta(&[], &d("quBf\\Y", Juhotyadi), ni, &["BUrRi"]);
}

#[test]
fn unadi_4_53() {
    assert_has_krdanta(&[], &d("vfY", Svadi), vin, &["varvi"]);
    assert_has_krdanta(&[], &d("df", Svadi), vin, &["darvi"]);
}

#[test]
fn unadi_4_54() {
    assert_has_krdanta(&[], &d("jF", Kryadi), kvin, &["jIrvi"]);
    assert_has_krdanta(&[], &d("SFY", Kryadi), kvin, &["SIrvi"]);
    assert_has_krdanta(&[], &d("stFY", Kryadi), kvin, &["stIrvi"]);
    assert_has_krdanta(&[], &d("jAgf", Adadi), kvin, &["jAgfvi"]);
}

#[test]
fn unadi_4_55() {
    assert_has_krdanta(&[], &d("divu~", Divadi), kvin, &["dIdivi"]);
}

#[test]
fn unadi_4_56() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), kvin, &["kfvi"]);
    assert_has_krdanta(&[], &d("Gfzu~", Bhvadi), kvin, &["Gfzvi"]);
    assert_has_krdanta(&[], &d("Co\\", Divadi), kvin, &["Cavi"]);
    assert_has_krdanta(
        &["kikI"],
        &d("divu~", Divadi),
        kvin,
        &["kikIdivi", "kikIdIvi"],
    );
}

#[test]
fn unadi_4_57() {
    assert_has_krdanta(&[], &d("pA\\", Adadi), qati, &["pati"]);
}

#[test]
fn unadi_4_58() {
    assert_has_krdanta(&[], &d("Sa\\kx~", Svadi), ftin, &["Sakft"]);
}

#[test]
fn unadi_4_59() {
    assert_has_krdanta(&[], &d("ama~", Bhvadi), ati_, &["amati"]);
}

#[test]
fn unadi_4_62() {
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), ati_, &["aMhati"]);
}

#[test]
fn unadi_4_63() {
    assert_has_krdanta(&[], &d("ra\\ma~\\", Bhvadi), ati_, &["ramati"]);
}

#[test]
fn unadi_4_64() {
    assert_has_krdanta(&[], &d("zUN", Adadi), kri, &["sUri"]);
}

#[test]
fn unadi_4_65() {
    assert_has_krdanta(&[], &d("a\\da~", Adadi), krin, &["adri"]);
    assert_has_krdanta(&[], &d("Sa\\dx~", Bhvadi), krin, &["Sadri"]);
    assert_has_krdanta(&[], &d("BU", Bhvadi), krin, &["BUri"]);
    assert_has_krdanta(&[], &d("SuBa~", Bhvadi), krin, &["SuBri"]);
}

#[test]
fn unadi_4_67() {
    assert_has_krdanta(&[], &d("rA\\", Adadi), trip, &["rAtri"]);
    assert_has_krdanta(&[], &d("Sa\\dx~", Bhvadi), trip, &["Sattri"]);
}

#[ignore]
#[test]
fn unadi_4_68() {
    let ad = d("a\\da~", Adadi);
    assert_has_krdanta(&[], &ad, trip, &["attri"]);
    assert_has_krdanta(&[], &ad, trin, &["attri"]);
}

#[test]
fn unadi_4_69() {
    assert_has_krdanta(&[], &d("patx~", Bhvadi), atrin, &["patatri"]);
}

#[test]
fn unadi_4_70() {
    assert_has_krdanta(&[], &d("mf\\N", Tudadi), Ici, &["marIci"]);
    assert_has_krdanta(&[], &d("kaRa~", Bhvadi), Ici, &["kaRIci"]);
}

#[test]
fn unadi_4_72() {
    assert_has_krdanta(&[], &d("ve\\Y", Bhvadi), Ici, &["vIci"]);
}

#[test]
fn unadi_4_73() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), Uzan, &["arUza"]);
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), Uzan, &["hanUza"]);
}

#[test]
fn unadi_4_74() {
    assert_has_krdanta(&[], &d("pura~", Tudadi), kuzan, &["puruza", "pUruza"]);
}

#[test]
fn unadi_4_75() {
    assert_has_krdanta(&[], &d("pF", Kryadi), uzac, &["paruza"]);
    assert_has_krdanta(&[], &d("Ra\\ha~^", Divadi), uzac, &["nahuza"]);
    assert_has_krdanta(&[], &d("kala~\\", Bhvadi), uzac, &["kaluza"]);
}

#[test]
fn unadi_4_77() {
    assert_has_krdanta(&[], &d("gaqa~", Bhvadi), Uzan, &["gaRqUza"]);
}

#[test]
fn unadi_4_78() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), aru, &["araru"]);
}

#[test]
fn unadi_4_79() {
    assert_has_krdanta(&[], &d("kuwa~", Tudadi), aru, &["kuwaru"]);
}

#[test]
fn unadi_4_80() {
    assert_has_krdanta(&[], &d("Sa\\kx~", Svadi), awan, &["Sakawa"]);
    assert_has_krdanta(&[], &d("kaki~\\", Bhvadi), awan, &["kaNkawa"]);
    assert_has_krdanta(&[], &d("divu~", Divadi), awan, &["devawa"]);
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), awan, &["karawa"]);
}

#[test]
fn unadi_4_82() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), ambac, &["karamba"]);
    // kAdamba also by 4.83.
    assert_has_krdanta(&[], &d("kada~\\", Bhvadi), ambac, &["kadamba", "kAdamba"]);
    assert_has_krdanta(&[], &d("kaqa~", Bhvadi), ambac, &["kaqamba"]);
    assert_has_krdanta(&[], &d("kawe~", Bhvadi), ambac, &["kawamba"]);
}

#[test]
fn unadi_4_83() {
    assert_has_krdanta(&[], &d("kada~\\", Bhvadi), ambac, &["kAdamba", "kadamba"]);
}

#[test]
fn unadi_4_84() {
    assert_has_krdanta(&[], &d("kala~\\", Bhvadi), ama, &["kalama"]);
    assert_has_krdanta(&[], &d("karda~", Bhvadi), ama, &["kardama"]);
}

#[test]
fn unadi_4_85() {
    assert_has_krdanta(&[], &d("kuRa~", Tudadi), kindac, &["kuRinda"]);
    assert_has_krdanta(&[], &d("pula~", Bhvadi), kindac, &["pulinda"]);
}

#[test]
fn unadi_4_86() {
    assert_has_krdanta(&[], &d("kupa~", Divadi), kindac, &["kupinda", "kuvinda"]);
}

#[test]
fn unadi_4_87() {
    assert_has_krdanta(&["ni"], &d("za\\nja~", Bhvadi), GaTin, &["nizaNgaTi"]);
}

#[test]
fn unadi_4_88() {
    assert_has_krdanta(&["ud"], &d("f\\", Bhvadi), GaTin, &["udaraTi"]);
}

#[test]
fn unadi_4_89() {
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), GaTin, &["sAraTi"]);
}

#[ignore]
#[test]
fn unadi_4_91() {
    assert_has_krdanta(&[], &d("ku\\", Adadi), caw, &["kUcI"]);
}

#[test]
fn unadi_4_94() {
    assert_has_krdanta(&[], &d("Samu~", Divadi), ban, &["Samba"]);
}

#[test]
fn unadi_4_96() {
    let stha = &d("zWA\\", Bhvadi);
    assert_has_krdanta(&[], &stha, ambac, &["stamba"]);
    assert_has_krdanta(&[], &stha, abaka, &["stabaka"]);
}

#[test]
fn unadi_4_97() {
    assert_has_krdanta(&[], &d("So\\", Divadi), da, &["SAda"]);
    assert_has_krdanta(&[], &d("Sa\\pa~^", Bhvadi), dan, &["Sabda"]);
}

#[test]
fn unadi_4_99() {
    assert_has_krdanta(&[], &d("vala~\\", Bhvadi), kayan, &["valaya"]);
    assert_has_krdanta(&[], &d("mala~\\", Bhvadi), kayan, &["malaya"]);
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), kayan, &["tanaya"]);
}

#[test]
fn unadi_4_100() {
    assert_has_krdanta(&[], &d("vf\\", Bhvadi), kayan, &["vfzaya"]);
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), kayan, &["hfdaya"]);
}

#[test]
fn unadi_4_101() {
    assert_has_krdanta(&[], &d("mI\\N", Divadi), ru, &["meru"]);
    assert_has_krdanta(&[], &d("pI\\N", Divadi), ru, &["peru"]);
}

#[ignore]
#[test]
fn unadi_4_103() {
    assert_has_krdanta(&[], &d("ru", Adadi), krun, &["ruru"]);
}

#[test]
fn unadi_4_106() {
    let kus = d("kusa~", Divadi);
    assert_has_krdanta(&[], &kus, umBa, &["kusumBa"]);
    assert_has_krdanta(&[], &kus, uma, &["kusuma"]);
    assert_has_krdanta(&[], &kus, Ida, &["kusIda"]);
    assert_has_krdanta(&[], &kus, ita, &["kusita"]);
}

#[test]
fn unadi_4_108() {
    assert_has_krdanta(&[], &d("mUN", Bhvadi), kla, &["mUla"]);
    assert_has_krdanta(&[], &d("Sa\\kx~", Svadi), kla, &["Sakla"]);
    assert_has_krdanta(&[], &d("abi~\\", Bhvadi), kla, &["ambla"]);
    assert_has_krdanta(&[], &d("ama~", Bhvadi), kla, &["amla"]);
}

#[test]
fn unadi_4_109() {
    assert_has_krdanta(&[], &d("mA\\", Adadi), ya, &["mAyA"]);
    assert_has_krdanta(&[], &d("Co\\", Divadi), ya, &["CAyA"]);
    assert_has_krdanta(&[], &d("zasa~", Adadi), ya, &["sasya"]);
    assert_has_krdanta(&[], &d("zu\\Y", Svadi), ya, &["savya"]);
}

#[test]
fn unadi_4_115() {
    assert_has_krdanta(&[], &d("a\\da~", Adadi), kvanip, &["aDvan"]);
}

#[test]
fn unadi_4_117() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), in_, &["paci"]);
    assert_has_krdanta(&[], &d("tuqa~", Tudadi), in_, &["tuqi"]);
    assert_has_krdanta(&[], &d("tuqi~\\", Bhvadi), in_, &["tuRqi"]);
    assert_has_krdanta(&[], &d("vala~\\", Bhvadi), in_, &["vali"]);
    assert_has_krdanta(&[], &d("vawa~", Bhvadi), in_, &["vawi"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), in_, &["yaji"]);
    assert_has_upapada_krdanta("deva", &[], &d("ya\\ja~^", Bhvadi), in_, &["devayaji"]);
    assert_has_krdanta(&[], &d("kASf~\\", Bhvadi), in_, &["kASi"]);
    assert_has_krdanta(&[], &d("yatI~\\", Bhvadi), in_, &["yati"]);
    assert_has_krdanta(&[], &d("malla~\\", Bhvadi), in_, &["malli"]);
    assert_has_krdanta(&[], &d("kelf~", Bhvadi), in_, &["keli"]);
    assert_has_krdanta(&[], &d("masI~", Divadi), in_, &["masi"]);
    assert_has_krdanta(&[], &d("kuwa~", Tudadi), in_, &["kowi"]);
    assert_has_krdanta(&[], &d("hila~", Tudadi), in_, &["heli"]);
    assert_has_krdanta(&[], &d("buDa~", Bhvadi), in_, &["boDi"]);
    assert_has_krdanta(&[], &d("wunadi~", Bhvadi), in_, &["nandi"]);
    assert_has_krdanta(&[], &d("kala~\\", Bhvadi), in_, &["kali"]);
}

#[test]
fn unadi_4_119() {
    assert_has_krdanta(&[], &d("kf\\za~", Bhvadi), in_, &["kfzi"]);
    assert_has_krdanta(&[], &d("fzI~", Tudadi), in_, &["fzi"]);
    assert_has_krdanta(&[], &d("I~Suci~^r", Divadi), in_, &["Suci"]);
    assert_has_krdanta(&[], &d("li\\pa~^", Tudadi), in_, &["lipi", "libi"]);
    assert_has_krdanta(&[], &d("tUla~", Bhvadi), in_, &["tUli"]);
}

#[test]
fn unadi_4_120() {
    assert_has_krdanta(&[], &d("Bramu~", Bhvadi), in_, &["Bfmi", "Brami"]);
}

#[test]
fn unadi_4_122() {
    assert_has_krdanta(&[], &d("ma\\na~\\", Divadi), in_, &["muni"]);
}

#[test]
fn unadi_4_125() {
    assert_has_krdanta(&[], &d("Ra\\ha~^", Divadi), iY, &["nABi"]);
}

#[test]
fn unadi_4_126() {
    assert_has_krdanta(&[], &d("SF", Kryadi), iY, &["SAri"]);
}

#[test]
fn unadi_4_129() {
    assert_has_krdanta(&[], &d("janI~\\", Divadi), iR, &["jani"]);
    assert_has_krdanta(&[], &d("Gasx~", Bhvadi), iR, &["GAsi"]);
}

#[test]
fn unadi_4_134() {
    assert_has_krdanta(&["pra"], &d("hf\\Y", Bhvadi), iR, &["prahi"]);
}

#[test]
fn unadi_4_135() {
    assert_has_krdanta(&["ni"], &d("vye\\Y", Bhvadi), iR, &["nIvi"]);
}

#[test]
fn unadi_4_138() {
    assert_has_krdanta(&[], &d("ru", Adadi), i, &["ravi"]);
    assert_has_krdanta(&[], &d("pUN", Bhvadi), i, &["pavi"]);
    assert_has_krdanta(&[], &d("tF", Bhvadi), i, &["tari"]);
    assert_has_krdanta(&[], &d("kUN", Tudadi), i, &["kavi"]);
    assert_has_krdanta(&[], &d("f\\", Bhvadi), i, &["ari"]);
}

#[test]
fn unadi_4_141() {
    assert_has_krdanta(&[], &d("Bu\\ja~", Rudhadi), i, &["Buji"]);
}

#[test]
fn unadi_4_142() {
    assert_has_krdanta(&[], &d("kF", Tudadi), i, &["kiri"]);
    assert_has_krdanta(&[], &d("gF", Tudadi), i, &["giri"]);
    assert_has_krdanta(&[], &d("SF", Kryadi), i, &["Siri"]);
    assert_has_krdanta(&[], &d("pF", Kryadi), i, &["puri"]);
    assert_has_krdanta(&[], &d("kuwa~", Tudadi), i, &["kuwi"]);
    assert_has_krdanta(&[], &d("Bi\\di~^r", Rudhadi), i, &["Bidi"]);
    assert_has_krdanta(&[], &d("Ci\\di~^r", Rudhadi), i, &["Cidi"]);
}

#[test]
fn unadi_4_143() {
    assert_has_krdanta(&[], &d("kuqa~", Tudadi), i, &["kuqi"]);
    assert_has_krdanta(&[], &d("kapi~\\", Bhvadi), i, &["kampi"]);
}

#[test]
fn unadi_4_144() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), manin, &["karman"]);
    assert_has_krdanta(&[], &d("cara~", Bhvadi), manin, &["carman"]);
    assert_has_krdanta(&[], &d("Basa~", Juhotyadi), manin, &["Basman"]);
    assert_has_krdanta(&[], &d("janI~\\", Divadi), manin, &["janman"]);
    assert_has_krdanta(&[], &d("SF", Kryadi), manin, &["Sarman"]);
    assert_has_krdanta(&[], &d("Cada~", Curadi), manin, &["Cadman"]);
    // TODO: sutrAman
}

#[test]
fn unadi_4_145() {
    assert_has_krdanta(&[], &d("bfhi~", Bhvadi), manin, &["brahman"]);
}

#[test]
fn unadi_4_147() {
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), imanic, &["hariman"]);
    assert_has_krdanta(&[], &d("Bf\\Y", Bhvadi), imanic, &["Bariman"]);
    assert_has_krdanta(&[], &d("Df\\Y", Bhvadi), imanic, &["Dariman"]);
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), imanic, &["sariman"]);
    assert_has_krdanta(&[], &d("stf\\Y", Svadi), imanic, &["stariman"]);
    assert_has_krdanta(&[], &d("SF", Kryadi), imanic, &["Sariman"]);
}

#[test]
fn unadi_4_148() {
    assert_has_krdanta(&[], &d("janI~\\", Divadi), imanin, &["janiman"]);
    assert_has_krdanta(&[], &d("mf\\N", Tudadi), imanin, &["mariman"]);
}

#[test]
fn unadi_4_149() {
    assert_has_krdanta(&[], &d("ve\\Y", Bhvadi), imanin, &["veman"]);
}

#[test]
fn unadi_4_154() {
    assert_has_krdanta(&[], &d("kUN", Tudadi), aran, &["kavara"]);
}

#[test]
fn unadi_4_155() {
    assert_has_krdanta(&[], &d("gF", Tudadi), uqac, &["garuqa"]);
}

#[test]
fn unadi_4_156() {
    assert_has_krdanta(&[], &d("idi~", Bhvadi), kamin, &["idam"]);
}

#[test]
fn unadi_4_157() {
    assert_has_krdanta(&[], &d("kE\\", Bhvadi), qimi, &["kim"]);
}

#[test]
fn unadi_4_158() {
    assert_has_krdanta(&[], &d("vasa~\\", Adadi), zwran, &["vastra"]);
    assert_has_krdanta(&[], &d("asu~", Divadi), zwran, &["astra"]);
    assert_has_krdanta(&[], &d("Sasu~", Bhvadi), zwran, &["Sastra"]);
}

#[test]
fn unadi_4_161() {
    assert_has_krdanta(&[], &d("uza~", Bhvadi), zwran, &["uzwra"]);
    assert_has_krdanta(&[], &d("Kanu~^", Bhvadi), zwran, &["KAtra"]);
}

#[test]
fn unadi_4_162() {
    assert_has_krdanta(&[], &d("zivu~", Divadi), zwran, &["sUtra"]);
    assert_has_krdanta(&[], &d("mu\\cx~^", Tudadi), zwran, &["mUtra"]);
}

#[test]
fn unadi_4_164() {
    assert_has_krdanta(&[], &d("pUN", Bhvadi), zwran, &["putra"]);
}

#[test]
fn unadi_4_165() {
    assert_has_krdanta(&[], &d("styE\\", Bhvadi), qraw, &["strI"]);
}

#[test]
fn unadi_4_168() {
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), tran, &["gAtra"]);
}

#[test]
fn unadi_4_171() {
    assert_has_krdanta(&[], &d("cara~", Bhvadi), Ritran, &["cAritra"]);
}

#[test]
fn unadi_4_173() {
    assert_has_krdanta(&[], &d("ama~", Bhvadi), itra, &["amitra"]);
}

#[test]
fn unadi_4_176() {
    assert_has_krdanta(&[], &d("sUca", Curadi), sman, &["sUkzma"]);
}

#[test]
fn unadi_4_177() {
    assert_has_krdanta(&[], &d("pA\\", Adadi), qumsun, &["pums"]);
}

#[test]
fn unadi_4_180() {
    assert_has_krdanta(&["su"], &d("asa~", Adadi), ti, &["svasti"]);
}

#[test]
fn unadi_4_181() {
    assert_has_krdanta(&["vi"], &d("tasu~", Divadi), ti, &["vitasti"]);
}

#[test]
fn unadi_4_184() {
    assert_has_krdanta(&[], &d("kF", Tudadi), kIwan, &["kirIwa"]);
    assert_has_krdanta(&[], &d("tF", Bhvadi), kIwan, &["tirIwa"]);
    assert_has_krdanta(&[], &d("kfpU~\\", Bhvadi), kIwan, &["kfpIwa"]);
}

#[test]
fn unadi_4_185() {
    assert_has_krdanta(&[], &d("ruca~\\", Bhvadi), kitac, &["rucita"]);
    assert_has_krdanta(&[], &d("va\\ca~", Adadi), kitac, &["ucita"]);
    assert_has_krdanta(&[], &d("kuca~", Bhvadi), kitac, &["kucita"]);
    assert_has_krdanta(&[], &d("kuwa~", Tudadi), kitac, &["kuwita"]);
}

#[test]
fn unadi_4_186() {
    assert_has_krdanta(&[], &d("kuwa~", Tudadi), kmalan, &["kuqmala"]);
    // kulmala is from 4.187.
    assert_has_krdanta(&[], &d("kuza~", Kryadi), kmalan, &["kuzmala", "kulmala"]);
}

#[test]
fn unadi_4_187() {
    // kuzmala is from 4.186.
    assert_has_krdanta(&[], &d("kuza~", Kryadi), kmalan, &["kuzmala", "kulmala"]);
}

#[test]
fn unadi_4_188() {
    assert_has_krdanta(&[], &d("citI~", Bhvadi), asun, &["cetas"]);
    assert_has_krdanta(&[], &d("sf\\", Bhvadi), asun, &["saras"]);
    assert_has_krdanta(&[], &d("pI\\N", Divadi), asun, &["payas"]);
    assert_has_krdanta(&[], &d("za\\dx~", Bhvadi), asun, &["sadas"]);
}

#[test]
fn unadi_4_189() {
    assert_has_krdanta(&[], &d("rapa~", Bhvadi), asun, &["repas"]);
}

#[test]
fn unadi_4_190() {
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), asun, &["yaSas"]);
}

#[test]
fn unadi_4_191() {
    assert_has_krdanta(&[], &d("ubja~", Tudadi), asun, &["ojas"]);
}

#[test]
fn unadi_4_192() {
    assert_has_krdanta(&[], &d("wuo~Svi", Bhvadi), asun, &["Savas"]);
}

#[test]
fn unadi_4_193() {
    assert_has_krdanta(&[], &d("SriY", Bhvadi), asun, &["Siras"]);
}

#[test]
fn unadi_4_194_and_unadi_4_195_and_unadi_4_196() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), asun, &["uras", "arSas", "arRas"]);
}

#[test]
fn unadi_4_197() {
    assert_has_krdanta(&[], &d("i\\R", Adadi), asun, &["enas"]);
}

#[test]
fn unadi_4_201() {
    assert_has_krdanta(&[], &d("sru\\", Bhvadi), asun, &["srotas"]);
    assert_has_krdanta(&[], &d("rI\\N", Divadi), asun, &["retas"]);
}

#[test]
fn unadi_4_202_and_unadi_4_203_and_unadi_4_204() {
    assert_has_krdanta(&[], &d("pA\\", Adadi), asun, &["pAjas", "pATas"]);
}

#[test]
fn unadi_4_206() {
    assert_has_krdanta(&[], &d("ska\\ndi~r", Bhvadi), asun, &["skandas"]);
}

#[test]
fn unadi_4_210() {
    assert_has_krdanta(&[], &d("Ra\\ha~^", Divadi), asun, &["naBas"]);
}

#[test]
fn unadi_4_211() {
    assert_has_krdanta(&[], &d("ama~", Bhvadi), asun, &["aMhas"]);
}

#[test]
fn unadi_4_212_and_unadi_4_213() {
    assert_has_krdanta(&[], &d("ra\\ma~\\", Bhvadi), asun, &["raMhas", "rahas"]);
}

#[test]
fn unadi_4_217() {
    assert_has_krdanta(&[], &d("vasa~\\", Adadi), asun, &["vAsas"]);
}

#[test]
fn unadi_4_218() {
    assert_has_krdanta(&[], &d("cadi~", Bhvadi), asun, &["Candas"]);
}

#[test]
fn unadi_4_219() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), asun, &["pakzas"]);
    assert_has_krdanta(&[], &d("va\\ca~", Adadi), asun, &["vakzas"]);
}

#[test]
fn unadi_4_221() {
    assert_has_krdanta(&[], &d("i\\R", Adadi), Asi, &["ayAs"]);
}

#[test]
fn unadi_4_225() {
    assert_has_krdanta(&[], &d("Ru", Adadi), asi, &["noDas"]);
}

#[test]
fn unadi_4_227() {
    assert_has_krdanta(&["candra"], &d("mA\\N", Divadi), asi, &["candramas"]);
}

#[test]
fn unadi_4_228() {
    assert_has_krdanta(&["vayas"], &d("quDA\\Y", Juhotyadi), asi, &["vayoDAs"]);
}

#[test]
fn unadi_4_229() {
    assert_has_krdanta(&["payas"], &d("quDA\\Y", Juhotyadi), asi, &["payoDAs"]);
}

#[test]
fn unadi_4_230() {
    assert_has_krdanta(&["puras"], &d("quDA\\Y", Juhotyadi), asi, &["puroDAs"]);
}

#[test]
fn unadi_4_231() {
    assert_has_krdanta(&["puru"], &d("ru", Adadi), asi, &["purUravas"]);
}

#[test]
fn unadi_4_232() {
    assert_has_krdanta(&["nf"], &d("ca\\kzi~\\N", Adadi), asi, &["nfcakzas"]);
}

#[test]
fn unadi_4_233() {
    assert_has_krdanta(&[], &d("uza~", Bhvadi), asi, &["uzas"]);
}

#[test]
fn unadi_4_234() {
    assert_has_krdanta(&[], &d("damu~", Divadi), unasi, &["damunas"]);
}

#[test]
fn unadi_4_235() {
    assert_has_krdanta(&[], &d("agi~", Bhvadi), asi, &["aNgiras"]);
}

#[test]
fn unadi_4_236() {
    assert_has_krdanta(&["ap"], &d("sf\\", Bhvadi), asi, &["apsaras"]);
}

#[test]
fn unadi_4_237() {
    assert_has_krdanta(&["viSva"], &d("vida~", Adadi), asi, &["viSvavedas"]);
    assert_has_krdanta(&["viSva"], &d("Bu\\ja~", Rudhadi), asi, &["viSvaBojas"]);
}

#[test]
fn unadi_4_238() {
    assert_has_krdanta(&[], &d("vaSa~", Adadi), kanasi, &["uSanas"]);
}

#[test]
fn unadi_5_1() {
    assert_has_krdanta(&["ad"], &d("BU", Bhvadi), qutac, &["adButa"]);
}

#[test]
fn unadi_5_2() {
    assert_has_krdanta(&[], &d("guDa~", Divadi), Uma, &["goDUma"]);
}

#[test]
fn unadi_5_3() {
    assert_has_krdanta(&[], &d("masI~", Divadi), Uran, &["masUra"]);
}

#[test]
fn unadi_5_4() {
    assert_has_krdanta(&[], &d("zWA\\", Bhvadi), Uran, &["sTUra"]);
}

#[test]
fn unadi_5_5() {
    assert_has_krdanta(&[], &d("pA\\", Adadi), ati_, &["pAti"]);
}

#[test]
fn unadi_5_6() {
    assert_has_krdanta(&[], &d("vA\\", Adadi), ati_, &["vAti"]);
}

#[test]
fn unadi_5_7() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), ati_, &["arati"]);
}

#[test]
fn unadi_5_8() {
    assert_has_krdanta(&[], &d("tfhU~", Tudadi), kna, &["tfRa"]);
}

#[ignore]
#[test]
fn unadi_5_10() {
    let dans = d("dasi~", Curadi);
    assert_has_krdanta(&[], &dans, wa, &["dAsa"]);
    assert_has_krdanta(&[], &dans, wan, &["dAsa"]);
}

#[ignore]
#[test]
fn unadi_5_11() {
    let dansh = d("da\\nSa~", Bhvadi);
    assert_has_krdanta(&[], &dansh, wa, &["dASa"]);
    assert_has_krdanta(&[], &dansh, wan, &["dASa"]);
}

#[ignore]
#[test]
fn unadi_5_12() {
    assert_has_krdanta(&["ud"], &d("ci\\Y", Svadi), qEsi, &["uccEH"]);
}

#[ignore]
#[test]
fn unadi_5_13() {
    assert_has_krdanta(&["ni"], &d("ci\\Y", Svadi), qEsi, &["nIcEH"]);
}

#[test]
fn unadi_5_14() {
    assert_has_krdanta(&["su"], &d("ra\\ma~\\", Bhvadi), kta, &["sUrata"]);
}

#[test]
fn unadi_5_15() {
    assert_has_krdanta(&[], &d("pUY", Kryadi), yat, &["puRya"]);
}

#[test]
fn unadi_5_16() {
    assert_has_krdanta(&[], &d("sransu~\\", Bhvadi), yat, &["Sikya"]);
}

#[test]
fn unadi_5_17() {
    assert_has_krdanta(&[], &d("f\\", Bhvadi), kyu, &["uraRa"]);
}

#[test]
fn unadi_5_18() {
    assert_has_krdanta(&[], &d("hisi~", Rudhadi), Iran, &["hiMsIra"]);
    assert_has_krdanta(&[], &d("hisi~", Rudhadi), Irac, &["hiMsIra"]);
}

#[test]
fn unadi_5_19() {
    let dr = &d("dF", Kryadi);
    assert_has_krdanta(&["ud"], &dr, al, &["udara"]);
    assert_has_krdanta(&["ud"], &dr, ac, &["udara"]);
}

#[test]
fn unadi_5_20() {
    let khan = d("Kanu~^", Bhvadi);
    assert_has_krdanta(&[], &khan, al, &["muKa"]);
    assert_has_krdanta(&[], &khan, ac, &["muKa"]);
}

#[test]
fn unadi_5_21() {
    assert_has_krdanta(&[], &d("ama~", Bhvadi), U::san, &["aMsa"]);
}

#[test]
fn unadi_5_22() {
    assert_has_krdanta(&[], &d("mu\\ha~", Divadi), Ka, &["mUrKa"]);
}

#[test]
fn unadi_5_23() {
    assert_has_krdanta(&[], &d("Ra\\ha~^", Divadi), Ka, &["naKa"]);
}

#[test]
fn unadi_5_24() {
    assert_has_krdanta(&[], &d("SIN", Adadi), Ka, &["SiKA"]);
}

#[test]
fn unadi_5_25() {
    assert_has_krdanta(&[], &d("mA\\N", Divadi), UKa, &["mayUKa"]);
}

#[test]
fn unadi_5_26() {
    assert_has_krdanta(&[], &d("kala~\\", Bhvadi), Pak, &["kulPa"]);
    assert_has_krdanta(&[], &d("gala~", Bhvadi), Pak, &["gulPa"]);
}

#[test]
fn unadi_5_27() {
    let sprsh = d("spf\\Sa~", Tudadi);
    assert_has_krdanta(&[], &sprsh, SvaR, &["pArSva"]);
    assert_has_krdanta(&[], &sprsh, Sun, &["parSu"]);
}

#[test]
fn unadi_5_28() {
    assert_has_krdanta(&["Sman"], &d("Sri\\Y", Bhvadi), qun, &["SmaSru"]);
}

#[test]
fn unadi_5_30() {
    assert_has_krdanta(&[], &d("janI~\\", Divadi), wan, &["jawA"]);
}

#[test]
fn unadi_5_31() {
    assert_has_krdanta(&[], &d("janI~\\", Divadi), ac, &["jaNGA"]);
}

#[test]
fn unadi_5_32() {
    // Gora by 5.64
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), ac, &["jaGana", "Gora"]);
}

#[test]
fn unadi_5_33() {
    assert_has_krdanta(&[], &d("kliSa~\\", Divadi), an, &["keSa"]);
}

#[test]
fn unadi_5_34() {
    assert_has_krdanta(&[], &d("Pala~", Bhvadi), itac, &["palita"]);
}

#[test]
fn unadi_5_36() {
    assert_has_krdanta(&[], &d("cIka~", Curadi), vun, &["kIcaka"]);
}

#[test]
fn unadi_5_37() {
    assert_has_krdanta(&[], &d("qupa\\ca~^z", Bhvadi), vun, &["pecaka"]);
    assert_has_krdanta(&[], &d("maca~\\", Bhvadi), vun, &["mecaka"]);
}

#[test]
fn unadi_5_38() {
    assert_has_krdanta(&[], &d("janI~\\", Divadi), ara, &["jaWara"]);
}

#[test]
fn unadi_5_39() {
    assert_has_krdanta(&[], &d("va\\ca~", Adadi), ara, &["vaWara"]);
    assert_has_krdanta(&[], &d("ma\\na~\\", Divadi), ara, &["maWara"]);
}

#[test]
fn unadi_5_40() {
    let dr = &d("dF", Kryadi);
    assert_has_krdanta(&["Urj"], &dr, al, &["Urdara"]);
    assert_has_krdanta(&["Urj"], &dr, ac, &["Urdara"]);
}

#[test]
fn unadi_5_42() {
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), yun, &["GAtana"]);
}

#[test]
fn unadi_5_43() {
    assert_has_krdanta(&[], &d("kramu~", Bhvadi), tun, &["krAntu"]);
    // "gantu" by 1.69
    assert_has_krdanta(&[], &d("ga\\mx~", Bhvadi), tun, &["gAntu", "gantu"]);
    assert_has_krdanta(&[], &d("kzamU~", Divadi), tun, &["kzAntu"]);
}

#[test]
fn unadi_5_44() {
    assert_has_krdanta(&[], &d("hf\\Y", Bhvadi), kanyan, &["hiraRya"]);
}

#[test]
fn unadi_5_45() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), pAsa, &["karpAsa"]);
}

#[test]
fn unadi_5_46() {
    // "jantu" is from 1.72
    assert_has_krdanta(&[], &d("janI~\\", Divadi), tu, &["jartu", "jantu"]);
}

#[test]
fn unadi_5_47() {
    assert_has_krdanta(&[], &d("UrRuY", Adadi), qa, &["UrRA"]);
}

#[test]
fn unadi_5_48() {
    assert_has_krdanta(&[], &d("quDA\\Y", Juhotyadi), yat, &["DAnya"]);
}

#[ignore]
#[test]
fn unadi_5_49() {
    assert_has_krdanta(&[], &d("jF", Kryadi), krin, &["jivri"]);
}

#[test]
fn unadi_5_50() {
    assert_has_krdanta(&[], &d("mavya~", Bhvadi), Ala, &["mamApatAla"]);
}

#[test]
fn unadi_5_51() {
    assert_has_krdanta(&[], &d("fja~\\", Bhvadi), kIkan, &["fjIka"]);
}

#[test]
fn unadi_5_52() {
    assert_has_krdanta(&[], &d("tanu~^", Tanadi), qau, &["titau"]);
}

#[test]
fn unadi_5_53() {
    assert_has_krdanta(&[], &d("fDu~", Divadi), vun, &["arBaka"]);
    assert_has_krdanta(&[], &d("praTa~\\", Bhvadi), kukan, &["pfTuka"]);
    assert_has_krdanta(&[], &d("pA\\", Bhvadi), kan, &["pAka"]);
}

#[test]
fn unadi_5_54() {
    assert_has_krdanta(&["naY"], &d("vada~", Bhvadi), yat, &["avadya"]);
    assert_has_krdanta(&[], &d("ava~", Bhvadi), ama, &["avama", "aDama"]);
    assert_has_krdanta(&[], &d("f\\", Bhvadi), vanip, &["arvan"]);
    assert_has_krdanta(&[], &d("riPa~", Tudadi), a, &["rePa"]);
}

#[test]
fn unadi_5_55() {
    assert_has_krdanta(&[], &d("lI\\", Kryadi), ta, &["lipta"]);
    assert_has_krdanta(&[], &d("rI\\N", Divadi), ra, &["ripra"]);
}

#[ignore]
#[test]
fn unadi_5_56() {
    assert_has_krdanta(&[], &d("kliSa~\\", Divadi), kan, &["kInASa"]);
}

#[test]
fn unadi_5_57() {
    assert_has_krdanta(&[], &d("aSU~\\", Svadi), varaw, &["ISvara"]);
}

#[test]
fn unadi_5_58() {
    assert_has_krdanta(&[], &d("cate~^", Bhvadi), uran_, &["catur"]);
}

#[test]
fn unadi_5_59() {
    assert_has_krdanta(&["pra"], &d("ata~", Bhvadi), aran_, &["prAtar"]);
}

#[test]
fn unadi_5_60() {
    assert_has_krdanta(&[], &d("ama~", Bhvadi), aran_, &["antar"]);
}

#[test]
fn unadi_5_61() {
    assert_has_krdanta(&[], &d("da\\ha~", Bhvadi), ga, &["naga"]);
}

#[test]
fn unadi_5_62() {
    assert_has_krdanta(&[], &d("zi\\ca~^", Tudadi), ka, &["siMha"]);
}

#[test]
fn unadi_5_63() {
    assert_has_krdanta(&["vi", "AN"], &d("GrA\\", Bhvadi), ka, &["vyAGra"]);
}

#[test]
fn unadi_5_64() {
    // jaGana by 5.32
    assert_has_krdanta(&[], &d("ha\\na~", Adadi), ac, &["Gora", "jaGana"]);
}

#[test]
fn unadi_5_65() {
    assert_has_krdanta(&[], &d("kzamU~", Divadi), ac, &["kzmA"]);
}

#[test]
fn unadi_5_66() {
    assert_has_krdanta(&[], &d("tF", Bhvadi), qri, &["tri"]);
}

#[test]
fn unadi_5_67() {
    assert_has_krdanta(&[], &d("graha~^", Kryadi), ani, &["grahaRi"]);
}

#[test]
fn unadi_5_68() {
    assert_has_krdanta(&[], &d("praTa~\\", Bhvadi), amac, &["praTama"]);
}

#[test]
fn unadi_5_69() {
    assert_has_krdanta(&[], &d("cara~", Bhvadi), amac, &["carama"]);
}

#[test]
fn unadi_5_70() {
    assert_has_krdanta(&[], &d("magi~", Bhvadi), alac, &["maNgala"]);
}
