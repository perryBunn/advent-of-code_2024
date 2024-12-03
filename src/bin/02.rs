use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe = 0;
        let mut reactors = Vec::new();
        for res in reader.lines() {
            let line = res?;
            let parts: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
            reactors.push(parts);
        };

        // 0 initial state not ascending or descending
        // 1 ascending
        // 2 descending
        for level in reactors {
            let mut state = 0;
            let mut level_safe = true;
            for report_group in level.iter().tuple_windows::<(_,_)>() {
                // upper range valid
                let report_a: usize = report_group.0.parse()?;
                let report_b: usize = report_group.1.parse()?;
                if report_a.abs_diff(report_b) > 3 {
                    level_safe = false;
                    break
                }

                if state == 0 {
                    if report_a < report_b {
                        state = 1;
                    } else if report_a > report_b {
                        state = 2
                    } else if report_a == report_b { // initial pair is equal
                        level_safe = false;
                        break
                    }
                } else if state == 1 && report_a > report_b { // pair in ascending sort descends
                    level_safe = false;
                    break
                } else if state == 2 && report_a < report_b { // pair in descending sort ascends
                    level_safe = false;
                    break
                } else if report_a == report_b { // i+n pair is equal
                    level_safe = false;
                    break
                }
            }
            if level_safe {
                safe += 1;
            }
        }

        let answer = safe;
        Ok(answer)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe = 0;
        let mut reactors = Vec::new();
        for res in reader.lines() {
            let line = res?;
            let parts: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
            reactors.push(parts);
        };

        // 0 initial state not ascending or descending
        // 1 ascending
        // 2 descending
        for level in reactors {
            let mut state = 0;
            let mut fault_tolerance = 0;
            let mut level_safe = true;
            for report_group in level.iter().tuple_windows::<(_,_)>() {
                // upper range valid
                let report_a: usize = report_group.0.parse()?;
                let report_b: usize = report_group.1.parse()?;
                if report_a.abs_diff(report_b) > 3 {
                    level_safe = false;
                    break
                }

                if state == 0 {
                    if report_a < report_b {
                        state = 1;
                    } else if report_a > report_b {
                        state = 2
                    } else if report_a == report_b { // initial pair is equal
                        fault_tolerance += 1
                    }
                } else if state == 1 && report_a > report_b { // pair in ascending sort descends
                    fault_tolerance += 1
                } else if state == 2 && report_a < report_b { // pair in descending sort ascends
                    fault_tolerance += 1
                } else if report_a == report_b { // i+n pair is equal
                    fault_tolerance += 1
                }
            }
            if level_safe && fault_tolerance < 2 {
                safe += 1;
            }
        }

        let answer = safe;
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
