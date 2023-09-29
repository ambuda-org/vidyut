use crate::enum_boilerplate;
use crate::errors::*;
use wasm_bindgen::prelude::wasm_bindgen;

/// The complete list of taddhita-pratyayas.
///
/// Rust's naming convention is to start enum values with capital letters. However, we allow mixed
/// case explicitly here so that we can name pratyayas more concisely with SLP1. Doing so helps us
/// distinguish between pratyayas like `naN` and `nan`.
#[allow(dead_code, non_camel_case_types)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
#[wasm_bindgen]
pub enum Taddhita {
    /// a
    a,
    /// -aka
    akac,
    /// -a
    ac,
    /// -a
    aR,
    /// -a
    aY,
    /// -a
    at,
    /// -atas
    atasuc,
    /// -astAt
    astAti,
    /// -Ara
    Arak,
    /// -i
    iY,
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
    /// -Ika,
    Ikak,
    /// -Iyas
    Iyasun,
    /// -eRya
    eRya,
    /// -Era
    Erak,
    /// -ka
    kak,
    /// -ka
    kan,
    /// -kalpa
    kalpap,
    /// -kftvas
    kftvasuc,
    /// -ka
    ka,
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
    /// -Ayana
    cPaY,
    /// -Iya
    Ca,
    /// -Iya,
    CaR,
    /// -Iya,
    Cas,
    /// -a,
    Ya,
    /// -ika
    YiW,
    /// -ya,
    YyaN,
    /// -ya
    Yya,
    /// -a
    waq,
    /// -iWa
    wiWan,
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
    /// -mat
    qmatup,
    /// -vala
    qvalac,
    /// -aka
    qvun,
    /// -eya
    Qak,
    /// -eyaka
    QakaY,
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
    /// -tya
    tyak,
    /// -tya
    tyap,
    /// -tra
    tral,
    /// -tva
    tva,
    /// -Tam
    Tamu,
    /// -Tya
    Tyan,
    /// -TA
    TAl,
    /// -dA
    dA,
    /// -dAnIm
    dAnIm,
    /// -deSya
    deSya,
    /// -deSIya
    deSIyar,
    /// -dhA
    DA,
    /// -na
    na,
    /// -na
    naY,
    /// -Ayana
    Pak,
    /// -Ayana
    PaY,
    /// -Ayani
    PiY,
    /// -Bakta
    Baktal,
    /// -ma
    ma,
    /// -mat
    matup,
    /// -ma
    map,
    /// -maya
    mayaw,
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
    /// -la
    lac,
    /// -vaya
    vaya,
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
    /// -Sas
    Sas,
    /// -ika
    zWac,
    /// -ika
    zWan,
    /// -ika
    zWal,
    /// Ayana
    zPak,
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
    aY => "aY",
    aR => "aR",
    at => "at",
    atasuc => "atasu~c",
    astAti => "astAti~",
    Arak => "Arak",
    iY => "iY",
    ini => "ini~",
    imanic => "imani~c",
    ila => "ila",
    ilac => "ilac",
    izWan => "izWan",
    Ikak => "Ikak",
    Iyasun => "Iyasu~n",
    eRya => "eRya",
    Erak => "Erak",
    kan => "kan",
    kak => "kak",
    kalpap => "kalpap",
    kftvasuc => "kftvasu~c",
    ka => "ka",
    Ka => "Ka",
    KaY => "KaY",
    Ga => "Ga",
    Gac => "Gac",
    Gan => "Gan",
    Gas => "Gas",
    cPaY => "cPaY",
    Ca => "Ca",
    CaR => "CaR",
    Cas => "Cas",
    Ya => "Ya",
    YiW => "YiW",
    Yya => "Yya",
    YyaN => "Yyan",
    waq => "waq",
    wiWan => "wi~Wan",
    wyaR => "wyaR",
    wyu => "wyu~",
    wyul => "wyu~l",
    wlaY => "wlaY",
    Wak => "Wak",
    Wac => "Wac",
    WaY => "WaY",
    Wan => "Wan",
    Wap => "Wap",
    qmatup => "qmatu~p",
    qvalac => "qvalac",
    qvun => "qvu~n",
    Qak => "Qak",
    QakaY => "QakaY",
    QaY => "QaY",
    Qinuk => "Qinu~k",
    Qrak => "Qrak",
    Ra => "Ra",
    Rini => "Rini~",
    Rya => "Rya",
    tamap => "tamap",
    tarap => "tarap",
    tal => "tal",
    tasi => "tasi~",
    tasil => "tasi~l",
    ti => "ti",
    tikan => "tikan",
    tyak => "tyak",
    tyap => "tyap",
    tral => "tral",
    tva => "tva",
    Tamu => "Tamu~",
    Tyan => "Tyan",
    TAl => "TAl",
    dA => "dA",
    dAnIm => "dAnIm",
    deSya => "deSya",
    deSIyar => "deSIyar",
    DA => "DA",
    na => "na",
    naY => "naY",
    ma => "ma",
    matup => "matu~p",
    map => "map",
    mayaw => "mayaw",
    Pak => "Pak",
    PaY => "PaY",
    PiY => "PiY",
    Baktal => "Baktal",
    ya => "ya",
    yak => "yak",
    yaY => "yaY",
    yat => "yat",
    yan => "yan",
    yus => "yus",
    ra => "ra",
    rhil => "rhil",
    lac => "lac",
    rUpap => "rUpap",
    viDal => "viDal",
    vaya => "vaya",
    vini => "vini~",
    vuk => "vu~k",
    vuY => "vu~Y",
    vun => "vu~n",
    vyat => "vyat",
    vyan => "vyan",
    Sa => "Sa",
    Sas => "Sas",
    zWac => "zWac",
    zWan => "zWan",
    zWal => "zWal",
    zPak => "zPak",
    sAti => "sAti~",
    suc => "su~c",
    snaY => "snaY",
    ha => "ha",
});

