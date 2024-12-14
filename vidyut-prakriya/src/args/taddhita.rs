use crate::args::Pratipadika;
use crate::core::errors::*;
use crate::enum_boilerplate;
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The complete list of *taddhita pratyaya*s.
///
/// Rust's naming convention is to start enum values with capital letters. However, we allow mixed
/// case explicitly here so that we can name pratyayas more concisely with SLP1. Doing so helps us
/// distinguish between pratyayas like `naN` and `nan`.
#[allow(dead_code, non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[wasm_bindgen]
pub enum Taddhita {
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

enum_boilerplate!(Taddhita, {
    a => "a",
    akac => "akac",
    ac => "ac",
    aWac => "aWac",
    aY => "aY",
    aR => "aR",
    at => "at",
    atasuc => "atasu~c",
    anic => "ani~c",
    ap => "ap",
    asic => "asi~c",
    astAti => "astAti~",
    Akinic => "Akini~c",
    Arak => "Arak",
    iY => "iY",
    itac => "itac",
    inac => "inac",
    ini => "ini~",
    imanic => "imani~c",
    ila => "ila",
    ilac => "ilac",
    izWan => "izWan",
    Ikak => "Ikak",
    Ikan => "Ikan",
    Iyasun => "Iyasu~n",
    eRya => "eRya",
    Erak => "Erak",
    ka => "ka",
    kak => "kak",
    kawac => "kawac",
    kap => "kap",
    kan => "kan",
    kalpap => "kalpap",
    kftvasuc => "kftvasu~c",
    kuwArac => "kuwArac",
    kuRap => "kuRap",
    Ka => "Ka",
    KaY => "KaY",
    Ga => "Ga",
    Gac => "Gac",
    Gan => "Gan",
    Gas => "Gas",
    caRap => "caRap",
    caraw => "caraw",
    cuYcup => "cuYcup",
    cPaY => "cPaY",
    cvi => "cvi~",
    Ca => "Ca",
    CaR => "CaR",
    Cas => "Cas",
    jAtIyar => "jAtIyar",
    jAhac => "jAhac",
    Ya => "Ya",
    YiW => "YiW",
    Yya => "Yya",
    YyaN => "YyaN",
    Yyaw => "Yyaw",
    wac => "wac",
    waq => "waq",
    wiWan => "wi~Wan",
    wIwac => "wIwac",
    weRyaR => "weRyaR",
    wyaR => "wyaR",
    wyu => "wyu~",
    wyul => "wyu~l",
    wlaY => "wlaY",
    Wak => "Wak",
    Wac => "Wac",
    WaY => "WaY",
    Wan => "Wan",
    Wap => "Wap",
    qaw => "qaw",
    qati => "qati",
    qatarac => "qatarac",
    qatamac => "qatamac",
    qupac => "qupac",
    qmatup => "qmatu~p",
    qyaR => "qyaR",
    qvalac => "qvalac",
    qvun => "qvu~n",
    Qak => "Qak",
    QakaY => "QakaY",
    Qa => "Qa",
    QaY => "QaY",
    Qinuk => "Qinu~k",
    Qrak => "Qrak",
    Ra => "Ra",
    Rini => "Rini~",
    Rya => "Rya",
    tamap => "tamap",
    tayap => "tayap",
    tarap => "tarap",
    tal => "tal",
    tasi => "tasi~",
    tasil => "tasi~l",
    ti => "ti",
    tikan => "tikan",
    tIya => "tIya",
    tyak => "tyak",
    tyakan => "tyakan",
    tyap => "tyap",
    tyu => "tyu~",
    tyul => "tyu~l",
    tral => "tral",
    trA => "trA",
    tva => "tva",
    Tamu => "Tamu~",
    Tyan => "Tyan",
    TAl => "TAl",
    daGnac => "daGnac",
    dvayasac => "dvayasac",
    dA => "dA",
    dAnIm => "dAnIm",
    deSya => "deSya",
    deSIyar => "deSIyar",
    DA => "DA",
    na => "na",
    nAwac => "nAwac",
    naY => "naY",
    ma => "ma",
    matup => "matu~p",
    map => "map",
    mayaw => "mayaw",
    pASap => "pASap",
    piwac => "piwac",
    Pak => "Pak",
    PaY => "PaY",
    PiY => "PiY",
    bahuc => "bahuc",
    biqac => "biqac",
    birIsac => "birIsac",
    Baktal => "Baktal",
    Brawac => "Brawac",
    mAtrac => "mAtrac",
    ya => "ya",
    yak => "yak",
    yaY => "yaY",
    yat => "yat",
    yan => "yan",
    yus => "yus",
    ra => "ra",
    rhil => "rhil",
    rUpya => "rUpya",
    lac => "lac",
    rUpap => "rUpap",
    vati => "vati~",
    vatup => "vatu~p",
    vaya => "vaya",
    valac => "valac",
    viDal => "viDal",
    vini => "vini~",
    vuk => "vu~k",
    vuY => "vu~Y",
    vun => "vu~n",
    vyat => "vyat",
    vyan => "vyan",
    Sa => "Sa",
    SaNkawac => "SaNkawac",
    SAlac => "SAlac",
    Sas => "Sas",
    za => "za",
    zkan => "zkan",
    zwarac => "zwarac",
    zWac => "zWac",
    zWan => "zWan",
    zWal => "zWal",
    zPak => "zPak",
    zyaN => "zyaN",
    zyaY => "zyaY",
    sa => "sa",
    sna => "sna",
    sAti => "sAti~",
    suc => "su~c",
    snaY => "snaY",
    ha => "ha",
});

impl Taddhita {
    /// Returns the *aupadeśika* form of this *pratyaya*.
    pub fn aupadeshika(&self) -> &'static str {
        self.as_str()
    }
}

