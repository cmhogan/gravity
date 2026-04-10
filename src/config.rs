use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct BodyConfig {
    pub name: String,
    pub mass: f64,
    pub position: [f64; 3],
    pub velocity: [f64; 3],
    pub color: [u8; 4],
}

#[derive(Debug, Deserialize)]
pub struct GalaxyConfig {
    pub name: String,
    pub num_stars: usize,
    pub core_mass: f64,
    pub radius_range: [f64; 2],
    pub center: [f64; 3],
    pub velocity: [f64; 3],
    pub tilt: [f64; 2], // [pitch, yaw] in degrees
    pub color: [u8; 4],
}

#[derive(Debug, Deserialize)]
pub struct ScenarioConfig {
    pub name: String,
    pub bodies: Option<Vec<BodyConfig>>,
    pub galaxies: Option<Vec<GalaxyConfig>>,
}

pub fn load_scenario_config(path: &Path) -> Result<ScenarioConfig, Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let config: ScenarioConfig = toml::from_str(&content)?;
    Ok(config)
}
