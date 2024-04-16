use sdl2::keyboard::Keycode;

use crate::engine::event::EventStore;
use crate::engine::utility::direction::Direction;

/**
 * player control utilities
 */

/// How a control was interacted with
#[derive(Copy, Clone)]
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
  Lock,
  Select,
  Debug,
  Escape,
  Trigger,
}

/// Check if `control` is being interacted with `behavior`
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
    Control::Debug => check(Keycode::Q),
    Control::Lock => check(Keycode::LShift) || check(Keycode::RShift),
    Control::Trigger => check(Keycode::J),
    Control::Escape => check(Keycode::Escape),
  }
}

/// Determine a net direction based on pressed controls
pub fn get_direction(events: &EventStore, behaviour: Behaviour) -> Option<Direction> {
  let up = is_control(Control::Up, behaviour, events);
  let down = is_control(Control::Down, behaviour, events);
  let left = is_control(Control::Left, behaviour, events);
  let right = is_control(Control::Right, behaviour, events);

  if (up && down) || (left && right) { return None; }

  if up && left { return Some(Direction::UpLeft); }
  if up && right { return Some(Direction::UpRight); }
  if down && left { return Some(Direction::DownLeft); }
  if down && right { return Some(Direction::DownRight); }

  if up { return Some(Direction::Up); }
  if down { return Some(Direction::Down); }
  if left { return Some(Direction::Left); }
  if right { return Some(Direction::Right); }

  None
}