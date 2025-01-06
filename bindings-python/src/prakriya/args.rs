/*!
Wrappers for vidyut-prakriya arguments.

Pyo3 doesn't allow us to annotate existing enums, and using a wrapping struct has poor ergonomics
for callers. So instead, redefine our enums of interest.
*/
use crate::utils::py_enum;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use vidyut_prakriya::args::{BaseKrt as Krt, *};

#[pyclass(name = "Gana", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd)]
pub enum PyGana {
    /// The first gaṇa, whose first dhatu is `BU`.
    Bhvadi,
    /// The second gaṇa, whose first dhatu is `ad`.
    Adadi,
    /// The third gaṇa, whose first dhatu is `hu`.
    Juhotyadi,
    /// The fourth gaṇa, whose first dhatu is `div`.
    Divadi,
    /// The fifth gaṇa, whose first dhatu is `su`.
    Svadi,
    /// The sixth gaṇa, whose first dhatu is `tud`.
    Tudadi,
    /// The seventh gaṇa, whose first dhatu is `ruD`.
    Rudhadi,
    /// The eighth gaṇa, whose first dhatu is `tan`.
    Tanadi,
    /// The ninth gaṇa, whose first dhatu is `krI`.
    Kryadi,
    /// The tenth gaṇa, whose first dhatu is `cur`.
    Curadi,
    /// The kandvAdi gaṇa, whose first dhatu is `kaRqU`.
    Kandvadi,
}

py_enum!(
    PyGana,
    Gana,
    [Bhvadi, Adadi, Juhotyadi, Divadi, Svadi, Tudadi, Rudhadi, Tanadi, Kryadi, Curadi, Kandvadi]
);

#[pyclass(name = "Antargana", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PyAntargana {
    /// Antargana of *bhU* gana. A dhatu in this antargana uses a shortened vowel when followed by
    /// Ric-pratyaya.
    Ghatadi,
    /// Antargana of *tud* gana. Pratyayas that follow dhatus in kut-Adi will generally be marked
    /// Nit per 1.2.1. Required because of duplicates like `juqa~`.
    Kutadi,
    /// Antargana of *cur* gana ending with `zvada~` / `svAda~`. A dhatu in this antargana
    /// optionaly uses Ric-pratyaya when taking an object. Required because of duplicates like
    /// `tuji~`.
    Asvadiya,
    /// Antargana of *cur* gana ending with `Dfza~`. A dhatu in this antargana optionally uses
    /// Ric-pratyaya. Required because of duplicates like `SraTa~`.
    Adhrshiya,
    /// Antargana of *cur* gana ending with `kusma~`. A dhatu in this antargana is always
    /// ātmanepadī. Required because of duplicates like `daSi~`.
    Akusmiya,
}

py_enum!(
    PyAntargana,
    Antargana,
    [Ghatadi, Kutadi, Asvadiya, Adhrshiya, Akusmiya]
);

