use sdl2::ttf::Sdl2TtfContext;

use crate::engine::asset::audio::{AudioPlayer, SoundType};
use crate::engine::asset::texture::TextureLoader;
use crate::engine::asset::typography::TypefaceLoader;
use crate::engine::render::Renderer;

pub mod audio;
/**
 * Combine the different assets loaders and stores into a single manager
 */
pub mod texture;
pub mod typography;

/// The different types of assets that can be managed
pub enum AssetType {
  Texture,
  Audio { sound_type: SoundType },
  Typeface { font_size: u16 },
}

/// Manages the loading and storage of game assets
pub struct AssetManager<'ttf> {
  pub texture: TextureLoader,
  pub audio: AudioPlayer,
  pub typeface: TypefaceLoader<'ttf, 'ttf>,
}

impl<'ttf> AssetManager<'ttf> {
  /// Instantiate a new asset manager
  pub fn new(renderer: &Renderer, ttf_context: &'ttf Sdl2TtfContext) -> Self {
    Self {
      texture: TextureLoader::new(renderer.new_texture_creator()),
      audio: AudioPlayer::new(),
      typeface: TypefaceLoader::new(&ttf_context),
    }
  }
}
