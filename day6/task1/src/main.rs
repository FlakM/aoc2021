use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FishPopulation {
    population: [usize; 9],
}

impl FishPopulation {
    pub fn from_str(input: &str) -> Self {
        let input = input
            .split(',')
            .map(|l| l.parse::<usize>().unwrap() - 1)
            .into_iter()
            .fold(HashMap::<usize, usize>::new(), |mut m, x| {
                *m.entry(x).or_default() += 1;
                m
            });
        let new_population: [usize; 9] = (0..=8)
            .map(|i| *input.get(&i).or_else(|| Some(&0)).unwrap())
            .collect::<Vec<usize>>()
            .try_into()
            .unwrap();
        FishPopulation {
            population: new_population,
        }
    }
    pub fn count(&self) -> usize {
        self.population.iter().fold(0, |x, y| x + y)
    }

    pub fn one_day(&mut self) -> () {
        let to_be_born = self.population[0];
        let mut new_population = [0; 9];
        for i in 1..9 {
            new_population[i - 1] = self.population[i];
        }
        // new fishes
        new_population[8] = to_be_born;
        // old fishes but restarted
        new_population[6] += to_be_born;
        self.population = new_population;
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut population = FishPopulation::from_str(input.trim());
    for _ in 1..80 {
        population.one_day();
    }
    println!("task 1: {}", population.count());
    let mut population2 = FishPopulation::from_str(input.trim());
    for _ in 1..256 {
        population2.one_day();
    }
    println!("task 2: {}", population2.count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let mut population = FishPopulation::from_str("3,4,3,1,2");
        println!("{:?}", population.population);
        for _ in 0..17 {
            population.one_day();
        }
        println!("{:?}", population.population);
        assert_eq!(population.count(), 26)
    }
    #[test]
    fn test_day_passing() {
        let mut population = FishPopulation::from_str("6");
        assert_eq!(population.population, [0, 0, 0, 0, 0, 1, 0, 0, 0]);
        population.one_day();
        assert_eq!(population.population, [0, 0, 0, 0, 1, 0, 0, 0, 0]);
        population.one_day();
        population.one_day();
        population.one_day();
        population.one_day();
        population.one_day();
        assert_eq!(population.population, [0, 0, 0, 0, 0, 0, 1, 0, 1]);
        population.one_day();
        assert_eq!(population.population, [0, 0, 0, 0, 0, 1, 0, 1, 0]);
    }
}
