use rusqlite::Connection;
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct ZoneJson {
    id: i32,
    name: String,
    category: Option<String>,
    level: Option<[Option<i32>; 2]>,
    territory: Option<String>,
}

pub fn import_zones(conn: &Connection, json_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(json_path)?;
    let zones: Vec<ZoneJson> = serde_json::from_str(&data)?;

    use std::io::Write;
    use std::time::Instant;
    let total = zones.len();
    let bar_len = 30;
    let start = Instant::now();
    for (i, zone) in zones.iter().enumerate() {
        let category_id = match &zone.category {
            Some(cat) => get_id(conn, "zone_category", cat)?,
            None => get_id(conn, "zone_category", "undefined")?,
        };
        let territory_id = match &zone.territory {
            Some(terr) => get_id(conn, "zone_territory", terr)?,
            None => get_id(conn, "zone_territory", "Contested")?,
        };
        let (level_min, level_max) = match zone.level {
            Some([min, max]) => (min, max),
            _ => (None, None),
        };
        conn.execute(
            "INSERT INTO zone (id, name, category_id, level_min, level_max, territory_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![zone.id, zone.name, category_id, level_min, level_max, territory_id],
        )?;
        let percent = ((i + 1) as f32 / total as f32) * 100.0;
        let filled = ((i + 1) as f32 / total as f32 * bar_len as f32).round() as usize;
        let bar = format!("{}{}", "#".repeat(filled), "-".repeat(bar_len - filled));
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
        print!("\rZones importées: [{}] {:>3}% {} / {} | Temps restant: {}", bar, percent as u8, i + 1, total, eta);
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
