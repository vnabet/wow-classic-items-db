mod item_class;
mod zone_category;
mod zone_territory;
mod item_subclass;
mod item_quality;
mod item_slot;
mod faction;
mod item_source_category;
mod item_tooltip_format;
mod item;
mod data_items;
mod data_zones;
mod zone;
use std::env;
use rusqlite::{Connection, Result};

use std::fs;


fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <db_name>", args[0]);
        std::process::exit(1);
    }
    let db_name = &args[1];
    // Créer le dossier db s'il n'existe pas
    let db_dir = "db";
    if !std::path::Path::new(db_dir).exists() {
        fs::create_dir_all(db_dir).expect("Erreur lors de la création du dossier db");
    }
    let db_file = format!("{}/{}.sqlite", db_dir, db_name);
    // Supprimer le fichier s'il existe déjà
    if std::path::Path::new(&db_file).exists() {
        fs::remove_file(&db_file).expect("Erreur lors de la suppression de l'ancienne base de données");
    }
    let conn = Connection::open(&db_file)?;

    zone_category::create_zone_category_table(&conn)?;
    zone_territory::create_zone_territory_table(&conn)?;
    zone::create_zone_table(&conn)?;
    data_zones::import_zones(&conn, "./data/zones.json").expect("Erreur lors de l'import des zones");
    item_class::create_item_class_table(&conn)?;
    item_subclass::create_item_subclass_table(&conn)?;
    item_quality::create_item_quality_table(&conn)?;
    item_slot::create_item_slot_table(&conn)?;
    faction::create_faction_table(&conn)?;
    item_source_category::create_item_source_category_table(&conn)?;
    item_tooltip_format::create_item_tooltip_format_table(&conn)?;
    // Création des tables avancées (item, tooltips, sources, etc.)
    item::create_item_quest_table(&conn)?;
    item::create_item_source_table(&conn)?;
    item::create_item_source_quest_table(&conn)?;
    item::create_item_tooltip_table(&conn)?;
    item::create_item_tooltip_link_table(&conn)?;
    item::create_item_table(&conn)?;
    // Importer les données depuis data.json
    data_items::import_data(&conn, "./data/data-test.json").expect("Erreur lors de l'import des données");
    println!("Base de données SQLite créée, tables et données de référence ajoutées : {}", db_file);
    Ok(())
}
