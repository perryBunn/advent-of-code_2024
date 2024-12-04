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

    fn valid_span(a: usize, b: usize) -> Result<bool> {
        Ok(a.abs_diff(b) >= 1 && a.abs_diff(b) <= 3)
    }

    fn valid_orientation(a: usize, b: usize, direction: i8) -> Result<bool> {
        Ok(match direction {
            1 => a <= b,
            2 => a >= b,
            _ => true,  // No initial state yet
        })
    }

    fn valid_level(level: Vec<String>) -> Result<bool> {
        let instance = level.clone();
        let mut ascending = 0;
        let mut res = true;
        for i in 0..instance.len() - 1 {
            let a = instance.get(i).unwrap().parse()?;
            let b = instance.get(i+1).unwrap().parse()?;
            if !valid_span(a, b)? || !valid_orientation(a, b, ascending)? {
                res = false;
                break
            }
            if ascending == 0 {
                match ascending {
                    0 if a < b => ascending = 1,
                    0 if a > b => ascending = 2,
                    _ => {}
                }
            }
        }
        Ok(res)
    }

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

        for level in reactors {
            if valid_level(level)? {
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

    fn write_reactors_to_file(reactors: Vec<Vec<String>>, output_file: &str) -> Result<()> {
        use std::fs::OpenOptions;
        use std::io::Write;

        let mut file = OpenOptions::new().write(true).create(true).open(output_file)?;

        for level in reactors {
            let line = level.join(" ");
            writeln!(file, "{}", line)?;
        }

        Ok(())
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe = 0;
        let mut reactors = Vec::new();
        for res in reader.lines() {
            let line = res?;
            let parts: Vec<String> = line.split(" ").map(|s| s.to_string()).collect();
            reactors.push(parts);
        };

        for level in reactors {
            for i in 0..level.len() {
                let mut l = level.clone();
                l.remove(i);
                if valid_level(l)? {
                    safe += 1;
                    break
                }
            }
        }

        let answer = safe;
        // write_reactors_to_file(failed_reactors, "debug/failed_reactors.txt")?;
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
