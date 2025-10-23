use rusqlite::Connection;

pub const ITEM_QUALITIES: &[&str] = &[
    "Poor",
    "Uncommon",
    "Common",
    "Epic",
    "Rare",
    "Legendary",
    "Artifact",
];

pub const ITEM_QUALITIES_FRFR: &[&str] = &[
    "Médiocre",
    "Inhabituel",
    "Commun",
    "Épique",
    "Rare",
    "Légendaire",
    "Artefact",
];

pub fn create_item_quality_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_quality (
            id INTEGER PRIMARY KEY,
            label TEXT NOT NULL,
            label_frFR TEXT NOT NULL
        )",
        [],
    )?;
    for (i, (label, label_fr_fr)) in ITEM_QUALITIES.iter().zip(ITEM_QUALITIES_FRFR.iter()).enumerate() {
        conn.execute(
            "INSERT INTO item_quality (id, label, label_frFR) VALUES (?1, ?2, ?3)",
            (i as i32 + 1, *label, *label_fr_fr),
        )?;
    }
    Ok(())
}
