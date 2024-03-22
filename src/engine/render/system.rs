use hecs::Or;

use crate::engine::component::text::Text;
use crate::engine::geometry::Vec2;
use crate::engine::render::component::Sprite;
use crate::engine::system::SysArgs;
use crate::game::component::position::Position;

/// Entities with a sprite and position are rendered
pub fn sys_render(SysArgs { world, render, asset, .. }: &mut SysArgs) {
  for (_, (renderable, position)) in world.query::<(Or<&Sprite, &mut Text>, &Position)>() {
    // fetch the texture key from either the 'sprite' or 'text' component
    let texture_key = match renderable {
      Or::Left(sprite) => sprite.texture,
      Or::Right(text) => {
        let typeface = asset.typeface
          .use_store()
          .get("typeface")
          .expect(format!("Failed to retrieve typeface at '{}'", "typeface").as_str());
        if let Some(texture_key) = text.get_content(&typeface, &mut asset.texture) {
          texture_key
        } else {
          continue; // no text content to render; skip this component
        }
      }
      Or::Both(..) => panic!("Entity has both Sprite and Text components")
    };

    let texture = asset.texture
      .use_store()
      .get(texture_key)
      .expect(format!("Failed to retrieve texture at {}", texture_key).as_str());

    render.draw_texture::<i32>(texture, Vec2::from(position.0));
  }
}
