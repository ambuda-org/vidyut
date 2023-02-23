extern crate test_utils;
use test_utils::*;
use vidyut_prakriya::args::Lakara::*;
use vidyut_prakriya::args::Linga::*;
use vidyut_prakriya::args::Purusha::*;
use vidyut_prakriya::args::Vacana::*;
use vidyut_prakriya::args::Vibhakti as V;
use vidyut_prakriya::args::*;

fn nic(u: &str, g: Gana) -> Dhatu {
    Dhatu::new(u, g).with_sanadi(&[Sanadi::Nic])
}

fn assert_has_lat_p_3d(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, Lat, Prathama, Dvi, expected)
}

fn assert_has_lan_p_1s(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, Lan, Uttama, Eka, expected)
}

fn assert_has_lot_p_1s(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, Lot, Uttama, Eka, expected)
}

fn assert_has_lan_p_3p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, Lan, Prathama, Bahu, expected)
}

fn assert_has_vidhilin_p_3p(prefixes: &[&str], dhatu: &Dhatu, expected: &[&str]) {
    assert_has_parasmai_tinanta(prefixes, dhatu, VidhiLin, Prathama, Bahu, expected)
}

#[test]
fn sutra_7_3_36() {
    assert_has_lat_p(&[], &nic("f\\", Gana::Bhvadi), &["arpayati"]);
    assert_has_lat_p(&[], &nic("f\\", Gana::Juhotyadi), &["arpayati"]);
    assert_has_lat_p(&[], &nic("hrI\\", Gana::Juhotyadi), &["hrepayati"]);
    assert_has_lat_p(&[], &nic("vlI\\", Gana::Kryadi), &["vlepayati"]);
    assert_has_lat_p(&[], &nic("rI\\N", Gana::Divadi), &["repayati"]);
    assert_has_lat_p(&[], &nic("rI\\", Gana::Kryadi), &["repayati"]);
    assert_has_lat_p(&[], &nic("knUyI~\\", Gana::Bhvadi), &["knopayati"]);
    assert_has_lat_p(&[], &nic("kzmAyI~\\", Gana::Bhvadi), &["kzmApayati"]);
    assert_has_lat_p(&[], &nic("qudA\\Y", Gana::Bhvadi), &["dApayati"]);
    assert_has_lat_p(&[], &nic("quDA\\Y", Gana::Bhvadi), &["DApayati"]);
}

#[test]
fn sutra_7_3_37() {
    assert_has_lat_p(&[], &nic("So\\", Gana::Divadi), &["SAyayati"]);
    assert_has_lat_p(&["ava"], &nic("Co\\", Gana::Divadi), &["avacCAyayati"]);
    assert_has_lat_p(&["ava"], &nic("zE\\", Gana::Bhvadi), &["avasAyayati"]);
    assert_has_lat_p(&[], &nic("hve\\Y", Gana::Kryadi), &["hvAyayati"]);
    assert_has_lat_p(&["sam"], &nic("vye\\Y", Gana::Divadi), &["saMvyAyayati"]);
    assert_has_lat_p(&[], &nic("pA\\", Gana::Bhvadi), &["pAyayati"]);
    assert_has_lat_p(&[], &nic("pE\\", Gana::Bhvadi), &["pAyayati"]);
}

#[test]
fn sutra_7_3_37_v1() {
    assert_has_lat_p(&[], &nic("pA\\", Gana::Adadi), &["pAlayati"]);
}

#[test]
fn sutra_7_3_37_v2() {
    assert_has_lat_p(&[], &nic("DUY", Gana::Svadi), &["DUnayati", "DAvayati"]);
    assert_has_lat_p(
        &[],
        &nic("prI\\Y", Gana::Kryadi),
        &["prIRayati", "prAyayati"],
    );
}

#[test]
fn sutra_7_3_38() {
    assert_has_lat_p(&[], &nic("vA\\", Gana::Adadi), &["vAjayati", "vApayati"]);
}

#[test]
fn sutra_7_3_39() {
    assert_has_lat_p(
        &["vi"],
        &nic("lI\\N", Gana::Divadi),
        &["vilInayati", "vilAyayati", "vilAlayati", "vilApayati"],
    );
    assert_has_lat_p(
        &["vi"],
        &nic("lI\\", Gana::Kryadi),
        &["vilInayati", "vilAyayati", "vilAlayati", "vilApayati"],
    );
    assert_has_lat_p(
        &["vi"],
        &nic("lA\\", Gana::Adadi),
        &["vilAlayati", "vilApayati"],
    );
}

