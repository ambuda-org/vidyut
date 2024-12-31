use crate::args::{DhatuPada, Purusha, Vacana, Vibhakti};
use crate::core::errors::Error;

macro_rules! internal_term {
    ($Enum:ident, { $( $variant:ident => $str:literal ),* $(,)? }) => {
        #[allow(non_camel_case_types)]
        #[allow(clippy::upper_case_acronyms)]
        #[allow(unused)]
        #[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
        pub enum $Enum {
            $(
                $variant,
            )*
        }

        #[allow(unused)]
        impl $Enum {
            pub fn as_str(&self) -> &'static str {
                self.aupadeshika()
            }

            pub fn aupadeshika(&self) -> &'static str {
                match self {
                    $(
                        $Enum::$variant => $str,
                    )*
                }
            }
        }

        impl std::str::FromStr for $Enum {
            type Err = $crate::core::errors::Error;

            fn from_str(value: &str) -> $crate::core::errors::Result<Self> {
                let ret = match value {
                    $(
                        $str => $Enum::$variant,
                    )*
                    _ => return Err(Error::enum_parse_error(value))
                };
                Ok(ret)
            }
        }
    }
}

macro_rules! internal_term_non_unique {
    ($Enum:ident, { $( $variant:ident => $str:literal ),* $(,)? }) => {
        #[allow(non_camel_case_types)]
        #[allow(unused)]
        #[derive(Copy, Clone, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
        pub enum $Enum {
            $(
                $variant,
            )*
        }

        #[allow(unused)]
        impl $Enum {
            pub fn as_str(&self) -> &'static str {
                self.aupadeshika()
            }

            pub fn aupadeshika(&self) -> &'static str {
                match self {
                    $(
                        $Enum::$variant => $str,
                    )*
                }
            }
        }
    }
}

internal_term!(Stri, {
    cAp => "cAp",
    wAp => "wAp",
    qAp => "qAp",
    NIn => "NIn",
    NIp => "NIp",
    NIz => "NIz",
    UN => "UN",
});

internal_term!(Vikarana, {
    aN => "aN",
    u => "u",
    ksa => "ksa",
    caN => "caN",
    ciR => "ciR",
    cli => "cli~",
    tAsi => "tAsi~",
    yak => "yak",
    Sa => "Sa",
    Sap => "Sap",
    SnA => "SnA",
    Snu => "Snu",
    Syan => "Syan",
    sic => "si~c",
    sya => "sya",
    sip => "si~p",
});

internal_term!(Agama, {
    aw => "aw",
    Aw => "Aw",
    Anuk => "Anu~k",
    Apuk => "Apu~k",
    iw => "iw",
    iruw => "iru~w",
    Iw => "Iw",
    Muk => "Mu~k",
    juk => "ju~k",
    juw => "ju~w",
    Ruk => "Ru~k",
    tuk => "tu~k",
    tuw => "tu~w",
    Tuk => "Tu~k",
    Tuw => "Tu~w",
    duk => "du~k",
    duw => "du~w",
    Duw => "Du~w",
    nIk => "nIk",
    nuk => "nu~k",
    nuw => "nu~w",
    puk => "pu~k",
    buk => "bu~k",
    maw => "maw",
    muk => "mu~k",
    muw => "mu~w",
    yAsuw => "yAsu~w",
    yAw => "yAw",
    yiw => "yi~w",
    yuk => "yu~k",
    yuw => "yu~w",
    rIk => "rIk",
    rik => "rik",
    ruk => "ru~k",
    ruw => "ru~w",
    luk => "lu~k",
    vuk => "vu~k",
    zuk => "zu~k",
    Suw => "Su~w",
    sIyuw => "sIyu~w",
    suw => "su~w",
    syAw => "syAw",
    huk => "hu~k",
});

internal_term_non_unique!(Sup, {
    su => "su~",
    O => "O",
    jas => "jas",
    am => "am",
    Ow => "Ow",
    Sas => "Sas",
    wA => "wA",
    ByAm3 => "ByAm",
    Bis => "Bis",
    Ne => "Ne",
    ByAm4 => "ByAm",
    Byas4 => "Byas",
    Nasi => "Nasi~",
    ByAm5 => "ByAm",
    Byas5 => "Byas",
    Nas => "Nas",
    os6 => "os",
    Am => "Am",
    Ni => "Ni",
    os7 => "os",
    sup => "sup",
});

internal_term!(Tin, {
    tip => "tip",
    tas => "tas",
    Ji => "Ji",
    sip => "sip",
    Tas => "Tas",
    Ta => "Ta",
    mip => "mip",
    vas => "vas",
    mas => "mas",
    ta => "ta",
    AtAm => "AtAm",
    Ja => "Ja",
    TAs => "TAs",
    ATAm => "ATAm",
    Dvam => "Dvam",
    iw => "iw",
    vahi => "vahi",
    mahiN => "mahiN",
});

internal_term!(Upasarga, {
    pra => "pra",
    parA => "parA",
    apa => "apa",
    sam => "sam",
    anu => "anu",
    ava => "ava",
    nis => "nis",
    nir => "nir",
    dus => "dus",
    dur => "dur",
    vi => "vi",
    AN => "AN",
    ni => "ni",
    aDi => "aDi",
    api => "api",
    ati => "ati",
    su => "su",
    ud => "ud",
    aBi => "aBi",
    prati => "prati",
    pari => "pari",
    upa => "upa",
});

