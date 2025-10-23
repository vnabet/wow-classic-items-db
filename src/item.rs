use rusqlite::Connection;

pub fn create_item_quest_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_quest (
            quest_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            faction_id INTEGER NOT NULL,
            FOREIGN KEY (faction_id) REFERENCES faction(id)
        )",
        [],
    )?;
    Ok(())
}

pub fn create_item_source_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_source (
            id INTEGER PRIMARY KEY,
            category_id INTEGER NOT NULL,
            name TEXT,
            faction_id INTEGER,
            cost INTEGER,
            zone_id INTEGER,
            drop_chance REAL,
            FOREIGN KEY (category_id) REFERENCES item_source_category(id),
            FOREIGN KEY (faction_id) REFERENCES faction(id),
            FOREIGN KEY (zone_id) REFERENCES zone(id)
        )",
        [],
    )?;
    Ok(())
}

pub fn create_item_source_quest_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_source_quest (
            item_source_id INTEGER NOT NULL,
            item_quest_id INTEGER NOT NULL,
            PRIMARY KEY (item_source_id, item_quest_id),
            FOREIGN KEY (item_source_id) REFERENCES item_source(id),
            FOREIGN KEY (item_quest_id) REFERENCES item_quest(quest_id)
        )",
        [],
    )?;
    Ok(())
}

pub fn create_item_tooltip_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_tooltip (
            id INTEGER PRIMARY KEY,
            label TEXT NOT NULL,
            format_id INTEGER,
            FOREIGN KEY (format_id) REFERENCES item_tooltip_format(id)
        )",
        [],
    )?;
    Ok(())
}

pub fn create_item_tooltip_link_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item_tooltip_link (
            item_id INTEGER NOT NULL,
            tooltip_id INTEGER NOT NULL,
            PRIMARY KEY (item_id, tooltip_id),
            FOREIGN KEY (item_id) REFERENCES item(item_id),
            FOREIGN KEY (tooltip_id) REFERENCES item_tooltip(id)
        )",
        [],
    )?;
    Ok(())
}

pub fn create_item_table(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS item (
            item_id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            icon TEXT,
            class_id INTEGER NOT NULL,
            subclass_id INTEGER NOT NULL,
            sell_price INTEGER,
            quality_id INTEGER NOT NULL,
            item_level INTEGER,
            required_level INTEGER,
            slot_id INTEGER NOT NULL,
            item_link TEXT,
            vendor_price INTEGER,
            source_id INTEGER,
            unique_name TEXT,
            FOREIGN KEY (class_id) REFERENCES item_class(id),
            FOREIGN KEY (subclass_id) REFERENCES item_subclass(id),
            FOREIGN KEY (quality_id) REFERENCES item_quality(id),
            FOREIGN KEY (slot_id) REFERENCES item_slot(id),
            FOREIGN KEY (source_id) REFERENCES item_source(id)
        )",
        [],
    )?;
    // AJOUTER la création d'index ici
    conn.execute(
        "CREATE INDEX IF NOT EXISTS idx_item_name ON item(name)",
        [],
    )?;
    Ok(())
}
