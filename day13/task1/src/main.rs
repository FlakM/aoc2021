#![feature(map_first_last)]
use std::{collections::BTreeMap, vec};

fn main() {
    let input = include_str!("../input.txt");
    println!("task 1 {}", task1(&input));
    task2(&input)
}

fn transpose2<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[derive(Clone, PartialEq, Eq)]
struct Sheet {
    ones: Vec<Vec<bool>>,
}

impl Sheet {
    fn fold(&mut self, n: usize, on_x_axis: bool) {
        let x = &self.ones.len();
        let y = &self.ones[0].len();
        if !on_x_axis {
            self.ones = transpose2(self.ones.clone());
        }
        let mut a = vec![];
        let mut b = vec![];
        for lines in &self.ones {
            let (aa, bb) = split_arr(lines, n + 1);
            a.push(aa);
            b.push(bb.iter().rev().copied().collect::<Vec<bool>>());
        }
        self.ones = a.iter().zip(b).map(|(a, b)| or(a, &b[..])).collect();
        if !on_x_axis {
            self.ones = transpose2(self.ones.clone());
        }
    }

    fn count_ones(&self) -> usize {
        self.ones
            .iter()
            .map(|a| a.iter().filter(|a| **a).count())
            .sum()
    }

    fn print(&self) {
        for row in &self.ones {
            println!();
            for elem in row {
                if *elem {
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!()
    }
}
fn parse_sheet(s: &str) -> (Sheet, Vec<(bool, usize)>) {
    let (coordinates, folds) = s.split_once("\n\n").unwrap();
    let mut max_column = 0;
    let rows = coordinates
        .split('\n')
        .map(|line| {
            let (y, x) = line.split_once(',').unwrap();
            (y.parse::<usize>().unwrap(), x.parse::<usize>().unwrap())
        })
        .fold(BTreeMap::new(), |mut acc, (y, x)| {
            let entry = acc.entry(x).or_insert_with(|| vec![false; 2000]); //estimated max_column_size
            max_column = usize::max(max_column, y);
            (*entry)[y] = true;
            acc
        });

    let last = rows.last_key_value().unwrap().0;

    println!("size {}x{}", last, max_column);
    let sheet: Sheet = Sheet {
        ones: (0..=*last).fold(vec![], |mut acc, num| {
            let empty = vec![false; max_column + 1];
            let r = rows.get(&num).unwrap_or(&empty);
            acc.push(r[0..=max_column].to_vec());
            acc
        }),
    };
    let folds = folds
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|line| {
            let (pre, num) = line.split_once('=').unwrap();
            (pre.ends_with('x'), num.parse().unwrap())
        })
        .collect::<Vec<(bool, usize)>>();
    (sheet, folds)
}

fn task2(s: &str) {
    let (mut sheet, folds) = parse_sheet(s);
    for (left, fold_on) in folds {
        println!("folding {} to left {}", fold_on, left);
        sheet.fold(fold_on, left);
    }
    sheet.print()
}

fn task1(s: &str) -> usize {
    let (mut sheet, folds) = parse_sheet(s);
    let (a, b) = folds[0];
    println!("folding {} to left: {}", b, a);
    sheet.fold(b, a);
    sheet.print();
    sheet.count_ones()
}
fn split_arr<T>(arr: &[T], n: usize) -> (&[T], &[T]) {
    (&arr[..n - 1], &arr[n..])
}

fn or<'a>(a: &'a [bool], b: &'a [bool]) -> Vec<bool> {
    a.iter().zip(b).map(|(a, b)| a | b).collect::<Vec<bool>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_arr() {
        let arr = vec![1, 2, 3, 4, 5];
        let (a, _) = split_arr(&arr, 3);
        assert_eq!(a, &[1, 2])
    }

    #[test]
    fn test_masking() {
        let a: u8 = 0b10101;
        let mask: u8 = u8::MAX << 2;
        let b = a & !mask;
        println!("a: {:08b} mask: {:08b} b: {:08b}", a, mask, b)
    }

    #[test]
    fn test_task_input() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        let result = task1(&input);
        assert_eq!(result, 17)
    }

    #[test]
    fn test_task_2() {
        let input = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";
        task2(&input);
    }
}
