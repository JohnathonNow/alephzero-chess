pub fn setup(conn: &mut Connection) -> std::io::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
            id           INTEGER PRIMARY KEY,
            white_castle INTEGER NOT NULL,
            black_castle INTEGER NOT NULL,
            turn         INTEGER NOT NULL
         );
         ",
        params![],
    ).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS boards (
            id           INTEGER PRIMARY KEY,
            gameid       INTEGER NOT NULL,
            board        TEXT
         );
         ",
        params![],
    ).unwrap();
    Ok(())
}