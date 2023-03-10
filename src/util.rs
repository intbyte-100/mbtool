use std::io;

#[inline]
pub(crate) fn input_string() -> String {
    let mut string = String::new();
    io::stdin().read_line(&mut string).unwrap();
    string
}