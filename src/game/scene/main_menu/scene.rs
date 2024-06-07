/**
 * The game menu scene
 */
use crate::engine::asset::asset::AssetManager;

use crate::engine::asset::texture::SrcRect;
use crate::engine::component::position::Position;
use crate::engine::component::sprite::Sprite;
use crate::engine::core::lifecycle::LifecycleArgs;
use crate::engine::core::scene::Scene;
use crate::engine::ecs::system::{Schedule, SysArgs, Systemize, SystemTag};
use crate::engine::ecs::world::World;
use crate::engine::geometry::shape::{Rec2, Vec2};
use crate::engine::render::camera::{Sticky1, Sticky2};
use crate::engine::render::renderer::layer::Layer9;
use crate::engine::utility::alias::{Size, Size2};
use crate::engine::utility::alignment::{Align, Aligner, Alignment};
use crate::engine::utility::color::color;
use crate::game::constant::{DEV_SAVE_FILE, USER_SAVE_FILE, WINDOW};
use crate::game::persistence::data::SaveData;
use crate::game::scene::level::scene::LevelScene;
use crate::game::ui::cursor::{Cursor, CURSOR_MARGIN, make_cursor};
use crate::game::ui::selection::Selection;
use crate::game::ui::text_builder::TextBuilder;
use crate::game::utility::controls::{Behaviour, Control, is_control};

pub const TITLE_Y: f32 = 70.0;
pub const COPYRIGHT_MARGIN: f32 = 10.0;

pub const TITLE_SIZE: Size2 = Size2::new(78, 20);

pub const BUTTON_GAP: f32 = 16.0;
pub const BUTTON_COUNT: f32 = 4.0;
pub const OPTIONS_BOUNDS: Size2 = Size2::new(48, (BUTTON_GAP * BUTTON_COUNT) as Size);

/// Add the main menu UI to the world
pub fn add_ui(world: &mut World, asset: &mut AssetManager) {
  let textures = &mut asset.texture;
  let typeface = asset.typeface.use_store().get("typeface").expect("Failed to get typeface");
  let cursor_texture = textures.load("asset/hud/cursor.png").expect("Failed to load cursor texture");

  let mut static_builder: TextBuilder::<Sticky2> = TextBuilder::<Sticky2>::new(typeface, textures, color::TEXT, WINDOW);
  world.add(static_builder.make_text::<()>("Aardhyn Lavender 2024", Alignment::new(Align::Center(0.0), Align::End(COPYRIGHT_MARGIN))));

  let title = textures.load("asset/typography/title.png").expect("Failed to load title texture");
  let title_alignment = Alignment::new(Align::Center(0.0), Align::Start(TITLE_Y));
  world.add((
    Sprite::new(title, SrcRect::new(Vec2::default(), TITLE_SIZE)),
    Position::from(WINDOW.align(title_alignment, TITLE_SIZE)),
    Layer9,
  ));

  let buttons_position = WINDOW.center(OPTIONS_BOUNDS);
  let buttons_aligner = Aligner::new(Rec2::new(Vec2::<i32>::from(buttons_position), OPTIONS_BOUNDS));
  let mut button_builder: TextBuilder<'_, '_, Sticky1> = TextBuilder::new(typeface, textures, color::TEXT, buttons_aligner);
  let buttons = [
    world.add(button_builder.make_text::<()>("start", Alignment::new(Align::Start(CURSOR_MARGIN), Align::Start(0.0)))),
    world.add(button_builder.make_text::<()>("new game", Alignment::new(Align::Start(CURSOR_MARGIN), Align::Start(BUTTON_GAP)))),
    world.add(button_builder.make_text::<()>("options", Alignment::new(Align::Start(CURSOR_MARGIN), Align::Start(BUTTON_GAP * 2.0)))),
    world.add(button_builder.make_text::<()>("quit", Alignment::new(Align::Start(CURSOR_MARGIN), Align::Start(BUTTON_GAP * 3.0)))),
  ];

  let cursor = make_cursor::<()>(world, cursor_texture);

  world.add((Selection::build(buttons, cursor).expect("Failed to build selection"), ));
}

// The main menu displayed when the application starts
pub struct MenuScene;

impl Scene for MenuScene {
  /// Set up the main menu scene
  fn setup(&mut self, LifecycleArgs { world, system, asset, .. }: &mut LifecycleArgs) {
    add_ui(world, asset);
    system.add(Schedule::PostUpdate, SystemTag::Suspendable, MenuScene::system).expect("Failed to add menu system");
    system.add(Schedule::PostUpdate, SystemTag::Suspendable, Cursor::system).expect("Failed to add menu system");
  }
  /// Destroy the main menu scene
  fn destroy(&mut self, LifecycleArgs { .. }: &mut LifecycleArgs) {}
}

impl Systemize for MenuScene {
  /// Manage the selection of the main menu
  fn system(SysArgs { scene, event, world, .. }: &mut SysArgs) -> Result<(), String> {
    let (.., menu) = world.query_one::<&mut Selection>().ok_or("Failed to get menu selection")?;

    let up = is_control(Control::Up, Behaviour::Pressed, event);
    let down = is_control(Control::Down, Behaviour::Pressed, event);
    let delta = if up { -1 } else if down { 1 } else { 0 };
    *menu += delta;

    if is_control(Control::Select, Behaviour::Pressed, event) {
      let (index, ..) = menu.get_selection();
      match index {
        0 => {
          let save_data = SaveData::from_file(USER_SAVE_FILE)
            .unwrap_or(SaveData::from_file(DEV_SAVE_FILE)
              .map_err(|error| eprintln!("Failed to load dev save file: {}", error))
              .unwrap_or(SaveData::default())
            );
          scene.queue_next(LevelScene::new(save_data))
        }
        1 => {
          // delete old save data and start from default
          let save_data = SaveData::from_erased(USER_SAVE_FILE)
            .unwrap_or(SaveData::default());
          scene.queue_next(LevelScene::new(save_data))
        }
        2 => { eprintln!("Not implemented yet") }
        3 => { event.queue_quit() }
        _ => { unreachable!("Invalid menu selection index"); }
      }
    }

    Ok(())
  }
}