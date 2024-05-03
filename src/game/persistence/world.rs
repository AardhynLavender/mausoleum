use crate::engine::component::text::TextBuilder;
use crate::engine::geometry::collision::{CollisionBox, CollisionMask, rec2_collision};
use crate::engine::geometry::shape::Vec2;
use crate::engine::rendering::color::color;
use crate::engine::system::SysArgs;
use crate::engine::utility::alignment::{Align, Alignment};
use crate::game::constant::{USER_SAVE_FILE, WINDOW};
use crate::game::persistence::assertion::assert_save_room;
use crate::game::persistence::data::SaveData;
use crate::game::physics::collision::{Collider, make_collision_box};
use crate::game::physics::position::Position;
use crate::game::player::world::{PlayerQuery, use_player};
use crate::game::scene::level::room::use_room;
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

type SaveAreaBundle = (SaveArea, Position, Collider);

/// Compose save area components from a save room and collision box
pub fn make_save_area(save_room: String, area: CollisionBox) -> Result<SaveAreaBundle, String> {
  Ok((
    SaveArea::build(save_room)?,
    Position::from(area.origin),
    Collider(CollisionBox::new(Vec2::default(), area.size))
  ))
}

/// Save the player's progress when they enter a save area
pub fn sys_save(SysArgs { world, asset, event, state, .. }: &mut SysArgs) {
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
        .get("typeface")
        .expect("Failed to get typeface");
      let mut builder = TextBuilder::new(&typeface, &mut asset.texture, color::TEXT, &WINDOW);
      world.add(builder.make_text::<SaveText>("Press UP to save", Alignment::new(Align::Center(0.0), Align::Center(0.0))));
    } else {
      let save_text = world.query::<&SaveText>().into_iter().next();
      if let Some((entity, _)) = save_text { world.free_now(entity).expect("Failed to remove save text"); }
    }
  }

  if save_key && in_save_area {
    let collection = use_player(world)
      .inventory
      .iter()
      .copied()
      .collect::<Vec<_>>();
    let save_room = use_room(state).get_name();
    println!("Saving progress in room: {}", save_room);
    SaveData::build(save_room, collection)
      .expect("Failed to build save data")
      .to_file(USER_SAVE_FILE)
      .expect("Failed to save progress");
  }
}