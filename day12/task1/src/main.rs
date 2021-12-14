use std::collections::HashMap;
use std::vec;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("task1: {}", task1(&input));
    println!("task2: {}", task2(&input));
}

fn parse(s: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();
    let caves = s
        .trim()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|l| l.split('-'));

    for path in caves {
        for elems in path.collect::<Vec<&str>>().windows(2).into_iter() {
            let (a, b) = (elems[0], elems[1]);
            let a_entry = graph.entry(a).or_insert_with(Vec::<&str>::new);
            (*a_entry).push(b);
            let b_entry = graph.entry(b).or_insert_with(Vec::<&str>::new);
            (*b_entry).push(a);
        }
    }
    graph
}

fn is_little_cave(s: &str) -> bool {
    s.to_uppercase() != s
}

#[derive(Clone)]
struct OldMoves<'a> {
    seen: Vec<&'a str>,
    duplicated: Option<&'a str>,
}

impl<'a> OldMoves<'a> {
    fn push(&mut self, new_value: &'a str) {
        if is_little_cave(new_value)
            && self.duplicated.is_none()
            && new_value != "start"
            && new_value != "end"
            && self.seen.contains(&new_value)
        {
            self.duplicated = Some(new_value);
        }
        self.seen.push(new_value)
    }
}

impl OldMoves<'static> {
    fn start() -> OldMoves<'static> {
        OldMoves {
            seen: vec!["start"],
            duplicated: None,
        }
    }
}

fn find_possible_path<'a>(
    possible_moves: &'a HashMap<&str, Vec<&str>>,
    from: OldMoves<'a>,
    check_can_visit: fn(&'a str, &OldMoves<'a>) -> bool,
) -> Vec<OldMoves<'a>> {
    let last = from.seen.last().unwrap();
    if *last == "end" {
        return vec![from];
    }
    let next = possible_moves.get(last);
    match next {
        None => vec![],
        Some(elems) => elems
            .iter()
            .filter(|s| check_can_visit(s, &from))
            .flat_map(|next| {
                let mut c = from.clone();
                c.push(next);
                find_possible_path(possible_moves, c, check_can_visit)
            })
            .collect::<Vec<OldMoves>>(),
    }
}

fn task1(s: &str) -> usize {
    let graph = parse(s);
    let moves_possible = find_possible_path(&graph, OldMoves::start(), |s, prev| {
        if is_little_cave(s) {
            !prev.seen.contains(&s)
        } else {
            true
        }
    });

    moves_possible.len()
}

fn task2(s: &str) -> usize {
    let graph = parse(s);
    let moves_possible = find_possible_path(&graph, OldMoves::start(), |s, prev| {
        if is_little_cave(s) && s != "end" && s != "start" {
            prev.duplicated.is_none() || !prev.seen.contains(&s)
        } else if is_little_cave(s) {
            !prev.seen.contains(&s)
        } else {
            true
        }
    });

    moves_possible.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "start-A
start-b
A-c
A-b
b-d
A-end
b-end";
        let task1 = task1(&input);
        assert_eq!(task1, 19);
    }
}