/// Models the meaning of a *taddhita pratyaya*.
///
/// Generally, *taddhita*s are available only in specific senses. A given *taddhita* might be
/// allowed in one sense but blocked in another. To model and test this behavior, we use the enum
/// below.
#[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TaddhitaArtha {
    /// Descendant. (4.1.92)
    TasyaApatyam,
    /// Patronymic lineage. (4.1.98)
    Gotra,
    /// A class. (4.1.161)
    Jatau,
    /// Country. (4.1.168)
    Janapada,
    /// One dyed by this. (4.2.1)
    TenaRaktam,
    /// Food that is prepared with this vessel or medium. (4.2.16)
    SamskrtamBhaksha,
    /// Having this as a god. (4.2.24)
    SaAsyaDevata,
    /// A collection of this. (4.2.37)
    TasyaSamuha,
    /// A domain of this. (4.2.52)
    TasyaVishayoDeshe,
    /// What one studies or knows. (4.2.59)
    TadAdhiteTadVeda,
    /// Country, by entities present (4.2.67), creator (4.2.68), dwelling place (4.2.69), or nearby
    /// features (4.2.70).
    Caturarthika,
    /// Miscellaneous. (4.2.92)
    Sheshe,
    /// Where this was born. (4.3.25)
    TatraJata,
    /// Where this was made, obtained, bought, or is skillful (4.3.38)
    TatraKrtaLabdhaKritaKushala,
    /// Where this generally occurs. (4.3.39)
    TatraPrayabhava,
    /// Where this was adapted. (4.3.41)
    TatraSambhute,
    /// Where this stays or exists. (4.3.53)
    TatraBhava,
    /// Where one came from. (4.3.74)
    TataAgata,
    /// Dwelling. (4.3.89)
    AsyaNivasa,
    /// Devotion. (4.3.95)
    Bhakti,
    /// What is proclaimed by this person. (4.3.101)
    TenaProktam,
    /// By whom this is made. (4.3.116)
    TenaKrte,
    /// One who has this. (4.3.120)
    TasyaIdam,
    /// Modification of this. (4.3.134)
    TasyaVikara,
    /// One who gambles, conquers, or is conquered with this. (4.4.2)
    TenaDivyatiJayatiJitam,
    /// Cultured or processed. (4.4.3)
    TenaSamskrtam,
    /// One who crosses with this. (4.4.5)
    TenaTarati,
    /// One who walks or eats with this. (4.4.8)
    TenaCarati,
    /// One who lives with this. (4.4.12)
    TenaJivati,
    /// One who conveys with this. (4.4.15)
    TenaHarati,
    /// Completed with. (4.4.19)
    TenaNirvrtte,
    /// Mixed with. (4.4.22)
    TenaSamsrshte,
    /// Sprinkled with. (4.4.26)
    TenaUpasikte,
    /// Existence. (4.4.27)
    Vartate,
    /// One who gives in a contemptible way. (4.4.30)
    PrayacchatiGarhyam,
    /// What is gleaned. (4.4.32)
    Unchati,
    /// What is protected. (4.4.33)
    TadRakshati,
    /// Who does this. (4.4.34)
    Karoti,
    /// What one kills. (4.4.35)
    Hanti,
    /// What one takes. (4.4.39)
    Grhnati,
    /// What one practices. (4.4.41)
    Carati,
    /// Who goes. (4.4.42)
    Eti,
    /// Assembly. (4.4.43)
    Samavaiti,
    /// What one sees. (4.4.46)
    Pashyati,
    /// One who has this custom. (4.4.47)
    TasyaDharmyam,
    /// One who sells this item. (4.4.51)
    TadAsyaPanyam,
    /// One who has this weapon. (4.4.57)
    Praharanam,
    /// What is taxed. (4.4.50)
    Avakraya,
    /// One who has this art. (4.4.55)
    Shilpam,
    /// One who has this belief. (4.4.60)
    Mati,
    /// One who has this habit. (4.4.61)
    Shilam,
    /// One who has this act in their studies. (4.4.63)
    KarmaAdhyayaneVrttam,
    /// One for whom this is a good diet. (4.4.65)
    HitamBhaksha,
    /// To whom this is given rightfully. (4.4.66)
    TadAsmaiDiyateNiyuktam,
    /// Where one is appointed. (4.4.69)
    Niyuktam,
    /// Where one does work. (4.4.72)
    Vyavaharati,
    /// Who dwells here. (4.4.73)
    Vasati,
    /// What one bears. (4.4.78)
    TadVahati,
    /// What one pierces. (4.4.83)
    TadVidhyati,
    /// What one has obtained. (4.4.85)
    Labdha,
    /// Where one has gone. (4.4.86)
    Gata,
    /// In which this is seen. (4.4.87)
    AsminDrshyam,
    /// Whose root is removed. (4.4.88)
    AsyaAbarhi,
    /// Joined with this. (4.4.90)
    Samyukta,
    /// Not deviating from this. (4.4.92)
    Anapeta,
    /// Made of this. (4.4.93)
    Nirmita,
    /// Dear to this. (4.4.95)
    Priya,
    /// Skilled in this. (4.4.98)
    TatraSadhu,
    /// Living there. (4.4.107)
    TatraVasi,
    /// One for whom this is good. (5.1.7)
    TasmaiHitam,
    /// What could be created from this raw material. (5.1.12)
    TadarthamVikrtehPrakrtau,
    /// What one could be connected to or be in. (5.1.16)
    TadAsyaTadAsminSyat,
    /// What is bought with this. (5.1.37)
    TenaKritam,
    /// For the sake of this, when the meaning is a relation or appearance. (5.1.38)
    TasyaNimittamSamyogotpattau,
    /// Sown with this. (5.1.45)
    TasyaVapa,
    /// In which this is an interest, rent, profit, tax, or bribe. (5.1.47)
    TadAsminVrddhiAyaLabhaSulkaUpada,
    /// What one carries away, or conveys, or brings. (5.1.50)
    TadDharatiVahatiAvahati,
    /// What is capable of this, or holds this, or cooks this. (5.152)
    SambhavatiAharatiPacati,
    /// The measure of this. (5.1.57)
    TadAsyaParimanam,
    /// One deserves this. (5.1.63)
    TadArhati,
    /// One performs this. (5.1.72)
    TadVartayati,
    /// Fallen into this. (5.1.73)
    Apanna,
    /// Who goes. (5.1.75)
    Gacchati,
    /// Who deserves approach (5.1.74 vArttika)
    AbhigamanamArhati,
    /// What is conveyed by that way. (5.1.77)
    Ahrtam,
    /// Completed by this time. (5.1.79)
    TenaNirvrttam,
    /// For which time one instructs, is paid, existed, or will exist. (5.1.80)
    TamAdhisteBhrtoBhutoBhavi,
    /// Age. (5.1.81)
    Vayasi,
    /// The existence of which. (5.1.119)
    TasyaBhava,
    /// A place of growing, when that place is a field. (5.2.1)
    DhanyanamBhavaneKshetre,
    /// Wholly made of this. (5.2.5)
    Krta,
    /// Mirror. (5.2.6)
    Darshana,
    /// Pervades. (5.2.7)
    Vyapnoti,
    /// Obtains. (5.2.8)
    Prapnoti,
    /// Who witnesses this. (5.2.10)
    TadAnubhavati,
    /// Who intends to go. (5.2.11)
    Gami,
    /// Fit to go. (5.2.15)
    AlamGami,
    /// Formerly this. (5.2.18)
    BhutaPurva,
    /// What can be traveled in one day. (5.2.19)
    EkahaGama,
    /// The ripening season of this. (5.2.24)
    TasyaPakamula,
    /// The root of this. (5.2.25)
    TasyaMula,
    /// Celebrated through this. (5.2.26)
    TenaVitta,
    /// Of which this is observed. (5.2.36)
    TadAsyaSamjatam,
    /// Measure. (5.2.37)
    TadAsyaPramanam,
    /// Volume. (5.2.39)
    Parimana,
    /// Parts of which. (5.2.42)
    Avasana,
    /// Given in exchange. (5.2.47)
    Nimana,
    /// Making full. (5.2.48)
    Purana,
    /// Skilled in this. (5.2.63)
    TatraKushala,
    /// A desire for which. (5.2.65)
    TatraKama,
    /// Voracious. (5.2.67)
    TatraAdyuna,
    /// Supplied richly with. (5.2.68)
    TatraParijata,
    /// Who must take this.. (5.2.69)
    Hari,
    /// Taken recently. (5.2.70)
    AciraApahrta,
    /// Going to work in this manner. (5.2.72)
    Karin,
    /// Who strives to gain by this. (5.2.75)
    Anvicchati,
    /// What one has or is in. (5.2.94)
    TadAsyaAstiAsmin,
    /// Words meaning direction, location, or time. (5.3.27)
    DigDeshaKala,
    /// Not known. (5.3.73)
    Ajnate,
    /// Contempt. (5.3.75)
    Kutsite,
    /// TODO
    Anukampayam,
    /// Slenderness. (5.3.91)
    Tanutve,
    /// One of two. (5.3.92)
    DvayorEka,
    /// One of many. (5.3.93)
    BahunamEka,
    /// Derision. (5.3.95)
    Avakshepane,
    /// TODO
    Alpe,
    /// TODO
    Hrasve,
    /// TODO
    IvePratikrtau,
    /// Those who make a living by arms. (5.3.114)
    AyudhaJiviSangha,
    /// TODO
    AnatyantaGati,
    /// TODO
    Acchadana,
    /// TODO
    Svarthe,
    /// TODO
    Matsye,
    /// TODO
    KriyaAbhyavrttiGanana,
    /// Expressing manner. (5.4.3)
    PrakaraVacane,
    /// What one is made of. (5.4.21)
    TatPrakrtaVacane,
    /// For the sake of which. (5.4.24)
    Tadarthye,
    /// Praise of this. (5.3.66, 5.4.41)
    Prashamsa,
    /// Becoming what one was not. (5.4.50)
    AbhutaTadbhava,
}