/// The complete list of ordinary *kṛt* pratyayas.
#[pyclass(name = "Krt", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum PyKrt {
    /// -a
    a,
    /// -a,
    aN,
    /// -a
    ac,
    /// -a
    aR,
    /// -aDyE
    aDyE,
    /// -aDyE
    aDyEn,
    /// -at (jarat)
    atfn,
    /// -aTu (vepaTu). Allowed only for dhatus that are `qvit`.
    aTuc,
    /// -ani
    ani,
    /// -anIya (gamanIya, BavanIya, ...)
    anIyar,
    /// -a
    ap,
    /// -ase
    ase,
    /// -ase
    asen,
    /// -Alu
    Aluc,
    /// -Aru
    Aru,
    /// -ika
    ika,
    /// -ikavaka
    ikavaka,
    /// -itra
    itra,
    /// -in. The trailing `_` is to avoid colliding with Rust's `in` keyword.
    in_,
    /// -in
    ini,
    /// -izRu (alaMkarizRu, prajanizRu, ...)
    izRuc,
    /// -u (yuyutsu, Bikzu, ...)
    u,
    /// -uka
    ukaY,
    /// -Uka
    Uka,
    /// -a
    ka,
    /// -a
    kaY,
    /// -aDyE
    kaDyE,
    /// -aDyE
    kaDyEn,
    /// -am
    kamul,
    /// -as (visfpaH, ...)
    kasun,
    /// -a
    kap,
    /// -ase
    kase,
    /// -ase
    kasen,
    /// -Ana (cakrARa, ...)
    kAnac,
    /// -i (udaDi, ...)
    ki,
    /// -i
    kin,
    /// -ura (BaNgura, ...)
    kurac,
    /// -elima (pacelima, ...)
    kelimar,
    /// -ta (gata, bhUta, ...)
    kta,
    /// -tavat (gatavat, bhUtavat, ...)
    ktavatu,
    /// -ti
    ktic,
    /// -ti
    ktin,
    /// -tri
    ktri,
    /// -tvA (gatvA, bhUtva, ...)
    ktvA,
    /// -nu
    knu,
    /// -mara
    kmarac,
    /// -ya
    kyap,
    /// -ru (BIru)
    kru,
    /// -ruka (BIruka)
    krukan,
    /// -luka (BIluka)
    klukan,
    /// -van
    kvanip,
    /// -vara
    kvarap,
    /// -vas
    kvasu,
    /// -snu (glAsnu, jizRu, ...)
    ksnu,
    /// (empty suffix)
    kvin,
    /// (empty suffix)
    kvip,
    /// -a (priyaMvada, vaSaMvada)
    Kac,
    /// -a
    KaS,
    /// -a (Izatkara, duzkara, sukara, ...)
    Kal,
    /// -izRu
    KizRuc,
    /// -uka
    KukaY,
    /// -ana
    Kyun,
    /// -a
    Ga,
    /// -a
    GaY,
    /// -in
    GinuR,
    /// -ura
    Gurac,
    /// -van
    Nvanip,
    /// -Ana
    cAnaS,
    /// -ana
    Yyuw,
    /// -a
    wa,
    /// -a
    wak,
    /// -a
    qa,
    /// -ara,
    qara,
    /// -u
    qu,
    /// -a
    Ra,
    /// -am
    Ramul,
    /// -in
    Rini,
    /// -ya
    Ryat,
    /// -ana
    Ryuw,
    /// (empty)
    Rvi,
    /// -aka
    Rvuc,
    /// -aka
    Rvul,
    /// -tave
    taveN,
    /// -tave
    taven,
    /// -tavE
    tavE,
    /// -tavya (gantavya, bhavitavya, ...)
    tavya,
    /// -tavya
    tavyat,
    /// -tum (gantum, bhavitum, ...)
    tumun,
    /// -tf (gantA, bhavitA, ...)
    tfc,
    /// -tf
    tfn,
    /// -tos (udetoH)
    tosun,
    /// -Taka (gATaka)
    Takan,
    /// -na
    naN,
    /// -naj
    najiN,
    /// -na (svapna)
    nan,
    /// -ni,
    ni,
    /// -man
    manin,
    /// -ya
    yat,
    /// -ana
    yuc,
    /// -na (namra, kampra, ...)
    ra,
    /// -ru
    ru,
    /// -ana
    lyu,
    /// -ana
    lyuw,
    /// -van
    vanip,
    /// -vara
    varac,
    /// (empty suffix)
    vic,
    /// (none)
    viw,
    /// -aka
    vuY,
    /// -aka
    vun,
    /// -Aka
    zAkan,
    /// -tra
    zwran,
    /// -aka
    zvun,
    /// -a
    Sa,
    /// -at (gacCat, Bavat, ...)
    Satf,
    /// -aDyE
    SaDyE,
    /// -aDyE
    SaDyEn,
    /// -Ana (laBamAna, sevamAna, ...)
    SAnac,
    /// -Ana
    SAnan,
    /// -se
    se,
    /// -se
    sen,
}

