use std::f64::consts::TAU;

#[derive(Debug, Clone)]
pub struct Camera {
    pub x: f64,
    pub y: f64,
    pub target_x: f64,
    pub target_y: f64,
    pub zoom: f64,
    pub target_zoom: f64,
    pub rotation: f64,
    pub target_rotation: f64,
    pub shake_intensity: f64,
    pub shake_decay: f64,
    pub shake_timer: f64,
    pub pulse_intensity: f64,
    pub pulse_frequency: f64,
    pub pulse_timer: f64,
    pub smooth_speed: f64,
    pub zoom_smooth_speed: f64,
    pub rotation_smooth_speed: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            target_x: 0.0,
            target_y: 0.0,
            zoom: 100.0,
            target_zoom: 100.0,
            rotation: 0.0,
            target_rotation: 0.0,
            shake_intensity: 0.0,
            shake_decay: 5.0,
            shake_timer: 0.0,
            pulse_intensity: 0.0,
            pulse_frequency: 2.0,
            pulse_timer: 0.0,
            smooth_speed: 5.0,
            zoom_smooth_speed: 3.0,
            rotation_smooth_speed: 4.0,
        }
    }
}

impl Camera {
    pub fn update(&mut self, dt: f64) {
        // Smooth position interpolation
        self.x += (self.target_x - self.x) * self.smooth_speed * dt;
        self.y += (self.target_y - self.y) * self.smooth_speed * dt;

        // Smooth zoom
        self.zoom += (self.target_zoom - self.zoom) * self.zoom_smooth_speed * dt;

        // Smooth rotation
        let rot_diff = self.target_rotation - self.rotation;
        self.rotation += rot_diff * self.rotation_smooth_speed * dt;

        // Shake decay
        if self.shake_intensity > 0.01 {
            self.shake_timer += dt;
            self.shake_intensity *= (-self.shake_decay * dt).exp();
        } else {
            self.shake_intensity = 0.0;
            self.shake_timer = 0.0;
        }

        // Pulse
        if self.pulse_intensity > 0.01 {
            self.pulse_timer += dt;
            self.pulse_intensity *= 0.95;
        } else {
            self.pulse_intensity = 0.0;
            self.pulse_timer = 0.0;
        }
    }

    pub fn look_at(&mut self, x: f64, y: f64) {
        self.target_x = x;
        self.target_y = y;
    }

    pub fn set_zoom(&mut self, zoom: f64) {
        self.target_zoom = zoom;
    }

    pub fn set_rotation(&mut self, rotation: f64) {
        self.target_rotation = rotation;
    }

    pub fn shake(&mut self, intensity: f64, decay: f64) {
        self.shake_intensity = intensity;
        self.shake_decay = decay;
        self.shake_timer = 0.0;
    }

    pub fn pulse(&mut self, intensity: f64, frequency: f64) {
        self.pulse_intensity = intensity;
        self.pulse_frequency = frequency;
        self.pulse_timer = 0.0;
    }

    pub fn get_shake_offset(&self) -> (f64, f64) {
        if self.shake_intensity < 0.01 {
            return (0.0, 0.0);
        }
        let t = self.shake_timer * 50.0;
        let x = self.shake_intensity * (t * 1.1).sin() * (t * 0.7).cos();
        let y = self.shake_intensity * (t * 1.3).cos() * (t * 0.9).sin();
        (x, y)
    }

    pub fn get_effective_zoom(&self) -> f64 {
        if self.pulse_intensity < 0.01 {
            return self.zoom;
        }
        let pulse = (self.pulse_timer * self.pulse_frequency * TAU).sin() * self.pulse_intensity;
        self.zoom + pulse * 10.0
    }

    pub fn move_event(&mut self, duration: f64, position: Option<(f64, f64)>, rotation: Option<f64>, zoom: Option<f64>, _ease: &str) {
        if let Some((x, y)) = position {
            self.target_x = x;
            self.target_y = y;
        }
        if let Some(r) = rotation {
            self.target_rotation = r;
        }
        if let Some(z) = zoom {
            self.target_zoom = z;
        }

        // Adjust smooth speed based on duration
        if duration > 0.0 {
            self.smooth_speed = 1.0 / duration * 2.0;
            self.zoom_smooth_speed = 1.0 / duration * 2.0;
            self.rotation_smooth_speed = 1.0 / duration * 2.0;
        }
    }
}
