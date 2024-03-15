use std::rc::Rc;

use sdl2::rect::Rect;
use sdl2::render::TextureCreator;
use sdl2::video::{FullscreenType, WindowContext};

use crate::engine::asset::texture::{SrcRect, Texture};
use crate::engine::geometry::{IntConvertable, Rec2, SizePrimitive, Vec2};
use crate::engine::render::color::RGBA;

/**
 * Rendering subsystem
 */

pub mod color;
pub mod text;

/// Properties required to create a new `Renderer`
#[derive(Clone)]
pub struct Properties {
  pub title: String,
  pub dimensions: Vec2<u32>,
  pub logical: Option<Vec2<u32>>,
  pub fullscreen: bool,
  pub show_cursor: bool,
  pub vsync: bool,
  pub opengl: bool,
  pub hardware_acceleration: bool,
  pub software_acceleration: bool,
  pub screen_color: RGBA,
}

/// Wrapper around `sdl2::render::WindowCanvas`
pub struct Renderer {
  subsystem: sdl2::render::WindowCanvas,
  properties: Properties,
}

impl Renderer {
  /// Instantiate a new `Renderer` with the given `Properties`
  pub fn build(context: &sdl2::Sdl, properties: Properties) -> Result<Self, String> {
    let window = build_window(context, &properties);

    // apply pre-construction properties
    let mut builder = window?.into_canvas(); // takes ownership of `Window`
    if properties.vsync { builder = builder.present_vsync(); }
    if properties.hardware_acceleration { builder = builder.accelerated(); }
    if properties.software_acceleration { builder = builder.software(); }
    if !properties.show_cursor { context.mouse().show_cursor(false); }

    // build renderer subsystem
    let mut subsystem = builder
      .build()
      .map_err(|e| e.to_string())?;

    // apply post-construction properties
    if let Some(size) = properties.logical {
      subsystem.set_logical_size(size.x, size.y).map_err(|e| e.to_string())?;
    }

    subsystem.set_draw_color(properties.screen_color);

    Ok(Self {
      subsystem,
      properties,
    })
  }

  /// Instantiate a new `TextureCreator` from the `Renderer`
  pub fn new_texture_creator(&self) -> TextureCreator<WindowContext> { self.subsystem.texture_creator() }

  /// Set the windows fullscreen mode
  pub fn set_fullscreen(&mut self, fullscreen: bool) {
    // note: we skip over the `sdl2::video::FullscreenType::True`
    //       I've found this mode to have visual artifacts
    if fullscreen {
      self.subsystem.window_mut().set_fullscreen(FullscreenType::Desktop).expect("Failed to set dekstop fullscreen")
    } else {
      self.subsystem.window_mut().set_fullscreen(FullscreenType::Off).expect("Failed to set windowed")
    }
  }
  /// Check if the window is in fullscreen mode
  pub fn is_fullscreen(&self) -> bool {
    self.subsystem.window().fullscreen_state() == FullscreenType::Desktop
  }

  /// Set the drawing color of the internal `sdl2::render::WindowCanvas`
  fn set_color(&mut self, color: RGBA) {
    self.subsystem.set_draw_color(color);
  }
  /// Clear the screen
  pub fn clear(&mut self) {
    self.set_color(self.properties.screen_color);
    self.subsystem.clear();
  }
  /// Present what has been rendered to the screen
  pub fn present(&mut self) {
    self.subsystem.present();
    self.clear();
  }

  /// Draw `texture` to the screen at `position`
  pub fn draw_texture<T: IntConvertable>(&mut self, texture: &Rc<Texture>, position: Vec2<T>) {
    let (x, y) = position.destructure();
    let (w, h) = texture.dimensions.destructure();
    let src = Rect::new(0, 0, w, h);
    let dest = Rect::new(x.into(), y.into(), w, h);
    self.subsystem.copy(&texture.internal, src, dest)
      .map_err(|error| eprintln!("{error}"))
      .ok();
  }
  /// Draw `from` `texture` to the screen at `position`
  pub fn draw_from_texture<T: IntConvertable>(&mut self, texture: &Rc<Texture>, position: Vec2<T>, from: SrcRect) {
    let (x, y) = position.destructure();
    let ((sx, sy), (w, h)) = from.destructure();
    let dest = Rect::new(x.into(), y.into(), w, h);
    let src = Rect::new(sx as i32, sy as i32, w, h);
    self.subsystem.copy(&texture.internal, src, dest)
      .map_err(|error| eprintln!("{error}"))
      .ok();
  }
  /// Draw `rect` of `color` to the screen
  pub fn draw_rect<T: IntConvertable, U: SizePrimitive>(&mut self, rect: Rec2<T, U>, color: RGBA) {
    self.set_color(color);
    self.subsystem
      .draw_rect(Rect::from(rect))
      .map_err(|error| eprintln!("{error}"))
      .ok();
  }
}

/// Create a new `sdl2::video::Window` with the given `RendererProperties`
fn build_window(context: &sdl2::Sdl, properties: &Properties) -> Result<sdl2::video::Window, String> {
  let (w, h) = properties.dimensions.destructure();
  let video_subsystem = context.video()?;

  let mut builder = video_subsystem.window(properties.title.as_str(), w, h);
  if properties.fullscreen { builder.fullscreen_desktop(); };
  if properties.opengl { builder.opengl(); };

  let window = builder.build().map_err(|e| e.to_string())?;
  Ok(window)
}
