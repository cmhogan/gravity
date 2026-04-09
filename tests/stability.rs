use gravity::physics::{init_solar_system, step, update_accelerations};

#[test]
fn test_inner_solar_system_stability() {
    let mut system = init_solar_system(0); // No trails needed for stability test
    update_accelerations(&mut system);

    // Initial position of Earth is at (1.0, 0.0, 0.0)
    let earth_index = 3; // Sun is 0, Mercury 1, Venus 2, Earth 3, Mars 4
    let initial_dist = system.positions[earth_index].length();
    assert!((initial_dist - 1.0).abs() < 1e-6);

    let dt = 0.001; // 0.001 years
    let total_years = 10.0;
    let steps = (total_years / dt) as usize;

    for i in 0..steps {
        step(&mut system, dt);
        
        let current_dist = system.positions[earth_index].length();
        // Distance should remain within 1% of 1.0 AU (0.01 AU)
        assert!(
            (current_dist - 1.0).abs() < 0.01, 
            "Earth drifted too far: {} AU at year {}", 
            current_dist, 
            i as f64 * dt
        );
    }

    let final_dist = system.positions[earth_index].length();
    assert!(
        (final_dist - 1.0).abs() < 0.01, 
        "Earth final distance out of range: {} AU", 
        final_dist
    );
}
