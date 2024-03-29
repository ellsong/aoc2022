enum Op {
    Mult(u64),
    Plus(u64),
    Square,
}

struct Monkey {
    items: Vec<u64>,
    op: Op,
    test: u64,
    pass: usize,
    fail: usize,
}

impl Monkey {
    fn business() -> [Monkey; 8] {
        [
            Monkey {
                items: Vec::from([65, 58, 93, 57, 66]),
                op: Op::Mult(7),
                test: 19,
                pass: 6,
                fail: 4,
            },
            Monkey {
                items: Vec::from([76, 97, 58, 72, 57, 92, 82]),
                op: Op::Plus(4),
                test: 3,
                pass: 7,
                fail: 5,
            },
            Monkey {
                items: Vec::from([90, 89, 96]),
                op: Op::Mult(5),
                test: 13,
                pass: 5,
                fail: 1,
            },
            Monkey {
                items: Vec::from([72, 63, 72, 99]),
                op: Op::Square,
                test: 17,
                pass: 0,
                fail: 4,
            },
            Monkey {
                items: Vec::from([65]),
                op: Op::Plus(1),
                test: 2,
                pass: 6,
                fail: 2,
            },
            Monkey {
                items: Vec::from([97, 71]),
                op: Op::Plus(8),
                test: 11,
                pass: 7,
                fail: 3,
            },
            Monkey {
                items: Vec::from([83, 68, 88, 55, 87, 67]),
                op: Op::Plus(2),
                test: 5,
                pass: 2,
                fail: 1,
            },
            Monkey {
                items: Vec::from([64, 81, 50, 96, 82, 53, 62, 92]),
                op: Op::Plus(5),
                test: 7,
                pass: 3,
                fail: 0,
            }
        ]
    }
}

struct Party {
    monkeys: [Monkey;8],
    inspections: [u64;8],
    divisor_product: u64,
}

impl Party {
    fn start() -> Self {
        let mb = Monkey::business();
        let divisor_product = mb.iter().map(|m| m.test).product();
        Party {
            monkeys: mb,
            inspections: [0;8],
            divisor_product
        }
    }
}

pub fn p2() {
    let mut party = Party::start();
    for _rounds in 0..10000 {
        for i in 0..party.monkeys.len() {
            while !party.monkeys[i].items.is_empty() {
                let mut worry = party.monkeys[i].items.remove(0);
                worry %= party.divisor_product;
                match party.monkeys[i].op {
                    Op::Mult(v) => worry *= v,
                    Op::Plus(v) => worry += v,
                    Op::Square => worry *= worry,
                }
                // worry /= 3;
                if worry%party.monkeys[i].test==0 {
                    party.monkeys[party.monkeys[i].pass].items.push(worry);
                } else {
                    party.monkeys[party.monkeys[i].fail].items.push(worry);
                }
                party.inspections[i] += 1;
            }
        }
    }

    party.inspections.sort();
    let r = party.inspections[7] * party.inspections[6];
    println!("Monkey business is {}", r);
}