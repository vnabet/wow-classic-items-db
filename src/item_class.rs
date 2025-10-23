use rusqlite::Connection;

pub const ITEM_CLASSES: &[&str] = &[
    "Armor",
    "Weapon",
    "Consumable",
    "Quest",
    "Miscellaneous",
    "Trade Goods",
    "Recipe",
    "Container",
    "Gem",
    "Quiver",
    "Projectile",
    "Key",
    "Reagent",
];

pub const ITEM_CLASSES_FRFR: &[&str] = &[
    "Armure",
    "Arme",
    "Consommable",
    "Quête",
    "Divers",
    "Composant commercial",
    "Recette",
    "Conteneur",
    "Gemme",
    "Carquois",
    "Projectile",
    "Clé",
    "Composant",
];

pub fn create_item_class_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_class (
            id INTEGER PRIMARY KEY,
            label TEXT NOT NULL,
            label_frFR TEXT NOT NULL
        )",
        [],
    )?;
    for (i, (label, label_fr_fr)) in ITEM_CLASSES.iter().zip(ITEM_CLASSES_FRFR.iter()).enumerate() {
        conn.execute(
            "INSERT INTO item_class (id, label, label_frFR) VALUES (?1, ?2, ?3)",
            (i as i32 + 1, *label, *label_fr_fr),
        )?;
    }
    Ok(())
}
