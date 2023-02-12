/*!
Utility functions for working with the Dhatupatha file included in this crate. For details, see the
comments on the `Dhatupatha` struct.
*/

use crate::args::{Antargana, Dhatu, Gana};
use crate::errors::*;
use std::path::Path;

/// An entry in the Dhatupatha.
pub struct Entry {
    code: String,
    dhatu: Dhatu,
    artha: String,
}

impl Entry {
    /// The numeric code for this entry.
    pub fn code(&self) -> &String {
        &self.code
    }

    /// The dhatu and its important metadata.
    pub fn dhatu(&self) -> &Dhatu {
        &self.dhatu
    }

    /// The meaning of this dhatu as provided in the Dhatupatha.
    pub fn artha(&self) -> &String {
        &self.artha
    }

    /// Returns the position of this entry within the gana.
    pub fn number(&self) -> u16 {
        let (_gana, number) = self.code.split_once('.').expect("should have been checked");
        number.parse().expect("should have been checked")
    }
}

/// An interface to the Dhatupatha used on <ashtadhyayi.com>.
///
/// Different traditional texts might use different dhatupathas. This struct manages the data for
/// the dhatupatha on ashtadhyayi.com, which is a superset of the dhatus from five sources:
///
/// - the *Siddhāntakaumudī*
/// - the *Bṛhaddhātukusumākaraḥ*
/// - the *Mādhavīyadhātuvṛttiḥ*
/// - the *Kṣīrataraṅgiṇī*
/// - the *Dhātupradīpaḥ*
///
/// The specific dhatupatha we use matters: for certain dhatus, we can determine their metadata
/// only if we know exactly where they are located. (For an example, see our implementation of the
/// private `maybe_find_antargana` function.)
pub struct Dhatupatha(Vec<Entry>);

/// Creates a dhatu with the given metadata. This function is meant for testing or for other ad-hoc
/// use cases that cannot load `Dhatupatha` directly.
///
/// This function uses the `number` parameter to determine the dhatu's antargana. If you wish to
/// specify the antargana explicitly, please construct `Dhatu` directly with [`Dhatu::builder`].
pub fn create_dhatu(upadesha: impl AsRef<str>, gana: Gana, number: u16) -> Result<Dhatu> {
    let upadesha = upadesha.as_ref();

    let mut builder = Dhatu::builder().upadesha(upadesha).gana(gana);
    if let Some(x) = maybe_find_antargana(gana, number) {
        builder = builder.antargana(x);
    }
    match upadesha {
        "i\\N" | "i\\k" => {
            builder = builder.prefixes(&["aDi"]);
        }
        "SAsu~\\" => {
            builder = builder.prefixes(&["AN"]);
        }
        _ => (),
    }

    builder.build()
}

fn create_entry(code: &str, upadesha: &str, artha: &str) -> Result<Entry> {
    let (gana, number) = code.split_once('.').ok_or(Error::InvalidFile)?;
    let gana = Gana::from_int(gana.parse()?)?;
    let number = number.parse()?;
    let dhatu = create_dhatu(upadesha, gana, number)?;

    Ok(Entry {
        code: code.to_string(),
        dhatu,
        artha: artha.to_string(),
    })
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
        let mut dhatus = Vec::new();
        let mut rdr = csv::ReaderBuilder::new().delimiter(b'\t').from_path(path)?;
        for maybe_row in rdr.records() {
            let r = maybe_row?;
            let code = &r[0];
            let upadesha = &r[1];
            let artha = &r[2];

            // If the upadesha is missing, this is a ganasutra -- skip.
            if upadesha == "-" {
                continue;
            }

            let entry = create_entry(code, upadesha, artha)?;
            dhatus.push(entry);
        }

        dhatus.sort_by(|x, y| x.code.cmp(&y.code));
        Ok(Self(dhatus))
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
            let upadesha = match fields.next() {
                Some(x) => x,
                None => return Err(Error::InvalidFile),
            };
            let artha = match fields.next() {
                Some(x) => x,
                None => return Err(Error::InvalidFile),
            };

            let entry = create_entry(code, upadesha, artha)?;
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
}

impl IntoIterator for Dhatupatha {
    type Item = Entry;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

fn maybe_find_antargana(gana: Gana, number: u16) -> Option<Antargana> {
    if gana == Gana::Tudadi && (93..=137).contains(&number) {
        // Check number explicitly because some roots are duplicated within tudAdi
        // but outside this gana (e.g. juq).
        Some(Antargana::Kutadi)
    } else if gana == Gana::Curadi && (192..=236).contains(&number) {
        // Need to check range explicitly because some of these roots appear
        // multiple times in the gana, e.g. lakza~
        Some(Antargana::Akusmiya)
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
        assert_eq!(dhatu.upadesha(), "BU");
        assert_eq!(dhatu.gana(), Gana::Bhvadi);
        assert!(dhatu.prefixes().is_empty());
        assert!(dhatu.sanadi().is_empty());
    }

    #[test]
    fn create_dhatu_with_ashas() {
        let dhatu = create_dhatu("SAsu~\\", Gana::Adadi, 23).unwrap();
        assert_eq!(dhatu.upadesha(), "SAsu~\\");
        assert_eq!(dhatu.gana(), Gana::Adadi);
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
}
