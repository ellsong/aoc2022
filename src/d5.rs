use crate::utils::read_lines;

fn p1() {
    let mut stacks: Vec<Vec<char>> = Vec::from([
        Vec::from(['Q','W','P','S','Z','R','H','D']),
        Vec::from(['V','B','R','W','Q','H','F']),
        Vec::from(['C','V','S','H']),
        Vec::from(['H','F','G']),
        Vec::from(['P','G','J','B','Z']),
        Vec::from(['Q','T','J','H','W','F','L']),
        Vec::from(['Z','T','W','D','L','V','J','N']),
        Vec::from(['D','T','Z','C','J','G','H','F']),
        Vec::from(['W','P','V','M','B','H'])
    ]);
    // read file
    if let Ok(lines) = read_lines("inputs/5.txt") {
        // for each line in file
        for line in lines {
            if let Ok(line) = line {
                let l: Vec<&str> = line.split_whitespace().collect();
                let moves: i32 = l[1].parse().unwrap();
                let from_stack: usize = l[3].parse::<usize>().unwrap() - 1;
                let to_stack: usize = l[5].parse::<usize>().unwrap()-1;
                for _i in 0..moves {
                    let item: char = stacks[from_stack].pop().unwrap();
                    stacks[to_stack].push(item);
                }
            }
        }
        // return the total score
        let mut result: String = "".to_string();
        for stack in stacks {
            result.push(stack.last().unwrap().clone());
        }
        println!("Result is {}", result);
    }
}

fn p2() {
    let mut stacks: Vec<Vec<char>> = Vec::from([
        Vec::from(['Q','W','P','S','Z','R','H','D']),
        Vec::from(['V','B','R','W','Q','H','F']),
        Vec::from(['C','V','S','H']),
        Vec::from(['H','F','G']),
        Vec::from(['P','G','J','B','Z']),
        Vec::from(['Q','T','J','H','W','F','L']),
        Vec::from(['Z','T','W','D','L','V','J','N']),
        Vec::from(['D','T','Z','C','J','G','H','F']),
        Vec::from(['W','P','V','M','B','H'])
    ]);
    // read file
    if let Ok(lines) = read_lines("inputs/5.txt") {
        // for each line in file
        for line in lines {
            if let Ok(line) = line {
                let l: Vec<&str> = line.split_whitespace().collect();
                let moves: usize = l[1].parse::<usize>().unwrap();
                let from_stack: usize = l[3].parse::<usize>().unwrap() - 1;
                let to_stack: usize = l[5].parse::<usize>().unwrap()-1;
                
                let mut crates: Vec<char> = stacks[from_stack][stacks[from_stack].len()-moves..stacks[from_stack].len()].to_vec();
                stacks[to_stack].append(&mut crates);

                stacks[from_stack] = stacks[from_stack][0..stacks[from_stack].len()-moves].to_vec();
            }
        }
        // return the total score
        let mut result: String = "".to_string();
        for stack in stacks {
            result.push(stack.last().unwrap().clone());
        }
        println!("Result is {}", result);
    }
}

pub fn d5(part: i32) {
    match part {
        1 => p1(),
        2 => p2(),
        _ => println!("Invalid part"),
    }
}
