use crossterm::style::Color;
use super::framebuffer::FrameBuffer;
use super::post_process::PostProcessor;
use super::particles::ParticleSystem;
use crate::camera::Camera;

pub struct RenderPipeline {
    pub fb: FrameBuffer,
    pub post_processor: PostProcessor,
    pub particles: ParticleSystem,
}

impl RenderPipeline {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            fb: FrameBuffer::new(width, height),
            post_processor: PostProcessor::default(),
            particles: ParticleSystem::new(500),
        }
    }

    pub fn begin_frame(&mut self, bg_color: Color) {
        self.fb.clear_with_color(bg_color);
    }

    pub fn update(&mut self, dt: f64) {
        self.particles.update(dt);
    }

    pub fn draw_particles(&mut self) {
        let fb = &mut self.fb;
        self.particles.draw(fb);
    }

    pub fn apply_post_processing(&mut self) {
        self.post_processor.apply(&mut self.fb);
    }

    pub fn world_to_screen(&self, wx: f64, wy: f64, camera: &Camera) -> (i32, i32) {
        let zoom = camera.zoom / 100.0;
        let cos_r = camera.rotation.to_radians().cos();
        let sin_r = camera.rotation.to_radians().sin();

        let dx = wx - camera.x;
        let dy = wy - camera.y;

        let rx = dx * cos_r - dy * sin_r;
        let ry = dx * sin_r + dy * cos_r;

        let sx = (rx * zoom) + (self.fb.width as f64 / 2.0);
        let sy = (ry * zoom) + (self.fb.height as f64 / 2.0);

        (sx as i32, sy as i32)
    }

    pub fn draw_tile_at_screen(&mut self, sx: i32, sy: i32, ch: char, fg: Color, bg: Color, active: bool) {
        if active {
            self.fb.set_char(sx, sy, ch, fg, bg);
            if ch == '●' {
                self.fb.set_char(sx - 1, sy, '(', fg, Color::Reset);
                self.fb.set_char(sx + 1, sy, ')', fg, Color::Reset);
            }
        } else {
            self.fb.set_char(sx, sy, ch, fg, Color::Reset);
        }
    }
}