py_enum!(
    PyKrt,
    Krt,
    [
        a, aN, ac, aR, aDyE, aDyEn, atfn, aTuc, ani, anIyar, ap, ase, asen, Aluc, Aru, ika,
        ikavaka, itra, in_, ini, izRuc, u, ukaY, Uka, ka, kaY, kaDyE, kaDyEn, kamul, kasun, kap,
        kase, kasen, kAnac, ki, kin, kurac, kelimar, kta, ktavatu, ktic, ktin, ktri, ktvA, knu,
        kmarac, kyap, kru, krukan, klukan, kvanip, kvarap, kvasu, ksnu, kvin, kvip, Kac, KaS, Kal,
        KizRuc, KukaY, Kyun, Ga, GaY, GinuR, Gurac, Nvanip, cAnaS, Yyuw, wa, wak, qa, qara, qu, Ra,
        Ramul, Rini, Ryat, Ryuw, Rvi, Rvuc, Rvul, taveN, taven, tavE, tavya, tavyat, tumun, tfc,
        tfn, tosun, Takan, naN, najiN, nan, ni, manin, yat, yuc, ra, ru, lyu, lyuw, vanip, varac,
        vic, viw, vuY, vun, zAkan, zwran, zvun, Sa, Satf, SaDyE, SaDyEn, SAnac, SAnan, se, sen
    ]
);

