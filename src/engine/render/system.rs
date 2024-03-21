use crate::engine::geometry::Vec2;
use crate::engine::render::component::Sprite;
use crate::engine::system::SysArgs;
use crate::game::component::position::Position;

/// Entities with a sprite and position are rendered
pub fn sys_render((_, world, renderer, _, _, assets): &mut SysArgs) {
  for (_, (sprite, position)) in world.query_mut::<(&Sprite, &Position)>() {
    let texture = assets.texture
      .use_store()
      .get(sprite.texture)
      .expect(format!("Failed to retrieve texture at {}", sprite.texture).as_str());
    renderer.draw_texture::<i32>(texture, Vec2::from(position.0));
  }
}
