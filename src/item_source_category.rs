use rusqlite::Connection;

pub const ITEM_SOURCE_CATEGORIES: &[&str] = &[
    "Vendor",
    "Quest",
    "Boss Drop",
    "Rare Drop",
    "Zone Drop",
];
pub const ITEM_SOURCE_CATEGORIES_FRFR: &[&str] = &[
    "Vendeur",
    "Quête",
    "Butin de boss",
    "Butin rare",
    "Butin de zone",
];

pub fn create_item_source_category_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_source_category (
            id INTEGER PRIMARY KEY,
            label TEXT NOT NULL,
            label_frFR TEXT NOT NULL
        )",
        [],
    )?;
    for (i, (label, label_fr_fr)) in ITEM_SOURCE_CATEGORIES.iter().zip(ITEM_SOURCE_CATEGORIES_FRFR.iter()).enumerate() {
        conn.execute(
            "INSERT INTO item_source_category (id, label, label_frFR) VALUES (?1, ?2, ?3)",
            (i as i32 + 1, *label, *label_fr_fr),
        )?;
    }
    Ok(())
}
