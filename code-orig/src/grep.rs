// ====================================================================
// Imports
// ====================================================================
use std::{error::Error, fs};
use crate::constructors::GrepArgs;
use r3bl_rs_utils::utils::{style_primary};

pub fn grep(options: GrepArgs) -> Result<(), Box<dyn Error>> {
  println!(
    "Looking for word: '{}' in file: '{}' {}",options.search, options.file_path,
    match options.case_sensitive {
      true => "case sensitive",
      false => "case insensitive",
    }
  );

  let content = fs::read_to_string(options.file_path)?; 
  // ? here - error handling - returns any errors
  println!("{}", content.lines().filter(|line| { // filter - picks the lines with the word in it
      if options.case_sensitive 
      {
        line.contains(&options.search)
      } 
      else 
      {
        line.to_lowercase().contains(&options.search.to_lowercase())
      }
    }).map(|line| { // format the word that is found
      let from = &options.search;
      let to = format!("{}", style_primary(&options.search));
      line.replace(from, &to)
    }).collect::<Vec<String>>().join("\n"));

  Ok(())
}
