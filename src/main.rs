/*  Author:  Lyall Jonathan Di Trapani ----------------------------------------
 *
 */

type Board = [[Cell; 10]; 10];

#[derive(Clone,Copy,Debug)]
struct Cell {
    live: bool,
    neighbor_count: u8,
}

impl Cell {
    fn update_neighbor_count(&self, board: Board) {
    }

    fn new() -> Cell {
        Cell { live: false, neighbor_count: 0 }
    }

    fn set(&mut self, live: bool, count: u8) {
        self.live = live;
        self.neighbor_count = count;
    }
}

fn main() {
    let mut board = [[Cell::new(); 10]; 10];
    println!("{:?}", board);
    println!("\n\n{:?}", board[0]);
    board[0][0] = Cell { live: true, neighbor_count: 255 };

    println!("\n\n{:?}", board[0][0]);

    board[0][1].live = true;
    board[0][1].neighbor_count = 14;
    println!("\n\n{:?}", board[0][1]);
}

#[cfg(test)]
mod tests {
    use super::Cell;

    #[test]
    fn new_cell() {
        let mut c = Cell::new();
        assert_eq!(c.live, false);
        assert_eq!(c.neighbor_count, 0);
    }

    #[test]
    fn set_cell() {
        let mut c = Cell::new();
        c.set(true, 7);
        assert_eq!(c.live, true);
        assert_eq!(c.neighbor_count, 7);
    }

    #[test]
    fn get_neighbor_count() {
        let mut board = [[Cell::new(); 10]; 10];
        let mut c = board[0][0];
        c.update_neighbor_count(board);
        assert_eq!(c.neighbor_count, 0);
        board[0][1].set(true, 0);
        c.update_neighbor_count(board);
        assert_eq!(c.neighbor_count, 1);
    }
}
