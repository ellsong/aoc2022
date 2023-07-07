use num_bigint::BigUint;

enum Op {
    Mult(u32),
    Plus(u32),
    Square,
}

struct Monkey {
    items: Vec<BigUint>,
    op: Op,
    test: u32,
    pass: usize,
    fail: usize,
}

impl Monkey {
    fn business() -> [Monkey; 8] {
        [
            Monkey {
                items: Vec::from([
                    BigUint::new(vec![65]),
                    BigUint::new(vec![58]),
                    BigUint::new(vec![93]),
                    BigUint::new(vec![57]),
                    BigUint::new(vec![66]),
                ]),
                op: Op::Mult(7),
                test: 19,
                pass: 6,
                fail: 4,
            },
            Monkey {
                items: Vec::from([
                    BigUint::new(vec![76]),
                    BigUint::new(vec![97]),
                    BigUint::new(vec![58]),
                    BigUint::new(vec![72]),
                    BigUint::new(vec![57]),
                    BigUint::new(vec![92]),
                    BigUint::new(vec![82]),
                ]),
                op: Op::Plus(4),
                test: 3,
                pass: 7,
                fail: 5,
            },
            Monkey {
                items: Vec::from([
                    BigUint::new(vec![90]),
                    BigUint::new(vec![89]),
                    BigUint::new(vec![96]),
                ]),
                op: Op::Mult(5),
                test: 13,
                pass: 5,
                fail: 1,
            },
            Monkey {
                items: Vec::from([
                    BigUint::new(vec![72]),
                    BigUint::new(vec![63]),
                    BigUint::new(vec![72]),
                    BigUint::new(vec![99]),
                ]),
                op: Op::Square,
                test: 17,
                pass: 0,
                fail: 4,
            },
            Monkey {
                items: Vec::from([BigUint::new(vec![65])]),
                op: Op::Plus(1),
                test: 2,
                pass: 6,
                fail: 2,
            },
            Monkey {
                items: Vec::from([BigUint::new(vec![97]), BigUint::new(vec![71])]),
                op: Op::Plus(8),
                test: 11,
                pass: 7,
                fail: 3,
            },
            Monkey {
                items: Vec::from([
                    BigUint::new(vec![83]),
                    BigUint::new(vec![68]),
                    BigUint::new(vec![88]),
                    BigUint::new(vec![55]),
                    BigUint::new(vec![87]),
                    BigUint::new(vec![67]),
                ]),
                op: Op::Plus(2),
                test: 5,
                pass: 2,
                fail: 1,
            },
            Monkey {
                items: Vec::from([
                    BigUint::new(vec![64]),
                    BigUint::new(vec![81]),
                    BigUint::new(vec![50]),
                    BigUint::new(vec![96]),
                    BigUint::new(vec![82]),
                    BigUint::new(vec![53]),
                    BigUint::new(vec![62]),
                    BigUint::new(vec![92]),
                ]),
                op: Op::Plus(5),
                test: 7,
                pass: 3,
                fail: 0,
            },
        ]
    }
}

struct Party {
    monkeys: [Monkey; 8],
    inspections: [BigUint; 8],
}

impl Party {
    fn start() -> Self {
        Party {
            monkeys: Monkey::business(),
            inspections: [
                BigUint::new(vec![0]),
                BigUint::new(vec![0]),
                BigUint::new(vec![0]),
                BigUint::new(vec![0]),
                BigUint::new(vec![0]),
                BigUint::new(vec![0]),
                BigUint::new(vec![0]),
                BigUint::new(vec![0]),
            ],
        }
    }
}

pub fn p2() {
    let mut party = Party::start();
    for rounds in 0..10000 {
        for i in 0..party.monkeys.len() {
            while !party.monkeys[i].items.is_empty() {
                let mut worry = party.monkeys[i].items.remove(0);
                match party.monkeys[i].op {
                    Op::Mult(v) => worry *= v,
                    Op::Plus(v) => worry += v,
                    Op::Square => worry *= worry.clone(),
                }
                // worry /= 3;
                if worry.clone() % party.monkeys[i].test == BigUint::new(vec![0]) {
                    party.monkeys[party.monkeys[i].pass].items.push(worry.clone());
                } else {
                    party.monkeys[party.monkeys[i].fail].items.push(worry.clone());
                }
                party.inspections[i] += BigUint::new(vec![1]);
            }
        }
        if rounds%10 == 0 {
            print!("{}",rounds)
        }
    }

    party.inspections.sort();
    let r = party.inspections[7].clone() * party.inspections[6].clone();
    println!("Monkey business is {}", r);
}
