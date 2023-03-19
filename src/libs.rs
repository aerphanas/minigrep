use std::env;

mod test;

pub struct Config<'a> {
  pub query: &'a str,
  pub file: &'a str,
  pub case: bool
}

impl Config<'_> {

  pub fn new(args: &[String]) -> Option<Config> {

      if args.len() != 3 { return None }

      let case = env::var("IGNORE_CASE").is_ok();

      let regx: &str = if let Some(num) = args.get(1) {
          num
      } else {
          return None
      };

      let file: &str = if let Some(name) = args.get(2) {
        name
      } else {
          return None
      };

      Some(
        Config {
          query: regx,
          file,
          case
      })

  }

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents.lines()
    .filter(|line| line.contains(&query))
    .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents.lines()
    .filter(|line| {
      line.to_lowercase().contains(&query.to_lowercase())
    })
    .collect()
}