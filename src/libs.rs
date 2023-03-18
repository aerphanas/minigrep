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
      Some(Config {
        query: regx,
        file,
        case
      })

  }

}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut temp: Vec<&str> = Vec::new();
  for line in contents.lines() {
    if line.contains(query) {
      temp.push(line);
    }
  }
  temp
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let low_query = query.to_lowercase();
  let mut temp: Vec<&str> = Vec::new();
  for line in contents.lines() {
    if line.to_lowercase().contains(&low_query) {
      temp.push(line);
    }
  }
  temp
}