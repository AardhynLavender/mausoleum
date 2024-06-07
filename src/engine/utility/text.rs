pub const NEW_LINE: char = '\n';
pub const COMMA: char = ',';

/// Remove newline characters from a string.
pub fn strip_newlines(s: &str) -> String {
  s.replace(NEW_LINE, "")
}

/// Split a string into lines of `line_length` characters
pub fn split_text(text: &String, line_length: usize) -> Vec<String> {
  text
    .split_whitespace()
    .enumerate()
    .fold(vec![String::new()], |mut lines, (index, word)| {
      let last_line = lines.last_mut().expect("Failed to get last line");
      let space = if index == 0 || index == text.len() - 1 { "" } else { " " };
      if word.len() > line_length { panic!("'{}' is too long to fit in a line", word) }
      let new_line_length = last_line.len() + word.len() + space.len();
      if new_line_length > line_length {
        lines.push(word.to_string());
      } else {
        last_line.push_str(space);
        last_line.push_str(word);
      }
      lines
    })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_strip_newlines() {
    let text = String::from("The quick\nbrown fox\njumps over\nthe lazy dog");
    let stripped = strip_newlines(&text);
    assert_eq!(stripped, String::from("The quickbrown foxjumps overthe lazy dog"));
  }

  #[test]
  fn test_split_text() {
    let text = String::from("The quick brown fox jumps over the lazy dog");
    let lines = split_text(&text, 10);
    assert_eq!(lines, vec![
      String::from("The quick"),
      String::from("brown fox"),
      String::from("jumps over"),
      String::from("the lazy"),
      String::from("dog"),
    ]);
  }

  #[test]
  #[should_panic(expected = "'Disestablishmentarianism' is too long to fit in a line")]
  fn text_split_long_word() {
    let text = String::from("Disestablishmentarianism");
    split_text(&text, 10);
  }

  #[test]
  fn text_split_empty_text() {
    let text = String::default();
    let lines = split_text(&text, 10);
    assert_eq!(lines, vec![String::default()]);
  }
}
