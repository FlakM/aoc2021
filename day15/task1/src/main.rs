use petgraph::algo::dijkstra;
use petgraph::data::Build;
use petgraph::prelude::*;
use petgraph::Graph;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let input = include_str!("../input.txt");
    let mut parsed = parse(input);
    println!("task 1: {}", task_1(&parsed));
    task_2_enlarge(&mut parsed);
    println!(
        "task 2: {} size: {}x{}",
        task_1(&parsed),
        &parsed.len(),
        &parsed[0].len()
    );
}

fn parse(s: &str) -> Vec<Vec<u32>> {
    s.split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<u32>>>()
}
fn bump(curr: u32, by: u32) -> u32 {
    let incremented = curr + by;
    if incremented > 9 {
        incremented % 10 + 1
    } else {
        incremented % 10
    }
}

fn print(elems: &Vec<Vec<u32>>) {
    println!();
    for row in elems {
        for &curr in row {
            print!("{}", curr);
        }
        println!();
    }
}

fn task_2_enlarge(elems: &mut Vec<Vec<u32>>) {
    let x = elems.len();
    // copy right
    for row in elems.iter_mut() {
        let original_row = row.clone();
        for i in 1..5 {
            for curr in &original_row {
                row.push(bump(*curr, i));
            }
        }
    }
    // copy down
    let rows_to_be_copied: Vec<Vec<u32>> = elems[..x].iter().map(|s| s.clone()).collect();
    for i in 1..5 {
        for row in &rows_to_be_copied {
            let mut r = row.clone();
            for elem in r.iter_mut() {
                *elem = bump(*elem, i as u32);
            }
            elems.push(r)
        }
    }
}

fn task_1(elems: &Vec<Vec<u32>>) -> usize {
    let mut graph: Graph<u32, u32, Directed> = Graph::new();
    let mut map: HashMap<(isize, isize), NodeIndex> = HashMap::new();
    for (x, row) in elems.iter().enumerate() {
        let mut previous_row = None;
        for (y, &elem) in row.iter().enumerate() {
            let node = graph.add_node(elem);
            map.insert((x as isize, y as isize), node);
            if let Some(previous) = previous_row {
                graph.add_edge(previous, node, elem);
                graph.add_edge(node, previous, elems[x][y - 1]);
            }
            if let Some(&upper) = map.get(&(x as isize - 1, y as isize)) {
                graph.add_edge(upper, node, elem);
                graph.add_edge(node, upper, elems[x - 1][y]);
            }
            previous_row = Some(node);
        }
    }

    let (x, y) = (elems.len() - 1, elems.last().unwrap().len() - 1);
    println!("going to... {}x{}", x, y);
    let node_last = *map.get(&(x as isize, y as isize)).unwrap();
    let path = dijkstra(&graph, *map.get(&(0, 0)).unwrap(), None, |n| *n.weight());
    *path.get(&node_last).unwrap() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_reddit() {
        let input = "
19999
19111
11191
";
        let mut elems = parse(input);
        //task_2_enlarge(&mut elems);
        print(&elems);
        assert_eq!(task_1(&elems), 8);
    }

    #[test]
    fn test_input() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let mut elems = parse(&input);
        assert_eq!(task_1(&elems), 40);
        task_2_enlarge(&mut elems);
        assert_eq!((50, 50), (elems.len(), elems[0].len()));
        assert_eq!(&elems[0][10..20], [2, 2, 7, 4, 8, 6, 2, 8, 5, 3]);
        assert_eq!(elems[10][0], 2);
        assert_eq!(elems[10][2], 7);
        assert_eq!(elems[20][0], 3);
        assert_eq!(task_1(&elems), 315);
    }

    #[test]
    fn test_enlarging() {
        let input = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";
        let mut elems = parse(&input);
        task_2_enlarge(&mut elems);
        let input_large = include_str!("../input_test.txt");
        let elems_large = parse(&input_large);
        for x in 0..50 {
            for y in 0..50 {
                println!(
                    "checking {}x{} taken from {} for iteration {}",
                    x,
                    y,
                    elems[x][y % 10],
                    y % 10
                );
                assert_eq!(elems[x][y], elems_large[x][y]);
            }
        }
    }
}
