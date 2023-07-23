pub fn print() {
    let start_char: char = 'Z';
    let end_char: char = 'a';

    for a in start_char ..= end_char {
        println!("{}",a);
    }
}