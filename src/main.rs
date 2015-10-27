/*  Author:  Lyall Jonathan Di Trapani ----------------------------------------
 *
 */
extern crate term;

const SIZE: usize = 20;
const SIZE_I8: i8 = SIZE as i8;

type Offset = (i8, i8);

const NEIGHBOR_OFFSETS: [Offset; 8] = [
    (-1, -1), (-1, 0), (-1, 1),
    ( 0, -1),          ( 0, 1),
    ( 1, -1), ( 1, 0), ( 1, 1)
];

#[derive(Clone,Copy,Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn get_neighbor_coords(&self) -> NeighborCoords {
        let mut nc = [Point { x: 0, y: 0 }; 8];
        for (p, &(dx, dy)) in nc.iter_mut().zip(NEIGHBOR_OFFSETS.iter()) {
            p.x = Point::compute_point(self.x, dx);
            p.y = Point::compute_point(self.y, dy);
        }
        nc
    }

    fn compute_point(x: usize, dx: i8) -> usize {
        let s: i8 = SIZE as i8;
        let v: i8 = ((x as i8) + dx) % s;
        match v {
            -1 => SIZE - 1,
            SIZE_I8 => 0,
            _ => v as usize,
        }
    }
}

type NeighborCoords = [Point; 8];

#[derive(Clone,Copy,Debug)]
struct Board {
    cells: [[Cell; SIZE]; SIZE]
}

#[derive(Clone,Copy,Debug)]
struct Cell {
    live: bool,
    neighbor_count: u8,
}

impl Cell {
    fn new() -> Cell {
        Cell { live: false, neighbor_count: 0 }
    }

    fn on(&mut self) {
        self.live = true;
    }

    fn off(&mut self) {
        self.live = false;
    }

    fn update_life_state(&mut self) {
        self.live = self.next_life_state();
    }

    fn next_life_state(&self) -> bool {
        match (self.neighbor_count, self.live) {
            (3, _) => true,
            (2, true) => true,
            _ => false,
        }
    }

    fn print(&self, t: &mut Box<term::StdoutTerminal>) {
        match self.live {
            true => {
                t.fg(term::color::YELLOW).unwrap();
                t.bg(term::color::CYAN).unwrap();
                write!(t, "<>").unwrap();
            },
            false => {
                t.fg(term::color::BRIGHT_BLACK).unwrap();
                t.bg(term::color::BLUE).unwrap();
                write!(t, "[]").unwrap();
            },
        };
    }
}

impl Board {
    fn new() -> Board {
        Board { cells: [[Cell::new(); SIZE]; SIZE] }
    }

    fn update_life_states(&mut self) {
        for row in &mut self.cells {
            for cell in row.iter_mut() {
                cell.update_life_state();
            }
        }
    }

    fn update_neighbor_counts(&mut self) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                self.update_neighbor_count(i, j);
            }
        }
    }

    fn update_neighbor_count(&mut self, i: usize, j: usize) {
        self.cells[i][j].neighbor_count = self.get_neighbor_count(i, j);
    }

    fn get_neighbor_count(&self, i: usize, j: usize) -> u8 {
        let p = Point { x: i, y: j };
        p.get_neighbor_coords().iter()
            .map(|&Point { x, y }| { self.cells[x][y].live })
            .filter(|&z| z == true)
            .collect::<Vec<bool>>()
            .len() as u8
    }

    fn step(&mut self) {
        self.update_neighbor_counts();
        self.update_life_states();
    }

    fn print(&self) {
        let mut t = term::stdout().unwrap();
        t.bg(term::color::BLACK).unwrap();
        println!("");
        for row in &self.cells {
            for cell in row.iter() {
                cell.print(&mut t);
            }
            t.bg(term::color::BLACK).unwrap();
            println!("");
        }
    }
}

fn main() {
    let mut board = Board::new();
    {
        board.cells[0][0].on();
        board.cells[0][1].on();
        board.cells[0][2].on();
    }
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        if input.trim() == "quit" {
            break;
        }
        board.print();
        board.step();
    }
}

#[cfg(test)]
mod tests {
    use super::{SIZE, Cell, Board, Point};

    fn set_cell(cell: &mut Cell, live: bool, count: u8) {
        cell.live = live;
        cell.neighbor_count = count;
    }

    #[test]
    fn new_cell() {
        let c = Cell::new();
        assert_eq!(c.live, false);
        assert_eq!(c.neighbor_count, 0);
    }

