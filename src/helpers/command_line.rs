use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand
};
use std::io::{stdin, stdout, Read};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    AICall, 
    UnitTest, 
    Issue
}

impl PrintCommand {
    pub fn print_agent_message(&self, agent_pos: &str, agent_statement: &str) -> String{
        let mut stdout: std::io::Stdout = stdout();

        // Decide on the color 
        let statement_color: Color = match self {
            Self::AICall => Color::Cyan,
            Self::UnitTest => Color::Magenta,
            Self::Issue => Color::Red,
        };

        // Print the agent statement in specific color
        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        print!("Agent: {}: ", agent_pos);

        // Make selected color
        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("{}", agent_statement);

        // Reset color
        stdout.execute(ResetColor).unwrap();

        // Read user input
        let mut user_response: String = String::new();

        stdin()
            .read_line(&mut user_response)
            .expect("Failed to read response");

        // Trim whitespace and return
        return user_response.trim().to_string();

    }
}


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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prints_agent_msg(){
        PrintCommand::AICall.print_agent_message("Managing Agent", "Tell me a joke");
    }
}