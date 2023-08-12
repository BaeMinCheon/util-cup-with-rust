
fn main() {
    let stdin = std::io::stdin();
    let mut token_1: String = String::new();
    let _ = stdin.read_line(&mut token_1);
    //let number: usize = token.parse().unwrap();
    let mut token_2: String = String::new();
    let _ = stdin.read_line(&mut token_2);
    let levels: Vec<&str> = token_2.split_whitespace().collect();
    for level in levels.iter() {
        let number: u32 = level.parse().unwrap();
        match number {
            300 => print!("1 "),
            275..=299 => print!("2 "),
            250..=274 => print!("3 "),
            1..=249 => print!("4 "),
            _ => panic!(),
        }
    }
}