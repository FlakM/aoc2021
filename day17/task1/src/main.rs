use std::collections::HashSet;

fn main() {
    let (xmin, xmax) = (96, 125);
    let (ymin, ymax) = (-144, -98);
    let field = Field {
        xmin,
        xmax,
        ymin,
        ymax,
    };

    let mut max_y = 0;
    let mut velocity = HashSet::new();
    for xvel in 1..=xmax {
        for yvel in -2000..2000 {
            let located = find_y(&field, 0, 0, 0, Velocity { x: xvel, y: yvel }, 0);
            if located > -1 {
                velocity.insert((xvel, yvel));
            }
            max_y = i32::max(located, max_y)
        }
    }

    println!("max y: {} counter: {}", max_y, velocity.len());
}

struct Field {
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
}
#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq)]
enum Position {
    Matched,
    NotYet,
    NotMatched,
}
impl Field {
    fn point_is_inside(&self, x: i32, y: i32) -> Position {
        if x > self.xmax || y < self.ymin {
            Position::NotMatched
        } else if x >= self.xmin && x <= self.xmax && y >= self.ymin && y <= self.ymax {
            Position::Matched
        } else {
            Position::NotYet
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_in() {
        // target area: x=20..30, y=-10..-5
        let field = Field {
            xmin: 20,
            xmax: 30,
            ymin: -10,
            ymax: -5,
        };

        assert_eq!(field.point_is_inside(0, 0), Position::NotYet);
        assert_eq!(field.point_is_inside(31, 0), Position::NotMatched);
        assert_eq!(field.point_is_inside(0, -15), Position::NotMatched);
        assert_eq!(field.point_is_inside(25, -6), Position::Matched);
        assert_eq!(field.point_is_inside(25, -11), Position::NotMatched);

        assert_eq!(find_y(&field, 0, 0, 0, Velocity { x: 9, y: 0 }, 0), 0);
    }

    #[test]
    fn test_find_y() {
        let field = Field {
            xmin: 20,
            xmax: 30,
            ymin: -10,
            ymax: -5,
        };
        assert_eq!(find_y(&field, 0, 0, 0, Velocity { x: 9, y: 0 }, 0), 0);
        assert_eq!(find_y(&field, 0, 0, 0, Velocity { x: 6, y: 9 }, 0), 45);
    }

    #[test]
    fn complete() {
        let field = Field {
            xmin: 20,
            xmax: 30,
            ymin: -10,
            ymax: -5,
        };
        let mut max_y = 0;
        let mut velocity = HashSet::new();
        for xvel in 0..=field.xmax {
            for yvel in -2000..1000 {
                let located = find_y(&field, 0, 0, 0, Velocity { x: xvel, y: yvel }, 0);
                if located > -1 {
                    velocity.insert((xvel, yvel));
                }
                max_y = i32::max(located, max_y)
            }
        }

        velocity.iter().for_each(|(k, v)| println!("{},{}", k, v));
        assert_eq!(velocity.len(), 112);
        assert_eq!(max_y, 45)
    }
}

fn find_y(field: &Field, x: i32, y: i32, curr_max_y: i32, velocity: Velocity, n: u32) -> i32 {
    match field.point_is_inside(x, y) {
        Position::NotYet => find_y(
            field,
            x + i32::max(0, velocity.x - n as i32),
            y + velocity.y - n as i32,
            i32::max(curr_max_y, y + velocity.y - n as i32),
            velocity,
            n + 1,
        ),
        Position::NotMatched => -1,
        Position::Matched => curr_max_y,
    }
}
