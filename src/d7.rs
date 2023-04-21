use crate::utils::read_lines;
use std::fs::File;
use std::io::BufReader;
use std::{collections::HashMap, io::Lines};

struct Obj {
    name: String,
    size: i32,
}
enum DiskObject {
    Dir(HashMap<String, DiskObject>),
    Obj(Obj),
}

// fn split_line()

fn p1(lines: Lines<BufReader<File>>) {
    let mut disk: HashMap<String, DiskObject> = HashMap::new();
    let mut cur: Vec<&str> = Vec::new();
    let mut lines = lines.into_iter();
    if let Some(Ok(line)) = lines.next() {
        let mut words = line.split_whitespace();
        let first_word = words.next().unwrap();
        match first_word {
            "$" => {
                let second_word = words.next().unwrap();
                match second_word {
                    "cd" => {
                        let third_word = words.next().unwrap();
                        match third_word {
                            "/" => {
                                cur = Vec::new();
                                
                            },
                            ".." => {
                                cur.pop();
                            }
                            _ => {
                                cur.push(third_word);
                            }
                        }
                    },
                    "ls" => {},
                    _ => {}
                }
            },
            "dir" => {}
            _ => {}
        }
    }
}

fn p2(lines: Lines<BufReader<File>>) {
    
}

pub fn d7(part: i32) {
    // read file
    if let Ok(lines) = read_lines("inputs/7.txt") {
        match part {
            1 => p1(lines),
            2 => p2(lines),
            _ => println!("Invalid part"),
        }
    }
}
