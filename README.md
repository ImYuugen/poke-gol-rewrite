## Pokemon's Game of life

Conway's game of life with a twist.
Every cell has a type, which is weak or strong to other types, a cell attacks
every adjacent ones each tick, potentially killing it.
The place where a cell was killed remains empty for a round, then
gets replaced by whatever type was the most present around it.

#### General ideas
- Not all cells are alive, a cell needs 1 turn to be populated by most present type around
, if no types are available, cell remains dead.
- Render polls from a queue, every time a cell is updated, it is pushed into the queue to be
redrawn, this way rendering does not need to way for the game to finish ticking.

#### TODO
- Rendering
- Separate rendering thread
- Game rules
