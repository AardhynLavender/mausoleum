use std::path::Path;

use crate::engine::utility::io::read_file;

const CREDITS_FILE: &str = "data/credits.txt";

pub fn load_credits() -> Result<Vec<String>, String> {
  let contents = read_file(Path::new(CREDITS_FILE))?;

  Ok(contents
    .lines()
    .map(|line| String::from(line))
    .collect::<Vec<String>>())
}