internal_term!(Aupadeshika, {
    gAN => "gAN",
    Divi => "Divi~",
    DmA => "DmA\\",
    GrA => "GrA\\",
    SAsu_u => "SAsu~",
    SAsu_a => "SAsu~\\",
    SIN => "SIN",
    asa => "asa~",
    daridrA => "daridrA",
    duza => "du\\za~",
    guhU => "guhU~^",
    hana => "ha\\na~",
    jAgf => "jAgf",
    kfvi => "kfvi~",
    ovijI_u => "o~vijI~",
    ovijI_a => "o~vijI~\\",
    qudAY => "qudA\\Y",
    quDAY => "quDA\\Y",
    qukfY => "qukf\\Y",
    vyaca => "vyaca~",
    Sadx => "Sa\\dx~",
    dAR => "dA\\R",
    dfSir => "df\\Si~r",
    f => "f\\",
    mnA => "mnA\\",
    pA => "pA\\",
    sf => "sf\\",
    zWA => "zWA\\",
    zada => "za\\da~",
    zadx => "za\\dx~",
    hveY => "hve\\Y",
    quvapa => "quva\\pa~^",
    vada_1 => "vada~",
    vaha => "va\\ha~^",
    vasa_1 => "va\\sa~",
    veY => "ve\\Y",
    vyeY => "vye\\Y",
    wuoSvi => "wuo~Svi",
    yaja => "ya\\ja~^",
    Brasja => "Bra\\sja~^",
    graha => "graha~^",
    jyA => "jyA\\",
    ovrascU => "o~vrascU~",
    praCa => "pra\\Ca~",
    vayi => "vayi~",
    vaSa => "vaSa~",
    vyaDa => "vya\\Da~",
    dIDIN => "dIDIN",
    vevIN => "vevIN",
    YitvarA => "YitvarA~\\",
    dF => "dF",
    mrada => "mrada~\\",
    praTa_u => "praTa~",
    smf => "smf\\",
    spaSa_u => "spaSa~",
    spaSa_s => "spaSa~^",
    stFY => "stFY",
    aja => "aja~",
    vraja => "vraja~",
    vaca => "va\\ca~",
    Yizvapa => "Yizva\\pa~",
    SAna => "SAna~^",
    baDa_a => "baDa~\\",
    dAna => "dAna~^",
    mAna_a => "mAna~\\",
    hi => "hi\\",
    vancu => "vancu~",
    vida_2 => "vida~",
    opyAyI => "o~pyAyI~\\",
});

impl Sup {
    pub fn from_args(vibhakti: Vibhakti, vacana: Vacana) -> Self {
        use Sup::*;
        use Vacana::*;
        use Vibhakti::*;
        match (vibhakti, vacana) {
            (Prathama, Eka) => su,
            (Prathama, Dvi) => O,
            (Prathama, Bahu) => jas,
            (Dvitiya, Eka) => am,
            (Dvitiya, Dvi) => Ow,
            (Dvitiya, Bahu) => Sas,
            (Trtiya, Eka) => wA,
            (Trtiya, Dvi) => ByAm3,
            (Trtiya, Bahu) => Bis,
            (Caturthi, Eka) => Ne,
            (Caturthi, Dvi) => ByAm4,
            (Caturthi, Bahu) => Byas4,
            (Panchami, Eka) => Nasi,
            (Panchami, Dvi) => ByAm5,
            (Panchami, Bahu) => Byas5,
            (Sasthi, Eka) => Nas,
            (Sasthi, Dvi) => os6,
            (Sasthi, Bahu) => Am,
            (Saptami, Eka) => Ni,
            (Saptami, Dvi) => os7,
            (Saptami, Bahu) => sup,
            (Sambodhana, Eka) => su,
            (Sambodhana, Dvi) => O,
            (Sambodhana, Bahu) => jas,
        }
    }
}

macro_rules! tin_rules {
    [ $( ($variant:ident, $pada:ident, $purusha:ident, $vacana:ident ) ),* $(,)? ] => {
        impl Tin {
            pub fn from_args(pada: DhatuPada, purusha: Purusha, vacana: Vacana) -> Self {
                match (pada, purusha, vacana) {
                    $(
                        (DhatuPada::$pada, Purusha::$purusha, Vacana::$vacana) => Tin::$variant,
                    )*
                }
            }

            #[allow(unused)]
            pub fn pada(&self) -> DhatuPada {
                match self {
                    $(
                        Self::$variant => DhatuPada::$pada,
                    )*
                }
            }

            #[allow(unused)]
            pub fn purusha(&self) -> Purusha {
                match self {
                    $(
                        Self::$variant => Purusha::$purusha,
                    )*
                }
            }

            #[allow(unused)]
            pub fn vacana(&self) -> Vacana {
                match self {
                    $(
                        Self::$variant => Vacana::$vacana,
                    )*
                }
            }

        }
    }
}

tin_rules![
    (tip, Parasmai, Prathama, Eka),
    (tas, Parasmai, Prathama, Dvi),
    (Ji, Parasmai, Prathama, Bahu),
    (sip, Parasmai, Madhyama, Eka),
    (Tas, Parasmai, Madhyama, Dvi),
    (Ta, Parasmai, Madhyama, Bahu),
    (mip, Parasmai, Uttama, Eka),
    (vas, Parasmai, Uttama, Dvi),
    (mas, Parasmai, Uttama, Bahu),
    (ta, Atmane, Prathama, Eka),
    (AtAm, Atmane, Prathama, Dvi),
    (Ja, Atmane, Prathama, Bahu),
    (TAs, Atmane, Madhyama, Eka),
    (ATAm, Atmane, Madhyama, Dvi),
    (Dvam, Atmane, Madhyama, Bahu),
    (iw, Atmane, Uttama, Eka),
    (vahi, Atmane, Uttama, Dvi),
    (mahiN, Atmane, Uttama, Bahu),
];
