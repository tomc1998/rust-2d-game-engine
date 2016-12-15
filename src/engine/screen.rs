use engine::Engine;

pub trait Screen {
  /// Function called when screen is pushed onto the screen stack
  /// (when the screen is shown on screen)
  fn on_show(&self, engine: &mut Engine);
  /// Function called every frame
  fn update(&self, engine: &mut Engine);
  /// Function called when screen is popped from the stack (when the
  /// screen stops being drawn)
  fn on_hide(&self, engine: &mut Engine);
}

