use std::{
    io::{self, Write},
    print, println,
};

use steam_tables::{data, error::Error, saturated_steam::SteamTable};

const PROMPT: &str = "--->";
const ANSWER_BRACKET: &str = "---------------------------";

struct TableHolder {
    pub saturated_by_temperature_table: SteamTable,
    pub saturated_by_pressure_table: SteamTable,
}

impl TableHolder {
    pub fn new() -> TableHolder {
        let saturated_by_temperature_table =
            SteamTable::new(data::SATURATED_BY_TEMPERATURE_TABLE.to_string()).unwrap();

        let saturated_by_pressure_table =
            SteamTable::new(data::SATURATED_BY_PRESSURE_TABLE.to_string()).unwrap();

        TableHolder {
            saturated_by_temperature_table,
            saturated_by_pressure_table,
        }
    }
}

fn main() {
    let table_holder = TableHolder::new();

    print_intro();

    loop {
        let input_result = get_user_input();

        let mut user_input: Vec<String> = Vec::new();

        match input_result {
            Ok(mut input) => {
                if input.len() == 0 {
                    continue;
                }

                user_input.append(&mut input);
            }
            Err(error) => {
                println!("Error occurred {:?}", error);
                continue;
            }
        }

        let handle_result = handle_user_input(user_input, &table_holder);

        match handle_result {
            Ok(state) => {
                if state == InteractiveState::Stop {
                    break;
                }
            }

            Err(error) => {
                println!("Error Occurred, {:?}", error);
            }
        }
    }

    print_outro();
}

fn print_intro() {
    let intro = include_str!("../data/intro.txt");
    println!("{}", intro);
}

fn get_user_input() -> Result<Vec<String>, InteractiveError> {
    print!("{} ", PROMPT);
    if io::stdout().flush().is_err() {
        return Err(InteractiveError::OutputError(
            "Error writing to stdout!".to_string(),
        ));
    }

    let mut input: String = String::new();
    if io::stdin().read_line(&mut input).is_err() {
        return Err(InteractiveError::InputError(
            "Error reading user input!".to_string(),
        ));
    }

    let input_slices = input
        .split(" ")
        .map(|str| str.trim().to_string())
        .collect::<Vec<String>>();

    Ok(input_slices)
}

fn handle_user_input(
    user_input: Vec<String>,
    table_holder: &TableHolder,
) -> Result<InteractiveState, InteractiveError> {
    if user_input[0].as_str().to_lowercase() == "quit" {
        return Ok(InteractiveState::Stop);
    }

    if user_input[0].as_str().to_lowercase() == "help" {
        print_help(user_input)?;
        return Ok(InteractiveState::Continue);
    }

    if user_input.len() < 2 {
        let input = vec!["help".to_string(), user_input[0].to_owned()];
        print_help(input)?;
        return Ok(InteractiveState::Continue);
    }

    if user_input[0].as_str().to_lowercase() == "saturated-steam"
        || user_input[0].as_str().to_lowercase() == "ss"
    {
        query_saturated_steam(user_input, table_holder)?;
    }

    Ok(InteractiveState::Continue)
}

fn query_saturated_steam(
    user_input: Vec<String>,
    table_holder: &TableHolder,
) -> Result<(), InteractiveError> {
    let parameter = user_input[2].parse::<f32>();
    if parameter.is_err() {
        let err_str = format!("{} can not be parsed to float", user_input[2]);
        return Err(InteractiveError::ParseFloatError(err_str));
    }

    let parameter = parameter.unwrap();

    if user_input[1].as_str().to_lowercase() == "temperature"
        || user_input[1].as_str().to_lowercase() == "t"
    {
        query_saturated_steam_at_parameter("temperature", parameter, table_holder)?;
    }

    if user_input[1].as_str().to_lowercase() == "pressure"
        || user_input[1].as_str().to_lowercase() == "p"
    {
        query_saturated_steam_at_parameter("pressure", parameter, table_holder)?;
    }

    Ok(())
}

fn query_saturated_steam_at_parameter(
    parameter_name: &str,
    parameter_value: f32,
    table_holder: &TableHolder,
) -> Result<(), InteractiveError> {
    let query_result = if parameter_name == "temperature" {
        table_holder
            .saturated_by_temperature_table
            .get_values_at_point(parameter_value)
    } else {
        table_holder
            .saturated_by_pressure_table
            .get_values_at_point(parameter_value)
    };

    if let Err(query) = query_result {
        match query {
            Error::ValueOutOfRange(min, max) => {
                let err_str = format!(
                    "{} value should be between {} and {} ",
                    parameter_name, min, max
                );
                return Err(InteractiveError::InputError(err_str));
            }

            _ => {
                return Err(InteractiveError::InputError(
                    "Something happened when querying saturated steam data".to_string(),
                ))
            }
        }
    }

    let query = query_result.unwrap();
    print_data(query);
    Ok(())
}

fn print_outro() {}

fn print_help(user_input: Vec<String>) -> Result<InteractiveState, InteractiveError> {
    if user_input.len() == 1 {
        print_complete_help();
    } else if user_input.len() > 1 {
        let help_option = user_input[1].to_owned();
        if help_option.to_lowercase().as_str() == "saturated-steam"
            || help_option.to_lowercase().as_str() == "ss"
        {
            print_saturated_steam_help();
        } else {
            return Err(InteractiveError::UnRecognizedParameter(help_option));
        }
    }

    Ok(InteractiveState::Continue)
}

fn print_complete_help() {
    let help = include_str!("../data/help.txt");
    println!("{}", help);
}

fn print_saturated_steam_help() {
    let help = include_str!("../data/saturated_steam_help.txt");
    println!("{}", help);
}

#[derive(Debug)]
enum InteractiveError {
    OutputError(String),
    InputError(String),
    UnRecognizedParameter(String),
    ParseFloatError(String),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum InteractiveState {
    Continue,
    Stop,
}

fn print_data(data: Vec<(String, f32)>) {
    println!("{}\n", ANSWER_BRACKET);
    for value in data {
        println!("{}: {}", value.0, value.1);
    }
    println!("\n{}", ANSWER_BRACKET);
}
