use crate::config::GalaxyConfig;
use crate::system::SystemState;
use glam::{DMat3, DVec3};
use rand::RngExt;
use rayon::prelude::*;

pub const G: f64 = 4.0 * std::f64::consts::PI * std::f64::consts::PI;
pub const SOFTENING: f64 = 1e-3;

/// Initialize a random set of bodies around a central Sun
/// If chaos is true, it uses a random 3D cloud. If false, a disk with orbital velocities.
pub fn init_random_disk(n: usize, chaos: bool, trail_length: usize) -> SystemState {
    let mut system = SystemState::new(trail_length);
    let mut rng = rand::rng();

    // Add a central Sun
    system.add_body(1.0, DVec3::ZERO, DVec3::ZERO);

    for _ in 0..n {
        // Mass: 10^-8 to 10^-4 Solar Masses
        let mass = rng.random_range(1e-8..1e-4);

        if chaos {
            // "Cloud" Mode: Random 3D volume
            let pos = DVec3::new(
                rng.random_range(-3.0..3.0),
                rng.random_range(-3.0..3.0),
                rng.random_range(-3.0..3.0),
            );
            let vel = DVec3::new(
                rng.random_range(-5.0..5.0),
                rng.random_range(-5.0..5.0),
                rng.random_range(-5.0..5.0),
            );
            system.add_body(mass, pos, vel);
        } else {
            // "Disk" Mode: Random 3D disk with orbital velocities
            let r = rng.random_range(0.3..3.0);
            let theta = rng.random_range(0.0..2.0 * std::f64::consts::PI);
            let z = rng.random_range(-0.05..0.05);

            let pos = DVec3::new(r * theta.cos(), r * theta.sin(), z);

            // Circular orbital velocity around the central Sun
            let v_mag = (G * 1.0 / r).sqrt();
            let v_dir = DVec3::new(-theta.sin(), theta.cos(), 0.0);

            // Add some random "jitter" to the velocity
            let jitter = DVec3::new(
                rng.random_range(-0.5..0.5),
                rng.random_range(-0.5..0.5),
                rng.random_range(-0.2..0.2),
            );

            let vel = v_dir * v_mag + jitter;
            system.add_body(mass, pos, vel);
        }
    }

    balance_momentum(&mut system);

    system
}

/// Balance the linear momentum of the system by adjusting the velocity of all bodies
pub fn balance_momentum(system: &mut SystemState) {
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
}

/// Add a procedurally generated galaxy to the system
pub fn add_galaxy(system: &mut SystemState, config: &GalaxyConfig, colors: &mut Vec<[u8; 4]>) {
    let mut rng = rand::rng();
    let center = DVec3::from_array(config.center);
    let velocity = DVec3::from_array(config.velocity);
    let rotation = DMat3::from_rotation_y(config.tilt[1].to_radians())
        * DMat3::from_rotation_x(config.tilt[0].to_radians());

    // Add central core (Black Hole)
    system.add_body(config.core_mass, center, velocity);
    colors.push(config.color);

    for _ in 0..config.num_stars {
        let r = rng.random_range(config.radius_range[0]..config.radius_range[1]);
        let theta = rng.random_range(0.0..2.0 * std::f64::consts::PI);
        let z = rng.random_range(-0.02 * r..0.02 * r); // Thin disk proportional to radius

        // Local position and velocity in the galaxy's orbital plane
        let local_pos = DVec3::new(r * theta.cos(), r * theta.sin(), z);
        let v_mag = (G * config.core_mass / r).sqrt();
        let local_vel = DVec3::new(-theta.sin() * v_mag, theta.cos() * v_mag, 0.0);

        // Apply rotation and translation to match the galaxy's state in the world
        let world_pos = center + rotation * local_pos;
        let world_vel = velocity + rotation * local_vel;

        // Mass: very small, so they don't significantly affect the cores
        let star_mass = 1e-10;
        system.add_body(star_mass, world_pos, world_vel);
        colors.push(config.color);
    }
}

