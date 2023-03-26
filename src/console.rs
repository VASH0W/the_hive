use std::{collections::VecDeque, fmt::Error};
use std::error::Error;
use std::fmt;

use crate::hive_objects::{self, HiveObject, HiveObjectsError};

pub mod hive_guard;

#[derive(Debug)]
pub struct ConsoleError {
    message: String,
}
pub struct HiveCommand {
    prompt: String,
    parameters: String,    
}
/// This function processes the arguments passed to the console function and constructs a relevant HiveCommand struct
fn parse_argument(argument: String) -> Option<HiveCommand> {
    let parameters_vec: Vec<&str> = argument.split(" ").collect();
    let mut parameters: VecDeque<&str> = VecDeque::from(parameters_vec);

    let instruction = match parameters.pop_front() {
        Some(i) => i.to_string(),
        None => return None,
    };
    let parameters = Vec::from(parameters).join(" ");
    Some(HiveCommand { prompt: instruction, parameters: parameters})
}


/// This function contains the logic to handle a HiveCommand executing a function based on the `HiveCommand.prompt`
/// Returns a Some(String) when successful and a None when not.
fn handle_command(command: HiveCommand) -> Result<String, ConsoleError> {
    let acceptable_commands = vec!["create", "destroy", "update", "show", "all", "join", "leave", "post", "vote"];
    let pattern: &str = &command.prompt;
    
    match pattern {
        "create" => Ok(create(&command.parameters).unwrap()),
        //"destroy" => destroy(&command.parameters),
        //"update" => update(&command.parameters),
        //"show" => show(&command.parameters),
        //"all" => all(&command.parameters),
        //"join" => join(&command.parameters),
        //"leave" => leave(&command.parameters),
        //"post" => post(&command.parameters),
        //"vote" => vote(&command.parameters),
        //"count" => count(&command.parameters),
        _ => Err(ConsoleError::InvalidCommand),
    }
}


/// This function creates a Hive object (`user`, `community`)  
fn create(parameters: &str) -> Result<String, ConsoleError> {
    let parameters = parameters.clone();
    let mut processedParameters: Vec<&str>  = parameters.split(" ").collect();
    let object: &str = match processedParameters.get(0) {
        Some(i) => i,
        None => return Err(ConsoleError::FailedInCreation),
    };
    let processedParameters = processedParameters[1..].join(" ");

    let spawned_object = match object {
        //"community" => hive_objects::Community.new(processedParameters),
        "user" => hive_objects::User::new(processedParameters),
        //"debate" => hive_objects::Debate.new(processedParameters),
        //"thought" => hive_objects::Thought.new(processedParameters),
        _ => Err(ConsoleError::FailedInCreation),
    };

    let object_id = spawned_object.get_id().to_string();
    return Ok(object_id);
}

#[cfg(test)]
mod console_tests {
    use super::*;

    #[test]
    fn parsing_arguments() {
        let command: String = String::from("create user 123455678");
        let test_command: HiveCommand = parse_argument(command).unwrap();
        assert_eq!("create", test_command.prompt);
        assert_eq!("user 123455678", test_command.parameters);
    }
    fn ensure_correct_handling() {
        let command = HiveCommand{prompt: "create".to_string(), parameters: "community".to_string()};
        assert_eq!(handle_command(command).unwrap(), "12345678".to_string());
    }
}