/// The complete list of *taddhita* pratyayas.
///
/// Each pratyaya name is written in the SLP1 encoding scheme.
#[pyclass(name = "Taddhita", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum PyTaddhita {
    /// a
    a,
    /// -aka
    akac,
    /// -a
    ac,
    /// -aWa
    aWac,
    /// -a
    aR,
    /// -a
    aY,
    /// -a
    at,
    /// -atas
    atasuc,
    /// -an
    anic,
    /// -a
    ap,
    /// -as
    asic,
    /// -astAt
    astAti,
    /// -Akin
    Akinic,
    /// -Ara
    Arak,
    /// -i
    iY,
    /// -ita
    itac,
    /// -ina
    inac,
    /// -in
    ini,
    /// -iman
    imanic,
    /// -ila
    ila,
    /// -ila
    ilac,
    /// -izWa
    izWan,
    /// -Ika
    Ikak,
    /// -Ika
    Ikan,
    /// -Iyas
    Iyasun,
    /// -eRya
    eRya,
    /// -Era
    Erak,
    /// -ka
    ka,
    /// -ka
    kak,
    /// -kawa
    kawac,
    /// -ka
    kan,
    /// -ka
    kap,
    /// -kalpa
    kalpap,
    /// -kftvas
    kftvasuc,
    /// -kuwAra
    kuwArac,
    /// -kura
    kuRap,
    /// -Ina
    Ka,
    /// -Ina
    KaY,
    /// -iya
    Ga,
    /// -iya
    Gac,
    /// -iya
    Gan,
    /// -iya
    Gas,
    /// -caRa
    caRap,
    /// -cara
    caraw,
    /// -cuYcu
    cuYcup,
    /// -Ayana
    cPaY,
    /// --
    cvi,
    /// -Iya
    Ca,
    /// -Iya
    CaR,
    /// -Iya
    Cas,
    /// -jAtIya
    jAtIyar,
    /// -jAha
    jAhac,
    /// -a
    Ya,
    /// -ika
    YiW,
    /// -ya
    Yya,
    /// -ya
    YyaN,
    /// -ya
    Yyaw,
    /// -a
    wac,
    /// -a
    waq,
    /// -iWa
    wiWan,
    /// -wIwa
    wIwac,
    /// -eRya
    weRyaR,
    /// -ya
    wyaR,
    /// -ana
    wyu,
    /// -ana
    wyul,
    /// -la
    wlaY,
    /// -ika
    Wak,
    /// -ika
    Wac,
    /// -ika
    WaY,
    /// -ika
    Wan,
    /// -ika
    Wap,
    /// -a
    qaw,
    /// -ati
    qati,
    /// -atara
    qatarac,
    /// -atama
    qatamac,
    /// -pa
    qupac,
    /// -mat
    qmatup,
    /// -ya
    qyaR,
    /// -vala
    qvalac,
    /// -aka
    qvun,
    /// -eya
    Qak,
    /// -eyaka
    QakaY,
    /// -eya
    Qa,
    /// -eya
    QaY,
    /// -eyin
    Qinuk,
    /// -era
    Qrak,
    /// -a
    Ra,
    /// -in
    Rini,
    /// -ya
    Rya,
    /// -tama
    tamap,
    /// -taya
    tayap,
    /// -tara
    tarap,
    /// -ta (becomes -tA)
    tal,
    /// -tas
    tasi,
    /// -tas
    tasil,
    /// -ti
    ti,
    /// -tika
    tikan,
    /// -tIya
    tIya,
    /// -tya
    tyak,
    /// -tyaka
    tyakan,
    /// -tya
    tyap,
    /// -tana
    tyu,
    /// -tana
    tyul,
    /// -tra
    tral,
    /// -trA
    trA,
    /// -tva
    tva,
    /// -Tam
    Tamu,
    /// -Tya
    Tyan,
    /// -TA
    TAl,
    /// -daGna
    daGnac,
    /// -dA
    dA,
    /// -dAnIm
    dAnIm,
    /// -deSya
    deSya,
    /// -deSIya
    deSIyar,
    /// -dvayasa
    dvayasac,
    /// -dhA
    DA,
    /// -na
    na,
    /// -na
    naY,
    /// -nAwa
    nAwac,
    /// -Ayana
    Pak,
    /// -Ayana
    PaY,
    /// -Ayani
    PiY,
    /// -bahu
    bahuc,
    /// -biqa
    biqac,
    /// -birIsa
    birIsac,
    /// -Bakta
    Baktal,
    /// -Brawa
    Brawac,
    /// -ma
    ma,
    /// -mat
    matup,
    /// -ma
    map,
    /// -maya
    mayaw,
    /// -mAtra
    mAtrac,
    /// -pASa
    pASap,
    /// -piwa
    piwac,
    /// -ya
    ya,
    /// -ya
    yak,
    /// -ya
    yaY,
    /// -ya
    yat,
    /// -ya
    yan,
    /// -yu
    yus,
    /// -ra
    ra,
    /// -rUpa
    rUpap,
    /// -rhi
    rhil,
    /// -rUpya
    rUpya,
    /// -la
    lac,
    /// -vat
    vati,
    /// -vat
    vatup,
    /// -vaya
    vaya,
    /// -vala
    valac,
    /// -vin
    vini,
    /// -viDu
    viDal,
    /// -aka
    vuk,
    /// -aka
    vuY,
    /// -aka
    vun,
    /// -vya
    vyat,
    /// -vya
    vyan,
    /// -Sa
    Sa,
    /// -SaNkawa
    SaNkawac,
    /// -SAla
    SAlac,
    /// -Sas
    Sas,
    /// -za
    za,
    /// -ka
    zkan,
    /// -tara
    zwarac,
    /// -ika
    zWac,
    /// -ika
    zWan,
    /// -ika
    zWal,
    /// Ayana
    zPak,
    /// -ya
    zyaN,
    /// -ya
    zyaY,
    /// -sa
    sa,
    /// -sna
    sna,
    /// -sAt
    sAti,
    /// -s
    suc,
    /// -sna
    snaY,
    /// -ha
    ha,
}

