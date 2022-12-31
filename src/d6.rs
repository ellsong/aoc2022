use crate::utils::read_lines;

fn p1() {
    // read file
    if let Ok(lines) = read_lines("inputs/6.txt") {
        // for each line in file
        let mut marker: Vec<char> = Vec::new();
        let mut pos: i32 = 1;
        for line in lines {
            if let Ok(line) = line {
                for c in line.chars() {
                    marker.push(c);
                    if marker.len() == 5{
                        marker.remove(0);
                        let mut ms = marker.clone();
                        ms.sort();
                        ms.dedup();
                        if ms.len() == 4 {
                            println!("Position is {}", pos);
                            break;
                        }
                    }

                    pos += 1;
                }
            }
        }
    }
}

fn p2() {
    // read file
    if let Ok(lines) = read_lines("inputs/6.txt") {
        // for each line in file
        let mut marker: Vec<char> = Vec::new();
        let mut pos: i32 = 1;
        for line in lines {
            if let Ok(line) = line {
                for c in line.chars() {
                    marker.push(c);
                    if marker.len() == 15{
                        marker.remove(0);
                        let mut ms = marker.clone();
                        ms.sort();
                        ms.dedup();
                        if ms.len() == 14 {
                            println!("Position is {}", pos);
                            break;
                        }
                    }

                    pos += 1;
                }
            }
        }
    }
}

pub fn d6(part: i32) {
    match part {
        1 => p1(),
        2 => p2(),
        _ => println!("Invalid part"),
    }
}
