pub const ZONE_TERRITORIES: &[&str] = &[
    "Contested",
    "Sanctuary",
    "Horde",
    "Alliance",
    "PvP",
];

pub const ZONE_TERRITORIES_FRFR: &[&str] = &[
    "Contesté",
    "Sanctuaire",
    "Horde",
    "Alliance",
    "JcJ",
];

pub fn create_zone_territory_table(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS zone_territory (
            id INTEGER PRIMARY KEY,
            label TEXT NOT NULL,
            label_frFR TEXT NOT NULL
        )",
        [],
    )?;
    for (i, (label, label_fr_fr)) in ZONE_TERRITORIES.iter().zip(ZONE_TERRITORIES_FRFR.iter()).enumerate() {
        conn.execute(
            "INSERT INTO zone_territory (id, label, label_frFR) VALUES (?1, ?2, ?3)",
            rusqlite::params![i as i32 + 1, *label, *label_fr_fr],
        )?;
    }
    Ok(())
}
