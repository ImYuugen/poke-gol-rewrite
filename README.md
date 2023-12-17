## Pokemon's Game of life

Conway's game of life with a twist.
Every cell has a type, which is weak or strong to other types, a cell attacks
every adjacent ones each tick, potentially killing it.

#### General ideas
- Render polls a queue, every time a cell is updated, it is pushed into the queue to be
redrawn, this way rendering does not need to way for the game to finish ticking.

#### TODO
- Rendering
- Separate rendering thread
- Torus shaped map
- Cell mutation (as in stat mutation)