#[test]
fn sutra_7_3_41() {
    assert_has_lat_p(&[], &nic("sPAyI~\\", Gana::Bhvadi), &["sPAvayati"]);
}

#[test]
fn sutra_7_3_42() {
    assert_has_lat_p(
        &[],
        &nic("Sa\\dx~", Gana::Bhvadi),
        &["SAtayati", "SAdayati"],
    );
}

#[test]
fn sutra_7_3_43() {
    assert_has_lat_p(
        &[],
        &nic("ru\\ha~", Gana::Bhvadi),
        &["ropayati", "rohayati"],
    );
}

#[test]
fn sutra_7_3_52() {
    let d = Dhatu::new;

    let pac = d("qupa\\ca~^z", Gana::Bhvadi);
    assert_has_krdanta(&[], &pac, Krt::GaY, &["pAka"]);
    assert_has_krdanta(&[], &d("tya\\ja~", Gana::Bhvadi), Krt::GaY, &["tyAga"]);
    assert_has_krdanta(
        &[],
        &d("ra\\nja~^", Gana::Bhvadi),
        Krt::GaY,
        &["rAga", "raNga"],
    );

    assert_has_krdanta(&[], &pac, Krt::Ryat, &["pAkya"]);
    assert_has_krdanta(&[], &d("va\\ca~", Gana::Bhvadi), Krt::Ryat, &["vAkya"]);
    assert_has_krdanta(&[], &d("ri\\ci~^r", Gana::Bhvadi), Krt::Ryat, &["rekya"]);
}

#[ignore]
#[test]
fn sutra_7_3_56() {
    let hi = Dhatu::new("hi\\", Gana::Svadi);
    let hi_san = hi.clone().with_sanadi(&[Sanadi::San]);
    let hi_yan = hi.clone().with_sanadi(&[Sanadi::Yan]);
    assert_has_lat(&["pra"], &hi_san, &["prajiGIzati"]);
    assert_has_lat(&["pra"], &hi_yan, &["prajeGIyate"]);
    assert_has_lat(&["pra"], &hi, &["prajiGAya"]);

    assert_has_lun(&["pra"], &hi, &["prAjIhayat"]);
}

#[ignore]
#[test]
fn sutra_7_3_57() {
    let ji = Dhatu::new("ji\\", Gana::Bhvadi);
    let ji_san = ji.clone().with_sanadi(&[Sanadi::San]);
    let ji_yan = ji.clone().with_sanadi(&[Sanadi::Yan]);
    assert_has_lat(&[], &ji_san, &["jigIzati"]);
    assert_has_lit(&[], &ji, &["jigAya"]);

    assert_has_lat(&[], &ji_yan, &["jejIyate"]);

    let jya = Dhatu::new("jyA\\", Gana::Kryadi);
    assert_has_parasmai_tinanta(&[], &jya, Lit, Prathama, Dvi, &["jijyatuH"]);
}

#[ignore]
#[test]
fn sutra_7_3_58() {
    let ci = Dhatu::new("ci\\Y", Gana::Svadi);
    let ci_san = ci.clone().with_sanadi(&[Sanadi::San]);
    let ci_yan = ci.clone().with_sanadi(&[Sanadi::Yan]);
    assert_has_lat_p(&[], &ci_san, &["cicIzati", "cikIzati"]);
    assert_has_lit_p(&[], &ci, &["cikAya", "cicAya"]);

    assert_has_lat(&[], &ci_yan, &["cecIyate"]);
}

#[test]
fn sutra_7_3_59() {
    let kuj = Dhatu::new("kUja~", Gana::Bhvadi);
    let kharj = Dhatu::new("Karja~", Gana::Bhvadi);
    let garj = Dhatu::new("garja~", Gana::Bhvadi);

    assert_has_krdanta(&[], &kuj, Krt::GaY, &["kUja"]);
    assert_has_krdanta(&[], &kharj, Krt::GaY, &["Karja"]);
    assert_has_krdanta(&[], &garj, Krt::GaY, &["garja"]);

    assert_has_krdanta(&[], &kuj, Krt::Ryat, &["kUjya"]);
    assert_has_krdanta(&[], &kharj, Krt::Ryat, &["Karjya"]);
    assert_has_krdanta(&[], &garj, Krt::Ryat, &["garjya"]);
}

