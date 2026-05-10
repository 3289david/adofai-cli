use crossterm::style::Color;
use rand::Rng;
use super::framebuffer::FrameBuffer;

#[derive(Debug, Clone)]
pub struct Particle {
    pub x: f64,
    pub y: f64,
    pub vx: f64,
    pub vy: f64,
    pub life: f64,
    pub max_life: f64,
    pub ch: char,
    pub color: Color,
    pub gravity: f64,
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub max_particles: usize,
}

impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            max_particles,
        }
    }

    pub fn emit(&mut self, x: f64, y: f64, count: usize, style: ParticleStyle) {
        let mut rng = rand::thread_rng();

        for _ in 0..count {
            if self.particles.len() >= self.max_particles { break; }

            let (vx, vy, ch, color, life, gravity) = match style {
                ParticleStyle::Spark => {
                    let angle = rng.gen_range(0.0..std::f64::consts::TAU);
                    let speed = rng.gen_range(0.5..3.0);
                    let chars = ['·', '•', '*', '✦', '✧'];
                    (
                        angle.cos() * speed,
                        angle.sin() * speed,
                        chars[rng.gen_range(0..chars.len())],
                        Color::Rgb { r: 255, g: rng.gen_range(150..255), b: rng.gen_range(50..150) },
                        rng.gen_range(0.3..1.0),
                        0.1,
                    )
                }
                ParticleStyle::Fire => {
                    (
                        rng.gen_range(-0.5..0.5),
                        rng.gen_range(-2.0..-0.5),
                        ['▓', '▒', '░', '█'][rng.gen_range(0..4)],
                        Color::Rgb { r: 255, g: rng.gen_range(80..200), b: 0 },
                        rng.gen_range(0.5..1.5),
                        -0.05,
                    )
                }
                ParticleStyle::Ice => {
                    let angle = rng.gen_range(0.0..std::f64::consts::TAU);
                    let speed = rng.gen_range(0.3..1.5);
                    (
                        angle.cos() * speed,
                        angle.sin() * speed,
                        ['❄', '✦', '·', '•'][rng.gen_range(0..4)],
                        Color::Rgb { r: rng.gen_range(150..200), g: rng.gen_range(200..255), b: 255 },
                        rng.gen_range(0.5..2.0),
                        0.02,
                    )
                }
                ParticleStyle::Hit => {
                    let angle = rng.gen_range(0.0..std::f64::consts::TAU);
                    let speed = rng.gen_range(1.0..4.0);
                    (
                        angle.cos() * speed,
                        angle.sin() * speed,
                        ['●', '○', '◎', '◉'][rng.gen_range(0..4)],
                        Color::White,
                        rng.gen_range(0.2..0.6),
                        0.0,
                    )
                }
                ParticleStyle::Trail => {
                    (
                        rng.gen_range(-0.2..0.2),
                        rng.gen_range(-0.2..0.2),
                        ['·', '∙', '°'][rng.gen_range(0..3)],
                        Color::Rgb { r: 222, g: 187, b: 123 },
                        rng.gen_range(0.5..1.5),
                        0.0,
                    )
                }
                ParticleStyle::Rainbow => {
                    let angle = rng.gen_range(0.0..std::f64::consts::TAU);
                    let speed = rng.gen_range(0.5..2.0);
                    let hue = rng.gen_range(0.0..360.0);
                    let (r, g, b) = hsl_to_rgb(hue, 1.0, 0.6);
                    (
                        angle.cos() * speed,
                        angle.sin() * speed,
                        ['✦', '✧', '★', '☆', '◆'][rng.gen_range(0..5)],
                        Color::Rgb { r, g, b },
                        rng.gen_range(0.5..1.5),
                        0.0,
                    )
                }
            };

            self.particles.push(Particle {
                x, y, vx, vy,
                life,
                max_life: life,
                ch, color, gravity,
            });
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.particles.retain_mut(|p| {
            p.x += p.vx * dt * 10.0;
            p.y += p.vy * dt * 10.0;
            p.vy += p.gravity * dt * 10.0;
            p.life -= dt;
            p.life > 0.0
        });
    }

    pub fn draw(&self, fb: &mut FrameBuffer) {
        for p in &self.particles {
            let alpha = (p.life / p.max_life).clamp(0.0, 1.0);
            let x = p.x as i32;
            let y = p.y as i32;

            let ch = if alpha > 0.7 { p.ch }
                else if alpha > 0.3 { '·' }
                else { '.' };

            let color = if alpha > 0.5 {
                p.color
            } else {
                dim_color(p.color, alpha)
            };

            fb.set_char(x, y, ch, color, Color::Reset);
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParticleStyle {
    Spark,
    Fire,
    Ice,
    Hit,
    Trail,
    Rainbow,
}

fn dim_color(color: Color, factor: f64) -> Color {
    match color {
        Color::Rgb { r, g, b } => Color::Rgb {
            r: (r as f64 * factor) as u8,
            g: (g as f64 * factor) as u8,
            b: (b as f64 * factor) as u8,
        },
        other => other,
    }
}

fn hsl_to_rgb(h: f64, s: f64, l: f64) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h as u32 {
        0..=59 => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179 => (0.0, c, x),
        180..=239 => (0.0, x, c),
        240..=299 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}
