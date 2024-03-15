use crate::engine::event::Events;
use crate::engine::render::{Properties, Renderer};

pub struct Subsystem {
  pub sdl_context: sdl2::Sdl,
  pub renderer: Renderer,
  pub events: Events,
}

impl Subsystem {
  /// Attempt to instantiate a subsystem of `Properties`
  pub fn build<'a, 'b>(properties: Properties) -> Result<Self, String> {
    let sdl_context = sdl2::init()?;
    sdl_context.audio()?;

    let renderer = Renderer::build(&sdl_context, properties)?;
    let events = Events::build(&sdl_context)?;

    Ok(Self {
      sdl_context,
      renderer,
      events,
    })
  }
}
