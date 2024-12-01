extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha::*;

#[test]
fn skip_sutra_6_3_1() {}

#[test]
fn sutra_6_3_2() {
    let mukta = create_krdanta("mukta", &[], &d("mu\\cx~^", Tudadi), Krt::kta);
    let agata = create_krdanta("Agata", &["AN"], &d("ga\\mx~", Bhvadi), Krt::kta);
    assert_has_panchami_tatpurusha("stoka", &mukta, &["stokAnmukta"]);
    assert_has_panchami_tatpurusha("alpa", &mukta, &["alpAnmukta"]);
    assert_has_panchami_tatpurusha("antika", &agata, &["antikAdAgata"]);
    assert_has_panchami_tatpurusha("aByASa", &agata, &["aByASAdAgata"]);
    assert_has_panchami_tatpurusha("dUra", &agata, &["dUrAdAgata"]);
    assert_has_panchami_tatpurusha("viprakfzwa", &agata, &["viprakfzwAdAgata"]);
    assert_has_panchami_tatpurusha("kfcCra", &mukta, &["kfcCrAnmukta"]);
}

#[test]
fn sutra_6_3_3() {
    let krta = create_krdanta("kfta", &[], &d("qukf\\Y", Tanadi), Krt::kta);
    assert_has_trtiya_tatpurusha("ojas", &krta, &["ojasAkfta"]);
    assert_has_trtiya_tatpurusha("sahas", &krta, &["sahasAkfta"]);
    assert_has_trtiya_tatpurusha("amBas", &krta, &["amBasAkfta"]);
    assert_has_trtiya_tatpurusha("tamas", &krta, &["tamasAkfta"]);
}

#[test]
fn sutra_6_3_7() {
    assert_has_caturthi_tatpurusha("Atman", "pada", &["Atmanepada"]);
}

#[test]
fn sutra_6_3_8() {
    assert_has_caturthi_tatpurusha("para", "pada", &["parasmEpada"]);
}

#[test]
fn sutra_6_3_11() {
    assert_has_saptami_tatpurusha("maDya", "guru", &["maDyeguru"]);
}

#[test]
fn sutra_6_3_11_v1() {
    assert_has_saptami_tatpurusha("anta", "guru", &["anteguru"]);
}

#[test]
fn sutra_6_3_15() {
    assert_has_saptami_tatpurusha("prAvfz", "ja", &["prAvfzija"]);
    assert_has_saptami_tatpurusha("Sarad", "ja", &["Saradija"]);
    assert_has_saptami_tatpurusha("kAla", "ja", &["kAleja"]);
    assert_has_saptami_tatpurusha("div", "ja", &["divija"]);
}

#[test]
fn sutra_6_3_16() {
    assert_has_saptami_tatpurusha("varza", "ja", &["varzeja", "varzaja"]);
    assert_has_saptami_tatpurusha("kzara", "ja", &["kzareja", "kzaraja"]);
    assert_has_saptami_tatpurusha("Sara", "ja", &["Sareja", "Saraja"]);
    assert_has_saptami_tatpurusha("vara", "ja", &["vareja", "varaja"]);
}

#[test]
fn sutra_6_3_43() {
    assert_has_taddhita("brAhmaRI", T::tarap, &["brAhmaRitara"]);
    assert_has_taddhita("brAhmaRI", T::tamap, &["brAhmaRitama"]);
    assert_has_taddhita("brAhmaRI", T::rUpap, &["brAhmaRirUpa"]);
    assert_has_taddhita("brAhmaRI", T::kalpap, &["brAhmaRikalpa"]);

    // TODO: others
    // assert_has_taddhitanta("brAhmaRI", T::tamap, &["brAhmaRibruva"]);
    // assert_has_taddhitanta("brAhmaRI", T::tamap, &["brAhmaRigotra"]);
    // assert_has_taddhitanta("brAhmaRI", T::tamap, &["brAhmaRimata"]);
    // assert_has_taddhitanta("brAhmaRI", T::tamap, &["brAhmaRihata"]);
}

