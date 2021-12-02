fn main() {
    let data = std::fs::read_to_string("../task1/input.txt")
        .unwrap()
        .lines()
        .map(|t| t.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let t = triples(data);
    println!("increased {}", count_how_many_times_increased(t))
}

fn triples(numbers: Vec<u32>) -> Vec<u32> {
    numbers
        .iter()
        .zip(numbers.iter().skip(1).zip(numbers.iter().skip(2)))
        .map(|(c, (n, l))| c + n + l)
        .collect()
}

fn count_how_many_times_increased(numbers: Vec<u32>) -> usize {
    let pairs = numbers.iter().zip(numbers.iter().skip(1));
    pairs.filter(|(current, next)| next > current).count()
}

mod tests {
    use super::*;
}
