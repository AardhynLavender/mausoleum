use crate::engine::asset::AssetManager;
use crate::engine::component::text::{make_text, Text};
use crate::engine::rendering::color::color;
use crate::engine::system::SysArgs;
use crate::engine::utility::alignment::{Align, Alignment};
use crate::engine::world::World;
use crate::game::constant::WINDOW;
use crate::game::player::world::{PQ, use_player};

#[derive(Default)]
pub struct PlayerHealth;

pub fn make_player_health_text(world: &mut World, asset: &mut AssetManager) {
  let PQ { health, .. } = use_player(world);
  let text = format!("{}", health);
  let font = asset.typeface
    .use_store()
    .get("typeface")
    .expect("Failed to get typeface");
  world.add(
    make_text::<PlayerHealth>(text, Alignment::new(Align::End(8.0), Align::Start(8.0)), &WINDOW, color::TEXT, font, &mut asset.texture)
  );
}

pub fn sys_render_player_health(SysArgs { world, .. }: &mut SysArgs) {
  let PQ { health, .. } = use_player(world);
  let text = format!("{}", health);
  let (_, (health_text, ..)) = world.query::<(&mut Text, &PlayerHealth)>()
    .into_iter()
    .next()
    .expect("Failed to get player health text");
  health_text.set_content(text);
}