#[test]
fn sutra_6_3_46() {
    assert_has_karmadharaya("mahat", "deva", &["mahAdeva"]);
    assert_has_karmadharaya("mahat", "brAhmaRa", &["mahAbrAhmaRa"]);
    assert_has_bahuvrihi("mahat", "bAhu", &["mahAbAhu"]);
    assert_has_bahuvrihi("mahat", "bala", &["mahAbala"]);
    assert_has_taddhita("mahat", T::jAtIyar, &["mahAjAtIya"]);

    // samAnAdhikaraNa?
    assert_has_sasthi_tatpurusha("mahat", "putra", &["mahatputra"]);
}

#[test]
fn sutra_6_3_53() {
    assert_has_artha_taddhita("pAda", TadVidhyati, T::yat, &["padya"]);
    assert_has_artha_taddhita("pAda", Tadarthye, T::yat, &["pAdya"]);
}

#[test]
fn sutra_6_3_67() {
    let tud = d("tu\\da~^", Tudadi);
    let taapi = nic(&d("ta\\pa~", Bhvadi));
    let man = d("ma\\na~\\", Divadi);
    assert_has_upapada_krdanta("arus", &[], &tud, Krt::KaS, &["aruntuda"]);
    assert_has_upapada_krdanta("dvizat", &[], &taapi, Krt::Kac, &["dvizantapa"]);
    assert_has_upapada_krdanta("kAli", &[], &man, Krt::KaS, &["kAlimmanya"]);
}

#[ignore]
#[test]
fn sutra_6_3_68() {
    let man = d("ma\\na~\\", Divadi);
    assert_has_upapada_krdanta("go", &[], &man, Krt::KaS, &["gAmmanya"]);
    assert_has_upapada_krdanta("strI", &[], &man, Krt::KaS, &["strImmanya", "striyammanya"]);
    assert_has_upapada_krdanta("SrI", &[], &man, Krt::KaS, &["Sriyammanya"]);
    assert_has_upapada_krdanta("BrU", &[], &man, Krt::KaS, &["Bruvammanya"]);
    // TODO: others
}

#[test]
fn sutra_6_3_69() {
    let yam = &d("ya\\ma~", Bhvadi);
    assert_has_upapada_krdanta("vAc", &[], &yam, Krt::Kac, &["vAcaMyama"]);
    let daari = nic(&d("dF", Bhvadi));
    assert_has_upapada_krdanta("pur", &[], &daari, Krt::Kac, &["purandara"]);
}

#[test]
fn sutra_6_3_73() {
    assert_has_avyaya_tatpurusha("naY", "brAhmaRa", &["abrAhmaRa"]);
    assert_has_avyaya_tatpurusha("naY", "vfzala", &["avfzala"]);
    assert_has_avyaya_tatpurusha("naY", "somapa", &["asomapa"]);
}

#[test]
fn sutra_6_3_74() {
    assert_has_avyaya_tatpurusha("naY", "aja", &["anaja"]);
    assert_has_avyaya_tatpurusha("naY", "aSva", &["anaSva"]);
}

#[test]
fn sutra_6_3_85() {
    assert_has_bahuvrihi("samAna", "jyotis", &["sajyotis"]);
    assert_has_bahuvrihi("samAna", "janapada", &["sajanapada"]);
    assert_has_bahuvrihi("samAna", "rAtri", &["sarAtri"]);
    assert_has_bahuvrihi("samAna", "nABi", &["sanABi"]);
    assert_has_bahuvrihi("samAna", "nAman", &["sanAman"]);
    assert_has_bahuvrihi("samAna", "gotra", &["sagotra"]);
    assert_has_bahuvrihi("samAna", "rUpa", &["sarUpa"]);
    assert_has_bahuvrihi("samAna", "sTAna", &["sasTAna"]);
    assert_has_bahuvrihi("samAna", "varRa", &["savarRa"]);
    assert_has_bahuvrihi("samAna", "vayas", &["savayas"]);
    assert_has_bahuvrihi("samAna", "vacana", &["savacana"]);
    assert_has_bahuvrihi("samAna", "banDu", &["sabanDu"]);
}

