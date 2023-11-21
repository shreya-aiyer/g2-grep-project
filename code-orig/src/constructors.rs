// ====================================================================
// To hold input arguments from command line
// ====================================================================
pub struct GrepArgs {
  pub search: String,
  pub file_path: String,
  pub case_sensitive: bool,
}

pub struct GrepArgsConstructor;

impl GrepArgsConstructor {
  pub fn parse(cli_inp: Vec<String>) -> Result<GrepArgs, String> {
    // Error handling
    if cli_inp.len() < 3 {
      return Err(format!("Expected at least {} arguments, got {}.",3,cli_inp.len()));
    }

    let mut cli_inp = cli_inp.iter();
    cli_inp.next(); // Skip the first argument.
    
    // ====================================================================
    // Filling values into the constructor
    // ====================================================================
    let gre_optns = GrepArgs {
      search: match cli_inp.next() {
        Some(arg) => arg.clone(),
        None => String::new(), // Empty string if no input
      },
      file_path: match cli_inp.next() {
        Some(arg) => arg.clone(),
        None => String::new(), // Empty string if no input
      },
      case_sensitive: cli_inp.next().is_some(),  // If case sensitive
    };

    Ok(gre_optns)
  }
}
