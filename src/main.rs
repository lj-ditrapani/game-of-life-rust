/*  Author:  Lyall Jonathan Di Trapani ----------------------------------------
 *
 */
#[derive(Clone,Copy,Debug)]
struct Cell {
    live: bool,
    neighbor_count: u8,
}

impl Cell {
    fn set_state(&mut self, live: bool) {
        self.live = live;
    }

    fn set_neighbor_count(&mut self, count: u8) {
        self.neighbor_count = count;
    }
}

fn main() {
    let mut board = [[Cell { live: false, neighbor_count: 0}; 10]; 10];
    println!("{:?}", board);
    println!("\n\n{:?}", board[0]);
    board[0][0] = Cell { live: true, neighbor_count: 255 };

    println!("\n\n{:?}", board[0][0]);

    board[0][1].set_state(true);
    board[0][1].set_neighbor_count(16);
    println!("\n\n{:?}", board[0][1]);
}
