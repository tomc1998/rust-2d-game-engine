use engine::entity::core;
use engine::entity;
use std::vec::Vec;

pub struct State {
  // Current ID. Can get a new ID using State::get_entity_id().
  curr_id: entity::EntityID,
  pub component_position_list: Vec<core::ComponentPosition>,
  pub component_animation_list: Vec<core::ComponentAnimation>,
  pub component_debug_draw_list: Vec<core::ComponentDebugDraw>,
}

impl State {
  pub fn new() -> State {
    State {
      curr_id: entity::EntityID(0),
      component_position_list: Vec::new(),
      component_animation_list: Vec::new(),
      component_debug_draw_list: Vec::new(),
    }
  }

  /// Gets a new entity id. Every time this is called, the value is increased by 1.
  pub fn get_entity_id(&mut self) -> entity::EntityID {
    self.curr_id += entity::EntityID(1);
    return self.curr_id;
  }
}
