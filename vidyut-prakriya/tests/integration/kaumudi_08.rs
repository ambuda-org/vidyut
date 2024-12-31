extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::BaseKrt as Krt;
use vidyut_prakriya::args::Gana::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Pratipadika;
use vidyut_prakriya::args::Slp1String;
use vidyut_prakriya::args::Taddhita as T;
use vidyut_prakriya::args::Unadi;
use vidyut_prakriya::args::Vibhakti;
use vidyut_prakriya::args::{Dhatu, Sanadi};

fn phit(s: &str) -> Pratipadika {
    Pratipadika::basic(Slp1String::from(s).expect("ok"))
}

#[test]
fn skip_sk_178_to_sk_186() {}

#[test]
fn sk_187() {
    assert_has_sup_1s("rAma", Pum, &["rAmaH"]);
}

#[test]
fn sk_188() {
    assert_has_sup_1d("rAma", Pum, &["rAmO"]);
}

#[test]
fn skip_sk_189_to_sk_190() {}

#[test]
fn sk_191() {
    assert_has_sup_1p("rAma", Pum, &["rAmAH"]);
}

#[test]
fn skip_sk_192() {}

#[test]
fn sk_193() {
    assert_has_sup_ss("rAma", Pum, &["rAma"]);
    assert_has_sup_sd("rAma", Pum, &["rAmO"]);
    assert_has_sup_sp("rAma", Pum, &["rAmAH"]);
    assert_has_sup_ss("hari", Pum, &["hare"]);
    assert_has_sup_ss("vizRu", Pum, &["vizRo"]);
}

#[test]
fn sk_194() {
    assert_has_sup_2s("rAma", Pum, &["rAmam"]);
    assert_has_sup_2d("rAma", Pum, &["rAmO"]);
}

#[test]
fn skip_sk_195_to_sk_197() {}

#[test]
fn sk_198() {
    assert_has_sup_2p("rAma", Pum, &["rAmAn"]);
}

#[test]
fn skip_sk_199_to_sk_200() {}

#[test]
fn sk_201() {
    assert_has_sup_3s("rAma", Pum, &["rAmeRa"]);
}

#[test]
fn sk_202() {
    assert_has_sup_3d("rAma", Pum, &["rAmAByAm"]);
}

#[test]
fn sk_203() {
    assert_has_sup_3p("rAma", Pum, &["rAmEH"]);
}

#[test]
fn sk_204() {
    assert_has_sup_4s("rAma", Pum, &["rAmAya"]);
    assert_has_sup_4d("rAma", Pum, &["rAmAByAm"]);
}

#[test]
fn sk_205() {
    assert_has_sup_4p("rAma", Pum, &["rAmeByaH"]);
}

#[test]
fn sk_206() {
    assert_has_sup_5s("rAma", Pum, &["rAmAt"]);
    assert_has_sup_5d("rAma", Pum, &["rAmAByAm"]);
    assert_has_sup_5p("rAma", Pum, &["rAmeByaH"]);
    assert_has_sup_6s("rAma", Pum, &["rAmasya"]);
}

#[test]
fn sk_207() {
    assert_has_sup_6d("rAma", Pum, &["rAmayoH"]);
}

#[test]
fn skip_sk_208() {}

#[test]
fn sk_209() {
    assert_has_sup_6p("rAma", Pum, &["rAmARAm"]);
    assert_has_sup_7s("rAma", Pum, &["rAme"]);
    assert_has_sup_7d("rAma", Pum, &["rAmayoH"]);
}

#[test]
fn skip_sk_210_to_sk_211() {}

#[test]
fn sk_212() {
    assert_has_sup_7p("rAma", Pum, &["rAmezu"]);
    assert_has_sup_6s("rAma", Pum, &["rAmasya"]);

    let pis = create_krdanta("pis", &[], &d("pisf~", Bhvadi), Krt::kvip);
    let supis = bahuvrihi("su", pis);
    assert_has_sup_1s(&supis, Pum, &["supIH"]);
    assert_has_sup_1d(&supis, Pum, &["supisO"]);
    assert_has_sup_1p(&supis, Pum, &["supisaH"]);
}

