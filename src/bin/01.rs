use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::absolute;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut list1 = Vec::new();
        let mut list2 = Vec::new();

        for res in reader.lines() {
            let line = res?;
            let parts: Vec<&str> = line.split("   ").collect();
            if let Some(first_part) = parts.get(0) {
                list1.push(first_part.to_string());
            }
            if let Some(second_part) = parts.get(1) {
                list2.push(second_part.to_string());
            }
        }
        list1.sort();
        list2.sort();
        let mut temp = 0;
        for (a, b) in list1.iter().zip(list2.iter()) {
            let a_value: isize = a.parse().unwrap_or(0);
            let b_value: isize = b.parse().unwrap_or(0);
            temp += a_value.abs_diff(b_value);
        };
        let answer = temp;
        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut list1 = Vec::new();
        let mut list2 = Vec::new();

        for res in reader.lines() {
            let line = res?;
            let parts: Vec<&str> = line.split("   ").collect();
            if let Some(first_part) = parts.get(0) {
                list1.push(first_part.to_string());
            }
            if let Some(second_part) = parts.get(1) {
                list2.push(second_part.to_string());
            }
        }
        let mut temp = 0;
        for a in list1.iter() {
            temp += a.parse::<usize>().unwrap() * list2.iter().filter(|&b| b == a).count();
        }
        let answer = temp;
        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
