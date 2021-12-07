use std::collections::HashMap;
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let state = Movements::parse_movements(&input);
    let min = state.find_min(task_1_fuel_strategy);
    println!("result task 1 {}", min);
    let min = state.find_min(task_2_fuel_strategy);
    println!("result task 2 {}", min)
}

#[derive(Default)]
struct Movements {
    crabs_locations: HashMap<i32, i32>,
    min: i32,
    max: i32,
}
impl Movements {
    fn parse_movements(s: &str) -> Movements {
        s.split(',').map(|s| s.trim().parse().unwrap()).fold(
            Movements::default(),
            |mut state, n| {
                let count = state.crabs_locations.entry(n).or_insert(0);
                *count += 1;
                state.max = std::cmp::max(state.max, n);
                state.min = std::cmp::min(state.min, n);
                state
            },
        )
    }

    fn find_min(&self, cost_fn: fn(i32, &Movements) -> i32) -> i32 {
        (self.min..self.max)
            .filter(|v| v != &0)
            .map(|hmove| cost_fn(hmove, &self))
            .min()
            .unwrap()
    }
}
fn task_1_fuel_strategy(move_n: i32, state: &Movements) -> i32 {
    state
        .crabs_locations
        .iter()
        .map(|(k, v)| (*k - move_n).abs() as i32 * v)
        .sum()
}

fn task_2_fuel_strategy(move_n: i32, state: &Movements) -> i32 {
    state
        .crabs_locations
        .iter()
        .map(|(k, v)| {
            let distance = (*k - move_n).abs();
            let partial: f32 = ((distance + 1) as f32 / 2f32) as f32 * distance as f32;
            let result = partial as i32 * v;
            result
        })
        .sum::<i32>()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example2() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let state = Movements::parse_movements(input);
        assert_eq!(task_2_fuel_strategy(2, &state), 206);
    }

    #[test]
    fn test_example() {
        let input = "16,1,2,0,4,2,7,1,2,14";
        let state = Movements::parse_movements(input);
        assert_eq!(state.min, 0);
        assert_eq!(state.max, 16);
        assert_eq!(task_1_fuel_strategy(1, &state), 41);
        assert_eq!(task_2_fuel_strategy(2, &state), 206);
        assert_eq!(state.find_min(task_1_fuel_strategy), 37);
        assert_eq!(state.find_min(task_2_fuel_strategy), 168);
    }
}