#[test]
fn skip_sk_213() {}

#[test]
fn sk_214() {
    assert_has_sup_1p("sarva", Pum, &["sarve"]);
}

#[test]
fn sk_215() {
    assert_has_sup_4s("sarva", Pum, &["sarvasmE"]);
}

#[test]
fn sk_216() {
    assert_has_sup_5s("sarva", Pum, &["sarvasmAt"]);
}

#[test]
fn sk_217() {
    assert_has_sup_6p("sarva", Pum, &["sarvezAm"]);
    assert_has_sup_7s("sarva", Pum, &["sarvasmin"]);
}

#[test]
fn sk_218() {
    assert_has_sup_1p("pUrva", Pum, &["pUrve", "pUrvAH"]);
    assert_has_sup_1p("dakziRa", Pum, &["dakziRe", "dakziRAH"]);
    assert_has_sup_1p("uttara", Pum, &["uttare", "uttarAH"]);
}

#[test]
fn sk_219() {
    assert_has_sup_1p("sva", Pum, &["sve", "svAH"]);
}

#[test]
fn sk_220() {
    assert_has_sup_1p("antara", Pum, &["antare", "antarAH"]);
}

#[test]
fn sk_221() {
    assert_has_sup_5s("pUrva", Pum, &["pUrvasmAt", "pUrvAt"]);
    assert_has_sup_7s("pUrva", Pum, &["pUrvasmin", "pUrve"]);
}

#[test]
fn sk_226() {
    assert_has_sup_1p("praTama", Pum, &["praTame", "praTamAH"]);
    assert_has_sup_1p("dvitaya", Pum, &["dvitaye", "dvitayAH"]);
    assert_has_sup_1p("nema", Pum, &["neme", "nemAH"]);
    assert_has_sup_4s("dvitIya", Pum, &["dvitIyasmE", "dvitIyAya"]);
}

#[test]
fn sk_228() {
    assert_has_sup_1s("pAda", Pum, &["pAdaH"]);
    assert_has_sup_1d("pAda", Pum, &["pAdO"]);
    assert_has_sup_1p("pAda", Pum, &["pAdAH"]);
    assert_has_sup_2s("pAda", Pum, &["pAdam"]);
    assert_has_sup_2d("pAda", Pum, &["pAdO"]);
    assert_has_sup_2p("pAda", Pum, &["padaH", "pAdAn"]);
    assert_has_sup_3s("pAda", Pum, &["padA", "pAdena"]);
}

#[test]
fn skip_sk_229_to_sk_231() {}

#[test]
fn sk_232() {
    assert_has_sup_2p("danta", Pum, &["dataH", "dantAn"]);
    assert_has_sup_3s("danta", Pum, &["datA", "dantena"]);
    assert_has_sup_3d("danta", Pum, &["dadByAm", "dantAByAm"]);

    assert_has_sup_2p("mAsa", Pum, &["mAsaH", "mAsAn"]);
    assert_has_sup_3s("mAsa", Pum, &["mAsA", "mAsena"]);
    assert_has_sup_3d("mAsa", Pum, &["mAByAm", "mAsAByAm"]);
    assert_has_sup_3p("mAsa", Pum, &["mABiH", "mAsEH"]);
}

#[test]
fn skip_sk_233_to_sk_234() {}

#[test]
fn sk_235() {
    assert_has_sup_2p("yUzan", Pum, &["yUzRaH"]);
    assert_has_sup_3s("yUzan", Pum, &["yUzRA"]);
}

#[test]
fn sk_236() {
    assert_has_sup_3d("yUzan", Pum, &["yUzaByAm"]);
    assert_has_sup_3p("yUzan", Pum, &["yUzaBiH"]);
    assert_has_sup_4p("yUzan", Pum, &["yUzaByaH"]);
}

#[test]
fn sk_237() {
    assert_has_sup_7s("yUzan", Pum, &["yUzRi", "yUzaRi"]);
}

