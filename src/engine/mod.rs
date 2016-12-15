/// Mod which defines how to render the game
pub mod renderer;
/// Mod which defines the ECS.
/// Multiple components all containing an entity ID, which work together in
/// defining the entity's behaviour.
pub mod entity;
/// Mod which defines resources and ways to load them
pub mod resource;
/// Mod which defines behaviour for screens, or different game states
/// (main menu screen, level 1, level 2 etc...)
pub mod screen;
/// Mod to define common structs or operations
pub mod common;
/// Mod defines 'state' which contain entity components. They get pushed onto
/// the engine's state stack.
pub mod state;

use engine;
use engine::entity::core;
use std::vec;

pub struct Engine<'a> {
  state_stack: vec::Vec<state::State>,
  screen_stack: vec::Vec<&'a screen::Screen>,
  pub renderer : renderer::Renderer,
}

impl<'a> Engine<'a> {
  pub fn new() -> Engine<'a> {
    Engine {
      renderer: engine::renderer::Renderer::new(),
      state_stack: vec![state::State::new()],
      screen_stack: vec![],
    }
  }

  /// Adds a debug draw component to the current engine state
#[inline]
  pub fn add_component_position(&mut self, c: core::ComponentPosition) {
    self.state_stack.last_mut().unwrap().component_position_list.push(c);
  }

  /// Adds an animation component to the current engine state
#[inline]
  pub fn add_component_animation_position(&mut self, c: core::ComponentAnimation) {
    self.state_stack.last_mut().unwrap().component_animation_list.push(c);
  }

  /// Adds a position component to the current engine state
#[inline]
  pub fn add_component_debug_draw(&mut self, c: core::ComponentDebugDraw) {
    self.state_stack.last_mut().unwrap().component_debug_draw_list.push(c);
  }

  /// Returns a reference to the current state.
  pub fn get_curr_state(&self) -> &state::State {
    return &self.state_stack.last().unwrap();
  }

  /// Returns a mutable reference to the current state.
  pub fn get_curr_state_mut(&mut self) -> &mut state::State {
    return self.state_stack.last_mut().unwrap();
  }

  pub fn swap_screen(&mut self, screen: &'a screen::Screen) {
    let s = self.screen_stack.pop();
    if s.is_some() {
      s.unwrap().on_hide(self);
    }
    self.screen_stack.push(screen);
    screen.on_show(self);
  }

  pub fn push_screen(&mut self, screen: &'a screen::Screen) {
    self.screen_stack.push(screen);
    screen.on_show(self);
  }

  /// Pops a screen from the engine. Returns the screen popped - may be None.
  pub fn pop_screen(&mut self) -> Option<&'a screen::Screen> {
    let s = self.screen_stack.pop();
    if s.is_some() {
      s.unwrap().on_hide(self);
    }
    return s;
  }

  /// Push a State struct to the engine. The state struct is where the
  /// components are held.
  pub fn push_state(&mut self) {
    self.state_stack.push(state::State::new());
  }

  /// Pops a State struct from the engine. Returns the state popped. If the state
  /// stack is empty, adds an empty state object to the engine. Return value
  /// should never be none.
  pub fn pop_state(&mut self) -> Option<state::State> {
    let state = self.state_stack.pop();
    if self.state_stack.is_empty() {
      self.push_state();
    }
    return state;
  }
}
