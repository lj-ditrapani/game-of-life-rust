/*  Author:  Lyall Jonathan Di Trapani ----------------------------------------
 *
 */

const SIZE: usize = 10;
const SIZEi8: i8 = SIZE as i8;

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
    fn get_neighbor_coords(&self, nc: &mut NeighborCoords) {
        for (p, &(dx, dy)) in nc.iter_mut().zip(NEIGHBOR_OFFSETS.iter()) {
            p.x = Point::compute_point(self.x, dx);
            p.y = Point::compute_point(self.y, dy);
        }
    }

    fn compute_point(x: usize, dx: i8) -> usize {
        let s: i8 = SIZE as i8;
        let mut v: i8 = ((x as i8) + dx) % s;
        match v {
            -1 => SIZE - 1,
            SIZEi8 => 0,
            _ => v as usize,
        }
    }
}

type NeighborCoords = [Point; 8];

fn new_neighbor_coords() -> NeighborCoords {
    [Point { x: 0, y: 0 }; 8]
}

#[derive(Clone,Copy,Debug)]
struct Board {
    cells: [[Cell; SIZE]; SIZE]
}

impl Board {
    fn new() -> Board {
        Board { cells: [[Cell::new(); SIZE]; SIZE] }
    }

    fn update_neighbor_counts(&mut self) {
        for i in 0..SIZE {
            for j in 0..SIZE {
                self.update_neighbor_count(i, j);
            }
        }
    }

    fn update_neighbor_count(&mut self, i: usize, j: usize) {
    }
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
}

fn main() {
    let mut board = Board::new();
    {
        let mut c: &mut Cell = &mut board.cells[0][1];
        c.on();
        c.neighbor_count = 14;
    }
    board.update_neighbor_counts();
    println!("\n\n{:?}", board.cells[0][1]);
}

#[cfg(test)]
mod tests {
    use super::{SIZE, Cell, Board, Point, NeighborCoords, new_neighbor_coords, NEIGHBOR_OFFSETS};

    fn set_cell(cell: &mut Cell, live: bool, count: u8) {
        cell.live = live;
        cell.neighbor_count = count;
    }

    #[test]
    fn new_cell() {
        let mut c = Cell::new();
        assert_eq!(c.live, false);
        assert_eq!(c.neighbor_count, 0);
    }

    #[test]
    fn Cell_on() {
        let mut c = Cell::new();
        c.on();
        assert_eq!(c.live, true);
    }

    #[test]
    fn Cell_off() {
        let mut c = Cell::new();
        c.on();
        assert_eq!(c.live, true);
        c.off();
        assert_eq!(c.live, false);
    }

    #[test]
    fn new_board() {
        let mut b = Board::new();
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
        assert_eq!(Point::compute_point(5, 0), 5);
        assert_eq!(Point::compute_point(1, -1), 0);
        assert_eq!(Point::compute_point(8, 1), 9);
        assert_eq!(Point::compute_point(0, -1), 9);
        assert_eq!(Point::compute_point(9, 1), 0);
    }

    #[test]
    fn test_new_neighbor_coords() {
        let nc = new_neighbor_coords();
        assert_eq!(nc.len(), 8);
        assert_eq!(nc[0].x, 0);
        assert_eq!(nc[0].y, 0);
        assert_eq!(nc[7].x, 0);
        assert_eq!(nc[7].y, 0);
    }

    #[test]
    fn get_neighbor_coords() {
        let mut nc = new_neighbor_coords();
        let p = Point { x: 1, y: 1 };
        p.get_neighbor_coords(&mut nc);
        assert_eq!(nc[0].x, 0);
        assert_eq!(nc[0].y, 0);
        assert_eq!(nc[1].x, 0);
        assert_eq!(nc[1].y, 1);
        assert_eq!(nc[2].x, 0);
        assert_eq!(nc[2].y, 2);
        assert_eq!(nc[3].x, 1);
        assert_eq!(nc[3].y, 0);
    }

    // #[test]
    fn update_neighbor_count() {
        let mut board = Board::new();
        let c = board.cells[0][0];
        board.update_neighbor_count(0, 0);
        assert_eq!(c.neighbor_count, 0);
        board.cells[0][1].on();
        board.update_neighbor_count(0, 0);
        assert_eq!(c.neighbor_count, 1);
    }

    // #[test]
    fn update_neighbor_counts() {
        let mut board = Board::new();
        let c = board.cells[0][0];
        board.update_neighbor_counts();
        assert_eq!(c.neighbor_count, 0);
        board.cells[0][1].on();
        board.update_neighbor_counts();
        assert_eq!(c.neighbor_count, 1);
    }
}