#[ignore]
#[test]
fn sk_238() {
    let dvyahan = tatpurusha("dvi", "ahan", Vibhakti::Prathama);
    assert_has_sup_7s(&dvyahan, Pum, &["dvyahni", "dvyahani", "dvyahne"]);

    let vyahan = tatpurusha("vi", "ahan", Vibhakti::Prathama);
    assert_has_sup_7s(&vyahan, Pum, &["vyahni", "vyahani", "vyahne"]);

    let sayahan = tatpurusha("sAya", "ahan", Vibhakti::Prathama);
    assert_has_sup_7s(&sayahan, Pum, &["sAyAhni", "sAyAhani", "sAyahne"]);

    let vishvapa = create_upapada_krdanta("viSvapA", "viSva", &[], &d("pA\\", Adadi), Krt::vic);
    assert_has_sup_1s(&vishvapa, Pum, &["viSvapO"]);
}

#[test]
fn sk_239() {
    let vishvapa = create_upapada_krdanta("viSvapA", "viSva", &[], &d("pA\\", Adadi), Krt::vic);
    assert_has_sup_1d(&vishvapa, Pum, &["viSvapO"]);
    assert_has_sup_1p(&vishvapa, Pum, &["viSvapAH"]);
}

#[test]
fn sk_240() {
    let vishvapa = create_upapada_krdanta("viSvapA", "viSva", &[], &d("pA\\", Adadi), Krt::vic);
    assert_has_sup_2p(&vishvapa, Pum, &["viSvapaH"]);

    assert_has_sup_2p("hAhA", Pum, &["hAhAn"]);
    assert_has_sup_3s("hAhA", Pum, &["hAhA"]);
    assert_has_sup_4s("hAhA", Pum, &["hAhE"]);
    assert_has_sup_5s("hAhA", Pum, &["hAhAH"]);
    assert_has_sup_6d("hAhA", Pum, &["hAhOH"]);
    assert_has_sup_7s("hAhA", Pum, &["hAhe"]);

    assert_has_sup_1s("hari", Pum, &["hariH"]);
    assert_has_sup_1d("hari", Pum, &["harI"]);
}

#[test]
fn sk_241() {
    assert_has_sup_1p("hari", Pum, &["harayaH"]);
}

#[test]
fn sk_242() {
    assert_has_sup_ss("hari", Pum, &["hare"]);
    assert_has_sup_2s("hari", Pum, &["harim"]);
    assert_has_sup_2d("hari", Pum, &["harI"]);
    assert_has_sup_2p("hari", Pum, &["harIn"]);
}

#[test]
fn sk_243() {
    let vataprami = create_upapada_krdanta(
        "vAtapramI",
        "vAta",
        &["pra"],
        &d("mA\\N", Juhotyadi),
        Krt::kvip,
    );
    assert_has_sup_4s("mati", Stri, &["mataye", "matyE"]);
    assert_has_sup_4s(&vataprami, Pum, &["vAtapramye"]);
    assert_has_sup_4s("mAtf", Stri, &["mAtre"]);
}

#[test]
fn sk_244() {
    assert_has_sup_3s("hari", Pum, &["hariRA"]);
    assert_has_sup_3s("mati", Stri, &["matyA"]);
}

#[test]
fn sk_245() {
    assert_has_sup_4s("hari", Pum, &["haraye"]);
    assert_has_sup_4s("saKi", Pum, &["saKye"]);
    assert_has_sup_4d("hari", Pum, &["hariByAm"]);
}

#[test]
fn sk_246() {
    assert_has_sup_6s("hari", Pum, &["hareH"]);
    assert_has_sup_6d("hari", Pum, &["haryoH"]);
    assert_has_sup_6p("hari", Pum, &["harIRAm"]);
}

#[test]
fn sk_247() {
    assert_has_sup_7s("hari", Pum, &["harO"]);
    assert_has_sup_7d("hari", Pum, &["haryoH"]);
    assert_has_sup_7p("hari", Pum, &["harizu"]);
}

#[test]
fn skip_sk_248_to_sk_251() {}

