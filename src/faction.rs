use rusqlite::Connection;

pub const FACTIONS: &[&str] = &[
    "Alliance",
    "Horde",
    "Both",
];
pub const FACTIONS_FR: &[&str] = &[
    "Alliance",
    "Horde",
    "Les deux",
];

pub fn create_faction_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS faction (
            id INTEGER PRIMARY KEY,
            label TEXT NOT NULL,
            label_frFR TEXT NOT NULL
        )",
        [],
    )?;
    for (i, (label, label_fr_fr)) in FACTIONS.iter().zip(FACTIONS_FR.iter()).enumerate() {
        conn.execute(
            "INSERT INTO faction (id, label, label_frFR) VALUES (?1, ?2, ?3)",
            (i as i32 + 1, *label, *label_fr_fr),
        )?;
    }
    Ok(())
}
