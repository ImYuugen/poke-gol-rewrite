## Pokemon's Game of life

Conway's game of life with a twist.
Every cell has a type, which is weak or strong to other types, a cell attack
every adjacent ones each tick, potentially killing it.
The place where a cell was killed remains empty for a round, then
gets replaced by whatever type was the most present around it.

#### General ideas
Not all cells are alive, a cell needs 1 turn to be populated by most present type around
, if no types are available, cell remains dead.

#### TODO
- Rendering
- Game rules
- CLI
