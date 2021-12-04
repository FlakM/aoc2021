pub fn main() {
    let input = std::fs::read_to_string("../task5/input.txt").unwrap();
    let inputs = to_integers(&input);
    let bits = input.lines().next().unwrap().chars().count() as u8;
    let oxygen = oxygen_generator_rating(&inputs, bits);
    let co2 = co2_rating(&inputs, bits);
    println!("result: {}", oxygen * co2)
}

fn to_integers(text: &str) -> Vec<u32> {
    text.lines()
        .map(|l| u32::from_str_radix(l, 2).unwrap())
        .collect::<Vec<u32>>()
}

// return bit at pos `n`
fn bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        panic!("out of range")
    }
}

fn oxygen_generator_rating(readings: &Vec<u32>, bits: u8) -> u32 {
    let mut readings = readings.clone();

    for i in 0..bits {
        let bit_index = bits - i - 1;
        let ones = readings.iter().filter(|b| bit_at(**b, bit_index)).count();
        let zeroes = readings.len() - ones;
        if ones >= zeroes {
            readings.retain(|b| bit_at(*b, bit_index));
        } else {
            readings.retain(|b| !bit_at(*b, bit_index));
        }
        if readings.len() == 1 {
            break;
        }
    }
    readings[0]
}

fn co2_rating(readings: &Vec<u32>, bits: u8) -> u32 {
    let mut readings = readings.clone();

    for i in 0..bits {
        let bit_index = bits - i - 1;
        let zeroes = readings.iter().filter(|b| !bit_at(**b, bit_index)).count();
        let ones = readings.len() - zeroes;
        if zeroes <= ones {
            readings.retain(|b| !bit_at(*b, bit_index));
        } else {
            readings.retain(|b| bit_at(*b, bit_index));
        }

        if readings.len() == 1 {
            break;
        }
    }
    readings[0]
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn check_bit_at() {
        assert_eq!(bit_at(0, 0), false);
        assert_eq!(bit_at(1, 0), true);
        assert_eq!(bit_at(2, 1), true);
    }

    const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn example_test() {
        let inputs = to_integers(TEST_INPUT);
        let bits = TEST_INPUT.lines().next().unwrap().chars().count() as u8;
        let oxygen = oxygen_generator_rating(&inputs, bits);
        let co2 = co2_rating(&inputs, bits);
        assert_eq!(oxygen, 23);
        assert_eq!(co2, 10);
    }
}
