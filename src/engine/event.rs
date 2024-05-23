/**
 * Manage and query events
 */

use std::collections::hash_set::HashSet;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use crate::engine::geometry::shape::Vec2;

/// A set of keycodes
type KeyStore = HashSet<Keycode>;

/// A store of events
pub struct EventStore {
  held_keys: KeyStore,
  pressed_keys: KeyStore,
  mouse_position: Vec2<i32>,
  must_quit: bool,
  must_pause: bool,
  did_pause: bool,
}

impl EventStore {
  /// Instantiate a new EventStore
  pub fn new() -> Self {
    Self {
      pressed_keys: HashSet::new(),
      held_keys: HashSet::new(),
      mouse_position: Vec2::default(),

      must_quit: false,
      must_pause: false,
      did_pause: false,
    }
  }
  /// Clear the pressed keys from the store
  pub fn clear_pressed_keys(&mut self) { self.pressed_keys.clear(); }

  /// Clear any keys currently held
  pub fn clear_held_keys(&mut self) { self.held_keys.clear(); }
 
  /// Mark a key as pressed
  pub fn press_key(&mut self, keycode: Keycode) {
    self.pressed_keys.insert(keycode);
    self.held_keys.insert(keycode);
  }
  /// Mark a key as released
  pub fn raise_key(&mut self, keycode: Keycode) { self.held_keys.remove(&keycode); }

  /// Query if the key was pressed this frame.
  pub fn is_key_pressed(&self, keycode: Keycode) -> bool { self.pressed_keys.contains(&keycode) }
  /// Query if the key is currently held down.
  pub fn is_key_held(&self, keycode: Keycode) -> bool { self.held_keys.contains(&keycode) }

  /// Mark the location of the mouse
  pub fn set_mose_position(&mut self, position: Vec2<i32>) { self.mouse_position = position; }

  /// Mark the application to quit
  pub fn queue_quit(&mut self) { self.must_quit = true; }
  /// Query if the event store should quit
  pub fn should_quit(&self) -> bool { self.must_quit }

  /// Pause the game
  pub fn queue_pause(&mut self) { self.must_pause = true; }
  /// Query if the game should be paused
  pub fn must_pause(&self) -> bool { self.must_pause }
  /// Query if the game has paused
  pub fn is_paused(&self) -> bool { self.did_pause }
  /// Resume the game
  pub fn queue_resume(&mut self) { self.must_pause = false; }
}

/// Manage events polled by SDL2
pub struct Events {
  event_pump: sdl2::EventPump,
  is_quit: bool,
  is_paused: bool,
}

impl Events {
  /// Instantiate a new event manager
  pub fn build(context: &sdl2::Sdl) -> Result<Self, String> {
    let event_pump = context.event_pump()?;
    Ok(Self {
      event_pump,
      is_quit: false,
      is_paused: false,
    })
  }

  /// Query if the game is paused
  pub fn is_paused(&self) -> bool { self.is_paused }

  /// Query if the game is quit
  pub fn is_quit(&self) -> bool { self.is_quit }

  /// Poll for events and update `event_store`
  pub fn update(&mut self, event_store: &mut EventStore) {
    event_store.clear_pressed_keys();

    if event_store.should_quit() {
      self.is_quit = true;
      return;
    }

    self.is_paused = event_store.must_pause();
    event_store.did_pause = self.is_paused;

    let events = self.event_pump.poll_iter();
    for event in events {
      match event {
        Event::Quit { .. } => self.is_quit = true,
        Event::KeyDown { keycode, .. } => {
          keycode.map(|keycode| {
            if !event_store.is_key_held(keycode) {
              event_store.press_key(keycode);
            }
          });
        }
        Event::KeyUp { keycode, .. } => {
          keycode.map(|keycode| event_store.raise_key(keycode));
        }
        Event::MouseMotion { x, y, .. } => {
          event_store.set_mose_position(Vec2 { x, y });
        }
        _ => {}
      }
    }
  }
}
