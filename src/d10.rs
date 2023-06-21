fn p2() {
    let mut lines = include_str!("../inputs/10.txt").lines().peekable();
    let mut reg: Vec<i32> = vec![1];
    while let Some(line) = lines.next() {
        let mut ln = line.split_whitespace();
        match ln.next().unwrap() {
            "addx" => {
                reg.push(reg.last().unwrap().clone());
                let v: i32 = ln.next().unwrap().parse().unwrap();
                reg.push(reg.last().unwrap() + v);
            }
            "noop" => reg.push(reg.last().unwrap().clone()),
            _ => {}
        }
    }
    let mut ln: String = String::new();
    for i in 0..reg.len() {
        let p = reg[i] % 40;
        let c: i32 = i as i32 % 40;
        if c <= p + 1 && c >= p - 1 {
            ln.push('#');
        } else {
            ln.push('.');
        }
        if c == 39 {
            println!("{}", ln);
            ln = String::new();
        }
    }
}

fn p1() {
    let mut lines = include_str!("../inputs/10.txt").lines().peekable();
    let mut reg: Vec<i32> = vec![1];
    while let Some(line) = lines.next() {
        let mut ln = line.split_whitespace();
        match ln.next().unwrap() {
            "addx" => {
                reg.push(reg.last().unwrap().clone());
                let v: i32 = ln.next().unwrap().parse().unwrap();
                reg.push(reg.last().unwrap() + v);
            }
            "noop" => reg.push(reg.last().unwrap().clone()),
            _ => {}
        }
    }
    let mut s: i32 = 0;
    for i in (20..=220).step_by(40) {
        s += reg[i - 1] * i as i32;
    }
    println!("Signal sum is {}", s);
}

pub fn d10(part: i32) {
    match part {
        1 => p1(),
        2 => p2(),
        _ => println!("Invalid part"),
    }
}
