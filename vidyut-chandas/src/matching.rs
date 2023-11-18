use crate::vrtta::{Vrtta, Jati};

#[derive(Debug, Clone)]
enum MetreType {
  Vrtta(Vrtta),
  Jati(Jati),
  Anusthup(Vrtta),
}

#[derive(Debug, Clone, Copy)]
pub enum MatchType {
  PadaOne,
  PadaTwo,
  PadaThree,
  PadaFour,
  PadasOneTwo,
  PadasThreeFour,
  PadasOneThree,
  PadasTwoFour,
  PadasAll
}