py_enum!(
    PyTaddhita,
    Taddhita,
    [
        a, akac, ac, aWac, aR, aY, at, atasuc, anic, ap, asic, astAti, Akinic, Arak, iY, itac,
        inac, ini, imanic, ila, ilac, izWan, Ikak, Ikan, Iyasun, eRya, Erak, ka, kak, kawac, kan,
        kap, kalpap, kftvasuc, kuwArac, kuRap, Ka, KaY, Ga, Gac, Gan, Gas, caRap, caraw, cuYcup,
        cPaY, cvi, Ca, CaR, Cas, jAtIyar, jAhac, Ya, YiW, Yya, YyaN, Yyaw, wac, waq, wiWan, wIwac,
        weRyaR, wyaR, wyu, wyul, wlaY, Wak, Wac, WaY, Wan, Wap, qaw, qati, qatarac, qatamac, qupac,
        qmatup, qyaR, qvalac, qvun, Qak, QakaY, Qa, QaY, Qinuk, Qrak, Ra, Rini, Rya, tamap, tayap,
        tarap, tal, tasi, tasil, ti, tikan, tIya, tyak, tyakan, tyap, tyu, tyul, tral, trA, tva,
        Tamu, Tyan, TAl, daGnac, dA, dAnIm, deSya, deSIyar, dvayasac, DA, na, naY, nAwac, Pak, PaY,
        PiY, bahuc, biqac, birIsac, Baktal, Brawac, ma, matup, map, mayaw, mAtrac, pASap, piwac,
        ya, yak, yaY, yat, yan, yus, ra, rUpap, rhil, rUpya, lac, vati, vatup, vaya, valac, vini,
        viDal, vuk, vuY, vun, vyat, vyan, Sa, SaNkawac, SAlac, Sas, za, zkan, zwarac, zWac, zWan,
        zWal, zPak, zyaN, zyaY, sa, sna, sAti, suc, snaY, ha
    ]
);

/// The lakara to use in the derivation.
#[pyclass(name = "Lakara", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PyLakara {
    /// Describes action in the present tense. Ssometimes called the *present indicative*.
    Lat,
    /// Describes unwitnessed past action. Sometimes called the *perfect*.
    Lit,
    /// Describes future action after the current day. Sometimes called the *periphrastic future*.
    Lut,
    /// Describes general future action. Sometimes called the *simple future*.
    Lrt,
    /// The Vedic subjunctive. `vidyut-prakriya` currently has poor support for this lakara.
    Let,
    /// Describes commands. Sometimes called the *imperative*.
    Lot,
    /// Describes past action before the current day. Sometimes called the *imperfect*.
    Lan,
    /// Describes potential or hypothetical actions. Sometimes called the *optative*.
    VidhiLin,
    /// Describes wishes and prayers. Sometimes called the *benedictive*.
    AshirLin,
    /// Describes general past action. Sometimes called the *aorist*.
    Lun,
    /// Describes past counterfactuals ("would not have ..."). Sometimes called the *conditional*.
    Lrn,
}

py_enum!(
    PyLakara,
    Lakara,
    [Lat, Lit, Lut, Lrt, Let, Lot, Lan, VidhiLin, AshirLin, Lun, Lrn]
);

/// The linga to use in the derivation.
#[pyclass(name = "Linga", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PyLinga {
    /// The masculine.
    Pum,
    /// The feminine.
    Stri,
    /// The neuter.
    Napumsaka,
}

py_enum!(PyLinga, Linga, [Pum, Stri, Napumsaka]);

/// The prayoga of some tinanta.
#[pyclass(name = "Prayoga", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PyPrayoga {
    /// Usage coreferent with the agent, e.g. "The horse *goes* to the village."
    Kartari,
    /// Usage coreferent with the object, e.g. "The village *is gone to* by the horse."
    Karmani,
    /// Usage without a referent, e.g. "*There is motion* by the horse to the village."
    /// bhAve prayoga generally produces the same forms as karmani prayoga.
    Bhave,
}

py_enum!(PyPrayoga, Prayoga, [Kartari, Karmani, Bhave]);

/// The person of some tinanta.
#[pyclass(name = "Purusha", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PyPurusha {
    /// The third person.
    Prathama,
    /// The second person.
    Madhyama,
    /// The first person.
    Uttama,
}

py_enum!(PyPurusha, Purusha, [Prathama, Madhyama, Uttama]);

