use engine;
use engine::screen;
use engine::entity::core;
use engine::common::color;

pub struct MainMenuScreen {
}

impl MainMenuScreen {
  pub fn new() -> MainMenuScreen {
    MainMenuScreen {}
  }
}

impl screen::Screen for MainMenuScreen {
  fn on_show(&self, engine: &mut engine::Engine) {
    engine.push_state();
    let test_entity_id = engine.get_curr_state_mut().get_entity_id();
    engine.add_component_position(core::ComponentPosition {
        entity_id: test_entity_id,
        x: 100.0f32,
        y: 100.0f32,
    });
    engine.add_component_debug_draw(core::ComponentDebugDraw {
        entity_id: test_entity_id,
        x_offset: 0.0f32,
        y_offset: 0.0f32,
        width: 64.0f32,
        height: 64.0f32,
        color: color::RGBf32::new(1.0f32, 0.0f32, 0.0f32)
    });
  }
  fn update(&self, engine: &mut engine::Engine) {
  }
  fn on_hide(&self, engine: &mut engine::Engine) {
  }
}