    #[test]
    fn cell_on() {
        let mut c = Cell::new();
        c.on();
        assert_eq!(c.live, true);
    }

    #[test]
    fn cell_off() {
        let mut c = Cell::new();
        c.on();
        assert_eq!(c.live, true);
        c.off();
        assert_eq!(c.live, false);
    }

    #[test]
    fn cell_next_life_state() {
        let mut c = Cell::new();
        let tests = [
            (0, true, false),
            (1, false, false),
            (2, false, false),
            (2, true, true),
            (3, false, true),
            (3, true, true),
            (4, true, false),
            (4, false, false),
            (5, false, false),
        ];
        for &(count, state, next_state) in tests.iter() {
            c.neighbor_count = count;
            c.live = state;
            assert_eq!(c.next_life_state(), next_state);
        }
    }

    #[test]
    fn cell_update_life_state() {
        let mut c = Cell { neighbor_count: 3, live: false };
        c.update_life_state();
        assert_eq!(c.live, true);
    }

    #[test]
    fn new_board() {
        let b = Board::new();
        assert_eq!(b.cells.len(), SIZE);
        assert_eq!(b.cells[0].len(), SIZE);
    }

    #[test]
    fn test_set_cell() {
        let mut c = Cell::new();
        set_cell(&mut c, true, 7);
        assert_eq!(c.live, true);
        assert_eq!(c.neighbor_count, 7);
    }

    #[test]
    fn point() {
        let p = Point { x: 5, y: 3 };
        assert_eq!(p.x, 5);
    }

    #[test]
    fn point_compute_point() {
        let tests = [
            (5, 0, 5),
            (1, -1, 0),
            (SIZE - 2, 1, SIZE - 1),
            (0, -1, SIZE - 1),
            (SIZE - 1, 1, 0),
        ];
        for &(x, dx, v) in tests.iter() {
            assert_eq!(Point::compute_point(x, dx), v);
        }
    }

    #[test]
    fn get_neighbor_coords11() {
        let p = Point { x: 1, y: 1 };
        let nc = p.get_neighbor_coords();
        let tests = [
            (0, 0, 0),
            (1, 0, 1),
            (2, 0, 2),
            (3, 1, 0),
        ];
        for &(i, x, y) in tests.iter() {
            assert_eq!(nc[i].x, x);
            assert_eq!(nc[i].y, y);
        }
    }

    #[test]
    fn get_neighbor_coords00() {
        let p = Point { x: 0, y: 0 };
        let nc = p.get_neighbor_coords();
        let tests = [
            (0, SIZE - 1, SIZE - 1),
            (1, SIZE - 1, 0),
            (2, SIZE - 1, 1),
            (3, 0, SIZE - 1),
            (4, 0, 1),
        ];
        for &(i, x, y) in tests.iter() {
            assert_eq!(nc[i].x, x);
            assert_eq!(nc[i].y, y);
        }
    }

    #[test]
    fn get_neighbor_count() {
        let mut board = Board::new();
        let mut count = board.get_neighbor_count(0, 0);
        assert_eq!(count, 0);
        board.cells[0][1].on();
        board.cells[SIZE - 1][SIZE - 1].on();
        count = board.get_neighbor_count(0, 0);
        assert_eq!(count, 2);
    }

    #[test]
    fn update_neighbor_count() {
        let mut board = Board::new();
        board.update_neighbor_count(0, 0);
        assert_eq!(board.cells[0][0].neighbor_count, 0);
        board.cells[0][1].on();
        board.cells[SIZE - 1][SIZE - 1].on();
        board.update_neighbor_count(0, 0);
        assert_eq!(board.cells[0][0].neighbor_count, 2);
    }

    #[test]
    fn update_neighbor_counts() {
        let mut board = Board::new();
        board.update_neighbor_counts();
        assert_eq!(board.cells[0][0].neighbor_count, 0);
        board.cells[0][1].on();
        board.update_neighbor_counts();
        assert_eq!(board.cells[0][0].neighbor_count, 1);
    }

    #[test]
    fn update_life_states() {
        let mut board = Board::new();
        board.cells[0][0].on();
        board.cells[0][SIZE - 1].on();
        board.cells[2][0].on();
        board.update_neighbor_counts();
        board.update_life_states();
        assert_eq!(board.cells[1][0].live, true);
        assert_eq!(board.cells[1][SIZE - 1].live, true);
        assert_eq!(board.cells[1][1].live, false);
    }
}
