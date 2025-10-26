use crate::macro_utils::py_aupadeshika;
use pyo3::prelude::*;

use crate::prakriya::args::PyAnubandha;
use vidyut_prakriya::morph::{Agama, Stri, Vikarana};

#[pyclass(name = "Agama", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum PyAgama {
    /// TODO
    aw,
    /// TODO
    Aw,
    /// TODO
    Anuk,
    /// TODO
    Apuk,
    /// Connecting *-i-* between dhatus and suffixes.
    iw,
    /// TODO
    iruw,
    /// TODO
    Iw,
    /// TODO
    Muk,
    /// TODO
    juk,
    /// TODO
    juw,
    /// TODO
    Ruk,
    /// TODO
    tuk,
    /// TODO
    tuw,
    /// TODO
    Tuk,
    /// TODO
    Tuw,
    /// TODO
    duk,
    /// TODO
    duw,
    /// TODO
    Duw,
    /// TODO
    nIk,
    /// TODO
    nuk,
    /// TODO
    nuw,
    /// TODO
    num,
    /// TODO
    puk,
    /// TODO
    buk,
    /// TODO
    maw,
    /// TODO
    muk,
    /// TODO
    muw,
    /// Inserted for *parasmaipada-liṅ*.
    yAsuw,
    /// TODO
    yAw,
    /// TODO
    yiw,
    /// TODO
    yuk,
    /// TODO
    yuw,
    /// TODO
    rIk,
    /// TODO
    rik,
    /// TODO
    ruk,
    /// TODO
    ruw,
    /// TODO
    luk,
    /// TODO
    vuk,
    /// TODO
    zuk,
    /// TODO
    Suw,
    /// Inserted for *ātmanepada-liṅ*.
    sIyuw,
    /// TODO
    suw,
    /// TODO
    syAw,
    /// TODO
    huk,
}

py_aupadeshika!(
    PyAgama,
    Agama,
    [
        aw, Aw, Anuk, Apuk, iw, iruw, Iw, Muk, juk, juw, Ruk, tuk, tuw, Tuk, Tuw, duk, duw, Duw,
        nIk, nuk, nuw, num, puk, buk, maw, muk, muw, yAsuw, yAw, yiw, yuk, yuw, rIk, rik, ruk, ruw,
        luk, vuk, zuk, Suw, sIyuw, suw, syAw, huk
    ]
);

#[pyclass(name = "Stri", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum PyStri {
    /// *-ā* (4.1.74)
    cAp,
    /// *-ā* (4.1.4)
    wAp,
    /// *-ā* (4.1.13)
    qAp,
    /// *-ī* (4.1.73)
    NIn,
    /// *-ī* (4.1.5)
    NIp,
    /// *-ī* (4.1.25, 4.1.40)
    NIz,
    /// *-ū* (4.1.66)
    UN,
}

py_aupadeshika!(PyStri, Stri, [cAp, wAp, qAp, NIn, NIp, NIz, UN]);

#[pyclass(name = "Vikarana", module = "prakriya", eq, eq_int, ord)]
#[derive(Copy, Clone, Eq, Hash, PartialEq, PartialOrd)]
#[allow(non_camel_case_types)]
pub enum PyVikarana {
    /// *luṅ-vikaraṇa* (3.1.52)
    aN,
    /// *sārvadhātuka-vikaraṇa* (3.1.79 - 3.1.80)
    u,
    /// *luṅ-vikaraṇa* (3.1.45)
    ksa,
    /// *luṅ-vikaraṇa* (3.1.48)
    caN,
    /// *luṅ-vikaraṇa* (3.1.60, ...)
    ciR,
    /// *luṅ-vikaraṇa* (3.1.43)
    cli,
    /// *luṭ-vikaraṇa* (3.1.33)
    tAsi,
    /// *sārvadhātuka-vikaraṇa* (3.1.67)
    yak,
    /// *sārvadhātuka-vikaraṇa* (3.1.77)
    Sa,
    /// *sārvadhātuka-vikaraṇa* (3.1.68)
    Sap,
    /// *sārvadhātuka-vikaraṇa* (3.1.78)
    Snam,
    /// *sārvadhātuka-vikaraṇa* (3.1.81 - 3.1.82)
    SnA,
    /// *sārvadhātuka-vikaraṇa* (3.1.73 - 3.1.76)
    Snu,
    /// *sārvadhātuka-vikaraṇa* (3.1.69 - 3.1.72)
    Syan,
    /// *luṅ-vikaraṇa* (3.1.44)
    sic,
    /// *lṛṭ-vikaraṇa* (3.1.33)
    sya,
    /// *leṭ-vikaraṇa* (3.1.34)
    sip,
}

py_aupadeshika!(
    PyVikarana,
    Vikarana,
    [aN, u, ksa, caN, ciR, cli, tAsi, yak, Sa, Sap, Snam, SnA, Snu, Syan, sic, sya, sip]
);
