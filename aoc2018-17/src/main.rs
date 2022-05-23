use core::fmt::{Display, Formatter};
use std::collections::HashMap;

mod line;
use line::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Val {
    Sand,
    Clay,
    Flow,
    Still,
    OOB,
    Source,
}

impl Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Val::Sand => ".",
                Val::Clay => "#",
                Val::Flow => "|",
                Val::Still => "~",
                Val::OOB => "x",
                Val::Source => "+",
            }
        )
    }
}

type Ground = HashMap<Point, Val>;

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point {
    r: i64,
    c: i64,
}

impl Point {
    fn up(&self) -> Self {
        Self {
            r: self.r - 1,
            c: self.c,
        }
    }

    fn left(&self) -> Self {
        Self {
            r: self.r,
            c: self.c - 1,
        }
    }

    fn right(&self) -> Self {
        Self {
            r: self.r,
            c: self.c + 1,
        }
    }
}

#[derive(Debug)]
struct Earth {
    ground: Ground,
    min: Point,
    max: Point,
}

impl Earth {
    fn show(&self) {
        for r in self.min.r - 1..=self.max.r + 1 {
            for c in self.min.c - 1..=self.max.c + 1 {
                match self.ground.get(&Point { r, c }) {
                    None => print!("."),
                    Some(v) => print!("{v}"),
                }
            }
            println!();
        }
    }

    fn xlate(&self, loc: &Point) -> Val {
        if loc.r > self.max.r {
            Val::OOB
        } else {
            match self.ground.get(loc) {
                None => Val::Sand,
                Some(v) => *v,
            }
        }
    }

    fn peek(&self, loc: &Point) -> (Val, Val) {
        let below = Point {
            r: loc.r + 1,
            c: loc.c,
        };

        (self.xlate(loc), self.xlate(&below))
    }

    fn set(&mut self, loc: Point, val: Val) {
        if let Some(x) = self.ground.get(&loc) {
            if *x == Val::Clay {
                panic!("Overwriting clay at {loc:?}");
            }
        }
        self.ground.insert(loc, val);
    }
}

fn main() -> Result<(), String> {
    let file = std::env::args().nth(1).unwrap();
    let data = std::fs::read_to_string(file)
        .expect("can't read data")
        .split("\n")
        .map(|x| x.to_string())
        .filter(|x| !x.is_empty())
        .collect::<Vec<String>>();

    let mut ground = HashMap::new();

    let mut min_x = std::i64::MAX;
    let mut max_x = std::i64::MIN;
    let mut min_y = std::i64::MAX;
    let mut max_y = std::i64::MIN;

    for i in &data {
        let line = Line::parse(i.as_str())?;
        for l in line.iter() {
            if l.1 < min_x {
                min_x = l.1;
            }
            if l.1 > max_x {
                max_x = l.1;
            }
            if l.0 < min_y {
                min_y = l.0;
            }
            if l.0 > max_y {
                max_y = l.0;
            }
            ground.insert(Point { r: l.0, c: l.1 }, Val::Clay);
        }
    }

    ground.insert(Point { r: 0, c: 500 }, Val::Source);

    let mut earth = Earth {
        ground,
        min: Point { r: min_y, c: min_x },
        max: Point { r: max_y, c: max_x },
    };

    drop_water(&mut earth);

    let mut flowing = 0;
    let mut still = 0;
    for (k, v) in earth.ground {
        if k.r >= min_y {
            match v {
                Val::Flow => {
                    flowing += 1;
                }
                Val::Still => {
                    still += 1;
                }
                _ => {}
            }
        }
    }
    println!("Part 1: {}", flowing + still);
    println!("Part 2: {still}");

    /*
    println!(
        "count = {}",
        earth
            .ground
            .into_values()
            .filter(|x| *x == Val::Flow || *x == Val::Still)
            .count()
    );
    */

    Ok(())
}

fn drop_water(earth: &mut Earth) {
    let mut source = vec![Point { r: 1, c: 500 }];

    while !source.is_empty() {
        // println!("{:?}", source);
        let start = source.pop().unwrap();
        let mut pos = start;
        // earth.show();

        loop {
            let sight = earth.peek(&pos);
            // println!("{sight:?}");
            match sight {
                (Val::Sand, Val::OOB) => {
                    earth.set(pos, Val::Flow);
                    break;
                }
                (Val::Flow, Val::Sand) | (Val::Sand, Val::Sand) => {
                    earth.set(pos, Val::Flow);
                    pos.r += 1;
                }
                (Val::Sand, Val::Still) | (Val::Flow, Val::Still) | (Val::Sand, Val::Clay) => {
                    let mut cur = pos;
                    let left = loop {
                        match earth.peek(&cur.left()) {
                            (Val::Clay, _) => break true,
                            (Val::OOB, _) => {
                                earth.show();
                                panic!("out of bounds")
                            }
                            (Val::Flow, Val::Flow) | (Val::Sand, Val::Sand) => break false,
                            _ => cur = cur.left(),
                        }
                    };
                    let leftmost = cur.c - 1;
                    let mut cur = pos;
                    let right = loop {
                        match earth.peek(&cur.right()) {
                            (Val::Clay, _) => break true,
                            (Val::OOB, _) => panic!("out of bounds"),
                            (Val::Flow, Val::Flow) | (Val::Sand, Val::Sand) => break false,
                            _ => cur = cur.right(),
                        }
                    };
                    let rightmost = cur.c + 1;
                    // println!("scanned from {leftmost} to {rightmost}");
                    match (left, right) {
                        (true, true) => {
                            for c in leftmost + 1..rightmost {
                                earth.set(Point { r: cur.r, c }, Val::Still);
                            }
                            if pos.r > 0 {
                                source.push(pos.up());
                            }
                        }
                        (false, true) => {
                            for c in leftmost..rightmost {
                                earth.set(Point { r: cur.r, c }, Val::Flow);
                            }
                            source.push(Point {
                                r: cur.r,
                                c: leftmost,
                            });
                        }
                        (true, false) => {
                            for c in leftmost + 1..=rightmost {
                                earth.set(Point { r: cur.r, c }, Val::Flow);
                            }
                            source.push(Point {
                                r: cur.r,
                                c: rightmost,
                            });
                        }
                        (false, false) => {
                            for c in leftmost..=rightmost {
                                earth.set(Point { r: cur.r, c }, Val::Flow);
                            }
                            source.push(Point {
                                r: cur.r,
                                c: rightmost,
                            });
                            source.push(Point {
                                r: cur.r,
                                c: leftmost,
                            });
                        }
                    }
                    break;
                }
                (Val::Sand, Val::Flow) => {
                    earth.set(pos, Val::Flow);
                    break;
                }
                (Val::Flow, Val::Flow) | (Val::Still, Val::Still) => {
                    break;
                }
                _ => {
                    earth.show();
                    todo!()
                }
            }
        }
    }
}
