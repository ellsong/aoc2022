fn p1() {
    let mut lines = include_str!("../inputs/8.txt").lines().peekable();
    let mut grid: Vec<Vec<u32>> = Vec::new();
    while let Some(line) = lines.next() {
        let mut row:Vec<u32> = Vec::new();
        row.push(0);
        for c in line.chars() {
            let h = c.to_digit(10).unwrap();
            row.push(h);
        }
        row.push(0);
        grid.push(row);
    }
    // let size = grid[0]
    println!("{:?}", grid);
}

pub fn d8(part: i32) {
    match part {
        1 => p1(),
        // 2 => p2(),
        _ => println!("Invalid part"),
    }
    // }
}
