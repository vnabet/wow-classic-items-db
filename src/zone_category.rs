
pub const ZONE_CATEGORIES: &[&str] = &[
    "Dungeon",
    "undefined",
    "Raid",
    "Arena",
    "Open World",
    "Battleground",
];

pub const ZONE_CATEGORIES_FRFR: &[&str] = &[
    "Donjon",
    "Non défini",
    "Raid",
    "Arène",
    "Monde ouvert",
    "Champ de bataille",
];

pub fn create_zone_category_table(conn: &rusqlite::Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS zone_category (
            id INTEGER PRIMARY KEY,
            label TEXT NOT NULL,
            label_frFR TEXT NOT NULL
        )",
        [],
    )?;
    for (i, (label, label_fr_fr)) in ZONE_CATEGORIES.iter().zip(ZONE_CATEGORIES_FRFR.iter()).enumerate() {
        conn.execute(
            "INSERT INTO zone_category (id, label, label_frFR) VALUES (?1, ?2, ?3)",
            rusqlite::params![i as i32 + 1, *label, *label_fr_fr],
        )?;
    }
    Ok(())
}
