use std::iter;
fn main() {
    const RADIX: u32 = 10;
    const MATRIX_COLUMN_SIZE: usize = 100;
    const MATRIX_ROW_COUNT: usize = 100;
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.split('\n');
    let numbers = lines.map(|line| line.chars().map(|c| c.to_digit(RADIX).unwrap() as u8));

    let mut arr: [[u8; MATRIX_COLUMN_SIZE]; MATRIX_ROW_COUNT] =
        [[0; MATRIX_COLUMN_SIZE]; MATRIX_ROW_COUNT];
    for (x, row) in numbers.enumerate() {
        for (y, elem) in row.enumerate() {
            arr[x][y] = elem;
        }
    }

    fn is_min(a: u8, other: Vec<u8>) -> bool {
        a < *other.iter().min().unwrap()
    }

    let mut minimums: Vec<u8> = vec![];
    for (x, row) in arr.iter().enumerate() {
        for (y, curr) in row.iter().enumerate() {
            if is_min(
                *curr,
                adjacent_fields((x, y))
                    .iter()
                    .map(|(x, y)| arr[*x][*y])
                    .collect(),
            ) {
                minimums.push(*curr);
            }
        }
    }

    println!(
        "task 1 {}",
        minimums.len() + minimums.into_iter().sum::<u8>() as usize
    );

    let mut minimums: Vec<usize> = vec![];

    fn adjacent_fields(p: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = p;
        let mut vec = vec![];
        if x > 0 {
            vec.push((x - 1, y)); //left
        }
        if x < MATRIX_ROW_COUNT - 1 {
            vec.push((x + 1, y)); //right
        }
        if y > 0 {
            vec.push((x, y - 1)); // down
        }
        if y < MATRIX_COLUMN_SIZE - 1 {
            vec.push((x, y + 1)); //up
        }
        vec
    }

    fn growing_iter<'a>(
        arr: &'a [[u8; MATRIX_COLUMN_SIZE]; MATRIX_ROW_COUNT],
        previous: (usize, usize),
        pos: (usize, usize),
    ) -> Box<dyn Iterator<Item = (usize, usize)> + 'a> {
        let (x, y) = pos;
        if x > MATRIX_COLUMN_SIZE - 1 || y > MATRIX_COLUMN_SIZE - 1 {
            Box::new(iter::empty())
        } else {
            let v = arr[x][y];
            let previous_v = arr[previous.0][previous.1];
            if v > previous_v && v < 9 {
                let following = adjacent_fields(pos)
                    .into_iter()
                    .filter(move |p| p != &previous)
                    .flat_map(move |p| growing_iter(arr, pos, p));
                Box::new([(x, y)].into_iter().chain(following))
            } else {
                Box::new(iter::empty())
            }
        }
    }

    for (x, row) in arr.iter().enumerate() {
        for (y, curr) in row.iter().enumerate() {
            let values = adjacent_fields((x, y));
            if is_min(*curr, values.iter().map(|(x, y)| arr[*x][*y]).collect()) {
                let mut group: Vec<(usize, usize)> = values
                    .iter()
                    .map(|(x, y)| (*x, *y))
                    .map(|p| growing_iter(&arr, (x, y), p))
                    .fold(vec![], |mut acc, a| {
                        acc.extend(a.collect::<Vec<(usize, usize)>>());
                        acc.push((x, y));
                        acc
                    });
                group.sort_by(|a, b| a.cmp(b));
                group.dedup();
                let sum = group.iter().count();
                minimums.push(sum);
            }
        }
    }

    minimums.sort();
    minimums.reverse();
    let best = &minimums[0..3];
    println!("{}", best[0] * best[1] * best[2])
}
