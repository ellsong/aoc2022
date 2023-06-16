use crate::d9b;

struct Rope {
    head: [i32; 2],
    tail: [i32; 2],
    knots: Vec<[i32; 2]>,
}

impl Rope {
    fn new() -> Self {
        let mut knots = Vec::new();
        knots.push([0, 0]);
        Rope {
            head: [0, 0],
            tail: [0, 0],
            knots,
        }
    }

    fn motion(&mut self, mv: [i32; 2]) {
        self.head = [self.head[0] + mv[0], self.head[1] + mv[1]];
        let diff = [self.head[0] - self.tail[0], self.head[1] - self.tail[1]];

        let mut tm: [i32; 2] = [0, 0];

        if diff[0].abs() == 2 {
            tm[0] = diff[0] / 2;
            tm[1] = diff[1];
        }

        if diff[1].abs() == 2 {
            tm[1] = diff[1] / 2;
            tm[0] = diff[0];
        }

        self.tail = [self.tail[0] + tm[0], self.tail[1] + tm[1]];

        if !self.knots.contains(&self.tail) {
            self.knots.push(self.tail.clone());
        }
    }
}

fn p1() {
    let mut lines = include_str!("../inputs/9.txt").lines().peekable();
    let mut moves: Vec<[i32; 2]> = Vec::new();
    while let Some(line) = lines.next() {
        let dir: [i32; 2];
        let mut ln = line.split_whitespace();
        match ln.next().unwrap() {
            "R" => dir = [1, 0],
            "L" => dir = [-1, 0],
            "U" => dir = [0, 1],
            "D" => dir = [0, -1],
            _ => dir = [0, 0],
        }
        for _i in 0..(ln.next().unwrap().parse().unwrap()) {
            moves.push(dir.clone());
        }
    }

    let mut rope = Rope::new();
    for mv in moves {
        rope.motion(mv);
    }

    println!("Tail has {} locations", rope.knots.len());
}

pub fn d9(part: i32) {
    match part {
        1 => p1(),
        2 => d9b::p2(),
        _ => println!("Invalid part"),
    }
}