#[allow(non_camel_case_types)]
#[pyclass(name = "Sanadi", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PySanadi {
    /// `san`, which creates desiderative roots per 3.1.7.
    ///
    /// Examples: buBUzati, ninIzati.
    san,
    /// `yaN`, which creates intensive roots per 3.1.22. For certain dhatus, the semantics are
    /// instead "crooked movement" (by 3.1.23) or "contemptible" action (by 3.1.24).
    ///
    /// Examples: boBUyate, nenIyate.
    ///
    /// Constraints: can be used only if the dhatu starts with a consonant and has exactly one
    /// vowel. If this constraint is violated, our APIs will return an `Error`.
    yaN,
    /// `yaN` with `luk` (unused)
    yaNluk,
    /// `Nic`, which creates causal roots per 3.1.26.
    ///
    /// Examples: BAvayati, nAyayati.
    Ric,
    /// TODO
    kAmyac,
    /// TODO
    kyaN,
    /// TODO
    kyac,
}

py_enum!(
    PySanadi,
    Sanadi,
    [san, yaN, yaNluk, Ric, kAmyac, kyaN, kyac]
);

/// The number of some tinanta or subanta.
#[pyclass(name = "Vacana", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PyVacana {
    /// The singular.
    Eka,
    /// The dual.
    Dvi,
    /// The plural.
    Bahu,
}

py_enum!(PyVacana, Vacana, [Eka, Dvi, Bahu]);

/// The case ending of some subanta.
#[pyclass(name = "Vibhakti", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum PyVibhakti {
    /// The first vibhakti. Sometimes called the *nominative case*.
    Prathama,
    /// The second vibhakti. Sometimes called the *accusative case*.
    Dvitiya,
    /// The third vibhakti. Sometimes called the *instrumental case*.
    Trtiya,
    /// The fourth vibhakti. Sometimes called the *dative case*.
    Caturthi,
    /// The fifth vibhakti. Sometimes called the *ablative case*.
    Panchami,
    /// The sixth vibhakti. Sometimes called the *genitive case*.
    Sasthi,
    /// The seventh vibhakti. Sometimes called the *locative case*.
    Saptami,
    /// The first vibhakti used in the sense of *sambodhana*. Sometimes called the *vocative case*.
    ///
    /// *Sambodhana* is technically not a *vibhakti but rather an additional semantic condition
    /// that conditions the first vibhakti. But we felt that users would find it more convenient to
    /// have this condition available on `Vibhakti` directly rather than have to define the
    /// *sambodhana* condition separately.
    Sambodhana,
}

py_enum!(
    PyVibhakti,
    Vibhakti,
    [Prathama, Dvitiya, Trtiya, Caturthi, Panchami, Sasthi, Saptami, Sambodhana]
);

/// A verb root.
#[pyclass(name = "Dhatu", module = "prakriya", eq, ord)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PyDhatu(Dhatu);

impl PyDhatu {
    pub(crate) fn as_rust(&self) -> &Dhatu {
        &self.0
    }
}

#[pymethods]
impl PyDhatu {
    /// Creates a new mula-dhatu.
    ///
    /// `aupadeshika` should be an SLP1 string and include anudatta and svarita accents as
    /// necessary. For examples, see the dhatus in `dhatupatha.tsv`. (If you don't have this file,
    /// see the `vidyut-py` documentation for information on where to find it.)
    #[staticmethod]
    #[pyo3(signature = (aupadeshika, gana, *, antargana=None, prefixes=None, sanadi=None))]
    pub fn mula(
        aupadeshika: String,
        gana: PyGana,
        antargana: Option<PyAntargana>,
        prefixes: Option<Vec<String>>,
        sanadi: Option<Vec<PySanadi>>,
    ) -> PyResult<Self> {
        let mut builder = Dhatu::builder().aupadeshika(&aupadeshika).gana(gana.into());
        if let Some(a) = antargana {
            builder = builder.antargana(a.into());
        }
        if let Some(ps) = prefixes {
            builder = builder.prefixes(&ps);
        }
        if let Some(ss) = sanadi {
            let sanadis: Vec<Sanadi> = ss.iter().map(|s| (*s).into()).collect();
            builder = builder.sanadi(&sanadis);
        }

        let dhatu = match builder.build() {
            Ok(dhatu) => dhatu,
            Err(_) => {
                return Err(PyValueError::new_err(format!(
                    "{aupadeshika} must be an SLP1 string."
                )))
            }
        };
        Ok(PyDhatu(dhatu))
    }

