extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::*;

fn d(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g)
}

#[test]
fn sutra_3_2_102() {
    let kf = d("qukf\\Y", Tanadi);
    assert_has_krdanta(&[], &kf, Krt::kta, &["kfta"]);
    assert_has_krdanta(&[], &kf, Krt::ktavatu, &["kftavat"]);
    let bhuj = d("Bu\\ja~", Rudhadi);
    assert_has_krdanta(&[], &bhuj, Krt::kta, &["Bukta"]);
    assert_has_krdanta(&[], &bhuj, Krt::ktavatu, &["Buktavat"]);
}

#[test]
fn sutra_3_2_103() {
    assert_has_krdanta(&[], &d("zu\\Y", Svadi), Krt::Nvanip, &["sutvan"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::Nvanip, &["yajvan"]);
}

#[test]
fn sutra_3_2_104() {
    let jf = d("jFz", Divadi);
    assert_has_krdanta(&[], &jf, Krt::atfn, &["jarat"]);
    assert_has_krdanta(&[], &jf, Krt::kta, &["jIrRa"]);
    assert_has_krdanta(&[], &jf, Krt::ktavatu, &["jIrRavat"]);
}

#[test]
fn sutra_3_2_110() {
    assert_has_lun_p(&[], &d("qukf\\Y", Tanadi), &["akArzIt"]);
    assert_has_lun_p(&[], &d("hf\\Y", Bhvadi), &["ahArzIt"]);
}

#[test]
fn sutra_3_2_111() {
    assert_has_lan_p(&[], &d("qukf\\Y", Tanadi), &["akarot"]);
    assert_has_lan_p(&[], &d("hf\\Y", Bhvadi), &["aharat"]);
}

#[test]
fn sutra_3_2_115() {
    assert_has_lit_p(&[], &d("qukf\\Y", Tanadi), &["cakAra"]);
    assert_has_lit_p(&[], &d("hf\\Y", Bhvadi), &["jahAra"]);
}

#[test]
fn sutra_3_2_123() {
    assert_has_lat_p(&[], &d("qupa\\ca~^z", Bhvadi), &["pacati"]);
    assert_has_lat_p(&[], &d("paWa~", Bhvadi), &["paWati"]);
}

#[ignore]
#[test]
fn sutra_3_2_124() {
    let pac = d("qupa\\ca~^z", Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::Satf, &["pacat"]);
    assert_has_krdanta(&[], &pac, Krt::SAnac, &["pacamAna"]);
    // others
    assert_has_krdanta(&[], &d("asa~", Adadi), Krt::Satf, &["sat"]);
    // TODO: more
}

#[ignore]
#[test]
fn sutra_3_2_128() {
    assert_has_krdanta(&[], &d("pUN", Bhvadi), Krt::SAnan, &["pavamAna"]);
    assert_has_krdanta(&[], &d("ya\\ja~^", Bhvadi), Krt::SAnan, &["yajamAna"]);
}

#[test]
fn sutra_3_2_135() {
    assert_has_krdanta(&[], &d("qukf\\Y", Tanadi), Krt::tfn, &["kartf"]);
    assert_has_krdanta(&[], &d("vada~", Bhvadi), Krt::tfn, &["vaditf"]);
}

#[test]
fn sutra_3_2_136() {
    assert_has_krdanta(
        &["alam"],
        &d("qukf\\Y", Tanadi),
        Krt::izRuc,
        &["alaNkarizRu"],
    );
    assert_has_krdanta(
        &["nis", "A"],
        &d("qukf\\Y", Tanadi),
        Krt::izRuc,
        &["nirAkarizRu"],
    );
    assert_has_krdanta(&["pra"], &d("janI~\\", Divadi), Krt::izRuc, &["prajanizRu"]);
    assert_has_krdanta(
        &["ud"],
        &d("qupa\\ca~^z", Bhvadi),
        Krt::izRuc,
        &["utpacizRu"],
    );
    assert_has_krdanta(&["ud"], &d("patx~", Bhvadi), Krt::izRuc, &["utpatizRu"]);
    assert_has_krdanta(&["ud"], &d("madI~", Divadi), Krt::izRuc, &["unmadizRu"]);
    assert_has_krdanta(&[], &d("ruca~\\", Bhvadi), Krt::izRuc, &["rocizRu"]);
    assert_has_krdanta(
        &["apa"],
        &d("trapU~\\z", Bhvadi),
        Krt::izRuc,
        &["apatrapizRu"],
    );
    assert_has_krdanta(&[], &d("vftu~\\", Bhvadi), Krt::izRuc, &["vartizRu"]);
    assert_has_krdanta(&[], &d("vfDu~\\", Bhvadi), Krt::izRuc, &["varDizRu"]);
    assert_has_krdanta(&[], &d("zaha~\\", Bhvadi), Krt::izRuc, &["sahizRu"]);
    assert_has_krdanta(&[], &d("cara~", Bhvadi), Krt::izRuc, &["carizRu"]);
}

#[test]
fn sutra_3_2_140() {
    assert_has_krdanta(&[], &d("trasI~", Divadi), Krt::knu, &["trasnu"]);
    assert_has_krdanta(&[], &d("gfDu~", Divadi), Krt::knu, &["gfDnu"]);
    assert_has_krdanta(&[], &d("YiDfzA~", Svadi), Krt::knu, &["DfzRu"]);
    assert_has_krdanta(&[], &d("kzi\\pa~^", Tudadi), Krt::knu, &["kzipRu"]);
}

#[test]
fn sutra_3_2_141() {
    assert_has_krdanta(&[], &d("Samu~", Divadi), Krt::GinuR, &["Samin"]);
    assert_has_krdanta(&[], &d("tamu~", Divadi), Krt::GinuR, &["tamin"]);
    assert_has_krdanta(&[], &d("damu~", Divadi), Krt::GinuR, &["damin"]);
    assert_has_krdanta(&[], &d("Sramu~", Divadi), Krt::GinuR, &["Sramin"]);
    assert_has_krdanta(&[], &d("Bramu~", Divadi), Krt::GinuR, &["Bramin"]);
    assert_has_krdanta(&[], &d("kzamU~", Divadi), Krt::GinuR, &["kzamin"]);
    assert_has_krdanta(&[], &d("klamu~", Divadi), Krt::GinuR, &["klamin"]);
    assert_has_krdanta(&["pra"], &d("madI~", Divadi), Krt::GinuR, &["pramAdin"]);
    // TODO: unmAdI (conflicts with unmadizRu)
    // assert_has_krdanta(&["ud"], &d("madI~", Divadi), Krt::GinuR, &["unmAdin"]);
    // azwAByaH
    assert_has_krdanta(&[], &d("asu~", Divadi), Krt::GinuR, &[]);
    assert_has_krdanta(&[], &d("asu~", Divadi), Krt::tfn, &["asitf"]);
}

#[test]
fn sutra_3_2_143() {
    assert_has_krdanta(&["vi"], &d("kaza~", Bhvadi), Krt::GinuR, &["vikAzin"]);
    assert_has_krdanta(&["vi"], &d("lasa~", Bhvadi), Krt::GinuR, &["vilAsin"]);
    assert_has_krdanta(&["vi"], &d("katTa~\\", Bhvadi), Krt::GinuR, &["vikatTin"]);
    assert_has_krdanta(&["vi"], &d("sranBu~\\", Bhvadi), Krt::GinuR, &["visramBin"]);
}

#[test]
fn sutra_3_2_144() {
    assert_has_krdanta(&["apa"], &d("laza~^", Bhvadi), Krt::GinuR, &["apalAzin"]);
    assert_has_krdanta(&["vi"], &d("laza~^", Bhvadi), Krt::GinuR, &["vilAzin"]);
}

#[test]
fn sutra_3_2_145() {
    assert_has_krdanta(&["pra"], &d("lapa~", Bhvadi), Krt::GinuR, &["pralApin"]);
    assert_has_krdanta(&["pra"], &d("sf\\", Bhvadi), Krt::GinuR, &["prasArin"]);
    assert_has_krdanta(&["pra"], &d("dru\\", Bhvadi), Krt::GinuR, &["pradrAvin"]);
    assert_has_krdanta(&["pra"], &d("maTe~", Bhvadi), Krt::GinuR, &["pramATin"]);
    assert_has_krdanta(&["pra"], &d("vada~", Bhvadi), Krt::GinuR, &["pravAdin"]);
    assert_has_krdanta(&["pra"], &d("va\\sa~", Bhvadi), Krt::GinuR, &["pravAsin"]);
    // vas-bhvAdi only
    assert_has_krdanta(&["pra"], &d("vasa~\\", Adadi), Krt::GinuR, &[]);
}

#[test]
fn sutra_3_2_165() {
    assert_has_krdanta(&[], &d("jAgf", Adadi), Krt::Uka, &["jAgarUka"]);
}

#[ignore]
#[test]
fn sutra_3_2_167() {
    assert_has_krdanta(&[], &d("Ra\\ma~", Bhvadi), Krt::ra, &["namra"]);
    assert_has_krdanta(&[], &d("kapi~\\", Bhvadi), Krt::ra, &["kampra"]);
    assert_has_krdanta(&[], &d("zmi\\N", Bhvadi), Krt::ra, &["smera"]);
    // TODO: ajasra, hiMsra
    assert_has_krdanta(&[], &d("kamu~\\", Bhvadi), Krt::ra, &["kamra"]);
    assert_has_krdanta(&[], &d("dIpI~\\", Divadi), Krt::ra, &["dIpra"]);
}

#[ignore]
#[test]
fn sutra_3_2_175() {
    assert_has_krdanta(&[], &d("zWA\\", Bhvadi), Krt::varac, &["sTAvara"]);
    assert_has_krdanta(&[], &d("ISa~\\", Adadi), Krt::varac, &["ISvara"]);
    assert_has_krdanta(&[], &d("BAsf~\\", Bhvadi), Krt::varac, &["BAsvara"]);
    assert_has_krdanta(&[], &d("pisf~", Bhvadi), Krt::varac, &["pesvara"]);
    assert_has_krdanta(&[], &d("kasa~", Bhvadi), Krt::varac, &["kasvara"]);
}

#[test]
fn sutra_3_2_187() {
    let d = Dhatu::new;
    assert_has_krdanta(&[], &d("YimidA~\\", Bhvadi), Krt::kta, &["minna"]);
    assert_has_krdanta(&[], &d("YikzvidA~", Divadi), Krt::kta, &["kzviRRa"]);
    assert_has_krdanta(&[], &d("YiDfzA~", Svadi), Krt::kta, &["Dfzwa"]);
}
