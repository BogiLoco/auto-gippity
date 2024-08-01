use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand
};
use std::io::{stdin, stdout, Read};


// Get user request
pub fn get_user_response(question: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    // Print the question in a sepcific color
    stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
    println!("");
    println!("{}", question);

    // Reset the color
    stdout.execute(ResetColor).unwrap();

    // Read user input
    let mut user_response: String = String::new();
    stdin()
        .read_line(&mut user_response)
        .expect("Failed to read response");
    
    // Trim whitespace and return
    return  user_response.trim().to_string();
}