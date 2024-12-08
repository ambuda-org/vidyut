/*!
Implements rules from the pāṇiṇīyaliṅgānuśāśanam, which assigns lingas to various terms.
*/

use crate::args::BaseKrt as K;
use crate::args::Taddhita as D;
use crate::core::{Prakriya, Rule};
use crate::core::{PrakriyaTag as PT, Tag as T};

const DVARA_ADI: &[&str] = &[
    "dvAra",
    "agra",
    "sPara",
    "takra",
    "vakra",
    "vapra",
    "kzipra",
    "kzudra",
    "nAra",
    "tIra",
    "dUra",
    "kfcCra",
    "randRa",
    "aSra",
    "SvaBra",
    "BIra",
    "gaBIra",
    "krUra",
    "vicitra",
    "keyUra",
    "kedAra",
    "udAra",
    "ajasra",
    "SarIra",
    "kandara",
    "mandAra",
    "paYjara",
    "ajara",
    "jaWara",
    "jira",
    "vEra",
    "cAmara",
    "puzkara",
    "gahvara",
    "kuhara",
    "kuwIra",
    "kulIra",
    "catvara",
    "kASmIra",
    "nIra",
    "ambara",
    "SiSira",
    "tantra",
    "yantra",
    "nakzatra",
    "kzetra",
    "mitra",
    "kalatra",
    "citra",
    "mUtra",
    "sUtra",
    "vaktra",
    "netra",
    "gotra",
    "aNgulitra",
    "Balatra",
    "Sastra",
    "SAstra",
    "vastra",
    "patra",
    "pAtra",
    "Catra",
];

#[derive(Debug)]
struct LingaPrakriya<'a> {
    p: &'a mut Prakriya,
    done: bool,
}

impl<'a> LingaPrakriya<'a> {
    fn new(p: &'a mut Prakriya) -> Self {
        LingaPrakriya { p, done: false }
    }
    fn mark_pum(&mut self, rule: Rule) {
        if !self.done {
            self.p.add_tag(PT::Pum);
            self.p.step(rule);
        }
        self.done = true;
    }

    fn mark_stri(&mut self, rule: Rule) {
        if !self.done {
            self.p.add_tag(PT::Stri);
            self.p.step(rule);
        }
        self.done = true;
    }

    fn mark_napumsaka(&mut self, rule: Rule) {
        if !self.done {
            self.p.add_tag(PT::Napumsaka);
            self.p.step(rule);
        }
        self.done = true;
    }

    fn mark_pum_napumsaka(&mut self, _rule: Rule) {}

    fn mark_stri_napumsaka(&mut self, _rule: Rule) {}

    fn mark_stri_pum(&mut self, _rule: Rule) {}
}