#[test]
fn sutra_7_3_60() {
    let aj = Dhatu::new("aja~", Gana::Bhvadi);
    let vraj = Dhatu::new("vraja~", Gana::Bhvadi);

    assert_has_krdanta(&["sam"], &aj, Krt::GaY, &["samAja"]);
    assert_has_krdanta(&["ud"], &aj, Krt::GaY, &["udAja"]);

    assert_has_krdanta(&["pari"], &vraj, Krt::GaY, &["parivrAja"]);
    assert_has_krdanta(&["pari"], &vraj, Krt::Ryat, &["parivrAjya"]);
}

#[test]
fn sutra_7_3_82() {
    let mid = Dhatu::new("YimidA~", Gana::Divadi);
    assert_has_lat(&[], &mid, &["medyati"]);
    assert_has_lat_karmani(&[], &mid, &["midyate"]);
}

#[test]
fn sutra_7_3_83() {
    let d = Dhatu::new;
    assert_has_lan_p_3p(&[], &d("hu\\", Gana::Juhotyadi), &["ajuhavuH"]);
    assert_has_lan_p_3p(&[], &d("YiBI\\", Gana::Juhotyadi), &["abiBayuH"]);
    assert_has_lan_p_3p(&[], &d("quBf\\Y", Gana::Juhotyadi), &["abiBaruH"]);

    assert_has_vidhilin_p_3p(&[], &d("ci\\Y", Gana::Svadi), &["cinuyuH"]);
    assert_has_vidhilin_p_3p(&[], &d("zu\\Y", Gana::Svadi), &["sunuyuH"]);
}

#[test]
fn sutra_7_3_84() {
    let d = Dhatu::new;
    assert_has_lat_p(&[], &d("tF", Gana::Bhvadi), &["tarati"]);
    assert_has_lat_p(&[], &d("RI\\Y", Gana::Bhvadi), &["nayati"]);
    assert_has_lat_p(&[], &d("BU", Gana::Bhvadi), &["Bavati"]);

    assert_has_krdanta(&[], &d("qukf\\Y", Gana::Tanadi), Krt::tfc, &["kartf"]);
    assert_has_krdanta(&[], &d("ci\\Y", Gana::Svadi), Krt::tfc, &["cetf"]);
    assert_has_krdanta(&[], &d("zwu\\Y", Gana::Adadi), Krt::tfc, &["stotf"]);

    // TODO: agnitva, agnikAmyati
}

#[test]
fn sutra_7_3_85() {
    let jagf = Dhatu::new("jAgf", Gana::Adadi);

    assert_has_lat_p(
        &[],
        &jagf.clone().with_sanadi(&[Sanadi::Nic]),
        &["jAgarayati"],
    );
    assert_has_krdanta(&[], &jagf, Krt::Rvul, &["jAgaraka"]);
    assert_has_krdanta(&[], &jagf, Krt::GaY, &["jAgara"]);
    assert_has_krdanta(&[], &jagf, Krt::kta, &["jAgarita"]);
    assert_has_krdanta(&[], &jagf, Krt::ktavatu, &["jAgaritavat"]);

    // TODO: add support for unadi sutras
    // assert_has_krdanta(&[], &jagf, Krt::Unadi::vin, &["jAgfvi"]);
    assert_has_lun_karmani(&[], &jagf, &["ajAgAri"]);
    assert_has_lit(
        &[],
        &jagf,
        &["jajAgAra", "jAgarAYcakAra", "jAgarAmAsa", "jAgarAmbaBUva"],
    );
    assert_has_lat_p_3d(&[], &jagf, &["jAgftaH"]);
}

