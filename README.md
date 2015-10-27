Conway's Game of Life on 2-D toroidal grid in rust

My first rust program.  I would really appreciate feedback on idiomatic rust.

You can change the const declaration to adjust the size of the grid.

Author: Lyall Jonathan Di Trapani

TODO
----

- Set cells method to easily turn on a list of cells to initialize the board.
- Make some demo programs selectable from list.
- Consider making the cells 5 X 2 block of chars instead of 2 X 1.
  It would be a 16 x 16 cell grid using 80 X 32 chars.
```
|   |/o.o\|   |/o.o\
|___|\---/|___|\---/
/o.o\|   |/o.o\|   |
\---/|___|\---/|___|
|   |/o.o\|   |/o.o\
|___|\---/|___|\---/
|   |/o.o\|   |/o.o\
|___|\---/|___|\---/
```
- Make a crate?
