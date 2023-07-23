pub fn print() {
    let start_char: char = 'A';
    let end_char: char = 'z';

    for a in start_char ..= end_char {
        println!("{}",a);
    }
}