#[test]
fn sk_252() {
    // counterexamples
    assert_has_sup_1s("grAmaRI", Pum, &["grAmaRIH"]);
    assert_has_sup_1s("nizkOSAmbi", Pum, &["nizkOSAmbiH"]);
    assert_has_sup_1s("atiKawva", Pum, &["atiKawvaH"]);
    assert_has_tip(&[], &d("Bi\\di~^r", Rudhadi), Lun, &["aBEtsIt", "aBidat"]);
    assert_has_tip(&[], &d("quBf\\Y", Juhotyadi), Lat, &["biBarti"]);
    assert_has_tip(&[], &d("Bi\\di~^r", Rudhadi), Lit, &["biBeda"]);
    assert_has_sup_1s("rAjan", Pum, &["rAjA"]);

    assert_has_sup_1s("saKi", Pum, &["saKA"]);
    assert_has_sup_ss("saKi", Pum, &["saKe"]);
}

#[test]
fn skip_sk_253() {}

#[test]
fn sk_254() {
    assert_has_sup_1d("saKi", Pum, &["saKAyO"]);
    assert_has_sup_1p("saKi", Pum, &["saKAyaH"]);
    assert_has_sup_2s("saKi", Pum, &["saKAyam"]);
    assert_has_sup_2d("saKi", Pum, &["saKAyO"]);
    assert_has_sup_3s("saKi", Pum, &["saKyA"]);
    assert_has_sup_4s("saKi", Pum, &["saKye"]);
}

#[test]
fn sk_255() {
    assert_has_sup_5s("saKi", Pum, &["saKyuH"]);
}

#[ignore]
#[test]
fn sk_256() {
    assert_has_sup_7s("saKi", Pum, &["saKyO"]);

    let susakhi = bahuvrihi("su", "saKi");
    assert_has_sup_1s(&susakhi, Pum, &["susaKA"]);
    assert_has_sup_1d(&susakhi, Pum, &["susaKAyO"]);
    assert_has_sup_1p(&susakhi, Pum, &["susaKAyaH"]);
    assert_has_sup_3s(&susakhi, Pum, &["susaKinA"]);
    assert_has_sup_4s(&susakhi, Pum, &["susaKaye"]);
    assert_has_sup_5s(&susakhi, Pum, &["susaKeH"]);
    assert_has_sup_7s(&susakhi, Pum, &["susaKO"]);

    let atisakhi = bahuvrihi("ati", "saKi");
    assert_has_sup_1s(&atisakhi, Pum, &["atisaKA"]);

    let paramasakhi = bahuvrihi("parama", "saKi");
    assert_has_sup_1s(&paramasakhi, Pum, &["paramasaKA"]);
    assert_has_sup_1d(&paramasakhi, Pum, &["paramasaKAyO"]);
}

#[test]
fn skip_sk_257() {
    assert_has_sup_3s("pati", Pum, &["patyA"]);
    assert_has_sup_4s("pati", Pum, &["patye"]);
    assert_has_sup_5s("pati", Pum, &["patyuH"]);
    assert_has_sup_6s("pati", Pum, &["patyuH"]);
    assert_has_sup_7s("pati", Pum, &["patyO"]);

    let bhupati = tatpurusha("BU", "pati", Vibhakti::Sasthi);
    assert_has_sup_3s(&bhupati, Pum, &["BUpatinA"]);
    assert_has_sup_4s(&bhupati, Pum, &["BUpataye"]);
}

#[test]
fn skip_sk_258_to_sk_262() {}

#[test]
fn sk_263() {
    assert_has_sup_1p("kati", Pum, &["kati"]);
    assert_has_sup_2p("kati", Pum, &["kati"]);
    assert_has_sup_3p("kati", Pum, &["katiBiH"]);
    assert_has_sup_4p("kati", Pum, &["katiByaH"]);
    assert_has_sup_5p("kati", Pum, &["katiByaH"]);
    assert_has_sup_6p("kati", Pum, &["katInAm"]);
    assert_has_sup_7p("kati", Pum, &["katizu"]);

    assert_has_sup_1p("tri", Pum, &["trayaH"]);
    assert_has_sup_2p("tri", Pum, &["trIn"]);
    assert_has_sup_3p("tri", Pum, &["triBiH"]);
    assert_has_sup_4p("tri", Pum, &["triByaH"]);
}

