# alephzero-chess  
An online infinite chess game.

## The Game  
The game is played with a standard chess board, except it extends infinitely in both directions.
Additionally, the knight's movement is slightly reinterpretted to help keep it on par with the bishop
(which would be able to move infinitely farther than the knight otherwise). The pieces are placed
where they would be normally relative to each other.

## Dev Stuff  
The project is split into two parts, the Rust **backend** and JS/HTML/CSS **frontend**.

### Backend  
Game state is stored in a SQLite database.
Each game gets a row in the `games` table, containing information about the players, current turn, etc.
Each piece for each game also gets a row in the `pieces` table, containing information about what
the piece is, its coordinates, if it was captured, etc.
