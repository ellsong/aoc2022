use std::collections::HashMap;

#[derive(Debug, Clone)]
enum FsNode {
    File(usize),
    Directory(HashMap<String, FsNode>),
}

fn parse_input(input: &str) -> FsNode {
    let mut stack: Vec<Box<FsNode>> = Vec::new();
    let mut root = Box::new(FsNode::Directory(HashMap::new()));
    let mut current: Option<Box<FsNode>> = Some(root);

    for line in input.lines() {
        if line.starts_with("$ cd ") {
            let dir_name = &line[5..];
            match dir_name {
                ".." => {
                    current = stack.pop();
                }
                "/" => {
                    current = Some(root.clone());
                }
                _ => {
                    if let Some(FsNode::Directory(contents)) = current.as_mut().map(|c| c.as_mut()).unwrap() {
                        let dir = contents.entry(dir_name.to_string()).or_insert(FsNode::Directory(HashMap::new()));
                        stack.push(current.take().unwrap());
                        current = Some(Box::new(dir.clone()));
                    }
                }
            }
        } else if line.starts_with("dir ") {
            let dir_name = &line[4..];
            if let Some(FsNode::Directory(contents)) = current.as_mut().map(|c| c.as_mut()).unwrap() {
                contents.entry(dir_name.to_string()).or_insert(FsNode::Directory(HashMap::new()));
            }
        } else {
            let mut parts = line.split_whitespace();
            let size = parts.next().unwrap().parse::<usize>().unwrap();
            let file_name = parts.next().unwrap();
            if let Some(FsNode::Directory(contents)) = current.as_mut().map(|c| c.as_mut()).unwrap() {
                contents.insert(file_name.to_string(), FsNode::File(size));
            }
        }
    }

    *current.unwrap()
}




fn directory_total_size(node: &FsNode) -> usize {
    match node {
        FsNode::File(size) => *size,
        FsNode::Directory(contents) => contents.values().map(|n| directory_total_size(n)).sum(),
    }
}

fn sum_directories_with_max_size(node: &FsNode, max_size: usize) -> usize {
    match node {
        FsNode::File(_) => 0,
        FsNode::Directory(contents) => {
            let mut sum = 0;
            for child in contents.values() {
                let child_size = directory_total_size(child);
                if child_size <= max_size {
                    sum += child_size;
                }
                sum += sum_directories_with_max_size(child, max_size);
            }
            sum
        }
    }
}

pub fn d7(part: i32) {
    // read file
    // Read the contents of the input file
    let input = match fs::read_to_string("inputs/7.txt") {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            process::exit(1);
        }
    };
    // if let Ok(lines) = read_lines("inputs/7.txt") {
    match part {
        1 => p1(&input),
        // 2 => p2(lines),
        _ => println!("Invalid part"),
    }
    // }
}

use std::fs;
use std::process;

fn p1(input: &String) {
    let filesystem = parse_input(&input);
    let sum = sum_directories_with_max_size(&filesystem, 100000);
    println!(
        "Sum of total sizes of directories with total size at most 100000: {}",
        sum
    );
}
