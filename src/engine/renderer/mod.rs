extern crate glium;

use engine;

/// Trait which defines something that can render to the screen
pub trait Renderable {
  fn render(&self, x: f32, y: f32, w: f32, h: f32);
}

pub struct Renderer {
}

impl Renderer {
  pub fn new() -> Renderer {
    Renderer {}
  }

  pub fn render(engine: engine::Engine) {
  }
}

