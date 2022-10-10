use crate::padas;
use crate::sandhi;
use std::error::Error;

pub struct DataPaths {
    pub sandhi_rules: String,
    pub shs_verbs: String,
    pub shs_adverbs: String,
    pub shs_final: String,
}

pub struct Context {
    pub sandhi_rules: sandhi::SandhiMap,
    pub pada_map: padas::PadaMap,
}

pub fn load_data(paths: &DataPaths) -> Result<Context, Box<dyn Error>> {
    Ok(Context {
        sandhi_rules: sandhi::read_rules(&paths.sandhi_rules)?,
        pada_map: padas::read_pada_data(&paths)?,
    })
}
