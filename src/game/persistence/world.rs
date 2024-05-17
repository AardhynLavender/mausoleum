use hecs::Entity;

use crate::engine::component::text::TextBuilder;
use crate::engine::geometry::collision::{CollisionBox, CollisionMask, rec2_collision};
use crate::engine::geometry::shape::Vec2;
use crate::engine::rendering::color::color;
use crate::engine::system::{SysArgs, Systemize};
use crate::engine::utility::alignment::{Align, Alignment};
use crate::engine::world::World;
use crate::game::constant::{USER_SAVE_FILE, WINDOW};
use crate::game::persistence::assertion::assert_save_room;
use crate::game::persistence::data::SaveData;
use crate::game::physics::collision::{Collider, make_collision_box};
use crate::game::physics::position::Position;
use crate::game::player::world::{PlayerQuery, use_player};
use crate::game::scene::level::room::use_room;
use crate::game::scene::level::scene::LevelScene;
use crate::game::utility::controls::{Behaviour, Control, is_control};

/// Marks a region within a room where the player can save their progress
#[derive(Clone)]
pub struct SaveArea {
  room: String,
  active: bool,
  used: bool,
}

#[derive(Default)]
pub struct SaveText;

impl SaveArea {
  /// Instantiate a new valid save area
  pub fn build(room: String) -> Result<Self, String> {
    assert_save_room(&room)?;
    Ok(Self { room, active: false, used: false })
  }
  /// Get the save room associated with the area
  pub fn get_area(&self) -> &String { &self.room }
  /// Activate the save area
  pub fn activate(&mut self) { self.active = true; }
  /// Deactivate the save area
  pub fn deactivate(&mut self) { self.active = false; }
  /// Mark the save area as used
  pub fn use_area(&mut self) { self.used = true; }
}

/// Save the player's progress when they enter a save area
impl Systemize for SaveArea {
  fn system(SysArgs { world, asset, event, state, scene, .. }: &mut SysArgs) -> Result<(), String> {
    let PlayerQuery { position, collider, .. } = use_player(world);
    let player_box = make_collision_box(&position, &collider);
    let save_key = is_control(Control::Up, Behaviour::Pressed, event);
    let mut in_save_area = false;

    let active_save_rooms = world
      .query::<(&mut SaveArea, &Position, &Collider)>()
      .into_iter()
      .filter_map(|(_, (area, position, collider))| {
        let save_box = make_collision_box(&position, &collider);
        if rec2_collision(&player_box, &save_box, CollisionMask::full()).is_some() && !area.used {
          in_save_area = true;
          if save_key { area.use_area(); }
          if !area.active {
            area.activate();
            return Some(area.clone());
          }
        } else if area.active {
          area.deactivate();
          return Some(area.clone());
        }
        return None;
      })
      .collect::<Vec<_>>();

    if let Some(area) = active_save_rooms.first() {
      if area.active {
        let typeface = asset.typeface
          .use_store()
          .get("typeface")?;
        let mut builder = TextBuilder::new(&typeface, &mut asset.texture, color::TEXT, &WINDOW);
        world.add(builder.make_text::<SaveText>("Press up to save", Alignment::new(Align::Center(0.0), Align::Center(0.0))));
      } else {
        let save_text = world.query::<&SaveText>().into_iter().next();
        if let Some((entity, _)) = save_text { world.free_now(entity)?; }
      }
    }

    if save_key && in_save_area {
      let collection = use_player(world)
        .inventory
        .iter()
        .copied()
        .collect::<Vec<_>>();
      let save_room = use_room(state).get_name();
      let player_position = use_player(world).position.0;
      let save_area_position = use_save_area(world).collider.origin;
      let saved_position = player_position - save_area_position;

      let save_data = SaveData::build(save_room, collection, saved_position)?;
      save_data.to_file(USER_SAVE_FILE)?;

      scene.queue_next(LevelScene::new(save_data));
    }

    Ok(())
  }
}

/// Save area components
type SaveAreaBundle = (SaveArea, Position, Collider);

/// Compose save area components from a save room and collision box
pub fn make_save_area(save_room: String, area: CollisionBox) -> Result<SaveAreaBundle, String> {
  Ok((
    SaveArea::build(save_room)?,
    Position::from(area.origin),
    Collider(CollisionBox::new(Vec2::default(), area.size))
  ))
}

/// Save area query
pub struct SaveAreaQueryResult<'a> {
  pub entity: Entity,
  pub area: &'a mut SaveArea,
  pub collider: CollisionBox,
}

/// Fetch the save area from the world
/// ## Panics
/// If the world contains no save area
pub fn use_save_area(world: &mut World) -> SaveAreaQueryResult {
  let (entity, (area, position, collider)) =
    world
      .query::<(&mut SaveArea, &Position, &Collider)>()
      .into_iter()
      .next()
      .expect("No save area");
  let collider = make_collision_box(&position, &collider);
  SaveAreaQueryResult { entity, area, collider }
}