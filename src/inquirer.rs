use std::io::stdin;

#[derive(Debug)]
pub struct Question {
    _type: String,
    pub name: String,
    pub message: String,
}

impl Question {
    pub fn new(name: String, message: String) -> Question {
        Question {
            _type: "input".to_string(),
            name: name,
            message: message,
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
    pub fn new(name: String, value: String) -> Answer {
        Answer {
            _type: "input".to_string(),
            name: name,
            value: value,
        }
    }
}


pub fn ask_question(question: &Question) -> Answer {
    println!("{}", question.message);
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => Answer::new(question.name.to_owned(), input.trim().to_string()),
        Err(error) => panic!("ERROR: {}", error),
    }
}
