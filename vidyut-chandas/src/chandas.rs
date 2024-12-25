use crate::akshara::{scan_lines, Akshara};
use crate::error::Result;
use crate::padya::{Jati, JatiKind, MatchType, Vrtta};
use std::fs;
use std::path::Path;

/// Models a padya type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Padya {
    Vrtta(Vrtta),
    Jati(Jati),
}

impl Padya {
    pub fn name(&self) -> &str {
        use Padya::*;

        match self {
            Vrtta(v) => v.name(),
            Jati(j) => j.name(),
        }
    }
}

/// Describes a result of classifying an input string with `Chandas`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Match {
    padya: Option<Padya>,
    match_type: MatchType,
    aksharas: Vec<Vec<Akshara>>,
}

impl Match {
    /// The padya match for this query.
    pub fn padya(&self) -> &Option<Padya> {
        &self.padya
    }

    /// The match type for this query.
    pub fn match_type(&self) -> MatchType {
        self.match_type
    }

    /// The aksharas in this query.
    pub fn aksharas(&self) -> &[Vec<Akshara>] {
        &self.aksharas
    }
}

/// Describes a result of classifying an input string with `Chandas`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Matches {
    padyas: Vec<Padya>,
    match_types: Vec<MatchType>,
    aksharas: Vec<Vec<Akshara>>,
}

impl Matches {
    /// The padya matches for the query.
    pub fn padyas(&self) -> &[Padya] {
        &self.padyas
    }

    /// The match type for this query.
    pub fn match_types(&self) -> &[MatchType] {
        &self.match_types
    }

    /// The aksharas in this query.
    pub fn aksharas(&self) -> &[Vec<Akshara>] {
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
/// assert_eq!(result.padya().as_ref().unwrap().name(), "vasantatilakA");
/// assert_eq!(result.match_type(), MatchType::Pada);
/// ```
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct Chandas {
    vrttas: Vec<Vrtta>,
    jatis: Vec<Jati>,
}

impl Chandas {
    /// Creates a new `Chandas` instance.
    pub fn new(vrttas: Vec<Vrtta>) -> Chandas {
        // List is from M. R. Kale's *A Higher Sanskrit Grammar*.
        // Order is roughly based on priority -- items earlier in the list should block items
        // later.
        let jatis = vec![
            Jati::with_kind("vEtAlIyam", vec![14, 16, 14, 16], JatiKind::Vaitaliyam),
            Jati::new("upagIti", vec![12, 15, 12, 15]),
            Jati::new("AryAgIti", vec![12, 20, 12, 20]),
            Jati::new("gIti", vec![12, 18, 12, 18]),
            Jati::new("udgIti", vec![12, 15, 12, 18]),
            Jati::with_kind(
                "OpacCandasikam",
                vec![16, 18, 16, 18],
                JatiKind::Aupacchandasikam,
            ),
            Jati::new("AryA", vec![12, 18, 12, 15]),
        ];

        Self { vrttas, jatis }
    }

    /// Creates a new `Chandas` instance by defining meters from the given text data.
    ///
    /// We recommend using this constructor when the program does not have access to the
    /// filesystem, e.g. when using this code in WebAssembly.
    pub fn from_text(data: impl AsRef<str>) -> Result<Self> {
        let vrttas: Result<Vec<_>> = data.as_ref().lines().map(Vrtta::try_from).collect();
        Ok(Self::new(vrttas?))
    }

    /// Creates a new classifier from the given data path.
    ///
    /// ### Usage
    ///
    /// ```no_run
    /// use vidyut_chandas::Chandas;
    ///
    /// let c = Chandas::from_file("/path/to/meters.tsv").unwrap();
    /// ```
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let data = fs::read_to_string(path.as_ref())?;
        let vrttas: Result<Vec<_>> = data.lines().map(Vrtta::try_from).collect();

        Ok(Self::new(vrttas?))
    }

    /// The vrttas available to this classifier.
    pub fn vrttas(&self) -> &[Vrtta] {
        &self.vrttas
    }

    /// The jatis available to this classifier.
    pub fn jatis(&self) -> &[Jati] {
        &self.jatis
    }

    /// Classifies the input string against an internal list of meters.
    ///
    /// ### Usage
    ///
    /// ```no_run
    /// use vidyut_chandas::Chandas;
    ///
    /// let c = Chandas::from_file("/path/to/meters.tsv").unwrap();
    /// let text = "kaScitkAntAvirahaguruRA svADikArapramattaH";
    /// let res = c.classify(text);
    /// ```
    pub fn classify(&self, text: impl AsRef<str>) -> Match {
        self.classify_inner(text.as_ref())
    }

    fn classify_inner(&self, text: &str) -> Match {
        let aksharas = scan_lines(text.lines());

        // Try vrttas first because these are more exact and can be confused for certain jati
        // types.
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
            return Match {
                padya: Some(Padya::Vrtta(self.vrttas[i].clone())),
                match_type: best_match,
                aksharas,
            };
        }

        for jati in &self.jatis {
            let aksharas = scan_lines(text.lines());
            let flattened_aksharas: Vec<_> = aksharas.clone().into_iter().flatten().collect();
            let res = jati.try_match(&flattened_aksharas);
            if res == MatchType::Full {
                return Match {
                    padya: Some(Padya::Jati(jati.clone())),
                    match_type: MatchType::Full,
                    aksharas,
                };
            }
        }

        // No luck -- return.
        Match {
            padya: None,
            match_type: MatchType::None,
            aksharas,
        }
    }

