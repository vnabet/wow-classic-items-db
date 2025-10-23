pub const ITEM_SLOTS_FRFR: &[&str] = &[
    "Chemise",
    "Deux mains",
    "Torse",
    "Jambes",
    "Pieds",
    "Main principale",
    "Mains",
    "Non-équipable",
    "Une main",
    "Poignets",
    "Bijou",
    "Sac",
    "Taille",
    "Doigt",
    "Main gauche",
    "Tenu(e) en main gauche",
    "Dos",
    "Relique",
    "Tête",
    "Cou",
    "Épaule",
    "À distance",
    "Munitions",
    "Tabard",
    "Arme de jet",
];
use rusqlite::Connection;

pub const ITEM_SLOTS: &[&str] = &[
    "Shirt",
    "Two-Hand",
    "Chest",
    "Legs",
    "Feet",
    "Main Hand",
    "Hands",
    "Non-equippable",
    "One-Hand",
    "Wrist",
    "Trinket",
    "Bag",
    "Waist",
    "Finger",
    "Off Hand",
    "Held In Off-hand",
    "Back",
    "Relic",
    "Head",
    "Neck",
    "Shoulder",
    "Ranged",
    "Ammo",
    "Tabard",
    "Thrown",
];

pub fn create_item_slot_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_slot (
            id INTEGER PRIMARY KEY,
            label TEXT NOT NULL,
            label_frFR TEXT NOT NULL
        )",
        [],
    )?;
    for (i, (label, label_fr_fr)) in ITEM_SLOTS.iter().zip(ITEM_SLOTS_FRFR.iter()).enumerate() {
        conn.execute(
            "INSERT INTO item_slot (id, label, label_frFR) VALUES (?1, ?2, ?3)",
            (i as i32 + 1, *label, *label_fr_fr),
        )?;
    }
    Ok(())
}
