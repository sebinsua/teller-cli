use std::io::stdin;

#[derive(Debug)]
pub struct Question {
    pub _type: String,
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
    pub _type: String,
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

pub fn ask_questions(questions: &Vec<Question>) -> Vec<Answer> {
    let answers: Vec<Answer> = questions.iter().map(ask_question).collect();
    let non_empty_answers: Vec<Answer> = answers.into_iter()
                                                .filter(|answer| !answer.value.is_empty())
                                                .collect();
    non_empty_answers
}

#[cfg(test)]
mod tests {
    use super::Question;
    use super::Answer;

    #[test]
    fn can_instantiate_question() {
        let expected_type = "input";
        let expected_name = "";
        let expected_message = "";

        let question = Question::new(expected_name, expected_message);

        assert_eq!(expected_type, question._type);
        assert_eq!(expected_name, question.name);
        assert_eq!(expected_message, question.message);
    }

    #[test]
    fn can_instantiate_answer() {
        let expected_type = "input";
        let expected_name = "";
        let expected_value = "";

        let answer = Answer::new(expected_name, expected_value);

        assert_eq!(expected_type, answer._type);
        assert_eq!(expected_name, answer.name);
        assert_eq!(expected_value, answer.value);
    }
}
