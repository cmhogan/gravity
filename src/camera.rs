use glam::{DMat3, DVec3};

/// 3D Camera for mapping world coordinates to pixels
pub struct Camera {
    pub offset: DVec3,
    pub scale: f64,
    pub initial_scale: f64,
    pub pitch: f64,
    pub yaw: f64,
}

impl Camera {
    pub fn new(scale: f64) -> Self {
        Self {
            offset: DVec3::ZERO,
            scale,
            initial_scale: scale,
            pitch: 0.0,
            yaw: 0.0,
        }
    }

    /// Reset the camera to its initial state
    pub fn reset(&mut self) {
        self.offset = DVec3::ZERO;
        self.scale = self.initial_scale;
        self.pitch = 0.0;
        self.yaw = 0.0;
    }

    /// Convert world coordinates to pixel coordinates using 3D rotation and 2D projection
    pub fn world_to_screen(
        &self,
        world_pos: DVec3,
        screen_width: u32,
        screen_height: u32,
    ) -> (i32, i32) {
        // Apply 3D rotation (Pitch: X-axis, Yaw: Y-axis)
        let rotation = DMat3::from_rotation_x(self.pitch) * DMat3::from_rotation_y(self.yaw);
        let rotated_pos = rotation * world_pos;

        // Orthographic projection into 2D screen space
        let x = (rotated_pos.x - self.offset.x) * self.scale + (screen_width as f64 / 2.0);
        let y = (rotated_pos.y - self.offset.y) * self.scale + (screen_height as f64 / 2.0);

        (x as i32, y as i32)
    }

    pub fn zoom(&mut self, factor: f64) {
        self.scale *= factor;
    }

    pub fn pan(&mut self, dx: f64, dy: f64) {
        self.offset.x -= dx / self.scale;
        self.offset.y -= dy / self.scale;
    }

    pub fn rotate(&mut self, dpitch: f64, dyaw: f64) {
        self.pitch += dpitch;
        self.yaw += dyaw;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_to_screen_center() {
        let camera = Camera::new(100.0);
        let (x, y) = camera.world_to_screen(DVec3::ZERO, 800, 600);
        assert_eq!(x, 400);
        assert_eq!(y, 300);
    }

    #[test]
    fn test_zoom() {
        let mut camera = Camera::new(100.0);
        camera.zoom(2.0);
        assert_eq!(camera.scale, 200.0);

        let (x, y) = camera.world_to_screen(DVec3::new(1.0, 0.0, 0.0), 800, 600);
        // (1.0 - 0.0) * 200.0 + 400.0 = 600.0
        assert_eq!(x, 600);
        assert_eq!(y, 300);
    }

    #[test]
    fn test_pan() {
        let mut camera = Camera::new(100.0);
        // Pan 100 pixels to the right, 50 pixels down
        camera.pan(100.0, 50.0);

        // Offset should be -1.0, -0.5
        assert_eq!(camera.offset.x, -1.0);
        assert_eq!(camera.offset.y, -0.5);

        let (x, y) = camera.world_to_screen(DVec3::ZERO, 800, 600);
        // (0.0 - (-1.0)) * 100.0 + 400.0 = 500.0
        // (0.0 - (-0.5)) * 100.0 + 300.0 = 350.0
        assert_eq!(x, 500);
        assert_eq!(y, 350);
    }
}