#[test]
fn sutra_6_3_89() {
    let drsh = d("df\\Si~r", Bhvadi);
    assert_has_upapada_krdanta("samAna", &[], &drsh, Krt::kvin, &["sadfS"]);
    assert_has_upapada_krdanta("samAna", &[], &drsh, Krt::kaY, &["sadfSa"]);
}

#[test]
fn sutra_6_3_90() {
    let drsh = d("df\\Si~r", Bhvadi);
    assert_has_upapada_krdanta("idam", &[], &drsh, Krt::kvin, &["IdfS"]);
    assert_has_upapada_krdanta("idam", &[], &drsh, Krt::kaY, &["IdfSa"]);
    assert_has_taddhita("idam", T::vatup, &["iyat"]);

    assert_has_upapada_krdanta("kim", &[], &drsh, Krt::kvin, &["kIdfS"]);
    assert_has_upapada_krdanta("kim", &[], &drsh, Krt::kaY, &["kIdfSa"]);
    assert_has_taddhita("kim", T::vatup, &["kiyat"]);
}

#[test]
fn sutra_6_3_91() {
    let drsh = d("df\\Si~r", Bhvadi);
    assert_has_upapada_krdanta("tad", &[], &drsh, Krt::kvin, &["tAdfS"]);
    assert_has_upapada_krdanta("tad", &[], &drsh, Krt::kaY, &["tAdfSa"]);
    assert_has_taddhita("tad", T::vatup, &["tAvat"]);

    assert_has_upapada_krdanta("yad", &[], &drsh, Krt::kvin, &["yAdfS"]);
    assert_has_upapada_krdanta("yad", &[], &drsh, Krt::kaY, &["yAdfSa"]);
    assert_has_taddhita("yad", T::vatup, &["yAvat"]);
}

#[test]
fn sutra_6_3_101() {
    assert_has_avyaya_tatpurusha("ku", "aja", &["kadaja"]);
    assert_has_avyaya_tatpurusha("ku", "aSva", &["kadaSva"]);
    assert_has_avyaya_tatpurusha("ku", "uzwra", &["kaduzwra"]);
    assert_has_avyaya_tatpurusha("ku", "anna", &["kadanna"]);

    // tatpuruze?
    assert_has_bahuvrihi("ku", "uzwra", &["kUzwra"]);

    // aci?
    assert_has_avyaya_tatpurusha("ku", "brAhmaRa", &["kubrAhmaRa"]);
    assert_has_avyaya_tatpurusha("ku", "puruza", &["kupuruza"]);
}

#[test]
fn sutra_6_3_102() {
    assert_has_avyaya_tatpurusha("ku", "raTa", &["kadraTa"]);
    assert_has_avyaya_tatpurusha("ku", "vada", &["kadvada"]);
}

#[test]
fn sutra_6_3_104() {
    // TODO: how to get kApaTa?
    assert_has_avyaya_tatpurusha("ku", "paTin", &["kApaTin"]);
    assert_has_avyaya_tatpurusha("ku", "akza", &["kAkza"]);
}

#[test]
fn sutra_6_3_111() {
    assert_has_krdanta(&[], &d("li\\ha~^", Adadi), Krt::kta, &["lIQa"]);
    assert_has_krdanta(&[], &d("mi\\ha~", Bhvadi), Krt::kta, &["mIQa"]);
    assert_has_krdanta(&["upa"], &d("guhU~^", Bhvadi), Krt::kta, &["upagUQa"]);
    assert_has_krdanta(&[], &d("mu\\ha~", Divadi), Krt::kta, &["mUQa", "mugDa"]);

    assert_has_sandhi("nis", "raktam", &["nI raktam"]);
    assert_has_sandhi("agnis", "raTaH", &["agnI raTaH"]);
    assert_has_sandhi("indus", "raTas", &["indU raTaH"]);
    assert_has_sandhi("punar", "raktam", &["punA raktam"]);
    assert_has_sandhi("prAtar", "rAjakrayas", &["prAtA rAjakrayaH"]);

    // aRaH?
    assert_has_krdanta(&["AN"], &d("tfhU~", Tudadi), Krt::kta, &["AtfQa"]);
    assert_has_krdanta(&["AN"], &d("vfhU~", Tudadi), Krt::kta, &["AvfQa"]);
}

