extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::TaddhitaArtha::*;

#[test]
fn sutra_6_3_43() {
    assert_has_taddhitanta(&prati("brAhmaRI"), T::tarap, &["brAhmaRitara"]);
    assert_has_taddhitanta(&prati("brAhmaRI"), T::tamap, &["brAhmaRitama"]);
    assert_has_taddhitanta(&prati("brAhmaRI"), T::rUpap, &["brAhmaRirUpa"]);
    assert_has_taddhitanta(&prati("brAhmaRI"), T::kalpap, &["brAhmaRikalpa"]);

    // TODO: others
    // assert_has_taddhitanta(&prati("brAhmaRI"), T::tamap, &["brAhmaRibruva"]);
    // assert_has_taddhitanta(&prati("brAhmaRI"), T::tamap, &["brAhmaRigotra"]);
    // assert_has_taddhitanta(&prati("brAhmaRI"), T::tamap, &["brAhmaRimata"]);
    // assert_has_taddhitanta(&prati("brAhmaRI"), T::tamap, &["brAhmaRihata"]);
}

#[test]
fn sutra_6_3_46() {
    assert_has_karmadharaya("mahat", "deva", &["mahAdeva"]);
    assert_has_karmadharaya("mahat", "brAhmaRa", &["mahAbrAhmaRa"]);
    assert_has_bahuvrihi("mahat", "bAhu", &["mahAbAhu"]);
    assert_has_bahuvrihi("mahat", "bala", &["mahAbala"]);
    assert_has_taddhitanta(&prati("mahat"), T::jAtIyar, &["mahAjAtIya"]);

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
