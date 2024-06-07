/**
  * UI Cursor component and system
 */

use hecs::{Component, Entity};

use crate::engine::asset::texture::{SrcRect, TextureKey};
use crate::engine::component::position::Position;
use crate::engine::component::sprite::Sprite;
use crate::engine::ecs::system::{SysArgs, Systemize};
use crate::engine::ecs::world::World;
use crate::engine::geometry::shape::Vec2;
use crate::engine::render::camera::Sticky2;
use crate::engine::utility::alias::Size2;
use crate::game::ui::selection::Selection;

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

impl Systemize for Cursor {
  // Place a cursor beside the selected entity
  fn system(SysArgs { world, .. }: &mut SysArgs) -> Result<(), String> {
    if let Some((cursor, selected)) = get_selection(world) {
      place_cursor(world, cursor, selected)?;
    }
    Ok(())
  }
}

fn get_selection(world: &mut World) -> Option<(Entity, Entity)> {
  if let Some((.., selection)) = world.query_one::<&Selection>() {
    let cursor = selection.get_cursor();
    let (.., selected) = selection.get_selection();
    return Some((cursor, selected));
  }
  None
}

fn place_cursor(world: &mut World, cursor: Entity, selected: Entity) -> Result<(), String> {
  let position = world.get_component::<Position>(selected)?;
  let mut cursor = world.get_component_mut::<Position>(cursor)?;
  cursor.0 = position.0 + CURSOR_OFFSET;
  Ok(())
}