#[test]
fn sk_264() {
    assert_has_sup_6p("tri", Pum, &["trayARAm"]);
    // TODO: paramatri, etc.
    assert_has_sup_7p("tri", Pum, &["trizu"]);
}

#[ignore]
#[test]
fn sk_265() {
    assert_has_sup_1d("dvi", Pum, &["dvO"]);
    assert_has_sup_2d("dvi", Pum, &["dvO"]);
    assert_has_sup_3d("dvi", Pum, &["dvAByAm"]);
    assert_has_sup_4d("dvi", Pum, &["dvAByAm"]);
    assert_has_sup_5d("dvi", Pum, &["dvAByAm"]);
    assert_has_sup_6d("dvi", Pum, &["dvayoH"]);
    assert_has_sup_7d("dvi", Pum, &["dvayoH"]);

    let bhavat = create_krdanta("Bavat", &[], &d("BA\\", Adadi), Unadi::qavatu);
    assert_has_sup_1s(&bhavat, Pum, &["BavAn"]);
    assert_has_sup_1d(&bhavat, Pum, &["BavantO"]);
    assert_has_sup_1p(&bhavat, Pum, &["BavantaH"]);

    // TODO: dviH, dvI
    let audulomi = taddhitanta("uquloman", T::iY);
    assert_has_sup_1s(&audulomi, Pum, &["OqulomiH"]);
    assert_has_sup_1d(&audulomi, Pum, &["OqulomI"]);
    // TODO: others
    assert_has_sup_2s(&audulomi, Pum, &["Oqulomim"]);
    assert_has_sup_2d(&audulomi, Pum, &["OqulomI"]);

    let vataprami = create_upapada_krdanta(
        "vAtapramI",
        "vAta",
        &["pra"],
        &d("mA\\N", Juhotyadi),
        Krt::kvip,
    );
    assert_has_sup_1s(&vataprami, Pum, &["vAtapramIH"]);
    assert_has_sup_1d(&vataprami, Pum, &["vAtapramyO"]);
    assert_has_sup_1p(&vataprami, Pum, &["vAtapramyaH"]);
    assert_has_sup_ss(&vataprami, Pum, &["vAtapramIH"]);
    assert_has_sup_2s(&vataprami, Pum, &["vAtapramIm"]);
    assert_has_sup_2d(&vataprami, Pum, &["vAtapramyO"]);
    assert_has_sup_2p(&vataprami, Pum, &["vAtapramIn"]);
    assert_has_sup_3s(&vataprami, Pum, &["vAtapramyA"]);
    assert_has_sup_3d(&vataprami, Pum, &["vAtapramIByAm"]);
    assert_has_sup_4s(&vataprami, Pum, &["vAtapramye"]);
    assert_has_sup_5s(&vataprami, Pum, &["vAtapramyaH"]);
    assert_has_sup_6s(&vataprami, Pum, &["vAtapramyaH"]);
    assert_has_sup_6d(&vataprami, Pum, &["vAtapramyoH"]);
    assert_has_sup_7d(&vataprami, Pum, &["vAtapramyoH"]);
    assert_has_sup_7d(&vataprami, Pum, &["vAtapramyAm"]);
    assert_has_sup_7p(&vataprami, Pum, &["vAtapramIzu"]);
}

#[test]
fn skip_sk_266() {}

#[test]
fn sk_267() {
    assert_has_sup_ss("bahuSreyasI", Pum, &["bahuSreyasi"]);
    assert_has_sup_2p("bahuSreyasI", Pum, &["bahuSreyasIn"]);
}

#[test]
fn skip_sk_268() {}

#[test]
fn sk_269() {
    assert_has_sup_4s("bahuSreyasI", Pum, &["bahuSreyasyE"]);
    assert_has_sup_6s("bahuSreyasI", Pum, &["bahuSreyasyAH"]);
    assert_has_sup_6p("bahuSreyasI", Pum, &["bahuSreyasInAm"]);
}