/// Models the meaning of a taddhita.
///
/// Generally, taddhitas are available only in specific senses. A given taddhita might be allowed
/// in one sense but blocked in another. To model and test this behavior, we use the enum below.
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Artha {
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
    /// Who does this. ($.4.34)
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
    /// One for whom this is good. (5.1.7)
    TasmaiHitam,
    /// What could be created from this raw material. (5.1.12)
    TadarthamVikrtehPrakrtau,
    /// What one could be connected to or be in. (5.1.16)
    TadAsyaTadAsminSyat,
    /// What is bought with this. (5.1.37)
    TenaKritam,
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
    /// What one has or is in. (5.2.94)
    TadAsyaAstiAsmin,
    /// Words meaning direction, location, or time. (5.3.27)
    DigDeshaKala,
    /// Expressing manner. (5.4.3)
    PrakaraVacane,
    /// What one is made of. (5.4.21)
    TatPrakrtaVacane,
    /// For the sake of which. (5.4.24)
    Tadarthye,
}

impl Artha {
    /// Returns whether `self` is either identical to `other` or falls under `other` as a subtype.
    pub fn is_type_of(&self, parent: Self) -> bool {
        match self {
            Self::Gotra => matches!(parent, Self::Gotra | Self::TasyaApatyam),
            _ => *self == parent,
        }
    }
}

/// The information required to derive a taddhitanta in the grammar.
pub struct TaddhitantaArgs {
    taddhita: Taddhita,
    artha: Option<Artha>,
}

impl TaddhitantaArgs {
    /// The taddhita-pratyaya to use in the derivation.
    pub fn taddhita(&self) -> Taddhita {
        self.taddhita
    }

    /// The artha condition to use in the derivation. If not set, any artha is allowed.
    pub fn artha(&self) -> Option<Artha> {
        self.artha
    }

    /// Returns a new builder for this struct.
    pub fn builder() -> TaddhitantaArgsBuilder {
        TaddhitantaArgsBuilder::default()
    }
}

/// Convenience struct for building a `TaddhitantaArgs` object.
#[derive(Clone, Default, Hash, Eq, PartialEq)]
pub struct TaddhitantaArgsBuilder {
    taddhita: Option<Taddhita>,
    artha: Option<Artha>,
}

impl TaddhitantaArgsBuilder {
    /// Sets the taddhita-pratyaya to use in the derivation.
    pub fn taddhita(&mut self, val: Taddhita) -> &mut Self {
        self.taddhita = Some(val);
        self
    }

    /// Sets the artha to use in the derivation.
    pub fn artha(&mut self, val: Artha) -> &mut Self {
        self.artha = Some(val);
        self
    }

    /// Converts the arguments in this builder into a `TaddhitantaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(&self) -> Result<TaddhitantaArgs> {
        Ok(TaddhitantaArgs {
            taddhita: match self.taddhita {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("taddhita")),
            },
            artha: self.artha,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn artha_includes() {
        use Artha::*;
        // Child relationship --> true
        assert!(Gotra.is_type_of(TasyaApatyam));
        // Equality --> true
        assert!(TasyaApatyam.is_type_of(TasyaApatyam));
        // Parent relationship --> false
        assert!(!TasyaApatyam.is_type_of(Gotra));
    }
}
