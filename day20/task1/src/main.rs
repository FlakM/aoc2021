use std::collections::HashMap;

struct Image {
    pixel_map: HashMap<(isize, isize), bool>,
    outer_marked: bool,
}

fn points(a: isize, b: isize) -> [(isize, isize); 9] {
    let mut ret = [(0, 0); 9];
    let mut iter = 0;
    for x in a - 1..=a + 1 {
        for y in b - 1..=b + 1 {
            ret[iter] = (x, y);
            iter += 1;
        }
    }
    ret
}
fn to_u32(slice: &[bool; 9]) -> usize {
    slice
        .iter()
        .rev()
        .enumerate()
        .fold(0, |acc, (i, &b)| acc | (b as usize) << i)
}
#[test]
fn test_to_u32() {
    let f = false;
    assert_eq!(to_u32(&[f, f, f, f, f, f, f, f, true]), 1);
    assert_eq!(to_u32(&[f, f, f, f, f, f, f, true, true]), 3);
}

impl Image {
    fn enhance(&mut self, enhancement_lookup: &[bool; 512]) {
        let mut new_image = HashMap::new();

        for (&(xx, yy), _) in &self.pixel_map {
            let p = points(xx, yy);
            for (x, y) in p {
                let bits =
                    points(x, y).map(|i| *self.pixel_map.get(&i).unwrap_or(&self.outer_marked));
                let sum = to_u32(&bits);
                new_image.insert((x, y), enhancement_lookup[sum]);
            }
        }
        let new_outer_marked = if self.outer_marked {
            *enhancement_lookup.last().unwrap()
        } else {
            enhancement_lookup[0]
        };
        *self = Image {
            pixel_map: new_image,
            outer_marked: new_outer_marked,
        };
    }

    fn count(&self) -> usize {
        self.pixel_map.iter().filter(|(_, v)| **v).count()
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (lookup_table, image) = input.split_once("\n\n").unwrap();
    let table: [bool; 512] = lookup_table[..512]
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap();
    let image = image
        .split('\n')
        .enumerate()
        .fold(HashMap::new(), |mut acc, (x, l)| {
            l.chars().enumerate().for_each(|(y, c)| {
                acc.insert((x as isize, y as isize), c == '#');
            });
            acc
        });
    let mut image = Image {
        pixel_map: image,
        outer_marked: false,
    };

    image.enhance(&table);
    image.enhance(&table);

    println!("task 1: {}", image.count());
    for _ in 0..48 {
        image.enhance(&table)
    }
    println!("task 2: {}", image.count());
}