    /// Classifies the input string against an internal list of meters and returns all possible
    /// matches.
    ///
    /// Currently, this function supports only vrttas.
    pub fn classify_all(&self, text: impl AsRef<str>) -> Matches {
        self.classify_all_inner(text.as_ref())
    }

    fn classify_all_inner(&self, text: &str) -> Matches {
        let aksharas = scan_lines(text.lines());
        let mut padyas = Vec::new();
        let mut match_types = Vec::new();

        for vrtta in &self.vrttas {
            let match_type = vrtta.try_match(&aksharas);
            if match_type != MatchType::None {
                padyas.push(Padya::Vrtta(vrtta.clone()));
                match_types.push(match_type);
            }
        }

        for jati in &self.jatis {
            let flattened_aksharas: Vec<_> = aksharas.clone().into_iter().flatten().collect();
            let res = jati.try_match(&flattened_aksharas);
            if res == MatchType::Full {
                padyas.push(Padya::Jati(jati.clone()));
                match_types.push(MatchType::Full);
            }
        }

        // No luck -- return.
        Matches {
            padyas,
            match_types,
            aksharas,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_has_padya(c: &Chandas, text: &str, expected: &str) {
        let res = c.classify(text);
        assert!(res.padya().is_some());
        assert_eq!(res.padya().clone().unwrap().name(), expected);
    }

    fn new_chandas() -> Chandas {
        Chandas::new(vec![
            "vasantatilakA\tvrtta\tGGLGLLLGLLGLGG".try_into().unwrap(),
            "mandAkrAntA\tvrtta\tGGGG|LLLLLG|GLGGLGG"
                .try_into()
                .unwrap(),
            "puzpitAgrA\tvrtta\tLLLLLLGLGLGG/LLLLGLLGLGLGG"
                .try_into()
                .unwrap(),
            "udgatA\tvrtta\tLLGLGLLLGL/LLLLLGLGLG/GLLLLLLGLLG/LLGLGLLLGLGLG"
                .try_into()
                .unwrap(),
        ])
    }

    #[test]
    fn chandas_struct() {
        let c = new_chandas();
        // Check that `vrttas()` is defined and returns real results.
        assert_eq!(c.vrttas().len(), 4);
        // Check that `jatis()` is defined and returns real results.
        assert_eq!(c.jatis().len(), 7);
    }

    #[test]
    fn classify_samavrtta_single_pada() {
        let c = new_chandas();
        assert_has_padya(&c, "mAtaH samastajagatAM maDukEwaBAreH", "vasantatilakA");
        assert_has_padya(&c, "mAtaH\nsamastajagatAM\nmaDukEwaBAreH", "vasantatilakA");
    }

    #[test]
    fn classify_samavrtta_full_verse() {
        let c = new_chandas();
        assert_has_padya(
            &c,
            "kaScitkAntAvirahaguruRA svADikArapramattaH
            zApenAstaMgamitamahimA varzaBogyeRa BartuH .
            yakzaScakre janakatanayAsnAnapuRyodakezu
            snigDacCAyAtaruzu vasatiM rAmagiryASramezu .. 1 ..",
            "mandAkrAntA",
        );
    }

    #[test]
    fn classify_ardhasamavrtta_pada_1() {
        let c = new_chandas();
        assert_has_padya(&c, "aTa madanavaDUrupaplavAntaM", "puzpitAgrA");
    }

    #[test]
    fn classify_ardhasamavrtta_half() {
        let c = new_chandas();
        assert_has_padya(
            &c,
            "aTa madanavaDUrupaplavAntaM vyasanakfSA paripAlayAmbaBUva",
            "puzpitAgrA",
        );
        assert_has_padya(
            &c,
            "aTa\nmadanavaDUrupaplavAntaM\nvyasanakfSA\nparipAlayAmbaBUva",
            "puzpitAgrA",
        );
    }

    #[test]
    fn classify_ardhasamavrtta_full_verse() {
        let c = new_chandas();
        assert_has_padya(
            &c,
            concat!(
                "aTa madanavaDUrupaplavAntaM vyasanakfSA paripAlayAmbaBUva |",
                "SaSina iva divAtanasya leKA kiraRaparikzayaDUsarA pradozam ||",
            ),
            "puzpitAgrA",
        );
    }

    #[test]
    fn classify_vishamavrtta_pada_1() {
        let c = new_chandas();
        assert_has_padya(&c, "aTa vAsavasya vacanena", "udgatA");
    }

    #[test]
    fn classify_vishamavrtta_full_verse() {
        let c = new_chandas();
        assert_has_padya(
            &c,
            concat!(
                "aTa vAsavasya vacanena ruciravadanastrilocanam |",
                "klAntirahitamaBirADayituM viDivattapAMsi vidaDe DanaMjayaH ||",
            ),
            "udgatA",
        );
    }

    #[test]
    fn classify_jati() {
        // Examples are from M. R. Kale's *A Higher Sanskrit Grammar*
        let c = new_chandas();

        let text = concat!(
            "yenAmandamarande daladaravinde dinAnyanAyizata |",
            "kuwaje Kalu tenehA tenehA maDukareRa kaTam ||"
        );
        assert_has_padya(&c, text, "AryA");

        let text = concat!(
            "pAwIra tava pawIyAnkaH paripAwImimAmurIkartum |",
            "yatpiMzatAmapi nfRAM pizwo'pi tanozi parimalEH puzwim ||",
        );
        assert_has_padya(&c, text, "gIti");

        let text = concat!(
            "navagopasuMdarIRAM rAsollAse murArAtim |",
            "asmArayadupagItiH svargakuraNgIdfSAM gIteH ||"
        );
        assert_has_padya(&c, text, "upagIti");

        let text = concat!(
            "nArAyaRasya saMtatamudgItiH saMsmftirBaktyA |",
            "arcAyAmAsaktirdustarasaMsArasAgare taraRiH ||",
        );
        assert_has_padya(&c, text, "udgIti");

        let text = concat!(
            "cArusamIraRavipine hariRakalaNkakiraNAvalI savilAsA |",
            "AbadDarAmamohA velAmUle viBAvarI parihIna ||",
        );
        assert_has_padya(&c, text, "AryAgIti");
    }

    #[test]
    fn classify_jati_vaitaliyam() {
        let c = new_chandas();
        let text = concat!(
            "kuzalaM Kalu tuByameva tat vacanaM kfzRayadaByaDAmaham |",
            "upadeSaparAH parezvapi svavinASABimuKezu sADavaH ||",
        );
        assert_has_padya(&c, text, "vEtAlIyam");
    }

    #[test]
    fn classify_jati_aupacchandasikam() {
        let c = new_chandas();
        let text = concat!(
            "AtanvAnaM surArikAntAsvOpacCandasikaM hfdo vinodam |",
            "kaMsaM yo nirjaGAna devo vande taM jagatAM sTitiM daDAnam ||",
        );
        assert_has_padya(&c, text, "OpacCandasikam");
    }

    #[test]
    fn classify_all() {
        let c = new_chandas();
        let ret = c.classify_all("kaScitkAntA");
        assert_eq!(ret.padyas().len(), 1);
        assert_eq!(ret.padyas()[0].name(), "mandAkrAntA");
    }
}
