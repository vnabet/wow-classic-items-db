CREATE TABLE zone_category (
    id INTEGER PRIMARY KEY,
    label TEXT NOT NULL,
    label_frFR TEXT NOT NULL
);

CREATE TABLE zone_territory (
    id INTEGER PRIMARY KEY,
    label TEXT NOT NULL,
    label_frFR TEXT NOT NULL
);

CREATE TABLE zone (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    category_id INTEGER NOT NULL,
    level_min INTEGER,
    level_max INTEGER,
    territory_id INTEGER NOT NULL,
    FOREIGN KEY (category_id) REFERENCES zone_category(id),
    FOREIGN KEY (territory_id) REFERENCES zone_territory(id)
);

CREATE TABLE item_class (
    id INTEGER PRIMARY KEY,
    label TEXT NOT NULL,
    label_frFR TEXT NOT NULL
);

CREATE TABLE item_subclass (
    id INTEGER PRIMARY KEY,
    label TEXT NOT NULL,
    label_frFR TEXT NOT NULL
);

CREATE TABLE item_quality (
    id INTEGER PRIMARY KEY,
    label TEXT NOT NULL,
    label_frFR TEXT NOT NULL
);

CREATE TABLE item_slot (
    id INTEGER PRIMARY KEY,
    label TEXT NOT NULL,
    label_frFR TEXT NOT NULL
);

CREATE TABLE faction (
    id INTEGER PRIMARY KEY,
    label TEXT NOT NULL,
    label_frFR TEXT NOT NULL
);

CREATE TABLE item_source_category (
    id INTEGER PRIMARY KEY,
    label TEXT NOT NULL,
    label_frFR TEXT NOT NULL
);

CREATE TABLE item_tooltip_format (
    id INTEGER PRIMARY KEY,
    label TEXT NOT NULL
);

CREATE TABLE item_quest (
    quest_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    faction_id INTEGER NOT NULL,
    FOREIGN KEY (faction_id) REFERENCES faction(id)
);

CREATE TABLE item_source (
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
);

-- Table de liaison entre item_source et item_quest (plusieurs quêtes par source)
CREATE TABLE item_source_quest (
    item_source_id INTEGER NOT NULL,
    item_quest_id INTEGER NOT NULL,
    PRIMARY KEY (item_source_id, item_quest_id),
    FOREIGN KEY (item_source_id) REFERENCES item_source(id),
    FOREIGN KEY (item_quest_id) REFERENCES item_quest(quest_id)
);

CREATE TABLE item_tooltip (
    id INTEGER PRIMARY KEY,
    label TEXT NOT NULL,
    format_id INTEGER,
    FOREIGN KEY (format_id) REFERENCES item_tooltip_format(id)
);

-- Table de liaison entre item et item_tooltip (plusieurs tooltips par item)
CREATE TABLE item_tooltip_link (
    item_id INTEGER NOT NULL,
    tooltip_id INTEGER NOT NULL,
    PRIMARY KEY (item_id, tooltip_id),
    FOREIGN KEY (item_id) REFERENCES item(item_id),
    FOREIGN KEY (tooltip_id) REFERENCES item_tooltip(id)
);

-- Table principale des items
CREATE TABLE item (
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
);

CREATE INDEX IF NOT EXISTS idx_item_name ON item(name);
