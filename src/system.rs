use glam::DVec3;
use std::collections::VecDeque;

/// Simulation state using Structure of Arrays (SoA) pattern
#[derive(Debug, Default)]
pub struct SystemState {
    pub masses: Vec<f64>,
    pub positions: Vec<DVec3>,
    pub velocities: Vec<DVec3>,
    pub accelerations: Vec<DVec3>,
    pub history: Vec<VecDeque<DVec3>>,
    pub trail_length: usize,
}

impl SystemState {
    pub fn new(trail_length: usize) -> Self {
        Self {
            trail_length,
            ..Default::default()
        }
    }

    pub fn add_body(&mut self, mass: f64, position: DVec3, velocity: DVec3) {
        self.masses.push(mass);
        self.positions.push(position);
        self.velocities.push(velocity);
        self.accelerations.push(DVec3::ZERO);
        self.history
            .push(VecDeque::with_capacity(self.trail_length));
    }

    pub fn record_history(&mut self) {
        if self.trail_length == 0 {
            return;
        }
        for (i, pos) in self.positions.iter().enumerate() {
            let h = &mut self.history[i];
            if h.len() >= self.trail_length {
                h.pop_front();
            }
            h.push_back(*pos);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_body_integrity() {
        let mut system = SystemState::new(100);
        assert_eq!(system.masses.len(), 0);

        system.add_body(1.0, DVec3::ZERO, DVec3::ZERO);
        assert_eq!(system.masses.len(), 1);
        assert_eq!(system.positions.len(), 1);
        assert_eq!(system.velocities.len(), 1);
        assert_eq!(system.accelerations.len(), 1);
        assert_eq!(system.history.len(), 1);

        system.add_body(2.0, DVec3::ONE, DVec3::ONE);
        assert_eq!(system.masses.len(), 2);
        assert_eq!(system.positions.len(), 2);
        assert_eq!(system.velocities.len(), 2);
        assert_eq!(system.accelerations.len(), 2);
        assert_eq!(system.history.len(), 2);
    }

    #[test]
    fn test_history_recording() {
        let trail_length = 100;
        let mut system = SystemState::new(trail_length);
        system.add_body(1.0, DVec3::ZERO, DVec3::ZERO);

        for _ in 0..(trail_length + 10) {
            system.record_history();
        }

        assert_eq!(system.history[0].len(), trail_length);
    }
}
