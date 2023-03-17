pub struct Config<'a> {
  pub query: &'a str,
  pub file: &'a str
}

impl Config<'_> {

  pub fn new(args: &[String]) -> Option<Config> {

      if args.len() != 3 { return None }

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
      Some(Config { query: regx, file })

  }

}