    /// Creates a new namadhatu with its *sanadi pratyaya*.
    ///
    /// If `nama_sanadi` is `None`, the program will try finding a sanadi match by appling the
    /// rules in pada 3.1 of the Ashtadhyayi. If no match is found, the prakriya will abort.
    #[staticmethod]
    #[pyo3(signature = (pratipadika, *, nama_sanadi=None))]
    pub fn nama(pratipadika: PyPratipadika, nama_sanadi: Option<PySanadi>) -> Self {
        PyDhatu(Dhatu::nama(
            pratipadika.as_ref().clone(),
            nama_sanadi.map(|s| s.into()),
        ))
    }

    /// Shorthand for modifying an existing dhatu with new prefixes.
    pub fn with_prefixes(&self, prefixes: Vec<String>) -> Self {
        self.0.clone().with_prefixes(&prefixes).into()
    }

    /// Shorthand for modifying an existing dhatu with new sanadis.
    pub fn with_sanadi(&self, sanadi: Vec<PySanadi>) -> Self {
        let sanadi: Vec<Sanadi> = sanadi.iter().map(|x| (*x).into()).collect();
        self.0.clone().with_sanadi(&sanadi).into()
    }

    /// The aupadeshika form of this dhatu.
    #[getter]
    pub fn aupadeshika(&self) -> String {
        match self.0.aupadeshika() {
            Some(s) => s.to_string(),
            None => String::new(),
        }
    }

    /// The gana that this dhatu belongs to.
    #[getter]
    pub fn gana(&self) -> Option<PyGana> {
        self.0.gana().map(|g| g.into())
    }

    /// The antargana that this dhatu belongs to.
    #[getter]
    pub fn antargana(&self) -> Option<PyAntargana> {
        self.0.antargana().map(|g| g.into())
    }

    /// The prefixes that this dhatu uses.
    #[getter]
    pub fn prefixes(&self) -> Vec<String> {
        self.0.prefixes().to_vec()
    }

    /// The sanadi pratyayas that this dhatu uses.
    #[getter]
    pub fn sanadi(&self) -> Vec<PySanadi> {
        self.0.sanadi().iter().map(|x| (*x).into()).collect()
    }

    pub fn __repr__(&self) -> String {
        let mut args = String::new();
        args.push_str(&format!("aupadeshika='{}'", self.aupadeshika()));

        if let Some(g) = self.gana() {
            args.push_str(&format!(", gana={}", g.__repr__()));
        }

        if let Some(a) = self.antargana() {
            args.push_str(&format!(", antargana={}", a.__repr__()));
        }

        if !self.prefixes().is_empty() {
            args.push_str(", prefixes=[");
            let mut pushed = false;
            for p in self.prefixes() {
                if pushed {
                    args.push_str(", ");
                }
                args.push_str(&format!("'{}'", p));
                pushed = true;
            }
            args.push(']');
        }

        if !self.sanadi().is_empty() {
            args.push_str(", sanadi=[");
            let mut pushed = false;
            for s in self.sanadi() {
                if pushed {
                    args.push_str(", ");
                }
                args.push_str(&s.__repr__());
                pushed = true;
            }
            args.push(']');
        }

        format!("Dhatu({})", args)
    }
}

impl PyDhatu {
    pub fn as_ref(&self) -> &Dhatu {
        &self.0
    }
}

impl From<Dhatu> for PyDhatu {
    fn from(val: Dhatu) -> Self {
        Self(val)
    }
}

impl From<&Dhatu> for PyDhatu {
    fn from(val: &Dhatu) -> Self {
        Self(val.clone())
    }
}

impl From<PyDhatu> for Dhatu {
    fn from(val: PyDhatu) -> Self {
        val.0
    }
}

/// A nominal stem.
#[pyclass(name = "Pratipadika", module = "prakriya", eq, ord)]
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PyPratipadika {
    pratipadika: Pratipadika,
    pub(crate) text: String,
}

