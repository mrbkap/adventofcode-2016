use std::collections::{ HashMap, VecDeque };

const FAVORITE_NUM : usize = 1364;
const TARGET : Point = Point { y: 39, x: 31 };

#[derive(PartialEq, Eq, Hash, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x: x, y: y }
    }
    fn snew(x: isize, y: isize) -> Point {
        Point::new(x as usize, y as usize)
    }
}

struct Direction {
    x: isize,
    y: isize,
}

impl Point {
    fn is_wall(&self) -> bool {
        let x = self.x;
        let y = self.y;

        let step1 = x * x + 3 * x + 2 * x * y + y + y * y;
        let step2 = step1 + FAVORITE_NUM;
        let step3 = step2.count_ones();
        return (step3 & 1) == 1;
    }
}

struct Board {
    visited: HashMap<Point, (Direction, usize)>,
}

impl Board {
    fn new() -> Board {
        Board { visited: HashMap::new() }
    }

    fn bfs(&mut self) -> usize {
        let mut todo: VecDeque<(Point, usize)> = VecDeque::new();
        todo.push_back((Point::new(1, 1), 0));
        while !todo.is_empty() {
            let (cur_pt, cur_move) = todo.pop_front().unwrap();
            if cur_pt == TARGET {
                return cur_move;
            }

            self.push_succs(cur_pt, cur_move, &mut todo);
        }

        assert!(false);
        0
    }

    fn push_succs(&mut self, pt: Point, mv: usize, todo: &mut VecDeque<(Point, usize)>) {
        fn try_dir(visited: &mut HashMap<Point, (Direction, usize)>, pt: &Point,
                   dir: Direction, mv: usize) -> Option<Point> {
            if dir.x < 0 && pt.x == 0 || dir.y < 0 && pt.y == 0 {
                return None;
            }

            let next = Point::snew((pt.x as isize) + dir.x, (pt.y as isize) + dir.y);
            if next.is_wall() || visited.contains_key(&next) {
                return None;
            }
            visited.insert(next.clone(), (dir, mv));

            Some(next)
        }

        if let Some(pt) = try_dir(&mut self.visited, &pt, Direction { x: 1, y: 0 }, mv + 1) {
            todo.push_back((pt, mv + 1));
        }
        if let Some(pt) = try_dir(&mut self.visited, &pt, Direction { x: 0, y: 1 }, mv + 1) {
            todo.push_back((pt, mv + 1));
        }
        if let Some(pt) = try_dir(&mut self.visited, &pt, Direction { x: -1, y: 0 }, mv + 1) {
            todo.push_back((pt, mv + 1));
        }
        if let Some(pt) = try_dir(&mut self.visited, &pt, Direction { x: 0, y: -1 }, mv + 1) {
            todo.push_back((pt, mv + 1));
        }
    }

    fn print(&self) {
        print!("  ");
        for i in 0..10 {
            print!("{}", i);
        }
        println!("");
        for i in 0..10 {
            print!("{} ", i);
            for j in 0..10 {
                if (Point { x: j, y: i }).is_wall() {
                    print!("#");
                } else {
                    if self.visited.contains_key(&Point { x: j, y: i }) {
                        print!("o");
                    } else {
                        print!(".");
                    }
                }
            }
            println!("");
        }
    }
}

fn main() {
    let mut board = Board::new();
    println!("{}", board.bfs());
    board.print();
}
