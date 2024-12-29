/*!
Utility functions for working with the Dhatupatha file included in this crate. For details, see the
comments on the `Dhatupatha` struct.
*/

use crate::args::{Antargana, Dhatu, Gana};
use crate::core::errors::*;
use std::path::Path;

/// An entry in the Dhatupatha.
pub struct Entry {
    code: String,
    dhatu: Dhatu,
    artha: String,
}

impl Entry {
    fn parse(code: &str, aupadeshika: &str, artha: &str) -> Result<Self> {
        let (gana, number) = code.split_once('.').ok_or(Error::InvalidFile)?;
        let gana = if let Some(stripped) = gana.strip_prefix('0') {
            stripped.parse()?
        } else {
            gana.parse()?
        };
        let number = number.parse()?;
        let dhatu = create_dhatu(aupadeshika, gana, number)?;

        Ok(Self {
            code: code.to_string(),
            dhatu,
            artha: artha.to_string(),
        })
    }

    /// The numeric code for this entry.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// The dhatu and its important metadata.
    pub fn dhatu(&self) -> &Dhatu {
        &self.dhatu
    }

    /// The meaning of this dhatu as provided in the Dhatupatha.
    pub fn artha(&self) -> &str {
        &self.artha
    }

    /// Returns the position of this entry within the gana.
    pub fn number(&self) -> u16 {
        let (_gana, number) = self.code.split_once('.').expect("should have been checked");
        number.parse().expect("should have been checked")
    }
}

/// An interface to the Dhatupatha.
///
/// Different traditional texts might use different dhatupathas. This struct manages the data for
/// the dhatupatha on <https://ashtadhyayi.com>, which is a superset of the dhatus from five
/// sources:
///
/// - the *Siddhāntakaumudī*
/// - the *Bṛhaddhātukusumākaraḥ*
/// - the *Mādhavīyadhātuvṛttiḥ*
/// - the *Kṣīrataraṅgiṇī*
/// - the *Dhātupradīpaḥ*
///
/// The specific Dhatupatha we use matters: for certain dhatus, we can determine their metadata
/// only if we know exactly where they are located. (For an example, see our implementation of the
/// private `maybe_find_antargana` function.)
pub struct Dhatupatha(Vec<Entry>);

/// Creates a dhatu with the given metadata. This function is meant for testing or for other ad-hoc
/// use cases that cannot load `Dhatupatha` directly.
///
/// This function uses the `number` parameter to determine the dhatu's antargana. If you wish to
/// specify the antargana explicitly, please construct `Dhatu` directly with [`Dhatu::builder`].
pub fn create_dhatu(aupadeshika: impl AsRef<str>, gana: Gana, number: u16) -> Result<Dhatu> {
    let aupadeshika = aupadeshika.as_ref();

    let mut builder = Dhatu::builder().aupadeshika(aupadeshika).gana(gana);
    if let Some(x) = maybe_find_antargana(gana, number) {
        builder = builder.antargana(x);
    }
    match aupadeshika {
        "i\\N" | "i\\k" => {
            builder = builder.prefixes(&["aDi"]);
        }
        "SAsu~\\" => {
            builder = builder.prefixes(&["AN"]);
        }
        "za\\da~" if gana == Gana::Curadi => {
            builder = builder.prefixes(&["AN"]);
        }
        _ => (),
    }

    builder.build()
}