#[test]
fn sutra_6_3_112() {
    let sah = d("zaha~\\", Bhvadi);
    assert_has_krdanta(&[], &sah, Krt::tfc, &["soQf", "sahitf"]);
    assert_has_krdanta(&[], &sah, Krt::tumun, &["soQum", "sahitum"]);
    assert_has_krdanta(&[], &sah, Krt::tavya, &["soQavya", "sahitavya"]);

    let vah = d("va\\ha~^", Bhvadi);
    assert_has_krdanta(&[], &vah, Krt::tfc, &["voQf"]);
    assert_has_krdanta(&[], &vah, Krt::tumun, &["voQum"]);
    assert_has_krdanta(&[], &vah, Krt::tavya, &["voQavya"]);

    assert_has_krdanta(&[], &vah, Krt::kta, &["UQa"]);
    assert_has_krdanta(&[], &vah, Krt::ktavatu, &["UQavat"]);

    assert_has_tas(&["ud"], &vah, Lun, &["udavoQAm"]);
    assert_has_thas(&["ud"], &vah, Lun, &["udavoQam"]);
}

#[test]
fn sutra_6_3_118() {
    let artha = TadAsyaAstiAsmin;
    assert_has_artha_taddhita("Asuti", artha, T::valac, &["AsutIvala"]);
    assert_has_artha_taddhita("kfzi", artha, T::valac, &["kfzIvala"]);
    assert_has_artha_taddhita("danta", artha, T::valac, &["dantAvala"]);
}

#[test]
fn sutra_6_3_118_v1() {
    let artha = TadAsyaAstiAsmin;
    assert_has_artha_taddhita("utsAha", artha, T::valac, &["utsAhavala"]);
    assert_has_artha_taddhita("BrAtf", artha, T::valac, &["BrAtfvala"]);
    assert_has_artha_taddhita("pitf", artha, T::valac, &["pitfvala"]);
}

#[test]
fn sutra_6_3_123() {
    let kash = d("kASf~", Bhvadi);
    assert_has_krdanta(&["ni"], &kash, Krt::ac, &["nIkASa"]);
    assert_has_krdanta(&["vi"], &kash, Krt::ac, &["vIkASa"]);
    assert_has_krdanta(&["anu"], &kash, Krt::ac, &["anUkASa"]);
    // ikaH?
    assert_has_krdanta(&["pra"], &kash, Krt::ac, &["prakASa"]);
}

#[test]
fn sutra_6_3_124() {
    let daa = d("qudA\\Y", Juhotyadi);
    assert_has_krdanta(&["ni"], &daa, Krt::kta, &["nItta"]);
    assert_has_krdanta(&["vi"], &daa, Krt::kta, &["vItta"]);
    // ikaH?
    assert_has_krdanta(&["pra"], &daa, Krt::kta, &["pratta"]);
    assert_has_krdanta(&["ava"], &daa, Krt::kta, &["avatta"]);
    // daH?
    let tf = d("tF", Bhvadi);
    assert_has_krdanta(&["vi"], &tf, Krt::kta, &["vitIrRa"]);
    assert_has_krdanta(&["ni"], &tf, Krt::kta, &["nitIrRa"]);
    // ti ?
    // TODO: how to best justify sudatta?
}

#[test]
fn sutra_6_3_138() {
    let dadhyac = create_upapada_krdanta("daDyac", "daDi", &[], &d("ancu~", Bhvadi), Krt::kvin);
    assert_has_sup_2p(&dadhyac, Pum, &["daDIcaH"]);
    assert_has_sup_3s(&dadhyac, Pum, &["daDIcA"]);
    assert_has_sup_4s(&dadhyac, Pum, &["daDIce"]);

    let madhvac = create_upapada_krdanta("maDvac", "maDu", &[], &d("ancu~", Bhvadi), Krt::kvin);
    assert_has_sup_2p(&madhvac, Pum, &["maDUcaH"]);
    assert_has_sup_3s(&madhvac, Pum, &["maDUcA"]);
    assert_has_sup_4s(&madhvac, Pum, &["maDUce"]);
}
