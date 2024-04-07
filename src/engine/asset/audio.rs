use std::path::Path;

use crate::engine::store::Store;
use crate::game::utility::path::get_filename;

/**
 * Loading and playing music and sfx
 */

/// Type of sound
pub enum SoundType {
  Music,
  Effect,
}

/// Sound data
pub enum Sound {
  Music { data: sdl2::mixer::Music<'static> },
  Effect { data: sdl2::mixer::Chunk },
}

/// Looping behavior
pub enum Loop {
  Forever,
  Once,
}

/// Audio data
pub struct Audio {
  pub sound: Sound,
  pub name: String,
  pub path: Box<Path>,
}

/// Store music and sfx
type AudioStore = Store<String, Audio>;

type AudioKey = String;

/// load and play music and sfx
pub struct AudioPlayer {
  store: AudioStore,
}

impl AudioPlayer {
  /// Instantiate a new audio player
  pub fn new() -> Self {
    initialize_audio_subsystem().expect("Failed to initialize audio subsystem");
    Self {
      store: AudioStore::new(),
    }
  }

  /// Load a sfx or music file
  pub fn load(&mut self, sound_type: SoundType, filepath: impl AsRef<Path>) -> Result<AudioKey, String> {
    let basename = get_filename(&filepath)?;

    match sound_type {
      SoundType::Music => {
        let music =
          sdl2::mixer::Music::from_file(&filepath).expect("Failed to load music");
        let audio = Audio {
          sound: Sound::Music { data: music },
          name: basename.clone(),
          path: filepath
            .as_ref()
            .to_path_buf()
            .into_boxed_path(),
        };
        Ok(self.store.add(audio.name.clone(), audio).name.clone())
      }
      SoundType::Effect => {
        let effect = sdl2::mixer::Chunk::from_file(&filepath)
          .expect("Failed to load sound effect");
        let audio = Audio {
          sound: Sound::Effect { data: effect },
          name: String::from(basename),
          path: filepath
            .as_ref()
            .to_path_buf()
            .into_boxed_path(),
        };
        Ok(self.store.add(audio.name.clone(), audio).name.clone())
      }
    }
  }

  /// Play a sfx or music
  pub fn play(&self, name: &str, volume: i32, looping: Loop) -> Result<(), String> {
    let audio = self.store.get(name.to_string())?;
    let loops = match looping {
      Loop::Forever => -1,
      Loop::Once => 0,
    };

    match &audio.sound {
      Sound::Music { data } => {
        sdl2::mixer::Music::set_volume(volume);
        data.play(loops).map_err(|_| "Failed to play music")?;
        Ok(())
      }
      Sound::Effect { data } => {
        let channel = sdl2::mixer::Channel::all().play(data, loops)?;
        channel.set_volume(volume);
        Ok(())
      }
    }
  }

  /// Stop a playing sfx or music
  pub fn stop(&self, name: &str) -> Result<(), String> {
    let audio = self.store.get(name.to_string())?;
    match &audio.sound {
      Sound::Music { data: _ } => sdl2::mixer::Music::halt(),
      Sound::Effect { data: _ } => {
        unimplemented!("Stopping sound effects is not yet implemented")
      }
    }
    Ok(())
  }
}

// Subsystem //

/// Samples per second
pub const FREQUENCY: i32 = 44_100;
// 44.1 kHz
/// Signed 16-bit samples
pub const FORMAT: sdl2::mixer::AudioFormat = sdl2::mixer::DEFAULT_FORMAT;
/// 2 channels (stereo)
pub const OUTPUT_CHANNELS: i32 = sdl2::mixer::DEFAULT_CHANNELS;
/// Number of channels available for mixing sound effects
pub const MIXER_CHANNELS: i32 = 16;
/// Samples processed per frame
pub const CHUNK_SIZE: i32 = 2048;

/// Initialize the SDL_Mixer audio subsystem
fn initialize_audio_subsystem() -> Result<(), String> {
  sdl2::mixer::open_audio(FREQUENCY, FORMAT, OUTPUT_CHANNELS, CHUNK_SIZE)?;
  sdl2::mixer::init(sdl2::mixer::InitFlag::all())?;
  sdl2::mixer::allocate_channels(MIXER_CHANNELS);
  Ok(())
}
