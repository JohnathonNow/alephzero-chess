pub fn setup(conn: &mut Connection) -> std::io::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
            id           INTEGER PRIMARY KEY,
            white_castle INTEGER NOT NULL,
            black_castle INTEGER NOT NULL,
            turn         INTEGER NOT NULL
         );
         CREATE TABLE IF NOT EXISTS pieces (
            id          INTEGER PRIMARY KEY,
            game_id     INTEGER NOT NULL,
            type        TEXT NOT NULL,
            rank        TEXT NOT NULL,
            file        TEXT NOT NULL,
            start_rank  TEXT NOT NULL,
            start_file  TEXT NOT NULL,
            color       TEXT NOT NULL
         );
         CREATE TABLE IF NOT EXISTS moves (
             id         INTEGER PRIMARY KEY,
             game_id    INTEGER NOT NULL,
             from_rank  TEXT NOT NULL,
             to_rank    TEXT NOT NULL,
             from_file  TEXT NOT NULL,
             to_file    TEXT NOT NULL,
             promotion  TEXT
         );
         CREATE TABLE IF NOT EXISTS pawn_ranks (
             id         INTEGER PRIMARY KEY,
             game_id    INTEGER NOT NULL,
             rank       TEXT NOT NULL
         );
         ",
        params![],
    )
    .unwrap();
    Ok(())
}