#[test]
fn sutra_7_3_86() {
    let d = |u, g| Dhatu::new(u, g).with_sanadi(&[Sanadi::Nic]);

    assert_has_lat_p(&[], &d("vlI\\", Gana::Kryadi), &["vlepayati"]);
    assert_has_lat_p(&[], &d("hrI\\", Gana::Juhotyadi), &["hrepayati"]);
    assert_has_lat_p(&[], &d("knUyI~\\", Gana::Bhvadi), &["knopayati"]);

    let bhid = Dhatu::new("Bi\\di~^r", Gana::Rudhadi);
    let chid = Dhatu::new("Ci\\di~^r", Gana::Rudhadi);
    assert_has_krdanta(&[], &bhid, Krt::lyuw, &["Bedana"]);
    assert_has_krdanta(&[], &chid, Krt::lyuw, &["Cedana"]);
    assert_has_krdanta(&[], &bhid, Krt::tfc, &["Bettf"]);
    assert_has_krdanta(&[], &chid, Krt::tfc, &["Cettf"]);
}

#[test]
fn sutra_7_3_87() {
    let d = Dhatu::new;

    let nij = d("Ri\\ji~^r", Gana::Juhotyadi);
    let vij = d("vi\\ji~^r", Gana::Juhotyadi);
    let viz = d("vi\\zx~^", Gana::Juhotyadi);

    assert_has_lot_p_1s(&[], &nij, &["nenijAni"]);
    assert_has_lot_p_1s(&[], &vij, &["vevijAni"]);
    assert_has_lot_p_1s(&["pari"], &viz, &["parivevizARi"]);

    assert_has_lan_p_1s(&[], &nij, &["anenijam"]);
    assert_has_lan_p_1s(&[], &vij, &["avevijam"]);
    assert_has_lan_p_1s(&["pari"], &viz, &["paryavevizam"]);

    assert_has_lot_p_1s(&[], &d("vida~", Gana::Adadi), &["vedAni", "vidANkaravARi"]);
    assert_has_lat_p(&[], &nij, &["nenekti"]);
    assert_has_lit_p(&[], &nij, &["nineja"]);

    let hu = d("hu\\", Gana::Juhotyadi);
    assert_has_lot_p_1s(&[], &hu, &["juhavAni"]);
    assert_has_lan_p_1s(&[], &hu, &["ajuhavam"]);

    // TODO: other examples
}

#[test]
fn sutra_7_3_88() {
    let bhu = Dhatu::new("BU", Gana::Bhvadi);
    assert_has_lun(&[], &bhu, &["aBUt"]);
    assert_has_parasmai_tinanta(&[], &bhu, Lun, Uttama, Eka, &["aBUvam"]);

    let su = Dhatu::new("zUN", Gana::Adadi);
    assert_has_atmane_tinanta(&[], &su, Lot, Uttama, Eka, &["suvE"]);
    assert_has_atmane_tinanta(&[], &su, Lot, Uttama, Dvi, &["suvAvahE"]);
    assert_has_atmane_tinanta(&[], &su, Lot, Uttama, Bahu, &["suvAmahE"]);

    assert_has_lat(&[], &bhu, &["Bavati"]);

    // TODO: boBavIti, boBUtu. How to derive vyatiBavizIzwa?
}

#[test]
fn sutra_7_3_89() {
    let yu = Dhatu::new("yu", Gana::Adadi);
    assert_has_parasmai_tinanta(&[], &yu, Lat, Prathama, Eka, &["yOti"]);
    assert_has_parasmai_tinanta(&[], &yu, Lat, Madhyama, Eka, &["yOzi"]);
    assert_has_parasmai_tinanta(&[], &yu, Lat, Uttama, Eka, &["yOmi"]);

    let nu = Dhatu::new("Ru", Gana::Adadi);
    assert_has_parasmai_tinanta(&[], &nu, Lat, Prathama, Eka, &["nOti"]);
    assert_has_parasmai_tinanta(&[], &nu, Lat, Madhyama, Eka, &["nOzi"]);
    assert_has_parasmai_tinanta(&[], &nu, Lat, Uttama, Eka, &["nOmi"]);

    let stu = Dhatu::new("zwu\\Y", Gana::Adadi);
    assert_has_parasmai_tinanta(&[], &stu, Lat, Prathama, Eka, &["stOti", "stavIti"]);
    assert_has_parasmai_tinanta(&[], &stu, Lat, Madhyama, Eka, &["stOzi", "stavIzi"]);
    assert_has_parasmai_tinanta(&[], &stu, Lat, Uttama, Eka, &["stOmi", "stavImi"]);

    let i = Dhatu::new("i\\R", Gana::Adadi);
    assert_has_parasmai_tinanta(&[], &i, Lat, Prathama, Eka, &["eti"]);
    assert_has_parasmai_tinanta(&[], &i, Lat, Madhyama, Eka, &["ezi"]);
    assert_has_parasmai_tinanta(&[], &i, Lat, Uttama, Eka, &["emi"]);

    // luki
    let su = Dhatu::new("zu\\Y", Gana::Svadi);
    assert_has_parasmai_tinanta(&[], &su, Lat, Prathama, Eka, &["sunoti"]);
    assert_has_parasmai_tinanta(&[], &su, Lat, Madhyama, Eka, &["sunozi"]);
    assert_has_parasmai_tinanta(&[], &su, Lat, Uttama, Eka, &["sunomi"]);

    // hali
    let ru = Dhatu::new("ru", Gana::Adadi);
    assert_has_parasmai_tinanta(&[], &yu, Lot, Uttama, Eka, &["yavAni"]);
    assert_has_parasmai_tinanta(&[], &ru, Lot, Uttama, Eka, &["ravARi"]);

    // piti
    assert_has_parasmai_tinanta(&[], &yu, Lat, Prathama, Dvi, &["yutaH"]);
    assert_has_parasmai_tinanta(&[], &ru, Lat, Prathama, Dvi, &["rutaH", "ruvItaH"]);

    // yAsuw
    assert_has_vidhilin_p(&[], &stu, &["stuyAt", "stuvIyAt"]);

    // TODO: yoyogi, roroti
}

