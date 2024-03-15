use rand::distributions::uniform::SampleUniform;
use rand::Rng;

/**
 * Random utilities
 */

/// Generate a random number between `from` and `to`.
pub fn random<T>(from: T, to: T) -> T where T: SampleUniform + PartialOrd {
  rand::thread_rng().gen_range(from..to)
}