impl Dhatupatha {
    /// Loads a dhatupatha from the provided TSV.
    ///
    /// This function expects a TSV with headers and at least two columns. The first column is a
    /// short numeric code associated with the dhatu (e.g. `"01.0001"`), and the second column is
    /// the upadesha (e.g. `"BU"`).
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use vidyut_prakriya::Error;
    /// # use vidyut_prakriya::dhatupatha::Dhatupatha;
    /// let d = Dhatupatha::from_path("dhatupatha.tsv")?;
    /// assert!(d.get("01.0001").is_some());
    /// # Ok::<(), Error>(())
    /// ```
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::from_text(&content)
    }

    /// Loads a dhatupatha from the input text string.
    ///
    /// This function is best suited for environments that don't have access to an underlying file
    /// system, such as when running with WebAssembly.
    ///
    /// # Example
    ///
    /// ```
    /// # use vidyut_prakriya::Error;
    /// # use vidyut_prakriya::dhatupatha::Dhatupatha;
    /// let d = Dhatupatha::from_text("code\tdhatu\tartha\n01.0001\tBU\tsattAyAm")?;
    /// assert!(d.get("01.0001").is_some());
    /// # Ok::<(), Error>(())
    /// ```
    pub fn from_text(csv: &str) -> Result<Self> {
        let mut dhatus = Vec::new();
        for (i, line) in csv.split('\n').enumerate() {
            // Skip header.
            if i == 0 || line.is_empty() {
                continue;
            }

            let mut fields = line.split('\t');
            let code = match fields.next() {
                Some(x) => x,
                None => return Err(Error::InvalidFile),
            };
            let aupadeshika = match fields.next() {
                Some(x) => x,
                None => return Err(Error::InvalidFile),
            };
            let artha = match fields.next() {
                Some(x) => x,
                None => return Err(Error::InvalidFile),
            };

            // If the upadesha is missing, this is a ganasutra -- skip.
            if aupadeshika == "-" {
                continue;
            }

            let entry = Entry::parse(code, aupadeshika, artha)?;
            dhatus.push(entry);
        }

        dhatus.sort_by(|x, y| x.code.cmp(&y.code));
        Ok(Self(dhatus))
    }

    /// Gets the dhatu with the given code.
    pub fn get(&self, code: &str) -> Option<&Dhatu> {
        match self.0.binary_search_by_key(&code, |d| &d.code) {
            Ok(i) => Some(&self.0.get(i)?.dhatu),
            Err(_) => None,
        }
    }

    /// Returns an iterator over this dhatupatha's contents.
    pub fn iter(&self) -> std::slice::Iter<Entry> {
        self.0.iter()
    }
}

impl IntoIterator for Dhatupatha {
    type Item = Entry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Dhatupatha {
    type Item = &'a Entry;
    type IntoIter = std::slice::Iter<'a, Entry>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

/// Returns the antargana of the dhatu at location `number` within `gana.`
///
/// We need to check the numeric position explicitly because some dhatus appear multiple times in
/// their respective ganas with identical forms. (We can usually distinguish these dhatus by
/// meaning, but vidyut-prakriya has poor support for modeling and comparing dhatu meanings.)
fn maybe_find_antargana(gana: Gana, number: u16) -> Option<Antargana> {
    if gana == Gana::Bhvadi && (867..=932).contains(&number) {
        Some(Antargana::Ghatadi)
    } else if gana == Gana::Tudadi && (93..=137).contains(&number) {
        // juqa~, etc.
        Some(Antargana::Kutadi)
    } else if gana == Gana::Curadi && (192..=236).contains(&number) {
        // lakza~, etc.
        Some(Antargana::Akusmiya)
    } else if gana == Gana::Curadi && (279..=337).contains(&number) {
        // tuji~, etc.
        Some(Antargana::Asvadiya)
    } else if gana == Gana::Curadi && (338..=388).contains(&number) {
        // SraTa~, etc.
        Some(Antargana::Adhrshiya)
    } else {
        None
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn create_dhatu_basic() {
        let dhatu = create_dhatu("BU", Gana::Bhvadi, 1).unwrap();
        assert_eq!(dhatu.aupadeshika().unwrap(), "BU");
        assert_eq!(dhatu.gana().expect("ok"), Gana::Bhvadi);
        assert!(dhatu.prefixes().is_empty());
        assert!(dhatu.sanadi().is_empty());
    }

    #[test]
    fn create_dhatu_with_ashas() {
        let dhatu = create_dhatu("SAsu~\\", Gana::Adadi, 23).unwrap();
        assert_eq!(dhatu.aupadeshika().unwrap(), "SAsu~\\");
        assert_eq!(dhatu.gana().expect("ok"), Gana::Adadi);
        assert_eq!(dhatu.prefixes(), &vec!["AN"]);
        assert!(dhatu.sanadi().is_empty());
    }

    #[test]
    fn create_dhatu_with_adhii() {
        let i_n = create_dhatu("i\\N", Gana::Adadi, 41).unwrap();
        assert_eq!(i_n.prefixes(), &vec!["aDi"]);

        let i_k = create_dhatu("i\\k", Gana::Adadi, 42).unwrap();
        assert_eq!(i_k.prefixes(), &vec!["aDi"]);
    }

    #[test]
    fn create_dhatu_with_asad() {
        let dhatu = create_dhatu("za\\da~", Gana::Curadi, 1).unwrap();
        assert_eq!(dhatu.aupadeshika().unwrap(), "za\\da~");
        assert_eq!(dhatu.gana().expect("ok"), Gana::Curadi);
        assert_eq!(dhatu.prefixes(), &vec!["AN"]);
        assert!(dhatu.sanadi().is_empty());
    }
}
