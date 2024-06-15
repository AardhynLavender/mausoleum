use std::path::Path;

use crate::engine::utility::io::read_file;

const HELP_FILE: &str = "data/help.txt";

/// Load the help data from the help file
pub fn load_help_data() -> Result<Vec<String>, String> {
  let contents = read_file(Path::new(HELP_FILE))?;

  Ok(contents
    .lines()
    .map(|line| String::from(line))
    .collect::<Vec<String>>())
}
