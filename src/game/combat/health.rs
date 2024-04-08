use num::clamp;

pub const MIN_HEALTH: i32 = 0;

#[derive(PartialEq, Debug)]
pub enum LiveState { Alive(i32), Dead }

#[derive(Debug)]
pub struct Health {
  current: i32,
  max: i32,
}

impl Health {
  pub fn build(max: i32) -> Result<Self, String> {
    if max <= MIN_HEALTH { return Err(String::from("Health must be greater than or equal to 0.")); }
    Ok(Self { current: max, max })
  }
  /// Is the entity alive?
  pub fn get_state(&self) -> LiveState {
    if self.current <= MIN_HEALTH { LiveState::Dead } else { LiveState::Alive(self.current) }
  }
  pub fn get_health(&self) -> i32 { self.current }
  pub fn get_max(&self) -> i32 { self.max }
  /// Damage the health by the given amount.
  pub fn deal(&mut self, amount: i32) -> LiveState {
    self.current = clamp(self.current - amount, MIN_HEALTH, self.max);
    self.get_state()
  }
  /// Heal the health by the given amount.
  pub fn heal(&mut self, amount: i32) -> LiveState {
    self.deal(-amount)
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