use std::path::PathBuf;
use config::{Config, get_config_path, get_config_file_to_write, write_config};
use inquirer::{Question, Answer, ask_question};

use client::get_accounts;
use super::representations::represent_list_accounts;

pub fn configure_cli(config_file_path: &PathBuf) -> Option<Config> {
    match ask_questions_for_config() {
        None => None,
        Some(config) => {
            match get_config_file_to_write(&config_file_path) {
                Ok(mut config_file) => {
                    let _ = write_config(&mut config_file, &config);
                    Some(config)
                },
                Err(e) => panic!("ERROR: opening file to write: {}", e),
            }
        },
    }
}

fn ask_questions_for_config() -> Option<Config> {
    let get_auth_token_question = Question::new(
        "auth_token",
        "What is your `auth_token` on teller.io?",
    );

    let auth_token_answer = ask_question(&get_auth_token_question);

    let mut config = Config::new_with_auth_token_only(auth_token_answer.value);

    print!("\n");
    let accounts = match get_accounts(&config) {
        Ok(accounts) => accounts,
        Err(e) => panic!("Unable to list accounts: {}", e),
    };
    represent_list_accounts(&accounts, &config);

    println!("Please type the row (e.g. 3) of the account you wish to place against an alias and press <enter> to set this in the config. Leave empty if irrelevant.");
    print!("\n");

    let questions = vec![
        Question::new(
            "current",
            "Which is your current account?",
        ),
        Question::new(
            "savings",
            "Which is your savings account?",
        ),
        Question::new(
            "business",
            "Which is your business account?",
        ),
    ];

    let answers: Vec<Answer> = questions.iter().map(ask_question).collect();
    let non_empty_answers: Vec<&Answer> = answers.iter().filter(|&answer| !answer.value.is_empty()).collect();
    let mut fa_iter = non_empty_answers.iter();

    match fa_iter.find(|&answer| answer.name == "current") {
        None => (),
        Some(answer) => {
            let row_number: u32 = answer.value.parse().expect(&format!("ERROR: {:?} did not contain a number", answer));
            config.current = accounts[(row_number - 1) as usize].id.to_owned()
        },
    };
    match fa_iter.find(|&answer| answer.name == "savings") {
        None => (),
        Some(answer) => {
            let row_number: u32 = answer.value.parse().expect(&format!("ERROR: {:?} did not contain a number", answer));
            config.savings = accounts[(row_number - 1) as usize].id.to_owned()
        }
    };
    match fa_iter.find(|&answer| answer.name == "business") {
        None => (),
        Some(answer) => {
            let row_number: u32 = answer.value.parse().expect(&format!("ERROR: {:?} did not contain a number", answer));
            config.business = accounts[(row_number - 1) as usize].id.to_owned()
        }
    };

    if config.auth_token.is_empty() {
        error!("`auth_token` was invalid so a config could not be created");
        None
    } else {
        Some(config)
    }
}

pub fn initialise_command() -> i32 {
    let config_file_path = get_config_path();
    println!("To create the config ({}) we need to find out your `auth_token` and assign aliases to some common bank accounts.", config_file_path.display());
    print!("\n");
    configure_cli(&config_file_path);
    0
}
