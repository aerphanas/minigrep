use super::{ search, search_case_insensitive };

#[test]
fn case_sensitive() {
  let query: &str = "duct";
  let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.";
  assert_eq!(
    vec!["safe, fast, productive."],
    search(query, contents)
  );
}

#[test]
fn case_insensitive() {
  let query: &str = "rUsT";
  let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
  assert_eq!(
    vec!["Rust:", "Trust me."],
    search_case_insensitive(query, contents)
  );
}