impl TaddhitaArtha {
    /// Returns whether `self` is either identical to `other` or falls under `other` as a subtype.
    pub fn is_type_of(&self, parent: Self) -> bool {
        match self {
            Self::Gotra => matches!(parent, Self::Gotra | Self::TasyaApatyam),
            _ => *self == parent,
        }
    }
}

/// The information required to derive a *taddhitānta*.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Taddhitanta {
    pratipadika: Pratipadika,
    taddhita: Taddhita,
    artha: Option<TaddhitaArtha>,
    require: Option<String>,
}

impl Taddhitanta {
    /// Defines a simple `Taddhitanta`.
    ///
    /// For more options, use `Taddhitanta::builder()` instead.
    pub fn new(pratipadika: Pratipadika, taddhita: Taddhita) -> Self {
        Self {
            pratipadika,
            taddhita,
            artha: None,
            require: None,
        }
    }
    /// Returns a new builder for this struct.
    pub fn builder() -> TaddhitantaBuilder {
        TaddhitantaBuilder::default()
    }

    /// The *prātipadika* to use in the derivation.
    pub fn pratipadika(&self) -> &Pratipadika {
        &self.pratipadika
    }

    /// The *taddhita pratyaya* to use in the derivation.
    pub fn taddhita(&self) -> Taddhita {
        self.taddhita
    }

