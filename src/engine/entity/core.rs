use engine;
use engine::entity;

/// Position component for entity. Defines a 2D world position.
pub struct ComponentPosition {
  pub entity_id: entity::EntityID,
  pub x: f32,
  pub y: f32,
}

/// Defines an animation to be rendered for this entity. Links to an animation
/// resource. 
/// Dependencies: 
/// ComponentPosition
pub struct ComponentAnimation {
  pub entity_id: entity::EntityID,
  pub animation_handle: engine::resource::animation::AnimationHandle,

  /// Should this animation be rendered, or hidden?
  pub does_render: bool,

  pub x_offset: f32,
  pub y_offset: f32,

  /// Width this animation should be rendered at. Stretching behaviour defined
  /// by animation.
  pub width:    f32,

  /// Height this animation should be rendered at. Stretching behaviour defined
  /// by animation.
  pub height:   f32,
}

/// Draws a rectangle at this entity's position over everything else. 50% opacity.
/// Dependencies: 
/// ComponentPosition
pub struct ComponentDebugDraw {
  pub entity_id: entity::EntityID,
  pub x_offset: f32,
  pub y_offset: f32,
  pub width:    f32,
  pub height:   f32,
  pub color:    engine::common::color::RGBf32,
}