/// Initialize the solar system with real-time NASA JPL State Vectors (April 2026)
pub fn init_solar_system(trail_length: usize) -> SystemState {
    let mut system = SystemState::new(trail_length);

    // Units: Solar Masses (M☉), Astronomical Units (AU), Years (yr)
    // G = 4π² in these units.

    // Sun
    system.add_body(
        1.0,
        DVec3::new(0.00512, 0.00214, -0.00011),
        DVec3::new(-0.0124, 0.0285, 0.0004),
    );

    // Mercury
    system.add_body(
        1.6601e-7,
        DVec3::new(-0.02308, -0.46298, -0.03572),
        DVec3::new(8.2022, 0.0139, -0.7512),
    );

    // Venus
    system.add_body(
        2.4478e-6,
        DVec3::new(0.23046, 0.68304, -0.00391),
        DVec3::new(-7.0246, 2.3268, 0.4373),
    );

    // Earth
    system.add_body(
        3.0034e-6,
        DVec3::new(-0.95312, -0.30628, 0.00002),
        DVec3::new(1.8203, -6.0064, 0.0004),
    );

    /*
    // MOON DATA (Commented out for future use)
    // Relative position to Earth: ~0.0025 AU
    // Relative velocity to Earth: ~0.21 AU/yr
    system.add_body(
        3.69e-8,
        DVec3::new(-0.95312, -0.30878, 0.00002),
        DVec3::new(1.8203 + 0.15, -6.0064 + 0.15, 0.0004) // Needs careful relative v calculation
    );
    */

    // Mars
    system.add_body(
        3.2271e-7,
        DVec3::new(1.32942, -0.37732, -0.04051),
        DVec3::new(1.5907, 5.3541, 0.0732),
    );

    // Jupiter
    system.add_body(
        9.5479e-4,
        DVec3::new(-2.37598, 4.67486, 0.03373),
        DVec3::new(-2.4918, -1.1206, 0.0604),
    );

    // Saturn
    system.add_body(
        2.8588e-4,
        DVec3::new(9.44468, 0.80889, -0.38993),
        DVec3::new(-0.2843, 2.0251, -0.0239),
    );

    // Uranus
    system.add_body(
        4.3662e-5,
        DVec3::new(9.54231, 16.96732, -0.06071),
        DVec3::new(-1.2620, 0.6369, 0.0187),
    );

    // Neptune
    system.add_body(
        5.1513e-5,
        DVec3::new(29.86106, 0.82011, -0.70503),
        DVec3::new(-0.0389, 1.1522, -0.0228),
    );

    // Ceres
    system.add_body(
        4.7e-10,
        DVec3::new(0.2994, 2.6549, 0.0285),
        DVec3::new(-3.8399, 0.1579, 0.7126),
    );

    // 1P/Halley
    system.add_body(
        2.2e-14,
        DVec3::new(-19.4597, 27.3682, -9.8885),
        DVec3::new(0.1901, 0.0631, 0.0412),
    );

    // Balance momentum: Adjust the entire system velocity so the net momentum is zero.
    let mut total_momentum = DVec3::ZERO;
    let mut total_mass = 0.0;
    for i in 0..system.masses.len() {
        total_momentum += system.masses[i] * system.velocities[i];
        total_mass += system.masses[i];
    }

    // Calculate the velocity of the system's barycenter
    let v_barycenter = total_momentum / total_mass;

    // Subtract the barycenter velocity from every body to "freeze" the system center
    for i in 0..system.velocities.len() {
        system.velocities[i] -= v_barycenter;
    }

    system
}

