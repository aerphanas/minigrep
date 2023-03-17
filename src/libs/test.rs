use super::*;

#[test]
fn one_ressult() {
  let query: &str = "duct";
  let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.";
  assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}
