
use std::cmp::max;
use crate::utils::read_lines;

fn p1() {
    if let Ok(lines) = read_lines("inputs/1.txt") {
        let mut max_cal: u32 = 0;
        let mut cur_cal: u32 = 0;
        for line in lines {
            if let Ok(line) = line {
                if let Ok(line) = line.parse::<u32>() {
                    cur_cal += line;
                    
                } else {
                    max_cal = max(max_cal, cur_cal);
                    cur_cal = 0;
                }
            }
        }
        println!("Max calories is {}", max_cal);
    } else {
        println!("No input file");
    }
}

fn p2() {
    if let Ok(lines) = read_lines("inputs/1.txt") {
        let mut max_cal: [u32; 3] = [0; 3];
        let mut cur_cal: u32 = 0;
        for line in lines {
            if let Ok(line) = line {
                if let Ok(line) = line.parse::<u32>() {
                    cur_cal += line;
                } else {
                    if cur_cal > max_cal[0] {
                        max_cal[0] = cur_cal;
                        max_cal.sort();
                    }
                    cur_cal = 0;
                }
            }
        }
        let sum: u32 = max_cal.iter().sum();
        println!("Max calories is {}", sum);
    } else {
        println!("No input file");
    }
}

pub fn d1(part: i32) {
    match part {
        1 => p1(),
        2 => p2(),
        _ => println!("Invalid part"),
    }
}
