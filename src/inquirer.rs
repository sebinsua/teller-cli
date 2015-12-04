use std::io::stdin;

pub fn ask_question() {
    println!("What's the auth token?");
    let mut input = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => println!("{}", input),
        Err(error) => println!("error: {}", error),
    }

    ()
}
