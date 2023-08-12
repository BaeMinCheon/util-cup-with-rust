fn main() {
    let stdin = std::io::stdin();
    let mut token_number = String::new();
    let _ = stdin.read_line(&mut token_number);
    let number: usize = token_number.trim().parse().unwrap();
    let mut levels: Vec<usize> = Vec::new();
    for _ in 0..number {
        let mut token_level = String::new();
        let _ = stdin.read_line(&mut token_level);
        let level: usize = token_level.trim().parse().unwrap();
        levels.push(level);
    }
    let mut iteration_size: usize = 42;
    if levels.len() < iteration_size {
        iteration_size = levels.len();
    }
    else {
        levels.sort_by(|left, right| right.cmp(left));
    }
    let mut sum_level: usize = 0;
    let mut sum_power: usize = 0;
    for index in 0..iteration_size {
        let level: usize = levels[index];
        sum_level += level;
        match level {
            250..=300 => sum_power += 5,
            200..=249 => sum_power += 4,
            140..=199 => sum_power += 3,
            100..=139 => sum_power += 2,
            60..=99 => sum_power += 1,
            1..=59 => sum_power += 0,
            _ => panic!(),
        }
    }
    println!("{} {}", sum_level, sum_power);
}
