use hecs::{Component, Entity};

use crate::engine::asset::texture::{SrcRect, TextureKey};
use crate::engine::component::ui::Selection;
use crate::engine::geometry::shape::Vec2;
use crate::engine::rendering::camera::Sticky2;
use crate::engine::rendering::component::Sprite;
use crate::engine::utility::alias::Size2;
use crate::engine::world::World;
use crate::game::physics::position::Position;

pub const CURSOR_DIMENSIONS: Size2 = Vec2::new(5, 5);
pub const CURSOR_MARGIN: f32 = 8.0;
pub const CURSOR_PATH: &str = "asset/hud/cursor.png";
pub const CURSOR_OFFSET: Vec2<f32> = Vec2::new(-(CURSOR_DIMENSIONS.x as f32) + -CURSOR_MARGIN, 1.0);

/// Cursor component

pub struct Cursor;

// Create cursor
pub fn make_cursor<C>(world: &mut World, texture: TextureKey) -> Entity where C: Default + Component {
  world.add((
    C::default(),
    Cursor,
    Position::default(),
    Sticky2::default(),
    Sprite::new(texture, SrcRect::new(Vec2::default(), CURSOR_DIMENSIONS)),
  ))
}

/// Place a cursor beside an interfaces selected entity
pub fn place_cursor(world: &mut World, cursor: Entity, interface: &Selection) {
  let (.., selected_entity) = interface.get_selection();
  let position = world.get_component::<Position>(selected_entity).expect("Selected button in interface has no Position!");
  let mut cursor = world.get_component_mut::<Position>(cursor).expect("Cursor has no Position!");
  cursor.0 = position.0 + CURSOR_OFFSET;
}
