pub fn setup(conn: &mut Connection) -> std::io::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS players (
            name        TEXT PRIMARY KEY,
            game_id     INTEGER,
            correct     INTEGER,
            drawer      INTEGER,
            score       INTEGER
              )",
        params![],
    )
    .unwrap();
    Ok(())
}