#[test]
fn sk_270() {
    assert_has_sup_7s("bahuSreyasI", Pum, &["bahuSreyasyAm"]);
    assert_has_sup_1s("atilakzmI", Pum, &["atilakzmIH"]);

    let kumari = nyap("kumArI");
    let kumariya = Dhatu::nama(kumari, Some(Sanadi::kyac));
    let kumari = krdanta(&[], &kumariya, Krt::kvip);
    assert_has_sup_1s(&kumari, Pum, &["kumArI"]);
}

#[test]
fn skip_sk_271() {}

#[ignore]
#[test]
fn sk_272() {
    let kumari = nyap("kumArI");
    let kumariya = Dhatu::nama(kumari, Some(Sanadi::kyac));
    let kumari = krdanta(&[], &kumariya, Krt::kvip);

    assert_has_sup_1d(&kumari, Pum, &["kumAryO"]);
    assert_has_sup_1p(&kumari, Pum, &["kumAryaH"]);
    assert_has_sup_ss(&kumari, Pum, &["kumAri"]);
    assert_has_sup_2s(&kumari, Pum, &["kumAryam"]);
    assert_has_sup_2p(&kumari, Pum, &["kumAryaH"]);
    assert_has_sup_4s(&kumari, Pum, &["kumAryE"]);
    assert_has_sup_5s(&kumari, Pum, &["kumAryAH"]);
    assert_has_sup_6s(&kumari, Pum, &["kumAryAH"]);
    assert_has_sup_6p(&kumari, Pum, &["kumArIRAm"]);
    assert_has_sup_7s(&kumari, Pum, &["kumAryAm"]);
}

#[ignore]
#[test]
fn sk_273() {
    let sakhiy = Dhatu::nama(phit("saKi"), Some(Sanadi::kyac));
    let sakhi = krdanta(&[], &sakhiy, Krt::kvip);
    assert_has_sup_1s(&sakhi, Pum, &["saKA"]);
    assert_has_sup_1d(&sakhi, Pum, &["saKAyO"]);
    assert_has_sup_1p(&sakhi, Pum, &["saKAyaH"]);
}

#[test]
fn skip_sk_274_to_sk_276() {}

#[test]
fn sk_277() {
    assert_has_sup_1s("krozwu", Pum, &["krozwA"]);
    assert_has_sup_1d("krozwu", Pum, &["krozwArO"]);
    assert_has_sup_1p("krozwu", Pum, &["krozwAraH"]);
    assert_has_sup_2s("krozwu", Pum, &["krozwAram"]);
    assert_has_sup_2d("krozwu", Pum, &["krozwArO"]);
    assert_has_sup_2p("krozwu", Pum, &["krozwUn"]);
}

#[test]
fn sk_278() {
    assert_has_sup_3s("krozwu", Pum, &["krozwrA", "krozwunA"]);
    assert_has_sup_4s("krozwu", Pum, &["krozwre", "krozwave"]);
}

#[test]
fn skip_sk_279() {}

#[test]
fn sk_280() {
    assert_has_sup_6s("krozwu", Pum, &["krozwuH", "krozwoH"]);
    assert_has_sup_6p("krozwu", Pum, &["krozwUnAm"]);
    assert_has_sup_7s("krozwu", Pum, &["krozwari", "krozwO"]);
    assert_has_sup_7d("krozwu", Pum, &["krozwroH", "krozwvoH"]);

    assert_has_sup_1s("hUhU", Pum, &["hUhUH"]);
    assert_has_sup_1d("hUhU", Pum, &["hUhvO"]);
    assert_has_sup_1p("hUhU", Pum, &["hUhvaH"]);
    assert_has_sup_2s("hUhU", Pum, &["hUhUm"]);
    assert_has_sup_2d("hUhU", Pum, &["hUhvO"]);
    assert_has_sup_2p("hUhU", Pum, &["hUhUn"]);

    assert_has_sup_ss("aticamU", Pum, &["aticamu"]);
    assert_has_sup_4s("aticamU", Pum, &["aticamvE"]);
    assert_has_sup_5s("aticamU", Pum, &["aticamvAH"]);
    assert_has_sup_6s("aticamU", Pum, &["aticamvAH"]);
    assert_has_sup_6p("aticamU", Pum, &["aticamUnAm"]);
    assert_has_sup_7s("aticamU", Pum, &["aticamvAm"]);

    let khalapu = upapada_krdanta("Kala", &[], &d("pUY", Kryadi), Krt::kvip);
    assert_has_sup_1s(&khalapu, Pum, &["KalapUH"]);
}

