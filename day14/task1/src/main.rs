use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let original = parse(&input);
    let mut template = original.clone();
    println!("Task 1: {}", task(&mut template, 10));
    let mut template = original.clone();
    println!("Task 2: {}", task(&mut template, 40));
}

#[derive(Clone)]
struct PolymerTemplate {
    initial_template: String,
    insertions: HashMap<(char, char), char>,
    char_map: HashMap<char, usize>,
}

fn join_maps(a: HashMap<char, usize>, b: HashMap<char, usize>) -> HashMap<char, usize> {
    a.into_iter()
        .chain(b.into_iter())
        .fold(HashMap::new(), |mut acc, (k, v)| {
            let entry = acc.entry(k).or_insert(0);
            *entry += v;
            acc
        })
}

fn calculate_insertions(
    chars: (char, char),
    insertions: &HashMap<(char, char), char>,
    n: usize,
    cache: &mut HashMap<((char, char), usize), HashMap<char, usize>>,
) -> HashMap<char, usize> {
    if let Some(hit) = cache.get(&(chars, n)) {
        return hit.clone();
    }
    let mut char_map = HashMap::new();
    if n == 0 {
        if let Some(c) = insertions.get(&chars) {
            let ch = char_map.entry(*c).or_insert(0);
            *ch += 1;
        }
    } else {
        match insertions.get(&chars) {
            Some(insert) => {
                let (a, b) = chars;
                let entry = char_map.entry(*insert).or_insert(0);
                *entry += 1;

                let left = calculate_insertions((a, *insert), insertions, n - 1, cache);
                let right = calculate_insertions((*insert, b), insertions, n - 1, cache);
                char_map = join_maps(join_maps(left, right), char_map);
            }
            None => (),
        }
    }
    cache.insert((chars, n), char_map.clone());
    char_map
}
impl PolymerTemplate {
    fn steps_n(&mut self, steps: usize) -> HashMap<char, usize> {
        let mut cache = HashMap::new();
        let result =
            self.initial_template
                .as_bytes()
                .windows(2)
                .fold(HashMap::new(), |acc, bytes| {
                    let chars = (bytes[0] as char, bytes[1] as char);
                    join_maps(
                        calculate_insertions(chars, &self.insertions, steps, &mut cache),
                        acc,
                    )
                });
        result
    }
}

fn task(template: &mut PolymerTemplate, iterations: usize) -> usize {
    let iterations = iterations - 1;
    let mut char_map = template.steps_n(iterations);
    for c in template.initial_template.chars() {
        let entry = char_map.entry(c).or_insert(0);
        *entry += 1
    }
    let first = char_map.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
    let last = char_map.iter().max_by(|a, b| b.1.cmp(a.1)).unwrap();
    first.1 - last.1
}

fn parse(s: &str) -> PolymerTemplate {
    let (template, insertions) = s.split_once("\n\n").unwrap();
    let template = template.to_string();
    let insertions = insertions
        .trim()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            let (between, insert) = l.split_once(" -> ").unwrap();
            (
                (
                    between.chars().next().unwrap(),
                    between.chars().last().unwrap(),
                ),
                insert.chars().next().unwrap(),
            )
        })
        .collect();
    let mut char_map = HashMap::new();

    for c in template.chars() {
        let entry = char_map.entry(c).or_insert(0);
        println!("incrementing {}", c);
        *entry += 1
    }
    PolymerTemplate {
        initial_template: template,
        insertions,
        char_map: char_map,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion() {
        let input = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";
        let template_original = parse(input);
        assert_eq!(template_original.insertions.len(), 16);
        //Template:     NNCB
        //After step 1: NCNBCHB
        let mut template = template_original.clone();
        //task(&mut template, 1);
        //assert_eq!(*template.char_map.get(&'N').unwrap(), 2);
        //assert_eq!(*template.char_map.get(&'B').unwrap(), 2);
        //assert_eq!(*template.char_map.get(&'C').unwrap(), 2);
        //assert_eq!(*template.char_map.get(&'H').unwrap(), 1);

        //let mut template = template_original.clone();
        //task(&mut template, 2);
        ////After step 2: NBCCNBBBCBHCB
        //assert_eq!(*template.char_map.get(&'N').unwrap(), 2);
        //assert_eq!(*template.char_map.get(&'B').unwrap(), 6);
        //assert_eq!(*template.char_map.get(&'C').unwrap(), 4);
        //assert_eq!(*template.char_map.get(&'H').unwrap(), 1);
        //After step 3: NBBBCNCCNBBNBNBBCHBHHBCHB
        //After step 4: NBBNBNBBCCNBCNCCNBBNBBNBBBNBBNBBCBHCBHHNHCBBCBHCB
        assert_eq!(task(&mut template, 10), 1588);
        let mut template = template_original.clone();
        assert_eq!(task(&mut template, 40), 2188189693529)
    }
}
