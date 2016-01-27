use std::io::{BufRead, Write};

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

pub fn ask_question<R, W>(reader: &mut R, mut writer: &mut W, question: &Question) -> Answer
    where R: BufRead,
          W: Write {
    write!(&mut writer, "{}\n", question.message).unwrap();

    let mut input = String::new();
    match reader.read_line(&mut input) {
        Ok(_) => Answer::new(question.name.to_owned(), input.trim().to_string()),
        Err(error) => panic!("Unable to read line for {}: {}", question.name, error),
    }
}

pub fn ask_questions<R, W>(reader: &mut R, writer: &mut W, questions: &Vec<Question>) -> Vec<Answer>
    where R: BufRead,
          W: Write {
    let answers: Vec<Answer> = questions.iter().map(move |question| {
        ask_question(reader, writer, &question)
    }).collect();
    let non_empty_answers: Vec<Answer> = answers.into_iter()
                                                .filter(|answer| !answer.value.is_empty())
                                                .collect();
    non_empty_answers
}

#[cfg(test)]
mod tests {
    use super::Question;
    use super::Answer;

    use std::io::Cursor;
    use std::str::from_utf8;
    use super::ask_question;

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

    #[test]
    fn can_ask_question() {
       let mut reader = Cursor::new(&b"Sebastian"[..]);
       let mut writer = Cursor::new(Vec::new());

       let question = Question::new("test-question", "What's your name?");

       let answer = ask_question(&mut reader, &mut writer, &question);

       assert_eq!(question.name, answer.name);
       assert_eq!("Sebastian", answer.value);
       assert_eq!("What's your name?\n", from_utf8(writer.get_ref()).unwrap());
   }
}
