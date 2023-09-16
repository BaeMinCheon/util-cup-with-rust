fn main() {
    let stdin = std::io::stdin();
    let mut token = String::new();
    let _ = stdin.read_line(&mut token);
    let study_days: usize = token.trim().parse().unwrap();
    let mut goals: Vec<u32> = Vec::new();
    {
        token.clear();
        let _ = stdin.read_line(&mut token);
        let splits: Vec<&str> = token.trim().split_whitespace().collect();
        for index in 0..study_days {
            let goal: u32 = splits[index].parse().unwrap();
            goals.push(goal);
        }
    }
    let mut progresses: Vec<u32> = Vec::new();
    {
        token.clear();
        let _ = stdin.read_line(&mut token);
        let splits: Vec<&str> = token.trim().split_whitespace().collect();
        for index in 0..study_days {
            let progress: u32 = splits[index].parse().unwrap();
            progresses.push(progress);
        }
    }
    let mut result = 0;
    for index in 0..study_days {
        let goal = goals[index];
        let progress = progresses[index];
        if progress >= goal {
            result += 1;
        }
    }
    println!("{}", result);
}
