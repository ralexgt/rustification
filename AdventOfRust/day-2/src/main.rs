use read_input::read_file;
use nom::{
    self,
    multi::separated_list1,
    character::complete,
    IResult
};
use itertools::Itertools;

type Report = Vec<i32>; // aliasing (Report is reffered in the puzzle)

// parse into a vector of vectors of ints
fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(
        complete::newline, 
        separated_list1(complete::space1,complete::i32)
    )(input)
}

fn is_safe(report: &Report) -> bool {
    let mut direction = 0;
    for (a, b) in report.iter().tuple_windows() {
        let diff = a-b;
        // if the direction changes, it fails
        // if a absolute difference is not 1, 2 or 3 it fails
        // if it doesnt fail continue with the next window of tuples
        // the direction can only be -1, 0, 1
        match diff.signum() { // signum() gives us the sign of the int basically giving the direction for the 2 value
            -1 => match direction {
                0 => {
                    if !(1..=3).contains(&diff.abs()) {
                        return false;
                        } else {
                            direction = -1;
                            continue;
                        }
                },
                1 => return false,
                -1 => if !(1..=3).contains(&diff.abs()) {
                    return false;
                    } else {
                        continue;
                    },
                _ => panic!("Should not have any other value"),
            }, 
            1 => match direction {
                0 => {
                    if !(1..=3).contains(&diff.abs()) {
                    return false;
                    } else {
                        direction = 1;
                        continue;
                    }
                },
                -1 => return false,
                1 => if !(1..=3).contains(&diff.abs()) {
                    return false;
                    } else {
                        continue;
                    },
                _ => panic!("Should not have any other value"),
            },
            _ => return false,
        }
    }
    println!("{report:?}");
    true
}

fn process_input_p1(input: &str) -> String {
    // panic if it errors, it shouldn't if the input is correct
    let reports = 
        match parse(input){
            Ok(val) => val,
            Err(_) => panic!("panicked in parsing"),
        };
    let safe_reports = reports.1
        .iter()
        .filter(|&report| {
            is_safe(report)
        })
        .count();

    
    safe_reports.to_string()
} 

fn main() {
    // read the input for the puzzle from an input file
    let puzzle_input = "input_day_2";
    let input = 
        match read_file(&puzzle_input) {
            Ok(result) => result,
            Err(e) => panic!("{}", e),
        };
    let result = process_input_p1(&input);
    println!("Advent of Code | Day 2 Part 1: {}", result);

    // to do part 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_2_p2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!("2", process_input_p1(input));
    }
}