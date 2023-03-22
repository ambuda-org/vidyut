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
#[allow(missing_docs)]
pub enum Taddhita {
    akac,
    aR,
    iY,
    izWan,
    Iyasun,
    Ka,
    Ga,
    Gac,
    Ca,
    Wak,
    Qak,
    tamap,
    tarap,
    tal,
    tva,
    Pak,
    yaY,
    yat,
}

enum_boilerplate!(Taddhita, {
    akac => "akac",
    aR => "aR",
    iY => "iY",
    izWan => "izWan",
    Iyasun => "Iyasu~n",
    Ka => "Ka",
    Ga => "Ga",
    Gac => "Gac",
    Ca => "Ca",
    Wak => "Wak",
    Qak => "Qak",
    tamap => "tamap",
    tarap => "tarap",
    tal => "tal",
    tva => "tva",
    Pak => "Pak",
    yaY => "yaY",
    yat => "yat",
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
