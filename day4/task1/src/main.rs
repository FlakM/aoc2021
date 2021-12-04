use regex;

fn main() {
    let text = std::fs::read_to_string("input.txt").unwrap();
    let (numbers, boards) = parse_input(&text);
    let score = play(numbers.clone(), boards.clone());
    println!("winning score is {}", score);
    let worst_score = play_to_lose(numbers.clone(), boards.clone());
    println!("loosing score is {}", worst_score);
}

#[derive(Clone, PartialEq, Eq)]
struct Board {
    numbers: Vec<Vec<u32>>,
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

impl Board {
    fn new() -> Self {
        Board { numbers: vec![] }
    }

    fn has_won(&self, numbers: &Vec<u32>) -> Option<Vec<u32>> {
        let by_row = self
            .numbers
            .iter()
            .filter(|row| row.iter().all(|item| numbers.contains(item)))
            .next()
            .map(|v| v.clone());

        by_row.or_else(|| {
            transpose2(self.numbers.clone())
                .into_iter()
                .filter(|column| column.iter().all(|item| numbers.contains(item)))
                .next()
        })
    }
}
fn parse_input(text: &str) -> (Vec<u32>, Vec<Board>) {
    let mut lines = text.lines();

    let numbers = lines
        .next()
        .unwrap()
        .split(',')
        .map(|i| i.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    lines.next().unwrap();

    let mut boards: Vec<Board> = vec![];
    let mut counter = 0;
    let mut board = Board::new();
    for line in lines {
        if line.trim().is_empty() {
            continue;
        }
        let line = line.trim();
        let num = regex::Regex::new(r"\s+")
            .unwrap()
            .split(line)
            .map(|i| i.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        board.numbers.push(num);
        counter += 1;
        if counter == 5 {
            boards.push(board);
            board = Board::new();
            counter = 0;
        }
    }
    (numbers, boards)
}

fn play_to_lose(numbers: Vec<u32>, boards: Vec<Board>) -> u32 {
    let mut all_numbers = numbers.into_iter();
    let mut currently_picked: Vec<u32> = vec![];
    let mut boards_left_to_win: Vec<Board> = boards.clone();
    loop {
        if let Some((_row_that_won, board)) =
            pick_winner(&currently_picked, &boards_left_to_win.clone())
        {
            if boards_left_to_win.len() == 1 {
                boards_left_to_win.retain(|b| b != board);
                return calculate_score(
                    board,
                    &currently_picked,
                    *currently_picked.last().unwrap(),
                );
            } else {
                boards_left_to_win.retain(|b| b != board);
                continue;
            }
        } else {
            let next = &all_numbers.next().unwrap();
            currently_picked.push(*next);
            continue;
        }
    }
}

fn play(numbers: Vec<u32>, boards: Vec<Board>) -> u32 {
    let mut all_numbers = numbers.into_iter();
    let mut currently_picked: Vec<u32> = vec![];
    loop {
        if let Some((_row_that_won, board)) = pick_winner(&currently_picked, &boards) {
            return calculate_score(board, &currently_picked, *currently_picked.last().unwrap());
        } else {
            let next = &all_numbers.next().unwrap();
            currently_picked.push(*next);
        }
    }
}

fn pick_winner<'a>(numbers: &Vec<u32>, boards: &'a Vec<Board>) -> Option<(Vec<u32>, &'a Board)> {
    boards
        .iter()
        .map(|b| (b.has_won(numbers), b))
        .filter(|(n, _)| n.is_some())
        .map(|(n, b)| (n.unwrap(), b))
        .next()
}

fn calculate_score(board: &Board, all_numbers: &Vec<u32>, last: u32) -> u32 {
    let sum: u32 = board
        .numbers
        .iter()
        .flatten()
        .filter(|n| !all_numbers.contains(n))
        .sum();
    sum * last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_task1() {
        let text = std::fs::read_to_string("test_input.txt").unwrap();
        let (numbers, boards) = parse_input(&text);
        let score = play(numbers, boards);
        assert_eq!(score, 4512)
    }

    #[test]
    fn test_input_task2() {
        let text = std::fs::read_to_string("test_input.txt").unwrap();
        let (numbers, boards) = parse_input(&text);
        let score = play_to_lose(numbers, boards);
        assert_eq!(score, 1924)
    }
}
