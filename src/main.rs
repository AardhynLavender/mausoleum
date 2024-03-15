use crate::engine::application::{Actions, run_application};
use crate::engine::asset::AssetManager;
use crate::engine::event::EventStore;
use crate::engine::render::{Properties, Renderer};
use crate::game::constant::{LOGICAL_SIZE, WINDOW_SIZE, WINDOW_TITLE};

pub mod engine;
pub mod game;

pub struct State {}

fn main() -> Result<(), String> {
  run_application(Properties {
    title: String::from(WINDOW_TITLE),
    dimensions: WINDOW_SIZE,
    logical: Some(LOGICAL_SIZE),
    fullscreen: false,
    show_cursor: false,
    vsync: true,
    opengl: true,
    hardware_acceleration: true,
    software_acceleration: false,
    screen_color: Default::default(),
  }, Actions::<State> {
    load,
    render,
    update,
    setup,
  })
}

fn load(_: &mut AssetManager) {
  println!("Loading game")
}

fn setup(_: &AssetManager) -> State {
  println!("Setting up game");
  State {}
}

fn render(_: &mut State, _: &AssetManager, _: &mut Renderer) {
  println!("Rendering game")
}

fn update(_: &EventStore, _: &AssetManager, _: &mut State, _: &mut Renderer) {
  println!("Updating game")
}
