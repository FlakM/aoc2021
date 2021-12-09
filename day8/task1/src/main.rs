/// This is nasty ass fever induced coding
/// I figured out that there should be a way to encode signals in single u8 but i didn't figure correct
/// way. The way to do it is:
///
/// ```
///fn to_bits(s: &str) -> u8 {
///    s.as_bytes()
///        .iter()
///        .fold(0_u8, |acc, b| acc + (1 << (b - b'a') as usize))
///}
///```
///
/// that way tou can compare how many common segments does a digit have using simple operation:
///
///```
///(to_bits("ad") & to_bits("ad")).count_ones()
///```
/// taken from https://github.com/jeremylt/advent2021/blob/8eec78971f8595100b7bd8e69acef1e70ff4f2ba/src/day08.rs#L28
fn decode_all_numbers<'a>(signals: Vec<&'a str>) -> HashMap<String, u8> {
    let mut representation: Vec<CharEncoding> = vec![];
    let one = signals.iter().find(|a| a.len() == 2);
    let four = signals.iter().find(|a| a.len() == 4);
    let seven = signals.iter().find(|a| a.len() == 3);
    let eight = signals.iter().find(|a| a.len() == 7);

    let mut len_5 = signals
        .clone()
        .into_iter()
        .map(|e| {
            let mut chars = e.chars().collect::<Vec<char>>();
            chars.sort_by(|a, b| b.cmp(a));
            String::from_iter(chars)
        })
        .filter(|a| a.len() == 5)
        .collect::<Vec<String>>();
    len_5.sort();
    len_5.dedup();
    println!("post dedup: {:?}", len_5);
    let mut len_6 = signals
        .clone()
        .into_iter()
        .map(|e| {
            let mut chars = e.chars().collect::<Vec<char>>();
            chars.sort_by(|a, b| b.cmp(a));
            String::from_iter(chars)
        })
        .filter(|a| a.len() == 6)
        .collect::<Vec<String>>();
    len_6.dedup();

    let mut push = |code: String, located: u8| {
        representation.push(CharEncoding {
            chars: code,
            representation: located,
        });
    };

    fn join_letters(a: &str, b: &str) -> String {
        format!("{}{}", a, b)
    }

    fn contains_all(base: &str, located: &str) -> bool {
        base.chars().all(|c| located.contains(c))
    }

    if let Some(one) = one {
        push((*one).to_owned(), 1);

        len_5.retain(|tree| {
            let is_tree = contains_all(one, tree);
            if is_tree {
                push((*tree).clone(), 3);
            }
            !is_tree
        });

        if let Some(six) = len_6.iter().find(|p| !contains_all(one, p)) {
            push((*six).clone().into(), 6);
        }
    }
    if let Some(four) = four {
        push((*four).into(), 4);
    }

    if let (Some(four), Some(seven)) = (four, seven) {
        if let Some(nine) = len_6
            .iter()
            .find(|c| contains_all(four, c) && contains_all(&seven, c))
        {
            push((*nine).clone().into(), 9);
        }
    }
    if let (Some(four), Some(one)) = (four, one) {
        len_5.retain(|p| {
            let letters = &join_letters(p, one);
            let is_five = contains_all(&four, letters);
            if is_five {
                push((*p).clone(), 5);
            }
            !is_five
        });

        if let Some(zero) = len_6
            .iter()
            .find(|p| contains_all(one, p) && !contains_all(&four, p))
        {
            push((*zero.clone()).into(), 0);
        }
    }
    if let Some(seven) = seven {
        push((*seven).into(), 7);
    }

    if let Some(eight) = eight {
        push((*eight).into(), 8);
    }

    if len_5.len() == 1 {
        push(len_5[0].clone(), 2);
    }

    HashMap::from_iter(
        representation
            .iter()
            .map(|r| (sort(&r.chars), r.representation)),
    )
}

use std::collections::HashMap;
#[derive(Clone, Debug)]
struct CharEncoding {
    chars: String,
    representation: u8,
}

fn sort(s: &str) -> String {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort_by(|a, b| a.cmp(b));
    String::from_iter(chars)
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lengths = ["cf", "bcdf", "acf", "abcdefg"].map(|c| c.chars().count());
    let numbers: usize = input
        .split('\n')
        .map(|s| {
            s.split('|')
                .last()
                .unwrap()
                .split(' ')
                .map(|c| c.chars().count())
                .filter(|n| lengths.contains(n))
                .count()
        })
        .sum();
    println!("task 1: {}", numbers);

    let numbers: usize = input
        .trim()
        .split('\n')
        .map(|s| {
            let (_, elems) = s.split_once('|').unwrap();
            let whole = s.replace('|', "");
            let whole = whole
                .split(' ')
                .filter(|p| !p.is_empty())
                .collect::<Vec<&str>>();
            let nums = decode_all_numbers(whole);
            let responses: String = elems
                .split(' ')
                .filter(|p| !p.is_empty())
                .map(|signal| nums.get(&sort(signal)).unwrap())
                .map(|&id| id.to_string())
                .collect();
            responses.parse::<usize>().unwrap()
        })
        .sum::<usize>();
    println!("task 2: {}", numbers);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorting() {
        assert_eq!(&sort("cba".into()), "abc")
    }

    #[test]
    fn test_binary_mask() {
        let binary_a = 1 << (b'a' - b'a') as u8;
        assert_eq!(binary_a, 1);
        let binary_b = 1 << (b'b' - b'a') as u8;
        assert_eq!(binary_b, 2);
        let binary_c = 1 << (b'c' - b'a') as u8;
        assert_eq!(binary_c, 4);
        assert_eq!(1.count_zeros(), 0);
    }
}
