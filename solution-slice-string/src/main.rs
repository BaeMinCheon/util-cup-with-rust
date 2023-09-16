
fn main() {
    let stdin = std::io::stdin();
    let mut token = String::new();
    let mut separators: std::collections::HashSet<char> = std::collections::HashSet::new();
    {
        token.clear();
        let _ = stdin.read_line(&mut token);
        token.clear();
        let _ = stdin.read_line(&mut token);
        for character in token.chars() {
            if character != ' ' {
                separators.insert(character);
            }
        }
    }
    {
        token.clear();
        let _ = stdin.read_line(&mut token);
        token.clear();
        let _ = stdin.read_line(&mut token);
        for character in token.chars() {
            if character != ' ' {
                separators.insert(character);
            }
        }
    }
    let mut jointers: std::collections::HashSet<char> = std::collections::HashSet::new();
    {
        token.clear();
        let _ = stdin.read_line(&mut token);
        token.clear();
        let _ = stdin.read_line(&mut token);
        for character in token.chars() {
            if character != ' ' {
                jointers.insert(character);
            }
        }
    }
    token.clear();
    let _ = stdin.read_line(&mut token);
    token.clear();
    let _ = stdin.read_line(&mut token);
    let mut results: Vec<String> = Vec::new();
    let mut initial_string = String::new();
    results.push(initial_string);
    for character in token.chars() {
        if jointers.contains(&character) {
            results.last_mut().unwrap().push(character);
        } else if (character == ' ') || separators.contains(&character) {
            if results.last().unwrap().is_empty() == false {
                let mut string = String::new();
                results.push(string);
            }
        } else {
            results.last_mut().unwrap().push(character);
        }
    }
    for result in results {
        println!("{}", result);
    }
}