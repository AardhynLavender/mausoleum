use sdl2::keyboard::Keycode;

use crate::engine::event::EventStore;

/**
 * player control utilities
 */

/// How a control was interacted with
pub enum Behaviour {
  Pressed,
  Held,
}

/// A control a player can execute
pub enum Control {
  Up,
  Down,
  Left,
  Right,
  Select,
  Escape,
}

pub fn is_control(control: Control, behaviour: Behaviour, events: &EventStore) -> bool {
  let check = |key| match behaviour {
    Behaviour::Pressed => events.is_key_pressed(key),
    Behaviour::Held => events.is_key_held(key),
  };

  match control {
    Control::Up => check(Keycode::Up) || check(Keycode::W),
    Control::Down => check(Keycode::Down) || check(Keycode::S),
    Control::Left => check(Keycode::Left) || check(Keycode::A),
    Control::Right => check(Keycode::Right) || check(Keycode::D),
    Control::Select => check(Keycode::Return) || check(Keycode::Space),
    Control::Escape => check(Keycode::Escape),
  }
}
