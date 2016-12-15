mod engine;
mod screen;
use screen::main_menu;

fn main() {
  let main_menu = main_menu::MainMenuScreen::new();
  let mut game_engine = engine::Engine::new();
  game_engine.swap_screen(&main_menu);
  loop {
  }
}
