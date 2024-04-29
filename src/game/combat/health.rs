use num::clamp;

pub const MIN_HEALTH: u32 = 0;

#[derive(PartialEq, Debug)]
pub enum LiveState { Alive(u32), Dead }

#[derive(Debug)]
pub struct Health {
  current: u32,
  max: u32,
}

impl Health {
  pub fn build(max: u32) -> Result<Self, String> {
    if max <= MIN_HEALTH { return Err(String::from("Health must be greater than or equal to 0.")); }
    Ok(Self { current: max, max })
  }
  /// Is the entity alive?
  pub fn get_state(&self) -> LiveState {
    if self.current <= MIN_HEALTH { LiveState::Dead } else { LiveState::Alive(self.current) }
  }
  /// Get the current health.
  pub fn get_health(&self) -> u32 { self.current }
  /// Get the maximum health.
  pub fn get_max(&self) -> u32 { self.max }
  /// Raise the maximum health by the given amount.
  pub fn set_max(&mut self, amount: u32) {
    self.max = amount;
    if self.current < self.max { self.current = self.max; }
  }

  /// Damage the health by the given amount.
  pub fn deal(&mut self, amount: u32) -> LiveState {
    let after = self.current as i32 - amount as i32;
    self.current = clamp(after, MIN_HEALTH as i32, self.max as i32) as u32;
    self.get_state()
  }
  /// Heal the health by the given amount.
  pub fn heal(&mut self, amount: u32) -> LiveState {
    self.current = clamp(self.current + amount, MIN_HEALTH, self.max);
    self.get_state()
  }
}

impl std::fmt::Display for Health {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "{}/{}", self.current, self.max)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_health() {
    let health = Health::build(100).unwrap();
    assert_eq!(health.get_health(), 100);
  }

  #[test]
  fn test_deal() {
    let mut health = Health::build(100).unwrap();
    assert_eq!(health.deal(50), LiveState::Alive(50));
  }

  #[test]
  fn test_heal() {
    let mut health = Health::build(100).unwrap();
    health.deal(50);
    assert_eq!(health.heal(25), LiveState::Alive(75));
  }

  #[test]
  fn test_dead() {
    let mut health = Health::build(100).unwrap();
    assert_eq!(health.deal(150), LiveState::Dead);
  }

  #[test]
  fn test_overkill() {
    let mut health = Health::build(100).unwrap();
    assert_eq!(health.deal(150), LiveState::Dead);
    assert_eq!(health.get_health(), 0);
  }

  #[test]
  fn test_overheal() {
    let mut health = Health::build(100).unwrap();
    assert_eq!(health.heal(25), LiveState::Alive(100));
  }

  #[test]
  fn test_display() {
    let mut health = Health::build(100).unwrap();
    health.deal(50);
    assert_eq!(format!("{}", health), "50/100");
  }
}