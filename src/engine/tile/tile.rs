use crate::engine::asset::texture::{SrcRect, TextureKey};
use crate::engine::geometry::collision::{CollisionBox, CollisionMask};
use crate::engine::geometry::shape::Vec2;
use crate::engine::render::color::{OPAQUE, RGBA};
use crate::engine::store::Key;
use crate::engine::system::SysArgs;
use crate::game::physics::position::Position;
use crate::game::utility::controls::{Behaviour, Control, is_control};

/**
 * Tile structure for rendering segments of a tileset to the screen
 */

/// A unique identifier for a tile
pub type TileKey = Key;

/// Data to create a tile entity (Sprite + Tile)
#[derive(Clone, Copy, Debug)]
pub struct TileData {
  pub texture_key: TextureKey,
  pub src: SrcRect,
  pub tile_key: TileKey,
  pub collision_mask: Option<CollisionMask>,
}

/// A tile object that can be rendered to the screen
#[derive(Clone, Debug)]
pub struct Tile(pub TileKey);

impl Tile {
  /// Instantiate a new tile of `tile_key` that references `tileset_key`
  pub fn new(tile_key: TileKey) -> Self {
    Self(tile_key)
  }
}

/// A tile that can be collided with
pub struct TileCollider {
  pub collision_box: CollisionBox,
  pub mask: CollisionMask,
}

impl TileCollider {
  pub fn new(collision_box: CollisionBox, mask: CollisionMask) -> Self {
    Self {
      collision_box,
      mask,
    }
  }
}

pub fn sys_render_tile_colliders(SysArgs { world, render, event, .. }: &mut SysArgs) {
  if !is_control(Control::Debug, Behaviour::Held, event) {
    return;
  }

  for (_, (position, collider)) in world.query::<(&Position, &TileCollider)>() {
    let color = RGBA::new(255, 0, 0, OPAQUE);
    let (width, height) = collider.collision_box.size.destructure();
    let p = Vec2::<i32>::from(position.0 + collider.collision_box.origin);
    if collider.mask.top {
      render.draw_line(p, p + Vec2::new(width as i32, 0), color);
    }
    if collider.mask.right {
      render.draw_line(p + Vec2::new(width as i32, 0), p + Vec2::new(width as i32, height as i32), color);
    }
    if collider.mask.bottom {
      render.draw_line(p + Vec2::new(0, height as i32), p + Vec2::new(width as i32, height as i32), color);
    }
    if collider.mask.left {
      render.draw_line(p, p + Vec2::new(0, height as i32), color);
    }
  }
}