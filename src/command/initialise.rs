use std::path::PathBuf;
use config::{Config, get_config_path, get_config_file_to_write, write_config};
use inquirer::{Question, Answer, ask_question, ask_questions};

use api::TellerClient;
use command::representations::represent_list_accounts;

pub fn configure_cli(config_file_path: &PathBuf) -> Option<Config> {
    match ask_questions_for_config() {
        None => None,
        Some(config) => {
            match get_config_file_to_write(&config_file_path) {
                Ok(mut config_file) => {
                    let _ = write_config(&mut config_file, &config);
                    Some(config)
                }
                Err(e) => panic!("ERROR: opening file to write: {}", e),
            }
        }
    }
}

fn ask_questions_for_config() -> Option<Config> {
    let get_auth_token_question = Question::new("auth_token",
                                                "What is your `auth_token` on teller.io?");

    let auth_token_answer = match ask_question(&get_auth_token_question) {
        Some(auth_token_answer) => auth_token_answer,
        None => {
            error!("An `auth_token` needs to be entered to initialise the config.");
            return None; // Exit the whole function returning None.
        }
    };

    let mut config = Config::new_with_auth_token_only(auth_token_answer.value);

    print!("\n");
    let accounts = {
        let teller = TellerClient::new(&config.auth_token);
        match teller.get_accounts() {
            Ok(accounts) => accounts,
            Err(e) => panic!("Unable to list accounts: {}", e),
        }
    };
    represent_list_accounts(&accounts, &config);

    println!("Please type the row (e.g. 3) of the account you wish to place against an alias and \
              press <enter> to set this in the config. Leave empty if irrelevant.");
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

    let non_empty_answers = ask_questions(&questions);
    let mut fa_iter = non_empty_answers.iter();

    let to_account_id = |answer: &Answer| {
        let row_number: u32 = answer.value
                                    .parse()
                                    .expect(&format!("ERROR: {:?} did not contain a number",
                                                     answer));
        accounts[(row_number - 1) as usize].id.to_owned()
    };

    match fa_iter.find(|&answer| answer.name == "current").map(&to_account_id) {
        None => (),
        Some(account_id) => config.current = account_id,
    };
    match fa_iter.find(|&answer| answer.name == "savings").map(&to_account_id) {
        None => (),
        Some(account_id) => config.savings = account_id,
    };
    match fa_iter.find(|&answer| answer.name == "business").map(&to_account_id) {
        None => (),
        Some(account_id) => config.business = account_id,
    };

    Some(config)
}

pub fn initialise_command() -> i32 {
    info!("Calling the initialise command");
    let config_file_path = get_config_path();
    println!("To create the config ({}) we need to find out your `auth_token` and assign aliases \
              to some common bank accounts.",
             config_file_path.display());
    print!("\n");
    configure_cli(&config_file_path);
    0
}
