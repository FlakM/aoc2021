use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone)]
pub enum Octopus {
    Normal(usize),
    Flashed,
}

impl Octopus {
    fn increment(&mut self) -> bool {
        match self {
            &mut Octopus::Normal(i) => {
                if i >= 9 {
                    *self = Octopus::Flashed;
                    true
                } else {
                    *self = Octopus::Normal(i + 1);
                    false
                }
            }
            &mut Octopus::Flashed => false,
        }
    }
}

#[derive(Clone)]
struct State {
    state: HashMap<(usize, usize), Octopus>,
}

impl State {
    fn new() -> State {
        State {
            state: HashMap::new(),
        }
    }

    fn find_neighbors(pos: (usize, usize)) -> Vec<(usize, usize)> {
        let (x, y) = pos;
        let mut vec = vec![];
        if x > 0 {
            vec.push((x - 1, y)); //left
        }
        if x < 10 - 1 {
            vec.push((x + 1, y)); //right
        }
        if y > 0 {
            vec.push((x, y - 1)); // down
        }
        if y < 10 - 1 {
            vec.push((x, y + 1)); //up
        }
        if x > 0 && y > 0 {
            vec.push((x - 1, y - 1)); //left top diagonal
        }
        if x < 10 - 1 && y > 0 {
            vec.push((x + 1, y - 1)); //left down diagonal
        }
        if y < 10 - 1 && x < 10 - 1 {
            vec.push((x + 1, y + 1)); //right down diagonal
        }
        if y < 10 - 1 && x > 0 {
            vec.push((x - 1, y + 1)); //right top diagonal
        }
        vec
    }
    fn increment_octopus(&mut self, to_be_incremented: Vec<(usize, usize)>) {
        to_be_incremented.iter().for_each(|pos| {
            let mut octopus = self.state.get_mut(&pos).unwrap();
            if octopus.increment() {
                self.increment_octopus(State::find_neighbors(*pos))
            }
        })
    }

    fn pass_day(&mut self) -> usize {
        for x in 0..10 {
            for y in 0..10 {
                self.increment_octopus(vec![(x, y)]);
            }
        }
        let mut counter: usize = 0;
        for (_, octopus) in self.state.iter_mut() {
            match octopus {
                &mut Octopus::Flashed => {
                    counter += 1;
                    *octopus = Octopus::Normal(0);
                }
                _ => (),
            }
        }
        counter
    }
}

impl FromStr for State {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.trim().split('\n').filter(|l| !l.is_empty());
        let mut state = State::new();
        for (x, row) in lines.enumerate() {
            for (y, curr) in row.chars().map(|c| c.to_digit(10).unwrap()).enumerate() {
                state.state.insert((x, y), Octopus::Normal(curr as usize));
            }
        }
        Ok(state)
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut state_original = State::from_str(&input).unwrap();
    let mut state = state_original.clone();
    let mut counter: usize = 0;
    for _i in 0..100 {
        counter += state.pass_day();
    }
    print!("task 1: {}", counter);
    let mut counter_task_2: usize = 1;
    while state_original.pass_day() != 100 {
        counter_task_2 += 1
    }
    print!("task 2: {}", counter_task_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut state = State::from_str(input).unwrap();
        assert_eq!(state.pass_day(), 0);
        assert_eq!(state.pass_day(), 35);
    }

    #[test]
    fn test_input_100_days() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut state = State::from_str(input).unwrap();
        let mut counter: usize = 0;
        for i in 0..100 {
            counter += state.pass_day();
        }
        assert_eq!(counter, 1656);
    }

    #[test]
    fn test_input_task2() {
        let input = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";
        let mut state = State::from_str(input).unwrap();
        let mut counter: usize = 1;
        while state.pass_day() != 100 {
            counter += 1
        }
        assert_eq!(counter, 195);
    }
}
