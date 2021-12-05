use std::{borrow::BorrowMut, cmp};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let result = play(&input);
    println!("result: {}", result);
    let result2 = play_day_2(&input);
    println!("result task 2: {}", result2);
}

fn to_u32(s: &str) -> u32 {
    s.parse().unwrap()
}

struct Range {
    start: (u32, u32),
    end: (u32, u32),
}

#[derive(Default)]
struct Map {
    map: std::collections::HashMap<(u32, u32), u32>,
}

impl Map {
    fn add_pos(&mut self, position: (u32, u32)) -> () {
        let entry = self.map.entry(position).or_insert(0).borrow_mut();
        *entry += 1;
    }

    fn find_over_lap(&self) -> u32 {
        self.map.iter().filter(|v| *v.1 > 1).count() as u32
    }

    fn print(&self) -> () {
        println!();
        for x in 0..10 {
            for y in 0..10 {
                let letter = self
                    .map
                    .get(&(y, x))
                    .map(|r| format!("{}", r))
                    .or_else(|| Some(".".to_owned()))
                    .unwrap();
                if y == 0 {
                    println!()
                }
                print!("{}", letter);
            }
        }
    }
}

impl Range {
    fn from_str(s: &str) -> Range {
        let (from, to) = s.split_once("->").unwrap();
        let ((a, b), (c, d)) = (
            from.trim().split_once(',').unwrap(),
            to.trim().split_once(',').unwrap(),
        );
        Range {
            start: (to_u32(a), to_u32(b)),
            end: (to_u32(c), to_u32(d)),
        }
    }

    fn get_fields_horizontal_or_vertical(&self) -> Vec<(u32, u32)> {
        let (x1, y1) = self.start;
        let (x2, y2) = self.end;
        if x1 != x2 && y1 != y2 {
            return vec![];
        } else {
            (cmp::min(x1, x2)..=cmp::max(x1, x2))
                .flat_map(|x| (cmp::min(y1, y2)..=cmp::max(y1, y2)).map(move |y| (x, y)))
                .collect()
        }
    }

    fn get_fields_diagonal(&self) -> Vec<(u32, u32)> {
        let (x1, y1) = self.start;
        let (x2, y2) = self.end;

        // 6,4 2,0
        let diagonal = (x1 as i32 - x2 as i32 == y1 as i32 - y2 as i32) || (x1 + y1 == x2 + y2);
        if diagonal {
            let xs = cmp::min(x1, x2)..=cmp::max(x1, x2);
            let ys = cmp::min(y1, y2)..=cmp::max(y1, y2);
            let xs: Vec<u32> = if x2 > x1 {
                xs.rev().collect()
            } else {
                xs.collect()
            };
            let ys: Vec<u32> = if y2 > y1 {
                ys.rev().collect()
            } else {
                ys.collect()
            };
            let mut vec = xs
                .into_iter()
                .zip(ys.into_iter())
                .collect::<Vec<(u32, u32)>>();
            // for tests only
            vec.sort_by(|a, b| a.cmp(b));
            vec
        } else {
            self.get_fields_horizontal_or_vertical()
        }
    }
}
fn parse_input(s: &str) -> Vec<Range> {
    s.split('\n')
        .filter(|l| !l.is_empty())
        .map(|s| Range::from_str(s))
        .collect::<Vec<Range>>()
}

fn play(s: &str) -> u32 {
    let parsed: Vec<Range> = parse_input(s);
    let mut map = Map::default();
    parsed
        .iter()
        .flat_map(|r| r.get_fields_horizontal_or_vertical())
        .for_each(|e| map.add_pos(e));
    let result = map.find_over_lap();
    result
}

fn play_day_2(s: &str) -> u32 {
    let parsed: Vec<Range> = parse_input(s);
    let mut map = Map::default();
    parsed
        .iter()
        .flat_map(|r| r.get_fields_diagonal())
        .for_each(|e| map.add_pos(e));
    let result = map.find_over_lap();
    result
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_fields_for_range() {
        let range_b = Range::from_str("9,7 -> 7,7");
        assert_eq!(
            range_b.get_fields_horizontal_or_vertical(),
            vec![(7, 7), (8, 7), (9, 7)]
        );
        let range_a = Range::from_str("1,1 -> 1,3");
        assert_eq!(
            range_a.get_fields_horizontal_or_vertical(),
            vec![(1, 1), (1, 2), (1, 3)]
        );
    }

    #[test]
    fn check_field_with_diagonal() {
        let range_a = Range::from_str("1,1 -> 3,3");
        assert_eq!(range_a.get_fields_diagonal(), vec![(1, 1), (2, 2), (3, 3)]);
        let range_b = Range::from_str("9,7 -> 7,9");
        assert_eq!(range_b.get_fields_diagonal(), vec![(7, 9), (8, 8), (9, 7)]);
        let range_old = Range::from_str("9,7 -> 7,7");
        assert_eq!(
            range_old.get_fields_diagonal(),
            vec![(7, 7), (8, 7), (9, 7)]
        );
        let range_old = Range::from_str("1,1 -> 1,3");
        assert_eq!(
            range_old.get_fields_diagonal(),
            vec![(1, 1), (1, 2), (1, 3)]
        );
    }

    #[test]
    fn test_input() {
        let input = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";
        let result = play(input);
        assert_eq!(result, 5);

        let result_task2 = play_day_2(input);
        assert_eq!(result_task2, 12);
    }

    #[test]
    fn test_prod_reading() {
        let mut d = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("input.txt");
        let input = std::fs::read_to_string(d).unwrap();
        let result = play(&input);
        println!("result: {}", result);
    }
}
