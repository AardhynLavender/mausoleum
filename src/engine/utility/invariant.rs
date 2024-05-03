pub fn invariant<C>(condition: C, message: impl Into<String>) -> Result<(), String>
  where C: Into<bool> + std::ops::Not<Output=bool>
{
  return if !condition { Err(message.into()) } else { Ok(()) };
}