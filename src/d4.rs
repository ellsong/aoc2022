use crate::utils::read_lines;

fn p1() {
    // read file
    if let Ok(lines) = read_lines("inputs/4.txt") {
        let mut score: u32 = 0;

        // for each line in file
        for line in lines {
            let mut sections: [u32; 4] = [0; 4];
            let mut section: String = String::new();
            let mut index: usize = 0;
            if let Ok(line) = line {
                for i in line.chars() {
                    if i == '-' || i == ',' {
                        sections[index] = section.parse().unwrap();
                        section = String::new();
                        index += 1;
                    } else {
                        section.push(i);
                    }
                }
                sections[index] = section.parse().unwrap();
                if (sections[0] <= sections[2] && sections[1] >= sections[3])
                    || (sections[0] >= sections[2] && sections[1] <= sections[3])
                {
                    score += 1;
                }
            }
        }
        // return the total score
        println!("Score is {}", score);
    }
}

fn p2() {
    // read file
    if let Ok(lines) = read_lines("inputs/4.txt") {
        let mut score: u32 = 0;

        // for each line in file
        for line in lines {
            let mut sections: [u32; 4] = [0; 4];
            let mut section: String = String::new();
            let mut index: usize = 0;
            if let Ok(line) = line {
                for i in line.chars() {
                    if i == '-' || i == ',' {
                        sections[index] = section.parse().unwrap();
                        section = String::new();
                        index += 1;
                    } else {
                        section.push(i);
                    }
                }
                sections[index] = section.parse().unwrap();
                println!(
                    "{}-{},{}-{}",
                    sections[0], sections[1], sections[2], sections[3]
                );
                if (sections[2] >= sections[0] && sections[2] <= sections[1])
                    || (sections[3] >= sections[0] && sections[3] <= sections[1])
                    || (sections[0] >= sections[2] && sections[0] <= sections[3])
                    || (sections[1] >= sections[2] && sections[1] <= sections[3])
                {
                    score += 1;
                }
            }
        }
        // return the total score
        println!("Score is {}", score);
    }
}

pub fn d4(part: i32) {
    match part {
        1 => p1(),
        2 => p2(),
        _ => println!("Invalid part"),
    }
}
