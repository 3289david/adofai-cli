use std::f64::consts::TAU;

#[derive(Debug, Clone)]
pub struct Planet {
    pub orbit_angle: f64,
    pub orbit_radius: f64,
    pub angular_velocity: f64,
    pub direction: f64, // 1.0 or -1.0
    pub is_active: bool,
}

#[derive(Debug, Clone)]
pub struct PlanetSystem {
    pub planets: Vec<Planet>,
    pub active_planet: usize,
    pub pivot_x: f64,
    pub pivot_y: f64,
    pub current_tile: usize,
    pub orbit_progress: f64,
    pub target_angle: f64,
    pub arrived: bool,
}

impl PlanetSystem {
    pub fn new() -> Self {
        Self {
            planets: vec![
                Planet {
                    orbit_angle: 0.0,
                    orbit_radius: 1.5,
                    angular_velocity: 0.0,
                    direction: 1.0,
                    is_active: true,
                },
                Planet {
                    orbit_angle: std::f64::consts::PI,
                    orbit_radius: 1.5,
                    angular_velocity: 0.0,
                    direction: 1.0,
                    is_active: true,
                },
            ],
            active_planet: 0,
            pivot_x: 0.0,
            pivot_y: 0.0,
            current_tile: 0,
            orbit_progress: 0.0,
            target_angle: std::f64::consts::PI,
            arrived: false,
        }
    }

    pub fn set_multi_planet(&mut self, count: u32) {
        let count = count.max(2) as usize;
        self.planets.clear();
        for i in 0..count {
            let angle = (i as f64 / count as f64) * TAU;
            self.planets.push(Planet {
                orbit_angle: angle,
                orbit_radius: 1.5,
                angular_velocity: 0.0,
                direction: 1.0,
                is_active: true,
            });
        }
    }

    pub fn twirl(&mut self) {
        for planet in &mut self.planets {
            planet.direction *= -1.0;
        }
    }

    pub fn update(&mut self, dt: f64, bpm: f64) {
        let beats_per_second = bpm / 60.0;
        let angular_speed = beats_per_second * std::f64::consts::PI;

        for planet in &mut self.planets {
            planet.angular_velocity = angular_speed * planet.direction;
            planet.orbit_angle += planet.angular_velocity * dt;
            planet.orbit_angle %= TAU;
        }

        // Track orbit progress
        if !self.arrived {
            self.orbit_progress += angular_speed * dt / std::f64::consts::PI;
            if self.orbit_progress >= 1.0 {
                self.arrived = true;
                self.orbit_progress = 1.0;
            }
        }
    }

    pub fn advance_tile(&mut self, tile_x: f64, tile_y: f64, next_angle: f64) {
        self.pivot_x = tile_x;
        self.pivot_y = tile_y;
        self.current_tile += 1;
        self.active_planet = (self.active_planet + 1) % self.planets.len();
        self.target_angle = next_angle.to_radians();
        self.orbit_progress = 0.0;
        self.arrived = false;
    }

    pub fn get_planet_positions(&self) -> Vec<(f64, f64)> {
        self.planets.iter().map(|p| {
            let x = self.pivot_x + p.orbit_angle.cos() * p.orbit_radius;
            let y = self.pivot_y + p.orbit_angle.sin() * p.orbit_radius;
            (x, y)
        }).collect()
    }

    pub fn get_active_position(&self) -> (f64, f64) {
        let p = &self.planets[self.active_planet];
        let x = self.pivot_x + p.orbit_angle.cos() * p.orbit_radius;
        let y = self.pivot_y + p.orbit_angle.sin() * p.orbit_radius;
        (x, y)
    }
}
