const INPUT: &str = include_str!("../inputs/03.txt");
enum Square {
    Open,
    Tree,
}
struct Map(Vec<Vec<Square>>);
struct Slope { right: usize, down: usize, }
struct Toboggan { x: usize, y: usize, trees_found: usize, slope: Slope }

impl Toboggan { 
    fn with_slope(right: usize, down: usize) -> Self {
        Toboggan { x: 0, y: 0, trees_found: 0, slope: Slope { right, down } }
    }

    fn traverse(&mut self, map: &Map) {
        self.x += self.slope.right;
        self.y += self.slope.down;
        self.trees_found += match map.lookup(self.x, self.y) { 
            Square::Tree => 1,
            _ => 0,
        };
    }
}

impl Map {
    fn from(s: &str) -> Self {
        Map( s.lines().
            map(|line| {
                line.chars().map(|c| 
                    match c {
                        '.' => Square::Open,
                        '#' => Square::Tree,
                        _ => panic!("Unrecognised input character: {}", c),
                    }
                ).collect()       
            }).collect()
        )
    }

    fn lookup(&self, x: usize, y: usize) -> &Square {
        &self.0[y][x%self.width()]        
    }

    fn len(&self) -> usize {
        self.0.len()
    }

    fn width(&self) -> usize {
        self.0[0].len()
    }
}

fn main() {
    let map = Map::from(INPUT);

    let slopes = [(1,1), (3,1), (5, 1), (7, 1), (1, 2)]; // (right, down)

    let mut toboggans: Vec<Toboggan> = slopes.iter().map(|(right, down)| Toboggan::with_slope(*right, *down)).collect();

    toboggans.iter_mut().for_each(|t| {
        (1..map.len()).step_by(t.slope.down).for_each(|_| t.traverse(&map));
    });

    println!("Day 3 Part 1: {}", toboggans[1].trees_found);
    println!("Day 3 Part 2: {}", toboggans.iter().map(|t| t.trees_found).product::<usize>());
}