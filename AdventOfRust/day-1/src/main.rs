use read_input::read_file;

fn process_input_p1(input: &str) -> String {
    let mut left = vec![];
    let mut right = vec![];
    
    for line in input.lines() {
        let mut locations = line.split_ascii_whitespace();
        left.push(
            locations.next().unwrap_or("0").parse::<i32>().unwrap() // we default to 0 if the line is empty because the result won't change
        );
        right.push(
            locations.next().unwrap_or("0").parse::<i32>().unwrap() // we default to 0 if the line is empty because the result won't change
        );
    }
    left.sort();  // we want to compare the locations at the same indexes in the sorted vectors
    right.sort();
    
    // take an iterator over the two vectors, map each value and make the difference between each two elements at the same indexes. 
    let result: i32 = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();

    result.to_string()
}

fn process_input_p2(input: &str) -> String {
    let mut left = vec![];
    let mut right = vec![];
    
    for line in input.lines() {
        let mut locations = line.split_ascii_whitespace();
        left.push(
            locations.next().unwrap_or("0").parse::<i32>().unwrap() // we default to 0 if the line is empty because the result won't change
        );
        right.push(
            locations.next().unwrap_or("0").parse::<i32>().unwrap() // we default to 0 if the line is empty because the result won't change
        );
    }

    // iterate over items in the first list of for each item check all items in the second list
    // if they appear we increase the count of how many times they appear. Finally for each number
    // in the first list we increase the result with item * appearances
    let mut result = 0;
    for l in left.iter() {
        let mut count: u8 = 0;
        for r in right.iter() {
            if l == r {
                count += 1;
            }
        }
        result += l * count as i32;
    }

    result.to_string()
}


fn main() {
    // read the input for the puzzle from an input file
    let puzzle_input = "input_day_1";
    let input = 
        match read_file(&puzzle_input) {
            Ok(result) => result,
            Err(e) => panic!("{}", e),
        };

    // process the input and output the result to the console
    let result = process_input_p1(&input);
    println!("Advent of Code | Day 1 Part 1: {}", result);
    let result = process_input_p2(&input);
    println!("Advent of Code | Day 1 Pary 2: {}", result);

}


// test on the given small dataset in the puzzle
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_1_p1() {
        let input = "3   4
4   3

2   5

1   3
3   9
3   3

";
        assert_eq!("11", process_input_p1(input));
    }
    

    #[test]
    fn test_day_1_p2() {
        let input = "3   4
4   3

2   5

1   3
3   9
3   3

";
        assert_eq!("31", process_input_p2(input));
    }
}