impl PyPratipadika {
    pub fn as_ref(&self) -> &Pratipadika {
        &self.pratipadika
    }
}

#[pymethods]
impl PyPratipadika {
    pub fn __repr__(&self) -> String {
        match &self.pratipadika {
            Pratipadika::Basic(_) => format!("Pratipadika(text='{}')", self.text),
            _ => "Pratipadika(...)".to_string(),
        }
    }

    /// Create a new pratipadika that is a simple base.
    ///
    /// `text` should be an SLP1 string.
    #[staticmethod]
    #[pyo3(signature = (text))]
    pub fn basic(text: String) -> PyResult<Self> {
        let safe = match Slp1String::from(text.clone()) {
            Ok(s) => s,
            Err(_) => {
                return Err(PyValueError::new_err(format!(
                    "{text} must be an SLP1 string."
                )))
            }
        };
        Ok(Self {
            pratipadika: Pratipadika::basic(safe),
            text,
        })
    }

    /// Create a new pratipadika that is a krdanta.
    #[staticmethod]
    #[pyo3(signature = (dhatu, krt))]
    pub fn krdanta(dhatu: PyDhatu, krt: PyKrt) -> Self {
        let krdanta = Krdanta::new(dhatu.into(), BaseKrt::from(krt));
        Self {
            pratipadika: Pratipadika::Krdanta(krdanta.into()),
            text: "".to_string(),
        }
    }

    /// Create a new pratipadika that is a taddhitanta.
    #[staticmethod]
    #[pyo3(signature = (pratipadika, taddhita))]
    pub fn taddhitanta(pratipadika: PyPratipadika, taddhita: PyTaddhita) -> Self {
        let taddhitanta = Taddhitanta::new(pratipadika.pratipadika, taddhita.into());
        Self {
            pratipadika: Pratipadika::Taddhitanta(taddhitanta.into()),
            text: "".to_string(),
        }
    }

    /// The text of this stem.
    #[getter]
    pub fn text(&self) -> Option<String> {
        match &self.pratipadika {
            Pratipadika::Basic(_) => Some(self.text.clone()),
            _ => None,
        }
    }
}

impl From<Pratipadika> for PyPratipadika {
    fn from(val: Pratipadika) -> PyPratipadika {
        let text = match val {
            Pratipadika::Basic(ref b) => b.text().to_string(),
            _ => String::new(),
        };
        PyPratipadika {
            pratipadika: val,
            text,
        }
    }
}

/// A Sanskrit pada.
///
/// Notes for `Pada.Tinanta`:
///
/// - If `skip_at_agama` is ``True`` and the `lakara` is `Lun`, `Lan`, or `Lrn`,
///   then the derivation will not add the *aṭ*/*āṭ* *āgama* to the verb. This
///   is to derive forms like *gamat*, *karot*, etc.
#[pyclass(name = "Pada", module = "prakriya", eq, ord)]
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum PyPada {
    #[pyo3(constructor = (pratipadika, linga, vibhakti, vacana, *, is_avyaya = false))]
    Subanta {
        pratipadika: PyPratipadika,
        linga: PyLinga,
        vibhakti: PyVibhakti,
        vacana: PyVacana,
        is_avyaya: bool,
    },

    #[pyo3(constructor = (dhatu, prayoga, lakara, purusha, vacana, *, skip_at_agama = false))]
    Tinanta {
        dhatu: PyDhatu,
        prayoga: PyPrayoga,
        lakara: PyLakara,
        purusha: PyPurusha,
        vacana: PyVacana,
        skip_at_agama: bool,
    },
}

#[pymethods]
impl PyPada {
    #[staticmethod]
    pub fn make_avyaya(pratipadika: PyPratipadika) -> Self {
        Self::Subanta {
            pratipadika: pratipadika.clone(),
            linga: PyLinga::Pum,
            vibhakti: PyVibhakti::Prathama,
            vacana: PyVacana::Eka,
            is_avyaya: true,
        }
    }
}
