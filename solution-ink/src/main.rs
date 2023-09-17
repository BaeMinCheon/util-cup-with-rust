
struct InkGame {
    ink_string: String,
    stage: Vec<Vec<char>>,
    commands: String,
    stage_size: usize,
    player_row: usize,
    player_col: usize,
    ink_amount: usize,
    ink_string_index: usize,
}

impl InkGame {
    fn new(ink_string: String, stage: Vec<Vec<char>>, commands: String, player_row: usize, player_col: usize) -> Self {
        let stage_size = stage.len();
        InkGame {
            ink_string,
            stage,
            commands,
            stage_size,
            player_row,
            player_col,
            ink_amount: 0,
            ink_string_index: 0,
        }
    }

    fn execute(&mut self) {
        let commands = self.commands.clone();
        for command in commands.chars() {
            match command {
                'U' => self.command_U(),
                'D' => self.command_D(),
                'L' => self.command_L(),
                'R' => self.command_R(),
                'j' => self.command_j(),
                'J' => self.command_J(),
                _ => panic!(),
            }
        }
    }

    fn print(&self) {
        for line in &self.stage {
            let string: String = line.into_iter().collect();
            print!("{}", string);
        }
    }

    fn command_U(&mut self) {
        if self.player_row > 0 {
            let destination = self.stage[self.player_row - 1][self.player_col];
            if destination == '.' {
                self.player_row -= 1;
            }
        }
    }

    fn command_D(&mut self) {
        if self.player_row < (self.stage_size - 1) {
            let destination = self.stage[self.player_row + 1][self.player_col];
            if destination == '.' {
                self.player_row += 1;
            }
        }
    }

    fn command_L(&mut self) {
        if self.player_col > 0 {
            let destination = self.stage[self.player_row][self.player_col - 1];
            if destination == '.' {
                self.player_col -= 1;
            }
        }
    }

    fn command_R(&mut self) {
        if self.player_col < (self.stage_size - 1) {
            let destination = self.stage[self.player_row][self.player_col + 1];
            if destination == '.' {
                self.player_col += 1;
            }
        }
    }

    fn command_j(&mut self) {
        self.ink_amount += 1;
    }

    fn command_J(&mut self) {
        let ink: char = self.ink_string.chars().nth(self.ink_string_index).unwrap();
        if self.ink_amount > 0 {
            self.fill_stage(ink);
        }
        self.ink_string_index += 1;
        if self.ink_string_index >= self.ink_string.len() {
            self.ink_string_index = 0;
        }
    }

    fn fill_stage(&mut self, ink: char) {
        let mut row_start_i32 = self.player_row as i32 - self.ink_amount as i32;
        if row_start_i32 < 0 {
            row_start_i32 = 0;
        }
        let row_start = row_start_i32 as usize;
        let mut col_start_i32 = self.player_col as i32 - self.ink_amount as i32;
        if col_start_i32 < 0 {
            col_start_i32 = 0;
        }
        let col_start = col_start_i32 as usize;
        for row in row_start..(self.player_row + self.ink_amount + 1) {
            for col in col_start..(self.player_col + self.ink_amount + 1) {
                if (row < (self.stage_size - 1)) && (col < (self.stage_size - 1)) {
                    let row_diff = self.player_row as i32 - row as i32;
                    let col_diff = self.player_col as i32 - col as i32;
                    if (row_diff.abs() + col_diff.abs()) as usize <= self.ink_amount {
                        if self.stage[row][col] != '.' {
                            self.stage[row][col] = ink;
                        }
                    }
                }
            }
        }
        self.ink_amount = 0;
    }
}

fn main() {
    let stdin = std::io::stdin();
    let mut token = String::new();
    
    let _ = stdin.read_line(&mut token);
    let splits: Vec<&str> = token.trim().split_whitespace().collect();
    let integer_n: u32 = splits[1].parse().unwrap();
    let integer_k: u32 = splits[2].parse().unwrap();

    token.clear();
    let _ = stdin.read_line(&mut token);
    let ink_string = token.trim().to_string();

    let mut stage: Vec<Vec<char>> = Vec::new();
    let mut player_row: usize = 0;
    let mut player_col: usize = 0;
    for row in 0..integer_n {
        token.clear();
        let _ = stdin.read_line(&mut token);
        let mut line: Vec<char> = Vec::new();
        let mut col = 0;
        for character in token.chars() {
            if character == '@' {
                player_row = row as usize;
                player_col = col as usize;
                line.push('.');
            } else {
                line.push(character);
            }
            col += 1;
        }
        stage.push(line);
    }

    token.clear();
    let _ = stdin.read_line(&mut token);
    let commands = token.trim().to_string();

    let mut game = InkGame::new(ink_string, stage, commands, player_row, player_col);
    game.execute();
    game.print();
}