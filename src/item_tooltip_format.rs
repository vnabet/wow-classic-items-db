use rusqlite::Connection;

pub const ITEM_TOOLTIP_FORMATS: &[&str] = &[
    "Common",
    "Misc",
    "alignRight",
    "Poor",
    "Uncommon",
    "Rare",
    "indent",
    "Epic",
    "Legendary",
];

pub fn create_item_tooltip_format_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_tooltip_format (
            id INTEGER PRIMARY KEY,
            label TEXT NOT NULL
        )",
        [],
    )?;
    for (i, label) in ITEM_TOOLTIP_FORMATS.iter().enumerate() {
        conn.execute(
            "INSERT INTO item_tooltip_format (id, label) VALUES (?1, ?2)",
            (i as i32 + 1, *label),
        )?;
    }
    Ok(())
}