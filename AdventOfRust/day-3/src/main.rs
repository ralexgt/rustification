use read_input::read_file;
use regex_split::RegexSplit;
use regex::Regex;

fn procces_input_p1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)").unwrap();
    let numbers: Vec<(&str, &str)> = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();
            (x, y)
        }).collect();
    
    let mut res = 0;
    for (x, y) in numbers.iter() {
        let x= (*x).parse::<i32>().unwrap();
        let y = (*y).parse::<i32>().unwrap();
        res += x * y;
    }
    
    res
}

fn procces_input_p2(input: &str) -> i32 {
    let mut result = 0;
    let re = Regex::new(r"don't()|do()").unwrap();
    
    let valid_inputs: Vec<&str> = re
        .split_inclusive_left(input)
        .into_iter()
        .filter(|&portion| {
            !portion.contains("don't()")
        })
        .collect();

    for &input in &valid_inputs {
        result += procces_input_p1(input);
    }

    result
}

fn main() {
    let puzzle_input = "input_day_3";
    let input = 
        match read_file(puzzle_input) {
            Ok(val) => val,
            Err(_) => panic!("Panicked while reading the file"),
        };
    let result = procces_input_p1(&input);
    println!("Advent of Code | Day 3 Part 1: {}", result);

    let result = procces_input_p2(&input);
    println!("Advent of Code | Day 3 Part 2: {}", result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3_p1() {
        let input = "
grAgarw[Mmul(1,3)grwgagrfulm(1,)]
fea1#%m^&mul*(6,4)mul(25,5)femult(1.5)feagfae\
        ";
        assert_eq!(128, procces_input_p1(input));
    }

    #[test]
    fn test_day_3_p2() {
        let input = "
grAgadon't()rw[Mmul(1,3)grwgado()grfulm(1,)]
fea1#%m^&mul*(6,4)mul(25,5)femult(1.5)feagfae\
        ";
        assert_eq!(125, procces_input_p2(input));
    }
}
