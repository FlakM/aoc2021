#[derive(Debug)]
pub enum Motion {
    Forward(u32),
    Down(u32),
    Up(u32),
}

#[derive(Debug)]
pub struct Position {
    depth: u32,
    horizontal: u32,
}

impl Position {
    pub fn start_position() -> Position {
        Position {
            depth: 0,
            horizontal: 0,
        }
    }

    pub fn start(&mut self, motions: Vec<Motion>) {
        for instruction in motions {
            self.swim(instruction);
        }
    }

    fn swim(&mut self, motion: Motion) {
        match motion {
            Motion::Forward(m) => self.horizontal += m,
            Motion::Down(d) => self.depth += d,
            Motion::Up(u) => self.depth -= u,
        }
    }
}

impl Motion {
    pub fn from_str(line: &str) -> Motion {
        let input: Vec<&str> = line.split(' ').collect();
        let (motion, n) = (input[0], input[1].parse::<u32>().unwrap());
        let motion = match motion {
            "forward" => Motion::Forward(n),
            "down" => Motion::Down(n),
            "up" => Motion::Up(n),
            _ => panic!("what?"),
        };
        return motion;
    }
}

fn main() {
    let motions = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|t| Motion::from_str(t))
        .collect::<Vec<Motion>>();
    let mut position = Position::start_position();
    position.start(motions);

    println!(
        "Final position: {:?}, task 1 result: {}",
        position,
        position.horizontal * position.depth
    );
}

#[cfg(test)]
mod tests {
    use super::Motion::*;
    use super::*;

    #[test]
    fn basic() {
        let motions = vec![Forward(1), Down(2), Up(1), Forward(1)];
        let mut position = Position::start_position();
        position.start(motions);
        assert_eq!(position.depth, 1);
        assert_eq!(position.horizontal, 2);
    }
}
