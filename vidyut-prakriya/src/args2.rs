#![allow(unused)]

use crate::args::{
    Antargana, DhatuPada, Gana, Krt, Lakara, Linga, Purusha, Sanadi, Taddhita, Vacana, Vibhakti,
};

/// Pratyayas
#[derive(Clone)]
pub enum Stri {}

#[derive(Clone)]
pub struct Muladhatu {
    aupadeshika: String,
    gana: Gana,
    antargana: Option<Antargana>,
    sanadi: Vec<Sanadi>,
}

#[derive(Clone)]
pub struct Namadhatu {
    pratipadika: Box<Pratipadika>,
    sanadi: Vec<Sanadi>,
}

#[derive(Clone)]
pub enum Dhatu {
    Mula(Muladhatu),
    Nama(Namadhatu),
}

#[derive(Clone)]
pub enum Pratipadika {
    Basic(String),
    Krt(Vec<Pada>, Dhatu, Krt),
    Taddhita(Box<Pratipadika>, Vec<Taddhita>),
    Samasa(Samasa),
    Stri(Box<Pratipadika>, Option<Stri>),
}

impl Pratipadika {
    fn basic(text: impl AsRef<str>) -> Self {
        Self::Basic(text.as_ref().to_string())
    }

    fn krt(dhatu: Dhatu, krt: Krt) -> Self {
        Self::Krt(vec![], dhatu, krt)
    }

    fn taddhita(prati: Pratipadika, taddhita: Taddhita) -> Self {
        Self::Taddhita(Box::new(prati), vec![taddhita])
    }
}

#[derive(Clone)]
pub enum Samasa {
    Bahuvrihi(Pada, Pada),
    Tatpurusha(Pada, Pada),
    Avyayibhava(Pada, Pada),
    Dvandva(Vec<Pada>),
}

impl Samasa {
    pub fn tatpurusha(purva: Pada, uttara: Pada) -> Self {
        Self::Tatpurusha(purva, uttara)
    }

    pub fn bahuvrihi(purva: Pada, uttara: Pada) -> Self {
        Self::Bahuvrihi(purva, uttara)
    }

    pub fn avyayibhava(padas: &[Pada]) -> Self {
        Self::Dvandva(padas.to_vec())
    }

    pub fn dvandva(padas: &[Pada]) -> Self {
        Self::Dvandva(padas.to_vec())
    }
}

#[derive(Clone)]
pub struct Subanta {
    pratipadika: Box<Pratipadika>,
    linga: Linga,
    vacana: Vacana,
    vibhakti: Vibhakti,
}

#[derive(Clone)]
pub struct Tinanta {
    dhatu: Box<Dhatu>,
    purusha: Purusha,
    vacana: Vacana,
    lakara: Lakara,
    pada: Option<DhatuPada>,
}

#[derive(Clone)]
pub enum Pada {
    Subanta(Subanta),
    Tinanta(Tinanta),
}

impl Pada {}

/*
impl Dhatu {
    fn mula(aupadeshika: impl AsRef<str>, gana: Gana) -> Self {
        Self {
            kind: DhatuKind::Mula(Muladhatu {
                aupadeshika: aupadeshika.as_ref().to_string(),
                gana,
                antargana: None,
            }),
            sanadi: vec![],
        }
    }

    fn nama(pratipadika: Pratipadika) -> Self {
        Self {
            kind: DhatuKind::Nama(pratipadika),
            sanadi: vec![],
        }
    }
}

fn foo() {
    use Gana::*;

    let bhu = Dhatu::mula("BU", Bhvadi);

    let putra = Pratipadika::from("putra");
    let putriya = Dhatu::nama(putra);

    let bubhush = Dhatu::builder()
        .aupadeshika("BU")
        .gana(Gana::Bhvadi)
        .antargana(Antargana::Akusmiya)
        .sanadi(&[Sanadi::san])
        .build();
}
*/

