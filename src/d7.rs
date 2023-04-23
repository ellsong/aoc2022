use std::iter::Peekable;

fn sum_node(lines: &mut Peekable<impl Iterator<Item = &'static str>>, sum: &mut i32) -> i32 {
    let mut size = 0;
    while let Some(line) = lines.next() {
            match line {
                "$ cd .." => break,
                _ if line.starts_with("$ ls") => {
                    size = std::iter::from_fn(|| lines.next_if(|line| !line.starts_with('$')))
                    .filter(|line| !line.starts_with('d'))
                    .map(|line| line.split(' ').next().unwrap().parse::<i32>().unwrap())
                    .sum()
                }
                _ => size += sum_node(lines, sum)
            }
    }
    if size <= 100000 {
        *sum += size;
    }
    return size;
}

fn p1(mut lines: Peekable<impl Iterator<Item = &'static str>>) {
    let mut sum = 0;
    sum_node(&mut lines, &mut sum);
    println!("{}", sum)
}

pub fn d7(part: i32) {
    // read file    
    // if let Ok(lines) = read_lines("inputs/7.txt") {
    //     let mut lines = lines.into_iter().peekable();
        let lines = include_str!("../inputs/7.txt").lines().peekable();
        match part {
            1 => p1(lines),
            // 2 => p2(lines),
            _ => println!("Invalid part"),
        }
    // }
}