/// this is taken from http://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm
/// if there is easier way i will harm myself
const MUTATIONS: &'static [[[i32; 3]; 3]; 24] = &[
    [[1, 0, 0], [0, 1, 0], [0, 0, 1]],
    [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
    [[1, 0, 0], [0, -1, 0], [0, 0, -1]],
    [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
    // next
    [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
    [[0, 0, 1], [1, 0, 0], [0, 1, 0]],
    [[0, 1, 0], [1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
    // next
    [[-1, 0, 0], [0, -1, 0], [0, 0, 1]],
    [[-1, 0, 0], [0, 0, -1], [0, -1, 0]],
    [[-1, 0, 0], [0, 1, 0], [0, 0, -1]],
    [[-1, 0, 0], [0, 0, 1], [0, 1, 0]],
    // next
    [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
    [[0, 0, 1], [-1, 0, 0], [0, -1, 0]],
    [[0, -1, 0], [-1, 0, 0], [0, 0, -1]],
    [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
    // next
    [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
    [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
    [[0, 0, 1], [0, -1, 0], [1, 0, 0]],
    [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
    // next
    [[0, 0, -1], [0, -1, 0], [-1, 0, 0]],
    [[0, -1, 0], [0, 0, 1], [-1, 0, 0]],
    [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
    [[0, 1, 0], [0, 0, -1], [-1, 0, 0]],
];

/// Prepare all permutations of input taking into considaration
/// that we don't know exact placement of scanner
fn permutations(input: Vec<[i32; 3]>) -> Vec<Vec<[i32; 3]>> {
    MUTATIONS
        .iter()
        .map(|trans| {
            input
                .iter()
                .map(|cord| {
                    let (x, y, z) = (cord[0], cord[1], cord[2]);
                    trans
                        .iter()
                        .map(|&row| row[0] * x + row[1] * y + row[2] * z)
                        .collect::<Vec<i32>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<[i32; 3]>>()
        })
        .collect::<Vec<Vec<[i32; 3]>>>()
}

struct FixedScanner {
    location: [i32; 3],
    beams: Vec<[i32; 3]>,
}

/// this method prepares all permutations of current scanner and checks if it contains
/// at least 12 common points from already located scanners.
/// It will return FixedScanner relative to scanner #0 (inputs first)
fn locate(scanner: Vec<[i32; 3]>, already_known: &Vec<FixedScanner>) -> Option<FixedScanner> {
    let mutations = permutations(scanner);
    for alternative in mutations {
        for known_scanner in already_known {
            for mutation in &alternative {
                for known_beam in known_scanner.beams.iter() {}
            }
        }
    }

    unimplemented!()
}

fn parse(s: &str) -> Vec<[i32; 3]> {
    s.split('\n')
        .filter(|l| !l.is_empty())
        .map(|s| {
            s.split(',')
                .map(|n| n.parse().unwrap())
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap()
        })
        .collect()
}
fn main() {
    let input = "686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390";
    let input = parse(input);
    let permutations = permutations(input);
    for p in permutations {
        println!("=====");
        for e in p {
            println!("{},{},{}", e[0], e[1], e[2]);
        }
    }
    println!("Hello, world!");
}
