fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let res: [u32; 13] = input.lines().fold([0; 13], |acc, line: &str| {
        let mut acc = acc.clone();
        let chars: Vec<char> = line.chars().collect();
        for i in 0..12 {
            acc[i] += chars[i].to_digit(10).unwrap();
        }
        acc[12] += 1;
        acc
    });
    let len = res[12];
    let mut gamma = String::new();
    let mut epsilon = String::new();
    for &i in res[0..12].iter() {
        if i >= len / 2 {
            gamma.push('1');
            epsilon.push('0')
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    println!("{:?} {}", res, len);
    let gamma_decimal = u32::from_str_radix(&gamma, 2).unwrap();
    println!("gamma: {}, decimal: {}", &gamma, gamma_decimal);

    let epsilon_decimal = u32::from_str_radix(&epsilon, 2).unwrap();
    println!(
        "epsilon {:08b}, decimal: {}",
        epsilon_decimal, epsilon_decimal
    );
    println!("result {}", epsilon_decimal * gamma_decimal)
}

#[cfg(test)]
mod tests {

    #[test]
    fn chars_to_digit() {
        assert_eq!('0'.to_digit(10).unwrap(), 0);
        assert_eq!('1'.to_digit(10).unwrap(), 1);
    }
}
