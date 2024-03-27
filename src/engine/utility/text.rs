pub const NEW_LINE: char = '\n';
pub const COMMA: char = ',';

/// Remove newline characters from a string.
pub fn strip_newlines(s: &str) -> String {
  s.replace(NEW_LINE, "")
}