/// Runs the linganushasana rules over the given prakriya.
pub fn run(p: &mut Prakriya) -> Option<()> {
    use Rule::Linganushasana as L;

    if p.has_tag(PT::Stri) {
        return None;
    }

    let mut lp = LingaPrakriya::new(p);
    let i_last = lp.p.terms().len() - 1;

    // strI (2 - 34)
    // =============

    let last = lp.p.get(i_last)?;
    if last.has_antya('f') && last.has_u_in(&["mAtf", "duhitf", "svasf", "yAtf", "nanAndf"]) {
        // mAtf, ...
        lp.mark_stri(L("3"));
    } else if last.is_unadi() && last.has_u_in(&["ani", "U"]) {
        // avani, camU, ...
        lp.mark_stri(L("4"));
    } else if last.has_text_in(&["aSani", "BaraRi", "araRi"]) {
        lp.mark_stri_pum(L("5"));
    } else if last.is_unadi() && last.has_u_in(&["mi", "ni"]) {
        // BUmi, glAni, ...
        lp.mark_stri(L("6"));
    } else if last.is_pratyaya() && last.is(K::ktin) {
        // kfti, ...
        lp.mark_stri(L("9"));
    } else if last.is_pratyaya() && last.has_antya('I') {
        // lakzmIH ...
        lp.mark_stri(L("10"));
    } else if last.is_pratyaya() && (last.has_antya('U') || last.has_u("NAp")) {
        // kurUH, vidyA, ...
        lp.mark_stri(L("11"));
    } else if last.is_taddhita() && last.is(D::tal) {
        // SuklatA, ...
        lp.mark_stri(L("17"));
    } else if last.has_text_in(&["BUmi", "vidyut", "sarit", "latA", "vanitA"]) {
        // TODO: others
        lp.mark_stri(L("18"));
    } else if last.has_text("yAdas") {
        // yAdaH
        lp.mark_stri(L("19"));
    } else if last.has_text_in(&["BAs", "sruj", "sraj", "diS", "uzRih", "upAnah"]) {
        // BAH, ...
        lp.mark_stri(L("20"));
    } else if last.has_text_in(&["gfhasTURa", "SaSorRa"]) {
        // gfhasTURam, SaSorRam
        lp.mark_napumsaka(L("22"));
    } else if last.has_text_in(&["sTURa", "UrRa"]) {
        // sTURA, sTURam, ...
        lp.mark_stri_napumsaka(L("21"));
    }

    // pumAn (35 - 117)
    // ================

    let last = lp.p.get(i_last)?;
    if last.is_krt() {
        let i_dhatu = lp.p.find_last_where(|t| t.is_dhatu())?;
        let dhatu = lp.p.get(i_dhatu)?;

        if last.has_text_in(&["Baya", "liNga", "Baga", "pada"]) {
            // Bayam, ...
            lp.mark_napumsaka(L("38"));
        } else if last.is_any_krt(&[K::GaY, K::ap]) {
            lp.mark_pum(L("36"));
        } else if last.is(K::Ga) || last.is(K::ac) {
            lp.mark_pum(L("37"));
        } else if last.is(K::naN) {
            if dhatu.has_text("yAc") {
                lp.mark_stri(L("40"));
            } else {
                lp.mark_pum(L("39"));
            }
        } else if dhatu.has_tag(T::Ghu) && last.has_u("ki") {
            lp.mark_pum(L("41"));
        }
    } else if last.has_text_in(&[
        "deva", "asura", "Atman", "svarga", "giri", "samudra", "naKa", "keSa", "danta", "stana",
        "Buja", "kaRWa", "Kaqga", "Sara", "PaNka",
    ]) {
        // TODO: the rule accepts pratipadikas with these *meanings*, not (just) the pratipadikas
        // themselves.
        lp.mark_napumsaka(L("43"));
    } else if last.has_text_in(&["trivizwapa", "triBuvana"]) {
        // trivizwapam, ...
        // (exception to 43)
        lp.mark_napumsaka(L("44"));
    } else if last.has_text_in(&["dyo", "div"]) {
        // dyOH
        // (exception to 43)
        lp.mark_stri(L("45"));
    } else if last.has_text_in(&["izu", "bAhu"]) {
        // izuH, ...
        // (exception to 43)
        lp.mark_stri_pum(L("46"));
    } else if last.has_text_in(&["bARa", "kARqa"]) {
        // bARaH, ...
        // (exception to 43)
        lp.mark_pum_napumsaka(L("47"));
    } else if last.has_antya('n') {
        // rAjA, ...
        lp.mark_pum(L("48"));
    } else if last.has_text_in(&["kratu", "puruza", "kapola", "gulPa", "meDa"]) {
        // TODO: the rule accepts pratipadikas with these *meanings*, not (just) the pratipadikas
        // themselves.
        // kratuH, ...
    } else if last.has_text("aBra") {
        // aBram
        // (exception to 49)
        lp.mark_napumsaka(L("50"));
    } else if last.has_antya('u') {
        if last.has_text_in(&[
            "Denu", "rajju", "kuhu", "sarayu", "tanu", "reRu", "priyaNgu",
        ]) {
            // DenuH, ...
            // TODO: rule has kuhU in most editions but this section is for udanta -- typo for
            // kuhu?
            lp.mark_stri(L("52"));
        } else if last.has_text_in(&[
            "SmaSru", "jAnu", "vasu", "svAdu", "aSru", "jatu", "trapu", "tAlu",
        ]) {
            // SmaSru, ...
            lp.mark_napumsaka(L("54"));
        } else if last.has_text_in(&["madgu", "maDu", "sIDu", "SIDu", "sAnu", "kamaRqalu"]) {
            // madguH, madgu
            lp.mark_pum_napumsaka(L("56"));
        } else {
            // praBuH, ...
            lp.mark_pum(L("51"));
        }
    } else if last.ends_with("ru") {
        if last.has_text_in(&["dAru", "kaSeru", "jatu", "vastu", "mastu"]) {
            lp.mark_napumsaka(L("58"));
        } else {
            lp.mark_pum(L("57"));
        }
    }

    // prAgaraSmer akArANtaH (60 - 100)
    // ================================
    // (nested within pumAn adhikAra)

    let last = lp.p.get(i_last)?;
    let at = last.has_antya('a');
    // adhikAra starts at rule 60 and ends at rule 100.
    if at && last.has_upadha('k') {
        if last.has_text_in(&["cibuka", "SAlUka", "prAtipadika", "aMSuka", "ulmuka"]) {
            lp.mark_napumsaka(L("62"));
        } else if last.has_text_in(&[
            "kaRwaka", "anIka", "saraka", "modaka", "cazaka", "mastaka", "pustaka", "taqAka",
            "nizka", "Suzka", "varcaska", "pinAka", "BARqaka", "piRqaka", "kawaka", "SaRqaka",
            "piwaka", "tAlaka", "Palaka", "pulAka",
        ]) {
            lp.mark_pum_napumsaka(L("62"));
        } else {
            lp.mark_pum(L("61"));
        }
    } else if at && last.has_upadha('w') {
        if last.has_text_in(&[
            "kirIwa", "mukuwa", "lalAwa", "vawa", "vIwa", "SfRgAwa", "karAwa", "lozwa",
        ]) {
            lp.mark_pum_napumsaka(L("65"));
        } else if last.has_text_in(&[
            "kuwa", "kUwa", "kawa", "pawa", "kavAwa", "karpawa", "nawa", "nikawa", "kIwa", "kawa",
        ]) {
            lp.mark_pum_napumsaka(L("66"));
        } else {
            lp.mark_pum(L("64"));
        }
    } else if at && last.has_upadha('R') {
        // guRaH, ...
        lp.mark_pum(L("67"));
    } else if at && last.has_upadha('T') {
        // raTaH, ...
        lp.mark_pum(L("70"));
    } else if at && last.has_upadha('n') {
        if last.has_text_in(&[
            "jaGana", "akina", "tuhina", "kAnana", "vana", "vfjina", "vipina", "vetana", "SAsana",
            "sopAna", "miTuna", "SmaSAna", "ratna", "nimna", "cihna",
        ]) {
            lp.mark_napumsaka(L("75"));
        } else if last.has_text_in(&[
            "mAna",
            "yAna",
            "aBiDAna",
            "malina",
            "pulina",
            "udyAna",
            "Sayana",
            "Asana",
            "sTAna",
            "candana",
            "asamAna",
            "Bavana",
            "vasana",
            "saMBAvana",
            "viBAvana",
            "vimAna",
        ]) {
            lp.mark_pum_napumsaka(L("76"));
        } else {
            // PenaH, ...
            lp.mark_pum(L("74"));
        }
    } else if at && last.has_upadha('p') {
        if last.has_text_in(&[
            "pApa", "rUpa", "uqupa", "talpa", "Silpa", "puzpa", "samIpa", "antarIpa",
        ]) {
            // pApam, ...
            lp.mark_napumsaka(L("78"));
        } else if last.has_text_in(&["SUrpa", "kutapa", "kuRapa", "dvIpa", "viwapa"]) {
            // SUrpaH, SUrpam, ...
            lp.mark_pum_napumsaka(L("79"));
        } else {
            // yUpaH, ...
            lp.mark_pum(L("77"));
        }
    } else if at && last.has_upadha('B') {
        if last.has_text("talaBa") {
            // talaBam
            lp.mark_napumsaka(L("81"));
        } else if last.has_text("jfmBa") {
            // jfmBam, jfmBaH
            lp.mark_pum_napumsaka(L("82"));
        } else {
            // stamBaH, ...
            lp.mark_pum(L("80"));
        }
    } else if at && last.has_upadha('m') {
        if last.has_text_in(&[
            "rukma", "siDma", "yugma", "eDma", "gulma", "aDyAtma", "kuNkuma",
        ]) {
            lp.mark_napumsaka(L("84"));
        } else if last.has_text_in(&[
            "saMgrAma", "dAqima", "kusuma", "ASrama", "kzema", "kzOma", "homa", "uddAma",
        ]) {
            // saMgrAmaH, saMgrAmam, ...
            lp.mark_pum_napumsaka(L("85"));
        } else {
            // somaH, ...
            lp.mark_pum(L("83"));
        }
    } else if at && last.has_upadha('y') {
        if last.has_text_in(&["kisalaya", "hfdaya", "indriya", "uttarIya"]) {
            // kisalayam, ...
            lp.mark_napumsaka(L("87"));
        } else if last.has_text_in(&["gomaya", "kazAya", "malaya", "anvaya", "avyaya"]) {
            lp.mark_pum_napumsaka(L("88"));
        } else {
            // samayaH, ...
            lp.mark_pum(L("86"));
        }
    } else if at && last.has_upadha('r') {
        if last.has_text_in(DVARA_ADI) {
            lp.mark_napumsaka(L("90"));
        } else if last.has_text_in(&[
            "cakra",
            "vajra",
            "anDakAra",
            "sAra",
            "avAra",
            "pAra",
            "kzIra",
            "tomara",
            "SfNgAra",
            "bazaNgAra",
            "mandAra",
            "uSIra",
            "timira",
            "SiSira",
        ]) {
            lp.mark_pum_napumsaka(L("92"));
        } else {
            // kzuraH
            lp.mark_pum(L("89"));
        }
    } else if at && last.has_upadha('z') {
        if last.has_text_in(&[
            "SirIza", "fjIza", "ambarIza", "pIyUza", "purIza", "kilbiza", "kalmAza",
        ]) {
            lp.mark_napumsaka(L("94"));
        } else if last.has_text_in(&["yUza", "karIza", "miza", "viza", "varza"]) {
            lp.mark_pum_napumsaka(L("95"));
        } else {
            // vfzaH
            lp.mark_pum(L("93"));
        }
    } else if at && last.has_upadha('s') {
        if last.has_text_in(&["panasa", "bisa", "busa", "sAhasa"]) {
            // panasam, ...
            lp.mark_napumsaka(L("97"));
        } else if last.has_text_in(&[
            "camasa", "aMsa", "rasa", "niryAsa", "upavAsa", "kArpAsa", "vAsa", "mAsa", "kAsa",
            "kAMsa", "mAMsa",
        ]) {
            // camasam, camasaH, ...
            lp.mark_pum_napumsaka(L("98"));
        } else {
            // vatsaH, ...
            lp.mark_pum(L("96"));
        }
    }

    let last = lp.p.get(i_last)?;
    if last.has_text_in(&["marut", "garut", "tarat", "ftvij"]) {
        // marut, ...
        lp.mark_pum(L("108"));
    } else if last.has_text_in(&[
        "fzi", "rASi", "dfti", "granTi", "krimi", "Dvani", "bali", "kOli", "mOli", "ravi", "kavi",
        "kapi", "muni",
    ]) {
        lp.mark_pum(L("109"));
    } else if last.has_text_in(&["Dvaja", "gaja", "muYja", "puYja"]) {
        // DvajaH, ...
        lp.mark_pum(L("110"));
    } else if last.has_text_in(&[
        "hasta", "kunta", "anta", "vrAta", "vAta", "dUta", "Duta", "sUta", "cUta", "muhUrta",
    ]) {
        // hasta, ...
        lp.mark_pum(L("111"));
    } else if last.has_text_in(&[
        "zaRqa", "maRqa", "karaRqa", "BaraRqa", "varaRqa", "tuRqa", "gaRqa", "muRqa", "pAzaRqa",
        "SiKaRqa",
    ]) {
        // zaRqaH, ...
        lp.mark_pum(L("112"));
    } else if last.has_text_in(&["vaMSa", "aMSa", "puroqASa"]) {
        // vaMSaH, ...
        lp.mark_pum(L("113"));
    } else if last.has_text_in(&["hrada", "kanda", "kunda", "budbuda", "Sabda"]) {
        // hradaH, ...
        lp.mark_pum(L("114"));
    }

    // napuMsakam (118 - 170)
    // ======================

    let last = lp.p.get(i_last)?;
    if last.has_u_in(&["tva", "zyaY"]) {
        // Suklatvam, SOklyam, ...
        lp.mark_napumsaka(L("121"));
    } else if last.has_suffix_in(&["is", "us"]) {
        if last.has_text("arcis") {
            // arciH
            lp.mark_stri_napumsaka(L("135"));
        } else if last.has_text("Cadis") {
            // CadiH
            lp.mark_stri(L("135"));
        } else {
            // haviH, DanuH
            lp.mark_napumsaka(L("134"));
        }
    } else if last.has_text_in(&["vaktra", "netra", "araRya", "gARqIva"]) {
        // vaktraH, vaktram
        lp.mark_pum_napumsaka(L("134"));
    } else if last.has_upadha('l') {
        if last.has_text_in(&[
            "tUla", "upala", "tAla", "kusUla", "tarala", "kambala", "devala", "vfzala",
        ]) {
            lp.mark_pum(L("142"));
        } else if last.has_text_in(&[
            "SIla", "mUla", "maNgala", "sAla", "kamala", "tala", "musala", "kuRqala", "patala",
            "mfRAla", "vAla", "nigala", "palAla", "biqAla", "Kila", "SUla",
        ]) {
            lp.mark_pum_napumsaka(L("142"));
        } else {
            // kulam
            lp.mark_napumsaka(L("141"));
        }
    } else if last.ends_with("tra") {
        lp.mark_napumsaka(L("154"));
    } else if last.has_text_in(&[
        "viyat", "jagat", "sakft", "Sakan", "pfzat", "Sakft", "yakft", "udaSvit",
    ]) {
        lp.mark_napumsaka(L("164"));
    } else if last.has_text("dEva") {
        lp.mark_pum_napumsaka(L("167"));
    }

    // strIpuMsayoH (171 - 175)
    // ========================

    let last = lp.p.get(i_last)?;
    if last.has_text_in(&[
        "go", "maRi", "yazwi", "muzwi", "pAwali", "vasti", "SAlmali", "truwi", "masi", "marIci",
    ]) {
        lp.mark_stri_pum(L("172"));
    } else if last.has_text_in(&["mftyu", "sIDu", "karkanDu", "kizku", "kuRqu", "reRu"]) {
        // TODO: reRu in rule 52?
        lp.mark_stri_pum(L("172"));
    }

    // puMnapuMsakayoH (176 - 182)
    // ===========================

    let last = lp.p.get(i_last)?;
    if last.has_text_in(&[
        "Gfta", "BUta", "musta", "kzvelita", "ErAvata", "pustaka", "busta", "lohita",
    ]) {
        // GftaH, Gftam
        lp.mark_pum_napumsaka(L("177"));
    } else if last.has_text_in(&["SfNga", "arGa", "nidAGa", "udyama", "Salya", "dfQa"]) {
        // SfNgaH, SfNgam
        lp.mark_pum_napumsaka(L("178"));
    } else if last.has_text_in(&[
        "vajra", "kuJya", "kuTa", "kUrca", "prasTa", "darpa", "arBa", "arDa", "arca", "darBa",
        "pucCa",
    ]) {
        // vajraH, vajram
        // TODO: vajra also in sutra 92?
        lp.mark_pum_napumsaka(L("179"));
    } else if last.has_text_in(&["kabanDa", "OzaDa", "AyuDa", "ant"]) {
        // kabanDaH, kabanDam
        lp.mark_pum_napumsaka(L("180"));
    } else if last.has_text_in(&[
        "daRqa", "maRqa", "Kama", "Sama", "sEnDava", "pArSva", "AkASa", "kuSa", "kASa", "aNkuSa",
        "kuliSa",
    ]) {
        // daRqaH, daRqam
        lp.mark_pum_napumsaka(L("181"));
    } else if last.has_text_in(&[
        "gfha", "meha", "deha", "pawwa", "pawaha", "azwApada", "ambuda", "kakuda",
    ]) {
        lp.mark_pum_napumsaka(L("182"));
    }

    // aviSizwaliNgam (183 - 189)
    // ==========================

    Some(())
}
