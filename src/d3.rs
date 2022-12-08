use crate::utils::read_lines;

fn p1() {
    // read file
    if let Ok(lines) = read_lines("inputs/3.txt") {
        let mut score: u32 = 0;
        // for each line in file
        for line in lines {
            if let Ok(line) = line {
                let (front, back) = line.split_at(line.len()/2);
                for i in front.chars() {
                    if back.contains(i) {
                        if i.is_uppercase() {
                            score += (i as u32) - ('A' as u32) + 27
                        } else {
                            score += (i as u32) - ('a' as u32) + 1
                        }
                        break;
                    }
                }
            }
        }
        // return the total score
        println!("Score is {}", score);
    }
    
}

fn p2 () {
    if let Ok(mut lines) = read_lines("inputs/3.txt") {
        let mut score: u32 = 0;
        // for each line in file
        while let (Some(line1), Some(line2), Some(line3)) = (lines.next(), lines.next(), lines.next()) {
            if let (Ok(line1), Ok(line2), Ok(line3)) = (line1, line2, line3) {
                for i in line1.chars() {
                    if line2.contains(i) && line3.contains(i) {
                        if i.is_uppercase() {
                            score += (i as u32) - ('A' as u32) + 27
                        } else {
                            score += (i as u32) - ('a' as u32) + 1
                        }
                        break;
                    }
                }
            }
        }
            
        // return the total score
        println!("Score is {}", score);
    }
}

pub fn d3(part: i32) {
    match part {
        1 => p1(),
        2 => p2(),
        _ => println!("Invalid part"),
    }
}