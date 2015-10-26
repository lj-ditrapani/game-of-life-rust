/*  Author:  Lyall Jonathan Di Trapani ----------------------------------------
 *
 */

const HEIGHT: usize = 10;
const WIDTH: usize = 10;

#[derive(Clone,Copy,Debug)]
struct Board {
    cells: [[Cell; WIDTH]; HEIGHT]
}

impl Board {
    fn new() -> Board {
        Board { cells: [[Cell::new(); WIDTH]; HEIGHT] }
    }

    fn update_neighbor_counts(&mut self) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
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
    use super::{HEIGHT, WIDTH, Cell, Board};

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
        assert_eq!(b.cells.len(), HEIGHT);
        assert_eq!(b.cells[0].len(), WIDTH);
    }

    #[test]
    fn test_set_cell() {
        let mut c = Cell::new();
        set_cell(&mut c, true, 7);
        assert_eq!(c.live, true);
        assert_eq!(c.neighbor_count, 7);
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