#[test]
fn sutra_7_3_90() {
    let urnu = Dhatu::new("UrRuY", Gana::Adadi);
    assert_has_parasmai_tinanta(
        &["pra"],
        &urnu,
        Lat,
        Prathama,
        Eka,
        &["prorROti", "prorRoti"],
    );
    assert_has_parasmai_tinanta(
        &["pra"],
        &urnu,
        Lat,
        Madhyama,
        Eka,
        &["prorROzi", "prorRozi"],
    );
    assert_has_parasmai_tinanta(&["pra"], &urnu, Lat, Uttama, Eka, &["prorROmi", "prorRomi"]);

    // hali
    assert_has_parasmai_tinanta(&["pra"], &urnu, Lot, Uttama, Eka, &["prorRavAni"]);
}

#[test]
fn sutra_7_3_91() {
    let urnu = Dhatu::new("UrRuY", Gana::Adadi);
    assert_has_parasmai_tinanta(&["pra"], &urnu, Lan, Prathama, Eka, &["prOrRot"]);
    assert_has_parasmai_tinanta(&["pra"], &urnu, Lan, Madhyama, Eka, &["prOrRoH"]);
}

#[test]
fn sutra_7_3_92() {
    let tfh = Dhatu::new("tfha~", Gana::Rudhadi);
    assert_has_parasmai_tinanta(&[], &tfh, Lat, Prathama, Eka, &["tfReQi"]);
    assert_has_parasmai_tinanta(&[], &tfh, Lat, Madhyama, Eka, &["tfRekzi"]);
    assert_has_parasmai_tinanta(&[], &tfh, Lat, Uttama, Eka, &["tfRehmi"]);
    assert_has_parasmai_tinanta(&[], &tfh, Lan, Prathama, Eka, &["atfRew"]);

    // hali
    assert_has_parasmai_tinanta(&[], &tfh, Lot, Uttama, Eka, &["tfRahAni"]);
    // piti
    assert_has_parasmai_tinanta(&[], &tfh, Lat, Prathama, Dvi, &["tfRQaH"]);
}

#[test]
fn sutra_7_3_93() {
    let bru = Dhatu::new("brUY", Gana::Adadi);
    assert_has_parasmai_tinanta(&[], &bru, Lat, Prathama, Eka, &["bravIti", "Aha"]);
    assert_has_parasmai_tinanta(&[], &bru, Lat, Madhyama, Eka, &["bravIzi", "AtTa"]);
    assert_has_parasmai_tinanta(&[], &bru, Lat, Uttama, Eka, &["bravImi"]);
    assert_has_parasmai_tinanta(&[], &bru, Lan, Prathama, Eka, &["abravIt"]);

    // hali
    assert_has_parasmai_tinanta(&[], &bru, Lot, Uttama, Eka, &["bravARi"]);
    // piti
    assert_has_parasmai_tinanta(&[], &bru, Lat, Prathama, Dvi, &["brUtaH", "AhatuH"]);
}

