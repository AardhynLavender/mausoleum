use std::time::Duration;

use crate::engine::asset::asset::AssetManager;
use crate::engine::core::lifecycle::LifecycleArgs;
use crate::engine::core::scene::Scene;
use crate::engine::ecs::system::{Schedule, SysArgs, Systemize, SystemTag};
use crate::engine::ecs::world::World;
use crate::engine::render::camera::{Camera, Sticky1};
use crate::engine::utility::alignment::{Align, Alignment};
use crate::engine::utility::color::color;
use crate::engine::utility::time::{ConsumeAction, Timer};
use crate::game::constant::WINDOW;
use crate::game::scene::credits::parse::load_credits;
use crate::game::scene::level::physics::velocity::Velocity;
use crate::game::scene::main_menu::scene::MenuScene;
use crate::game::ui::text_builder::TextBuilder;

const CREDITS_OFFSCREEN_MARGIN: f32 = 16.0;
const CREDIT_LINE_HEIGHT: f32 = 8.0;
const SCROLL_SPEED: f32 = 20.0;

pub struct CreditScene;

pub struct CreditState {
  credits_timer: Timer,
}

/// Add the credit text to the world
fn add_credits(world: &mut World, camera: &Camera, asset: &mut AssetManager) -> Duration {
  let typeface = asset.typeface.use_store().get("typeface").expect("Failed to get typeface");
  let mut builder = TextBuilder::<Sticky1>::new(typeface, &mut asset.texture, color::TEXT, WINDOW);

  let lines = load_credits().expect("Failed to load credits");
  let line_count = lines.len();

  let viewport_height = camera.get_viewport().size.y as f32;
  let start_y = viewport_height + CREDITS_OFFSCREEN_MARGIN;

  for (index, line) in lines.iter().enumerate() {
    let y = start_y + index as f32 * CREDIT_LINE_HEIGHT;
    let line_alignment = Alignment::new(Align::Center(0.0), Align::Start(y));
    let text = world.add(builder.make_text::<()>(line, line_alignment));

    world.add_components(text, (
      Velocity::new(0.0, -SCROLL_SPEED),
    )).expect("Failed to add velocity component");
  };

  let credit_duration = (line_count as f32 * CREDIT_LINE_HEIGHT + viewport_height + CREDITS_OFFSCREEN_MARGIN) / SCROLL_SPEED;
  Duration::from_secs_f32(credit_duration)
}

impl Scene for CreditScene {
  /// Set up the credit scene
  fn setup(&mut self, LifecycleArgs { world, state, camera, system, asset, .. }: &mut LifecycleArgs) {
    system.add(Schedule::FixedUpdate, SystemTag::Scene, Velocity::system).expect("Failed to add system");
    system.add(Schedule::FixedUpdate, SystemTag::Scene, CreditScene::system).expect("Failed to add system");

    let duration = add_credits(world, camera, asset);
    let credit_timer = Timer::new(duration, true);
    state.add(CreditState { credits_timer: credit_timer }).expect("Failed to add credit state");
  }
  /// Tear down the credit scene
  fn destroy(&mut self, LifecycleArgs { state, .. }: &mut LifecycleArgs) {
    state.remove::<CreditState>().expect("Failed to remove credit state");
  }
}

impl Systemize for CreditScene {
  /// Run the credit scene
  fn system(SysArgs { scene, state, .. }: &mut SysArgs) -> Result<(), String> {
    let mut timer = state.get_mut::<CreditState>().expect("Failed to get credit state").credits_timer;
    timer.consume_map(ConsumeAction::Disable, || {
      scene.queue_next(MenuScene);
    });
    Ok(())
  }
}
