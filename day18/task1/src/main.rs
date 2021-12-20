use itertools::Itertools;
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;
#[derive(Debug, PartialEq, Clone)]
pub enum SnailNumberValue {
    Regular(u64),
    // todo add pair utility method here
    Complex {
        left: Box<SnailNumberValue>,
        right: Box<SnailNumberValue>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct SnailFishNumber {
    left: SnailNumberValue,
    right: SnailNumberValue,
}

impl SnailFishNumber {
    fn to_complex(self) -> SnailNumberValue {
        SnailNumberValue::Complex {
            left: Box::new(self.left),
            right: Box::new(self.right),
        }
    }
}

fn add_beggining(snail: &mut SnailNumberValue, value: Option<u64>) {
    if let Some(v) = value {
        match snail {
            SnailNumberValue::Regular(a) => {
                *a += v;
            }
            SnailNumberValue::Complex { left, .. } => add_beggining(&mut *left, value),
        }
    }
}

fn add_end(snail: &mut SnailNumberValue, value: Option<u64>) {
    if let Some(v) = value {
        match snail {
            SnailNumberValue::Regular(a) => {
                *a += v;
            }
            SnailNumberValue::Complex { right, .. } => add_end(&mut *right, value),
        }
    }
}

fn explode(snail: &mut SnailNumberValue, n: u64) -> (bool, Option<u64>, Option<u64>) {
    match snail {
        SnailNumberValue::Regular(_) => (false, None, None),
        SnailNumberValue::Complex { left, right } if n > 3 => match (&**left, &**right) {
            (SnailNumberValue::Regular(n1), SnailNumberValue::Regular(n2)) => {
                let res = (true, Some(*n1), Some(*n2));
                *snail = SnailNumberValue::Regular(0);
                res
            }
            _ => panic!(),
        },
        SnailNumberValue::Complex { left, right } => {
            let (t, a1, a2) = explode(&mut *left, n + 1);
            if t {
                add_beggining(&mut *right, a2);
                return (true, a1, None);
            }
            let (t, a1, a2) = explode(&mut *right, n + 1);
            if t {
                add_end(&mut *left, a1);
                return (true, None, a2);
            }
            (false, None, None)
        }
    }
}

fn do_explode(t: &mut SnailNumberValue) -> bool {
    explode(t, 0).0
}
use SnailNumberValue::*;

fn split(t: &mut SnailNumberValue) -> bool {
    match t {
        SnailNumberValue::Regular(a) => {
            if *a >= 10 {
                *t = Complex {
                    left: Box::new(Regular(*a / 2)),
                    right: Box::new(Regular((*a + 1) / 2)),
                };
                true
            } else {
                false
            }
        }
        Complex { left, right } => split(&mut *left) || split(&mut *right),
    }
}

fn reduce(i: &mut SnailNumberValue) {
    loop {
        while do_explode(i) {}
        if !split(i) {
            break;
        }
    }
}

fn magnitude(t: &SnailNumberValue) -> u64 {
    match t {
        &Regular(a) => a,
        Complex { left, right } => 3 * magnitude(&*left) + 2 * magnitude(&*right),
    }
}

mod parsing {
    use super::*;
    use nom::{
        branch::alt,
        character::complete::{self, char},
        combinator::map,
        sequence::{delimited, separated_pair},
        IResult,
    };
    fn pair(input: &str) -> IResult<&str, (Box<SnailNumberValue>, Box<SnailNumberValue>)> {
        delimited(
            char('['),
            separated_pair(snail_value, char(','), snail_value),
            char(']'),
        )(input)
        .map(|(s, (a, b))| (s, (Box::new(a), Box::new(b))))
    }

    fn snail_value(input: &str) -> IResult<&str, SnailNumberValue> {
        use SnailNumberValue::*;
        alt((
            map(complete::u64, Regular),
            map(pair, |(left, right)| Complex { left, right }),
        ))(input)
    }

    pub fn snail_fish_number(input: &str) -> IResult<&str, SnailFishNumber> {
        delimited(
            char('['),
            separated_pair(snail_value, char(','), snail_value),
            char(']'),
        )(input)
        .map(|(s, (left, right))| (s, SnailFishNumber { left, right }))
    }
}

impl FromStr for SnailFishNumber {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parsing::snail_fish_number(s).map(|a| a.1).map_err(|err| {
            eprintln!("{}", err);
            panic!("unable to parse");
        })
    }
}
impl fmt::Display for SnailNumberValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular(a) => write!(f, "{}", a),
            Self::Complex { left, right } => {
                write!(f, "[")?;
                left.fmt(f)?;
                write!(f, ",")?;
                right.fmt(f)?;
                write!(f, "]")
            }
        }
    }
}

impl fmt::Display for SnailFishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        self.left.fmt(f)?;
        write!(f, ",")?;
        self.right.fmt(f)?;
        write!(f, "]")
    }
}

fn add(a: SnailNumberValue, b: SnailNumberValue) -> u64 {
    let mut n = SnailNumberValue::Complex {
        left: Box::new(a),
        right: Box::new(b),
    };
    reduce(&mut n);
    magnitude(&n)
}

fn main() {
    let input_test = std::fs::read_to_string("input.txt").unwrap();
    let input: Vec<SnailNumberValue> = input_test
        .trim()
        .split("\n")
        .map(|l| parsing::snail_fish_number(l).unwrap().1.to_complex())
        .collect::<Vec<SnailNumberValue>>();

    let task2 = input
        .iter()
        .cloned()
        .reduce(|a, b| {
            let mut n = SnailNumberValue::Complex {
                left: Box::new(a),
                right: Box::new(b),
            };
            reduce(&mut n);
            n
        })
        .unwrap();
    println!("Task 1 {}", magnitude(&task2));

    let task2 = input
        .iter()
        .cartesian_product(input.iter())
        .map(|(a, b)| add(a.clone(), b.clone()))
        .max()
        .unwrap();
    println!("Task 2 {}", task2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_format() {
        let a = "[1,2]";
        let expected = SnailFishNumber {
            left: Regular(1),
            right: Regular(2),
        };
        let (_, parsed) = parsing::snail_fish_number(&a).unwrap();
        assert_eq!(parsed, expected);

        let b = "[9,[8,7]]";
        assert_eq!(
            parsing::snail_fish_number(b).unwrap().1,
            SnailFishNumber {
                left: Regular(9),
                right: Complex {
                    left: Box::new(Regular(8)),
                    right: Box::new(Regular(7))
                }
            }
        );

        let c = "[[1,[2,3]],[4,5]]";

        assert_eq!(
            parsing::snail_fish_number(c).unwrap().1,
            SnailFishNumber {
                left: Complex {
                    left: Box::new(Regular(1)),
                    right: Box::new(Complex {
                        left: Box::new(Regular(2)),
                        right: Box::new(Regular(3))
                    })
                },
                right: Complex {
                    left: Box::new(Regular(4)),
                    right: Box::new(Regular(5))
                }
            }
        );
    }
}
