use crate::models::{self, general::llm::Message};

// Extend ai function to encourage certain specific output
pub fn extend_ai_function(ai: fn(&str) -> &'static str, func_input: &str) -> Message{
    let ai_function_str = ai(func_input);
    
    // Extend the tring to encourage only printing the output
    let msg = format!("FUNCTION {}
    INSTRUCTION: You are a function printer. You ONLY printthe results of functions.
    Nothing else. No commentary. Here is the input to the function: {}. Print out what the function will return.",
     ai_function_str, func_input);

     // Return message
     Message {
        role: "system".to_string(),
        content: msg
     }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;

    #[test]
    fn tests_extending_ai_functions(){
       let extended_msg = extend_ai_function(convert_user_input_to_goal, "dummy variable");
       dbg!(&extended_msg);
       assert_eq!(extended_msg.role, "system".to_string());
    }
}