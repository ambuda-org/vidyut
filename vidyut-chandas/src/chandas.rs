use crate::akshara::{scan_lines, Akshara};
use crate::vrtta::{MatchType, Vrtta};
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

/// Describes a result of classifying an input string with `Chandas`.
pub struct MatchResult {
    vrtta: Option<Vrtta>,
    match_type: MatchType,
    aksharas: Vec<Vec<Akshara>>,
}

impl MatchResult {
    /// The vrtta match for this query.
    pub fn vrtta(&self) -> &Option<Vrtta> {
        &self.vrtta
    }

    /// The match type for this query.
    pub fn match_type(&self) -> MatchType {
        self.match_type
    }

    /// The aksharas in this query.
    pub fn aksharas(&self) -> &Vec<Vec<Akshara>> {
        &self.aksharas
    }
}

/// A metrical classifier.
///
///
/// ### Usage
///
/// ```
/// use vidyut_chandas::{Chandas, MatchType, Vrtta};
///
/// let vrttas: Vec<Vrtta> = vec![
///     "vasantatilakA\tvrtta\tGGLGLLLGLLGLGG".try_into().unwrap(),
///     "mandAkrAntA\tvrtta\tGGGGLLLLLGGLGGLGG".try_into().unwrap(),
///     "puzpitAgrA\tvrtta\tLLLLLLGLGLGG/LLLLGLLGLGLGG".try_into().unwrap(),
///     "udgatA\tvrtta\tLLGLGLLLGL/LLLLLGLGLG/GLLLLLLGLLG/LLGLGLLLGLGLG".try_into().unwrap()
/// ];
/// let chandas = Chandas::new(vrttas);
///
/// let result = chandas.classify("mAtaH samastajagatAM maDukEwaBAreH");
/// assert_eq!(result.vrtta().as_ref().unwrap().name(), "vasantatilakA");
/// assert_eq!(result.match_type(), MatchType::Pada);
/// ```
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Chandas {
    vrttas: Vec<Vrtta>,
}

impl Chandas {
    /// Creates a new `Chandas` instance.
    pub fn new(vrttas: Vec<Vrtta>) -> Chandas {
        Self { vrttas }
    }

    /// Creates a new `Chandas` instance by defining meters from the given text data.
    ///
    /// We recommend using this constructor when the program does not have access to the
    /// filesystem, e.g. when using this code in WebAssembly.
    pub fn from_text(data: &str) -> Result<Self, Box<dyn Error>> {
        let vrttas: Result<Vec<_>, _> = data.lines().map(Vrtta::try_from).collect();
        Ok(Self::new(vrttas?))
    }

    /// Creates a new classifier from the given data path.
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let path = PathBuf::from(path).join(path);
        let data = fs::read_to_string(path)?;
        let vrttas: Result<Vec<_>, _> = data.lines().map(Vrtta::try_from).collect();

        Ok(Self::new(vrttas?))
    }

    /// The vrttas available to this classifier.
    pub fn vrttas(&self) -> &Vec<Vrtta> {
        &self.vrttas
    }

    /// Classifies the input string against an internal list of meters.
    ///
    /// Currently, this function supports only simple samavrtta.
    pub fn classify(&self, text: impl AsRef<str>) -> MatchResult {
        let aksharas = scan_lines(text.as_ref().lines());

        let mut best_match = MatchType::None;
        let mut i_best = None;
        for (i, vrtta) in self.vrttas.iter().enumerate() {
            let match_type = vrtta.try_match(&aksharas);
            if match_type > best_match {
                i_best = Some(i);
                best_match = match_type;
            }
        }

        if let Some(i) = i_best {
            MatchResult {
                vrtta: Some(self.vrttas[i].clone()),
                match_type: best_match,
                aksharas,
            }
        } else {
            MatchResult {
                vrtta: None,
                match_type: best_match,
                aksharas,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_has_vrtta(c: &Chandas, text: &str, expected: &str) {
        let res = c.classify(text);
        assert_eq!(res.vrtta().as_ref().unwrap().name(), expected);
    }

    fn new_chandas() -> Chandas {
        Chandas::new(vec![
            "vasantatilakA\tvrtta\tGGLGLLLGLLGLGG".try_into().unwrap(),
            "mandAkrAntA\tvrtta\tGGGGLLLLLGGLGGLGG".try_into().unwrap(),
            "puzpitAgrA\tvrtta\tLLLLLLGLGLGG/LLLLGLLGLGLGG"
                .try_into()
                .unwrap(),
            "udgatA\tvrtta\tLLGLGLLLGL/LLLLLGLGLG/GLLLLLLGLLG/LLGLGLLLGLGLG"
                .try_into()
                .unwrap(),
        ])
    }

    #[test]
    fn classify_samavrtta_single_pada() {
        let c = new_chandas();
        assert_has_vrtta(&c, "mAtaH samastajagatAM maDukEwaBAreH", "vasantatilakA");
        assert_has_vrtta(&c, "mAtaH\nsamastajagatAM\nmaDukEwaBAreH", "vasantatilakA");
    }

    #[test]
    fn classify_samavrtta_full_verse() {
        let c = new_chandas();
        assert_has_vrtta(
            &c,
            "kaScitkAntAvirahaguruRA svADikArapramattaH
            zApenAstaMgamitamahimA varzaBogyeRa BartuH .
            yakzaScakre janakatanayAsnAnapuRyodakezu
            snigDacCAyAtaruzu vasatiM rAmagiryASramezu .. 1 ..",
            "mandAkrAntA",
        );
        assert!(c.classify("mo mo go go vidyunmAlA").vrtta().is_none());
    }

    #[test]
    fn classify_ardhasamavrtta_pada_1() {
        let c = new_chandas();
        assert_has_vrtta(&c, "aTa madanavaDUrupaplavAntaM", "puzpitAgrA");
    }

    #[test]
    fn classify_ardhasamavrtta_half() {
        let c = new_chandas();
        assert_has_vrtta(
            &c,
            "aTa madanavaDUrupaplavAntaM vyasanakfSA paripAlayAmbaBUva",
            "puzpitAgrA",
        );
        assert_has_vrtta(
            &c,
            "aTa\nmadanavaDUrupaplavAntaM\nvyasanakfSA\nparipAlayAmbaBUva",
            "puzpitAgrA",
        );
    }

    #[test]
    fn classify_ardhasamavrtta_full_verse() {
        let c = new_chandas();
        assert_has_vrtta(
            &c,
            "aTa madanavaDUrupaplavAntaM vyasanakfSA paripAlayAmbaBUva |
                SaSina iva divAtanasya leKA kiraRaparikzayaDUsarA pradozam ||",
            "puzpitAgrA",
        );
    }

    #[test]
    fn classify_vishamavrtta_pada_1() {
        let c = new_chandas();
        assert_has_vrtta(&c, "aTa vAsavasya vacanena", "udgatA");
    }

    #[test]
    fn classify_vishamavrtta_full_verse() {
        let c = new_chandas();
        assert_has_vrtta(
            &c,
            "aTa vAsavasya vacanena ruciravadanastrilocanam |
                klAntirahitamaBirADayituM viDivattapAMsi vidaDe DanaMjayaH ||",
            "udgatA",
        );
    }
}
