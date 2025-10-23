pub fn create_zone_table(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS zone (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            category_id INTEGER NOT NULL,
            level_min INTEGER,
            level_max INTEGER,
            territory_id INTEGER NOT NULL,
            FOREIGN KEY (category_id) REFERENCES zone_category(id),
            FOREIGN KEY (territory_id) REFERENCES zone_territory(id)
        )",
        [],
    )?;
    Ok(())
}
