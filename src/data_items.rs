#![allow(dead_code)]
use rusqlite::Connection;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Item {
    #[serde(rename = "itemId")]
    pub item_id: i32,
    pub name: String,
    pub icon: Option<String>,
    pub class: String,
    pub subclass: Option<String>,
    #[serde(rename = "sellPrice")]
    pub sell_price: Option<i32>,
    pub quality: String,
    #[serde(rename = "itemLevel")]
    pub item_level: Option<i32>,
    #[serde(rename = "requiredLevel")]
    pub required_level: Option<i32>,
    pub slot: String,
    pub tooltip: Option<Vec<ItemTooltip>>,
    #[serde(rename = "itemLink")]
    pub item_link: Option<String>,
    #[serde(rename = "vendorPrice")]
    pub vendor_price: Option<i32>,
    pub source: Option<ItemSource>,
    #[serde(rename = "uniqueName")]
    pub unique_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ItemTooltip {
    pub label: String,
    pub format: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ItemSource {
    pub category: String,
    pub name: Option<String>,
    pub faction: Option<String>,
    pub cost: Option<i32>,
    pub quests: Option<Vec<ItemQuest>>,
    pub zone: Option<i32>,
    #[serde(rename = "dropChance")]
    pub drop_chance: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct ItemQuest {
    #[serde(rename = "questId")]
    pub quest_id: i32,
    pub name: String,
    pub faction: Option<String>,
}

pub fn import_data(conn: &Connection, json_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(json_path)?;
    let items: Vec<Item> = serde_json::from_str(&data)?;

    use std::io::Write;
    let total = items.len();
    let bar_width = 40;
    use std::time::Instant;
    let start = Instant::now();
    for (i, item) in items.iter().enumerate() {
        // Récupérer les ids des tables de référence
        let class_id = get_id(conn, "item_class", &item.class)?;
        let subclass_id = match &item.subclass {
            Some(subclass) => get_id(conn, "item_subclass", subclass)?,
            None => {
                eprintln!("Avertissement : item_id {} sans subclass, insertion ignorée.", item.item_id);
                continue;
            }
        };
        let quality_id = get_id(conn, "item_quality", &item.quality)?;
        let slot_id = get_id(conn, "item_slot", &item.slot)?;

        // Insérer la source si présente (simplifié, sans gestion des quêtes)
        let source_id = if let Some(source) = &item.source {
            let category_id = get_id(conn, "item_source_category", &source.category)?;
            let faction_id = match &source.faction {
                Some(f) => Some(get_id(conn, "faction", f)?),
                None => None,
            };
            let zone_id = source.zone;
            conn.execute(
                "INSERT INTO item_source (category_id, name, faction_id, cost, zone_id, drop_chance) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![
                    category_id,
                    source.name,
                    faction_id,
                    source.cost,
                    zone_id,
                    source.drop_chance
                ],
            )?;
            Some(conn.last_insert_rowid() as i32)
        } else {
            None
        };

        // Insérer l'item
        conn.execute(
            "INSERT INTO item (item_id, name, icon, class_id, subclass_id, sell_price, quality_id, item_level, required_level, slot_id, item_link, vendor_price, source_id, unique_name) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
            rusqlite::params![
                item.item_id,
                item.name,
                item.icon,
                class_id,
                subclass_id,
                item.sell_price,
                quality_id,
                item.item_level,
                item.required_level,
                slot_id,
                item.item_link,
                item.vendor_price,
                source_id,
                item.unique_name
            ],
        )?;

        // Insérer les tooltips et la table de liaison
        if let Some(tooltips) = &item.tooltip {
            for tooltip in tooltips {
                conn.execute(
                    "INSERT INTO item_tooltip (label, format_id) VALUES (?1, (SELECT id FROM item_tooltip_format WHERE label = ?2))",
                    rusqlite::params![tooltip.label, tooltip.format],
                )?;
                let tooltip_id = conn.last_insert_rowid() as i32;
                conn.execute(
                    "INSERT INTO item_tooltip_link (item_id, tooltip_id) VALUES (?1, ?2)",
                    rusqlite::params![item.item_id, tooltip_id],
                )?;
            }
        }
        // Affichage de la barre de progression dynamique avec estimation du temps restant
        let percent = ((i + 1) as f32 / total as f32) * 100.0;
        let filled = ((i + 1) as f32 / total as f32 * bar_width as f32).round() as usize;
        let bar = format!("{}{}", "#".repeat(filled), "-".repeat(bar_width - filled));
        let elapsed = start.elapsed().as_secs_f32();
        let remaining = if i > 10 && percent > 0.0 {
            let avg = elapsed / (i + 1) as f32;
            let left = (total - (i + 1)) as f32 * avg;
            Some(left)
        } else {
            None
        };
        let eta = match remaining {
            Some(secs) => {
                let total_sec = secs.round() as u32;
                let h = total_sec / 3600;
                let m = (total_sec % 3600) / 60;
                let s = total_sec % 60;
                format!("~{:02}:{:02}:{:02}", h, m, s)
            },
            None => String::from("--")
        };
        print!("\rItems importés: [{}] {:>3}% {} / {} | Temps restant: {}", bar, percent as u8, i + 1, total, eta);
        std::io::stdout().flush().unwrap();
    }
    println!("");
    Ok(())
}

fn get_id(conn: &Connection, table: &str, label: &str) -> Result<i32, Box<dyn std::error::Error>> {
    let mut stmt = conn.prepare(&format!("SELECT id FROM {} WHERE label = ?1", table))?;
    let id: i32 = stmt.query_row([label], |row| row.get(0))?;
    Ok(id)
}
