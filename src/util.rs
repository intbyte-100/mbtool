use std::io::{self, Write};

use colored::Colorize;

#[inline]
pub(crate) fn input_string() -> String {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();
    string
}

// return true if answer is 'y' or false if 'n'
pub(crate) fn request_yes_or_no(request: &str) -> bool {
    loop {
        print!("{} {} ", request.green(), "[y/n]:".green());
        io::stdout().flush().unwrap();

        let string = input_string();
        let answer = string.trim();

        
        match answer {
            "y" => return true,
            "n" => return false,
            answer => {
                let message =
                    format!("'{}' is incorrect choice! Requires'y' or 'n'.", answer).red();
                println!("{}", message);
            }
        }
    }
}