#[test]
fn sutra_7_3_95() {
    let d = Dhatu::new;
    assert_has_lat(&["ud"], &d("tu\\", Gana::Adadi), &["uttOti", "uttavIti"]);
    assert_has_lat(&["upa"], &d("ru", Gana::Adadi), &["uparOti", "uparavIti"]);

    let stu = d("zwu\\Y", Gana::Adadi);
    assert_has_lat_p(&["upa"], &stu, &["upastOti", "upastavIti"]);
    assert_has_atmane_tinanta(&[], &stu, VidhiLin, Prathama, Eka, &["stuvIta"]);

    // Sam and am are chAndasa.
}

#[test]
fn sutra_7_3_96() {
    let d = Dhatu::new;

    let asa = d("asa~", Gana::Adadi);
    assert_has_parasmai_tinanta(&[], &asa, Lan, Prathama, Eka, &["AsIt"]);
    assert_has_parasmai_tinanta(&[], &asa, Lan, Madhyama, Eka, &["AsIH"]);

    let kf = Dhatu::new("qukf\\Y", Gana::Tanadi);
    assert_has_lun_p(&[], &kf, &["akArzIt"]);
    assert_has_lun_p(&[], &d("zu\\Y", Gana::Svadi), &["asAvIt"]);
    assert_has_lun_p(&[], &d("lUY", Gana::Kryadi), &["alAvIt"]);
    assert_has_lun_p(&[], &d("pUY", Gana::Kryadi), &["apAvIt"]);

    assert_has_lat(&[], &asa, &["asti"]);
    assert_has_parasmai_tinanta(&[], &kf, Lun, Uttama, Eka, &["akArzam"]);
}

#[test]
fn sutra_7_3_98_and_sutra_7_3_99() {
    let d = Dhatu::new;

    let rud = d("rudi~r", Gana::Adadi);
    assert_has_parasmai_tinanta(&[], &rud, Lan, Prathama, Eka, &["arodIt", "arodat"]);
    assert_has_parasmai_tinanta(&[], &rud, Lan, Madhyama, Eka, &["arodIH", "arodaH"]);

    let svap = d("Yizva\\pa~", Gana::Adadi);
    assert_has_parasmai_tinanta(&[], &svap, Lan, Prathama, Eka, &["asvapIt", "asvapat"]);
    assert_has_parasmai_tinanta(&[], &svap, Lan, Madhyama, Eka, &["asvapIH", "asvapaH"]);

    let svas = d("Svasa~", Gana::Adadi);
    assert_has_parasmai_tinanta(&[], &svas, Lan, Prathama, Eka, &["aSvasIt", "aSvasat"]);
    assert_has_parasmai_tinanta(&[], &svas, Lan, Madhyama, Eka, &["aSvasIH", "aSvasaH"]);

    let an = d("ana~", Gana::Adadi);
    assert_has_parasmai_tinanta(&["pra"], &an, Lan, Prathama, Eka, &["prARIt", "prARat"]);
    assert_has_parasmai_tinanta(&["pra"], &an, Lan, Madhyama, Eka, &["prARIH", "prARaH"]);

    let jaks = d("jakza~", Gana::Adadi);
    assert_has_parasmai_tinanta(&[], &jaks, Lan, Prathama, Eka, &["ajakzIt", "ajakzat"]);
    assert_has_parasmai_tinanta(&[], &jaks, Lan, Madhyama, Eka, &["ajakzIH", "ajakzaH"]);

    // pancabhyaH
    let jagf = d("jAgf", Gana::Adadi);
    assert_has_parasmai_tinanta(&[], &jagf, Lan, Madhyama, Eka, &["ajAgaH"]);
    // apkrtasya
    assert_has_lat(&[], &rud, &["roditi"]);
}

#[ignore]
#[test]
fn sutra_7_3_109() {
    assert_has_subantas("agni", Pum, V::Prathama, Bahu, &["harayaH"]);
    assert_has_subantas("vAyu", Pum, V::Prathama, Bahu, &["vAyavaH"]);
    assert_has_subantas("pawu", Pum, V::Prathama, Bahu, &["pawavaH"]);
    assert_has_subantas("Denu", Stri, V::Prathama, Bahu, &["DenavaH"]);
    assert_has_subantas("budDi", Stri, V::Prathama, Bahu, &["budDayaH"]);
}
