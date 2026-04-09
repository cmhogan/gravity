use crate::system::SystemState;
use glam::DVec3;
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
pub struct ScenarioConfig {
    pub name: String,
    pub bodies: Vec<BodyConfig>,
}

pub fn load_scenario(
    path: &Path,
    trail_length: usize,
) -> Result<(SystemState, Vec<[u8; 4]>), Box<dyn Error>> {
    let content = fs::read_to_string(path)?;
    let config: ScenarioConfig = toml::from_str(&content)?;

    let mut system = SystemState::new(trail_length);
    let mut colors = Vec::with_capacity(config.bodies.len());

    for body in &config.bodies {
        let pos = DVec3::from_array(body.position);
        let vel = DVec3::from_array(body.velocity);
        system.add_body(body.mass, pos, vel);
        colors.push(body.color);
    }

    // Balance momentum: Adjust the entire system velocity so the net momentum is zero.
    let mut total_momentum = DVec3::ZERO;
    let mut total_mass = 0.0;
    for i in 0..system.masses.len() {
        total_momentum += system.masses[i] * system.velocities[i];
        total_mass += system.masses[i];
    }

    if total_mass > 0.0 {
        let v_barycenter = total_momentum / total_mass;
        for i in 0..system.velocities.len() {
            system.velocities[i] -= v_barycenter;
        }
    }

    Ok((system, colors))
}