/// Update accelerations using O(n^2) direct summation, parallelized with Rayon
pub fn update_accelerations(system: &mut SystemState) {
    let n = system.masses.len();
    let positions = &system.positions;
    let masses = &system.masses;

    system.accelerations = (0..n)
        .into_par_iter()
        .map(|i| {
            let pi = positions[i];
            let mut acc = DVec3::ZERO;
            let softening_sq = SOFTENING * SOFTENING;

            #[cfg(target_arch = "aarch64")]
            {
                unsafe {
                    use std::arch::aarch64::*;

                    let pi_xy = vsetq_lane_f64(pi.y, vsetq_lane_f64(pi.x, vdupq_n_f64(0.0), 0), 1);
                    let mut acc_xy = vdupq_n_f64(0.0);
                    let mut acc_z = 0.0;

                    for j in 0..n {
                        if i == j {
                            continue;
                        }

                        let pj = positions[j];
                        let mj = masses[j];

                        let pj_xy =
                            vsetq_lane_f64(pj.y, vsetq_lane_f64(pj.x, vdupq_n_f64(0.0), 0), 1);
                        let diff_xy = vsubq_f64(pj_xy, pi_xy);
                        let diff_z = pj.z - pi.z;

                        let dist_sq_xy = vmulq_f64(diff_xy, diff_xy);
                        let dist_sq = vaddvq_f64(dist_sq_xy) + diff_z * diff_z + softening_sq;

                        let inv_dist = 1.0 / dist_sq.sqrt();
                        let inv_dist3 = inv_dist * inv_dist * inv_dist;
                        let factor = G * mj * inv_dist3;
                        let factor_vec = vdupq_n_f64(factor);

                        acc_xy = vfmaq_f64(acc_xy, diff_xy, factor_vec);
                        acc_z += diff_z * factor;
                    }

                    acc.x = vgetq_lane_f64(acc_xy, 0);
                    acc.y = vgetq_lane_f64(acc_xy, 1);
                    acc.z = acc_z;
                }
            }

            #[cfg(not(target_arch = "aarch64"))]
            {
                for j in 0..n {
                    if i == j {
                        continue;
                    }

                    let pj = positions[j];
                    let mj = masses[j];

                    let diff = pj - pi;
                    let dist_sq = diff.length_squared() + softening_sq;
                    let inv_dist = 1.0 / dist_sq.sqrt();
                    let inv_dist3 = inv_dist * inv_dist * inv_dist;

                    acc += diff * (G * mj * inv_dist3);
                }
            }
            acc
        })
        .collect();
}

