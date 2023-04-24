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
            _ => size += sum_node(lines, sum),
        }
    }
    if size <= 100000 {
        *sum += size;
    }
    return size;
}

fn p1() {
    let mut lines = include_str!("../inputs/7.txt").lines().peekable();
    let mut sum = 0;
    sum_node(&mut lines, &mut sum);
    println!("{}", sum)
}

const FS_TOTAL: u64 = 70000000;
const UP: u64 = 30000000;

#[derive(Default)]
struct Dir {
    dirs: Vec<Dir>,
    size: u64,
}

fn build_dirs(lines: &mut Peekable<impl Iterator<Item = &'static str>>) -> Dir {
    let mut dirs: Dir = Dir::default();

    while let Some(line) = lines.next() {
        match line {
            "$ cd .." => break,
            _ if line.starts_with("$ ls") => {
                dirs.size = std::iter::from_fn(|| lines.next_if(|line| !line.starts_with('$')))
                    .filter(|line| !line.starts_with('d'))
                    .map(|line| line.split(' ').next().unwrap().parse::<u64>().unwrap())
                    .sum()
            }
            _ => dirs.dirs.push(build_dirs(lines))
        }
    }
    dirs.size += dirs.dirs.iter().map(|d| d.size).sum::<u64>();
    return dirs;
}

fn search(dirs: &Dir, min: u64) -> Option<u64> {
    dirs.dirs.iter()
        .filter(|d| d.size >= min)
        .flat_map(|d| [Some(d.size), search(d, min)])
        .flatten()
        .min()
}

fn p2() {
    let mut lines = include_str!("../inputs/7.txt").lines().peekable();
    let dirs = build_dirs(&mut lines);
    let min = UP - (FS_TOTAL-dirs.size);
    let min_dir = search(&dirs, min).unwrap();
    println!("{}", min_dir);
}

pub fn d7(part: i32) {
    match part {
        1 => p1(),
        2 => p2(),
        _ => println!("Invalid part"),
    }
    // }
}
