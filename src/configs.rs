use serde_derive::Deserialize;

pub fn load() -> Result<RishConfig, Box<dyn std::error::Error>> {
    toml::from_str(&std::fs::read_to_string(
        crate::dirutils::gethome() + crate::HISTORY_FILE,
    )?)?
}

pub fn gethistfile(histfile: String) {}

#[derive(Deserialize)]
pub struct RishConfig {
    path: Option<String>,
    prompt: Option<String>,
}
