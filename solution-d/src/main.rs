fn main() {
    let stdin = std::io::stdin();
    let mut token_size = String::new();
    let _ = stdin.read_line(&mut token_size);
    let matrix_size: usize = token_size.trim().parse().unwrap();
    let mut matrix: Vec<Vec<u32>> = Vec::new();
    for row in 0..matrix_size {
        let mut token_number = String::new();
        let _ = stdin.read_line(&mut token_number);
        let splits: Vec<&str> = token_number.trim().split_whitespace().collect();
        matrix.push(Vec::new());
        for col in 0..matrix_size {
            let number: u32 = splits[col].parse().unwrap();
            matrix[row].push(number);
        }
    }
    token_size.clear();
    let _ = stdin.read_line(&mut token_size);
    let operation_number: usize = token_size.trim().parse().unwrap();
    for _ in 0..operation_number {
        let mut token = String::new();
        let _ = stdin.read_line(&mut token);
        let splits: Vec<&str> = token.trim().split_whitespace().collect();
        // operation 1
        if splits.len() > 1 {
            let row_number: usize = splits[1].parse().unwrap();
            let row = row_number - 1;
            let element: u32 = matrix[row].pop().unwrap();
            matrix[row].insert(0, element);
        }
        // operation 2
        else {
            let mut depth: usize = 0;
            let mut target_column: usize = 0;
            let mut inner_matrix_size = matrix_size;
            while inner_matrix_size > 1 {
                let row_1: usize = depth;
                let col_1: usize = depth + target_column;
                let element_1: u32 = matrix[row_1][col_1];
                let row_2: usize = col_1;
                let col_2: usize = (matrix_size - 1) - row_1;
                let element_2: u32 = matrix[row_2][col_2];
                let row_3: usize = col_2;
                let col_3: usize = (matrix_size - 1) - row_2;
                let element_3: u32 = matrix[row_3][col_3];
                let row_4: usize = col_3;
                let col_4: usize = (matrix_size - 1) - row_3;
                let element_4: u32 = matrix[row_4][col_4];
                matrix[row_1][col_1] = element_4;
                matrix[row_2][col_2] = element_1;
                matrix[row_3][col_3] = element_2;
                matrix[row_4][col_4] = element_3;
                target_column += 1;
                if target_column >= (inner_matrix_size - 1) {
                    depth += 1;
                    target_column = 0;
                    inner_matrix_size -= 2;
                }
            }
        }
    }
    for row in 0..matrix_size {
        for col in 0..matrix_size {
            print!("{} ", matrix[row][col]);
        }
        println!();
    }
}
