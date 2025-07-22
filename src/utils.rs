pub fn clean_path<'a>(path: &'a str) -> &'a str {
  path.trim_start_matches(&['/', '\\'])
}
