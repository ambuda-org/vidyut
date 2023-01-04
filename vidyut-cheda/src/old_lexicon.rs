//! Maps Sanskrit words and stems to their semantics.

use multimap::MultiMap;
use vidyut_kosha::semantics::*;

pub type StemMap = MultiMap<String, Pratipadika>;
pub type PadaMap = MultiMap<String, Pada>;
pub type EndingMap = MultiMap<String, (String, Pada)>;
