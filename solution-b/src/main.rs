
struct Date {
    year: i32,
    month: i32,
    day: i32,
}

impl Date {
    fn new(raw_date: String) -> Self {
        let splits: Vec<&str> = raw_date.trim().split("-").collect();
        Self {
            year: splits[0].parse().unwrap(),
            month: splits[1].parse().unwrap(),
            day: splits[2].parse().unwrap(),
        }
    }
    fn is_later_than(self, other: &Date) -> bool {
        let mut result: bool = false;
        let year_delta: i32 = self.year - other.year;
        let month_delta: i32 = self.month - other.month;
        let day_delta: i32 = self.day - other.day;
        if year_delta > 0 {
            result = true;
        }
        else if year_delta == 0 {
            if month_delta > 0 {
                result = true;
            }
            else if month_delta == 0 {
                if day_delta >= 0 {
                    result = true;
                }
            }
        }
        return result;
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut token_today: String = String::new();
    let _ = stdin.read_line(&mut token_today);
    let today: Date = Date::new(token_today);
    let mut token_number: String = String::new();
    let _ = stdin.read_line(&mut token_number);
    let number: usize = token_number.trim().parse().unwrap();
    let mut count: u32 = 0;
    for _ in 0..number {
        let mut token_expire_date: String = String::new();
        let _ = stdin.read_line(&mut token_expire_date);
        let other: Date = Date::new(token_expire_date);
        if other.is_later_than(&today) {
            count += 1;
        }
    }
    println!("{}", count);
}
