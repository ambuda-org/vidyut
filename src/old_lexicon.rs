//! Maps Sanskrit words and stems to their semantics.

use crate::semantics::*;
use multimap::MultiMap;

pub type StemMap = MultiMap<String, Pratipadika>;
pub type PadaMap = MultiMap<String, Pada>;
pub type EndingMap = MultiMap<String, (String, Pada)>;
