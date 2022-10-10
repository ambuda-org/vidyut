use multimap::MultiMap;

pub type StemMap = MultiMap<String, String>;
pub type PadaMap = MultiMap<String, String>;
pub type EndingMap = MultiMap<String, String>;

pub fn is_pada(text: &str, data: &PadaMap) -> bool {
    data.contains_key(text)
}
