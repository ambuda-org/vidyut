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
    /// -aka
    akac,
    /// -a
    aR,
    /// -a
    aY,
    /// -a
    at,
    /// -Ara
    Arak,
    /// -i
    iY,
    /// -iman
    imanic,
    /// -ila
    ilac,
    /// -izWa
    izWan,
    /// -Iyas
    Iyasun,
    /// -era,
    Erak,
    /// -ka
    kan,
    /// -kalpa
    kalpap,
    /// -kftvas
    kftvasuc,
    /// -Ina
    Ka,
    /// -Ina
    KaY,
    /// -iya
    Ga,
    /// -iya
    Gac,
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
    /// -ya
    Yya,
    /// -a
    waq,
    /// -ika
    Wak,
    /// -ika
    WaY,
    /// -mat
    qmatup,
    /// -aka
    qvun,
    /// -eya
    Qak,
    /// -eya
    QaY,
    /// -era
    Qrak,
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
    /// -tika
    tikan,
    /// -tra
    tral,
    /// -tva
    tva,
    /// -Tam
    Tamu,
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
    /// -Ayana
    Pak,
    /// -Ayana
    PaY,
    /// -Ayani
    PiY,
    /// -mat
    matup,
    /// -ma
    map,
    /// -maya
    mayaw,
    /// -ya
    yaY,
    /// -ya
    yat,
    /// -yu
    yus,
    /// -rUpa
    rUpap,
    /// -rhi
    rhil,
    /// -la
    lac,
    /// -in
    vini,
    /// -aka
    vuY,
    /// -Sa
    Sa,
    /// -Sas
    Sas,
    /// -ika
    zWan,
    /// -sAt
    sAti,
    /// -s
    suc,
    /// -ha
    ha,
}

enum_boilerplate!(Taddhita, {
    akac => "akac",
    aY => "aY",
    aR => "aR",
    at => "at",
    Arak => "Arak",
    iY => "iY",
    imanic => "imani~c",
    ilac => "ilac",
    izWan => "izWan",
    Iyasun => "Iyasu~n",
    Erak => "Erak",
    kan => "kan",
    kalpap => "kalpap",
    kftvasuc => "kftvasu~c",
    Ka => "Ka",
    KaY => "KaY",
    Ga => "Ga",
    Gac => "Gac",
    Gas => "Gas",
    cPaY => "cPaY",
    Ca => "Ca",
    CaR => "CaR",
    Cas => "Cas",
    Yya => "Yya",
    waq => "waq",
    Wak => "Wak",
    WaY => "WaY",
    qmatup => "qmatu~p",
    qvun => "qvu~n",
    Qak => "Qak",
    QaY => "QaY",
    Qrak => "Qrak",
    tamap => "tamap",
    tarap => "tarap",
    tal => "tal",
    tasi => "tasi~",
    tasil => "tasi~l",
    tikan => "tikan",
    tral => "tral",
    tva => "tva",
    Tamu => "Tamu~",
    TAl => "TAl",
    dA => "dA",
    dAnIm => "dAnIm",
    deSya => "deSya",
    deSIyar => "deSIyar",
    DA => "DA",
    na => "na",
    matup => "matu~p",
    map => "map",
    mayaw => "mayaw",
    Pak => "Pak",
    PaY => "PaY",
    PiY => "PiY",
    yaY => "yaY",
    yat => "yat",
    yus => "yus",
    rhil => "rhil",
    lac => "lac",
    rUpap => "rUpap",
    vini => "vini~",
    vuY => "vuY",
    Sa => "Sa",
    Sas => "Sas",
    zWan => "zWan",
    sAti => "sAti~",
    suc => "su~c",
    ha => "ha",
});

/// The information required to derive a taddhitanta in the grammar.
pub struct TaddhitantaArgs {
    taddhita: Taddhita,
}

impl TaddhitantaArgs {
    /// The taddhita-pratyaya to use in the derivation.
    pub fn taddhita(&self) -> Taddhita {
        self.taddhita
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
}

impl TaddhitantaArgsBuilder {
    /// Sets the taddhita-pratyaya to use in the derivation.
    pub fn taddhita(&mut self, val: Taddhita) -> &mut Self {
        self.taddhita = Some(val);
        self
    }

    /// Converts the arguments in this builder into a `TaddhitantaArgs` struct.
    ///
    /// `build()` will fail if any args are missing.
    pub fn build(&self) -> Result<TaddhitantaArgs> {
        Ok(TaddhitantaArgs {
            taddhita: match self.taddhita {
                Some(x) => x,
                _ => return Err(Error::missing_required_field("krt")),
            },
        })
    }
}
