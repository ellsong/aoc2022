use crate::utils::read_lines;

fn p1() {
    if let Ok(lines) = read_lines("inputs/2.txt") {
        let mut total: i32 = 0;
        for line in lines {
            if let Ok(line) = line {
                let chars: Vec<char> = line.chars().collect();
                match chars[2] {
                    'X' => {
                        total += 1;
                        match chars[0] {
                            'A' => total += 3,
                            'B' => total += 0,
                            'C' => total += 6,
                            _ => {}
                        }
                    },
                    'Y' => {
                        total += 2;
                        match chars[0] {
                            'A' => total += 6,
                            'B' => total += 3,
                            'C' => total += 0,
                            _ => {}
                        }
                    },
                    'Z' => {
                        total += 3;
                        match chars[0] {
                            'A' => total += 0,
                            'B' => total += 6,
                            'C' => total += 3,
                            _ => {}
                        }
                    },
                    _ => {},
                }
            }
        }
        println!("Total score is {}", total);
    } else {
        println!("No input file");
    }
}

fn p2() {
    if let Ok(lines) = read_lines("inputs/2.txt") {
        let mut total: i32 = 0;
        for line in lines {
            if let Ok(line) = line {
                let chars: Vec<char> = line.chars().collect();
                match chars[2] {
                    'X' => {
                        total += 0;
                        match chars[0] {
                            'A' => total += 3,
                            'B' => total += 1,
                            'C' => total += 2,
                            _ => {}
                        }
                    },
                    'Y' => {
                        total += 3;
                        match chars[0] {
                            'A' => total += 1,
                            'B' => total += 2,
                            'C' => total += 3,
                            _ => {}
                        }
                    },
                    'Z' => {
                        total += 6;
                        match chars[0] {
                            'A' => total += 2,
                            'B' => total += 3,
                            'C' => total += 1,
                            _ => {}
                        }
                    },
                    _ => {},
                }
            }
        }
        println!("Total score is {}", total);
    } else {
        println!("No input file");
    }
}

pub fn d2(part: i32) {
        match part {
            1 => p1(),
            2 => p2(),
            _ => println!("Invalid part"),
        }
}
