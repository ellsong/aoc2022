use crate::utils::read_lines;
use std::fs::File;
use std::io::BufReader;
use std::str::SplitWhitespace;
use std::{collections::HashMap, io::Lines};

struct Obj {
    name: String,
    size: i32,
}
enum DiskObject {
    Dir(HashMap<String, DiskObject>),
    Obj(Obj),
}

enum Command {
    Cd(String),
    Ls,
}

enum Entry {
    Cd(String),
    Ls,
    Dir(String),
    Fl(u32)
}

fn parse_cd(mut words: SplitWhitespace, cur: &mut Vec<String>) {
    let loc = words.next().unwrap();
    match loc {
        "/" => {
            *cur = Vec::new();
        }
        ".." => {
            cur.pop();
        }
        _ => {
            cur.push(loc.to_owned());
        }
    }
    println!("{:?}", cur);
}

fn parse_ls(mut lines: Lines<BufReader<File>>) {
    loop {
        let mut sum: i32 = 0;
        if let Some(Ok(line)) = lines.next() {
            let mut words = line.split_whitespace();
            let first_word = words.next().unwrap();
            match first_word {
                "$" => return ,
                "dir" => {},
                _ => {sum += first_word.parse::<i32>().unwrap();}
            }
        }
    }
}

fn parse_command(mut words: SplitWhitespace, cur: &mut Vec<String>) {
    let cmd = words.next().unwrap();
    match cmd {
        "cd" => parse_cd(words, cur),
        "ls" => parse_ls(),
        _ => {}
    }
}

fn parse_line(line: String, cur: &mut Vec<String>) {
    let mut words = line.split_whitespace();
    let first_word = words.next().unwrap();
    match first_word {
        "$" => parse_command(words, cur),
        "dir" => {}
        _ => {}
    }
}

// fn build_disk(lines: Lines<BufReader<File>>) {
    
// }

fn p1(lines: Lines<BufReader<File>>) {
    let mut disk: HashMap<String, DiskObject> = HashMap::new();
    let mut sum: i32 = 0;
    let mut cur: Vec<String> = Vec::new();
    let mut lines: Lines<BufReader<File>> = lines.into_iter();
    if let Some(Ok(line)) = lines.next() {
        parse_line(line, &mut cur);
    }
}

fn p2(lines: Lines<BufReader<File>>) {}

pub fn d7(part: i32) {
    // read file
    if let Ok(lines) = read_lines("inputs/7.txt") {
        // build_disk(lines);
        match part {
            1 => p1(lines),
            2 => p2(lines),
            _ => println!("Invalid part"),
        }
    }
}