/// Advance the simulation by dt using the Velocity Verlet integrator
pub fn step(system: &mut SystemState, dt: f64) {
    // 1. Kick: v(t + 0.5*dt) = v(t) + 0.5 * a(t) * dt
    for i in 0..system.velocities.len() {
        system.velocities[i] += system.accelerations[i] * 0.5 * dt;
    }

    // 2. Drift: x(t + dt) = x(t) + v(t + 0.5*dt) * dt
    for i in 0..system.positions.len() {
        system.positions[i] += system.velocities[i] * dt;
    }

    // 3. Update accelerations: a(t + dt)
    update_accelerations(system);

    // 4. Kick: v(t + dt) = v(t + 0.5*dt) + 0.5 * a(t + dt) * dt
    for i in 0..system.velocities.len() {
        system.velocities[i] += system.accelerations[i] * 0.5 * dt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn calculate_total_energy(system: &SystemState) -> f64 {
        let mut kinetic = 0.0;
        let mut potential = 0.0;
        let n = system.masses.len();

        for i in 0..n {
            kinetic += 0.5 * system.masses[i] * system.velocities[i].length_squared();
            for j in (i + 1)..n {
                let r = (system.positions[j] - system.positions[i]).length();
                potential -= G * system.masses[i] * system.masses[j] / (r + SOFTENING);
            }
        }
        kinetic + potential
    }

    fn calculate_total_linear_momentum(system: &SystemState) -> DVec3 {
        let mut momentum = DVec3::ZERO;
        for i in 0..system.masses.len() {
            momentum += system.masses[i] * system.velocities[i];
        }
        momentum
    }

    fn calculate_total_angular_momentum(system: &SystemState) -> DVec3 {
        let mut momentum = DVec3::ZERO;
        for i in 0..system.masses.len() {
            momentum += system.positions[i].cross(system.masses[i] * system.velocities[i]);
        }
        momentum
    }

    #[test]
    fn test_init_random_disk() {
        let n = 200;
        let system = init_random_disk(n, false, 100);
        assert_eq!(system.masses.len(), n + 1); // +1 for the Sun

        // Skip the Sun at index 0
        for i in 1..=n {
            assert!(system.masses[i] >= 1e-8 && system.masses[i] <= 1e-4);
            let r = system.positions[i].length();
            // Max r is roughly sqrt(3^2 + 0.1^2) approx 3.0016
            assert!(r <= 3.1, "Position out of bounds: {}", r);
        }
    }

    #[test]
    fn test_conservation_laws() {
        let mut system = init_solar_system(100);
        update_accelerations(&mut system);

        let initial_energy = calculate_total_energy(&system);
        let initial_linear_momentum = calculate_total_linear_momentum(&system);
        let initial_angular_momentum = calculate_total_angular_momentum(&system);

        let dt = 0.01;
        for _ in 0..100 {
            step(&mut system, dt);
        }

        let final_energy = calculate_total_energy(&system);
        let final_linear_momentum = calculate_total_linear_momentum(&system);
        let final_angular_momentum = calculate_total_angular_momentum(&system);

        let energy_error = (final_energy - initial_energy).abs() / initial_energy.abs();
        let linear_momentum_error = (final_linear_momentum - initial_linear_momentum).length();
        let angular_momentum_error = (final_angular_momentum - initial_angular_momentum).length();

        // Velocity Verlet should conserve energy reasonably well
        assert!(
            energy_error < 1e-4,
            "Energy should be conserved, relative error: {}",
            energy_error
        );
        assert!(
            linear_momentum_error < 1e-12,
            "Linear momentum should be conserved, error: {}",
            linear_momentum_error
        );
        assert!(
            angular_momentum_error < 1e-12,
            "Angular momentum should be conserved, error: {}",
            angular_momentum_error
        );
    }

    #[test]
    fn test_orbital_closure() {
        let mut system = SystemState::new(100);
        // Sun
        system.add_body(1.0, DVec3::ZERO, DVec3::ZERO);
        // Earth
        let start_pos = DVec3::new(1.0, 0.0, 0.0);
        let start_vel = DVec3::new(0.0, 2.0 * std::f64::consts::PI, 0.0);
        system.add_body(3.0034e-6, start_pos, start_vel);

        update_accelerations(&mut system);

        let dt = 0.0001;
        let years = 1.0;
        let steps = (years / dt) as usize;

        for _ in 0..steps {
            step(&mut system, dt);
        }

        let end_pos = system.positions[1];
        let distance = (end_pos - start_pos).length();

        // After 1 year, the Earth should be back to its starting position
        // With dt=0.0001, we expect decent accuracy
        assert!(
            distance < 0.01,
            "Earth should be close to starting position after 1 year, distance was {}",
            distance
        );
    }

    #[test]
    fn test_earth_sun_conservation() {
        let mut system = SystemState::new(100);
        // Sun
        system.add_body(1.0, DVec3::ZERO, DVec3::ZERO);
        // Earth
        system.add_body(
            3.0034e-6,
            DVec3::new(1.0, 0.0, 0.0),
            DVec3::new(0.0, 2.0 * std::f64::consts::PI, 0.0),
        );

        update_accelerations(&mut system);

        let initial_energy = calculate_total_energy(&system);
        let initial_linear_momentum = calculate_total_linear_momentum(&system);
        let initial_angular_momentum = calculate_total_angular_momentum(&system);

        let dt = 0.001;
        let steps = (1.0 / dt) as usize;
        for _ in 0..steps {
            step(&mut system, dt);
        }

        let final_energy = calculate_total_energy(&system);
        let final_linear_momentum = calculate_total_linear_momentum(&system);
        let final_angular_momentum = calculate_total_angular_momentum(&system);

        let energy_error = (final_energy - initial_energy).abs() / initial_energy.abs();
        let linear_momentum_error = (final_linear_momentum - initial_linear_momentum).length();
        let angular_momentum_error = (final_angular_momentum - initial_angular_momentum).length();

        assert!(
            energy_error < 1e-4,
            "Energy should be conserved within 1e-4, error: {}",
            energy_error
        );
        assert!(
            linear_momentum_error < 1e-10,
            "Linear momentum should be conserved, error: {}",
            linear_momentum_error
        );
        assert!(
            angular_momentum_error < 1e-10,
            "Angular momentum should be conserved, error: {}",
            angular_momentum_error
        );
    }
}