#[ignore]
#[test]
fn sk_281() {
    let khalapu = upapada_krdanta("Kala", &[], &d("pUY", Kryadi), Krt::kvip);
    assert_has_sup_1d(&khalapu, Pum, &["KalapvO"]);
    assert_has_sup_1p(&khalapu, Pum, &["KalapvaH"]);

    let lu = krdanta(&[], &d("lUY", Kryadi), Krt::kvip);
    assert_has_sup_1s(&lu, Pum, &["lUH"]);
    assert_has_sup_1d(&lu, Pum, &["luvO"]);
    assert_has_sup_1p(&lu, Pum, &["luvaH"]);
}

#[test]
fn skip_sk_284() {
    assert_has_sup_1s("go", Pum, &["gOH"]);
    assert_has_sup_1d("go", Pum, &["gAvO"]);
    assert_has_sup_1p("go", Pum, &["gAvaH"]);
}

#[test]
fn sk_285() {
    // Counterexamples
    assert_has_mip(&[], &d("ci\\Y", Svadi), Lan, &["acinavam"]);
    assert_has_mip(&[], &d("zu\\Y", Svadi), Lan, &["asunavam"]);

    assert_has_sup_2s("go", Pum, &["gAm"]);
    assert_has_sup_2d("go", Pum, &["gAvO"]);
    assert_has_sup_2p("go", Pum, &["gAH"]);
    assert_has_sup_4s("go", Pum, &["gave"]);
    assert_has_sup_5s("go", Pum, &["goH"]);
    assert_has_sup_6s("go", Pum, &["goH"]);

    assert_has_sup_1s("sudyo", Pum, &["sudyOH"]);
    assert_has_sup_1d("sudyo", Pum, &["sudyAvO"]);
    assert_has_sup_1p("sudyo", Pum, &["sudyAvaH"]);

    assert_has_sup_ss("BAnu", Pum, &["BAno"]);
    assert_has_sup_1p("BAnu", Pum, &["BAnavaH"]);

    assert_has_sup_1s("smfto", Pum, &["smftOH"]);
    assert_has_sup_1d("smfto", Pum, &["smftAvO"]);
    assert_has_sup_1p("smfto", Pum, &["smftAvaH"]);
    assert_has_sup_2s("smfto", Pum, &["smftAm"]);
    assert_has_sup_2d("smfto", Pum, &["smftAvO"]);
    assert_has_sup_2p("smfto", Pum, &["smftAH"]);
}

#[test]
fn sk_286() {
    assert_has_sup_1s("rE", Pum, &["rAH"]);
    assert_has_sup_1p("rE", Pum, &["rAyaH"]);
    assert_has_sup_2s("rE", Pum, &["rAyam"]);
    assert_has_sup_2d("rE", Pum, &["rAyO"]);
    assert_has_sup_2p("rE", Pum, &["rAyaH"]);
    assert_has_sup_3s("rE", Pum, &["rAyA"]);

    assert_has_sup_1s("glO", Pum, &["glOH"]);
    assert_has_sup_1d("glO", Pum, &["glAvO"]);
    assert_has_sup_1p("glO", Pum, &["glAvaH"]);
    assert_has_sup_2s("glO", Pum, &["glAvam"]);
    assert_has_sup_2d("glO", Pum, &["glAvO"]);
    assert_has_sup_2p("glO", Pum, &["glAvaH"]);
}
