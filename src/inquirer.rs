use std::io::stdin;

#[derive(Debug)]
pub struct Question {
    _type: String,
    pub name: String,
    pub message: String,
}

impl Question {
    pub fn new<S: Into<String>>(name: S, message: S) -> Question {
        Question {
            _type: "input".to_string(),
            name: name.into(),
            message: message.into(),
        }
    }
}

#[derive(Debug)]
pub struct Answer {
    _type: String,
    pub name: String,
    pub value: String,
}

impl Answer {
    pub fn new<S: Into<String>>(name: S, value: S) -> Answer {
        Answer {
            _type: "input".to_string(),
            name: name.into(),
            value: value.into(),
        }
    }
}


pub fn ask_question(question: &Question) -> Answer {
    let question_name = question.name.to_owned();
    println!("{}", question.message);
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => Answer::new(question_name, input.trim().to_string()),
        Err(error) => panic!("Unable to read line for {}: {}", question_name, error),
    }
}
