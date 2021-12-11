use anyhow::anyhow;
use std::{fmt::Error, usize};
pub enum Bracket {
    Open(char),
    Close(char),
}

impl Bracket {
    pub fn from_char(c: char) -> Option<Bracket> {
        match c {
            '{' | '[' | '(' | '<' => Some(Bracket::Open(c)),
            '}' => Some(Bracket::Close('{')),
            ']' => Some(Bracket::Close('[')),
            ')' => Some(Bracket::Close('(')),
            '>' => Some(Bracket::Close('<')),
            _ => None,
        }
    }
}

fn task_1(input: &str) -> usize {
    input
        .split('\n')
        .map(|line| find_offending(line).map_or(0, |c| calculate_syntax_error(c)))
        .sum::<usize>()
}

pub fn calculate_completion_score(s: char) -> usize {
    match s {
        ')' | '(' => 1,
        ']' | '[' => 2,
        '{' | '}' => 3,
        '<' | '>' => 4,
        _ => panic!("unexpected char {}", s),
    }
}
pub fn autocomplete_points(completed_chars: Vec<char>) -> usize {
    completed_chars
        .iter()
        .fold(0, |acc, a| acc * 5 + calculate_completion_score(*a))
}

fn task_2(input: &str) -> usize {
    let mut results: Vec<usize> = input
        .split('\n')
        .filter(|a| !a.trim().is_empty())
        .filter_map(|line| autocomplete(line).ok())
        .map(|completed| autocomplete_points(completed))
        .collect();
    results.sort();
    println!("{:?}", results);
    results[results.len() / 2]
}

pub fn calculate_syntax_error(s: char) -> usize {
    let points = match s {
        ')' | '(' => 3,
        ']' | '[' => 57,
        '{' | '}' => 1197,
        '<' | '>' => 25137,
        _ => panic!("unexpected char {}", s),
    };
    points
}
pub fn find_offending(string: &str) -> Option<char> {
    let mut brackets: Vec<char> = vec![];
    for c in string.chars() {
        match Bracket::from_char(c) {
            Some(Bracket::Open(char_bracket)) => {
                brackets.push(char_bracket);
            }
            Some(Bracket::Close(char_close_bracket)) => {
                if brackets.pop() != Some(char_close_bracket) {
                    return Some(char_close_bracket);
                }
            }
            _ => {}
        }
    }
    // some lines are empty but according to task we can ignore them
    // to check if there are some unmached chars we can check
    // if brackets are empty
    None
}

pub fn autocomplete(string: &str) -> anyhow::Result<Vec<char>> {
    let mut brackets: Vec<char> = vec![];
    for c in string.chars() {
        match Bracket::from_char(c) {
            Some(Bracket::Open(char_bracket)) => {
                brackets.push(char_bracket);
            }
            Some(Bracket::Close(char_close_bracket)) => {
                if brackets.pop() != Some(char_close_bracket) {
                    return Err(anyhow!("invalid char {}", char_close_bracket));
                }
            }
            _ => {}
        }
    }
    brackets.reverse();
    Ok(brackets)
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let task1 = task_1(&input);
    println!("task 1 {}", task1);
    let task2 = task_2(&input);
    println!("task 2 {}", task2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        assert_eq!(find_offending("{([(<{}[<>[]}>{[]{[(<()>"), Some('{'));
        assert_eq!(find_offending("[[<[([]))<([[{}[[()]]]"), Some('('));
        assert_eq!(find_offending("[({(<(())[]>[[{[]{<()<>>"), None);
        assert_eq!(find_offending("[{[{({}]{}}([{[{{{}}([]"), Some('['));
        assert_eq!(find_offending("[<(<(<(<{}))><([]([]()"), Some('('));
        assert_eq!(find_offending("<{([([[(<>()){}]>(<<{{"), Some('<'));
    }

    #[test]
    fn task_1_test_input() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(task_1(input), 26397);
    }

    #[test]
    fn task_2_test_input() {
        let input = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";
        assert_eq!(task_2(input), 288957);
    }

    fn points(s: &str) -> usize {
        autocomplete_points(autocomplete(s).unwrap())
    }
    #[test]
    fn test_autocompletion() {
        assert_eq!(points("[({(<(())[]>[[{[]{<()<>>"), 288957);
        assert_eq!(points("[(()[<>])]({[<{<<[]>>("), 5566);
        assert_eq!(points("(((({<>}<{<{<>}{[]{[]{}"), 1480781);
        assert_eq!(points("{<[[]]>}<{[{[{[]{()[[[]"), 995444);
        assert_eq!(points("<{([{{}}[<[[[<>{}]]]>[]]"), 294);
    }
}
