/**
  * Rendering system
  */

use hecs::{Component, Or};
use crate::engine::asset::asset::AssetManager;
use crate::engine::component::position::Position;
use crate::engine::component::sprite::Sprite;
use crate::engine::component::text::Text;
use crate::engine::ecs::system::{SysArgs, Systemize};
use crate::engine::geometry::shape::Vec2;
use crate::engine::render::camera::{Sticky1, Sticky2};
use crate::engine::render::renderer::{layer, Renderer};

/// components marking entities as renderable
type Renderable<'a> = Or<&'a Sprite, &'a mut Text>;

/// Query for entities of layer `L`
type QueryRenderableOf<'a, L> = (Renderable<'a>, &'a Position, &'a L);

/// Entities with a sprite and position are rendered
impl Systemize for Renderer {
  fn system(args: &mut SysArgs) -> Result<(), String> {
    render_layer::<layer::Layer1>(args);
    render_layer::<layer::Layer2>(args);
    render_layer::<layer::Layer3>(args);
    render_layer::<layer::Layer4>(args);
    render_layer::<layer::Layer5>(args);
    render_layer::<layer::Layer6>(args);
    render_layer::<layer::Layer7>(args);

    render_sticky::<Sticky1>(args);
    render_sticky::<Sticky2>(args);

    Ok(())
  }
}

/// render entities of layer T using the world origin
pub fn render_layer<T>(SysArgs { world, camera, render, asset, .. }: &mut SysArgs) where T: Component {
  for (_, (renderable, position, ..)) in world.query::<QueryRenderableOf<T>>() {
    let position = camera.translate(Vec2::from(position.0));
    render_renderable(render, asset, renderable, position);
  }
}

/// render entities of layer T using the camera origin
pub fn render_sticky<T>(SysArgs { world, render, asset, .. }: &mut SysArgs) where T: Component {
  for (_, (renderable, position, ..)) in world.query::<QueryRenderableOf<T>>() {
    let position = Vec2::from(position.0);
    render_renderable(render, asset, renderable, position);
  }
}

/// render the texture of a `renderable` at `position`
pub fn render_renderable(render: &mut Renderer, asset: &mut AssetManager, mut renderable: Renderable, position: Vec2<i32>) {
  let texture_key = match renderable {
    Or::Left(sprite) => sprite.texture,
    Or::Right(ref mut text) => {
      let typeface = asset.typeface
        .use_store()
        .get("typeface")
        .expect(format!("Failed to retrieve typeface at '{}'", "typeface").as_str());
      if let Some(texture_key) = text.get_content(&typeface, &mut asset.texture) {
        texture_key
      } else {
        return; // no text content to render; skip this component
      }
    }
    Or::Both(..) => panic!("Entity has both Sprite and Text components")
  };

  let texture = asset.texture
    .use_store()
    .get(texture_key)
    .expect(format!("Failed to retrieve texture at {}", texture_key).as_str());

  match renderable {
    Or::Left(sprite) => {
      render.draw_from_texture::<i32>(texture, position, sprite.src, sprite.rotation);
    }
    Or::Right(..) => {
      render.draw_texture::<i32>(texture, position);
    }
    Or::Both(..) => panic!("Entity has both Sprite and Text components")
  }
}