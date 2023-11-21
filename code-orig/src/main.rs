// ====================================================================
// Imports
// ====================================================================
use grep::grep;
use constructors::GrepArgsConstructor;
mod grep;
mod constructors;
use r3bl_rs_utils::utils::{style_error, with};
use std::env::args;
use std::error::Error;
use std::process::exit;

// ====================================================================
// Function to execute the grep functionality
// ====================================================================
fn run_grep(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let grep_args = GrepArgsConstructor::parse(args)?; // Parse arguments using GrepArgsConstructor
    println!("Hi!");
    grep(grep_args)?; // Execute grep function with parsed arguments

    Ok(()) // Return Ok(()) if successful
}

// ====================================================================
// Function to handle errors and exit the program
// ====================================================================
fn handle_error(err: Box<dyn Error>) {
    eprintln!("{}: {}", style_error("Error Detected"), err);
    exit(1); // Exit with code 1 in case of an error
}

// ====================================================================
// Function to handle the result of running the grep functionality
// ====================================================================
fn handle_result(result: Result<(), Box<dyn Error>>) {
    match result {
        Ok(()) => exit(0), // Exit with code 0 if successful
        Err(err) => handle_error(err), // Handle error if an error occurs
    }
}

// ====================================================================
// Main
// ====================================================================
fn main() {
    let args = args().collect::<Vec<String>>(); // Fetch command-line arguments as a vector of strings

    let result = run_grep(args); // Execute the run_grep function using the command-line arguments

    handle_result(result); // Handle the result of running the grep functionality
}
