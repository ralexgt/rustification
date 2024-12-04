// Part 1: 
// The general idea is to parse the input into a matrix of characters. The matrix would be n lines by m characters on each line given that
// the input has the same amount of characters on each line. 
// I go through the matrix line by line and check for either XMAS or SAMX, then the same for column, then the same for all diagonals.
use diagonal;
use read_input::read_file;


type Matrix = Vec<Vec<char>>;

// take the original input and make a matrix each cell containing a character
fn get_mat(input: &str) -> Matrix {
    let mut final_mat = vec![];
    for line in input.lines() {
        let mut line_vec = vec![];
        for character in line.as_bytes() {
            line_vec.push(*character as char);     
        }
        final_mat.push(line_vec);
    }
    final_mat
}

fn process_by_lines(input: &Matrix) -> i32 {
    let mut count = 0;
    for line in input.iter() {
        let s: String = line.iter().collect();
        let s: Vec<&str> = s.split("XMAS").collect();
        count += s.len() - 1;
        let s: String = line.iter().collect();
        let s: Vec<&str> = s.split("SAMX").collect();
        count += s.len() - 1;
    }
    count as i32
}

fn process_by_col(input: &Matrix) -> i32 {
    let mut count = 0;
    for j in 0..input.len() {
        let mut s = vec![];
        for i in 0..input[0].len() {
            s.push(input[i][j]);
        }
        let s1: String = s.iter().collect();
        let s1: Vec<&str> = s1.split("XMAS").collect();
        count += s1.len() - 1;
        let s1: String = s.iter().collect();
        let s1: Vec<&str> = s1.split("SAMX").collect();
        count += s1.len() - 1;
    }
    count as i32
}

fn process_by_diag(input: &Matrix) -> i32 {
    let mut count = 0;
    
    // takes all secondary diagonals and puts them in a vector of vectors
    let s = diagonal::diagonal_pos_pos(input);
    
    // go through all secondary diagonals and check for XMAS, then SAMX
    for diag in s.clone() {
        let s1: String = diag.into_iter().collect();
        let s1: Vec<&str> = s1.split("XMAS").collect();
        count += s1.len() - 1;
    }
    for diag in s.clone() {
        let s1: String = diag.into_iter().collect();
        let s1: Vec<&str> = s1.split("SAMX").collect();
        count += s1.len() - 1;
    }

    let s = diagonal::diagonal_pos_neg(input);
    
    for diag in s.clone() {
        let s1: String = diag.into_iter().collect();
        let s1: Vec<&str> = s1.split("XMAS").collect();
        count += s1.len() - 1;
    }
    for diag in s.clone() {
        let s1: String = diag.into_iter().collect();
        let s1: Vec<&str> = s1.split("SAMX").collect();
        count += s1.len() - 1;
    }

    count as i32
}

fn process_input_p1(input: &str) -> i32 {
    let mut res = 0;
    let input = get_mat(input);
    res += process_by_lines(&input);
    res += process_by_col(&input);
    res += process_by_diag(&input);
    res
}

// Part 2
// I took the matrix of character from part 1 and went through it extracting every chunk of 3x3 chunks.
// I checked the diagonals, as both of them had to be either MAS or SAM, I incremented the result by
// 1 for each chunk satisfying the puzzle's requirement.
 
fn process_input_p2(input: &str) -> i32 {
    let mut res = 0;
    let input = get_mat(input);
    
    let mut chunk: Matrix = vec![];
    for i in 0..=input.len()-3 {
        for j in 0..=input[0].len()-3 {
            chunk.push(vec![input[i][j], input[i][j+1], input[i][j+2]]);
            chunk.push(vec![input[i+1][j], input[i+1][j+1], input[i+1][j+2]]);
            chunk.push(vec![input[i+2][j], input[i+2][j+1], input[i+2][j+2]]);
            
            let diag_p = format!("{}{}{}", chunk[0][0], chunk[1][1], chunk[2][2]);
            let diag_s = format!("{}{}{}", chunk[0][2], chunk[1][1], chunk[2][0]);
            if (diag_p.contains("MAS") || diag_p.contains("SAM")) && (diag_s.contains("MAS") || diag_s.contains("SAM")) {
                res += 1;
            }
            chunk.clear();
        }
    }

    res
}

fn main() {
    let puzzle_path = "input_day_4";
    let input = 
        match read_file(&puzzle_path) {
            Ok(val) => val,
            Err(_) => panic!("Should be able to read file"),
        };
    let result = process_input_p1(&input);
    println!("Advent of Code | Day 4 Part 1: {}", result);
    let result = process_input_p2(&input);
    println!("Advent of Code | Day 4 Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_day_4_p1() {
        let input = "MMMSXXMASM
MSAMXMSMSA  
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(18, process_input_p1(&input));
    }

    #[test]
    fn test_day_4_p2() {
        let input = "MMMSXXMASM
MSAMXMSMSA  
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(9, process_input_p2(&input));
    }
}