use std::io::{self, BufRead, Write};

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

pub fn raw_ask_question<R, W>(reader: &mut R, mut writer: &mut W, question: &Question) -> Option<Answer>
    where R: BufRead,
          W: Write {
    write!(&mut writer, "{}\n", question.message).unwrap();

    let mut input = String::new();
    match reader.read_line(&mut input) {
        Ok(_) => {
            let answer_value = input.trim().to_string();
            if !answer_value.is_empty() {
                Some(Answer::new(question.name.to_owned(), answer_value))
            } else {
                None
            }
        },
        Err(error) => panic!("Unable to read line for {}: {}", question.name, error),
    }
}

pub fn raw_ask_questions<R, W>(reader: &mut R, writer: &mut W, questions: &Vec<Question>) -> Vec<Answer>
    where R: BufRead,
          W: Write {
    let non_empty_answers: Vec<Answer> = questions.iter().filter_map(|question| {
        raw_ask_question(reader, writer, &question)
    }).collect();

    non_empty_answers
}

pub fn ask_question(question: &Question) -> Option<Answer> {
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // A locked stdin implements BufRead.
    let mut writer = io::stdout();
    raw_ask_question(&mut reader, &mut writer, &question)
}

pub fn ask_questions(questions: &Vec<Question>) -> Vec<Answer> {
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // A locked stdin implements BufRead.
    let mut writer = io::stdout();
    raw_ask_questions(&mut reader, &mut writer, &questions)
}

#[cfg(test)]
mod tests {
    use super::Question;
    use super::Answer;

    use std::io::Cursor;
    use std::str::from_utf8;
    use super::{raw_ask_question, raw_ask_questions};

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
       let mut reader = Cursor::new(&b"Sebastian\n"[..]);
       let mut writer = Cursor::new(Vec::new());

       let question = Question::new("test-question", "What's your name?");

       let answer = raw_ask_question(&mut reader, &mut writer, &question).unwrap();

       assert_eq!(question.name, answer.name);
       assert_eq!("Sebastian", answer.value);
       assert_eq!("What's your name?\n", from_utf8(writer.get_ref()).unwrap());
   }

   #[test]
   fn can_ask_question_and_not_receive_answer() {
       let mut reader = Cursor::new(&b"\n"[..]);
       let mut writer = Cursor::new(Vec::new());

       let question = Question::new("test-question", "What's your name?");

       let answer = raw_ask_question(&mut reader, &mut writer, &question);

       assert_eq!(true, answer.is_none());
   }

   #[test]
   fn can_ask_questions() {
      let mut reader = Cursor::new(&b"First Answer\nSecond Answer\nThird Answer\n"[..]);
      let mut writer = Cursor::new(Vec::new());

      let questions = vec![
          Question::new(
              "first-question",
              "Tell me your first answer?",
          ),
          Question::new(
              "second-question",
              "Tell me your second answer?",
          ),
          Question::new(
              "third-question",
              "Tell me your third answer?",
          ),
      ];

      let answers = raw_ask_questions(&mut reader, &mut writer, &questions);

      assert_eq!(questions[0].name, answers[0].name);
      assert_eq!(questions[1].name, answers[1].name);
      assert_eq!(questions[2].name, answers[2].name);

      assert_eq!("First Answer", answers[0].value);
      assert_eq!("Second Answer", answers[1].value);
      assert_eq!("Third Answer", answers[2].value);

      assert_eq!("Tell me your first answer?\nTell me your second answer?\nTell me your third answer?\n", from_utf8(writer.get_ref()).unwrap());
  }
}