    /// The artha condition to use in the derivation. If not set, any artha is allowed.
    pub fn artha(&self) -> Option<TaddhitaArtha> {
        self.artha
    }

    /// The value that the *kṛdanta* must match, if defined.
    pub fn require(&self) -> &Option<String> {
        &self.require
    }

    /// Sets the required value for this *taddhitānta*.
    pub fn with_require(mut self, s: impl AsRef<str>) -> Self {
        self.require = Some(s.as_ref().to_string());
        self
    }
}

/// Convenience struct for building a `TaddhitantaArgs` object.
#[derive(Clone, Default, Eq, Hash, PartialEq)]
pub struct TaddhitantaBuilder {
    pratipadika: Option<Pratipadika>,
    taddhita: Option<Taddhita>,
    artha: Option<TaddhitaArtha>,
    require: Option<String>,
}

impl TaddhitantaBuilder {
    /// Sets the pratipadika to use in the derivation.
    pub fn pratipadika(&mut self, val: Pratipadika) -> &mut Self {
        self.pratipadika = Some(val);
        self
    }

    /// Sets the taddhita-pratyaya to use in the derivation.
    pub fn taddhita(&mut self, val: Taddhita) -> &mut Self {
        self.taddhita = Some(val);
        self
    }

    /// Sets the artha to use in the derivation.
    pub fn artha(&mut self, val: TaddhitaArtha) -> &mut Self {
        self.artha = Some(val);
        self
    }

    /// Sets the value that the krdanta must have.
    pub fn require(&mut self, text: impl AsRef<str>) -> &mut Self {
        self.require = Some(text.as_ref().to_string());
        self
    }

    /// Converts the arguments in this builder into a `TaddhitantaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(&self) -> Result<Taddhitanta> {
        Ok(Taddhitanta {
            pratipadika: match &self.pratipadika {
                Some(x) => x.clone(),
                _ => return Err(Error::missing_required_field("pratipadika")),
            },
            taddhita: match self.taddhita {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("taddhita")),
            },
            artha: self.artha,
            require: self.require.clone(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artha_includes() {
        use TaddhitaArtha::*;
        // Child relationship --> true
        assert!(Gotra.is_type_of(TasyaApatyam));
        // Equality --> true
        assert!(TasyaApatyam.is_type_of(TasyaApatyam));
        // Parent relationship --> false
        assert!(!TasyaApatyam.is_type_of(Gotra));
    }
}
