use ndarray::{s, Array2, Axis};

fn p1() {
    let mut lines = include_str!("../inputs/8.txt").lines().peekable();
    let mut grid: Vec<Vec<u32>> = Vec::new();
    while let Some(line) = lines.next() {
        let mut row: Vec<u32> = Vec::new();
        // row.push(0);
        for c in line.chars() {
            let h = c.to_digit(10).unwrap();
            row.push(h);
        }
        // row.push(0);
        grid.push(row);
    }

    let mut vis: u32 = ((2 * grid.len()) + (2 * grid[0].len()) - 4)
        .try_into()
        .unwrap();

    let grid: Array2<u32> = Array2::from_shape_vec(
        (grid.len(), grid[0].len()),
        grid.into_iter().flatten().collect(),
    )
    .unwrap();

    for row in 1..(grid.len_of(Axis(0)) - 1) {
        for col in 1..(grid.len_of(Axis(1)) - 1) {
            let height: u32 = grid[[row, col]];
            let mut is_vis: bool = false;

            // check to left
            if !is_vis {
                // let left = grid.slice_axis(Axis(0), Slice::from(..row));
                let left = grid.slice(s![row, ..col]);
                // println!("{}, {}, {}", left, row, col);
                if height > *left.iter().max().unwrap() {
                    is_vis = true;
                }
            }

            // check to right
            if !is_vis {
                let right = grid.slice(s![row, (col + 1)..]);
                if height > *right.iter().max().unwrap() {
                    is_vis = true;
                }
            }

            // check to up
            if !is_vis {
                let up = grid.slice(s![..row, col]);
                if height > *up.iter().max().unwrap() {
                    is_vis = true;
                }
            }

            // check to down
            if !is_vis {
                let down = grid.slice(s![(row + 1).., col]);
                if height > *down.iter().max().unwrap() {
                    is_vis = true;
                }
            }

            if is_vis {
                vis += 1;
            }
        }
    }
    println!("{} trees visible", vis);
}

fn p2() {
    let mut lines = include_str!("../inputs/8.txt").lines().peekable();
    let mut grid: Vec<Vec<u32>> = Vec::new();
    while let Some(line) = lines.next() {
        let mut row: Vec<u32> = Vec::new();
        // row.push(0);
        for c in line.chars() {
            let h = c.to_digit(10).unwrap();
            row.push(h);
        }
        // row.push(0);
        grid.push(row);
    }

    let mut max_score = 0;

    let grid: Array2<u32> = Array2::from_shape_vec(
        (grid.len(), grid[0].len()),
        grid.into_iter().flatten().collect(),
    )
    .unwrap();

    for row in 1..(grid.len_of(Axis(0)) - 1) {
        for col in 1..(grid.len_of(Axis(1)) - 1) {
            let height: u32 = grid[[row, col]];
            let mut score: [u32; 4] = [0, 0, 0, 0];

            // check to left
            let left = grid.slice(s![row,..col;-1]);
            for tree in left.iter() {
                score[0] += 1;
                if tree >= &height {
                    break;
                }
            }

            // check to right
            let right = grid.slice(s![row, (col + 1)..]);
            for tree in right.iter() {
                score[1] += 1;
                if tree >= &height {
                    break;
                }
            }

            // check to up
            let up = grid.slice(s![..row;-1,col]);
            for tree in up.iter() {
                score[2] += 1;
                if tree >= &height {
                    break;
                }
            }

            // check to down
            let down = grid.slice(s![(row + 1).., col]);
            for tree in down.iter() {
                score[3] += 1;
                if tree >= &height {
                    break;
                }
            }

            let total = score[0] * score[1] * score[2] * score[3];
            if max_score < total {
                max_score = total;
            }
        }
    }
    println!("{} most scenic", max_score);
}

pub fn d8(part: i32) {
    match part {
        1 => p1(),
        2 => p2(),
        _ => println!("Invalid part"),
    }
    // }
}
