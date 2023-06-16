struct Rope {
    knots: [[i32; 2]; 10],
    tail_loc: Vec<[i32; 2]>,
}

impl Rope {
    fn new() -> Self {
        Rope {
            knots: [[0, 0]; 10],
            tail_loc: vec![[0, 0]],
        }
    }

    fn motion(&mut self, mv: [i32; 2]) {
        self.knots[0] = [self.knots[0][0] + mv[0], self.knots[0][1] + mv[1]];

        for i in 1..10 {
            let mut diff = [
                self.knots[i - 1][0] - self.knots[i][0],
                self.knots[i - 1][1] - self.knots[i][1],
            ];

            while diff[0].abs().max(diff[1].abs()) > 1 {
                let mut tm: [i32; 2] = diff.clone();
                for v in 0..tm.len() {
                    if tm[v].abs() > 1 {
                        tm[v] = tm[v] / tm[v].abs();
                    }
                }
                self.knots[i] = [self.knots[i][0] + tm[0], self.knots[i][1] + tm[1]];
                diff = [
                    self.knots[i - 1][0] - self.knots[i][0],
                    self.knots[i - 1][1] - self.knots[i][1],
                ];
                if i == 9 {
                    if !self.tail_loc.contains(&self.knots[9]) {
                        self.tail_loc.push(self.knots[9].clone());
                    }
                }
            }
        }
    }
}

pub fn p2() {
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

    println!("Tail has {} locations", rope.tail_loc.len());
}
