use read_input::read_file;
use std::collections::HashMap;

fn get_ordering_rule(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut ordering_rule: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let line: Vec<&str> = line.split("|").collect();
        let x = line[0];
        let y = line[1];
        let mut current_y;
        match ordering_rule.get(&x) {
            Some(v) => current_y = v.clone(),
            None => current_y = vec![],
        }
        current_y.push(y);
        ordering_rule.insert(x, current_y);
    }
    
    ordering_rule
}

fn is_correct(line: &str, ordering_rule: &HashMap<&str, Vec<&str>>) -> bool {
    // we split the string in a vector with all numbers and reverse it so we can check if it follows the rules
    let mut nums: Vec<&str> = line.split(",").collect();
    nums.reverse();
    for num in nums.clone() {
        // we remove the first element of the remaining vector and check if the following numbers follow the rules
        nums.remove(0);
        let rule_vec = 
            match ordering_rule.get(num) {
                Some(v) => v,
                None => continue,
            };
        // comparing each following number in the list with the rule associated with the current number (which we removed)
        for n in &nums {
            if rule_vec.contains(n) {
                return false
            }
        }
    }
    println!("Corrects");
    dbg!(line);
    true
}

fn make_it_right(line: &str, ordering_rule: &HashMap<&str, Vec<&str>>) -> String {
    let mut nums: Vec<&str> = line.split(",").collect();
    nums.reverse();
    
    while is_correct(line, ordering_rule) {
        for i in 0..nums.len() {
            let rule_vec = 
                match ordering_rule.get(nums[i]) {
                    Some(v) => v,
                    None => continue,
                };
            for (j, n) in nums.clone().iter().enumerate() {
                for r in rule_vec.iter() {
                    if n == r {
                        nums.swap(i, j);
                        break;
                    }
                }
            }
        }
    }
    
    nums.reverse();
    let line = nums.join(",");
    line
}

fn process_input_p1_2(input: &str) -> i32 {
    // split the input into the ordering list and the outputs
    // given that we know the contents of the file won't change I will not be error handling
    let input: Vec<&str> = input.split("\n\n").collect();
    let ordering_list = input[0];
    let outputs = input[1];
    let ordering_rule = get_ordering_rule(ordering_list);
    let mut res = 0;
    for line in outputs.lines() {
        // if the line is correct get the middle element, parse it to integer and add it to the final result
        if is_correct(line, &ordering_rule) {
            let line_vec: Vec<&str> = line.split(",").collect();
            let mid = line_vec[line_vec.len() / 2];
            let mid = mid.parse::<i32>().unwrap();
            res += mid;
        }
        else {
            let res_line = make_it_right(line, &ordering_rule);
            println!("reformated");
            dbg!(res_line);
        }
    }

    res
} 

fn main() {
    let puzzle_path = "input_day_5";
    let input = 
        match read_file(&puzzle_path) {
            Ok(val) => val,
            Err(_) => panic!("Should be able to read file"),
        };
    let res = process_input_p1_2(&input);
    println!("Advent of Code | Day 5 Part 1: {}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_5_p1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        
        assert_eq!(143, process_input_p1_2(input));
    }
}


